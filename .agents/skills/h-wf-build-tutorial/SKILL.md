---
name: h-wf-build-tutorial
description: Build an onboarding tutorial walkthrough for one HAMR workflow experiment from an h-transcribe session transcript, workflow/process definitions, workflow-status evolution, and produced artifacts. Use after a HAMR development session has been transcribed when developers need a chronological, evidence-backed explanation of human invocations, automated workflow steps, artifact changes, audit gates, and situational-awareness reports.
---

# Build a HAMR workflow tutorial

Write `<experiment>/experiment-reports/tutorial-walkthrough.md`. Accept an optional
experiment path; otherwise use the current directory.

## Establish the evidence boundary

1. Read every applicable `AGENTS.md`. Resolve the shared context from
   `HAMR_AGENT_CONTEXT` and read its `AGENTS.md` before HAMR-specific analysis.
2. Assess only the selected experiment. Do not read sibling experiments, their
   tutorials, or cross-experiment history.
3. Require `experiment-reports/session-transcript.md` and `session-metrics.json`
   produced by `h-transcribe`. Confirm the transcript metadata CWD identifies the
   selected experiment.
4. If either transcription artifact is absent or belongs to another experiment, stop
   and request the original development session/path or an explicitly selected
   transcript. Do not run `h-transcribe` implicitly: a later tutorial-authoring
   session can overwrite the development-session evidence.
5. Do not read or merge an existing `tutorial-walkthrough.md`; regenerate the whole
   tutorial from primary evidence. The existing file may contain manual prose or a
   prior interpretation rather than session evidence.

Treat the normalized transcript as the authoritative interaction timeline, subject to
its recorded omissions and truncation limits. Use source artifacts and machine results
as authoritative for technical claims. Use current-experiment assessment reports only
as secondary, post-hoc evidence.

## Reconstruct the session

Read `process/README.md` and `process/conventions.md` from the shared context. From the
transcript, enumerate every developer skill/workflow invocation, parameter, profile,
answer, review decision, approval, commit request, and final handoff. For each invoked
workflow:

1. Read its definition under `process/workflows/`.
2. Expand every executed composite workflow and slice into its nested workflows and
   stable step IDs.
3. Read the invoked skill definitions, elicitations, and directly referenced process
   or documentation sections needed to explain what the agent was required to do.
4. Distinguish executed steps, iteration/re-entry steps, audit points, optional steps,
   and steps correctly recorded `n/a` or not needed.

Inspect the selected experiment's contemporaneous artifacts, including as applicable:

- `reports/workflow-status.md` at the actual target-project location;
- change sketches, change plans, Review Records, and requirements;
- authored model/contracts, specification audits, and integration results;
- code-generation reports, editable implementations, tests, coverage, verification,
  deployment schedules, change reports, and attestation;
- Git commits and diffs limited to paths within the selected experiment.

Use the transcript and path-scoped Git history to reconstruct status evolution. Explain
that `workflow-status.md` is a keyed current-state ledger whose rows may be updated; use
Review Records, commits, and transcript snapshots for historical transitions. If the
repository moved after the transcript ended, distinguish transcript-time completion
from later commits or publication.

Before writing, build an internal timeline with these fields:

| Field | Required content |
|---|---|
| Developer action | Exact invocation or decision and the authority it supplied |
| Workflow state | Workflow/profile, current step, audit or blocking state |
| Agent activity | What the agent actually did, including iteration and diagnosis |
| Artifact delta | Files or artifact classes read, created, modified, or preserved |
| Evidence | Tool outcome, report, test/coverage/proof count, or status note |
| Developer judgment | What a reviewer needed to assess at that point |
| Limit | Vacuity, unsupported metric, mock/stub, scope exclusion, or parser warning |

Begin drafting once every required timeline field has evidence or an explicit evidence
gap. Do not delay the tutorial for open-ended searches through optional reports or
unrelated history; identify residual gaps and their effect on confidence in the
walkthrough.

Do not infer an action merely because a workflow defines it. Do not report a check as
passing unless the transcript or artifact records the result. Mark source-based
inferences explicitly.

## Write the walkthrough

Write for developers new to HAMR agent workflows, not as a verbatim transcript. Use
clear chronological prose, compact tables, short evidence excerpts, and relative links
to experiment artifacts. Explain HAMR terminology where it first matters.

Adapt headings to the session, but include all applicable content below:

1. **Purpose and path conventions** — audience, evidence sources, target system, and
   the difference between skills, workflows, steps, slices, elicitations, profiles,
   and audit points.
2. **System/change overview and starting state** — concept, architecture, baseline
   commit/state, existing evidence, and initial workflow-status snapshot.
3. **Chronological developer invocations** — why each invocation was made, what input
   or authority the developer supplied, and where the agent stopped or continued.
4. **Automated step walkthrough** — cover every executed workflow and nested step. For
   each step state its purpose, actual agent activities, artifacts affected,
   evidence/outcome, and what the developer should assess.
5. **Human review and automation behavior** — elicitations, Review Record decisions,
   audited stops, rapid waivers, blocked states, approval authority, and commit
   boundaries.
6. **Iterations, deviations, and friction** — failures and unexpected conditions,
   which owning workflow handled them, what was regenerated/retested, and whether the
   approved scope or semantics changed.
7. **Workflow-status evolution** — meaningful snapshots or a phase table covering
   `in-progress`, `blocked`, `done`, `waived`, and `n/a` where present.
8. **Situational-awareness artifacts** — explain how plans, status, audits, codegen
   reports, tests/coverage, verification, schedules, change reports, transcripts, and
   assessments help judge quality and completeness.
9. **Acceptance guidance** — give an evidence-based checklist for reviewing the final
   outcome, including important limitations and non-impact claims.
10. **Quick reference and closing perspective** — list every developer invocation in
    order and summarize the developer/agent division of labor.

Preserve exact names, counts, dates, profiles, statuses, and commit identifiers only
when supported. Never hardcode example-specific CR IDs, components, workflow sequences,
or evidence counts into the reusable structure.

## Preserve evidence integrity

- Explain that `waived` under `rapid` means the agent performed and recorded the audit
  without a human pause; do not describe it as an omitted quality gate.
- Report the expected and generated handshake/VC count for integration checks. Qualify
  a zero-obligation success as vacuous rather than a cross-component proof.
- Separate line, branch, contract-oracle, test, and proof evidence. Do not claim a
  numeric coverage result the tool did not emit.
- State mocked components, absent proof scopes, draft/TBD workflow steps, proof escapes,
  and other acceptance boundaries.
- Preserve `h-transcribe` limitations: hidden reasoning/instructions, truncated tool
  data, excluded subagents, unavailable actual billing, and parser coverage warnings
  where relevant.
- Distinguish generated/model-to-code attestation from behavioral or end-to-end proof.
- Prefer evidence-backed non-impact arguments over lists of supposedly untouched files.

## Validate before completion

1. Confirm every developer invocation in the transcript appears in the quick-reference
   table.
2. Confirm every executed workflow and nested step ID is covered, or explicitly explain
   why a defined step was not applicable.
3. Check claims against the final source artifacts and machine results; reconcile any
   plan-versus-actual deviation.
4. Resolve every relative Markdown link from `experiment-reports/` and remove stale
   line anchors.
5. Check Markdown for conflict markers, malformed tables, and trailing whitespace.
6. Confirm no sibling-experiment or pre-existing-tutorial content entered the result.
7. Report the output path, major evidence sources, validation performed, and any
   remaining evidence limitations.
