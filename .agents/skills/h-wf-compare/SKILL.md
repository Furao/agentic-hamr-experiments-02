---
name: h-wf-compare
description: Compare HAMR experiments across system concept, harness, model, workflow profile, normalized development metrics, findings, outcomes, and authored artifact quality; supports current and legacy report formats.
---

# Compare workflow experiments

This is the sanctioned cross-experiment reader. Resolve each named experiment to the
project root containing `reports/workflow-status.md`; when no names are supplied, consider
all repository-root `ex*` folders and report skipped or missing inputs.

## Normalize meta evidence first

Resolve the bundled script relative to this selected skill directory and run:

```sh
python3 scripts/normalize_experiments.py --out <temporary-or-report-path>.json <project>...
```

The normalizer consumes schema-v2 metrics and `assessment-summary.json` first. It adapts
schema-v1 metrics and legacy Markdown-only assessments without rewriting them. Use its
deterministic values for harness and version, models, profile, outcomes, tokens,
API-list-price-equivalent cost, actual cost, time, friction, scope-separated finding
counts, and data-quality caveats.

## Judge authored artifacts

Compare ConOps, requirements, non-library SysMLv2 models, GUMBO contracts, editable thread
application code and tests, schedule inputs/assignments, and authored system verification
specifications. Generated HAMR code is not an agent-quality artifact; use it only for
outcomes. Group decision-level comparisons by comparable system concept. Classify material
divergence as underspecified concept, shared guidance, model capability, harness/environment,
or benign alternative. Audit `ASSUMED:` provenance and precise documentation citations.

## Write the standard comparison

Resolve `template/comparison.md` relative to this skill and use it to write
`reports/EXP-COMPARE-<YYYY-MM-DD>.md` with:

- one row per experiment with explicit system concept, harness/version, model, profile,
  outcomes, scope-separated finding counts, tokens, API-price-equivalent and actual cost,
  active/wall time, and friction;
- recurring HAMR/shared-workflow findings separated from harness-specific findings;
- artifact-quality subsections for ConOps, requirements, model, contracts, component code,
  tests, schedule, and system verification;
- an engineering-decision divergence table with likely driver and repeatability fix;
- an assumptions audit and recommendations split by HAMR, shared workflow, harness, and
  environment targets;
- explicit data-quality caveats for parser warnings, null actual cost, legacy adapters,
  excluded subagents, missing data, and materially different harness versions.

Retain agent judgment for artifact quality and divergence classification, but never replace
normalized meta fields with estimates taken from prose.
