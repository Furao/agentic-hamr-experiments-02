---
name: h-transcribe
description: Generate a normalized Claude Code or Codex development-session transcript and schema-v2 metrics only when the current user message explicitly invokes `$h-transcribe` or `/h-transcribe`. On-demand or end-of-session context, plain-name mentions, prior turns, other skills, and automated workflows never authorize it.
---

# Transcribe an experiment session

## Authorization gate

Before taking any action, inspect only the current user message. Continue only when that
message contains the literal invocation `$h-transcribe` or `/h-transcribe`.

Authorization applies only to that current message. Do not inherit it from an earlier turn,
another skill, or an automated workflow. Repository location, development-completion or
end-of-session language, and the plain name `h-transcribe` do not authorize execution.

Without authorization, stop immediately. Do not run scripts or create, replace, or modify
transcript, metrics, or other artifacts.

After the authorization gate passes, run the deterministic driver bundled in this skill.
Resolve `scripts/transcribe.py` and `pricing.json` relative to this selected skill directory,
then invoke:

```sh
python3 scripts/transcribe.py --harness auto --out <experiment>/experiment-reports --pricing pricing.json
```

Accepted options:

- `--harness auto|claude|codex`: select an adapter. `auto` fails if both formats match.
- `--session <path-or-id>`: override current-directory session selection.
- `--out <dir>`: default `experiment-reports/` under the current directory.
- `--pricing <file>`: provider/model-qualified public API rates.

The driver selects sessions by matching transcript cwd before recency, normalizes dialog
and tool activity, and writes `session-transcript.md` plus schema-v2
`session-metrics.json`. The machine-readable contract is bundled at
`schema/session-metrics-v2.schema.json`. Summarize the printed harness, models, token counts, API-list-price
equivalent, time, friction, and parser warnings to the developer.

Treat the limitations in the output as part of the evidence contract:

- API-list-price equivalent is comparable across runs; actual billing remains null unless
  the transcript contains a trustworthy billed amount.
- Subscription credits and rate-limit percentages are never converted to dollars.
- Main-session activity only is measured; subagents are explicitly excluded.
- Hidden reasoning and developer/system instructions are omitted.
- Approval prompts are not always observable. Distinguish confirmed failures, requested
  escalation, observable outcomes, and heuristic dynamic-command candidates.
- Codex reasoning-output tokens are an informational subset of output and are not added to
  output cost a second time.

If the adapter reports coverage warnings, retain them and call them out; do not silently
repair or discard uncertain records.

When a harness format changes, preserve the failing sanitized record as a fixture, update
only its adapter, increment the adapter/source-format version, and run the cross-adapter
golden tests. Introduce a new metrics schema version instead of changing schema v2 in place.
