#!/usr/bin/env python3
"""Normalize current and legacy HAMR experiment metadata for comparison."""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any


EMOJI_TO_SEVERITY = {"🔴": "blocker", "🟠": "moderate", "🟡": "minor", "🟢": "positive"}
LEGACY_CATEGORY_FILES = {
    1: ("01-workflow-and-skill-design.md", "01-workflow-and-skill-design-issues.md"),
    2: ("02-hamr-codegen-and-tooling.md", "02-codegen-and-tooling-improvements.md"),
    3: ("03-documentation-and-tool-use-guidance.md", "03-documentation-and-guidance-improvements.md"),
    4: (
        "04-agent-harness-and-environment.md",
        "04-claude-code-settings-and-environment.md",
        "05-permission-grant-analysis.md",
        "05-permission-friction-and-preauthorization.md",
    ),
}
FINDING_RE = re.compile(r"^##\s+([A-Za-z][A-Za-z0-9_-]*)\s+—\s+(.+?)\s+(🔴|🟠|🟡|🟢)\s*$", re.MULTILINE)


def safe_json(path: Path) -> dict[str, Any] | None:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
        return value if isinstance(value, dict) else None
    except (OSError, json.JSONDecodeError):
        return None


def totals(models: dict[str, Any]) -> dict[str, int]:
    result = {key: 0 for key in ("input", "output", "cache_read", "cache_write", "reasoning_output")}
    for usage in models.values():
        if not isinstance(usage, dict):
            continue
        for key in result:
            result[key] += int(usage.get(key, 0) or 0)
    return result


def normalize_friction_v2(value: dict[str, Any]) -> dict[str, int]:
    def length(key: str) -> int:
        item = value.get(key)
        return len(item) if isinstance(item, list) else int(item or 0)
    return {
        "escalation_requests": length("escalation_requests"),
        "dynamic_shell_commands": length("dynamic_shell_commands"),
        "sandbox_failures": length("sandbox_failures"),
        "observable_approval_outcomes": length("observable_approval_outcomes"),
        "parser_coverage_warnings": length("parser_coverage_warnings"),
    }


def normalize_friction_v1(value: Any) -> dict[str, int]:
    value = value if isinstance(value, dict) else {}
    return {
        "escalation_requests": int(value.get("sandbox_escape_calls", 0) or 0),
        "dynamic_shell_commands": int(value.get("dynamic_command_calls", 0) or 0),
        "sandbox_failures": int(value.get("sandbox_failure_results", 0) or 0),
        "observable_approval_outcomes": 0,
        "parser_coverage_warnings": 0,
    }


def normalize_metrics(metrics: dict[str, Any] | None) -> tuple[dict[str, Any], list[str]]:
    caveats: list[str] = []
    if not metrics:
        return {
            "schema_version": None,
            "session_id": None,
            "harness": None,
            "harness_version": None,
            "models": [],
            "tokens": totals({}),
            "api_list_price_equivalent_usd": None,
            "actual_cost_usd": None,
            "active_seconds": None,
            "wall_clock_seconds": None,
            "friction": normalize_friction_v2({}),
        }, ["missing or invalid session-metrics.json"]
    if metrics.get("schema_version") == 2:
        harness = metrics.get("harness") if isinstance(metrics.get("harness"), dict) else {}
        session = metrics.get("session") if isinstance(metrics.get("session"), dict) else {}
        span = metrics.get("span") if isinstance(metrics.get("span"), dict) else {}
        models = metrics.get("models") if isinstance(metrics.get("models"), dict) else {}
        costs = metrics.get("cost_usd") if isinstance(metrics.get("cost_usd"), dict) else {}
        equivalent = costs.get("api_list_price_equivalent") if isinstance(costs.get("api_list_price_equivalent"), dict) else {}
        friction_value = metrics.get("friction") if isinstance(metrics.get("friction"), dict) else {}
        if costs.get("actual") is None:
            caveats.append("actual billed cost unavailable")
        if metrics.get("scope", {}).get("subagent_activity") == "excluded":
            caveats.append("subagents excluded")
        warnings = friction_value.get("parser_coverage_warnings")
        if isinstance(warnings, list) and warnings:
            caveats.append(f"{len(warnings)} parser coverage warning(s)")
        return {
            "schema_version": 2,
            "session_id": session.get("id"),
            "harness": harness.get("name"),
            "harness_version": harness.get("version"),
            "adapter_version": harness.get("adapter_version"),
            "source_format_version": harness.get("source_format_version"),
            "models": sorted(models),
            "tokens": totals(models),
            "api_list_price_equivalent_usd": equivalent.get("total"),
            "actual_cost_usd": costs.get("actual"),
            "active_seconds": span.get("active_seconds"),
            "wall_clock_seconds": span.get("wall_clock_seconds"),
            "friction": normalize_friction_v2(friction_value),
        }, caveats

    caveats.extend(["legacy schema-v1 metrics adapted", "actual billed cost unavailable", "subagent inclusion unknown or excluded"])
    models = metrics.get("models") if isinstance(metrics.get("models"), dict) else {}
    span = metrics.get("span") if isinstance(metrics.get("span"), dict) else {}
    cost = metrics.get("cost_usd") if isinstance(metrics.get("cost_usd"), dict) else {}
    return {
        "schema_version": 1,
        "session_id": metrics.get("session_id"),
        "harness": "claude-code",
        "harness_version": metrics.get("version"),
        "adapter_version": "legacy-adapter-v1",
        "source_format_version": "claude-jsonl-v1",
        "models": sorted(models),
        "tokens": totals(models),
        "api_list_price_equivalent_usd": cost.get("total"),
        "actual_cost_usd": None,
        "active_seconds": span.get("active_seconds"),
        "wall_clock_seconds": span.get("wall_clock_seconds"),
        "friction": normalize_friction_v1(metrics.get("friction")),
    }, caveats


def markdown_findings(report_dir: Path) -> list[dict[str, Any]]:
    findings = []
    seen_files: set[Path] = set()
    for category, names in LEGACY_CATEGORY_FILES.items():
        for name in names:
            path = report_dir / name
            if not path.is_file() or path in seen_files:
                continue
            seen_files.add(path)
            for code, title, emoji in FINDING_RE.findall(path.read_text(encoding="utf-8")):
                scope = "claude" if "claude" in name else ("environment" if category == 4 else ("hamr" if category == 2 else "shared-workflow"))
                findings.append({
                    "code": code,
                    "category": category,
                    "severity": EMOJI_TO_SEVERITY[emoji],
                    "scope": scope,
                    "title": title.strip(),
                    "source": name,
                })
    return findings


def normalize_assessment(report_dir: Path) -> tuple[dict[str, Any], list[str]]:
    caveats: list[str] = []
    summary = safe_json(report_dir / "assessment-summary.json")
    if summary and summary.get("schema_version") == 1 and isinstance(summary.get("findings"), list):
        findings = [item for item in summary["findings"] if isinstance(item, dict)]
        source = "assessment-summary-v1"
    else:
        findings = markdown_findings(report_dir)
        source = "legacy-markdown"
        caveats.append("legacy Markdown-only assessment adapted")
    severity = Counter(item.get("severity") for item in findings)
    scope = Counter(item.get("scope") for item in findings)
    category = Counter(str(item.get("category")) for item in findings)
    return {
        "source": source,
        "finding_count": len(findings),
        "by_severity": dict(sorted(severity.items())),
        "by_scope": dict(sorted(scope.items())),
        "by_category": dict(sorted(category.items())),
        "findings": findings,
    }, caveats


def parse_profile(status_text: str) -> str | None:
    match = re.search(r"(?:\*\*)?Profile(?:\*\*)?\s*:\s*`?([A-Za-z0-9_-]+)", status_text, re.IGNORECASE)
    return match.group(1).lower() if match else None


def parse_outcomes(status_text: str) -> dict[str, str | None]:
    patterns = {
        "built": r"(?:end-to-end build|SysDevAll\.4[^\n]*\|\s*done\s*\|)",
        "tipe": r"tipe[^\n]{0,160}\b(?:pass(?:ed)?|success|done|clean|well-formed)\b",
        "logika": r"logika[^\n]{0,180}(?:\b(?:pass(?:ed)?|success|done|clean|vacuous|verified)\b|exit\s+0)",
        "verus": r"\bverus\b[^\n]{0,180}(?:\b(?:pass(?:ed)?|success|done|clean|verified)\b|0\s+errors)",
        "tests": r"\btests?\b[^\n]{0,160}\b(?:pass(?:ed)?|success|done|clean)\b",
    }
    result: dict[str, str | None] = {}
    for key, pattern in patterns.items():
        match = re.search(pattern, status_text, re.IGNORECASE)
        result[key] = "pass" if match else None
    coverage_values = []
    for line in status_text.splitlines():
        if "coverage" in line.lower():
            coverage_values.extend(float(value) for value in re.findall(r"(\d+(?:\.\d+)?)%", line))
    result["coverage"] = f"{max(coverage_values):g}%" if coverage_values else None
    return result


def normalize_project(project: Path) -> dict[str, Any]:
    report_dir = project / "experiment-reports"
    metrics, metric_caveats = normalize_metrics(safe_json(report_dir / "session-metrics.json"))
    assessment, assessment_caveats = normalize_assessment(report_dir)
    status_path = project / "reports" / "workflow-status.md"
    status = status_path.read_text(encoding="utf-8") if status_path.is_file() else ""
    caveats = metric_caveats + assessment_caveats
    if not status:
        caveats.append("missing workflow-status.md")
    return {
        "project": str(project.resolve()),
        "name": project.name,
        "profile": parse_profile(status),
        "outcomes": parse_outcomes(status),
        "metrics": metrics,
        "assessment": assessment,
        "data_quality_caveats": list(dict.fromkeys(caveats)),
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("projects", nargs="+", help="resolved experiment project roots")
    parser.add_argument("--out", help="write normalized JSON to this path; stdout if omitted")
    args = parser.parse_args(argv)
    projects = [Path(item) for item in args.projects]
    missing = [str(path) for path in projects if not path.is_dir()]
    if missing:
        print(f"error: project directories not found: {', '.join(missing)}", file=sys.stderr)
        return 2
    output = {
        "normalizer_schema_version": 1,
        "experiments": [normalize_project(path) for path in projects],
    }
    rendered = json.dumps(output, indent=2, ensure_ascii=False) + "\n"
    if args.out:
        path = Path(args.out)
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(rendered, encoding="utf-8")
        print(f"wrote {path.resolve()}")
    else:
        print(rendered, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
