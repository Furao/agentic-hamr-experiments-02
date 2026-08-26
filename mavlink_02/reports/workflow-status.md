# Workflow Status

Project: open_platform | Profile: audited | Updated: 2026-08-26

| Step | Status | Updated | Notes |
|------|--------|---------|-------|
| ChangePlan(CR-01) | done | 2026-08-26 | Exit criteria met; plan approved by Robbie VanVossen |
| ChangePlan(CR-01).1 | done | 2026-08-26 | Existing sketch resolved and preserved; audited ChangeScope answers recorded in change-plan.md |
| ChangePlan(CR-01).2 | done | 2026-08-26 | Baseline pinned at 7a99f23; no pre-existing workflow/test/verification reports found |
| ChangePlan(CR-01).3 | done | 2026-08-26 | Iteration 2: amended sketch checked; shared firewall_core creates TxFirewall verification impact, not source impact |
| ChangePlan(CR-01).4 | done | 2026-08-26 | Iteration 2: TxFirewall VerifyOnly evidence added; model/contracts/application remain explicit non-impact |
| ChangePlan(CR-01).5 | done | 2026-08-26 | Iteration 2: W2 now gates shared firewall_core changes on clean TxFirewall verification |
| ChangePlan(CR-01).6 | done | 2026-08-26 | Iteration 2: sketch drift and resulting plan revision recorded; approved at AP1 |
| ChangePlan(CR-01).AP1 | done | 2026-08-26 | Approved by Robbie VanVossen on 2026-08-26; ChangeExec unlocked |
| ChangeExec(CR-01) | in-progress | 2026-08-26 | Draft workflow accepted; executing approved plan under audited profile |
| ChangeExec(CR-01).1 | done | 2026-08-26 | Approved plan consistent; drift since 7a99f23 limited to reviewed sketch amendment, plan, and workflow status |
| ChangeExec(CR-01).2 | in-progress | 2026-08-26 | Executing W1 of W4 |
| ChangeExec(CR-01).W1 | in-progress | 2026-08-26 | Starting delta SysPlanAndReq; audited sub-workflow gates apply |
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
| ChangeExec(CR-01).W2 | in-progress | 2026-08-26 | Starting affected-component implementation and verification |
| CompDev(RxFirewall) | in-progress | 2026-08-26 | Generated code synchronized; implementation and test updates pending |
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
| CompDev(RxFirewall).AP2 | pending | 2026-08-26 | Awaiting verification sign-off; remaining external bodies are limited to info, trace, and warn_channel platform adapters |
| CompDev(RxFirewall).cross-crate-diagnostic | done | 2026-08-26 | TxFirewall passes 16/0; LowLevel driver blocked before verification by missing SEL4_INCLUDE_DIRS/SEL4_PREFIX; MAVLinkFirewall awaits its planned developer MAVLink predicate hooks |
