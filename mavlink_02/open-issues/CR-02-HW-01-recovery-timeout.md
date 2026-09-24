# CR-02-HW-01 — Recovery observation timeout

| Field | Value |
|---|---|
| Criticality | **High** — assigned by developer |
| Status | **Open** |
| Recorded | 2026-09-23 |
| Disposition | Deferred by developer to a future CR; non-blocking for CR-02 completion |
| Follow-up CR | Not yet assigned |
| Related requirements | HLR-30; LLR-15/16 recovery deadline and monitor semantics |
| Owner | Not yet assigned |

## Evidence

Source: [manual test log](../manual_test_results/26_09_23_14_19_open_platform.log).
SHA-256: cd9c505763f68649075f4afef4b9ad3979024fe6b621b9c8fce5c7667871eada.
Five lane-0 MAVLink firmware-flash denials appear at lines 673, 691, 714, 735 and
766. Lines 767–768 contain the Normal-to-Recovery diagnostic interleaved with
`Mode-transition timeout: Recovery not observed by D2`. Exact byte-level
interleaving verification is documented in the
[manual log review](../reports/CR-02/deployment/CR-02-manual-log-review-26_09_23_14_19.md).

The threshold-triggered state change is observed, but the monitor reports that it
did not observe Recovery by D2. The log does not establish exact dispatch timing,
publication/consumption order, or the cause of the timeout. This is an observed
monitor failure report, not a confirmed diagnosis of scheduling or monitor logic.

## Impact

Required timely Recovery observation is not demonstrated. Delayed delivery or
observation could delay mode-based suppression of otherwise allowed traffic; a
monitor/reporting defect could instead produce a false deadline diagnostic. The
available evidence does not distinguish those cases or demonstrate actual unwanted
forwarding. Criticality remains High as directed by the developer.

## Attempted mitigation

Adding 200 ms to each of ModeManager and MAVLinkFirewall did not fix the problem,
as reported by the developer, who reverted that change. Source settings are restored
to ModeManager 100 ms, MAVLinkFirewall 300 ms, frame 2080 ms, and legacy slots
10000/30000. Do not treat the reverted mitigation as a resolution.

## Acceptance and closure

On 2026-09-23 the developer explicitly directed that this timing issue will be
addressed by a future CR and is no longer a blocker for CR-02 final review. The
issue remains High/Open here, with its evidence and closure criteria intact.
This accepts deferral for CR-02; it does not claim the timeout is fixed or remove
HLR-30 and the related timing requirements from the authoritative requirements.

On 2026-09-23 the developer explicitly instructed: "accept the manual testing with
a high crtiticality finding about the timeout issue". Manual testing is accepted
with CR-02-HW-01 open. This acceptance neither resolves the finding nor waives or
changes the timeout requirement. No additional approval for this disposition is
required. Overall Wave 3/final change approval remains separately tracked.

To close: identify the cause, correct it, and capture a repeatable hardware trace
showing the first threshold crossing at D0, mode publication and MAVLink's frozen
mode observations at D1/D2. Demonstrate timely Recovery without the nominal timeout,
and retain the late/absent-Recovery monitor regression. If the issue is reporting
rather than actual latency, demonstrate that distinction with corresponding runtime
evidence. Record the tested image and any remaining timing limitations.

## Wave 3 approval — 2026-09-23

Developer explicitly approved Wave 3 after the full verification/build recorded in
[the full verification/build report](../reports/CR-02/final-validation/CR-02-full-verify-build.md).
The wave is accepted with CR-02-HW-01 retained
as High/Open. This supersedes earlier statements that Wave 3 approval was pending;
it does not resolve the timeout or grant final CR-02 completion approval.

## CR-02 completion — 2026-09-23

The developer approved final CR-02 completion with this issue deferred to a future
CR. The issue remains High/Open; its owner and follow-up CR ID remain unassigned.
The acceptance does not establish resolution of the timeout.
