---
name: h-wf-assess
description: Assess one HAMR experiment from normalized transcript, schema-v2 metrics, workflow status, and produced artifacts; write harness-neutral category reports and a validated assessment-summary.json.
---

# Assess one workflow experiment

Assess only the current experiment. Do not read sibling experiments.

## Gather evidence

Read `experiment-reports/session-transcript.md` and `session-metrics.json`, running
`h-transcribe` first if needed. Also inspect `reports/workflow-status.md`, requirements,
the authored model and application code, generated outcomes, and any contemporaneous run
notes. Cite files, transcript events, commands, or exact errors.

## Write the common report set

Resolve `template/` relative to this selected skill directory. Write these same filenames
for every harness:

1. `01-workflow-and-skill-design.md` — workflow and skill design.
2. `02-hamr-codegen-and-tooling.md` — HAMR codegen and tooling.
3. `03-documentation-and-tool-use-guidance.md` — documentation and tool-use guidance.
4. `04-agent-harness-and-environment.md` — agent harness and environment.

Write `README.md` from the bundled template. For each finding include a unique stable code,
severity, concrete evidence, impact, root cause when supported, a concrete recommendation,
and an evidence-limit statement. Use severities `blocker`, `moderate`, `minor`, and
`positive`, rendered as 🔴, 🟠, 🟡, and 🟢 in Markdown.

Also write `assessment-summary.json` with:

- `schema_version: 1`;
- `run`: project, session id, harness, harness version, models, and profile;
- `findings`: objects containing `code`, category `1..4`, severity, scope
  (`hamr|shared-workflow|claude|codex|environment`), title, non-empty evidence references,
  recommendation target, and evidence limit.

Use the bundled `schema/assessment-summary-v1.schema.json` as the machine-readable
contract; add a new schema version rather than changing version 1 in place.

## Required category-4 friction inventory

Drive the inventory from schema-v2 `friction` data:

1. Enumerate confirmed sandbox failures and requested escalations separately.
2. List dynamic shell commands only as heuristic prompt candidates.
3. Record observable approval outcomes without claiming invisible prompts were observed.
4. Identify the harness policy/configuration or command-shape cause for each class.
5. Name a concrete configuration, workflow, or agent-command-shape fix.
6. State the epistemic limits from metrics, including excluded subagents and parser warnings.

Harness-specific findings belong in scope `claude` or `codex`; do not attribute them to
HAMR or model quality.

## Validate before completion

From this skill directory run:

```sh
python3 scripts/validate_assessment.py <experiment>/experiment-reports
```

Fix missing fields, duplicate codes, invalid values, and Markdown/JSON finding mismatches
before reporting completion. Existing assessments keep their legacy filenames and do not
need retroactive rewriting.
