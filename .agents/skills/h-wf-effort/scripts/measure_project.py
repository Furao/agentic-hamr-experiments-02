#!/usr/bin/env python3
"""Measure a HAMR experiment project for effort estimation (Wave 1).

Produces project-measures.json (schema project-measures-v1): per-file provenance
buckets, template accounting, assurance observations, and rollups. The codegen
report is treated as evidence, not gospel: it is validated against the on-disk
tree, and marker/path-rule classification supplements it for orphaned files.
Without a generation baseline, non-marker content of generate-once files is
classified preserved_origin_unknown, never developer_authored.

This script never executes builds, tests, or verification.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
DEFAULT_STEP_MAP = SCRIPT_DIR.parent / "config" / "step-map-v1.json"
BASELINE_AGREEMENT_THRESHOLD = 0.9


class MeasureError(Exception):
    pass


# --------------------------------------------------------------------------
# Line counting
# --------------------------------------------------------------------------

C_STYLE_LANGUAGES = {"rust", "c", "sysml", "gumbo", "dot"}
HASH_LANGUAGES = {"makefile", "shell", "toml"}

SUFFIX_LANGUAGE = {
    ".rs": "rust",
    ".c": "c",
    ".h": "c",
    ".xml": "xml",
    ".system": "xml",
    ".dot": "dot",
    ".md": "markdown",
    ".sysml": "sysml",
    ".toml": "toml",
    ".json": "json",
    ".mk": "makefile",
    ".cmd": "shell",
    ".sh": "shell",
}


def language_for_path(path: str) -> str:
    name = path.rsplit("/", 1)[-1]
    if name == "Makefile" or name.endswith(".mk"):
        return "makefile"
    suffix = Path(name).suffix
    return SUFFIX_LANGUAGE.get(suffix, "text")


def _classify_c_style(text: str) -> list[str]:
    """Per-line 'code' | 'comment' | 'blank' with // and /* */ handling.

    Double-quoted string literals are skipped when scanning for comment
    delimiters so that URLs or globs inside strings do not count as comments.
    """
    classes: list[str] = []
    in_block = False
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped:
            classes.append("blank")
            continue
        has_code = False
        has_comment = in_block
        i = 0
        while i < len(line):
            if in_block:
                end = line.find("*/", i)
                if end < 0:
                    i = len(line)
                else:
                    in_block = False
                    i = end + 2
                continue
            ch = line[i]
            if ch == '"':
                has_code = True
                i += 1
                while i < len(line):
                    if line[i] == "\\":
                        i += 2
                        continue
                    if line[i] == '"':
                        i += 1
                        break
                    i += 1
                continue
            if line.startswith("//", i):
                has_comment = True
                break
            if line.startswith("/*", i):
                has_comment = True
                in_block = True
                i += 2
                continue
            if not ch.isspace():
                has_code = True
            i += 1
        classes.append("code" if has_code else ("comment" if has_comment else "blank"))
    return classes


def _classify_xml(text: str) -> list[str]:
    classes: list[str] = []
    in_comment = False
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped:
            classes.append("blank")
            continue
        has_code = False
        has_comment = in_comment
        i = 0
        while i < len(line):
            if in_comment:
                end = line.find("-->", i)
                if end < 0:
                    i = len(line)
                else:
                    in_comment = False
                    i = end + 3
                continue
            if line.startswith("<!--", i):
                has_comment = True
                in_comment = True
                i += 4
                continue
            if not line[i].isspace():
                has_code = True
            i += 1
        classes.append("code" if has_code else ("comment" if has_comment else "blank"))
    return classes


def classify_lines(text: str, language: str) -> list[str]:
    if language in C_STYLE_LANGUAGES:
        return _classify_c_style(text)
    if language == "xml":
        return _classify_xml(text)
    classes: list[str] = []
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped:
            classes.append("blank")
        elif language in HASH_LANGUAGES and stripped.startswith("#"):
            classes.append("comment")
        else:
            classes.append("code")
    return classes


def count_lines(text: str, language: str) -> tuple[int, int, int]:
    classes = classify_lines(text, language)
    return (classes.count("code"), classes.count("comment"), classes.count("blank"))


def tally(classes: list[str]) -> dict[str, int]:
    return {
        "sloc": classes.count("code"),
        "comment_lines": classes.count("comment"),
        "blank_lines": classes.count("blank"),
    }


# --------------------------------------------------------------------------
# GUMBO span extraction
# --------------------------------------------------------------------------

GUMBO_OPEN_RE = re.compile(r'language\s+"GUMBO"\s*/\*\{')
PART_DEF_RE = re.compile(r"\bpart\s+def\s+(\w+)(.*)$")
PACKAGE_RE = re.compile(r"\b(?:library\s+)?package\s+(\w+)")


def extract_gumbo_spans(text: str) -> list[dict]:
    """Return GUMBO blocks as {start_line, end_line, owner, owner_kind} (1-based,
    inclusive, covering `language "GUMBO" /*{` through `}*/`)."""
    lines = text.splitlines()
    spans: list[dict] = []
    for match in GUMBO_OPEN_RE.finditer(text):
        start_line = text[: match.start()].count("\n") + 1
        close = text.find("}*/", match.end())
        end_line = text[: close].count("\n") + 1 if close >= 0 else len(lines)
        owner = None
        owner_kind = "unknown"
        for index in range(start_line - 1, -1, -1):
            line = lines[index]
            part = PART_DEF_RE.search(line)
            if part:
                owner = part.group(1)
                owner_kind = "system" if ":> System" in part.group(2) else "component"
                break
            package = PACKAGE_RE.search(line)
            if package:
                owner = package.group(1)
                owner_kind = "package"
                break
        spans.append({"start_line": start_line, "end_line": end_line, "owner": owner, "owner_kind": owner_kind})
    return spans


def count_gumbo_clauses(span_text: str) -> int:
    return len(re.findall(r"^\s*(?:guarantee|assume|case\s|inv\s|invariant)\b", span_text, re.MULTILINE))


# --------------------------------------------------------------------------
# Marker regions
# --------------------------------------------------------------------------

RUST_BEGIN_MARKER_RE = re.compile(r"//\s*BEGIN MARKER\b")
RUST_END_MARKER_RE = re.compile(r"//\s*END MARKER\b")
RUST_PLACEHOLDER_RE = re.compile(r"//\s*PLACEHOLDER MARKER\b")
XML_CONTENT_BEGIN_RE = re.compile(r"<!--\s*BEGIN (?:MSD )?CONTENT MARKER\b")
XML_CONTENT_END_RE = re.compile(r"<!--\s*END (?:MSD )?CONTENT MARKER\b")


def rust_marker_line_flags(lines: list[str]) -> list[bool]:
    """True for lines inside BEGIN/END MARKER regions (inclusive) or on a
    PLACEHOLDER MARKER line: HAMR-woven content in generate-once Rust files."""
    flags = [False] * len(lines)
    in_region = False
    for index, line in enumerate(lines):
        if in_region:
            flags[index] = True
            if RUST_END_MARKER_RE.search(line):
                in_region = False
            continue
        if RUST_BEGIN_MARKER_RE.search(line):
            flags[index] = True
            in_region = True
        elif RUST_PLACEHOLDER_RE.search(line):
            flags[index] = True
    return flags


def xml_content_line_flags(lines: list[str]) -> list[bool]:
    """True for lines INSIDE content-marker regions (exclusive of delimiters):
    developer-preserved content in microkit.system."""
    flags = [False] * len(lines)
    in_region = False
    for index, line in enumerate(lines):
        if in_region and XML_CONTENT_END_RE.search(line):
            in_region = False
            continue
        if in_region:
            flags[index] = True
            continue
        if XML_CONTENT_BEGIN_RE.search(line):
            in_region = True
    return flags


# --------------------------------------------------------------------------
# Codegen report parsing and validation (report is evidence, not gospel)
# --------------------------------------------------------------------------

def parse_codegen_report(path: Path) -> dict:
    data = json.loads(path.read_text(encoding="utf-8"))
    if data.get("type") != "CodegenReports":
        raise MeasureError(f"{path}: unexpected top-level type {data.get('type')!r}")
    entries = data.get("reports", {}).get("entries")
    if not isinstance(entries, list):
        raise MeasureError(f"{path}: reports.entries is not a list")
    tool = None
    microkit = None
    for entry in entries:
        if not (isinstance(entry, list) and len(entry) == 2):
            raise MeasureError(f"{path}: malformed reports entry")
        key, value = entry
        if key == "KEY_TOOL_REPORT":
            tool = value
        elif key == "MicrokitReporterPlugin":
            microkit = value
    if tool is None:
        raise MeasureError(f"{path}: KEY_TOOL_REPORT entry missing")
    resources_raw = tool.get("resources", [])
    deduped: dict[str, bool] = {}
    for resource in resources_raw:
        rel = resource.get("path")
        flag = resource.get("overwrittenIfExists")
        if rel is None or flag is None:
            raise MeasureError(f"{path}: malformed ResourceReport {resource!r}")
        if rel in deduped and deduped[rel] != flag:
            raise MeasureError(
                f"{path}: duplicate resource {rel!r} with conflicting overwrittenIfExists flags"
            )
        deduped[rel] = flag
    components: list[dict] = []
    if microkit is not None:
        for entry in microkit.get("componentReport", {}).get("entries", []):
            id_path = entry[0].get("idPath", [])
            if len(id_path) >= 3:
                process, thread = id_path[-2], id_path[-1]
                components.append(
                    {"id_path": id_path, "process": process, "thread": thread, "crate": f"{process}_{thread}"}
                )
    return {
        "path": path,
        "command_line_args": tool.get("commandLineArgs"),
        "internal_status": tool.get("status", {}).get("value"),
        "warning_messages": tool.get("warningMessages", []),
        "error_messages": tool.get("errorMessages", []),
        "resource_count_raw": len(resources_raw),
        "resources": deduped,
        "components": components,
    }


ORPHAN_WHITELIST_PREFIXES = ("attestation/", "bin/", "reporting/", "build/", ".claude/")
ORPHAN_WHITELIST_NAMES = {"Cargo.lock", ".DS_Store"}
ORPHAN_WHITELIST_DIR_PARTS = {"target", "out"}


def _whitelisted(rel: str) -> bool:
    if rel.startswith(ORPHAN_WHITELIST_PREFIXES):
        return True
    parts = rel.split("/")
    if parts[-1] in ORPHAN_WHITELIST_NAMES:
        return True
    return any(part in ORPHAN_WHITELIST_DIR_PARTS for part in parts[:-1])


def walk_files(root: Path) -> list[str]:
    found: list[str] = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames.sort()
        # never descend into build output; it can be huge
        dirnames[:] = [d for d in dirnames if d not in ORPHAN_WHITELIST_DIR_PARTS and d != ".git"]
        for filename in sorted(filenames):
            found.append(str(Path(dirpath, filename).relative_to(root)))
    return found


def validate_report(hamr_dir: Path, report: dict) -> dict:
    resources = report["resources"]
    escapes = []
    missing = []
    for rel in sorted(resources):
        target = (hamr_dir / rel).resolve()
        if hamr_dir.resolve() not in target.parents and target != hamr_dir.resolve():
            escapes.append(rel)
            continue
        if not target.is_file():
            missing.append(rel)
    on_disk = walk_files(hamr_dir)
    orphans = [rel for rel in on_disk if rel not in resources and not _whitelisted(rel)]
    status = "complete"
    problems: list[str] = []
    if escapes:
        status = "partial"
        problems.append(f"{len(escapes)} reported path(s) escape the output tree: {escapes}")
    if missing:
        status = "partial"
        problems.append(f"{len(missing)} reported file(s) missing on disk: {missing}")
    if orphans:
        status = "partial"
        problems.append(f"{len(orphans)} generated-looking file(s) on disk are absent from the report")
    return {"report_status": status, "orphaned_files": orphans, "missing": missing, "problems": problems}


# --------------------------------------------------------------------------
# Path rules
# --------------------------------------------------------------------------

def detect_model_dir(project: Path) -> str | None:
    """The subdirectory of sysmlv2/ (or sysml/) actually containing .sysml
    files, excluding aadl-lib/. Never trusts docs."""
    for base_name in ("sysmlv2", "sysml"):
        base = project / base_name
        if not base.is_dir():
            continue
        candidates = []
        for child in sorted(base.iterdir()):
            if not child.is_dir() or child.name == "aadl-lib":
                continue
            if any(child.rglob("*.sysml")):
                candidates.append(f"{base_name}/{child.name}")
        if candidates:
            return candidates[0]
    return None


def detect_proof_crates(hamr_dir: Path) -> list[dict]:
    """Proof crates matched by structure (src/ contains vc_*.rs), never by
    documented name."""
    crates_dir = hamr_dir / "crates"
    results = []
    if not crates_dir.is_dir():
        return results
    for crate in sorted(crates_dir.iterdir()):
        src = crate / "src"
        if not src.is_dir():
            continue
        vc_files = [p for p in src.rglob("vc_*.rs") if "target" not in p.parts]
        if not vc_files:
            continue
        rs_files = [p for p in src.rglob("*.rs") if "target" not in p.parts]
        properties = sorted(
            {p.parent.name for p in vc_files if p.parent != src}
        )
        total_lines = sum(len(p.read_text(encoding="utf-8").splitlines()) for p in rs_files)
        results.append(
            {
                "crate": crate.name,
                "rs_files": len(rs_files),
                "total_lines": total_lines,
                "properties": properties,
                "property_count": len(properties),
            }
        )
    return results


def artifact_rules_for_hamr_path(rel: str, crate_components: dict[str, str], proof_crate_names: set[str]) -> tuple[str, str | None, str | None]:
    """Return (artifact_type, artifact_id, component) for a path under hamr/microkit/."""
    parts = rel.split("/")
    name = parts[-1]
    component = None
    if parts[0] == "crates" and len(parts) > 1:
        crate = parts[1]
        component = crate_components.get(crate)
        if crate in proof_crate_names:
            if name.endswith(".rs"):
                return "proof_code", "SysProofVCs", None
            if name == "Makefile":
                return "makefile", "SysProofVCs", None
            return "build_config", "SysProofVCs", None
        if name == "Makefile":
            return "makefile", "GenCode", component
        if name.endswith(".toml"):
            return "build_config", "GenCode", component
        if crate == "data":
            return "rust_infra", "GenCode", None
        if "component" in parts and name.endswith("_app.rs"):
            return "rust_app_code", "CompImpls", component
        if "component" in parts:
            return "rust_infra", "GenCode", component
        if "test" in parts and name == "tests.rs":
            return "rust_test_code", "CompTests", component
        if "test" in parts:
            return "rust_test_infra", "GenCode", component
        return "rust_infra", "GenCode", component
    if parts[0] in ("components", "types", "util"):
        if parts[0] == "components" and len(parts) > 1:
            component = crate_components.get(parts[1].removesuffix("_MON"))
        if name.endswith("_user.c"):
            return "c_user_code", "GenCode", component
        return "c_infra", "GenCode", component
    if rel == "bin/build.cmd":
        return "build_script", "BuildScript", None
    if name == "microkit.schedule.xml":
        return "microkit_config", "Schedule", None
    if name in ("microkit.system",) or name.endswith(".dot"):
        return "microkit_config", "GenCode", None
    if name == "Makefile" or name.endswith(".mk"):
        return "makefile", "GenCode", None
    return "docs" if name.endswith(".md") else "rust_infra" if name.endswith(".rs") else "c_infra" if name.endswith((".c", ".h")) else "microkit_config" if name.endswith(".xml") else "docs", None, component


REQUIREMENTS_ARTIFACTS = {
    "conops.md": "ConOps",
    "requirements.md": "SysReqs",
    "component-requirements.md": "CompReqs",
    "data-dictionary.md": "DataDict",
}


# --------------------------------------------------------------------------
# Step mapping
# --------------------------------------------------------------------------

def load_step_map(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def step_for_artifact(step_map: dict, artifact_id: str | None, component: str | None) -> dict:
    if artifact_id is None:
        entry = step_map["unattributed"]
        return {"key": entry["step"], "granularity": entry["granularity"]}
    entry = step_map["artifacts"].get(artifact_id)
    if entry is None:
        fallback = step_map["unattributed"]
        return {"key": fallback["step"], "granularity": fallback["granularity"]}
    key = entry["step"]
    if entry.get("parameterized"):
        key = key.replace("<component>", component if component else "*")
    return {"key": key, "granularity": entry["granularity"]}


# --------------------------------------------------------------------------
# Template comparison (§5.3) — exact-match conservative
# --------------------------------------------------------------------------

def compare_with_template(final_lines: list[str], final_classes: list[str], final_nonmarker: list[bool],
                          template_lines: list[str], template_classes: list[str], template_nonmarker: list[bool]) -> dict:
    """Partition the final file's non-marker lines into retained (present in the
    template's non-marker multiset) vs developer-authored. Exact match after
    trailing-whitespace strip; blank lines follow the retained bucket."""
    template_pool: dict[str, int] = {}
    supplied = 0
    for line, cls, keep in zip(template_lines, template_classes, template_nonmarker):
        if not keep or cls == "blank":
            continue
        key = line.rstrip()
        template_pool[key] = template_pool.get(key, 0) + 1
        if cls == "code":
            supplied += 1
    retained = {"sloc": 0, "comment_lines": 0, "blank_lines": 0}
    developer = {"sloc": 0, "comment_lines": 0, "blank_lines": 0}
    retained_indices: list[int] = []
    developer_indices: list[int] = []
    retained_code = 0
    for index, (line, cls, keep) in enumerate(zip(final_lines, final_classes, final_nonmarker)):
        if not keep:
            continue
        if cls == "blank":
            retained["blank_lines"] += 1
            retained_indices.append(index)
            continue
        key = line.rstrip()
        if template_pool.get(key, 0) > 0:
            template_pool[key] -= 1
            bucket = retained
            retained_indices.append(index)
            if cls == "code":
                retained_code += 1
        else:
            bucket = developer
            developer_indices.append(index)
        bucket["sloc" if cls == "code" else "comment_lines"] += 1
    return {
        "retained": retained,
        "developer": developer,
        "retained_indices": retained_indices,
        "developer_indices": developer_indices,
        "accounting": {"supplied": supplied, "retained": retained_code, "replaced": supplied - retained_code},
    }


# --------------------------------------------------------------------------
# Workflow status parsing
# --------------------------------------------------------------------------

ROW_RE = re.compile(r"^\|\s*([^|]+?)\s*\|\s*([^|]*?)\s*\|\s*([^|]*?)\s*\|\s*(.*?)\s*\|\s*$")
KEY_ALIAS_RE = re.compile(r"^(\S+)(?:\s+\(([^)]+)\))?")


def parse_workflow_status(text: str) -> dict:
    rows: list[dict] = []
    for line in text.splitlines():
        match = ROW_RE.match(line)
        if not match:
            continue
        key_raw, status, updated, notes = match.groups()
        if key_raw.lower() == "step" or set(key_raw) <= {"-"}:
            continue
        key_match = KEY_ALIAS_RE.match(key_raw)
        key = key_match.group(1) if key_match else key_raw
        alias = key_match.group(2) if key_match else None
        alias_key = alias.split(",")[0].strip().split(" ")[0] if alias else None
        rows.append(
            {
                "key": key,
                "alias": alias_key,
                "status": status.strip().lower(),
                "updated": updated.strip(),
                "notes": notes.strip(),
            }
        )
    profile = None
    profile_match = re.search(r"Profile:\s*(\w+)", text)
    if profile_match:
        profile = profile_match.group(1)
    return {"profile": profile, "rows": rows}


def _norm_key(key: str) -> str:
    return key.lower()


def _expand_row_keys(row: dict) -> list[str]:
    keys = [row["key"]]
    if row["alias"]:
        keys.append(row["alias"])
    expanded: list[str] = []
    for key in keys:
        expanded.append(key)
        range_match = re.match(r"^(.*)\.([A-Za-z0-9]+)-([A-Za-z0-9]+)$", key)
        if range_match:
            base, first, second = range_match.groups()
            expanded.extend([f"{base}.{first}", f"{base}.{second}"])
    return expanded


def build_status_index(status: dict, components: list[str]) -> dict[str, dict]:
    index: dict[str, dict] = {}
    for row in status["rows"]:
        for key in _expand_row_keys(row):
            wildcard = re.match(r"^([A-Za-z]+)\(\*\)(.*)$", key)
            if wildcard and components:
                for component in components:
                    index.setdefault(_norm_key(f"{wildcard.group(1)}({component}){wildcard.group(2)}"), row)
            else:
                index.setdefault(_norm_key(key), row)
    return index


STATUS_RANK = {"done": 5, "waived": 4, "in-progress": 3, "blocked": 2, "n/a": 1, "not-started": 0}


def _normalize_status(raw: str) -> str:
    return raw.split("/")[0].strip()


def status_for(index: dict[str, dict], key: str) -> str | None:
    row = index.get(_norm_key(key))
    return _normalize_status(row["status"]) if row else None


WORKFLOWS = [
    "SysPlanAndReq",
    "SysModeling",
    "CompGUMBOSpec",
    "SysGUMBOIntegrationCheck",
    "CodeGen",
    "CompDev",
    "SysSchedDef",
    "SysGUMBOSysSpecCheck",
    "SysPlanMod",
    "SysDevAll",
]


def derive_workflow_statuses(status: dict, components: list[str]) -> dict[str, str]:
    """Best-ranked status across all of a workflow's rows (whole-workflow rows,
    parenthesized aliases, instance rows, and numbered step rows). Workflow-status
    files sometimes leave a composite's sub-workflow row stuck at in-progress
    while every step row is done; a single done step row is evidence the
    workflow ran, and `done` outranks stale in-progress markers."""
    result: dict[str, str] = {}
    for workflow in WORKFLOWS:
        best: str | None = None
        for row in status["rows"]:
            for key in _expand_row_keys(row):
                base = re.match(r"^([A-Za-z]+)", key)
                if not base or base.group(1).lower() != workflow.lower():
                    continue
                value = _normalize_status(row["status"])
                if best is None or STATUS_RANK.get(value, 0) > STATUS_RANK.get(best, 0):
                    best = value
        result[workflow] = best or "not-started"
    return result


# --------------------------------------------------------------------------
# Assurance collection (§8) — workflow-status Notes are the Wave 1 workhorse
# --------------------------------------------------------------------------

VERIFIED_RE = re.compile(r"(\d+)\s+verified,\s*(\d+)\s+errors?")
TESTS_SLASH_RE = re.compile(r"(\d+)\s*/\s*(\d+)(?:\s+tests)?\s+pass")
TESTS_PASSED_RE = re.compile(r"(\d+)\s+passed(?:,\s*(\d+)\s+failed)?")
TESTS_PASS_RE = re.compile(r"(\d+)\s+tests\s+pass")
HANDSHAKES_RE = re.compile(r"handshakes[^0-9]{0,6}(\d+)")


def _sourced(value, source: str, basis: str, confidence: str, note: str | None = None) -> dict:
    result = {"value": value, "source": source, "classification_basis": basis, "confidence": confidence}
    if note:
        result["note"] = note
    return result


def collect_assurance(project: Path, hamr_dir: Path, status: dict, index: dict[str, dict],
                      components: list[str], crate_of: dict[str, str], proof_crates: list[dict],
                      report_info: dict | None) -> dict:
    ws_source = "reports/workflow-status.md"

    tests_static: dict[str, dict] = {}
    for component, crate in sorted(crate_of.items()):
        counts = {}
        for label, rel in (("tests_rs", f"crates/{crate}/src/test/tests.rs"),
                           ("cb_apis_rs", f"crates/{crate}/src/test/util/cb_apis.rs")):
            path = hamr_dir / rel
            counts[label] = len(re.findall(r"#\[test\]", path.read_text(encoding="utf-8"))) if path.is_file() else 0
        tests_static[component] = {
            **counts,
            "total": counts["tests_rs"] + counts["cb_apis_rs"],
            "source": "static #[test] count incl. cb_apis.rs macro tests",
            "classification_basis": "static_count",
            "confidence": "exact",
        }

    tests_runtime: dict[str, dict] = {}
    coverage: dict[str, dict] = {}
    component_verus: dict[str, dict] = {}
    for component in components:
        run_row = index.get(_norm_key(f"CompDev({component}).4"))
        top_row = index.get(_norm_key(f"CompDev({component})"))
        passed = failed = None
        note_text = ""
        for row in (run_row, top_row):
            if row is None:
                continue
            note_text = row["notes"]
            slash = TESTS_SLASH_RE.search(note_text)
            if slash:
                passed, failed = int(slash.group(1)), int(slash.group(2)) - int(slash.group(1))
                break
            plain = TESTS_PASSED_RE.search(note_text)
            if plain:
                passed = int(plain.group(1))
                failed = int(plain.group(2) or 0)
                break
            pass_count = TESTS_PASS_RE.search(note_text)
            if pass_count:
                passed, failed = int(pass_count.group(1)), 0
                break
        tests_runtime[component] = _sourced(
            {"passed": passed, "failed": failed},
            f"{ws_source} CompDev({component}).4", "workflow_status_note", "derived",
            note=note_text or None,
        )
        cov_notes = []
        for suffix in (".4", ".5", ""):
            row = index.get(_norm_key(f"CompDev({component}){suffix}"))
            if row and row["notes"]:
                cov_notes.append(row["notes"])
        cov_text = " | ".join(cov_notes)
        full = bool(re.search(r"100%|fully covered", cov_text))
        coverage[component] = _sourced(
            {"entrypoint_lines_percent": 100 if full else None, "summary": cov_text or None},
            f"{ws_source} CompDev({component}).4/.5", "workflow_status_note", "derived",
        )
        verus_row = index.get(_norm_key(f"CompDev({component}).6"))
        verified = errors = None
        if verus_row:
            verus_match = VERIFIED_RE.search(verus_row["notes"])
            if verus_match:
                verified, errors = int(verus_match.group(1)), int(verus_match.group(2))
        component_verus[component] = _sourced(
            {"verified": verified, "errors": errors},
            f"{ws_source} CompDev({component}).6", "workflow_status_note", "derived",
        )

    proof_row = index.get(_norm_key("SysGUMBOSysSpecCheck.5"))
    system_proof: dict = {"structure": None}
    verified = errors = None
    if proof_row:
        proof_match = VERIFIED_RE.search(proof_row["notes"])
        if proof_match:
            verified, errors = int(proof_match.group(1)), int(proof_match.group(2))
    system_proof["results"] = _sourced(
        {"verified": verified, "errors": errors},
        f"{ws_source} SysGUMBOSysSpecCheck.5", "workflow_status_note", "derived",
        note=None if proof_row else "no SysGUMBOSysSpecCheck.5 row (step deferred or absent)",
    )
    if proof_crates:
        system_proof["structure"] = _sourced(
            proof_crates, "proof-crate inventory (structural match on vc_*.rs)", "path_rule", "exact",
        )

    handshakes = None
    vacuous = False
    integration_note = None
    for row in status["rows"]:
        joined = " ".join(_expand_row_keys(row)).lower()
        if "sysgumbointegrationcheck" in joined:
            match = HANDSHAKES_RE.search(row["notes"])
            if match:
                handshakes = int(match.group(1))
                integration_note = row["notes"]
            if "vacuous" in row["notes"].lower():
                vacuous = True
                integration_note = integration_note or row["notes"]
    integration = _sourced(
        {"expected_handshakes": handshakes, "vacuous": vacuous},
        f"{ws_source} SysGUMBOIntegrationCheck rows", "workflow_status_note", "derived",
        note=integration_note,
    )

    audit_findings: dict = {"present": False}
    assessment_path = project / "experiment-reports" / "assessment-summary.json"
    if assessment_path.is_file():
        try:
            assessment = json.loads(assessment_path.read_text(encoding="utf-8"))
            by_severity: dict[str, int] = {}
            for finding in assessment.get("findings", []):
                severity = finding.get("severity", "unknown")
                by_severity[severity] = by_severity.get(severity, 0) + 1
            audit_findings = {
                "present": True,
                **_sourced(
                    {"total": len(assessment.get("findings", [])), "by_severity": by_severity},
                    "experiment-reports/assessment-summary.json", "codegen_report", "exact",
                ),
            }
        except (json.JSONDecodeError, OSError) as error:
            audit_findings = {"present": False, "error": str(error)}
    else:
        audit_findings["note"] = "no assessment-summary.json for this experiment"

    contract_audit: dict[str, dict] = {}
    for component in components:
        row = index.get(_norm_key(f"CompGUMBOSpec({component}).3"))
        findings = None
        if row:
            zero = re.search(r"zero findings|0 findings", row["notes"], re.IGNORECASE)
            count_match = re.search(r"(\d+)\s+(?:Low|Moderate|High|finding)", row["notes"], re.IGNORECASE)
            if zero:
                findings = 0
            elif "AP-" in row["notes"] and count_match:
                findings = int(count_match.group(1))
            elif count_match:
                findings = int(count_match.group(1))
        contract_audit[component] = _sourced(
            {"findings": findings},
            f"{ws_source} CompGUMBOSpec({component}).3", "workflow_status_note", "derived",
            note=row["notes"] if row else None,
        )

    generated_resources = None
    if report_info:
        generated_resources = _sourced(
            {
                "raw": report_info["resource_count_raw"],
                "deduped": len(report_info["resources"]),
                "overwritten_true": sum(1 for flag in report_info["resources"].values() if flag),
                "overwritten_false": sum(1 for flag in report_info["resources"].values() if not flag),
            },
            "hamr/microkit/reporting/codegen_report_sysml.json (validated)", "codegen_report", "exact",
        )

    reconciliation = {}
    for component in components:
        static_total = tests_static.get(component, {}).get("total")
        runtime = tests_runtime.get(component, {}).get("value", {}).get("passed")
        reconciliation[component] = {
            "static": static_total,
            "runtime_passed": runtime,
            "consistent": (static_total == runtime) if (static_total is not None and runtime is not None) else None,
        }

    return {
        "tests": {"defined_static": tests_static, "runtime": tests_runtime, "reconciliation": reconciliation},
        "coverage": coverage,
        "component_verus": component_verus,
        "system_proof": system_proof,
        "integration_check": integration,
        "audit_findings": audit_findings,
        "contract_audit_findings": contract_audit,
        "generated_resources": generated_resources or {"note": "codegen report absent"},
        "structured_sources": {
            "verus_results_json": None,
            "lcov": None,
            "note": "no structured Verus or coverage outputs exist in Wave 1 subjects; schema hooks reserved",
        },
    }


# --------------------------------------------------------------------------
# Measurement driver
# --------------------------------------------------------------------------

def _bucket(artifact_type, generation_mode, provenance, basis, confidence, workflow, counts,
            artifact_id=None, component=None, items=None) -> dict:
    return {
        "artifact_type": artifact_type,
        "artifact_id": artifact_id,
        "component": component,
        "generation_mode": generation_mode,
        "provenance": provenance,
        "classification_basis": basis,
        "confidence": confidence,
        "workflow": workflow,
        "sloc": counts["sloc"],
        "comment_lines": counts["comment_lines"],
        "blank_lines": counts["blank_lines"],
        "items": items,
    }


def _tally_subset(classes: list[str], flags: list[bool]) -> dict[str, int]:
    subset = [cls for cls, keep in zip(classes, flags) if keep]
    return {"sloc": subset.count("code"), "comment_lines": subset.count("comment"), "blank_lines": subset.count("blank")}


def _indices_where(flags: list[bool]) -> list[int]:
    return [index for index, value in enumerate(flags) if value]


def _compress_ranges(indices: list[int]) -> list[list[int]]:
    """Sorted 0-based line indices -> 1-based inclusive [start, end] ranges."""
    ranges: list[list[int]] = []
    for index in sorted(indices):
        line = index + 1
        if ranges and ranges[-1][1] == line - 1:
            ranges[-1][1] = line
        else:
            ranges.append([line, line])
    return ranges


def _finalize_file(files: list, line_details: dict, path_str: str, language: str,
                   classes: list[str], pairs: list[tuple[dict, list[int]]],
                   filter_empty: bool) -> None:
    """Record a file's buckets plus the parallel per-bucket line-index lists."""
    kept = pairs
    if filter_empty:
        kept = [(bucket, indices) for bucket, indices in pairs
                if bucket["sloc"] or bucket["comment_lines"] or bucket["blank_lines"]]
        if not kept:
            kept = pairs[:1]
    files.append({"path": path_str, "language": language, "buckets": [bucket for bucket, _ in kept]})
    line_details[path_str] = {"classes": classes, "allocations": [indices for _, indices in kept]}


def build_line_allocation(files: list, line_details: dict, now: str, project_info: dict) -> dict:
    """Line-range detail per bucket, validated: for every file the per-bucket
    ranges exactly partition lines 1..N, and per-bucket class counts over the
    owned lines equal the counts recorded in project-measures."""
    allocation_files = []
    for entry in files:
        detail = line_details[entry["path"]]
        classes = detail["classes"]
        line_count = len(classes)
        seen: list[int] = []
        bucket_entries = []
        for position, (bucket, indices) in enumerate(zip(entry["buckets"], detail["allocations"])):
            seen.extend(indices)
            counts = {"code": 0, "comment": 0, "blank": 0}
            for index in indices:
                counts[classes[index]] += 1
            if (counts["code"], counts["comment"], counts["blank"]) != (
                bucket["sloc"], bucket["comment_lines"], bucket["blank_lines"]
            ):
                raise MeasureError(
                    f"line allocation counts disagree with bucket counts for {entry['path']} bucket {position}: "
                    f"{counts} vs {bucket['sloc']}/{bucket['comment_lines']}/{bucket['blank_lines']}"
                )
            bucket_entries.append({
                "bucket_index": position,
                "artifact_type": bucket["artifact_type"],
                "provenance": bucket["provenance"],
                "generation_mode": bucket["generation_mode"],
                "workflow_key": bucket["workflow"]["key"],
                "component": bucket["component"],
                "ranges": _compress_ranges(indices),
                "sloc": bucket["sloc"],
                "comment_lines": bucket["comment_lines"],
                "blank_lines": bucket["blank_lines"],
            })
        if sorted(seen) != list(range(line_count)):
            raise MeasureError(
                f"line allocation does not partition {entry['path']}: "
                f"{len(seen)} owned line(s) vs {line_count} in file"
            )
        allocation_files.append({
            "path": entry["path"],
            "language": entry["language"],
            "line_count": line_count,
            "buckets": bucket_entries,
        })
    return {
        "schema_version": 1,
        "generated": now,
        "project": project_info,
        "measures_ref": None,
        "files": allocation_files,
    }


class BaselineSource:
    """Template lookup from a prepared pristine-generation directory or a git ref."""

    def __init__(self, baseline_dir: Path | None, baseline_ref: str | None, project: Path):
        self.baseline_dir = baseline_dir
        self.baseline_ref = baseline_ref
        self.project = project

    def template_text(self, hamr_rel: str) -> str | None:
        if self.baseline_dir is not None:
            candidate = self.baseline_dir / "hamr" / "microkit" / hamr_rel
            if candidate.is_file():
                return candidate.read_text(encoding="utf-8")
            return None
        if self.baseline_ref is not None:
            rel_to_repo = (self.project / "hamr" / "microkit" / hamr_rel).resolve()
            result = subprocess.run(
                ["git", "-C", str(self.project), "show", f"{self.baseline_ref}:./{Path('hamr/microkit') / hamr_rel}"],
                capture_output=True, text=True, check=False,
            )
            _ = rel_to_repo
            if result.returncode == 0:
                return result.stdout
            return None
        return None


def measure(project: Path, manifest_path: Path, step_map_path: Path,
            baseline_dir: Path | None, baseline_ref: str | None, now: str) -> dict:
    caveats: list[str] = []
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    quote = manifest.get("concept_quote", "")
    digest = hashlib.sha256(quote.encode("utf-8")).hexdigest()
    if manifest.get("concept_sha256") and manifest["concept_sha256"] != digest:
        caveats.append("manifest concept_sha256 does not match the concept_quote text")

    step_map = load_step_map(step_map_path)

    hamr_dir = project / "hamr" / "microkit"
    report_path = hamr_dir / "reporting" / "codegen_report_sysml.json"
    report_info = None
    validation = {"report_status": "absent", "orphaned_files": [], "missing": [], "problems": []}
    if report_path.is_file():
        report_info = parse_codegen_report(report_path)
        validation = validate_report(hamr_dir, report_info)
        if report_info["resource_count_raw"] != len(report_info["resources"]):
            caveats.append(
                f"codegen report contains {report_info['resource_count_raw'] - len(report_info['resources'])} duplicate resource path(s); deduplicated (flags agreed)"
            )
        if validation["report_status"] == "partial":
            caveats.append(
                "codegen report is PARTIAL: internal status "
                f"{report_info['internal_status']!r} is not trusted as a completeness signal; "
                + "; ".join(validation["problems"])
            )
    else:
        caveats.append("codegen report absent; full marker/path-rule fallback classification")

    classification_mode = (
        "report" if validation["report_status"] == "complete"
        else "report_plus_fallback" if validation["report_status"] == "partial"
        else "fallback_only"
    )

    model_dir = detect_model_dir(project)
    proof_crates = detect_proof_crates(hamr_dir) if hamr_dir.is_dir() else []
    proof_crate_names = {entry["crate"] for entry in proof_crates}

    components: list[str] = []
    crate_of: dict[str, str] = {}
    crate_components: dict[str, str] = {}
    if report_info:
        for component in report_info["components"]:
            components.append(component["thread"])
            crate_of[component["thread"]] = component["crate"]
            crate_components[component["crate"]] = component["thread"]
    if not components and hamr_dir.is_dir():
        for crate in sorted((hamr_dir / "crates").iterdir()) if (hamr_dir / "crates").is_dir() else []:
            if crate.name in ("data", *proof_crate_names) or not crate.is_dir():
                continue
            thread = crate.name.split("_", 1)[-1]
            components.append(thread)
            crate_of[thread] = crate.name
            crate_components[crate.name] = thread
        if components:
            caveats.append("component identity derived from crate names only (no componentReport)")
    components.sort()

    # ---- workflow status
    status_path = project / "reports" / "workflow-status.md"
    status = parse_workflow_status(status_path.read_text(encoding="utf-8")) if status_path.is_file() else {"profile": None, "rows": []}
    index = build_status_index(status, components)
    workflow_statuses = derive_workflow_statuses(status, components)

    # ---- baseline
    baseline_info = None
    baseline_source = None
    baseline_usable = False
    if baseline_dir is not None or baseline_ref is not None:
        baseline_source = BaselineSource(baseline_dir, baseline_ref, project)
        provenance_data = None
        agreement = None
        if baseline_dir is not None:
            provenance_path = baseline_dir / "baseline-provenance.json"
            if provenance_path.is_file():
                provenance_data = json.loads(provenance_path.read_text(encoding="utf-8"))
            if report_info:
                compared = identical = 0
                differing: list[str] = []
                for rel, flag in sorted(report_info["resources"].items()):
                    if not flag:
                        continue
                    final_file = hamr_dir / rel
                    base_file = baseline_dir / "hamr" / "microkit" / rel
                    if not final_file.is_file():
                        continue
                    compared += 1
                    if base_file.is_file() and base_file.read_bytes() == final_file.read_bytes():
                        identical += 1
                    else:
                        differing.append(rel)
                fraction = (identical / compared) if compared else 0.0
                agreement = {
                    "overwrite_true_compared": compared,
                    "identical": identical,
                    "fraction": round(fraction, 4),
                    "threshold": BASELINE_AGREEMENT_THRESHOLD,
                    "differing_sample": differing[:10],
                }
                baseline_usable = fraction >= BASELINE_AGREEMENT_THRESHOLD
                if not baseline_usable:
                    caveats.append(
                        f"generation baseline agreement {fraction:.2%} below threshold; treated as version mismatch — "
                        "generate-once files fall back to preserved_origin_unknown"
                    )
                elif fraction < 1.0:
                    caveats.append(
                        f"generation baseline agrees on {identical}/{compared} regenerated files; "
                        f"differing files degrade matching conservatively: {differing[:5]}"
                    )
            else:
                baseline_usable = True
        else:
            baseline_usable = True
            caveats.append("baseline from git ref; no regeneration-agreement check possible")
        if provenance_data and manifest.get("hamr_version") is None:
            caveats.append(
                "original run's toolchain version unknown; baseline generated with "
                f"{provenance_data.get('sireum_version', 'unknown version')}"
            )
        baseline_info = {
            "present": True,
            "path": str(baseline_dir) if baseline_dir else None,
            "ref": baseline_ref,
            "provenance": provenance_data,
            "agreement": agreement,
            "usable": baseline_usable,
        }
    else:
        baseline_info = {"present": False, "path": None, "provenance": None, "agreement": None, "usable": False}
        caveats.append(
            "no generation baseline supplied; non-marker content of generate-once files is preserved_origin_unknown"
        )

    files: list[dict] = []
    line_details: dict[str, dict] = {}
    template_accounting: list[dict] = []
    excluded: list[dict] = []

    def exclude_dir(rel: str, reason: str) -> None:
        target = project / rel
        if target.is_dir():
            count = sum(len(filenames) for _, _, filenames in os.walk(target))
            if count:
                excluded.append({"path": rel + "/", "reason": reason, "file_count": count})
        elif target.is_file():
            excluded.append({"path": rel, "reason": reason, "file_count": 1})

    # ---- project-level exclusions (§4 last row + meta), listed with reasons
    exclude_dir("sysmlv2/aadl-lib", "HAMR/AADL shared library, not project-authored")
    exclude_dir("reports", "process/audit documentation; excluded from effort modeling (conservative)")
    exclude_dir("experiment-reports", "experiment meta-artifacts, not system development work")
    exclude_dir("action-requests", "change-request sketches (empty in greenfield subjects)")
    exclude_dir(".slang", "tool scratch")
    exclude_dir(".claude", "harness configuration")
    for meta in ("AGENTS.md", "CLAUDE.md", ".mcp.json", ".gitignore"):
        exclude_dir(meta, "harness/repository metadata, not system work product")

    # ---- requirements
    requirements_dir = project / "requirements"
    if requirements_dir.is_dir():
        for path in sorted(requirements_dir.glob("*.md")):
            text = path.read_text(encoding="utf-8")
            classes = classify_lines(text, "markdown")
            artifact_id = REQUIREMENTS_ARTIFACTS.get(path.name)
            ids = sorted(set(re.findall(r"\b[A-Z]{2,8}-\d{3}\b", text)))
            bucket = _bucket(
                "requirements_text", "not_codegen", "developer_authored", "path_rule", "exact",
                step_for_artifact(step_map, artifact_id, None), tally(classes),
                artifact_id=artifact_id,
                items={"requirement_ids": len(ids)} if ids else None,
            )
            _finalize_file(files, line_details, f"requirements/{path.name}", "markdown",
                           classes, [(bucket, list(range(len(classes))))], filter_empty=False)

    # ---- model dir (.sysml): GUMBO spans vs model remainder
    if model_dir:
        for path in sorted((project / model_dir).rglob("*.sysml")):
            rel = str(path.relative_to(project))
            text = path.read_text(encoding="utf-8")
            lines = text.splitlines()
            classes = classify_lines(text, "sysml")
            spans = extract_gumbo_spans(text)
            gumbo_flags = [False] * len(lines)
            span_pairs: list[tuple[dict, list[int]]] = []
            for span in spans:
                # Interior lines only: the `language "GUMBO" /*{` and `}*/` wrapper
                # lines are embedding syntax (counted as 2 SLOC); counting the
                # interior inside the block comment would misread contracts as comments.
                interior = lines[span["start_line"]: span["end_line"] - 1]
                span_text = "\n".join(interior)
                # splitlines() drops the final empty item after join; retain it so
                # per-span counts and the file-level line allocation stay identical.
                if interior and interior[-1] == "":
                    span_text += "\n"
                interior_classes = classify_lines(span_text, "gumbo")
                # Overlay per-line classes: wrapper lines are code; interior lines
                # take the GUMBO classification.
                classes[span["start_line"] - 1] = "code"
                classes[span["end_line"] - 1] = "code"
                for offset, cls in enumerate(interior_classes):
                    classes[span["start_line"] + offset] = cls
                span_indices = list(range(span["start_line"] - 1, span["end_line"]))
                for line_number in span_indices:
                    gumbo_flags[line_number] = True
                span_classes = interior_classes + ["code", "code"]  # the two wrapper lines
                owner = span["owner"]
                component = owner.lower() if owner and span["owner_kind"] == "component" else None
                if span["owner_kind"] == "system":
                    artifact_id = "SysSpecs"
                    component = None
                else:
                    artifact_id = "CompContracts"
                span_pairs.append((
                    _bucket(
                        "gumbo_contract", "not_codegen", "developer_authored", "path_rule", "exact",
                        step_for_artifact(step_map, artifact_id, component),
                        {"sloc": span_classes.count("code"), "comment_lines": span_classes.count("comment"), "blank_lines": span_classes.count("blank")},
                        artifact_id=artifact_id, component=component,
                        items={"clauses": count_gumbo_clauses(span_text), "owner": owner},
                    ),
                    span_indices,
                ))
            model_indices = _indices_where([not flag for flag in gumbo_flags])
            model_counts = _tally_subset(classes, [not flag for flag in gumbo_flags])
            model_pair = (
                _bucket(
                    "sysml_model", "not_codegen", "developer_authored", "path_rule", "exact",
                    step_for_artifact(step_map, "SysModel", None), model_counts, artifact_id="SysModel",
                ),
                model_indices,
            )
            _finalize_file(files, line_details, rel, "sysml", classes,
                           [model_pair, *span_pairs], filter_empty=False)

    # ---- hamr/microkit tree
    if hamr_dir.is_dir():
        resources = report_info["resources"] if report_info else {}
        on_disk = walk_files(hamr_dir)
        measurable: list[tuple[str, bool | None, str]] = []  # (rel, overwritten_flag, basis)
        for rel in on_disk:
            if rel.startswith(("attestation/", "reporting/", "build/", ".claude/")):
                continue  # excluded wholesale below
            if rel.split("/")[-1] in ORPHAN_WHITELIST_NAMES:
                excluded.append({"path": f"hamr/microkit/{rel}", "reason": "dependency lock / OS metadata (build output)", "file_count": 1})
                continue
            if rel in resources:
                measurable.append((rel, resources[rel], "codegen_report"))
            elif rel == "bin/build.cmd":
                measurable.append((rel, None, "path_rule"))
            elif rel in validation["orphaned_files"]:
                measurable.append((rel, None, "path_rule"))
            elif rel.startswith("bin/"):
                measurable.append((rel, None, "path_rule"))
            else:
                measurable.append((rel, None, "path_rule"))
        for sub, reason in (
            ("attestation", "codegen attestation evidence, not development work product"),
            ("reporting", "codegen report artifacts (parsed as evidence, not measured)"),
            ("build", "build output"),
            (".claude", "harness configuration"),
        ):
            target = hamr_dir / sub
            if target.is_dir():
                count = sum(len(filenames) for _, _, filenames in os.walk(target))
                excluded.append({"path": f"hamr/microkit/{sub}/", "reason": reason, "file_count": count})

        for rel, flag, basis in measurable:
            full = hamr_dir / rel
            try:
                text = full.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                excluded.append({"path": f"hamr/microkit/{rel}", "reason": "binary file", "file_count": 1})
                continue
            language = language_for_path(rel)
            lines = text.splitlines()
            classes = classify_lines(text, language)
            artifact_type, artifact_id, component = artifact_rules_for_hamr_path(rel, crate_components, proof_crate_names)
            pairs: list[tuple[dict, list[int]]] = []
            all_indices = list(range(len(lines)))
            project_rel = f"hamr/microkit/{rel}"

            if rel == "bin/build.cmd":
                pairs.append((
                    _bucket(
                        "build_script", "not_codegen", "developer_authored", "path_rule", "exact",
                        step_for_artifact(step_map, "BuildScript", None), tally(classes), artifact_id="BuildScript",
                        items={"note": "skill-generated in agentic runs; called out separately"},
                    ),
                    all_indices,
                ))
            elif flag is True:
                pairs.append((
                    _bucket(
                        artifact_type, "overwrite", "hamr_generated", "codegen_report", "exact",
                        step_for_artifact(step_map, artifact_id, component), tally(classes),
                        artifact_id=artifact_id, component=component,
                    ),
                    all_indices,
                ))
            elif rel == "microkit.system":
                # Content-marker regions hold developer-preserved content; everything
                # outside them is HAMR-written regardless of the overwrite flag.
                content_flags = xml_content_line_flags(lines)
                generated_flags = [not value for value in content_flags]
                generation_mode = "generate_once" if flag is False else ("overwrite" if flag else "unknown")
                if any(content_flags):
                    pairs.append((
                        _bucket(
                            "microkit_config", generation_mode, "developer_authored", "marker", "exact",
                            step_for_artifact(step_map, artifact_id, component),
                            _tally_subset(classes, content_flags), artifact_id=artifact_id, component=component,
                        ),
                        _indices_where(content_flags),
                    ))
                pairs.append((
                    _bucket(
                        "microkit_config", generation_mode, "hamr_generated", "marker", "exact",
                        step_for_artifact(step_map, "GenCode", None),
                        _tally_subset(classes, generated_flags), artifact_id="GenCode",
                    ),
                    _indices_where(generated_flags),
                ))
            elif flag is None and artifact_id in ("GenCode", "SysProofVCs"):
                # Orphaned or report-absent file whose path identifies it as generated
                # (supplemental path-rule classification; never overrides the report).
                pairs.append((
                    _bucket(
                        artifact_type, "unknown", "hamr_generated", "path_rule", "derived",
                        step_for_artifact(step_map, artifact_id, component), tally(classes),
                        artifact_id=artifact_id, component=component,
                    ),
                    all_indices,
                ))
            else:
                # Generate-once family (flag False), or unknown non-generated file:
                # marker regions are HAMR-woven; non-marker content resolves by
                # template comparison, else preserved_origin_unknown.
                generation_mode = "generate_once" if flag is False else "unknown"
                nonmarker_flags = [True] * len(lines)
                if language == "rust":
                    marker_flags = rust_marker_line_flags(lines)
                    nonmarker_flags = [not value for value in marker_flags]
                    if any(marker_flags):
                        pairs.append((
                            _bucket(
                                "rust_contract_woven", "woven", "hamr_woven", "marker", "exact",
                                step_for_artifact(step_map, "GenCode", component),
                                _tally_subset(classes, marker_flags), artifact_id="GenCode", component=component,
                            ),
                            _indices_where(marker_flags),
                        ))
                if any(nonmarker_flags):
                    template_text = baseline_source.template_text(rel) if (baseline_source and baseline_usable) else None
                    if template_text is not None:
                        template_lines = template_text.splitlines()
                        template_classes = classify_lines(template_text, language)
                        template_nonmarker = (
                            [not value for value in rust_marker_line_flags(template_lines)]
                            if language == "rust" else [True] * len(template_lines)
                        )
                        comparison = compare_with_template(
                            lines, classes, nonmarker_flags, template_lines, template_classes, template_nonmarker
                        )
                        template_accounting.append({"file": project_rel, **comparison["accounting"]})
                        if any(value > 0 for value in comparison["retained"].values()):
                            pairs.append((
                                _bucket(
                                    artifact_type, generation_mode, "hamr_template_retained",
                                    "template_comparison", "derived",
                                    step_for_artifact(step_map, "GenCode", component),
                                    comparison["retained"], artifact_id="GenCode", component=component,
                                ),
                                comparison["retained_indices"],
                            ))
                        pairs.append((
                            _bucket(
                                artifact_type, generation_mode, "developer_authored",
                                "template_comparison", "derived",
                                step_for_artifact(step_map, artifact_id, component),
                                comparison["developer"], artifact_id=artifact_id, component=component,
                            ),
                            comparison["developer_indices"],
                        ))
                    else:
                        pairs.append((
                            _bucket(
                                artifact_type, generation_mode, "preserved_origin_unknown", "fallback", "unknown",
                                step_for_artifact(step_map, artifact_id, component),
                                _tally_subset(classes, nonmarker_flags), artifact_id=artifact_id, component=component,
                            ),
                            _indices_where(nonmarker_flags),
                        ))
            if not pairs:
                pairs = [(
                    _bucket(
                        artifact_type, "unknown", "unknown", "path_rule", "unknown",
                        step_for_artifact(step_map, artifact_id, component), tally(classes),
                        artifact_id=artifact_id, component=component,
                    ),
                    all_indices,
                )]
            _finalize_file(files, line_details, project_rel, language, classes, pairs, filter_empty=True)

    files.sort(key=lambda item: item["path"])

    # ---- rollups
    def add_rollup(target: dict, key: str, bucket: dict) -> None:
        slot = target.setdefault(key, {"sloc": 0, "comment_lines": 0, "blank_lines": 0, "buckets": 0})
        slot["sloc"] += bucket["sloc"]
        slot["comment_lines"] += bucket["comment_lines"]
        slot["blank_lines"] += bucket["blank_lines"]
        slot["buckets"] += 1

    by_provenance: dict = {}
    by_type: dict = {}
    by_component: dict = {}
    by_step: dict = {}
    total_sloc = 0
    for entry in files:
        for bucket in entry["buckets"]:
            total_sloc += bucket["sloc"]
            add_rollup(by_provenance, bucket["provenance"], bucket)
            add_rollup(by_type, bucket["artifact_type"], bucket)
            if bucket["component"]:
                add_rollup(by_component, bucket["component"], bucket)
            step_key = bucket["workflow"]["key"]
            slot = by_step.setdefault(step_key, {"sloc": 0, "granularity": bucket["workflow"]["granularity"], "by_provenance": {}})
            slot["sloc"] += bucket["sloc"]
            slot["by_provenance"][bucket["provenance"]] = slot["by_provenance"].get(bucket["provenance"], 0) + bucket["sloc"]

    step_total = sum(slot["sloc"] for slot in by_step.values())
    if step_total != total_sloc:
        raise MeasureError(f"accounting invariant violated: step buckets {step_total} != total {total_sloc}")

    assurance = collect_assurance(project, hamr_dir, status, index, components, crate_of, proof_crates, report_info)

    resources = report_info["resources"] if report_info else {}
    project_info = {"name": manifest.get("experiment_id", project.name), "path": str(project)}
    line_allocation = build_line_allocation(files, line_details, now, project_info)
    measures = {
        "schema_version": 1,
        "generated": now,
        "project": project_info,
        "manifest_ref": str(manifest_path),
        "inputs": {
            "codegen_report": {
                "present": report_info is not None,
                "path": "hamr/microkit/reporting/codegen_report_sysml.json" if report_info else None,
                "resource_count_raw": report_info["resource_count_raw"] if report_info else None,
                "resource_count_deduped": len(resources) if report_info else None,
                "overwritten_true": sum(1 for value in resources.values() if value) if report_info else None,
                "overwritten_false": sum(1 for value in resources.values() if not value) if report_info else None,
                "internal_status": report_info["internal_status"] if report_info else None,
                "report_status": validation["report_status"],
                "orphaned_files": validation["orphaned_files"],
            },
            "classification_mode": classification_mode,
            "model_dir": model_dir,
            "generation_baseline": baseline_info,
        },
        "files": files,
        "template_accounting": sorted(template_accounting, key=lambda item: item["file"]),
        "assurance": assurance,
        "workflow_status": {
            "profile": status["profile"],
            "rows": status["rows"],
            "workflows": workflow_statuses,
            "completed": sorted(key for key, value in workflow_statuses.items() if value == "done"),
            "components": components,
        },
        "excluded": sorted(excluded, key=lambda item: item["path"]),
        "rollups": {
            "by_provenance": dict(sorted(by_provenance.items())),
            "by_artifact_type": dict(sorted(by_type.items())),
            "by_component": dict(sorted(by_component.items())),
            "by_step_bucket": dict(sorted(by_step.items())),
            "total_sloc": total_sloc,
        },
        "caveats": caveats,
    }
    return measures, line_allocation, line_details


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--project", required=True, help="experiment project root")
    parser.add_argument("--manifest", help="effort manifest path (default <project>/experiment-reports/effort-manifest.json)")
    parser.add_argument("--generation-baseline", help="prepared pristine-generation directory (from prepare_generation_baseline.py)")
    parser.add_argument("--baseline-ref", help="git ref containing untouched post-codegen generate-once files")
    parser.add_argument("--step-map", default=str(DEFAULT_STEP_MAP))
    parser.add_argument("--now", help="ISO timestamp for the generated field (reproducibility)")
    parser.add_argument("--out", help="output path (default <project>/experiment-reports/project-measures.json)")
    parser.add_argument("--line-allocation-out", help="line-allocation output path (default line-allocation.json next to --out)")
    parser.add_argument("--audit-file", help="project-relative path: print an annotated per-line listing (class + owning bucket) for one measured file")
    args = parser.parse_args(argv)

    project = Path(args.project).resolve()
    if not project.is_dir():
        print(f"error: project directory not found: {project}", file=sys.stderr)
        return 2
    manifest_path = Path(args.manifest) if args.manifest else project / "experiment-reports" / "effort-manifest.json"
    if not manifest_path.is_file():
        print(f"error: manifest not found: {manifest_path}", file=sys.stderr)
        return 2
    now = args.now or __import__("datetime").datetime.now().astimezone().isoformat(timespec="seconds")
    try:
        result, line_allocation, line_details = measure(
            project,
            manifest_path,
            Path(args.step_map),
            Path(args.generation_baseline).resolve() if args.generation_baseline else None,
            args.baseline_ref,
            now,
        )
    except MeasureError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    out = Path(args.out) if args.out else project / "experiment-reports" / "project-measures.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"wrote {out.resolve()}")
    allocation_out = Path(args.line_allocation_out) if args.line_allocation_out else out.parent / "line-allocation.json"
    line_allocation["measures_ref"] = out.name
    allocation_out.parent.mkdir(parents=True, exist_ok=True)
    allocation_out.write_text(json.dumps(line_allocation, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"wrote {allocation_out.resolve()}")
    if args.audit_file:
        target = next((entry for entry in line_allocation["files"] if entry["path"] == args.audit_file), None)
        if target is None:
            print(f"error: --audit-file {args.audit_file} is not a measured file", file=sys.stderr)
            return 2
        classes = line_details[args.audit_file]["classes"]
        text_lines = (project / args.audit_file).read_text(encoding="utf-8").splitlines()
        owner: dict[int, str] = {}
        for bucket in target["buckets"]:
            label = f"b{bucket['bucket_index']} {bucket['provenance']}/{bucket['artifact_type']} -> {bucket['workflow_key']}"
            for start, end in bucket["ranges"]:
                for line_number in range(start, end + 1):
                    owner[line_number] = label
        print(f"\n== {args.audit_file} ({target['language']}, {target['line_count']} lines)")
        for line_number, (cls, line_text) in enumerate(zip(classes, text_lines), start=1):
            print(f"{line_number:5d} {cls:7s} {owner.get(line_number, '(unowned)'):64s} | {line_text}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
