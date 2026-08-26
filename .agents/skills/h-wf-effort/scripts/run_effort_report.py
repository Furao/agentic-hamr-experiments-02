#!/usr/bin/env python3
"""Thin orchestrator: measure -> estimate -> render for one experiment.

Writes project-measures.json, effort-estimate.json, EFFORT-<project>-<date>.md,
and EFFORT-<project>-<date>.html into the experiment's experiment-reports/.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import estimate_effort  # noqa: E402
import measure_project  # noqa: E402
import render_report  # noqa: E402


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--project", required=True, help="experiment project root")
    parser.add_argument("--manifest", help="effort manifest (default <project>/experiment-reports/effort-manifest.json)")
    parser.add_argument("--generation-baseline", help="prepared pristine-generation directory")
    parser.add_argument("--baseline-ref", help="git ref with untouched post-codegen files")
    parser.add_argument("--now", help="ISO timestamp for reproducible output")
    args = parser.parse_args(argv)

    project = Path(args.project).resolve()
    reports = project / "experiment-reports"
    manifest = args.manifest or str(reports / "effort-manifest.json")
    measures_out = reports / "project-measures.json"
    estimate_out = reports / "effort-estimate.json"

    measure_args = ["--project", str(project), "--manifest", manifest, "--out", str(measures_out)]
    if args.generation_baseline:
        measure_args += ["--generation-baseline", args.generation_baseline]
    if args.baseline_ref:
        measure_args += ["--baseline-ref", args.baseline_ref]
    if args.now:
        measure_args += ["--now", args.now]
    code = measure_project.main(measure_args)
    if code != 0:
        return code

    estimate_args = ["--measures", str(measures_out), "--manifest", manifest, "--out", str(estimate_out)]
    if args.now:
        estimate_args += ["--now", args.now]
    code = estimate_effort.main(estimate_args)
    if code != 0:
        return code

    return render_report.main(["--estimate", str(estimate_out), "--measures", str(measures_out)])


if __name__ == "__main__":
    raise SystemExit(main())
