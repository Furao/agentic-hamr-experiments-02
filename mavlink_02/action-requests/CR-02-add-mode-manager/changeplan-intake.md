# CR-02 ChangePlan intake evidence

Date: 2026-09-22. Invocation: `ChangePlan(cr=CR-02-add-mode-manager) audited`
(user supplied `$changeplan CR-02 audit`). Status: intake awaiting developer answers;
this is preliminary evidence, not an approved change plan.

## Provenance

- Immutable sketch: `add-mode-manager-sketch.md`.
- Latest supplied requirements: `Open_Platform_HLRs_26_09_22_04.md`.
- Earlier supplied revisions _01 through _03 remain archived unchanged.
- Conversation clarification: HAMR inputs are frozen immediately before dispatch.
  A firewall uses its own frozen mode snapshot; a new publication after freezing is
  not visible during that dispatch. Revision _04 uses a threshold of five rejections
  and logs a missing-Recovery timeout during D2 based on that dispatch's snapshot.

## Preliminary baseline observations

- Current commit: `043d574970d28ff172f7261ea0392ddd14ae50a8`; developer confirmation
  that this is the intended baseline is pending.
- Before this workflow's writes, `git status --short` reported only the untracked
  `action-requests/CR-02-add-mode-manager/` directory. Request inputs are therefore
  supplied change provenance outside the committed baseline.
- Model has RxFirewall, MAVLinkFirewall, TxFirewall, LowLevelEthernetDriver, and
  ArduPilot/VMM. No Mode Manager or mode/error interfaces exist yet.
- Rx and MAVLink forwarding guarantees in
  `sysmlv2/open_platform/open_platform_Software.sysml` currently have no mode guard.
- Existing schedule assigns domains 2 through 6 to ArduPilot, TxFirewall, driver,
  RxFirewall, and MAVLinkFirewall respectively; pacer is domain 1. Platform
  `Max_Domain` is 7. Adding a component requires model/deployment/schedule analysis.
- Recorded maintenance evidence in `reports/mavlink-core-consolidation.md` reports
  core tests 6/6, component tests 8/8, core verification 9/0 and component 17/0.
  These are historical results, not freshly rerun checks. That maintenance report
  explicitly excludes rerunning hardware, full build, and coverage.

## Preliminary consistency findings for later ChangePlan.3

1. **Impact:** existing unconditional forwarding contracts need Normal-mode guards
   and Recovery suppression; stateful counter/watchdog and manager contracts are new.
2. **Traceability decision:** `requirements/updated_reqs.md` uses HLR-19 through
   HLR-31 for existing routing, logging, parser, integration, timing, capacity, and
   separation obligations. Revision _04 reuses many of those IDs for different
   meanings. Preserve the archived baseline and explicitly map old derived
   obligations to the supplied authoritative HLRs or distinct derived identifiers;
   do not silently discard obligations or conflate same-number requirements.
3. **Developer clarification pending:** baseline HLR-13 allows whitelisted UDP
   unless BOTH source 14550 and destination 14562 match. Revision _04 requires
   source not 14550 AND destination not 14562. Thus source 14550/destination 68
   changes from allowed to rejected. Proposed interpretation: apply _04's stricter
   direct-UDP policy as an explicit CR-02 delta.
4. **Developer clarification pending:** baseline HLR-24 and component
   `firmware_flash_spec` also deny SECURE_COMMAND message 11004, operation 7.
   Revision _04 HLR-19 explicitly describes COMMAND_INT/LONG command 42650, while
   HLR-22 refers to a blacklist without enumerating additional entries. Proposed
   interpretation: retain the existing secure-command denial as an additional
   blacklist entry. Do not silently remove it.

## Pending ChangeScope answers

The developer has been asked to confirm the baseline and specify wave granularity,
approver/external review, additional testing, verification scope, documents to
update/preserve, frozen areas and deployment constraints, and report additions.
Proposals supplied with the question are not accepted decisions or rapid defaults.
Policy questions 3 and 4 above were also sent for clarification.

The sketch already requires custom.mk for all make commands, reuse of firewall_core,
TxFirewall verification non-regression, human-readable abstractions, separation of
networking and MAVLink concerns, independent executable implementation rather than
GUMBOX delegation, requirement traceability, and preservation of rejection logging.

No model, requirements, implementation, source-control commit, or execution changes
have been made by this planning intake.

## Superseding planning update — 2026-09-22

The developer confirmed user-supplied requirements as authoritative and adoption of
the stricter UDP policy. Supplied revision _05 adds SECURE_COMMAND operation 7 as
HLR-32. On the instruction to start with updated requirements, the full draft plan
was prepared in `change-plan.md`. Earlier pending intake questions are retained
above as history; unanswered preferences are now concrete proposals in its Review
Record, not presumed approvals. Workflow is awaiting ChangePlan.AP1.

## Requirements revision _06 — 2026-09-22

The developer supplied _06 and requested the plan update. HLR-19 now assigns both
COMMAND_INT and COMMAND_LONG to v1 bytes 34–35 and v2 bytes 38–39, matching the
current implementation. The plan uses _06 as authority, closes RD-4, and removes
the predicted command-offset implementation change. Requirements remain developer-owned.
RD-1, RD-2, RD-3, RD-5 and RD-6 remain open; overall approval is pending.
