#!/usr/bin/env python3
"""Render Claude Code or Codex JSONL as one transcript/metrics contract.

The adapters deliberately stop at observable transcript data. They do not infer actual
billing, hidden approval prompts, hidden reasoning, or subagent usage.
"""

from __future__ import annotations

import argparse
import datetime as dt
import glob
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Iterable


ADAPTER_VERSION = "2.0.0"
CLAUDE_FORMAT_VERSION = "claude-jsonl-v1"
CODEX_FORMAT_VERSION = "codex-rollout-jsonl-v2"
ACTIVE_GAP_SECONDS = 300
RESULT_CAP = 700
COMMAND_CAP = 400
MESSAGE_CAP = 4000

FAILURE_SIGNATURES = (
    "operation not permitted",
    "permission denied",
    "read-only file system",
    "sandbox violation",
    "sandbox denied",
    "network is unreachable",
    "network access is restricted",
    "could not resolve host",
    "proxy connect",
)


class TranscribeError(RuntimeError):
    pass


@dataclass
class SessionData:
    harness: str
    source_path: str
    source_type: str
    source_format_version: str
    session_id: str | None = None
    cwd: str | None = None
    version: str | None = None
    git_branch: str | None = None
    events: list[dict[str, Any]] = field(default_factory=list)
    models: dict[str, dict[str, int]] = field(default_factory=dict)
    timestamps: list[dt.datetime] = field(default_factory=list)
    counts: dict[str, int] = field(default_factory=lambda: {
        "user_turns": 0,
        "assistant_messages": 0,
        "tool_calls": 0,
        "tool_results": 0,
        "malformed_records": 0,
        "compactions": 0,
    })
    friction: dict[str, list[Any]] = field(default_factory=lambda: {
        "escalation_requests": [],
        "dynamic_shell_commands": [],
        "sandbox_failures": [],
        "observed_policy_settings": [],
        "observable_approval_outcomes": [],
        "parser_coverage_warnings": [],
        "epistemic_limitations": [],
    })


def parse_timestamp(value: Any) -> dt.datetime | None:
    if not isinstance(value, str):
        return None
    try:
        return dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return None


def truncate(value: Any, cap: int) -> str:
    if not isinstance(value, str):
        value = json.dumps(value, ensure_ascii=False, sort_keys=True)
    value = value.rstrip()
    if len(value) <= cap:
        return value
    return f"{value[:cap].rstrip()}  …[+{len(value) - cap} chars]"


def content_text(content: Any) -> str:
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        parts: list[str] = []
        for block in content:
            if isinstance(block, str):
                parts.append(block)
            elif isinstance(block, dict):
                text = block.get("text")
                if text is None:
                    text = block.get("input_text") or block.get("output_text")
                if text is not None:
                    parts.append(str(text))
        return "\n".join(parts)
    if content is None:
        return ""
    return str(content)


def clean_user_text(text: str) -> str:
    patterns = (
        r"<system-reminder>.*?</system-reminder>",
        r"<local-command-[^>]*>.*?</local-command-[^>]*>",
        r"<command-[^>]*>.*?</command-[^>]*>",
    )
    for pattern in patterns:
        text = re.sub(pattern, "", text, flags=re.DOTALL)
    return text.strip()


def add_model_usage(data: SessionData, model: str | None, usage: dict[str, Any]) -> None:
    if not model:
        model = "unknown"
    totals = data.models.setdefault(model, {
        "input": 0,
        "output": 0,
        "cache_read": 0,
        "cache_write": 0,
    })
    for key in ("input", "output", "cache_read", "cache_write", "reasoning_output"):
        value = usage.get(key)
        if value is not None:
            totals[key] = totals.get(key, 0) + max(0, int(value or 0))


def shell_commands(tool_input: dict[str, Any]) -> list[str]:
    commands = tool_input.get("commands")
    if isinstance(commands, list):
        return [str(item.get("cmd") or item.get("command") or "") for item in commands if isinstance(item, dict)]
    command = tool_input.get("cmd")
    if command is None:
        command = tool_input.get("command")
    return [str(command)] if command else []


def dynamic_markers(command: str) -> list[str]:
    markers = []
    if "$(" in command or "`" in command:
        markers.append("substitution")
    if "<<" in command:
        markers.append("heredoc")
    if "${" in command or re.search(r"\$[A-Za-z_]", command):
        markers.append("variable_expansion")
    if any(token in command for token in (" && ", " || ", ";", " | ")):
        markers.append("compound_command")
    return markers


def observe_shell_call(data: SessionData, call_id: str | None, tool_input: dict[str, Any], mechanism: str) -> None:
    for command in shell_commands(tool_input):
        markers = dynamic_markers(command)
        if markers:
            data.friction["dynamic_shell_commands"].append({
                "call_id": call_id,
                "command": truncate(command, 240),
                "markers": markers,
                "classification": "heuristic_prompt_candidate",
            })
    escalated = tool_input.get("sandbox_permissions") == "require_escalated"
    escalated = escalated or bool(tool_input.get("dangerouslyDisableSandbox"))
    if escalated:
        data.friction["escalation_requests"].append({
            "call_id": call_id,
            "command": truncate(shell_commands(tool_input)[0], 240) if shell_commands(tool_input) else None,
            "mechanism": mechanism,
            "justification": tool_input.get("justification") or tool_input.get("description"),
        })


def failure_signature(text: str) -> str | None:
    # Limit matching to the beginning of the observable result. Long successful
    # read/search outputs often quote an old error or the harness policy text and
    # must not become fresh sandbox failures merely because that phrase occurs later.
    for line in text[:2000].splitlines():
        stripped = line.strip()
        lower = stripped.lower()
        signature = next((item for item in FAILURE_SIGNATURES if item in lower), None)
        if not signature:
            continue
        error_shaped = (
            lower.startswith(signature)
            or re.match(r"^(?:error|fatal|exception|caused by)\b", lower)
            or re.match(r"^[A-Za-z0-9_./-]+:\s", stripped)
        )
        if error_shaped:
            return signature
    return None


def observe_tool_result(data: SessionData, call_id: str | None, text: str) -> None:
    signature = failure_signature(text)
    if signature:
        data.friction["sandbox_failures"].append({
            "call_id": call_id,
            "signature": signature,
            "result_excerpt": truncate(text, 240),
            "classification": "confirmed_failure",
        })
    request_ids = {item.get("call_id") for item in data.friction["escalation_requests"]}
    if call_id in request_ids:
        data.friction["observable_approval_outcomes"].append({
            "call_id": call_id,
            "outcome": "tool_result_observed",
            "success": signature is None,
            "note": "Execution is observable; whether a human prompt appeared is not.",
        })


def load_jsonl(path: str) -> tuple[list[dict[str, Any]], list[str], int]:
    records: list[dict[str, Any]] = []
    warnings: list[str] = []
    malformed = 0
    raw_lines = Path(path).read_text(encoding="utf-8", errors="replace").splitlines()
    nonempty = [i for i, line in enumerate(raw_lines) if line.strip()]
    final_nonempty = nonempty[-1] if nonempty else -1
    for index, line in enumerate(raw_lines):
        if not line.strip():
            continue
        try:
            record = json.loads(line)
            if isinstance(record, dict):
                records.append(record)
            else:
                raise ValueError("JSON value is not an object")
        except (json.JSONDecodeError, ValueError):
            malformed += 1
            if index == final_nonempty:
                warnings.append(f"ignored partially written final JSONL record at line {index + 1}")
            else:
                warnings.append(f"ignored malformed JSONL record at line {index + 1}")
    return records, warnings, malformed


def record_timestamp(record: dict[str, Any]) -> dt.datetime | None:
    value = record.get("timestamp") or record.get("created_at")
    return parse_timestamp(value)


def event(data: SessionData, kind: str, timestamp: Any = None, **values: Any) -> None:
    item = {"kind": kind, **values}
    parsed = parse_timestamp(timestamp) if isinstance(timestamp, str) else timestamp
    if parsed:
        item["timestamp"] = parsed.isoformat()
    data.events.append(item)


def normalize_tool_name(name: str | None) -> str:
    if name in {"Bash", "bash", "exec_command", "shell", "functions.exec_command"}:
        return "shell"
    return name or "unknown_tool"


def claude_project_key(cwd: str) -> str:
    return re.sub(r"[^A-Za-z0-9]", "-", os.path.abspath(cwd))


def claude_record_cwd(records: Iterable[dict[str, Any]]) -> str | None:
    for record in records:
        for key in ("cwd", "projectPath", "project_path"):
            if isinstance(record.get(key), str):
                return record[key]
    return None


def peek_session(path: str) -> dict[str, Any]:
    try:
        records, _, _ = load_jsonl(path)
    except OSError:
        return {}
    harness = detect_records_harness(records)
    result: dict[str, Any] = {"harness": harness}
    if harness == "codex":
        meta = next((record.get("payload", {}) for record in records if record.get("type") == "session_meta"), {})
        result.update(
            id=meta.get("session_id") or meta.get("id"),
            cwd=meta.get("cwd"),
            thread_source=meta.get("thread_source"),
        )
    elif harness == "claude":
        result.update(
            id=next((record.get("sessionId") for record in records if record.get("sessionId")), None),
            cwd=claude_record_cwd(records),
        )
    return result


def detect_records_harness(records: Iterable[dict[str, Any]]) -> str | None:
    claude_score = codex_score = 0
    for record in records:
        typ = record.get("type")
        if typ in {"session_meta", "turn_context", "event_msg", "response_item"}:
            codex_score += 1
        if typ in {"user", "assistant"} and isinstance(record.get("message"), dict):
            claude_score += 1
        if record.get("sessionId"):
            claude_score += 1
    if claude_score and codex_score:
        return "ambiguous"
    if claude_score:
        return "claude"
    if codex_score:
        return "codex"
    return None


def discover_claude(cwd: str, base: str | None = None) -> list[str]:
    base = base or os.path.expanduser("~/.claude/projects")
    exact_dir = os.path.join(base, claude_project_key(cwd))
    exact = []
    for path in glob.glob(os.path.join(exact_dir, "*.jsonl")):
        observed_cwd = peek_session(path).get("cwd")
        if not observed_cwd or os.path.abspath(observed_cwd) == os.path.abspath(cwd):
            exact.append(path)
    matches = []
    for path in glob.glob(os.path.join(base, "*", "*.jsonl")):
        info = peek_session(path)
        if info.get("cwd") and os.path.abspath(info["cwd"]) == os.path.abspath(cwd):
            matches.append(path)
    candidates = list(dict.fromkeys(matches + exact))
    candidates.sort(key=os.path.getmtime, reverse=True)
    return candidates


def discover_codex(cwd: str, base: str | None = None) -> list[str]:
    base = base or os.path.expanduser("~/.codex/sessions")
    matches = []
    for path in glob.glob(os.path.join(base, "**", "rollout-*.jsonl"), recursive=True):
        info = peek_session(path)
        primary = info.get("thread_source") in {None, "user"}
        if primary and info.get("cwd") and os.path.abspath(info["cwd"]) == os.path.abspath(cwd):
            matches.append(path)
    matches.sort(key=os.path.getmtime, reverse=True)
    return matches


def resolve_session(
    harness: str,
    session: str | None,
    cwd: str,
    claude_base: str | None = None,
    codex_base: str | None = None,
) -> tuple[str, str]:
    if session and os.path.isfile(os.path.expanduser(session)):
        path = os.path.abspath(os.path.expanduser(session))
        detected = peek_session(path).get("harness")
        if detected in {None, "ambiguous"}:
            raise TranscribeError(f"cannot identify transcript format: {path}")
        if harness != "auto" and harness != detected:
            raise TranscribeError(f"--harness {harness} does not match {detected} transcript: {path}")
        return path, detected

    claude_candidates = discover_claude(cwd, claude_base) if harness in {"auto", "claude"} else []
    codex_candidates = discover_codex(cwd, codex_base) if harness in {"auto", "codex"} else []

    if session:
        all_candidates = []
        if harness in {"auto", "claude"}:
            base = claude_base or os.path.expanduser("~/.claude/projects")
            all_candidates.extend(glob.glob(os.path.join(base, "*", "*.jsonl")))
        if harness in {"auto", "codex"}:
            base = codex_base or os.path.expanduser("~/.codex/sessions")
            all_candidates.extend(glob.glob(os.path.join(base, "**", "rollout-*.jsonl"), recursive=True))
        selected = [path for path in all_candidates if session in os.path.basename(path) or peek_session(path).get("id") == session]
        if len(selected) != 1:
            raise TranscribeError(f"session selector {session!r} matched {len(selected)} transcripts; pass an exact path")
        detected = peek_session(selected[0]).get("harness")
        return os.path.abspath(selected[0]), detected

    available = [("claude", path) for path in claude_candidates[:1]] + [("codex", path) for path in codex_candidates[:1]]
    if harness == "auto" and len(available) > 1:
        details = ", ".join(f"{kind}: {path}" for kind, path in available)
        raise TranscribeError(f"auto selection is ambiguous for cwd {cwd}: {details}; pass --harness or --session")
    if not available:
        roots = "~/.claude/projects and ~/.codex/sessions" if harness == "auto" else f"the {harness} session store"
        raise TranscribeError(f"no {harness} transcript matching cwd {cwd} under {roots}")
    kind, path = available[0]
    return os.path.abspath(path), kind


def parse_claude(path: str) -> SessionData:
    records, warnings, malformed = load_jsonl(path)
    data = SessionData("claude-code", os.path.abspath(path), "claude-jsonl", CLAUDE_FORMAT_VERSION)
    data.counts["malformed_records"] = malformed
    data.friction["parser_coverage_warnings"].extend(warnings)
    data.friction["epistemic_limitations"].extend([
        "Approval prompts are not fully represented; escalation and dynamic-command data are observable proxies.",
        "Subagent JSONLs are excluded from tokens, time, dialog, and tool counts.",
        "Actual billed cost is not present in the session transcript.",
    ])
    call_names: dict[str, str] = {}

    for record in records:
        stamp = record_timestamp(record)
        if stamp:
            data.timestamps.append(stamp)
        data.session_id = record.get("sessionId") or data.session_id
        data.version = record.get("version") or data.version
        data.git_branch = record.get("gitBranch") or data.git_branch
        data.cwd = record.get("cwd") or record.get("projectPath") or data.cwd
        message = record.get("message")
        if not isinstance(message, dict):
            continue
        typ = record.get("type")
        content = message.get("content")
        if typ == "assistant":
            model = message.get("model")
            usage = message.get("usage")
            if isinstance(usage, dict):
                add_model_usage(data, model, {
                    "input": usage.get("input_tokens", 0),
                    "output": usage.get("output_tokens", 0),
                    "cache_write": usage.get("cache_creation_input_tokens", 0),
                    "cache_read": usage.get("cache_read_input_tokens", 0),
                })
            if not isinstance(content, list):
                continue
            for block in content:
                if not isinstance(block, dict):
                    continue
                if block.get("type") == "text" and str(block.get("text") or "").strip():
                    event(data, "assistant_message", stamp, text=str(block["text"]).strip(), phase="response")
                    data.counts["assistant_messages"] += 1
                elif block.get("type") == "tool_use":
                    raw_name = block.get("name")
                    name = normalize_tool_name(raw_name)
                    tool_input = block.get("input") if isinstance(block.get("input"), dict) else {}
                    call_id = block.get("id")
                    if call_id:
                        call_names[str(call_id)] = name
                    event(data, "tool_call", stamp, name=name, call_id=call_id, input=tool_input)
                    data.counts["tool_calls"] += 1
                    if name == "shell":
                        observe_shell_call(data, call_id, tool_input, "claude-dangerouslyDisableSandbox")
        elif typ == "user":
            if isinstance(content, str):
                text = clean_user_text(content)
                if text:
                    event(data, "user_message", stamp, text=text)
                    data.counts["user_turns"] += 1
            elif isinstance(content, list):
                for block in content:
                    if not isinstance(block, dict):
                        continue
                    if block.get("type") == "text":
                        text = clean_user_text(str(block.get("text") or ""))
                        if text:
                            event(data, "user_message", stamp, text=text)
                            data.counts["user_turns"] += 1
                    elif block.get("type") == "tool_result":
                        text = content_text(block.get("content"))
                        call_id = block.get("tool_use_id")
                        event(data, "tool_result", stamp, call_id=call_id, output=text)
                        data.counts["tool_results"] += 1
                        if call_names.get(str(call_id)) == "shell":
                            observe_tool_result(data, call_id, text)

    if not data.cwd:
        data.cwd = os.getcwd()
    pending = {item.get("call_id") for item in data.events if item["kind"] == "tool_call"}
    completed = {item.get("call_id") for item in data.events if item["kind"] == "tool_result"}
    for call_id in sorted(str(value) for value in pending - completed if value):
        data.friction["parser_coverage_warnings"].append(f"tool call {call_id} has no observed result")
    return data


def js_tool_calls(source: str) -> list[tuple[str, int]]:
    """Return tools.<name>( positions that occur outside JavaScript string literals."""
    calls: list[tuple[str, int]] = []
    quote: str | None = None
    escaped = False
    index = 0
    while index < len(source):
        char = source[index]
        if quote:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == quote:
                quote = None
            index += 1
            continue
        if char in {'"', "'", "`"}:
            quote = char
            index += 1
            continue
        match = re.match(r"tools\.([A-Za-z0-9_]+)\s*\(", source[index:])
        if match:
            calls.append((match.group(1), index))
            index += match.end()
            continue
        index += 1
    return calls


def extract_json_exec_calls(source: str) -> tuple[list[dict[str, Any]], list[str]]:
    """Extract tools.exec_command(<JSON object>) calls from orchestration source."""
    calls: list[dict[str, Any]] = []
    warnings: list[str] = []
    needle = "tools.exec_command("
    positions = [position for name, position in js_tool_calls(source) if name == "exec_command"]
    for start in positions:
        opening = source.find("(", start, start + len(needle) + 8)
        if opening < 0:
            warnings.append("tools.exec_command orchestration expression has no opening parenthesis")
            continue
        arg_start = opening + 1
        depth = 1
        quote: str | None = None
        escaped = False
        index = arg_start
        while index < len(source):
            char = source[index]
            if quote:
                if escaped:
                    escaped = False
                elif char == "\\":
                    escaped = True
                elif char == quote:
                    quote = None
            elif char in {'"', "'", "`"}:
                quote = char
            elif char == "(":
                depth += 1
            elif char == ")":
                depth -= 1
                if depth == 0:
                    break
            index += 1
        if depth:
            warnings.append("unterminated tools.exec_command orchestration expression")
            continue
        raw = source[arg_start:index].strip()
        try:
            value = json.loads(raw)
            if not isinstance(value, dict):
                raise ValueError("argument is not an object")
            calls.append(value)
        except (json.JSONDecodeError, ValueError):
            warnings.append("tools.exec_command arguments were not a JSON literal")
    return calls, warnings


def json_arguments(value: Any) -> tuple[dict[str, Any], str | None]:
    if isinstance(value, dict):
        return value, None
    if isinstance(value, str):
        try:
            parsed = json.loads(value)
            if isinstance(parsed, dict):
                return parsed, None
        except json.JSONDecodeError:
            pass
    return {}, "tool arguments were not a JSON object"


def codex_message_text(payload: dict[str, Any]) -> str:
    text = content_text(payload.get("content"))
    if not text:
        text = str(payload.get("message") or "")
    return text.strip()


def compact_policy(payload: dict[str, Any]) -> dict[str, Any]:
    collaboration = payload.get("collaboration_mode")
    if isinstance(collaboration, dict):
        collaboration = collaboration.get("mode") or collaboration.get("kind")
    return {
        "approval_policy": payload.get("approval_policy"),
        "sandbox_policy": payload.get("sandbox_policy"),
        "collaboration_mode": collaboration,
    }


def parse_codex(path: str) -> SessionData:
    records, warnings, malformed = load_jsonl(path)
    data = SessionData("codex", os.path.abspath(path), "codex-rollout-jsonl", CODEX_FORMAT_VERSION)
    data.counts["malformed_records"] = malformed
    data.friction["parser_coverage_warnings"].extend(warnings)
    data.friction["epistemic_limitations"].extend([
        "Approval outcomes are reported only when an escalation request or result is observable in JSONL.",
        "Custom orchestration is decoded only when nested shell arguments are JSON literals.",
        "Subagent sessions are excluded from tokens, time, dialog, and tool counts.",
        "Reasoning records are omitted; reasoning_output tokens are an informational subset of output tokens.",
        "Actual billed cost and subscription-credit value are not inferred from rate-limit data.",
    ])
    current_model = "unknown"
    previous_snapshot = {key: 0 for key in (
        "input_tokens", "output_tokens", "cached_input_tokens", "cache_write_input_tokens", "reasoning_output_tokens"
    )}
    raw_call_to_normalized: dict[str, str] = {}
    raw_call_names: dict[str, str] = {}
    response_user_present = any(
        record.get("type") == "response_item"
        and isinstance(record.get("payload"), dict)
        and record["payload"].get("type") == "message"
        and record["payload"].get("role") == "user"
        for record in records
    )
    response_assistant_present = any(
        record.get("type") == "response_item"
        and isinstance(record.get("payload"), dict)
        and record["payload"].get("type") == "message"
        and record["payload"].get("role") == "assistant"
        for record in records
    )

    for record in records:
        stamp = record_timestamp(record)
        if stamp:
            data.timestamps.append(stamp)
        typ = record.get("type")
        payload = record.get("payload") if isinstance(record.get("payload"), dict) else {}

        if typ == "session_meta":
            data.session_id = payload.get("session_id") or payload.get("id") or data.session_id
            data.cwd = payload.get("cwd") or data.cwd
            data.version = payload.get("cli_version") or data.version
            git = payload.get("git") if isinstance(payload.get("git"), dict) else {}
            data.git_branch = git.get("branch") or data.git_branch
            if payload.get("thread_source") not in {None, "user"}:
                data.friction["parser_coverage_warnings"].append(
                    f"selected Codex transcript has thread_source={payload.get('thread_source')!r}; primary-session scope may be violated"
                )
        elif typ == "turn_context":
            current_model = payload.get("model") or current_model
            data.cwd = payload.get("cwd") or data.cwd
            policy = compact_policy(payload)
            if policy not in data.friction["observed_policy_settings"]:
                data.friction["observed_policy_settings"].append(policy)
        elif typ == "event_msg":
            event_type = payload.get("type")
            if event_type == "token_count":
                info = payload.get("info") if isinstance(payload.get("info"), dict) else {}
                total = info.get("total_token_usage") if isinstance(info.get("total_token_usage"), dict) else {}
                if total:
                    delta: dict[str, int] = {}
                    reset = False
                    for key in previous_snapshot:
                        value = int(total.get(key, 0) or 0)
                        if value < previous_snapshot[key]:
                            reset = True
                            delta[key] = value
                        else:
                            delta[key] = value - previous_snapshot[key]
                        previous_snapshot[key] = value
                    if reset:
                        data.friction["parser_coverage_warnings"].append(
                            "cumulative token snapshot decreased; treated the new values as a counter reset"
                        )
                    # Codex/OpenAI input_tokens includes cached/read and cache-write
                    # subsets. Normalize to the same mutually exclusive fields emitted
                    # by the Claude adapter before pricing or cross-run comparison.
                    uncached_input = max(
                        0,
                        delta["input_tokens"]
                        - delta["cached_input_tokens"]
                        - delta["cache_write_input_tokens"],
                    )
                    add_model_usage(data, current_model, {
                        "input": uncached_input,
                        "output": delta["output_tokens"],
                        "cache_read": delta["cached_input_tokens"],
                        "cache_write": delta["cache_write_input_tokens"],
                        "reasoning_output": delta["reasoning_output_tokens"],
                    })
            elif event_type in {"context_compacted", "compaction", "compact"}:
                data.counts["compactions"] += 1
            elif event_type == "user_message" and not response_user_present:
                text = str(payload.get("message") or "").strip()
                if text:
                    event(data, "user_message", stamp, text=text)
                    data.counts["user_turns"] += 1
            elif event_type == "agent_message" and not response_assistant_present:
                text = str(payload.get("message") or "").strip()
                if text:
                    event(data, "assistant_message", stamp, text=text, phase=payload.get("phase") or "response")
                    data.counts["assistant_messages"] += 1
        elif typ == "response_item":
            item_type = payload.get("type")
            if item_type == "message":
                role = payload.get("role")
                text = codex_message_text(payload)
                if role == "user" and text:
                    event(data, "user_message", stamp, text=text)
                    data.counts["user_turns"] += 1
                elif role == "assistant" and text:
                    event(data, "assistant_message", stamp, text=text, phase=payload.get("phase") or "response")
                    data.counts["assistant_messages"] += 1
                elif role == "developer" and text:
                    approved = re.search(r"Approved command prefix saved|approval granted", text, re.IGNORECASE)
                    denied = re.search(r"approval (?:was )?denied|request denied", text, re.IGNORECASE)
                    if approved or denied:
                        data.friction["observable_approval_outcomes"].append({
                            "call_id": None,
                            "outcome": "approval_granted" if approved else "approval_denied",
                            "evidence": truncate(text, 200),
                        })
            elif item_type in {"function_call", "custom_tool_call"}:
                raw_call_id = str(payload.get("call_id") or payload.get("id") or f"call-{data.counts['tool_calls'] + 1}")
                raw_name = payload.get("name")
                warning: str | None = None
                if item_type == "custom_tool_call" and raw_name == "exec":
                    source = str(payload.get("input") or "")
                    calls, call_warnings = extract_json_exec_calls(source)
                    if len(calls) == 1:
                        name, tool_input = "shell", calls[0]
                    elif len(calls) > 1:
                        name, tool_input = "shell_batch", {"commands": calls}
                    else:
                        nested_tools = sorted({name for name, _ in js_tool_calls(source)})
                        name, tool_input = "orchestration", {
                            "tools": nested_tools,
                            "source": truncate(source, COMMAND_CAP),
                        }
                        if "exec_command" in nested_tools:
                            warning = f"custom orchestration call {raw_call_id} could not be decoded; nested shell coverage is incomplete"
                    for item in call_warnings:
                        data.friction["parser_coverage_warnings"].append(f"{raw_call_id}: {item}")
                else:
                    arguments = payload.get("arguments") if item_type == "function_call" else payload.get("input")
                    tool_input, warning = json_arguments(arguments)
                    name = normalize_tool_name(raw_name)
                if warning:
                    data.friction["parser_coverage_warnings"].append(warning)
                raw_call_to_normalized[raw_call_id] = raw_call_id
                raw_call_names[raw_call_id] = name
                event(data, "tool_call", stamp, name=name, call_id=raw_call_id, input=tool_input)
                data.counts["tool_calls"] += 1
                if name in {"shell", "shell_batch"}:
                    observe_shell_call(data, raw_call_id, tool_input, "codex-require_escalated")
            elif item_type in {"function_call_output", "custom_tool_call_output"}:
                raw_call_id = str(payload.get("call_id") or payload.get("id") or "unknown")
                call_id = raw_call_to_normalized.get(raw_call_id, raw_call_id)
                output = content_text(payload.get("output"))
                event(data, "tool_result", stamp, call_id=call_id, output=output)
                data.counts["tool_results"] += 1
                if raw_call_names.get(raw_call_id) in {"shell", "shell_batch"}:
                    observe_tool_result(data, call_id, output)
            elif item_type in {"compaction", "context_compaction"}:
                data.counts["compactions"] += 1

    if not data.cwd:
        data.cwd = os.getcwd()
    pending = {item.get("call_id") for item in data.events if item["kind"] == "tool_call"}
    completed = {item.get("call_id") for item in data.events if item["kind"] == "tool_result"}
    for call_id in sorted(str(value) for value in pending - completed if value):
        data.friction["parser_coverage_warnings"].append(f"tool call {call_id} has no observed result")
    return data


def load_pricing(path: str | None, data: SessionData) -> dict[str, Any]:
    if not path:
        data.friction["parser_coverage_warnings"].append("no pricing file supplied; API-list-price equivalent is unavailable")
        return {}
    try:
        value = json.loads(Path(path).read_text(encoding="utf-8"))
        if not isinstance(value, dict):
            raise ValueError("pricing root is not an object")
        return value
    except (OSError, json.JSONDecodeError, ValueError) as error:
        data.friction["parser_coverage_warnings"].append(f"could not load pricing file {path}: {error}")
        return {}


def pricing_entry(pricing: dict[str, Any], model: str) -> tuple[str, dict[str, Any]] | None:
    models = pricing.get("models") if isinstance(pricing.get("models"), dict) else {}
    if model in models and isinstance(models[model], dict):
        return model, models[model]
    for qualified, entry in models.items():
        if not isinstance(entry, dict):
            continue
        aliases = entry.get("aliases") if isinstance(entry.get("aliases"), list) else []
        if model in aliases:
            return qualified, entry
    return None


def calculate_cost(data: SessionData, pricing: dict[str, Any]) -> dict[str, Any]:
    by_model: dict[str, float | None] = {}
    unknown: list[str] = []
    for model, usage in data.models.items():
        matched = pricing_entry(pricing, model)
        if not matched:
            by_model[model] = None
            unknown.append(model)
            continue
        _, rates = matched
        cost = (
            usage.get("input", 0) * float(rates.get("input", 0) or 0)
            + usage.get("output", 0) * float(rates.get("output", 0) or 0)
            + usage.get("cache_read", 0) * float(rates.get("cache_read", 0) or 0)
            + usage.get("cache_write", 0) * float(rates.get("cache_write", 0) or 0)
        ) / 1_000_000
        by_model[model] = round(cost, 6)
    if unknown:
        data.friction["parser_coverage_warnings"].append(
            f"no public API pricing entry for model(s): {', '.join(sorted(unknown))}; total cost is null"
        )
    total = None if unknown or not data.models else round(sum(value or 0 for value in by_model.values()), 6)
    return {
        "api_list_price_equivalent": {"total": total, "by_model": by_model},
        "actual": None,
        "billing_basis": None,
        "pricing_date": pricing.get("pricing_date"),
        "note": (
            "API list-price equivalence is the cross-run comparison measure. Actual billing remains null unless "
            "a transcript supplies a trustworthy billed amount; subscription credits and rate-limit percentages "
            "are not converted to dollars."
        ),
    }


def git_branch(cwd: str | None) -> str | None:
    if not cwd or not os.path.isdir(cwd):
        return None
    try:
        result = subprocess.run(
            ["git", "-C", cwd, "branch", "--show-current"],
            check=False,
            capture_output=True,
            text=True,
            timeout=5,
        )
        return result.stdout.strip() or None
    except (OSError, subprocess.SubprocessError):
        return None


def span(data: SessionData) -> dict[str, Any]:
    timestamps = sorted(data.timestamps)
    if not timestamps:
        return {"first": None, "last": None, "wall_clock_seconds": 0, "active_seconds": 0, "active_gap_seconds": ACTIVE_GAP_SECONDS}
    active = 0.0
    for earlier, later in zip(timestamps, timestamps[1:]):
        gap = (later - earlier).total_seconds()
        if 0 <= gap <= ACTIVE_GAP_SECONDS:
            active += gap
    return {
        "first": timestamps[0].isoformat(),
        "last": timestamps[-1].isoformat(),
        "wall_clock_seconds": round((timestamps[-1] - timestamps[0]).total_seconds()),
        "active_seconds": round(active),
        "active_gap_seconds": ACTIVE_GAP_SECONDS,
    }


def build_metrics(data: SessionData, pricing: dict[str, Any]) -> dict[str, Any]:
    branch = data.git_branch or git_branch(data.cwd)
    return {
        "schema_version": 2,
        "session": {
            "id": data.session_id,
            "source_path": data.source_path,
            "source_type": data.source_type,
            "cwd": data.cwd,
            "git_branch": branch,
        },
        "harness": {
            "name": data.harness,
            "version": data.version,
            "adapter_version": ADAPTER_VERSION,
            "source_format_version": data.source_format_version,
        },
        "scope": {
            "primary_development_session_only": True,
            "subagent_activity": "excluded",
            "note": "Subagents are excluded until both harness adapters can include them consistently.",
        },
        "span": span(data),
        "models": data.models,
        "counts": data.counts,
        "cost_usd": calculate_cost(data, pricing),
        "friction": data.friction,
    }


def hms(seconds: int | float) -> str:
    value = max(0, int(seconds))
    return f"{value // 3600}h{(value % 3600) // 60:02d}m"


def token_totals(models: dict[str, dict[str, int]]) -> dict[str, int]:
    keys = ("input", "output", "cache_read", "cache_write", "reasoning_output")
    return {key: sum(values.get(key, 0) for values in models.values()) for key in keys}


def render_tool_input(name: str, tool_input: dict[str, Any]) -> list[str]:
    if name == "shell":
        command = shell_commands(tool_input)
        return ["```sh", truncate(command[0] if command else "", COMMAND_CAP), "```"]
    if name == "shell_batch":
        lines = []
        for index, command in enumerate(shell_commands(tool_input), 1):
            lines.extend([f"Command {index}:", "```sh", truncate(command, COMMAND_CAP), "```"])
        return lines
    return ["```json", truncate(tool_input, COMMAND_CAP), "```"]


def render_transcript(data: SessionData, metrics: dict[str, Any]) -> str:
    lines = ["# Session Transcript", "", "## Session metadata", ""]
    session = metrics["session"]
    harness = metrics["harness"]
    lines.extend([
        f"- **Session:** `{session.get('id') or 'n/a'}`",
        f"- **Harness:** {harness['name']} {harness.get('version') or 'n/a'}",
        f"- **Adapter/source format:** {harness['adapter_version']} / {harness['source_format_version']}",
        f"- **Source:** `{session['source_path']}`",
        f"- **CWD / branch:** `{session.get('cwd') or 'n/a'}` / `{session.get('git_branch') or 'n/a'}`",
        "",
        "## Token and time summary",
        "",
    ])
    timing = metrics["span"]
    totals = token_totals(metrics["models"])
    equivalent = metrics["cost_usd"]["api_list_price_equivalent"]["total"]
    cost_text = "unavailable" if equivalent is None else f"${equivalent:,.4f}"
    lines.extend([
        f"- **Span:** {timing['first'] or 'n/a'} → {timing['last'] or 'n/a'}",
        f"- **Time:** {hms(timing['wall_clock_seconds'])} wall / {hms(timing['active_seconds'])} active (gaps > {ACTIVE_GAP_SECONDS}s excluded)",
        f"- **Models:** {', '.join(sorted(metrics['models'])) or 'n/a'}",
        f"- **Tokens:** input {totals['input']:,} · output {totals['output']:,} · cache-read {totals['cache_read']:,} · cache-write {totals['cache_write']:,} · reasoning-output subset {totals['reasoning_output']:,}",
        f"- **API-list-price equivalent:** {cost_text} · **actual billed cost:** unavailable",
        f"- **Activity:** {data.counts['user_turns']} user turns · {data.counts['tool_calls']} tool calls · {data.counts['compactions']} compactions",
        "",
        "## Dialog and tool activity",
        "",
    ])
    for item in data.events:
        kind = item["kind"]
        if kind == "user_message":
            lines.extend(["### User", "", truncate(item.get("text", ""), MESSAGE_CAP), ""])
        elif kind == "assistant_message":
            phase = item.get("phase")
            suffix = f" ({phase})" if phase and phase != "response" else ""
            lines.extend([f"### Assistant{suffix}", "", truncate(item.get("text", ""), MESSAGE_CAP), ""])
        elif kind == "tool_call":
            lines.extend([f"#### Tool call — {item.get('name')} `{item.get('call_id') or 'n/a'}`", ""])
            lines.extend(render_tool_input(item.get("name", ""), item.get("input") or {}))
            lines.append("")
        elif kind == "tool_result":
            lines.extend([
                f"##### Tool result — `{item.get('call_id') or 'n/a'}`",
                "",
                "```text",
                truncate(item.get("output", ""), RESULT_CAP),
                "```",
                "",
            ])
    lines.extend([
        "## Omissions and coverage",
        "",
        "- Developer/system instructions and internal reasoning are omitted.",
        "- Tool results and shell commands are truncated; the source JSONL remains authoritative.",
        "- Subagent activity is excluded from all metrics and dialog.",
    ])
    warnings = data.friction["parser_coverage_warnings"]
    if warnings:
        lines.append("- Parser coverage warnings:")
        lines.extend(f"  - {warning}" for warning in warnings)
    else:
        lines.append("- Parser coverage warnings: none.")
    lines.append("")
    return "\n".join(lines)


def write_outputs(data: SessionData, out_dir: str, pricing_path: str | None) -> tuple[dict[str, Any], str, str]:
    pricing = load_pricing(pricing_path, data)
    metrics = build_metrics(data, pricing)
    transcript = render_transcript(data, metrics)
    os.makedirs(out_dir, exist_ok=True)
    transcript_path = os.path.join(out_dir, "session-transcript.md")
    metrics_path = os.path.join(out_dir, "session-metrics.json")
    Path(transcript_path).write_text(transcript, encoding="utf-8")
    Path(metrics_path).write_text(json.dumps(metrics, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return metrics, transcript_path, metrics_path


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--harness", choices=("auto", "claude", "codex"), default="auto")
    parser.add_argument("--session", help="exact JSONL path or session id")
    parser.add_argument("--out", default=None, help="output directory (default: ./experiment-reports)")
    parser.add_argument("--pricing", default=str(Path(__file__).resolve().parent.parent / "pricing.json"))
    args = parser.parse_args(argv)
    try:
        path, detected = resolve_session(args.harness, args.session, os.getcwd())
        data = parse_claude(path) if detected == "claude" else parse_codex(path)
        out_dir = os.path.abspath(args.out or os.path.join(os.getcwd(), "experiment-reports"))
        metrics, transcript_path, metrics_path = write_outputs(data, out_dir, args.pricing)
    except (TranscribeError, OSError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2

    totals = token_totals(metrics["models"])
    cost = metrics["cost_usd"]["api_list_price_equivalent"]["total"]
    print(f"wrote {transcript_path}")
    print(f"wrote {metrics_path}")
    print(f"session {data.session_id or 'n/a'} | harness {data.harness} {data.version or 'n/a'} | models {', '.join(sorted(data.models)) or 'n/a'}")
    print(f"tokens input {totals['input']:,} output {totals['output']:,} cache-read {totals['cache_read']:,} cache-write {totals['cache_write']:,}")
    print(f"time {hms(metrics['span']['wall_clock_seconds'])} wall / {hms(metrics['span']['active_seconds'])} active | tool calls {data.counts['tool_calls']}")
    print(f"API-list-price equivalent: {'unavailable' if cost is None else f'${cost:,.4f}'} | actual: unavailable")
    print(
        "friction: "
        f"{len(data.friction['escalation_requests'])} escalation requests | "
        f"{len(data.friction['sandbox_failures'])} sandbox failures | "
        f"{len(data.friction['parser_coverage_warnings'])} parser warnings"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
