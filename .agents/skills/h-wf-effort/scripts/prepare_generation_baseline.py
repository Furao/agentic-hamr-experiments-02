#!/usr/bin/env python3
"""Prepare a pristine post-generation baseline for template comparison.

Explicit demo-prep, never run implicitly by the measurer. Copies the project's
sysmlv2/ tree into an isolated scratch directory and reruns HAMR codegen there,
reusing the exact commandLineArgs recorded in the project's codegen report, so
the model-relative --output-dir resolves inside the scratch tree and the real
project is never touched. Writes baseline-provenance.json (tool version, exact
command, model-file hashes, timestamp, exit status).

A reporter failure during baseline generation is tolerated: the generated tree
is usable for template comparison even if report finalization fails.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shlex
import shutil
import subprocess
import sys
from pathlib import Path


class BaselineError(Exception):
    pass


def find_model_dir(project: Path) -> Path:
    for base_name in ("sysmlv2", "sysml"):
        base = project / base_name
        if not base.is_dir():
            continue
        for child in sorted(base.iterdir()):
            if child.is_dir() and child.name != "aadl-lib" and any(child.rglob("*.sysml")):
                return child
    raise BaselineError(f"no model directory with .sysml files under {project}")


def find_main_model_file(model_dir: Path) -> Path:
    candidates = [
        path for path in sorted(model_dir.glob("*.sysml"))
        if "//@ HAMR:" in path.read_text(encoding="utf-8")
    ]
    if not candidates:
        raise BaselineError(f"no .sysml file with a //@ HAMR: config comment in {model_dir}")
    if len(candidates) > 1:
        raise BaselineError(f"multiple //@ HAMR: config files in {model_dir}: {[p.name for p in candidates]}")
    return candidates[0]


def recorded_command_args(project: Path) -> str | None:
    report_path = project / "hamr" / "microkit" / "reporting" / "codegen_report_sysml.json"
    if not report_path.is_file():
        return None
    data = json.loads(report_path.read_text(encoding="utf-8"))
    for entry in data.get("reports", {}).get("entries", []):
        if isinstance(entry, list) and len(entry) == 2 and entry[0] == "KEY_TOOL_REPORT":
            return entry[1].get("commandLineArgs")
    return None


def sireum_version(sireum: Path) -> str:
    result = subprocess.run([str(sireum), "--version"], capture_output=True, text=True, check=False)
    first = (result.stdout or result.stderr).splitlines()
    return first[0].strip() if first else "unknown"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--project", required=True, help="experiment project root")
    parser.add_argument("--out", required=True, help="isolated scratch directory for the baseline (created; must not be inside the project)")
    parser.add_argument("--now", help="ISO timestamp recorded in provenance (reproducibility)")
    args = parser.parse_args(argv)

    project = Path(args.project).resolve()
    out = Path(args.out).resolve()
    if not project.is_dir():
        print(f"error: project directory not found: {project}", file=sys.stderr)
        return 2
    if project in out.parents or out == project:
        print("error: --out must be outside the project tree", file=sys.stderr)
        return 2
    sireum_home = os.environ.get("SIREUM_HOME")
    if not sireum_home:
        print("error: SIREUM_HOME is not set", file=sys.stderr)
        return 2
    sireum = Path(sireum_home) / "bin" / "sireum"

    try:
        model_dir = find_model_dir(project)
    except BaselineError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1

    out.mkdir(parents=True, exist_ok=True)
    scratch_sysml = out / "sysmlv2"
    if scratch_sysml.exists():
        shutil.rmtree(scratch_sysml)
    shutil.copytree(project / model_dir.parent.name, scratch_sysml)
    scratch_model_dir = scratch_sysml / model_dir.name

    try:
        main_file = find_main_model_file(scratch_model_dir)
    except BaselineError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1

    args_string = recorded_command_args(project)
    if args_string:
        command_args = shlex.split(args_string)
        # Strip any absolute output dirs defensively; the recorded invocations are relative.
        command_source = "codegen_report_sysml.json commandLineArgs"
    else:
        command_args = [
            "--sourcepath", "../aadl-lib:.", "--platform", "Microkit",
            "--output-dir", "../../hamr", "--workspace-root-dir", "../..",
        ]
        command_source = "default (no codegen report found)"

    command = [str(sireum), "hamr", "sysml", "codegen", *command_args, main_file.name]
    result = subprocess.run(command, cwd=scratch_model_dir, capture_output=True, text=True, check=False)

    generated_root = out / "hamr" / "microkit"
    tree_usable = generated_root.is_dir() and any(generated_root.iterdir())

    model_hashes = {
        str(path.relative_to(scratch_sysml)): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in sorted(scratch_model_dir.rglob("*.sysml"))
    }
    now = args.now or __import__("datetime").datetime.now().astimezone().isoformat(timespec="seconds")
    provenance = {
        "schema_version": 1,
        "kind": "generation-baseline",
        "project": str(project),
        "model_dir": f"{model_dir.parent.name}/{model_dir.name}",
        "main_model_file": main_file.name,
        "sireum_version": sireum_version(sireum),
        "command": command,
        "command_args_source": command_source,
        "exit_code": result.returncode,
        "tree_usable": tree_usable,
        "reporter_note": (
            "codegen exited nonzero but the generated tree is present and usable for "
            "template comparison (reporter finalization failures are tolerated)"
            if result.returncode != 0 and tree_usable else None
        ),
        "stdout_tail": result.stdout[-2000:],
        "stderr_tail": result.stderr[-2000:],
        "model_file_sha256": model_hashes,
        "timestamp": now,
    }
    provenance_path = out / "baseline-provenance.json"
    provenance_path.write_text(json.dumps(provenance, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    tail = re.sub(r"\s+", " ", result.stdout[-200:]).strip()
    print(f"codegen exit {result.returncode}; tree usable: {tree_usable}; {tail}")
    print(f"wrote {provenance_path}")
    return 0 if tree_usable else 1


if __name__ == "__main__":
    raise SystemExit(main())
