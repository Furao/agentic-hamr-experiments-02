# CR-02 — Add ModeManager

| Field | Value |
|---|---|
| Change ID | CR-02 |
| Review date | 2026-09-23 |
| Status | Complete — ChangeExec.AP2 approved by developer on 2026-09-23 |
| Completion approval | Developer (user), explicit “approved” response to final CR-02 review |
| Summary | Add latched Normal/Recovery control, rejection counting and generated R2U2 monitoring; enforce strict UDP routing and bounded Ethernet transmission |
| Scope | Developer-supplied requirements, SysML model/contracts, generated interfaces/monitor, applications/tests, toolchain, schedule and custom target build |
| Baseline | `043d574970d28ff172f7261ea0392ddd14ae50a8` |
| Implementation HEAD | `9cf01d9b9cebb31c94265c082a2ea257ca8dbe52`; current requirements, closeout documents and build helpers also included in working-tree delivery |
| Requirements authority | `action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md` |
| Plan / provenance | Approved `action-requests/CR-02-add-mode-manager/change-plan.md` and immutable `add-mode-manager-sketch.md` |
| Verification | 67 application/core tests plus 2 driver helper tests pass; six proof-enabled crates pass; full ZCU102/debug build passes |
| Accepted exception | High/Open CR-02-HW-01 deferred by developer to a future CR; non-blocking for this change |

All three waves are explicitly approved. Requirements reconciliation is complete:
the developer supplied the revised logging requirement and authorized retirement of
four legacy CR-01 documents. The timing issue remains tracked independently; this
report does not assert that nominal hardware D2 timing has been demonstrated.

## 1. Requirements changes

The developer retained ownership of requirements throughout CR-02. Revision _07
established the consolidated HLRs and LLR-1–21. Revision _26_09_23_01 supplied the
1586-byte IPv4 limit; _26_09_23_02 changed only LLR-18 to reconcile logging with
the approved implementation. The supplied _02 document is preserved unchanged.

| Requirement / subject | Previous behavior or identity | Current requirement / disposition |
|---|---|---|
| HLR-13 direct UDP | CR-01 allowed direct traffic unless both GCS/ArduPilot ports matched | Normal only; source must not be 14550, destination must not be 14562, destination must be whitelisted (68) |
| HLR-18 MAVLink route | Bounded GCS-to-ArduPilot carrier route | Normal-only source 14550/destination 14562; carrier preservation retained |
| HLR-23/24/26/27 | No ModeManager | Initialize Normal; publish mode each dispatch; true error causes same-dispatch Recovery publication, latched until reboot |
| HLR-28/29; LLR-10 | No mode-controlled forwarding | Suppress all receiving-firewall Ethernet outputs when that dispatch's frozen mode is Recovery |
| HLR-25/31; LLR-12/13 | No cumulative mode-triggering rejection counter | Count malformed/blacklisted messages once each, across four lanes, saturate at 20; publish final count >=5 every dispatch |
| HLR-19/32; LLR-7/8 | Existing command/secure flash blacklist | Retain command 42650 and SECURE_COMMAND operation 7; correctly zero-extend legal truncated v2 payload fields; COMMAND_INT offsets require no change |
| HLR-30; LLR-15/16 | No first-error deadline monitor | Generated R2U2 observes frozen mode and final error; first assertion D0, timely Recovery D1/D2, one timeout during D2 per boot |
| HLR-12/13/18; LLR-4 | IPv4 maximum 9000 | Maximum 1586 within 1600-byte carrier; Tx output bounded to 1600; independent driver bounds defense authorized |
| LLR-18 | Required Recovery-suppression diagnostics | No mode-only per-message suppression logs; one ModeManager transition log; retain rejection/routing diagnostics and independent timeout error |
| LLR-19/20/21 | CR-01 integration/assurance requirements | Preserve VMM/carriers; add manager/control/monitor integration and evidence obligations; hardware timing correction deferred by developer |

Historical identity migration is explicit: CR-01 HLR-19/20 map to current LLR-2/18;
HLR-21 to current HLR-20/22 and LLR-3/6/7; HLR-23 to LLR-8; HLR-24 to HLR-19/32;
HLR-25 to HLR-20; HLR-26 to HLR-21; HLR-27 to LLR-18; HLR-28 to LLR-19;
HLR-29 to LLR-20; HLR-30 to LLR-1; HLR-31 to LLR-5; HLR-32 to the approved
regression obligation/LLR-21; HLR-33 to LLR-17/21. Historical IDs retain their old
meaning; they must not be read as current IDs.

`requirements/conops.md`, `updated_reqs.md`, `component-requirements.md` and
`data-dictionary.md` are retired with current-source/provenance links. Their original
bodies, including ConOps §0, are preserved. This is the developer's explicit
replacement for the plan's legacy-publication update obligations. See
[requirements reconciliation](CR-02-closeout-requirements-review.md) and
[complete requirement allocation](../action-requests/CR-02-add-mode-manager/w1-requirements-planning-07.md).

## 2. Model changes

Paths in this table are under `sysmlv2/open_platform/`.

| File / model element | Change and requirements | Implementation commits |
|---|---|---|
| `open_platform_Data_Model.sysml`: OperatingMode | Add Normal/Recovery enum; existing Ethernet carrier shapes unchanged (HLR-24/26, LLR-10/11) | 2421087 |
| `open_platform.sysml`: ModeManager_seL4, seL4 | Add manager process/thread and three sampled control connections; retain four-lane data connections (HLR-25/26/27) | 2421087 |
| `open_platform_Properties.sysml`, `Platform.sysml` | Manager domain 7, Max_Domain 8; frame 2080 ms, clock 2 ms; deployment configuration (LLR-20) | 2421087, b68c70a |
| `open_platform_Software.sysml`: RxFirewall | Frozen-mode guards, initialization no-send and exhaustive lane routing (HLR-5/13/15/17/18/29) | 86c4a5b |
| Same: MAVLinkFirewall | Counter/status contracts, mode guards and first-threshold future-time R2U2 formula (HLR-19–22/25/28/30/31/32) | 8ee4d70, d6fbae6 |
| Same: ModeManager | Initial Normal, exact latched state transition and both output equalities (HLR-23/24/26/27) | af1edc8 |
| Same: TxFirewall; `GumboLib.sysml` | Shared 1586-byte IPv4 bound; four Tx size integration guarantees; direct/MAVLink UDP share rx_bounded_udp (HLR-12/13/18, LLR-4) | 86c4a5b, 09ae41d |

Model type checks passed after contract changes and again with the updated HAMR:
`CR-02-hamr-upgrade-tipe.txt` reports `Well-formed!`, exit 0. No model change
followed that check. Contract audits and all wave gates were approved. The GUMBO
integration check returned exit 0 with **zero receiver assumptions (N=0)**; this
is a vacuous integration pass, not an end-to-end mode-propagation proof.

The R2U2 trigger uses pre-count below five and final true status. Its future-time
interval accepts Recovery at D1 or D2; repeated true status does not retrigger it.
Generated pre/compute/post ordering and startup defaults were inspected, and host
runtime tests check actual verdict/report delivery. Deployed nominal timing remains
the separately deferred issue.

## 3. Code changes

### 3.1 Generated artifacts

HAMR regenerated Rust data/APIs/GUMBOX/tests, C bridges/queues, monitor hooks and
system configuration. The current generator includes the former false-verdict fix:
fresh output matches the patched baseline byte-for-byte across 1,320 compared
files, including all 52 reported editable resources. The regression probe passes
2/2 without applying a patch. Automatic post-codegen patching is retired.

| Model element | Generated artifacts under `hamr/microkit/` | Effect |
|---|---|---|
| OperatingMode and sampled controls | `crates/data/src/open_platform_Data_Model/OperatingMode.rs`, `types/{include,src}/sb_queue_*OperatingMode_1.*`, `sb_queue_bool_1.*` | Normal=0 default, Recovery=1, bool/mode queues |
| ModeManager | `crates/seL4_ModeManager_ModeManager/`, `components/seL4_ModeManager_ModeManager/` | APIs, scaffold, contracts, initialization/output wrappers, C bridge and monitor |
| Rx/MAVLink mode/status contracts | Affected `src/bridge/*_api.rs`, `*_GUMBOX.rs`, `extern_c_api.rs`, test utilities and C component bridges | Sampled mode reads, error writes/peek, oracle checks and woven postconditions |
| MAVLink temporal guarantee | `crates/seL4_MAVLinkFirewall_MAVLinkFirewall/src/component/{r2u2_monitor.rs,spec.c2po,spec.map,spec.bin}`, `src/lib.rs` | Four signals, one step per dispatch, false verdict retained for reporting |
| Shared bounds / Tx guarantees | `crates/GumboLib/src/lib.rs`, Tx API/GUMBOX and initialization test helper | 1586 limit and <=1600 output obligations |
| Deployment | `microkit.system`, generated build files, C monitors and pacer | Manager images/domain, queues and generated scaffolding |

The [complete artifact inventory](CR-02-final-artifact-inventory.md) lists every
tracked model/code delta and its role, including mechanical generator/toolchain
changes. No orphan components were removed. Monitor, compiler/runtime and logger
behavior remains a tested runtime boundary outside application Verus proofs.

### 3.2 Developer implementations and tests

For each component below, the crate prefix is `hamr/microkit/crates/seL4_<name>_<name>/`;
`app` means `src/component/seL4_<name>_<name>_app.rs`, and tests are `src/test/tests.rs`.

| Component / path | Implementation and test changes |
|---|---|
| ModeManager app:28,47 | Initialize/publish Normal; latch Recovery on true error; publish both outputs every dispatch; log only the Normal-to-Recovery transition. Tests cover all transitions, reboot, repeated dispatches and oracle rejection. |
| RxFirewall app:33,136,165 | Strict network classification; frozen-mode gate on every lane; preserve valid raw/carrier bytes; no mode-only suppression log. Tests cover UDP exclusions, bounds, all modes/lanes and oracle partitions. |
| MAVLinkFirewall app:58–199,228,257; `src/logging.rs` | Component-owned command/operation constants and verified policy helpers, bounded classification, saturating count, final status, Recovery suppression; synchronous one-time timeout adapter. Tests cover independent protocol fixtures, counter boundaries and actual generated monitor/production logger. |
| TxFirewall app:131,167 | Existing executable routing retained; current generator rewove final-state contract syntax; shared core now enforces amended maximum. Tests add per-lane size/bounds and oracle regressions. |
| LowLevelEthernetDriver app:91 and app subdirectory `tx_bounds.rs` | Validate a nonempty in-buffer slice before acquiring Tx token; skip invalid request and continue later lanes; never truncate. Two helper tests cover all u16 sizes and mixed requests. |
| `hamr/microkit/crates/firewall_core/src/net.rs`, `src/lib.rs` | MAX_MTU 9000→1586; protocol/length boundary test. |
| `hamr/microkit/custom.mk`, `microkit.schedule.xml` | Manager application/monitor/control objects, retained VMM linkage, pinned installed R2U2 compiler, manager-first application schedule in legacy format. |
| Component/core manifests, toolchains, Makefiles | Developer-authorized migration to Verus 0.2026.08.09.92f466f / Rust 1.97.1; consistent solver settings. |
| `bin/compile-r2u2.py`, `hamr/microkit/bin/build.cmd` | Reproducible specification compilation with installed CLI 4.2.4 and orchestration helper. These delivered working-tree files are currently untracked; hashes are recorded in final-nonimpact.json. |

Logging adapters retain the approved external-body boundary (two in ModeManager,
three in each firewall); no new assume/admit escape was introduced for closeout.
Rx/MAVLink startup output-emptiness preconditions rely on inspected HAMR platform
initialization, not a proof across the extern-C boundary. Driver verification stays
disabled in its existing manifest.

## 4. Traceability matrices

The complete HLR/LLR allocation is in the W1 requirements planning report; the
following matrices connect the final implementation to its test evidence. File
abbreviations are defined in §§2–3.

### 4.1 Requirements to model

| Requirements | Model elements / files |
|---|---|
| HLR-5/13/15/17/18/29; LLR-1–5/10/11 | RxFirewall contracts in Software; network/carrier predicates in GumboLib; retained carrier definitions and lane connections |
| HLR-7/12/14/16; LLR-4/21 | TxFirewall output guarantees; GumboLib IPv4 length |
| HLR-19/20/21/22/25/28/31/32; LLR-6–13/17 | MAVLinkFirewall validity, blacklist, count, status and frozen-mode contracts |
| HLR-23/24/26/27; LLR-11/14 | OperatingMode enum, ModeManager contracts and sampled control connections |
| HLR-30; LLR-15/16 | MAVLinkFirewall `hlr_30_llr_15_16_recovery_deadline` temporal guarantee |
| LLR-18 | Component diagnostic implementation/tests; no new GUMBO logging theorem |
| LLR-19/20/21 | Preserved carrier/lane model, manager properties, platform bindings and separate deployment/assurance evidence |

### 4.2 Model to generated code

The per-element generated mapping is given in §3.1 and expanded by file in the
artifact inventory. Source/generated paths, requirement identity and guarantee
names remain linked in the component audit and generation reports. R2U2 compiler
output is tested separately from executable GUMBOX oracles.

### 4.3 Requirements to developer code

| Requirements | Implementation location |
|---|---|
| HLR-23/24/26/27; LLR-14/18 | ModeManager initialize/timeTriggered: app lines 28/47; transition diagnostic line 72 |
| HLR-13/18/29; LLR-2/4/10 | Rx classifier line 33 and timeTriggered line 165; shared core MAX_MTU |
| HLR-19/32; LLR-7/8 | MAVLink command/secure-operation readers lines 58/67/74 and classifier line 142 |
| HLR-20/21/22/28/31/25 | MAVLink carrier/classifier lines 168/199 and compute line 257; final count/status lines 429/433 |
| HLR-30; LLR-15–18 | Generated monitor hooks plus MAVLink `src/logging.rs` timeout reporter/reset |
| HLR-12; LLR-4; authorized driver defense | Tx compute line 167, core net.rs maximum, driver compute line 91 and app/tx_bounds.rs |
| LLR-19/20 | Existing VMM virtio integration, custom.mk, legacy schedule |

### 4.4 Requirements to tests

| Requirements / behavior | Named tests | File |
|---|---|---|
| Manager startup/latch/publication/logging | `initialization_and_reboot_reset`, `all_transitions_publish_both_outputs_each_dispatch`, `recovery_stays_latched_and_notifications_preserve_state`, `exhaustive_contract_oracle_accepts_only_required_results` | ModeManager tests |
| Strict UDP, frozen Recovery, lane isolation | `strict_udp_bounds_modes_and_lane_matrix`, `routes_each_lane_without_cross_lane_output`, `every_lane_and_contract_partition_passes_oracle`, `oracle_rejects_injection_duplication_and_modified_carriers` | Rx tests |
| Count/status/saturation and firmware rules | `count_modes_threshold_saturation_and_empty_dispatches`, `secure_operation_zero_extension_and_upper_bytes`, `routes_allowed_and_drops_flash_and_malformed_per_lane` | MAVLink tests |
| D2 reporting and noninterference | `production_monitor_reports_at_d2_once_and_resets_on_boot`, `timeout_does_not_change_routing_or_count` | MAVLink tests |
| Bounds/carrier consistency | `carrier_boundaries_and_destination_octets`, `ipv4_carrier_length_boundary_matches_contract` | MAVLink tests and app-local tests |
| Tx size/preservation | `all_lanes_preserve_frames_and_sizes`, `output_size_invariants_reject_oversized_values_on_every_lane`, `oracle_rejects_injection_wrong_size_and_changed_bytes` | Tx tests |
| Driver bounds | `every_u16_size_is_bounded_and_preserves_bytes`, `invalid_request_does_not_prevent_later_valid_lanes` | Driver app/tx_bounds.rs |
| Monitor generated/runtime interface | `generated_monitor_and_reporting_traces`, `raw_runtime_verdicts` | `tests/r2u2_monitor_probe/src/lib.rs` |
| Contract oracle checks | Manual GUMBOX positive/negative partitions and existing generated randomized tests, included in suite totals | Component `src/test/tests.rs` and generated `src/test/util/` |

## 5. Process and final validation

| Step | Action / method | Evidence |
|---|---|---|
| W1 — approved | Review user requirements; SysModeling, component contracts/audits, Logika integration and HAMR generation | CR-02-w1-gate.md; contract audit reports; CR-02-integration-check.md |
| W2 — approved | Component implementation, independent tests/GUMBOX, coverage and Verus; production monitor reporting | CR-02-w2-gate.md; component development reports |
| W3 — approved | Custom build/schedule; approved bounds/driver correction and transition logging; hardware acceptance | CR-02-bounds-*.md, CR-02-mode-log.md, CR-02-hardware-acceptance.md |
| HAMR upgrade | Compare raw output to patched baseline; probe 2/2; retire patch workflow | CR-02-hamr-upgrade.md |
| Requirements closeout | User supplies _02; explicitly retires four legacy docs | CR-02-closeout-requirements-review.md |
| Final verification/build | Six proof-enabled crates pass; full target loader passes | CR-02-full-verify-build.md and build manifest |
| Final test sweep | 67 application/core tests and 2 isolated driver tests pass; entry-point coverage checked | CR-02-final-validation.md, final-test-results.json and entrypoint-coverage.json |
| Issue disposition | Developer defers hardware timing correction to a future CR | open-issues/CR-02-HW-01-recovery-timeout.md |

Final proof counts (verified/errors): ModeManager 9/0, Rx 28/0, Tx 16/0,
MAVLink 69/0, firewall_core 39/0, mavlink_core 38/0. All 1,347 captured source/config
files remain byte-identical to the preceding full verify/build, so its results are
current. There is no separate sys_proof crate or whole-system temporal proof.

Final tests: ModeManager 7, Rx 12, Tx 8, MAVLink 16, firewall_core 18,
mavlink_core 6. Each application's initialize/timeTriggered has 100% measured line
coverage. GUMBOX coverage is 67/67, 382/382, 205/205 and 382/382 respectively.
Approved unused Tx trace-helper and MAVLink unexpected-fixture panic exclusions
remain; LLVM branch counters are unavailable. These are not unqualified branch
coverage claims.

The driver's full host suite cannot run without target seL4 dependencies; it fails
before executing tests. Its isolated helper passes 2/2 and deployed-feature target
check/build succeeds. Its application proof is disabled. This accepted limitation
is distinct from test assertion failure.

Full loader: `hamr/microkit/build/loader.img`, 155,953,132 bytes (148.73 MiB), SHA-256
`cac588401b5a4853aa4a9d482f5a3743f5840063ee888f52027361893c7aba43`.
Build uses custom.mk, ZCU102/debug, microkit-sdk-2.2.0-dev and R2U2 CLI/runtime 4.2.4.
The restored 12-entry legacy schedule matches merged configuration; manager slot
10000 and MAVLink slot 30000, total 208000 legacy units. Physical timing conversion
is not inferred from those raw values.

Manual testing is developer-accepted. Supplied serial evidence contains five flash
denials, a Recovery transition and a D2 timeout; not every procedure has separate
captures and the serial log does not identify its image hash. No new hardware run
is claimed. The developer has explicitly accepted deferral of the timing issue for
this CR. The [final validation summary](CR-02-final-validation.md) consolidates the
exact scope and limitations.

Commit history is appended below. Full hashes are retained in
[CR-02-final-commits.txt](CR-02-final-commits.txt). Uncommitted delivery includes the
supplied _02 requirements, retirement/status/issue records, final reports and the
previously created build helpers; no commit or merge is claimed by this review.

### Commit history

| Commit | Change |
|---|---|
| `24ba011` | mode_manager: Add changeplan and reqs for mode_mgr |
| `2a81bee` | mode_manager: completed requirements |
| `2421087` | mode_manager: SysModeling |
| `86c4a5b` | mode_manager: RxFirewall Contracts |
| `8ee4d70` | mode_manager: Mavlink Firewall GumboSpec |
| `af1edc8` | mode_manager: Mode Manager GumboSpec |
| `88ce34b` | mode_manager: integration checks |
| `b68c70a` | mode_manager: Ran codegen |
| `a562cae` | mode_manager: R2U2 reports |
| `d6fbae6` | mode_manager: Updated R2U2 contracts based on new HAMR context |
| `4bc9a9a` | Update r2u2_monitor.rs |
| `f488aff` | mode_manager: temporary workaround for R2U2 issues |
| `276dd1b` | mode_manager: Finished Wave 1 |
| `06db395` | mode_manager: Updated Verus version for the project |
| `899a204` | mode_manager: Completed Rx Firewall testing and verification |
| `de1682b` | mode_manager: Completed coverage/testing for mavlink firewall |
| `1ddcc75` | mode_manager: Completed mavlink firewall verification |
| `4a57504` | mode_manager: Updated TX Firewall tests |
| `909a2cf` | mode_manager: Completed wave 2 |
| `cff72bc` | mode_manager: Updated domain schedule |
| `04fa3a9` | mode_manager: Update requirements to address TX vulnerability |
| `09ae41d` | mode_manager: Fixed max ipv4 size issue |
| `aaa4312` | mode_manager: update mode_manager verification |
| `9cf01d9` | mode_manager: Use latest HAMR and drop R2U2 patch workaround |

## 6. Impact confirmation and deviations

| Plan prediction | Actual / evidence | Disposition |
|---|---|---|
| Generated monitor needs a supported same-D2 reporting path | Original generator hid false followed by true in one step; authorized patch initially fixed it; current HAMR emits identical fix directly | Temporary workaround approved, now retired; production/probe tests pass |
| Tx and driver initially frozen | Requirements _01 exposed carrier limit mismatch; shared bound, Tx guarantees/tests and independent driver guard added | Explicitly authorized scope amendment; full validation passes |
| Existing toolchain adequate | Project migrated to already-installed newer Verus/Rust | Explicit developer request and approval; six crate proofs pass |
| Per-message Recovery logs per early requirements | Logs removed at developer direction; manager logs transition once | _02 LLR-18 reconciles requirements |
| Legacy requirements documents updated | Developer chose explicit retirement and consolidated supplied requirements | Authorized closeout disposition; original content preserved |
| W3 demonstrates nominal D2 timing | Manual log reports timeout; +200 ms attempt ineffective and reverted | High/Open issue deferred by developer to future CR; non-blocking here, not claimed fixed |
| All component host tests/proofs available | Driver host suite depends on target environment; application has verify=false | Retained documented limitation; exhaustive helper tests and target build pass |
| Full branch coverage measurements | LLVM emits no branch counters | Entry-point/active-line coverage plus semantic partitions and reviewed exclusions retained |

Non-impact inspection against the baseline confirms VMM source/integration,
`mavlink_core/src` parser/dialect and original manual requirements/CR-01 sketch
are unchanged. Parser tests 6/6 and proof 38/0 pass. Carrier model fields remain
unchanged; Rust carrier changes are trailing commas, while the OperatingMode enum
is additive. Tx has no mode input; driver four-lane interfaces remain. Generated
bridge/queue formatting and API changes are recorded rather than claimed untouched.
No unsupported claim of unmeasured end-to-end traffic preservation is made.
See [non-impact evidence](CR-02-final-nonimpact.json) and artifact inventory.

## 7. Completion review

ChangeExec.AP2 approved by the developer on 2026-09-23: all waves approved, requirements reconciliation complete,
test/verification/build evidence consolidated, impact deviations accounted for,
and the timing issue explicitly transferred to future-CR work. The issue remains
High/Open; its future CR ID and owner are unassigned. This transfer does not block
CR-02 acceptance and does not weaken its recorded closure criteria.

The developer explicitly approved this final review. CR-02 is complete with the
recorded limitations and deferred timing issue. No commit or merge is implied by
this completion approval.
