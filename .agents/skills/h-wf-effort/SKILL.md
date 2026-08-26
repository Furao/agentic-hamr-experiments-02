---
name: h-wf-effort
description: Estimate human-equivalent effort and cost for one HAMR greenfield experiment from measured artifacts, organized by workflow step, rendered beside observed agent session metrics; supports schema-v2 and legacy session metrics and partial codegen reports.
---

# Estimate effort and cost for a workflow experiment

Measure a completed greenfield HAMR experiment, model human-equivalent effort under
explicit uncalibrated rate profiles, and render stakeholder-viewable Markdown and
self-contained HTML reports. Every rendered number is Observed (measured), Modeled
(estimated under a named profile, scenario, and counterfactual — always labeled
UNCALIBRATED MODEL), or Compared (scope-gated). Never present modeled values as
measured savings.

## Prepare inputs

Each experiment needs `experiment-reports/effort-manifest.json` (schema
`experiment-manifest-v1`, see `schema/`). Hand-author it once per experiment; the
concept quote must be the verbatim developer concept from `requirements/conops.md` §0.
The measurer derives completed workflow scope from `reports/workflow-status.md` — do
not duplicate scope in the manifest.

## Optional: prepare a pristine generation baseline

To resolve the provenance of generate-once files (`*_app.rs`, `tests.rs`,
`microkit.schedule.xml`), regenerate a pristine tree from the final model into an
isolated scratch directory:

```sh
python3 scripts/prepare_generation_baseline.py --project <experiment-dir> --out <scratch-dir>
```

This copies the model into the scratch tree, reuses the codegen invocation recorded in
the project's codegen report, and writes `baseline-provenance.json` (tool version,
command, model hashes). It never runs implicitly and never touches the project.
Without a baseline, non-marker content of generate-once files is reported honestly as
`preserved_origin_unknown` — never silently as developer-authored.

## Run the pipeline for one experiment

```sh
python3 scripts/run_effort_report.py --project <experiment-dir> [--generation-baseline <scratch-dir>]
```

This runs measure → estimate → render and writes into the experiment's
`experiment-reports/`: `project-measures.json`, `line-allocation.json` (line-level audit
detail: the 1-based line ranges owned by each measurement bucket, validated to exactly
partition every measured file), `effort-estimate.json`, `EFFORT-<experiment>-<date>.md`,
and `EFFORT-<experiment>-<date>.html`. The stages can also be run individually
(`scripts/measure_project.py`, `scripts/estimate_effort.py`, `scripts/render_report.py`);
pass `--now <iso>` for reproducible timestamps. For manual review of a single file's
counting and allocation, `scripts/measure_project.py --audit-file <project-relative-path>`
prints an annotated per-line listing (classification plus owning bucket). The
counting/allocation rules themselves are specified in `docs/effort-measurement-spec.md`
at the repository root.

## Compare experiments

```sh
python3 scripts/render_report.py --compare <estimate.json> <estimate.json>... --out <path>.md
```

Comparison pages show per-step values side by side over the commonly completed
workflow subset only; steps completed in one run but not another render as
`not comparable (scope)`. Session totals appear once per experiment. No
cross-experiment ratios are rendered; the only permitted ratio is the
within-experiment modeled-vs-observed juxtaposition under `exact` scope alignment.

## Interpret and edit the model

Rates, profiles, and equivalence scenarios live in `config/effort-model-v1.json` (the
calibration surface — all values are uncalibrated placeholders); counterfactual
category assignments in `config/counterfactual-v1.json`; artifact-to-step mapping in
`config/step-map-v1.json`. Change configs, not code, to revise the model; the reports
record config SHA-256s for reproducibility.
