# Open Platform — CR-02 only — Experiment Reports

Findings from one HAMR workflow-evaluation run.

## Run context

- **System:** Add latched Normal/Recovery control, rejection counting, generated R2U2 monitoring, strict UDP routing and amended transmit bounds to the existing Open Platform.
- **Entry point / profile:** CR-02 requirements review → ChangePlan(CR-02) → ChangeExec(CR-02), W1–W3 and final approval / audited
- **Harness / model:** Codex 0.154.0 / gpt-6-astra
- **Outcome:** CR-02 complete and developer-approved, with CR-02-HW-01 High/Open deferred. Type-check/codegen, six proof-enabled crates and the ZCU102 build passed; 67 application/core tests plus 2 driver helper tests passed. Entry-point line coverage is 100%; branch counters are unavailable. W1 integration check had N=0 handshakes.
- **Evidence:** [normalized transcript](../session-transcript.md), [schema-v2 metrics](../session-metrics.json),
  [workflow status](../../reports/workflow-status.md), current CR-02 requirements and produced artifacts

## Reports by category

| File | Category | Findings |
|---|---|---|
| [01-workflow-and-skill-design.md](01-workflow-and-skill-design.md) | Workflow and skill design | WF-01, WF-02, WF-03 |
| [02-hamr-codegen-and-tooling.md](02-hamr-codegen-and-tooling.md) | HAMR codegen and tooling | CG-01, CG-02, CG-03, CG-04 |
| [03-documentation-and-tool-use-guidance.md](03-documentation-and-tool-use-guidance.md) | Documentation and tool-use guidance | DOC-01, DOC-02 |
| [04-agent-harness-and-environment.md](04-agent-harness-and-environment.md) | Agent harness and environment | ENV-02, ENV-03 |

Severity: 🔴 blocker · 🟠 moderate · 🟡 minor · 🟢 positive.

Machine-readable findings: [assessment-summary.json](assessment-summary.json).

## CR-02 assessment boundary

Baseline: `043d574970d28ff172f7261ea0392ddd14ae50a8`.
Current authority: [requirements revision _26_09_23_02](../../action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md).
The [CR-02 final report](../../reports/CR-02/CR-02-add-mode-manager.md) records the
accepted scope amendments and completion. The assessed transcript interval is
**lines 136–10560**: first CR-02 requirements review through final approval and its
recording. Final approval is the user response at line 10516. This interval includes
requirements review before formal ChangePlan, the three implementation waves,
toolchain migration, bounds/logging corrections, the unsuccessful reverted timing
experiment, upgrade validation and requirements reconciliation.

Excluded: original CR-01 development; unchanged baseline authorship; the initial
status question/experiment setup; post-closeout report reorganization beginning at
line 10562; transcription, assessment and effort-report work. No sibling experiment
was read. Current relocated evidence paths are navigation only, not credit for
post-closeout work. Legacy requirements are cited only for the retirement notices
added during CR-02. The baseline capacity defect and driver environment limits are
explicitly identified as inherited problems encountered during this change.

There are **11 findings**: one historical blocker resolved during CR-02, six
moderate findings, one minor finding and three positives. Stable finding codes are
retained from the earlier assessment. ENV-01 is not a CR-02 development finding;
its parser limitations remain in the required friction inventory. DOC-02 now covers
requirements reconciliation only, excluding later report reorganization.

The generator verdict-loss blocker was resolved; it is separate from the accepted
**High/Open CR-02-HW-01** hardware timeout. No current CR-02 approval blocker is
asserted. Future timeout diagnosis, fix and hardware retest are recommendations,
not work performed or claimed complete in CR-02.

## Evidence interpretation

Session `01a0c915-a92e-7ae2-a6ad-737746c47d43` spans September 22–24 UTC
(September 22–23 local). Its 132704 wall-clock seconds, 18143 active seconds,
97 user turns and 527 tool calls describe the **whole normalized session**. They
are not recomputed or presented as CR-02-only totals. No cost or savings estimate
is made. Subagents are excluded; nested-call decoding is incomplete.

The bounded event inventory confirms three visible filesystem-denial events and
seven escalation requests, with network errors listed separately. The metrics'
zero extracted arrays do not establish absence. The single unlinked saved-prefix
approval and 1267 parser warnings remain session-wide metadata, not CR-02 counts.
Observable execution is not proof that an invisible approval dialog was displayed.

Final assurance evidence records ModeManager/Rx/Tx/MAVLink/core proof counts
9/28/16/69/39/38 with zero errors. These are CR-02 verification/regression results,
not counts of newly authored obligations. No whole-system temporal proof, driver
application proof, complete driver host suite or numerical branch coverage is
claimed. No codegen, patch, test, build, hardware run or h-transcribe invocation
was performed for this assessment. Sources and issue/workflow states were preserved.

[Scope and input hashes](assessment-scope.json) record the boundary and exclusions.
[CR-02-only effort](../CR-02-effort/README.md) is separate modeled analysis and was
not used to infer assessment findings or measured savings.

## Validation

From the selected h-wf-assess skill directory:

```sh
python3 scripts/validate_assessment.py /home/robertvanvossen/dev/agentic-hamr-experiments-02/mavlink_02/experiment-reports/CR-02-assessment
```

The common filenames and v1 schema are preserved in this isolated CR-02 report
set; the earlier assessment is retained in the parent directory.

Result: **PASS — 11 findings**. The bundled validator and JSON Schema validation
passed. Evidence links resolve; input hashes are unchanged; cited transcript
findings and all seven escalation requests / three explicit filesystem denials
were checked against the CR-02 interval. No development source was modified.
