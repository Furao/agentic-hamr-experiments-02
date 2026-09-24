# Change Plan — CR-02 Add Mode Manager

| Field | Value |
|---|---|
| Change ID | CR-02 |
| Status | Executed |
| Execution completion | Developer approved final CR-02 review on 2026-09-23; report: ../../reports/CR-02-add-mode-manager.md |
| Approved by / date | Developer (user), 2026-09-22 — explicit “approve plan” instruction |
| Target project | `/home/robertvanvossen/dev/agentic-hamr-experiments-02/mavlink_02` |
| Baseline | `043d574970d28ff172f7261ea0392ddd14ae50a8`, confirmed by developer 2026-09-22 |
| Sketch | `add-mode-manager-sketch.md` (unchanged) |
| Authoritative requirements | `Open_Platform_HLRs_26_09_23_02.md` (developer supplied; supersedes _26_09_23_01; logging reconciled) |
| Profile | audited; user token `audit` interpreted as audited |
| Workflow | `ChangePlan(cr=CR-02-add-mode-manager)` |

## Requirements reconciliation — revision _26_09_23_02

The developer supplied `Open_Platform_HLRs_26_09_23_02.md` and reported requirements
reconciliation on 2026-09-23. This is the current source of truth. Its only change
from _26_09_23_01 is LLR-18: omit per-message Recovery-suppression diagnostics and
log ModeManager's Normal-to-Recovery transition once. This matches the implemented,
tested and approved behavior; no model, codegen, code or verification changes are
needed. The bounds, strict UDP policy and D2 deadline are unchanged.

The developer explicitly retired the four legacy CR-01 documents on 2026-09-23:
`requirements/conops.md`, `updated_reqs.md`, `component-requirements.md` and
`data-dictionary.md`. Retirement notices link to the current requirements and
CR-02 sketch/plan; their historical bodies, including ConOps §0, are preserved.
This disposition replaces their planned publication updates and closes the
remaining requirements reconciliation obligation. See
`reports/CR-02-closeout-requirements-review.md`. CR-02-HW-01 remains High/Open.

## Requirements amendment — 2026-09-23 (historical _01 scope)

The developer explicitly directs adding a defensive driver bounds check and updating
the firewalls against `Open_Platform_HLRs_26_09_23_01.md`. This authorizes the scope
amendment; the three waves and audited technical gates remain. References to _07
below record the original planning basis; _02 above is now authoritative.
No user-owned requirements are edited.

The supplied delta changes HLR-12/13/18's IPv4 maximum from 9000 to 1586 bytes
and retains LLR-4's 1600-byte Ethernet carrier. The revised Tx contract drops lengths above 1586;
Rx/MAVLink retain their existing stricter UDP/carrier checks. All four lanes retain
exact byte preservation. ARP, frozen mode, rejection counting and R2U2 semantics
are unchanged. The driver independently skips zero or out-of-buffer transmit sizes
before requesting a DMA token; it never truncates an oversized request.

During W3, re-enter CompGUMBOSpec for Tx/Rx/MAVLink (shared bound and Tx output-size
invariants), obtain contract sign-off, then CodeGen and the reporting probe, and
re-enter affected CompDev tests/coverage/verification. The historical workaround
used during this amendment is retired following the HAMR upgrade.
Align firewall_core and MAVLink executable length limits with the revised contracts.
Rebuild the full target after these changes. Earlier W2 proofs and W3 loader remain
historical evidence, not acceptance of the amended code.

Acceptance: valid IPv4 1586 produces Tx size 1600; 1587, 9000, 9001 and 65535
are rejected on every Tx lane. Rx direct/MAVLink routes reject oversize carriers;
MAVLink rejection counting remains correct in both modes. Exercise driver size 0,
1, 1599, 1600, 1601 and 65535 (the helper test exhausts all u16 values), preserving
valid bytes and continuing after bad lanes. Verify affected firewalls/shared core,
retain monitor regressions, and build the driver for ZCU102. Physical hardware and
timing acceptance remain separate outstanding W3 obligations.

### Execution budget amendment — 2026-09-23 (reverted by developer)

Developer explicitly requested +200 ms for ModeManager and MAVLinkFirewall after
reviewing the D2 timeout in manual_test_results/26_09_23_14_19_open_platform.log.
The attempted settings were ModeManager 300 ms, MAVLinkFirewall 500 ms and model frame 2480 ms.
The legacy schedule follows its existing scale (domain_7=30000, domain_6=50000),
with unchanged ordering. The developer subsequently reported no improvement and reverted this amendment.
Current settings are again ModeManager 100 ms, MAVLinkFirewall 300 ms, frame 2080 ms,
and legacy domain_7/domain_6 lengths 10000/30000. The timeout remains unresolved;
requirements edits remain developer-owned. See reports/CR-02-budget-increase.md
for validation and the unresolved physical-unit/hardware timing qualification.

## 1. Change Summary

Final-review disposition (developer instruction, 2026-09-23): the unresolved timing
issue CR-02-HW-01 is deferred to a future CR and does not block CR-02 completion.
The issue remains High/Open in `open-issues/CR-02-HW-01-recovery-timeout.md`; a
follow-up CR ID has not been assigned. This explicitly dispositions the unmet
nominal D2 timing evidence in the original W3 gate. Required monitor behavior,
the source requirements and the issue's closure criteria remain unchanged.

Add a periodic ModeManager component publishing Normal/Recovery mode to RxFirewall
and MAVLinkFirewall. MAVLinkFirewall aggregates malformed/blacklisted rejections
across four input lanes, saturates its lifetime counter at 20, and publishes Error
Status after processing each dispatch: true at a count of five or greater. ModeManager
latches Recovery until reboot and publishes the new mode in the dispatch that consumes
true. Both receiving firewalls suppress Ethernet forwarding as soon as Recovery is
present in their own frozen input snapshot. A HAMR-generated R2U2 monitor checks for Recovery by MAVLinkFirewall dispatch D2
after the first error assertion D0; verdict handling logs a timeout at most once per boot.

Also adopt the supplied stricter direct-UDP policy and retain SECURE_COMMAND operation
7 denial under new HLR-32. Preserve the four Ethernet lanes, bounded MAVLink carrier,
network/parser separation, and custom VMM integration. No ChangeExec work is performed
by this plan. The developer approved this plan and its proposed scope on 2026-09-22.
Approval completes ChangePlan; ChangeExec remains a separate workflow.

## 2. Sketch Provenance and Clarifications

- Immutable request files: sketch and supplied requirements revisions through
  `Open_Platform_HLRs_26_09_23_01.md`; that latest revision is authoritative.
- Developer, 2026-09-22: user-supplied requirements are the source of truth; adopt
  their stricter UDP policy. Baseline-derived requirements do not override them.
- Developer, 2026-09-22: inputs are frozen immediately before HAMR component dispatch.
  This governs all mode observation and timeout semantics below.
- Revision _05 added HLR-32 for SECURE_COMMAND message 11004, operation 7. It supersedes
  the earlier suggestion to extend HLR-19. HLR-25 retains the threshold of five;
  HLR-30 retains D2 logging, not the earlier suggested D3 logging.
- Revision _06 corrected HLR-19 (retained in _07): COMMAND_INT and COMMAND_LONG both use MAVLink v1
  bytes 34–35 and v2 bytes 38–39, matching the current implementation. RD-4 is
  resolved by the developer-supplied revision; no command-offset code change is planned.
- Developer, 2026-09-22: implement HLR-30 using a HAMR-generated R2U2 monitor
  (Resolved RD-8). This replaces the proposed hand-written deadline counter;
  requirements ownership and the exact D2/one-time logging behavior are unchanged.
- Sketch constraints: reuse/update firewall_core where needed; preserve Tx verification;
  readable abstractions; no application delegation to GUMBOX; retain reason logging;
  keep MAVLink policy out of Rx and networking policy out of MAVLink parsing; prefix
  every make invocation with `SYSTEM_MAKEFILE=custom.mk`.

Resolved (RD-1): The developer confirmed baseline commit
`043d574970d28ff172f7261ea0392ddd14ae50a8` on 2026-09-22. The supplied prefix
`043d574970d28ff172f7261ea0392ddd14ae50a` resolves to that full commit.

Resolved (RD-2): The developer accepted the three-wave structure and subsequently
approved the plan in this session on 2026-09-22. This approval satisfies the plan
review gate; no additional external review gate is required by the approved plan.

Resolved (RD-3): The developer's explicit plan approval accepts the testing,
verification, frozen-area boundaries, and standard report obligations in sections
5–9. Requirements ownership remains with the developer under RD-7.

Resolved (RD-7): The developer owns requirements and will manually update them based
on agent review feedback (developer instruction, 2026-09-22). This applies to the
requirements portion of W1 and subsequent requirements reconciliation: the agent
reviews, identifies gaps, and proposes wording in review artifacts; the developer
edits the requirements documents. The agent does not run SysPlanAndReq to author
those documents. Model, contract, implementation, and planning/report work retain
their existing assignments.

### Requirements planning update — revision _07

Developer direction on 2026-09-22 makes `Open_Platform_HLRs_26_09_22_07.md`,
including `RC_INSPECTA_00-LLR-1` through `RC_INSPECTA_00-LLR-21`, the consolidated
source of truth. Requirements planning and complete allocation are recorded in
`w1-requirements-planning-07.md`. This supersedes the _06 reference additions and
resolves the missing source requirements identified in W1 feedback. Use actual LLR
IDs, not the reference draft's CR02-DR IDs. The three-wave scope and resolved review
choices remain; LLR-20's manager-first/100 ms preference is incorporated below.
Legacy requirements documents remain historical until the developer reconciles them;
their synchronization is tracked in the final back-propagation sweep, not a requirement
to duplicate _07 before modeling. No requirement file is edited by this plan update.

## 3. Baseline

Tracked source was clean before planning. The CR-02 folder is untracked change
provenance, excluded from the committed baseline. Current tracked modifications are
planning updates to `reports/workflow-status.md`. Intake evidence is recorded in
`changeplan-intake.md`. Execution must compare drift against this pinned commit.

| Area | Observed state / evidence |
|---|---|
| Workflow | CR-01 recorded complete August 27; subsequent core consolidation recorded complete September 11 in project workflow status |
| Architecture | `sysmlv2/open_platform/open_platform.sysml`: five process instances, four event-data lanes per Ethernet connection, no ModeManager or control/status connections |
| Contracts | `open_platform_Software.sysml`: Rx and MAVLink valid-input forwarding is unconditional; no mode, lifetime count, or watchdog state |
| Data | `open_platform_Data_Model.sysml`: RawEthernetMessage is a fixed 1600-byte array; MAVLinkUDPMessage_Impl preserves frame and UDP payload offset/length |
| Deployment | Domains 2 ArduPilot, 3 Tx, 4 driver, 5 Rx, 6 MAVLink; pacer domain 1; Platform.sysml Max_Domain 7 |
| Schedule | `hamr/microkit/microkit.schedule.xml`: ArduPilot → Tx → driver → Rx → MAVLink with interleaved pacer slots; retained by regeneration |
| Build | `hamr/microkit/custom.mk` explicitly enumerates component images, monitors, types, Rust builds, test/verify targets, and manual VMM library linkage |
| Rx code | `crates/seL4_RxFirewall_RxFirewall/src/component/seL4_RxFirewall_RxFirewall_app.rs` uses firewall_core; shared GumboLib carries routing predicates |
| MAVLink code | Component owns classify_mavlink and firmware_flash_spec; mavlink_core provides verified framing/CRC parsing and dialect metadata |
| Latest recorded evidence | `reports/mavlink-core-consolidation.md`: core tests 6/6, MAVLink tests 8/8; core verification 9/0, MAVLink 17/0; no full-build/hardware/coverage rerun in that maintenance |
| Build helper | Neither root `bin/` nor `hamr/microkit/bin/` currently exists; reconcile old status claims with actual files and provide supported coverage orchestration before CompDev |

Historical test results are not current execution evidence. No tests or verification
were rerun for this planning-only change. No existing system-proof crate was identified
in the surveyed crate manifest list; do not claim an existing end-to-end mode proof.

## 4. Consistency Findings

| ID | Finding | Class | Resolution / plan treatment |
|---|---|---|---|
| F1 | Existing Rx/MAVLink forwarding guarantees conflict with Recovery suppression | impact | Guard forwarding by frozen Normal mode; preserve lane/no-input and validity obligations; add exhaustive Recovery clauses |
| F2 | Baseline direct UDP permits source 14550/destination 68; _07 forbids it | impact | Developer resolved: implement source !=14550 AND destination !=14562 AND whitelist membership; test the changed case explicitly |
| F3 | Baseline secure-command deny was previously absent from supplied HLRs | note | Resolved by _07 HLR-32; retain implementation behavior and trace it to HLR-32 |
| F4 | Old derived HLR-19–32 identifiers collide with new authoritative meanings | impact | Version-qualified migration map; authoritative IDs unchanged; move compatible derived details to distinct component/derived IDs; no silent renumbering of historical records |
| F5 | Mode publication and observation occur at different component dispatches | note | _07 and developer clarification resolve semantics; use each consumer's frozen snapshot, not global instantaneous mode changes |
| F6 | Earlier COMMAND_INT offset discrepancy resolved by revision _07 | note | HLR-19 now uses v1 bytes 34–35 and v2 bytes 38–39 for both command envelopes, matching current payload offset 28. No command-field implementation/specification change is needed; retain regression coverage and update traceability |
| F7 | Source well-formedness descriptions are less detailed than baseline refinements | note | User-supplied requirements remain authoritative. Preserve compatible bounds, framing, CRC, dialect and legal-truncation refinements as derived obligations; provide the developer feedback for any newly identified conflict rather than silently overriding supplied requirements |
| F8 | Missing build helper despite older workflow notes | impact | Establish actual build/coverage entry point during W1 using setup-build-script; do not rely on nonexistent paths |

Resolved (RD-4): Revision _07 supplies COMMAND_INT offsets matching the current
implementation: bytes 34–35 in v1 and 38–39 in v2. Both command envelopes retain
payload-relative offset 28. Source authority was already established by the developer;
the former command-offset delta is removed from the impact, risk, and acceptance
sections. Compatible baseline refinements remain subject to developer-owned
requirements review, not automatic edits or unrecorded new policy.

### Dispatch design

Use sampled DataPorts for current Mode and ErrorStatus because each producer publishes
a current value every dispatch. Retain EventDataPorts for Ethernet lanes. Initialize
Mode to Normal and ErrorStatus to false, and verify generated startup behavior gives
consumers Normal before the manager first runs. Model a two-value enum with Normal as
the explicit startup value; do not rely on an unchecked generated numeric default.

ModeManager state is a latched mode. Its compute transition is Recovery if prior mode
is Recovery or frozen ErrorStatus is true; otherwise Normal. Both outputs equal that
post-state on every dispatch. No false status can undo Recovery.

MAVLinkFirewall state includes count in 0..20 and the minimal trigger/reporting
latches needed to integrate the generated R2U2 monitor. R2U2 owns temporal tracking;
a separate hand-written deadline counter is not part of the planned implementation.
Count each malformed or blacklisted input once across all lanes; use saturating
addition. Proposed processing continues classification in Recovery so malformed or
blacklisted inputs remain countable, while mode-only suppression does not increment
the count. Preserve per-message reasons and add Recovery suppression diagnostics.
Publish `count >= 5` after count updates on every dispatch, including empty and
Recovery dispatches. State initialization and count monotonicity must be specified.

### HLR-30 generated R2U2 implementation

Resolved (RD-8): Generate the temporal monitor from a GUMBO monitor specification
attached to MAVLinkFirewall, using HAMR's Microkit/Rust R2U2 support. The monitor runs
within that component's dispatch path; no separate monitor process/domain is planned.

- Define one logical monitor sample per MAVLinkFirewall compute dispatch, including
  dispatches without Ethernet input. Observe the frozen Mode input through the
  generated pre-dispatch hook and the ErrorStatus output through the post-dispatch
  hook, after application processing and final status assignment. Step the monitor
  once afterward. Initialization must not shift the D0/D1/D2 indexing.
- Encode a single obligation triggered by the first true ErrorStatus assertion D0:
  Recovery must be observed by D2. Use a first-assertion predicate or minimal latch
  where needed; repeated true values must not start new obligations. Select and
  validate concrete GUMBO temporal syntax during W1 rather than treating this prose
  as an already compiled specification.
- At D2, a Recovery value in that frozen snapshot meets the deadline. Otherwise the
  generated monitor must make the violation available and the reporting integration
  must log the timeout during D2. D3 arrival or no arrival cannot erase the timeout.
- Implement one-time error reporting per boot. Default generated per-step status
  logging does not satisfy this requirement. Integrate supported verdict handling
  with a reporting latch, suppress routine repeated status reports for HLR-30, and
  preserve the integration across regeneration. An alert sent to a separately
  dispatched component alone is insufficient if it delays the required log past D2.
- Validate the supported hook/extension mechanism before committing to its generated
  file placement. Do not depend on edits in overwrite-only generated regions. If
  current hooks cannot deliver/log during D2, record the limitation and revise the
  plan for review; do not silently substitute D3 logging or a hand-written watchdog.
- Generate/compile the temporal specification and include the R2U2 Rust runtime,
  compiler, specification binary and build dependencies in the custom build. Pin and
  record the actual tool/runtime versions and generation options used.
- Keep ErrorStatus, counting, mode enforcement and packet policy independent of the
  monitor verdict. A timeout never enables forwarding or clears Recovery.

Installed generator-source evidence for feasibility: `GumboRustPlugin.scala` under
`$SIREUM_HOME/hamr/codegen/shared/src/main/scala/org/sireum/hamr/codegen/microkit/plugins/gumbo/`
emits pre/post-timeTriggered hooks and calls `r2u2_core::monitor_step` after sampling;
`GumboR2U2Util.scala` implements per-step status logging and mapped alert verdicts.
Source inspection is not a completed generation/build test for this project.

Verification boundary: Verus verifies application state/control and any supported
reporting-latch proof obligations. Generated R2U2 evaluation and compiler/runtime
correctness are not thereby proved. Test actual monitor verdicts and physical logging
with dispatch traces and record that runtime-monitor trust boundary explicitly.

Rx reads its frozen mode and suppresses all eight frame outputs in Recovery (four
direct plus four MAVLink carrier outputs). MAVLink suppresses its four frame outputs.
A D0 snapshot that is still Normal continues to use Normal forwarding rules for
otherwise allowed messages, even when another lane crosses the rejection threshold.
Mode updates published after input freezing affect a later dispatch only.

## 5. Impact Analysis

### 5.1 Impacted

| Artifact / element | Predicted impact | Wave |
|---|---|---|
| Consolidated _07 HLR/LLR source and legacy requirements/updated_reqs.md (developer-owned) | Agent feedback for developer edits: _07 supplies source content; allocation/ID map in w1-requirements-planning-07.md; legacy document synchronization remains developer-owned | W1 source review; final documentation sweep |
| requirements/component-requirements.md (developer-owned) | Agent feedback for developer edits: Allocate HLR-23/24/26/27 to ModeManager; HLR-25/28/30/31 to MAVLink; HLR-29 to Rx; correct old aliases | W1 |
| requirements/conops.md (developer-owned) | Agent feedback for developer edits: Dated CR-02 annotations, threshold/reboot scenarios, domain model, changed availability and direct UDP behavior | W1 |
| requirements/data-dictionary.md (developer-owned) | Agent feedback for developer edits: Mode enum, ErrorStatus, count 0..20, threshold 5, D0–D2, initial values, strict UDP and secure-command mapping | W1 |
| open_platform_Data_Model.sysml | Add Mode enum without changing Ethernet carrier layout | W1 |
| open_platform_Software.sysml | New ModeManager thread; sampled ports; initialization/compute/state contracts for all three affected components; MAVLink GUMBO temporal monitor for HLR-30; traceability correction | W1 |
| open_platform.sysml | New process wrapper/instance, error connection, two mode connections and wrapper delegation | W1 |
| open_platform_Properties.sysml, Platform.sysml | New unique domain (candidate 7), Max_Domain adjustment (candidate 8); verify generator convention | W1 |
| GumboLib.sysml and generated GumboLib | Strict direct-UDP predicate and reusable mode/counter abstractions where appropriate | W1/W2 |
| Rx application and firewall_core | Reuse verified network helpers; stricter routing, frozen-mode gate, logging, proofs/tests; change core only where justified by shared abstraction | W2 |
| MAVLink application/specification/tests | Counter, generated R2U2 integration and one-time verdict logging, frozen-mode gate, status publication; retained command-field handling with regression coverage and new traceability | W2 |
| R2U2 generated specification/runtime integration | GUMBO monitor lowering, generated pre/post-dispatch sampling, temporal specification binary, Rust dependencies and regeneration-safe verdict handling | W1/W2 |
| New ModeManager crate | Initialize, latch and publish mode; tests, coverage, Verus | W2 |
| Generated APIs/bridges/GUMBOX/data/monitors/system files | Regenerate after model/contracts, inspect marker preservation and startup control values | W1 |
| custom.mk | Add manager image/monitor/build/test/verify rules, control-type objects and R2U2 specification/runtime build dependencies; preserve manual VMM linkage | W1/W3 |
| microkit.schedule.xml | Add manager/pacer slot and validate actual observation order/latency; hand-maintained | W3 |
| Build helper | Provide bin/build.cmd through setup-build-script and include new crate/coverage path | W1 |
| Tx/Rx/MAVLink contracts and executable length validation | Shared IPv4 maximum 1586; Tx output-size invariants; refreshed boundary tests and proofs under latest requirements | W3 correction loop |
| LowLevelEthernetDriver transmit path | Independent buffer bounds check before token acquisition, no truncation; exhaustive helper tests and target build | W3 |
| Reports | Contract audits, component tests/coverage/verification, timing trace, hardware results and final CR-02 report | All |

### 5.2 Non-Impact Argument

| Element | Proposed non-impact and required evidence |
|---|---|
| TxFirewall mode interface | No mode port added. Length policy/contracts are now impacted by the 2026-09-23 amendment; full affected tests/verification required |
| LowLevelEthernetDriver interfaces | Four input/output lanes unchanged. Transmit functionality now includes the authorized defensive bounds check; test it and rebuild the target |
| VMM application/guest/virtio behavior | Carrier layout and receive/transmit queues unchanged; no manager connection to VMM. Preserve manual C/library linkage and exercise end-to-end traffic |
| mavlink_core framing/CRC/dialect | Mode and blacklist policy remain component-owned; no intended parser or dialect change. Rerun core regressions/verification; _07 requires no command-offset or parser change |
| Ethernet data layouts | Raw/Sized/MAVLink carrier layouts unchanged; new control enum is additive. Check generated C/Rust type compatibility |
| manual_reqs.md and archived sketches | Preserve historical input; _07 is the new authority and evolving requirements carry explicit provenance. Do not rewrite the original ConOps concept block |

Resolved (RD-5): The developer's plan approval accepts sampled control ports,
continued malformed/blacklisted rejection counting during Recovery, and the stated
non-impact boundaries. These are approved implementation choices; requirements
edits remain developer-owned.

## 6. Waves

| Wave | Intent | Bound workflows / actions | Expected deltas | Verification gate |
|---|---|---|---|---|
| W1 | Requirements, architecture, contracts and regeneration | Review developer-supplied consolidated _07 HLRs/LLRs and record allocation (complete); SysModeling (delta); CompGUMBOSpec(component=RxFirewall); CompGUMBOSpec(component=MAVLinkFirewall); CompGUMBOSpec(component=ModeManager); SysGUMBOIntegrationCheck; CodeGen; setup-build-script | Requirements/ID map, mode/status wiring and state contracts, new domain declarations, generated R2U2 specification/hooks, generated scaffold and build helper; preserve custom build content | Developer-supplied requirements updates reviewed and applicable findings resolved; SysML tipe clean; all three contract audits reviewed; integration check result with expected handshake count and explicit vacuity accounting; generation and R2U2 specification compilation succeed; sampling hooks/startup values inspected; demonstrate a supported same-dispatch verdict/logging path before W2. This is a model/scaffold gate, not a runnable-system claim |
| W2 | Implement and verify mode control and affected policies | CompDev(component=ModeManager); CompDev(component=RxFirewall); CompDev(component=MAVLinkFirewall); CompDev(component=TxFirewall)/TestOnly; CompDev(component=TxFirewall)/VerifyOnly; core test/verification commands | Executable state machines, R2U2 first-trigger/deadline monitoring and one-time logging, strict UDP, blacklist reconciliation, tests, coverage and proofs | Changed components' application/GUMBOX branch coverage and passing tests; Verus clean with R2U2 trust boundary recorded; actual monitor/verdict/logging trace tests pass; both core regressions pass; Tx tests/verification pass; any coverage gaps explicitly reviewed |
| W3 | Deployable integration and acceptance | 2026-09-23 bounds correction loop (CompGUMBOSpec → CodeGen → CompDev; independent driver guard); SysSchedDef; custom.mk reconciliation and full target build; dispatch-trace integration tests; manual ZCU102 acceptance; final test-components and verify; ChangeExec.3–.5 reconciliation/report | Valid schedule/domain map, complete loader, timing and hardware evidence, final requirements sweep and report | Full ZCU102 custom.mk build; D0/D1/D2 observation and R2U2 verdict/log evidence, including runtime budget impact; hardware cases pass; final cross-crate results and non-impact diffs reviewed; unresolved tool limits explicitly dispositioned |

W1's requirements input review is complete against developer-supplied _07. The
HLR/LLR allocation and feedback dispositions in `w1-requirements-planning-07.md`
are the planning basis for dependent model/contract work. Legacy document publication
remains developer-owned and is tracked for the final sweep. No agent requirements-
writing sub-workflow is assigned.

The developer accepted this three-wave structure on 2026-09-22.
Waves run sequentially. W1 generates changed interfaces before W2 implementation;
no intermediate scaffold is deployed. W3 closes integration only after component
gates pass. Every wave closes at ChangeExec.AP1, in addition to audited sub-workflow
and component audit points. Contract corrections loop through CompGUMBOSpec and
CodeGen with affected tests/proofs rerun; do not edit generated contracts in isolation.

SysSchedDef is draft: its schedule-order step does not establish timing-budget or
abstract-system-proof conformance. Per _07 LLR-20, prefer ModeManager as the first application component in the
schedule, with ModeManager at 100 ms and MAVLinkFirewall at 300 ms after the
developer reverted the unsuccessful budget increase. Preserve required pacer slots,
Rx at 300 ms, and existing 1000 ms configured periods.
Record ModeManager's period separately and check actual slot units and total budget.
Determine period/budget and communication visibility from generated deployment
semantics, then record a concrete trace demonstrating normal operation observes
Recovery by D2. Do not infer a wall-clock bound from a dispatch count or claim this
candidate schedule is already validated.

Resolved (RD-6): The developer's plan approval accepts affected-component proofs,
final cross-crate verification, and concrete timing/hardware tests. No new whole-system
temporal proof is included. The generated R2U2 monitor and its reporting integration
must satisfy the specified runtime evidence gates.

## 7. Back-Propagation Plan

Closeout disposition (2026-09-23): the developer supplied _26_09_23_02 to reconcile
LLR-18 and explicitly retired the four legacy documents listed below. Their
retirement notices supersede the original update obligations in this section.
Current requirements remain in the developer-supplied _02 document; historical
content and provenance remain available in place. Requirements reconciliation is
complete under this developer-directed disposition.

Update for _07: consolidated HLR/LLR source content is supplied and reviewed. The
legacy document actions below remain developer-owned publication/reconciliation
obligations; they do not block modeling against _07. Check them at ChangeExec.3 and
record any developer-directed retirement or replacement explicitly.

Requirements ownership remains with the developer. For the first four rows below,
the agent provides review feedback and suggested updates; the developer performs
the edits. At ChangeExec.3 the agent checks the supplied revisions and reports any
remaining discrepancies rather than changing requirements itself. The agent owns
model/contract traceability edits and the change report.

| Document | Update | When |
|---|---|---|
| ConOps | Preserve §0; annotate CR-02 changes to purpose, scenarios, mode/availability, reboot, actor/domain descriptions and constraints | W1; final sweep ChangeExec.3 |
| SysReqs | Incorporate _07 authoritative IDs; map old version-qualified obligations to new HLRs or named derived requirements; strict UDP and HLR-32 | W1; reconcile execution discoveries at ChangeExec.3 |
| CompReqs | Three affected component allocations, state/timing requirements, shared-core regression and independent implementation constraints | W1; sweep ChangeExec.3 |
| DataDict | New control values, startup, counter, watchdog and protocol offset conventions | W1; actual schedule details W3 |
| Model/contract traceability | Replace misleading baseline guarantee aliases; link each new guarantee to _07 HLR and derived obligation | W1; confirm after W2 |

Archive old requirement identities as `CR-01:RC_INSPECTA_00-HLR-n` in the migration
table, preserving their historical meaning. Do not assign a conflicting old derived
meaning to an authoritative CR-02 HLR. Retain only compatible details; source conflicts
follow the developer's authority decision and are documented as behavior changes.

## 8. Risks and Open Questions

1. Blacklist behavior must survive the added mode/counter logic. Retain independent
   byte-oriented fixtures for the _07 command offsets and secure operation 7; do not
   generate all tests using the same offset helper as the implementation. The earlier
   F6 offset discrepancy is resolved and creates no planned code delta.
2. Schedule, monitor tick alignment, verdict availability, and startup visibility may
   differ from an abstract ordering. Inspect generated freeze/publication/monitor
   paths and demonstrate same-D2 violation logging; include R2U2 execution and memory
   overhead in the deployment budget review.
3. Stateful GUMBO/Verus weaving may expose new tool limitations; isolate count/latch/
   reporting-latch proofs and preserve evidence. Treat R2U2 temporal evaluation as
   a separately tested runtime dependency, not a discharged Verus theorem. No unreviewed assumption or external-body
   escape for policy, state transitions, or mode gating.
4. Regeneration can disturb custom make rules or dependency wiring; review diffs and
   verify both core dependencies and the manual VMM linkage survive.
5. Shared network changes can break Tx proofs despite no Tx source changes; keep its
   verification gate mandatory.
6. Full branch coverage and physical logging are distinct evidence: exercise timeout
   once/never/late cases and observe diagnostics, not just oracle satisfaction.
7. Existing 9000-byte conceptual IPv4 upper bound versus fixed 1600-byte carrier is a
   baseline refinement boundary; do not silently expand capacity under this CR.
8. Hardware evidence requires developer/device availability. A passing host suite or
   loader build is not a hardware pass. Historical driver host-build limitations need
   target evidence or an explicit reviewed exception, not an invented success.

## 9. Acceptance Criteria and Report Obligations

- _07 is the authoritative requirement revision, including all LLR-1–21; complete HLR-to-contract/test mapping
  covers Rx 5/13/15/17/18/29, Tx 7/12/14/16, MAVLink 19/20/21/22/25/28/30/31/32,
  and ModeManager 23/24/26/27. The companion _07 planning report maps every LLR
  to its component, implementation obligation and verification wave.
- Strict UDP matrix includes source 14550/destination 68 rejection; other-source/68
  forwarding in Normal; 14550/14562 MAVLink routing; disallowed combinations dropping.
- Normal/Recovery and empty-input tests cover all four lanes and Rx's two outputs per
  lane; no Ethernet output in the first consumer dispatch observing Recovery.
- Counter tests cover zero, 4→5, multi-lane crossing, 19→20 with multiple rejections,
  already 20, overlap counted once, allowed/empty inputs, and Recovery classification.
- ErrorStatus reflects post-count state each dispatch; stays true after saturation.
- Tests executing the actual generated R2U2 monitor and reporting integration cover Recovery at D1 and D2 (no timeout), missing D2 (one timeout),
  D3 arrival, never-arriving Recovery, repeated true status, empty frame inputs,
  persistence without duplicate logs, and reboot reset. Assert verdict availability
  and exactly one physical timeout log during D2, not merely eventual failure.
  Include no-error traces, initial sampling alignment, and regeneration preservation.
- Manager tests cover initial Normal, false status, true transition/publication,
  Recovery retention after false, and reboot initialization.
- Blacklist fixtures cover both command envelopes/versions, HLR-32 secure operation 7,
  nonblacklisted operations, malformed inputs and legal truncation. COMMAND_INT and
  COMMAND_LONG fixtures use _07 offsets: v1 bytes 34–35 and v2 bytes 38–39.
- W2 gates prove functional state/control behavior and cover application/GUMBOX
  branches. W3 demonstrates freeze/dispatch propagation, target build, and hardware
  transition/reboot, normal routing, strict UDP, blacklist, and unchanged transmit.
- Run all make commands with `SYSTEM_MAKEFILE=custom.mk`; retain tool exit status,
  exact commands, build configuration, evidence paths and limitations in reports.
- Final report: `reports/CR-02-add-mode-manager.md`; include predicted/actual impact,
  deviations, non-impact evidence, requirement migration, coverage/proof results,
  schedule trace, hardware results, R2U2 compiler/runtime versions, monitor formula,
  verdict timing and reporting integration, trust boundaries, and unresolved limitations.

## 10. Review Record

| ID | Item | Decision | Rationale | Decided by | Date |
|---|---|---|---|---|---|
| RD-1 | Baseline commit (§2) | Resolved: `043d574970d28ff172f7261ea0392ddd14ae50a8` | Explicit developer confirmation; supplied prefix verified against repository | Developer (user) | 2026-09-22 |
| RD-2 | Three waves, approver and external-review needs (§2, §6) | Resolved: three waves approved; developer approval in this session satisfies review; no additional external review gate | Explicit developer approval of the current plan | Developer (user) | 2026-09-22 |
| RD-3 | Test/report expectations and frozen areas (§2, §5–9); requirements ownership resolved in RD-7 | Resolved: testing/report expectations and frozen-area boundaries accepted as written; requirements developer-owned | Explicit developer approval of the current plan | Developer (user) | 2026-09-22 |
| RD-4 | Requirement authority and COMMAND_INT offsets (§4 F6/F7) | Resolved: supplied _07 is authoritative; command offsets match current implementation | Developer previously established source authority; supplied _07 removes the offset discrepancy | Developer (supplied requirements and instruction) | 2026-09-22 |
| RD-5 | Sampled control ports, Recovery counting and non-impact boundaries (§4–5) | Resolved: sampled control ports, Recovery counting interpretation and non-impact boundaries accepted | Explicit developer approval of the current plan | Developer (user) | 2026-09-22 |
| RD-6 | Verification scope and schedule/temporal evidence (§6) | Resolved: affected-component proofs plus final cross-crate verification and timing/hardware tests; no new whole-system temporal proof | Explicit developer approval of the current plan | Developer (user) | 2026-09-22 |
| RD-7 | Requirements ownership (§2, §5–7) | Resolved: developer edits requirements manually; agent provides review feedback | Explicit developer instruction; replaces agent-authored SysPlanAndReq in W1 and governs later requirements reconciliation | Developer (user) | 2026-09-22 |
| RD-8 | HLR-30 implementation (§4–6, §8–9) | Resolved: HAMR-generated R2U2 temporal monitor with same-D2 one-time error reporting | Explicit developer instruction; replaces hand-written deadline counter; generation and timing remain to be validated | Developer (user) | 2026-09-22 |

Final approval: the developer explicitly instructed “approve plan” on 2026-09-22.
That instruction accepts the remaining proposed decisions above and approves
ChangePlan.AP1. All eight Review Record items are resolved. Material changes require
the ChangePlan.4–.6 revision loop and renewed approval. ChangeExec is in W1; requirements planning against _07 is complete.
Execution began after approval; see workflow status and the _07 requirements planning report for the current handoff.


### Execution clarification — retired generated-code workaround (2026-09-23)

The developer previously authorized capture and post-codegen application of commit
`4bc9a9ae60daefad311bcf23546ee8d4e7468c1b`. Updated HAMR now generates that fix
directly; fresh output matches the patched baseline and the probe passes 2/2.
The developer explicitly removed workaround patching from the workflow on
2026-09-23. Retain the patch/helper as historical artifacts; do not invoke them
after codegen. Continue the reporting probe against fresh generated output.
The three-wave scope, D2 deadline, one-time production logging obligation and
verification gates remain unchanged. See `reports/CR-02-hamr-upgrade.md` for the
comparison and `reports/CR-02-w1-gate.md` for historical workaround evidence.

### Manual testing disposition — 2026-09-23

Developer accepted manual testing with High/Open finding CR-02-HW-01 for the
Recovery observation timeout. Carry [the issue record](../../open-issues/CR-02-HW-01-recovery-timeout.md) into
Wave 3 review and the final change report. The +200 ms mitigation was ineffective
and reverted. Manual acceptance is complete; deadline conformance and finding
resolution are not claimed. Requirements remain developer-owned and unchanged.

## Wave 3 approval — 2026-09-23

Developer explicitly approved Wave 3 after the full verification/build recorded in
reports/CR-02-full-verify-build.md. The wave is accepted with CR-02-HW-01 retained
as High/Open. This supersedes earlier statements that Wave 3 approval was pending;
it does not resolve the timeout or grant final CR-02 completion approval.

## Final completion approval — 2026-09-23

The developer explicitly approved the final CR-02 review. All three waves,
requirements reconciliation, final validation and change reporting are complete.
ChangeExec.AP2 is approved and this plan is Executed. CR-02-HW-01 remains High/Open
in open-issues, deferred to a future CR and non-blocking for this completed change.
