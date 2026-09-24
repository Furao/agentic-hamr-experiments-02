# Open Platform / mavlink_02 — Experiment Reports

Findings from one HAMR workflow-evaluation run.

## CR-02-only assessment

Use the [CR-02-only assessment](CR-02-assessment/README.md) for findings bounded to
the change’s requirements review through final approval. It contains 11 validated
findings and explicitly excludes CR-01 execution and post-closeout reporting work.
The original assessment below is retained for provenance.

## Run context

- **System:** ZCU102/seL4/Microkit network firewall with latched Normal/Recovery control and generated R2U2 deadline monitoring.
- **Entry point / profile:** CR-02 ChangePlan → ChangeExec (three waves), followed by closeout / audited
- **Harness / model:** Codex 0.154.0 / gpt-6-astra
- **Outcome:** Developer-approved completion with High/Open CR-02-HW-01 deferred. Recorded type-check/codegen and target build pass; six proof-enabled crates pass (9/28/16/69/39/38 obligations); 67 application/core tests plus 2 driver helper tests pass. Application entry-point line coverage is 100%; numerical branch coverage is unavailable. Historical Logika integration pass has N=0 handshakes.
- **Evidence:** `session-transcript.md`, `session-metrics.json`, and
  `../reports/workflow-status.md`

## Reports by category

| File | Category | Findings |
|---|---|---|
| [01-workflow-and-skill-design.md](01-workflow-and-skill-design.md) | Workflow and skill design | WF-01, WF-02, WF-03 |
| [02-hamr-codegen-and-tooling.md](02-hamr-codegen-and-tooling.md) | HAMR codegen and tooling | CG-01, CG-02, CG-03, CG-04 |
| [03-documentation-and-tool-use-guidance.md](03-documentation-and-tool-use-guidance.md) | Documentation and tool-use guidance | DOC-01, DOC-02 |
| [04-agent-harness-and-environment.md](04-agent-harness-and-environment.md) | Agent harness and environment | ENV-01, ENV-02, ENV-03 |

Severity: 🔴 blocker · 🟠 moderate · 🟡 minor · 🟢 positive.

Machine-readable findings: [assessment-summary.json](assessment-summary.json).

## Assessment scope and limits

This is an assessment of the existing normalized primary session and current local
artifacts, not a new development or hardware validation run. No sibling experiment
was read. CR-01 artifacts were consulted only as this project’s inherited baseline;
its earlier development is not attributed to the measured CR-02 session.

Session `01a0c915-a92e-7ae2-a6ad-737746c47d43` spans
2026-09-22T12:27:27Z through 2026-09-24T01:19:11Z (UTC; local closeout September 23).
Metrics record 132704 seconds wall time and 18143 seconds active time under a
300-second gap rule, 97 user turns, 527 tool calls and 526 results. These are parser
measures, not human effort, uninterrupted compute time or counts of nested shell
commands. Subagents are excluded. Cost is unavailable; no dollar estimate is inferred.
Existing transcript and metrics were used unchanged; h-transcribe was not invoked.

There are 12 findings: one historical blocker now resolved, seven moderate findings,
one minor finding and three positives. Severity reflects the evidenced impact;
disposition identifies what remains open. In particular, the generator reporting fix
and accepted hardware timeout are different issues. No current CR-02 approval blocker
is asserted, and no timing issue is closed by this assessment.

The category-4 inventory reconciles empty metrics arrays with visible transcript
evidence. Its event counts are lower bounds, not reconstructed prompt totals.
Recommendations do not themselves modify workflows, generators, source or issue status.

## Validation

Run from the selected h-wf-assess skill directory:

```sh
python3 scripts/validate_assessment.py /home/robertvanvossen/dev/agentic-hamr-experiments-02/mavlink_02/experiment-reports
```

Result: **PASS — 12 findings**, with matching codes, categories, titles and severities
across Markdown and JSON. Validation against the bundled JSON Schema also passed;
all category-report Markdown evidence links resolve. No source/build/test commands
were rerun for this assessment. A repository-wide whitespace check reports two
pre-existing trailing-whitespace lines in the supplied session transcript; that
evidence file was left unchanged.

## CR-02-only effort report

For the change-only estimate, use the [CR-02 effort report](CR-02-effort/README.md):
[HTML](CR-02-effort/EFFORT-CR-02-2026-09-23.html) ·
[Markdown](CR-02-effort/EFFORT-CR-02-2026-09-23.md). This uses the approved CR-02
baseline and excludes inherited work. The snapshot report below has a broader
scope and does not answer the CR-02-only question.

## Effort and cost report — current project snapshot

- [View the self-contained HTML report](EFFORT-mavlink_02-2026-09-23.html)
- [Read the Markdown report](EFFORT-mavlink_02-2026-09-23.md)
- [Manifest](effort-manifest.json), [measures](project-measures.json),
  [line allocations](line-allocation.json), [estimate](effort-estimate.json)

**UNCALIBRATED MODEL; partial scope.** This refresh uses the current project snapshot
and the CR-02 session metrics. It is not an incremental CR-02 effort estimate:
inherited CR-01/vendor artifacts are present, current requirements under
`action-requests/` are excluded by the v1 measurer, and some workflow labels and
assurance values are historical or unrecognized. Generate-once provenance remains
unknown without a pristine baseline. The manifest and report caveats detail these
limitations. No measured savings or modeled-versus-observed ratio is claimed.
The August 27 reports are retained as historical outputs; the unversioned JSON
files now describe the September 23 refresh.

Observed session activity is 5.04 active hours and 36.86 wall-clock hours under the
metrics adapter's gap rule. Both agent cost measures are unavailable. The model
assigns 262.67 hours / $39,400.50 (`experienced_sel4`) or 545.02 hours / $59,952.20
(`new_to_sel4`) to its classified snapshot artifacts and parsed activity overheads.
Those placeholder estimates exclude unknown-provenance application content and
are neither complete project estimates nor change-only estimates.

Reproduce with the project wrapper, which runs the bundled pipeline, carries
manifest caveats into the estimate, rerenders, and adds the scope notice:

```sh
python3 experiment-reports/refresh-effort-report.py --now 2026-09-23T21:34:00-04:00
```

Rate/counterfactual configurations are unchanged and their hashes are recorded in
the estimate. No code generation or application test/build was performed. The
referenced repository-root measurement-spec document is absent; the bundled
scripts and schemas were used without inventing replacement rules.

Validation passed for all four JSON schemas, exact line partition and matching
bucket counts across all 1,343 measured files, scope-ratio suppression, and the HTML
scope notice/self-contained resource checks. These checks do not calibrate the
model or validate its accuracy for change requests.
