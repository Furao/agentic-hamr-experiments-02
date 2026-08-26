# Change Plan – CR-01 Add MAVLink Firewall

| Field | Value |
|---|---|
| **Change ID** | CR-01 |
| **Title** | Add inbound MAVLink firewall |
| **Status** | Approved |
| **Approved by / date** | Robbie VanVossen / 2026-08-26 |
| **Target project** | `open_platform` (`mavlink_02`) |
| **Baseline** | `7a99f23e7a97460a446bc9dcaf7cb5e18972d8e9` (surveyed 2026-08-26) |
| **Sketch (provenance)** | `mavlink-firewall-change-sketch.md` (developer-amended 2026-08-26; original at baseline commit); bundled MAVLink references under `mavlink_spec/` |
| **Profile** | audited |

## 1. Change Summary

Add a new Rust MAVLink firewall between the receive-side network firewall and the
ArduPilot VMM. The RxFirewall will reject TCP, continue sending existing permitted
non-ArduPilot traffic directly to the VMM, and route UDP traffic from source port
14550 to destination port 14562 through the new component. The MAVLink firewall will
accept exactly one well-formed MAVLink v1 or v2 frame per UDP payload, reject malformed
frames and firmware-update traffic, log every rejection reason, and forward allowed
frames to the VMM. The VMM receive path and static schedule must accommodate the new
component; LowLevelEthernetDriver and TxFirewall are frozen.

## 2. Sketch Provenance and Clarifications

- Sketch: `mavlink-firewall-change-sketch.md` — archived verbatim and not edited
  (change-development.md §2).
- Bundled protocol sources: the saved MAVLink packet-serialization guide and the
  ArduPilotMega, common, standard, and minimal dialect XML files under `mavlink_spec/`.
- Developer clarifications, 2026-08-26:
  1. The target is this `open_platform` project and the current commit is the intended
     baseline.
  2. Use separate waves with separate review/test/verify cycles.
  3. The developer approves the plan; no additional external approver was named.
  4. Add manual hardware testing to the standard CompDev test bar.
  5. Verify affected components and finish with the cross-crate verification pass.
  6. Preserve `requirements/manual_reqs.md`; create `requirements/updated_reqs.md`
     and update/create ConOps, component requirements, and data dictionary artifacts.
  7. Do not modify LowLevelEthernetDriver or TxFirewall components.
  8. Use the standard change-report sections.
- Developer review decision, 2026-08-26: allow `FILE_TRANSFER_PROTOCOL` messages;
  deny firmware-flash activation commands rather than file transfer itself.
- Developer sketch amendment, 2026-08-26: completed the previously truncated
  `firewall_core` guideline to require that changes to the shared dependency do not
  affect TxFirewall verification.
- Provenance note: the sketch was amended after ChangePlan began, contrary to the
  immutable-sketch rule in change-development.md §2. The pre-amendment text remains
  recoverable at baseline commit `7a99f23e`; this plan records the one-line amendment
  explicitly instead of concealing the drift.

## 3. Baseline (§3)

- Commit: `7a99f23e7a97460a446bc9dcaf7cb5e18972d8e9`, committed
  2026-08-26T14:22:31-04:00 (`mavlink_02: Add mavlink_firewall change sketch`).
- Workflow status before ChangePlan contained no completed or active development rows.
  No baseline test, coverage, integration-check, tipe, or Verus reports are present in
  `reports/`; the plan therefore makes no clean-baseline claim beyond source presence.
- SysModel has four thread components: LowLevelEthernetDriver, RxFirewall, TxFirewall,
  and ArduPilot. Four event-data ports model each logical queued path. Receive traffic
  currently flows LowLevelEthernetDriver → RxFirewall → ArduPilot.
- RxFirewall contracts unconditionally forward valid ARP, whitelisted TCP, and
  whitelisted UDP on each lane and reject other inputs. The requirements currently
  whitelist TCP destination port 5760 and UDP destination port 68. Rx code relies on
  the local `firewall_core` parser/policy library.
- No MAVLink component, MAVLink contracts, MAVLink crate, or MAVLink schedule domain
  exists. ArduPilot has only the four RxFirewall receive ports. Static schedule domains
  are ArduPilot 2, TxFirewall 3, LowLevelEthernetDriver 4, and RxFirewall 5.
- Generated Microkit code exists for the four baseline components, but no codegen
  report is present. The VMM is manually maintained under `hamr/microkit/vmm/`.
- The working tree was clean at the pinned commit. Planning then changed
  `reports/workflow-status.md` and created this plan; the developer subsequently amended
  one sketch guideline as recorded in §2.

## 4. Consistency Findings (§3)

| # | Finding | Class | Resolution |
|---|---|---|---|
| F1 | The sketch removes TCP forwarding, while baseline requirement HLR-6 and all four RxFirewall compute guarantees require valid TCP forwarding. | impact | Retire/supersede the behavior in `updated_reqs.md`; revise RxFirewall policy, contracts, implementation, and tests in W1–W2. Preserve `manual_reqs.md`. |
| F2 | Baseline RxFirewall has only one four-lane output path directly to ArduPilot; conditional routing to MAVLink inspection needs a second four-lane output path and corresponding system connections. | impact | Add ports/contracts/connections and regenerate in W1; implement routing in W2. |
| F3 | ArduPilot/VMM has only the direct RxFirewall receive path. | impact | Add a four-lane input path from MAVLinkFirewall in the model and manually integrate the corresponding VMM receive path in W3. |
| F4 | “Reject messages that can update firmware” requires separating permitted file transfer from commands that activate firmware flashing. The bundled dialect identifies message 110 `FILE_TRANSFER_PROTOCOL`, command 42650 `MAV_CMD_FLASH_BOOTLOADER`, and secure-command flash-bootloader operation 7. | impact | Resolved (RD-1): allow `FILE_TRANSFER_PROTOCOL`; reject command envelopes carrying `MAV_CMD_FLASH_BOOTLOADER` and secure-command flash-bootloader operation 7. |
| F5 | The sketch states each qualifying UDP payload contains one serialized message. MAVLink v1/v2 framing still requires explicit length, checksum/CRC-extra, v2 incompatibility/signature, truncation, and trailing-byte rules. | impact | Encode exact parsing validity in requirements, contracts, parser tests, and malformed-input tests in W1–W2. |
| F6 | Adding a component requires a unique domain and a hand-edited static schedule slot; code generation does not refresh `microkit.schedule.xml`. | impact | Allocate the model domain during W1 and update/test the schedule in W3. |
| F7 | LowLevelEthernetDriver and TxFirewall lie outside the changed receive route, but TxFirewall depends on the shared `firewall_core` that W2 must update. | impact / non-impact obligation | Freeze both components' model, contracts, and application code. Re-verify TxFirewall after `firewall_core` changes and require unchanged verification success; confirm LowLevelEthernetDriver by diff and final cross-crate evidence. |
| F8 | The amended sketch explicitly requires `firewall_core` changes not to affect TxFirewall verification. | impact | Add `CompDev(component=TxFirewall)/VerifyOnly` and shared-library regression checks to W2 without authorizing TxFirewall implementation changes. |

No finding prevents production of a reviewable plan. RD-1 is resolved; final plan
approval remains required before ChangeExec may begin.

## 5. Impact Analysis (§4)

### 5.1 Impacted

| Artifact / element | Impact | Addressed in |
|---|---|---|
| `requirements/updated_reqs.md` | Create the evolving system requirements, retaining the existing requirement naming scheme; replace TCP allowance, specify MAVLink routing/parsing/deny/logging behavior, and trace additions to CR-01. | W1 |
| ConOps, component requirements, data dictionary | Create/update the listed requirements-level artifacts and document the new inspection boundary, component responsibilities, port/data semantics, and CR provenance. | W1 |
| SysModel: RxFirewall | Add four MAVLink-route outputs and conditional direct-versus-MAVLink routing contracts; remove TCP allowance. | W1 |
| SysModel: MAVLinkFirewall | Add thread/process, four inputs, four outputs, GUMBO contracts, unique domain, and system assembly connections. | W1 |
| SysModel: ArduPilot | Add four inbound ports fed by MAVLinkFirewall while retaining the direct permitted-traffic path. | W1 |
| Integration contracts | Prove Rx routing, MAVLink allow/drop behavior, and VMM delivery connections are compatible and non-vacuous. | W1 |
| Generated code | Regenerate bridge APIs, new component crate/C glue, Microkit system description, and affected proof/test scaffolds; review rewoven editable regions. | W1 |
| `firewall_core` | Extend UDP parsing/policy support to check both source 14550 and destination 14562 and expose routing classification without adding MAVLink awareness; preserve every proof obligation consumed by TxFirewall. | W2 |
| New MAVLink parsing/policy core | Implement bounded MAVLink v1/v2 parsing, dialect metadata/CRC-extra handling, malformed-frame rejection, explicit allowance of `FILE_TRANSFER_PROTOCOL`, and denial of firmware-flash activation commands independently of Ethernet/IP parsing. | W2 |
| RxFirewall implementation/tests | Remove TCP forwarding, route qualifying ArduPilot UDP frames to MAVLinkFirewall, preserve allowed direct traffic, and log every drop/route reason across four lanes. | W2 |
| MAVLinkFirewall implementation/tests | Parse exactly one frame, apply the blacklist, forward allowed packets unchanged, reject malformed/denied packets, and log reasons across four lanes. | W2 |
| TxFirewall verification evidence | Re-run verification against the modified shared `firewall_core`; no TxFirewall model, contract, or implementation delta is permitted. | W2 |
| ArduPilot/VMM integration | Consume the four new MAVLinkFirewall inputs in the manually maintained VMM path without disturbing transmit behavior. | W3 |
| Static schedule | Add a MAVLinkFirewall domain slot while retaining all existing domain ordering/allocations unless timing analysis requires a reviewed adjustment. | W3 |
| Hardware evidence | Exercise allowed, malformed, firmware-update, non-ArduPilot UDP, TCP, and four-lane/burst cases on ZCU102 with ArduPilot over UDP. | W4 |
| Change report | Standard report with predicted-versus-actual impact and explicit non-impact evidence. | W4 / ChangeExec.5 |

### 5.2 Non-Impact Argument

| Untouched artifact / component | Why unaffected |
|---|---|
| `requirements/manual_reqs.md` | Developer explicitly froze it. It remains provenance; requirements evolution is captured in `updated_reqs.md` and traceable companion documents. |
| LowLevelEthernetDriver model/contracts/code | Its four receive outputs still carry the same `RawEthernetMessage` type to RxFirewall, and its transmit inputs are outside the changed receive-side route. Confirm no file delta and rerun unchanged-component checks only if cross-crate effects demand it. |
| TxFirewall model/contracts/code | The ArduPilot → TxFirewall → driver transmit path is disjoint from the inserted receive-side component. Its shared `firewall_core` dependency means verification evidence is impacted even though TxFirewall files are frozen; confirm no file delta, re-verify, and preserve its schedule slot. |
| Ethernet/data types | Existing `RawEthernetMessage` can carry the bounded Ethernet/IPv4/UDP/MAVLink frame and no sketch requirement changes `SizedEthernetMessage_Impl`; avoid a type change unless W1 modeling proves it necessary, in which case revise this plan. |
| Transmit-side VMM behavior | Only VMM receive inputs are extended; its existing transmit outputs and TxFirewall handoff are unchanged. |
| MAVLink dialect source files and sketch | They are immutable planning/implementation inputs under the CR folder, not generated artifacts. |
| Existing schedule slots | New capacity is needed for MAVLinkFirewall, but the responsibilities and relative route of existing domains remain unchanged; any timing-driven reorder requires plan revision. |

## 6. Waves (§5)

| Wave | Intent | Invocations | Expected artifact deltas | Verification gate |
|---|---|---|---|---|
| W1 | Establish requirements, architecture, contracts, and generated scaffold. | Delta `SysPlanAndReq`; delta `SysModeling`; `CompGUMBOSpec(component=RxFirewall)`; `CompGUMBOSpec(component=MAVLinkFirewall)`; `SysGUMBOIntegrationCheck`; `CodeGen`. | New requirements documents; RxFirewall/MAVLinkFirewall/ArduPilot model and connection deltas; generated MAVLinkFirewall and affected bridge scaffolds. | Requirements review complete; SysML tipe clean; integration checks passing and non-vacuous; generated tree reviewed with no LowLevelEthernetDriver/TxFirewall semantic delta. |
| W2 | Implement and verify receive routing and MAVLink policy without regressing TxFirewall proofs. | `CompDev(component=RxFirewall)`; `CompDev(component=MAVLinkFirewall)`; direct development/testing of `firewall_core` and a separate MAVLink core as dependencies; `CompDev(component=TxFirewall)/VerifyOnly`. | RxFirewall, MAVLinkFirewall, core libraries, unit/property/GUMBOX tests, affected-component coverage/Verus evidence, and refreshed TxFirewall verification evidence with no TxFirewall source delta. | RD-1 policy encoded; valid `FILE_TRANSFER_PROTOCOL` messages pass; firmware-flash activation commands and malformed v1/v2 frames are rejected; full branch coverage of changed application logic and GUMBOX oracles; RxFirewall/MAVLinkFirewall Verus clean; TxFirewall verification remains clean against the changed `firewall_core`. |
| W3 | Integrate the VMM and static schedule. | VMM manual integration per project convention; `SysSchedDef`; affected integration/build checks. | VMM receive-path changes; `microkit.schedule.xml` new domain slot; any necessary build glue. | Full system builds; schedule validation passes; four MAVLinkFirewall outputs reach the corresponding VMM inputs; existing transmit path remains operational. |
| W4 | Validate on hardware and close system evidence. | Developer/manual ZCU102 test procedure; `/test-components`; `/verify`; ChangeExec final pass/report steps. | Hardware test record and standard `reports/CR-01-add-mavlink-firewall.md`. | On ZCU102: allowed MAVLink, including valid `FILE_TRANSFER_PROTOCOL`, reaches ArduPilot; malformed frames, firmware-flash activation commands, and TCP traffic are dropped with reasons; non-ArduPilot allowed traffic still reaches VMM; all lanes exercised; cross-crate tests and verification pass. |

Ordering keeps each boundary checkable: W1 makes the model/contracts coherent before
code, W2 closes verified component behavior before platform integration, W3 closes the
deployable system, and W4 supplies the separately requested hardware and final evidence.
Each wave pauses at ChangeExec.AP1 under the audited profile.

## 7. Back-Propagation Plan (§7)

| Document | Update | When |
|---|---|---|
| `requirements/manual_reqs.md` | No changes; cite as preserved baseline provenance. | n/a; verify unchanged in W4 |
| `requirements/updated_reqs.md` | Create from the applicable baseline requirement set, retaining IDs/naming; revise TCP/UDP routing requirements and add stable CR-01-traced MAVLink validity, firmware-deny, logging, and no-output requirements. | W1 |
| `requirements/conops.md` | Create/update operational receive-flow and trust-boundary descriptions with a dated CR-01 revision annotation; do not rewrite any provenance block if one is introduced from baseline text. | W1 |
| `requirements/component-requirements.md` | Define and trace RxFirewall, MAVLinkFirewall, and ArduPilot/VMM receive responsibilities; explicitly state LowLevelEthernetDriver and TxFirewall non-impact. | W1 |
| `requirements/data-dictionary.md` | Define relevant Ethernet/IPv4/UDP/MAVLink frame, port, length, version, message-ID, command-ID, CRC, signature, and blacklist terms without duplicating dialect XML. | W1 |

ChangeExec.3 performs a final sweep to ensure these documents match implemented
behavior and records deviations without modifying the archived sketch.

## 8. Risks and Open Questions

1. Firmware images may be staged through generic MAVLink file transfer, which the
   developer requires the firewall to allow. Mitigation: deny the activation paths
   identified in the bundled dialect—command envelopes carrying
   `MAV_CMD_FLASH_BOOTLOADER` and secure flash-bootloader operation 7—and test both
   allowed file transfer and rejected flash activation. Any newly identified activation
   command requires a policy update rather than blocking file transfer broadly.
2. MAVLink v2 signing creates a choice between syntactic validation and authentication.
   This change promises malformed-frame rejection, not key-based authenticity; signed
   frames must be structurally validated, while cryptographic verification is out of
   scope unless the reviewer expands the request.
3. The four-lane queue workaround can reorder independent arrivals across dispatches.
   Keep lane correspondence end-to-end and test simultaneous/burst traffic on hardware.
4. The VMM is C and manually maintained outside HAMR, so generated interfaces cannot
   by themselves prove receive-path correctness. Isolate the patch, inspect port mapping,
   and make W3 build plus W4 hardware evidence mandatory.
5. Adding a scheduled domain can exceed timing budget or alter latency. Preserve current
   slots, add an explicit slot, and require reviewed timing/schedule evidence in W3.
6. No baseline reports establish a green test/verification state. Capture pre-change
   failures before attributing regressions; do not weaken W2/W4 gates.
7. `firewall_core` is shared by receive and transmit policy code, so an apparently
   receive-only change can invalidate TxFirewall proofs without changing TxFirewall
   source. Keep additions proof-compatible, run TxFirewall VerifyOnly in W2, and treat
   any required TxFirewall source/contract edit as plan drift requiring review.

## 9. Acceptance Criteria and Report Obligations (§8)

- `requirements/manual_reqs.md` is byte-for-byte unchanged; the four requested evolving
  requirements artifacts exist and trace changed/new requirements to CR-01.
- RxFirewall rejects TCP, routes only UDP source 14550/destination 14562 to
  MAVLinkFirewall, forwards other still-permitted traffic directly to the VMM, and
  logs decisions without MAVLink parsing knowledge.
- MAVLinkFirewall accepts exactly one structurally well-formed MAVLink v1/v2 message,
  allows valid `FILE_TRANSFER_PROTOCOL` messages, rejects command envelopes carrying
  `MAV_CMD_FLASH_BOOTLOADER` or secure flash-bootloader operation 7, forwards other
  allowed packets unchanged, and logs every drop reason without Ethernet/IP parsing
  knowledge.
- Model, GUMBO contracts, integration checks, generated interfaces, implementation,
  tests, schedule, and VMM port mapping agree for all four lanes.
- LowLevelEthernetDriver and TxFirewall model/contracts/implementation remain unchanged;
  TxFirewall still verifies against the changed `firewall_core`, and transmit behavior
  plus existing schedule slots retain regression evidence.
- Affected component tests reach full branch coverage for application logic and GUMBOX
  oracles; affected crates verify; the final cross-crate test and `/verify` passes are
  recorded.
- Manual ZCU102 testing demonstrates allowed, malformed, denied firmware-update, TCP,
  non-ArduPilot UDP, and multi-lane cases against ArduPilot over UDP.
- Standard change report `reports/CR-01-add-mavlink-firewall.md` confirms or corrects
  this impact analysis and includes deviations plus non-impact evidence.

## 10. Review Record

| ID | Item (ref) | Decision | Rationale | Decided by | Date |
|----|---|---|---|---|---|
| RD-1 | Firmware-update deny policy (§4 F4, §6 W2, §8 risk 1) | Allow `FILE_TRANSFER_PROTOCOL`; reject command envelopes carrying `MAV_CMD_FLASH_BOOTLOADER` or secure flash-bootloader operation 7. | Preserve MAVLink file transfer while blocking the dialect's identified firmware-flash activation commands. | Developer | 2026-08-26 |

Approval rule: the header Status may change to **Approved** only when all review rows
are resolved. RD-1 is resolved, its plan-body revision is complete, and Robbie
VanVossen approved the plan on 2026-08-26.
