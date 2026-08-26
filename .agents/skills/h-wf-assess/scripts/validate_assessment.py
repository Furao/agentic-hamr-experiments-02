#!/usr/bin/env python3
"""Validate the common HAMR experiment assessment contract."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any


CATEGORY_FILES = {
    1: "01-workflow-and-skill-design.md",
    2: "02-hamr-codegen-and-tooling.md",
    3: "03-documentation-and-tool-use-guidance.md",
    4: "04-agent-harness-and-environment.md",
}
SEVERITIES = {"blocker": "🔴", "moderate": "🟠", "minor": "🟡", "positive": "🟢"}
SCOPES = {"hamr", "shared-workflow", "claude", "codex", "environment"}
FINDING_RE = re.compile(r"^##\s+([A-Za-z][A-Za-z0-9_-]*)\s+—\s+(.+?)\s+(🔴|🟠|🟡|🟢)\s*$", re.MULTILINE)


def load_summary(report_dir: Path) -> tuple[dict[str, Any] | None, list[str]]:
    path = report_dir / "assessment-summary.json"
    if not path.is_file():
        return None, [f"missing {path.name}"]
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        return None, [f"invalid {path.name}: {error}"]
    if not isinstance(value, dict):
        return None, [f"{path.name} root must be an object"]
    return value, []


def validate(report_dir: Path) -> list[str]:
    summary, errors = load_summary(report_dir)
    if summary is None:
        return errors
    if summary.get("schema_version") != 1:
        errors.append("schema_version must be 1")
    run = summary.get("run")
    if not isinstance(run, dict):
        errors.append("run must be an object")
    else:
        for key in ("project", "session_id", "harness", "harness_version", "models", "profile"):
            if key not in run:
                errors.append(f"run.{key} is required (use null or [] when unavailable)")
        if run.get("harness") not in {"claude-code", "codex", None}:
            errors.append("run.harness must be claude-code, codex, or null")
        if "models" in run and not isinstance(run.get("models"), list):
            errors.append("run.models must be an array")

    findings = summary.get("findings")
    if not isinstance(findings, list):
        errors.append("findings must be an array")
        findings = []
    json_by_code: dict[str, dict[str, Any]] = {}
    for index, finding in enumerate(findings):
        prefix = f"findings[{index}]"
        if not isinstance(finding, dict):
            errors.append(f"{prefix} must be an object")
            continue
        for key in ("code", "category", "severity", "scope", "title", "evidence", "recommendation_target", "evidence_limit"):
            if key not in finding:
                errors.append(f"{prefix}.{key} is required")
        code = finding.get("code")
        if not isinstance(code, str) or not re.fullmatch(r"[A-Za-z][A-Za-z0-9_-]*", code):
            errors.append(f"{prefix}.code is invalid")
        elif code in json_by_code:
            errors.append(f"duplicate finding code {code}")
        else:
            json_by_code[code] = finding
        if finding.get("category") not in CATEGORY_FILES:
            errors.append(f"{prefix}.category must be 1, 2, 3, or 4")
        if finding.get("severity") not in SEVERITIES:
            errors.append(f"{prefix}.severity must be one of {', '.join(SEVERITIES)}")
        if finding.get("scope") not in SCOPES:
            errors.append(f"{prefix}.scope must be one of {', '.join(sorted(SCOPES))}")
        if not isinstance(finding.get("title"), str) or not finding.get("title", "").strip():
            errors.append(f"{prefix}.title must be non-empty")
        evidence = finding.get("evidence")
        if not isinstance(evidence, list) or not evidence or not all(isinstance(item, str) and item.strip() for item in evidence):
            errors.append(f"{prefix}.evidence must be a non-empty string array")
        for key in ("recommendation_target", "evidence_limit"):
            if not isinstance(finding.get(key), str) or not finding.get(key, "").strip():
                errors.append(f"{prefix}.{key} must be non-empty")

    markdown_by_code: dict[str, tuple[int, str, str]] = {}
    for category, filename in CATEGORY_FILES.items():
        path = report_dir / filename
        if not path.is_file():
            errors.append(f"missing {filename}")
            continue
        text = path.read_text(encoding="utf-8")
        for code, title, emoji in FINDING_RE.findall(text):
            if code in markdown_by_code:
                errors.append(f"duplicate Markdown finding code {code}")
            markdown_by_code[code] = (category, title.strip(), emoji)

    for code in sorted(json_by_code.keys() - markdown_by_code.keys()):
        errors.append(f"finding {code} is in JSON but not in a category Markdown file")
    for code in sorted(markdown_by_code.keys() - json_by_code.keys()):
        errors.append(f"finding {code} is in Markdown but not in assessment-summary.json")
    for code in sorted(json_by_code.keys() & markdown_by_code.keys()):
        finding = json_by_code[code]
        category, title, emoji = markdown_by_code[code]
        if finding.get("category") != category:
            errors.append(f"finding {code} category mismatch: JSON {finding.get('category')} vs Markdown {category}")
        if finding.get("title", "").strip() != title:
            errors.append(f"finding {code} title mismatch")
        expected_emoji = SEVERITIES.get(finding.get("severity"))
        if expected_emoji and expected_emoji != emoji:
            errors.append(f"finding {code} severity mismatch: JSON {finding.get('severity')} vs Markdown {emoji}")

    readme = report_dir / "README.md"
    if not readme.is_file():
        errors.append("missing README.md")
    return errors


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("report_dir", help="experiment-reports directory")
    args = parser.parse_args(argv)
    report_dir = Path(args.report_dir).resolve()
    errors = validate(report_dir)
    if errors:
        print(f"assessment validation failed ({len(errors)} error(s)):", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    summary = json.loads((report_dir / "assessment-summary.json").read_text(encoding="utf-8"))
    print(f"assessment valid: {len(summary['findings'])} finding(s) in {report_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
