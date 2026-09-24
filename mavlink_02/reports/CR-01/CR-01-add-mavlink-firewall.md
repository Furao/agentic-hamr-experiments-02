# Change Report — CR-01 Add MAVLink Firewall

| Field | Value |
|---|---|
| **Change ID** | CR-01 |
| **Date** | 2026-08-27 |
| **Summary** | Add a verified inbound MAVLink firewall between RxFirewall and the ArduPilot VMM. |
| **Scope** | Requirements, SysML model, GUMBO contracts, generated Microkit interfaces, Rust components and libraries, VMM integration, schedule, tests, verification, and hardware evidence |
| **Verification** | Hardware 12/12; capture 221/221; component tests pass; firewall_core 39/0, mavlink_core 7/0, MAVLinkFirewall 16/0, RxFirewall 27/0, and TxFirewall 16/0 Verus; full ZCU102/custom.mk build passes |
| **Approved plan** | `action-requests/CR-01-add-mavlink-firewall/change-plan.md` |
| **Baseline** | `7a99f23e7a97460a446bc9dcaf7cb5e18972d8e9` |

## 1. Requirements Changes

`requirements/manual_reqs.md` remains byte-for-byte unchanged from the baseline.
CR-01 created the evolving requirements set and companion engineering documents.

| Requirement | Change | Result |
|---|---|---|
| HLR-6–HLR-17 | Superseded the former receive-side TCP/UDP forwarding behavior while retaining stable identifiers. | RxFirewall now rejects TCP, directly routes retained traffic, and separates the MAVLink path. |
| HLR-18–HLR-20 | Added ArduPilot UDP routing, exclusive outputs, and RxFirewall decision logging. | Allocated to RxFirewall and generic `firewall_core`. |
| HLR-21–HLR-27 | Added MAVLink framing, allow, firmware-activation deny, malformed/no-input handling, and reason logging. | Allocated to MAVLinkFirewall and policy-neutral `mavlink_core`. |
| HLR-28–HLR-33 | Added VMM delivery, schedule/domain, carrier-boundary, capacity, and implementation obligations. | Allocated across the model, VMM, schedule, RxFirewall, and MAVLinkFirewall. |
| HLR-21 revision | Clarified after hardware iteration that MAVLink 2 may omit a trailing all-zero payload suffix up to the dialect maximum; MAVLink 1 remains fixed-length. | Corrects the initial implementation's rejection of legal v2 messages. |

The operational trust boundary and CR provenance are recorded in
`requirements/conops.md`; component allocation and traceability are in
`requirements/component-requirements.md`; network, carrier, MAVLink, command, lane,
and timing terms are in `requirements/data-dictionary.md`.

## 2. Model Changes

| Requirement(s) | Model element | File | Validation |
|---|---|---|---|
| HLR-18, HLR-28, HLR-31 | `MAVLinkUDPMessage_Impl` with preserved Ethernet frame and bounded payload metadata | `sysmlv2/open_platform/open_platform_Data_Model.sysml` | HAMR SysML tipe: Well-formed |
| HLR-18–HLR-20 | Four RxFirewall MAVLink outputs and exclusive direct/MAVLink/drop contracts | `sysmlv2/open_platform/open_platform_Software.sysml` | Well-formed; RxFirewall Verus 27/0 |
| HLR-21–HLR-27 | New four-lane `MAVLinkFirewall` thread and component-owned semantic predicates/contracts | `sysmlv2/open_platform/open_platform_Software.sysml` | Well-formed; contract audit has zero AP-1–AP-9 findings |
| HLR-28, HLR-31 | RxFirewall → MAVLinkFirewall → ArduPilot connections and process wrappers | `sysmlv2/open_platform/open_platform.sysml` | Code generation succeeded |
| HLR-29–HLR-30 | Domain 6 and maximum-domain update | `sysmlv2/open_platform/open_platform_Properties.sysml`, `Platform.sysml` | Schedule/build gate passed |
| HLR-21, HLR-31 | Shared carrier getter/specification functions | `sysmlv2/open_platform/GumboLib.sysml` | Integration check exit 0; generated contracts verify |

Model work is represented by commits `2a8f387`, `937cdbf`, `72bb5c3`, `b49fe20`,
and `e2fd822`; the final MAVLink 2 requirements clarification is in `8d016f7`.

## 3. Code Changes

### 3.1 Auto-Generated Files

| Artifact group | Change | Model driver |
|---|---|---|
| `hamr/microkit/types/` | Added the MAVLink carrier C/Rust type and queue. | `MAVLinkUDPMessage_Impl` |
| `hamr/microkit/components/seL4_MAVLinkFirewall_MAVLinkFirewall/` | Added component C glue, monitor, headers, and user entrypoint. | New MAVLinkFirewall process/thread and ports |
| RxFirewall and ArduPilot generated interfaces | Added four MAVLink carrier outputs/inputs. | New ports and system connections |
| Generated Rust bridges and GUMBOX files | Added carrier APIs and rewoven RxFirewall/MAVLinkFirewall contracts. | Component contracts and port types |
| `microkit.system`, `microkit.dot`, `system.mk` | Added component images, queues, channels, and build rules. | System assembly and code generation |

Code generation completed successfully and wrote its report under
`hamr/microkit/reporting/`. Editable developer regions were preserved and reviewed.

### 3.2 Developer-Written Files

| Category | File(s) | Change and trace |
|---|---|---|
| Rx policy | `crates/seL4_RxFirewall_RxFirewall/src/component/*_app.rs` | Four-lane fail-closed Direct/MAVLink/Drop routing and specific reasons (HLR-18–HLR-20). |
| Generic network parsing | `crates/firewall_core/src/lib.rs`, `src/net.rs` | Policy-neutral Ethernet/IPv4/UDP parsing with source/destination ports and verified bounds (HLR-18, HLR-31). |
| MAVLink policy | `crates/seL4_MAVLinkFirewall_MAVLinkFirewall/src/component/*_app.rs` | Four-lane validation, unchanged forwarding, flash-activation deny, and distinct rejection logging (HLR-21–HLR-27). |
| MAVLink parsing | `crates/mavlink_core/` | Policy-neutral v1/v2 framing, signing structure, dialect metadata, CRC, legal v2 zero-suffix truncation, bounded payload getters, and verified policy refinement. |
| VMM | `hamr/microkit/vmm/vmm.c` | Drains four MAVLink carrier inputs and injects their preserved Ethernet frames into the existing virtio receive backend (HLR-28). |
| Schedule/build | `microkit.schedule.xml`, `custom.mk` | Adds domain 6 after RxFirewall and includes MAVLink image/type/link rules (HLR-29–HLR-30). |
| Tests | RxFirewall/MAVLinkFirewall `src/test/tests.rs`; core unit tests | Lane, routing, validity, CRC, signing, truncation, FTP, deny-policy, malformed, notification, and contract partitions. |

## 4. Traceability Matrices

### 4.1 Requirements to Model

| Requirement | Model element(s) | Model file |
|---|---|---|
| HLR-18–HLR-20 | RxFirewall ports and compute guarantees | `open_platform_Software.sysml` |
| HLR-21–HLR-27 | MAVLinkFirewall thread, functions, and compute guarantees | `open_platform_Software.sysml` |
| HLR-28, HLR-31 | Carrier type, process wrappers, and eight receive-path connections | `open_platform_Data_Model.sysml`, `open_platform.sysml` |
| HLR-29–HLR-30 | Domain 6 and platform allocation | `open_platform_Properties.sysml`, `Platform.sysml` |

### 4.2 Model to Code (Auto-Generated)

| Model element | Generated artifact(s) |
|---|---|
| `MAVLinkUDPMessage_Impl` | Rust/C data definitions and `sb_queue_*MAVLinkUDPMessage*` queue |
| MAVLinkFirewall thread/process | Component C glue, Rust bridge/GUMBOX APIs, system image and channels |
| RxFirewall MAVLink outputs | Rx bridge APIs, C queues, GUMBOX oracle, system connections |
| ArduPilot MAVLink inputs | ArduPilot C header/glue and VMM-facing receivers |

### 4.3 Requirements to Code (Developer-Written)

| Requirement | Implementation | File |
|---|---|---|
| HLR-18–HLR-20 | `classify_frame`, carrier construction, per-lane dispatch/logging | RxFirewall `*_app.rs` |
| HLR-21, HLR-25–HLR-27 | Verified parser/classifier and reason mapping | `mavlink_core/src/lib.rs`, `verified.rs`; MAVLinkFirewall `*_app.rs` |
| HLR-22–HLR-24 | Allow route and component-owned firmware activation policy | MAVLinkFirewall `*_app.rs` |
| HLR-28 | Four carrier-queue drains and preserved-frame delivery | `hamr/microkit/vmm/vmm.c` |
| HLR-29–HLR-30 | Domain slot and custom system build | `microkit.schedule.xml`, `custom.mk` |

### 4.4 Requirements to Tests

| Requirement | Evidence | Location |
|---|---|---|
| HLR-18–HLR-20 | Direct/MAVLink/drop partitions, malformed lengths, disallowed ports, all lanes | RxFirewall tests and GUMBOX property tests |
| HLR-21–HLR-27 | v1/v2, signed v2, CRC, IDs, flags, exact datagram framing, legal v2 truncation, FTP, flash deny, lane isolation | MAVLinkFirewall and mavlink_core tests |
| HLR-28–HLR-30 | Full custom.mk ZCU102 build and schedule/image generation | Build result and workflow status |
| HLR-21–HLR-33 | Twelve-case hardware procedure and 221-message follow-up capture | `reports/CR-01/CR-01-add-mavlink-firewall-hardware-test.md`, `manual_test_results/` |

## 5. Change Process Summary

| Step | Action | Tool/method | Artifact/result |
|---|---|---|---|
| W1 | Requirements, model, contracts, integration, generation | SysPlanAndReq, SysModeling, CompGUMBOSpec, SysGUMBOIntegrationCheck, CodeGen | Approved requirements/model wave |
| W2 | Rx/MAVLink implementation and shared-core verification | CompDev, unit/property tests, coverage, Verus | Approved component wave |
| W3 | VMM, schedule, and build integration | Manual VMM integration, SysSchedDef, `SYSTEM_MAKEFILE=custom.mk` | Approved 147.98 MiB ZCU102 image |
| W4 | Host verification and physical validation | test-components, verify, ZCU102 procedure and captures | 12/12 hardware pass; refreshed software evidence |
| Iteration | Correct legal MAVLink 2 trailing-zero truncation and improve reason logging | Capture analysis, parser/spec update, tests, Verus, rebuild | 221/221 captured messages validate |

| Commit | Summary |
|---|---|
| `12ed8a3`–`def9958` | Change plan creation and approval |
| `ea1115e`–`b49fe20` | Requirements, model, contracts, and code generation |
| `835e0d1`–`b502b16` | RxFirewall/MAVLinkFirewall implementation and verification |
| `57a47ca` | VMM and build integration |
| `e77dce1` | Logging refinement |
| `cf93a10`, `8d016f7` | MAVLink 2 zero-byte truncation implementation and requirements correction |

## 6. Impact Confirmation

### Deviations from the Approved Plan

| Plan prediction | Actual | Explanation |
|---|---|---|
| Reject malformed truncation while accepting well-formed MAVLink 2. | Initial minimum-length rule rejected 24 legal zero-suffix-truncated messages; corrected implementation accepts all 221 follow-up messages. | MAVLink 2 transmits a shortened all-zero suffix; the dialect maximum, exact datagram boundary, and CRC are the correct checks. Requirements and verified specs were back-propagated. |
| Integration checks passing and non-vacuous. | Logika exited 0, but the reusable-contract receiver handshake count was zero. | Recorded as a vacuous integration limitation; component proofs, generated connections, full build, and hardware evidence close the practical integration argument. |
| Sketch remains immutable. | Developer amended one truncated guideline after planning began. | Original remains recoverable at baseline; the exact amendment and provenance exception are recorded in the approved plan. |
| LowLevelEthernetDriver and TxFirewall source remain unchanged. | Their semantic model/contracts/application behavior remain frozen; code generation produced mechanical scaffolding/logging/generator drift. | TxFirewall verification remains 16/0 against changed firewall_core; LowLevel builds in the full AArch64 system. No behavioral edit was required. |

### Non-Impact Confirmation

| Artifact/component | Evidence |
|---|---|
| `requirements/manual_reqs.md` | `git diff 7a99f23e -- requirements/manual_reqs.md` is empty. |
| LowLevelEthernetDriver behavior | Receive carrier/API remains compatible; standalone host build requires seL4 configuration as before; full ZCU102 build passes. |
| TxFirewall behavior | Existing transmit policy and path retained; host tests 4/4 and Verus 16/0 pass with firewall_core 39/0. |
| Transmit-side VMM behavior | W3 modified only receive queue consumption; HW-12 confirms guest-originated transmit traffic remains operational. |
| MAVLink XML dialect inputs | Bundled CR-local XML files remain implementation inputs; generated lookup tables derive from them. |
| Existing schedule domains | Existing domain identities and ordering are retained; domain 6 is inserted between RxFirewall and ArduPilot. |

## 7. Verification Summary

| Target | Result |
|---|---|
| firewall_core tests / Verus | 17/17; 39 verified, 0 errors |
| mavlink_core tests / Verus | 4/4; 7 verified, 0 errors |
| RxFirewall tests / Verus | 10/10; 27 verified, 0 errors |
| MAVLinkFirewall tests / Verus | 7/7; 16 verified, 0 errors |
| TxFirewall tests / Verus | 4/4; 16 verified, 0 errors |
| LowLevel standalone host target | Known configuration limitation: requires `SEL4_INCLUDE_DIRS` or `SEL4_PREFIX` |
| Full ZCU102/custom.mk build | Pass; loader 147.98 MiB; SHA-256 `f0189ae8ea42ef2e46b1257510ccf2544c2c420cbe2e0c6cce7624be978156e9` |
| Manual ZCU102 validation | 12/12 passed, operator Robbie VanVossen |

