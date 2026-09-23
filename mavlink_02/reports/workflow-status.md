# Workflow Status

Project: open_platform | Profile: audited | Updated: 2026-09-23

| Step | Status | Updated | Notes |
|------|--------|---------|-------|
| ChangeExec(CR-02) | in-progress | 2026-09-22 | Audited W1; CodeGen approved and build helper ready; generated R2U2 reporter loses D2 false verdict, blocking W1 gate |
| ChangeExec(CR-02).1 | done | 2026-09-22 | Approved plan/all RDs resolved; HEAD 24ba011d4f0a49c727a9b6c9cd28941f536891b4 drift is request/planning records only; approved source baseline unchanged |
| ChangeExec(CR-02).2 | in-progress | 2026-09-22 | W1 started per approved sequence; W2/W3 not started |
| ChangeExec(CR-02).W1 | blocked | 2026-09-22 | Raw R2U2 detects D2 violation, but generated latest-verdict cache hides it; reproducer/report ready; W2 not authorized to start |
| ChangeExec(CR-02).W1.Requirements | done | 2026-09-22 | _07 supplies derived LLR-1–21; allocation and RF-1–RF-4 disposition in w1-requirements-planning-07.md; legacy document synchronization tracked for final sweep |
| ChangeExec(CR-02).W1.SysModeling | done | 2026-09-22 | Developer approved architecture and audited sub-workflow boundary; continuing to RxFirewall contracts |
| ChangeExec(CR-02).W1.SysModeling.1 | done | 2026-09-22 | Retained existing package/file layout and local aadl-lib |
| ChangeExec(CR-02).W1.SysModeling.2 | done | 2026-09-22 | Added OperatingMode enum; existing carriers unchanged |
| ChangeExec(CR-02).W1.SysModeling.3 | done | 2026-09-22 | Added ModeManager thread/process and sampled control ports; 1000 ms period, 100 ms compute budget |
| ChangeExec(CR-02).W1.SysModeling.4 | done | 2026-09-22 | Added 3 control connections, domain 7, Max_Domain 8; existing Ethernet connections/bindings retained |
| ChangeExec(CR-02).W1.SysModeling.5 | done | 2026-09-22 | First tipe run exit 0 Well-formed; structural preservation checks passed; see w1-sysmodeling-report.md |
| ChangeExec(CR-02).W1.SysModeling.AP1 | done | 2026-09-22 | Developer explicitly approved architecture; tipe clean |
| ChangeExec(CR-02).W1.CompGUMBOSpec(RxFirewall) | done | 2026-09-22 | Developer approved contracts including shared rx_bounded_udp refactor and audited boundary |
| ChangeExec(CR-02).W1.CompGUMBOSpec(RxFirewall).1 | done | 2026-09-22 | Frozen Normal guards, Recovery suppression, initialization no-send, strict direct UDP policy and LLR-4 bounds |
| ChangeExec(CR-02).W1.CompGUMBOSpec(RxFirewall).2 | done | 2026-09-22 | First tipe run exit 0 Well-formed |
| ChangeExec(CR-02).W1.CompGUMBOSpec(RxFirewall).3 | done | 2026-09-22 | Zero AP-1–AP-9 model findings; generated GUMBOX stale until planned W1 CodeGen |
| ChangeExec(CR-02).W1.CompGUMBOSpec(RxFirewall).4 | done | 2026-09-22 | Developer review: both UDP routes reuse rx_bounded_udp; no unresolved catalog findings or waivers; implementation and refreshed oracle tests remain W2 |
| ChangeExec(CR-02).W1.CompGUMBOSpec(RxFirewall).AP1 | done | 2026-09-22 | Developer explicitly approved RxFirewall contracts |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall) | done | 2026-09-22 | Developer approved contracts and audited boundary; recorded W1/W2 obligations remain required |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall).1 | done | 2026-09-22 | Added saturated count, unconditional post-count status, frozen-mode guards, initialization and past-time R2U2 deadline formula |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall).2 | done | 2026-09-22 | Iteration 2 Well-formed; replaced unsupported thread invariant with inductive compute pre/post bound |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall).3 | done | 2026-09-22 | Zero new model-clause catalog findings; stale generated contracts and truncated SECURE_COMMAND developer-hook concern recorded |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall).4 | done | 2026-09-22 | Model revisions complete; W1 generated reporter feasibility and W2 hook refinement/runtime tests remain explicit obligations |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall).AP1 | done | 2026-09-22 | Developer approved contracts, refinement observation and runtime obligations; no implementation requirement waived |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall)-R2 | done | 2026-09-23 | Developer approved revised future-time monitor; CodeGen rerun follows |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall)-R2.1 | done | 2026-09-23 | _07 authoritative; first threshold-crossing trigger with Eventually[1,2] frozen Recovery; compute unchanged |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall)-R2.2 | done | 2026-09-23 | Fresh tipe exit 0: Well-formed! |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall)-R2.3 | done | 2026-09-23 | Full-mode audit: no new compute catalog findings; hook refinement, stale tests and D2 reporting obligations retained; see dated audit |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall)-R2.4 | done | 2026-09-23 | No further model revisions; generated temporal artifacts stale pending approval and CodeGen |
| ChangeExec(CR-02).W1.CompGUMBOSpec(MAVLinkFirewall)-R2.AP1 | done | 2026-09-23 | Developer explicitly approved revised monitor; implementation/reporting obligations retained |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager) | done | 2026-09-22 | Developer approved contracts, zero-finding audit and audited boundary |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager).1 | done | 2026-09-22 | Retained mode, initial Normal outputs, exact latched transition and post-state publication to both consumers |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager).2 | done | 2026-09-22 | First tipe run exit 0 Well-formed |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager).3 | done | 2026-09-22 | Specification-only audit: zero AP-1–AP-9 findings; all four state/input combinations covered |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager).4 | done | 2026-09-22 | No audit revisions or waivers required; startup/runtime publication checks remain W1/W2 |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager).AP1 | done | 2026-09-22 | Developer explicitly approved ModeManager contracts |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R2 | done | 2026-09-23 | Rerun with r2u2-HAMR-agent-context; unchanged contracts and existing approval retained; see CR-02-ModeManager-specification-audit-2026-09-23.md |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R2.1 | done | 2026-09-23 | Reviewed _07 requirements and new-context guidance; existing six guarantees need no revisions |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R2.2 | done | 2026-09-23 | Fresh tipe exit 0: Well-formed! |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R2.3 | done | 2026-09-23 | Full-mode audit: zero AP-1–AP-9 findings; inspected model, GUMBOX, woven ensures and tests |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R2.4 | done | 2026-09-23 | No revisions or demonstration tests required; no CodeGen needed; implementation remains W2 |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R2.AP1 | done | 2026-09-23 | Prior explicit developer approval applies to unchanged contracts; no new waiver or approval requested |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R3 | done | 2026-09-23 | Repeat requested; context cf109a5; unchanged ModeManager contracts; audit report R3 addendum |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R3.1 | done | 2026-09-23 | Rechecked _07, new-context rules and current model; preserved pre-existing MAVLink monitor removal |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R3.2 | done | 2026-09-23 | Fresh tipe: Well-formed!, exit 0 |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R3.3 | done | 2026-09-23 | Zero AP-1–AP-9 findings; ModeManager generated contracts remain consistent |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R3.4 | done | 2026-09-23 | No revisions or regeneration required for ModeManager; HLR-30 monitor removal is separate outstanding work |
| ChangeExec(CR-02).W1.CompGUMBOSpec(ModeManager)-R3.AP1 | done | 2026-09-23 | Prior explicit approval retained for unchanged contracts |
| ChangeExec(CR-02).W1.SysGUMBOIntegrationCheck | done | 2026-09-22 | Exit criteria met: Logika exit 0, vacuous by design N=0; no scratch; audited boundary approval pending |
| ChangeExec(CR-02).W1.SysGUMBOIntegrationCheck.1 | done | 2026-09-22 | Enumerated zero receiver integration assumes before run; no diagnostics/output artifacts; see reports/CR-02-integration-check.md |
| ChangeExec(CR-02).W1.SysGUMBOIntegrationCheck.2 | n/a | 2026-09-22 | No integration failures to diagnose |
| ChangeExec(CR-02).W1.SysGUMBOIntegrationCheck.3 | n/a | 2026-09-22 | No contract revisions required |
| ChangeExec(CR-02).W1.SysGUMBOIntegrationCheck.boundary | done | 2026-09-22 | Developer approved N=0 result and continuation to CodeGen |
| ChangeExec(CR-02).W1.CodeGen | done | 2026-09-22 | Developer approved successful generation and recorded limitations |
| ChangeExec(CR-02).W1.CodeGen.1 | done | 2026-09-22 | Microkit output ../../hamr, workspace ../.., Platform::ZCU102_Impl; frame-period prerequisite raised to 2080 ms |
| ChangeExec(CR-02).W1.CodeGen.2 | done | 2026-09-22 | Generation and attestation reporting passed after developer HAMR update; earlier frame/marker/parser failures resolved |
| ChangeExec(CR-02).W1.CodeGen.3 | done | 2026-09-22 | Project output confirmed; custom.mk/schedule/tests preserved; R2U2 spec recompiled with CLI 4.2.4; see reports/CR-02-codegen.md |
| ChangeExec(CR-02).W1.CodeGen.4 | n/a | 2026-09-22 | Regeneration; setup-build-script remains a separate approved-plan task |
| ChangeExec(CR-02).W1.CodeGen.boundary | done | 2026-09-22 | Developer explicitly approved CodeGen; W1 reporting/startup obligations remain |
| ChangeExec(CR-02).W1.CodeGen-R2 | in-progress | 2026-09-23 | Approved monitor regenerated successfully; artifact criteria met, audited boundary review pending |
| ChangeExec(CR-02).W1.CodeGen-R2.1 | done | 2026-09-23 | Reused Microkit configuration and recorded invocation |
| ChangeExec(CR-02).W1.CodeGen-R2.2 | done | 2026-09-23 | Exit 0, report Success, no warnings/errors; future-time spec emitted |
| ChangeExec(CR-02).W1.CodeGen-R2.3 | done | 2026-09-23 | 12 editable files unchanged by hashes; compiler 4.2.4 rebuilt specification and bounds |
| ChangeExec(CR-02).W1.CodeGen-R2.4 | n/a | 2026-09-23 | Regeneration; existing build helper retained |
| ChangeExec(CR-02).W1.CodeGen-R2.boundary | pending | 2026-09-23 | Generated artifact review ready; reporting prerequisite remains blocked |
| ChangeExec(CR-02).W1.R2U2Reporting-R2 | blocked | 2026-09-23 | New future formula reproduces verdict loss: raw test passes, reporting test fails; future-output evidence recorded |
| ChangeExec(CR-02).W1.SetupBuildScript | done | 2026-09-22 | Microkit/Rust helper created; usage parses, lists five components and two cores; application tests not run |
| ChangeExec(CR-02).W1.R2U2Reporting | blocked | 2026-09-22 | Probe: raw verdict test passes, generated reporting test fails; false D2 verdict overwritten by same-step true; reports/CR-02-w1-r2u2-reporting.md |
| ChangeExec(CR-02).W1.AP1 | blocked | 2026-09-22 | Same-dispatch one-time reporting prerequisite not met; generator must expose raw violations before cache overwrite |
| ChangeExec(CR-02).W2 | not-started | 2026-09-22 | Awaiting W1 gate |
| ChangeExec(CR-02).W3 | not-started | 2026-09-22 | Awaiting W2 gate |
| ChangeExec(CR-02).3 | not-started | 2026-09-22 | Back-propagation review follows waves; requirements developer-owned |
| ChangeExec(CR-02).4 | not-started | 2026-09-22 | Final tests/verification pending |
| ChangeExec(CR-02).5 | not-started | 2026-09-22 | Final change report pending |
| ChangeExec(CR-02).AP2 | not-started | 2026-09-22 | Completion review pending |
| ChangePlan(CR-02) | done | 2026-09-22 | Approved three-wave plan updated per developer direction to _07 consolidated HLR/LLR source; execution in W1 |
| ChangePlan(CR-02).1 | done | 2026-09-22 | Sketch preserved; _07 authoritative HLR/LLR source; developer owns requirements |
| ChangePlan(CR-02).2 | done | 2026-09-22 | Surveyed 043d574970d28ff172f7261ea0392ddd14ae50a8; untracked request provenance and historical validation limits recorded; developer confirmed execution baseline (RD-1 resolved) |
| ChangePlan(CR-02).3 | done | 2026-09-22 | Revision _06 resolves COMMAND_INT discrepancy; no offset code delta; authority and secure deny resolved; contract/ID/UDP impacts retained |
| ChangePlan(CR-02).4 | done | 2026-09-22 | _07 LLR allocation and manager-first/100 ms preference incorporated; no added wave or component |
| ChangePlan(CR-02).5 | done | 2026-09-22 | Three waves approved, including R2U2 generation, verdict/logging tests and target evidence; review and verification scope resolved |
| ChangePlan(CR-02).6 | done | 2026-09-22 | Plan and source references updated for _07; all existing Review Record decisions retained |
| ChangePlan(CR-02).AP1 | done | 2026-09-22 | Developer (user) explicitly approved plan on 2026-09-22; remaining proposed scope accepted; no additional external review gate |
| ChangePlan(CR-01) | done | 2026-08-26 | Exit criteria met; plan approved by Robbie VanVossen |
| ChangePlan(CR-01).1 | done | 2026-08-26 | Existing sketch resolved and preserved; audited ChangeScope answers recorded in change-plan.md |
| ChangePlan(CR-01).2 | done | 2026-08-26 | Baseline pinned at 7a99f23; no pre-existing workflow/test/verification reports found |
| ChangePlan(CR-01).3 | done | 2026-08-26 | Iteration 2: amended sketch checked; shared firewall_core creates TxFirewall verification impact, not source impact |
| ChangePlan(CR-01).4 | done | 2026-08-26 | Iteration 2: TxFirewall VerifyOnly evidence added; model/contracts/application remain explicit non-impact |
| ChangePlan(CR-01).5 | done | 2026-08-26 | Iteration 2: W2 now gates shared firewall_core changes on clean TxFirewall verification |
| ChangePlan(CR-01).6 | done | 2026-08-26 | Iteration 2: sketch drift and resulting plan revision recorded; approved at AP1 |
| ChangePlan(CR-01).AP1 | done | 2026-08-26 | Approved by Robbie VanVossen on 2026-08-26; ChangeExec unlocked |
| ChangeExec(CR-01) | done | 2026-08-27 | Exit criteria met; all four waves, back-propagation, final pass, change report, and audited completion review approved by Robbie VanVossen |
| ChangeExec(CR-01).1 | done | 2026-08-26 | Approved plan consistent; drift since 7a99f23 limited to reviewed sketch amendment, plan, and workflow status |
| ChangeExec(CR-01).2 | done | 2026-08-27 | W1-W4 executed; W4 hardware and refreshed software gate pass after MAVLink v2 truncation correction |
| SysPlanAndReq | done | 2026-08-26 | Exit criteria met; requirements approved by developer |
| SysPlanAndReq.1 | done | 2026-08-26 | ConOps records actors, fail-closed behavior, Rx-equivalent timing/capacity, and CR provenance |
| SysPlanAndReq.2 | done | 2026-08-26 | updated_reqs.md retains HLR naming; gaps resolved with exact routing, failure, timing, and capacity criteria |
| SysPlanAndReq.3 | done | 2026-08-26 | Component requirements allocate HLRs to RxFirewall, MAVLinkFirewall, VMM, shared core, and frozen components |
| SysPlanAndReq.4 | done | 2026-08-26 | Data dictionary defines networking/MAVLink fields, ranges, routing, lanes, and timing |
| SysPlanAndReq.AP1 | done | 2026-08-26 | Developer approved ConOps, SysReqs, CompReqs, and DataDict |
| ChangeExec(CR-01).W1.SysPlanAndReq | done | 2026-08-26 | Audited sub-workflow boundary approved; continuing to delta SysModeling |
| SysModeling | done | 2026-08-26 | Exit criteria met after bounded-payload iteration; architecture approved |
| SysModeling.1 | done | 2026-08-26 | Existing model-package structure retained |
| SysModeling.2 | done | 2026-08-26 | Iteration 1: added MAVLinkUDPMessage_Impl with preserved frame and payload offset/length |
| SysModeling.3 | done | 2026-08-26 | Iteration 2: bounded-payload carrier is preserved from RxFirewall through MAVLinkFirewall to VMM; VMM unwraps ethernet_frame |
| SysModeling.4 | done | 2026-08-26 | Added process wrapper, domain 6, Max_Domain 7, and eight receive-path connections |
| SysModeling.5 | done | 2026-08-26 | Iteration 1: renamed reserved field frame to ethernet_frame; tipe Well-formed |
| SysModeling.AP1 | done | 2026-08-26 | Developer approved revised bounded-payload architecture |
| ChangeExec(CR-01).W1.SysModeling | done | 2026-08-26 | Audited sub-workflow boundary approved; continuing to RxFirewall contracts |
| CompGUMBOSpec(RxFirewall) | done | 2026-08-26 | Exit criteria met; getter-refactored contract approved |
| CompGUMBOSpec(RxFirewall).1 | done | 2026-08-26 | Replaced TCP contract with exhaustive direct/MAVLink/drop/no-input clauses and output invariants |
| CompGUMBOSpec(RxFirewall).2 | done | 2026-08-26 | Iteration 1 getter refactor; sireum hamr sysml tipe: Well-formed |
| CompGUMBOSpec(RxFirewall).3 | done | 2026-08-26 | audit-gumbo-contracts: zero AP-1–AP-9 findings; generated GUMBOX noted stale until CodeGen |
| CompGUMBOSpec(RxFirewall).4 | done | 2026-08-26 | Review feedback: centralized layout and length logic in getter/spec functions; corrected ipv4_length to network byte order |
| CompGUMBOSpec(RxFirewall).AP1 | done | 2026-08-26 | Developer approved getter-refactored RxFirewall contract and zero-finding audit |
| ChangeExec(CR-01).W1.CompGUMBOSpec(RxFirewall) | done | 2026-08-26 | Audited sub-workflow boundary approved; continuing to MAVLinkFirewall contracts |
| CompGUMBOSpec(MAVLinkFirewall) | done | 2026-08-26 | Exit criteria met; carrier-preserving fail-closed contract approved with deliberate uninterpreted spec predicates |
| CompGUMBOSpec(MAVLinkFirewall).1 | done | 2026-08-26 | Added four-lane allow, firmware-flash deny, invalid, and no-input guarantees plus output invariants |
| CompGUMBOSpec(MAVLinkFirewall).2 | done | 2026-08-26 | Preserved MAVLinkUDPMessage_Impl through the firewall to VMM; sireum hamr sysml tipe: Well-formed |
| CompGUMBOSpec(MAVLinkFirewall).3 | done | 2026-08-26 | audit-gumbo-contracts specification-only mode: zero AP-1–AP-9 findings; abstract predicate refinement recorded for W2 |
| CompGUMBOSpec(MAVLinkFirewall).4 | done | 2026-08-26 | Developer accepted deliberate uninterpreted mavlink_frame_valid and mavlink_firmware_flash_command proof boundary |
| CompGUMBOSpec(MAVLinkFirewall).AP1 | done | 2026-08-26 | Developer approved MAVLinkFirewall contracts on 2026-08-26 |
| ChangeExec(CR-01).W1.CompGUMBOSpec(MAVLinkFirewall) | done | 2026-08-26 | Audited sub-workflow boundary approved; continuing to system GUMBO integration check |
| SysGUMBOIntegrationCheck | done | 2026-08-26 | Exit criteria met; Logika returned exit 0 |
| SysGUMBOIntegrationCheck.1 | done | 2026-08-26 | Expected receiver-side integration handshakes N=0; pass is vacuous by reusable-contract design; no scratch artifacts produced |
| SysGUMBOIntegrationCheck.2 | n/a | 2026-08-26 | No Logika failures to diagnose |
| SysGUMBOIntegrationCheck.3 | n/a | 2026-08-26 | No contract revisions required by Logika |
| ChangeExec(CR-01).W1.SysGUMBOIntegrationCheck | done | 2026-08-26 | Audited sub-workflow boundary approved; continuing to CodeGen |
| CodeGen | done | 2026-08-26 | Generation completed successfully under hamr/microkit; awaiting audited sub-workflow boundary approval |
| CodeGen.1 | done | 2026-08-26 | Existing model configuration confirmed: Microkit, ../../hamr, workspace root ../..; selected Platform::ZCU102_Impl at line 13 |
| CodeGen.2 | done | 2026-08-26 | Iteration 1 selected the unbound inner system and failed; iteration 2 exposed missing new-component markers; iteration 3 generated successfully after marker reconciliation |
| CodeGen.3 | done | 2026-08-26 | Output confirmed under project hamr/microkit; VMM/driver custom marker content preserved; generated fixme scratch removed; report status Success |
| CodeGen.4 | n/a | 2026-08-26 | Existing generated project; first-generation build-script setup does not apply |
| ChangeExec(CR-01).W1.CodeGen | done | 2026-08-26 | Audited sub-workflow boundary approved; W1 complete |
| ChangeExec(CR-01).W1 | done | 2026-08-26 | Requirements, model, contracts, integration check, and CodeGen approved |
| ChangeExec(CR-01).W2 | done | 2026-08-27 | Developer approved affected-component implementation and verification wave on 2026-08-27 |
| CompDev(RxFirewall) | done | 2026-08-26 | Exit criteria met: tests and full coverage pass; Verus 27/0; only justified platform logging adapters remain external |
| CompDev(RxFirewall).1 | done | 2026-08-26 | Model tipe-clean and CodeGen current; generated RxFirewall APIs include direct and bounded-MAVLink outputs |
| CompGUMBOSpec(MAVLinkFirewall)-W2R1 | done | 2026-08-26 | Component-local spec-function refactor approved |
| CompGUMBOSpec(MAVLinkFirewall)-W2R1.1 | done | 2026-08-26 | Component owns mavlink_frame_valid, mavlink_firmware_flash_command, and derived mavlink_allowed; GumboLib retains valid_mavlink_carrier |
| CompGUMBOSpec(MAVLinkFirewall)-W2R1.2 | done | 2026-08-26 | sireum hamr sysml tipe Platform.sysml: Well-formed |
| CompGUMBOSpec(MAVLinkFirewall)-W2R1.3 | done | 2026-08-26 | Audit refreshed; zero AP-1–AP-9 findings and unchanged implementation proof obligations |
| CompGUMBOSpec(MAVLinkFirewall)-W2R1.4 | done | 2026-08-26 | Developer accepted component ownership of MAVLink semantic predicates |
| CompGUMBOSpec(MAVLinkFirewall)-W2R1.AP1 | done | 2026-08-26 | Developer approved revised contract on 2026-08-26 |
| CodeGen-W2R1 | done | 2026-08-26 | Regenerated successfully after approved component-local function refactor; GumboLib no longer depends on MAVLink predicate hooks |
| CompDev(RxFirewall).2 | done | 2026-08-26 | Replaced legacy dispatch with four-lane fail-closed routing; RxFirewall owns all Direct/MAVLink/Drop policy, consumes generic EthFrame parse results, and makes no runtime GUMBO-helper calls |
| CompDev(RxFirewall).3 | done | 2026-08-26 | Added lane-isolation, malformed-length, disallowed-UDP, ARP/IPv6 boundary, no-input, notification, and exhaustive per-lane GUMBOX partition tests; firewall_core remains policy-neutral while exposing generic UDP source-port, destination-port, and length fields |
| CompDev(RxFirewall).4 | done | 2026-08-26 | 10 Rx tests and 17 firewall_core tests pass; post-proof-refactor instrumentation covers app 96/96 lines and GUMBOX 285/285 lines |
| CompDev(RxFirewall).5 | done | 2026-08-26 | Full active application and contract-oracle paths exercised; grcov reports 100% line coverage for both Rx target files |
| CompDev(RxFirewall).AP1 | done | 2026-08-26 | Developer approved 10/10 Rx tests, 17/17 firewall_core tests, and 100% Rx app/GUMBOX line coverage |
| CompDev(RxFirewall).6 | done | 2026-08-26 | Iteration 2 RxFirewall Verus pass: 27 verified, 0 errors; firewall_core reports 39/0 after strengthening generic UDP parser postconditions |
| CompDev(RxFirewall).7 | done | 2026-08-26 | Removed external_body from classifier and carrier constructor; proved parser/GUMBO equivalence and single allowed UDP port; only three platform logging adapters remain external |
| CompDev(RxFirewall).AP2 | done | 2026-08-26 | Developer approved Verus 27/0 and the three remaining platform logging adapter external bodies |
| CompDev(RxFirewall).cross-crate-diagnostic | done | 2026-08-26 | TxFirewall passes 16/0; LowLevel driver blocked before verification by missing SEL4_INCLUDE_DIRS/SEL4_PREFIX; MAVLinkFirewall awaits its planned developer MAVLink predicate hooks |
| ChangeExec(CR-01).W2.CompDev(RxFirewall) | done | 2026-08-26 | Developer approved audited RxFirewall sub-workflow boundary and continued W2 |
| CompDev(TxFirewall)/VerifyOnly.6 | done | 2026-08-26 | Current shared firewall_core verifies 39/0 and frozen TxFirewall verifies 16/0 |
| CompDev(TxFirewall)/VerifyOnly.7 | done | 2026-08-26 | No verification iteration required; TxFirewall component, bridge, tests.rs, and lib.rs are unchanged from baseline 7a99f23 |
| CompDev(TxFirewall)/VerifyOnly.AP2 | done | 2026-08-26 | Developer accepted 16/0 verification and generator-only codegen drift |
| CompDev(TxFirewall)/VerifyOnly | done | 2026-08-26 | Exit criteria for VerifyOnly met without TxFirewall behavioral or proof regression |
| CompDev(MAVLinkFirewall) | done | 2026-08-27 | Exit criteria met: approved full coverage, 7/7 component tests, 3/3 core tests, MAVLinkFirewall 16/0, and mavlink_core 7/0 |
| CompDev(MAVLinkFirewall).1 | done | 2026-08-26 | Model is Well-formed with aadl-lib source path; generated component-local predicate hooks and four-lane APIs are current |
| CompDev(MAVLinkFirewall).2 | done | 2026-08-26 | Added separate policy-neutral mavlink_core with generated 284-message CRC/min/max metadata and exact v1/v2 framing/checksum/signature checks; component owns flash-command policy, four-lane routing, and reason logging |
| CompDev(MAVLinkFirewall).3 | done | 2026-08-26 | Added FTP, non-flash, COMMAND_LONG flash, secure-operation-7, malformed checksum, lane isolation, no-input, notification, per-lane GUMBOX partition, signed-v2, v1, bounds, flags, length, unknown-ID, and checksum tests |
| CompDev(MAVLinkFirewall).4 | done | 2026-08-26 | 7 MAVLinkFirewall tests and 3 mavlink_core tests pass; coverage is app 53/53, GUMBOX 191/191, and policy-neutral parser lib 111/111 lines |
| CompDev(MAVLinkFirewall).5 | done | 2026-08-26 | Full active application, contract-oracle, and parser branches exercised; generated 284-arm dialect lookup recorded as mechanical metadata |
| CompDev(MAVLinkFirewall).AP1 | done | 2026-08-26 | Developer approved 7/7 component tests, 3/3 parser tests, and 100% app/GUMBOX/parser line coverage |
| CompDev(MAVLinkFirewall).6 | done | 2026-08-26 | Iteration 2 Verus pass: 14 verified, 0 errors; Rust-only parser inspection moved outside verus block |
| CompDev(MAVLinkFirewall).7 | done | 2026-08-27 | Removed classifier trust boundary: concrete component-owned specs refine verified mavlink_core framing/CRC/policy; only log_info and log_warn_channel retain external_body; no assumptions or assume_specification |
| CompDev(MAVLinkFirewall).8 | done | 2026-08-27 | Post-CodeGen verification/tests pass: MAVLinkFirewall 16/0 and 7/7 tests; mavlink_core 7/0 and 3/3 tests; shared-contract regression RxFirewall 27/0 and TxFirewall 16/0 |
| CompDev(MAVLinkFirewall).AP2 | done | 2026-08-27 | Developer approved trust-boundary removal and affected-component verification results on 2026-08-27 |
| CompGUMBOSpec(MAVLinkFirewall)-W2R2 | done | 2026-08-27 | Overflow-safe carrier contract and concrete verified MAVLink predicates approved; exit criteria met |
| CompGUMBOSpec(MAVLinkFirewall)-W2R2.1 | done | 2026-08-27 | Concrete mavlink_core specs cover v1/v2 framing, exact carrier bounds, 284-message metadata, X.25 CRC, and flash classification; standalone Verus passes 7/0; valid_ardupilot_udp now uses guarded subtraction to exclude u16 wraparound |
| CompGUMBOSpec(MAVLinkFirewall)-W2R2.2 | done | 2026-08-27 | sireum hamr sysml tipe --sourcepath ../aadl-lib:. Platform.sysml: Well-formed |
| CompGUMBOSpec(MAVLinkFirewall)-W2R2.3 | done | 2026-08-27 | audit-gumbo-contracts full audit refreshed: zero AP-1–AP-9 findings; resolved unsigned length-wrap observation and concrete verified predicate refinement recorded |
| CompGUMBOSpec(MAVLinkFirewall)-W2R2.4 | done | 2026-08-27 | Contract revisions complete; no catalog finding-demonstration tests required |
| CompGUMBOSpec(MAVLinkFirewall)-W2R2.AP1 | done | 2026-08-27 | Developer approved revised contract and audit on 2026-08-27 |
| CodeGen-W2R2 | done | 2026-08-27 | Regeneration exit criteria met; woven overflow-safe contract verified by affected components |
| CodeGen-W2R2.1 | done | 2026-08-27 | Reused recorded Microkit configuration from Platform.sysml: output ../../hamr with workspace root ../.. |
| CodeGen-W2R2.2 | done | 2026-08-27 | HAMR SysML code generation completed successfully; generated GumboLib now contains guarded IPv4-minus-header UDP length predicate |
| CodeGen-W2R2.3 | done | 2026-08-27 | Output confirmed under project hamr/microkit; editable MAVLink app/tests, mavlink_core dependency, and verified core preserved |
| CodeGen-W2R2.4 | n/a | 2026-08-27 | Existing bin/build.cmd retained; regeneration rather than first generation |
| ChangeExec(CR-01).W2.CompDev(MAVLinkFirewall) | done | 2026-08-27 | Audited CompDev boundary approved; MAVLinkFirewall implementation and verified parser complete |
| ChangeExec(CR-01).W2.AP1 | done | 2026-08-27 | Developer approved W2 wave gate on 2026-08-27 |
| ChangeExec(CR-01).W3 | done | 2026-08-27 | Developer approved VMM/custom.mk integration, full build, and domain-6 schedule on 2026-08-27 |
| ChangeExec(CR-01).W3.VMM | done | 2026-08-27 | VMM drains all four MAVLink carrier queues and delivers each preserved ethernet_frame through the existing virtio receive backend; generated direct and transmit paths retained |
| SysSchedDef | done | 2026-08-27 | Draft workflow exit criteria met for concrete schedule ordering/domain mapping; broader timing/schema analysis remains TBD |
| SysSchedDef.1 | done | 2026-08-27 | Added domain 6 after RxFirewall so cyclic order is driver 4 -> Rx 5 -> MAVLink 6 -> ArduPilot 2; retained existing slots and 30,000-unit firewall allocation |
| SysSchedDef.2 | n/a | 2026-08-27 | Workflow step is explicitly TBD; limitation recorded for W3 review |
| ChangeExec(CR-01).W3.Build | done | 2026-08-27 | Updated custom.mk with MAVLink image/type/rules and VMM link; full SYSTEM_MAKEFILE=custom.mk ZCU102 debug build passes and Microkit produced 147.98 MiB loader image |
| ChangeExec(CR-01).W3.AP1 | done | 2026-08-27 | Developer approved W3 wave gate on 2026-08-27 |
| ChangeExec(CR-01).W4 | done | 2026-08-27 | Manual ZCU102 procedure and refreshed software/build gate passed and were approved by Robbie VanVossen |
| ChangeExec(CR-01).W4.Hardware | done | 2026-08-27 | Robbie VanVossen reports all 12 cases pass; console and packet evidence recorded, including 221/221 valid messages after truncation correction |
| ChangeExec(CR-01).W4.MAVLinkV2Iteration | done | 2026-08-27 | Hardware exposed legal trailing-zero payload truncation rejected as too short; runtime and verified v2 length rules corrected, policy reads bounded, and requirements back-propagation staged |
| ChangeExec(CR-01).W4.Tests | done | 2026-08-27 | Fresh host runs: firewall_core 17/17, mavlink_core 4/4, MAVLinkFirewall 7/7, RxFirewall 10/10, TxFirewall 4/4; frozen LowLevel host test retains missing seL4 build-environment limitation |
| ChangeExec(CR-01).W4.Verify | done | 2026-08-27 | Fresh Verus: firewall_core 39/0, mavlink_core 7/0 via MAVLink build, MAVLinkFirewall 16/0, RxFirewall 27/0, TxFirewall 16/0; full custom.mk ZCU102 build passes including frozen LowLevel |
| ChangeExec(CR-01).W4.Build | done | 2026-08-27 | Full SYSTEM_MAKEFILE=custom.mk ZCU102/debug rebuild passes; 147.98 MiB loader SHA-256 f0189ae8ea42ef2e46b1257510ccf2544c2c420cbe2e0c6cce7624be978156e9 |
| ChangeExec(CR-01).W4.AP1 | done | 2026-08-27 | Robbie VanVossen approved the W4 hardware/software/build wave gate |
| ChangeExec(CR-01).3 | done | 2026-08-27 | Back-propagation sweep complete; manual_reqs.md unchanged and HLR-21, component allocation, and DataDict clarify legal MAVLink v2 trailing-zero truncation |
| ChangeExec(CR-01).4 | done | 2026-08-27 | Final pass consolidated fresh tests, Verus, 221/221 capture validation, and passing full custom.mk ZCU102 build; known LowLevel host-only limitation retained |
| ChangeExec(CR-01).5 | done | 2026-08-27 | Standard change report written at reports/CR-01-add-mavlink-firewall.md with traceability, deviations, non-impact evidence, and verification summary |
| ChangeExec(CR-01).AP2 | done | 2026-08-27 | Robbie VanVossen approved the completed change, final report, impact reconciliation, and evidence |

## Subsequent maintenance

- 2026-09-11: Developer-requested MAVLink core consolidation completed, including follow-up policy separation. One verified core parser serves routing, runtime contracts, and diagnostics; firmware-flash implementation/specification/tests reside in MAVLinkFirewall. Duplicate parser/table removed. Core tests 6/6, component tests 8/8; target verification core 9/0 and MAVLinkFirewall 17/0. Details: [mavlink-core-consolidation.md](mavlink-core-consolidation.md).
