# Effort report — mavlink_02 (2026-09-23)

> **Scope limitation.** CHANGE-REQUEST SCOPE — This is a current-project snapshot estimate under the greenfield-oriented v1 model, not an incremental CR-02 effort estimate. Inherited CR-01 and vendor content are present; current CR-02 requirements under action-requests are excluded by the measurer, while retired requirements are counted. Some workflow status and assurance values below are historical or unrecognized by the parser. Unknown-provenance code is excluded from modeled headline effort, so those totals are not a complete implementation estimate. The observed session covers CR-02 only. Ratios are suppressed; no savings are measured. All hours and costs beyond observed session metrics are an UNCALIBRATED MODEL. See the data-quality caveats for details.

**UNCALIBRATED MODEL** — modeled values use uncalibrated placeholder rates; every headline is tagged Observed (measured), Modeled (estimated under a named profile/scenario/counterfactual), or Compared (scope-gated). Experiment `mavlink_02`, workflow ChangeExec(CR-02) (audited profile), harness codex.

## 1. Executive summary

**HAMRvsManual (Modeled).** HAMR generated **16,496 SLOC** of infrastructure, configuration, and assurance artifacts. Hand-building a manual equivalent is modeled at **1,122 h–4,708 h** (**$168,224–$517,915**) across profiles (experienced_sel4, new_to_sel4) and equivalence scenarios (nominal, conservative). This is a *modeled manual-equivalent replacement value*, never a measured net saving.

**HAMR-AGENTvsHAMR (Compared).** Observed agent session: **—** (unavailable), **5.04 active hours** (295,554 output tokens). Modeled human effort for the classified project-snapshot artifacts plus parsed tool-running overheads (different scope from the session): **263 h–545 h** (**$39,400–$59,952**) by profile. Scope alignment is `partial`; no ratio is rendered.

> **UNCALIBRATED MODEL.** Every modeled number above uses uncalibrated placeholder productivity rates under a named profile, scenario, and counterfactual (§7). Observed values are measured; modeled values are estimates; no calibrated prediction accuracy is claimed.

Authored-artifact composition (Observed SLOC; modeled per profile in §2/§4):

| Artifact type | Counterfactual category | SLOC |
|---|---|---|
| build_script | HAMR-path authored | 523 |
| gumbo_contract | HAMR-path authored | 453 |
| requirements_text | common (both paths) | 453 |
| sysml_model | common (both paths) | 421 |

## 2. Per-workflow-step view

Modeled hours cover developer/agent-authored work plus tool-running overheads on the
HAMR path; generated SLOC appears per step for context but is costed only in §3.
Per-step agent actuals are unavailable in this pipeline — the session total appears once in §6.

| Step | Status | Artifacts | SLOC by provenance | Assurance (Observed) | Modeled hours (experienced_sel4 / new_to_sel4) | Agent actuals |
|---|---|---|---|---|---|---|
| SysPlanAndReq.1 | done | ConOps | gen 0 · auth 160 · unk 0 | — | 13 h / 20 h | — |
| SysPlanAndReq.3 | done | CompReqs | gen 0 · auth 56 · unk 0 | — | 4.7 h / 7.0 h | — |
| SysPlanAndReq.4 | done | DataDict | gen 0 · auth 39 · unk 0 | — | 3.2 h / 4.9 h | — |
| SysModeling † | done | SysModel | gen 0 · auth 421 · unk 0 | — | 70 h / 105 h | — |
| SysModeling.5 | done | run sireum tipe and fix trivial findings | — | — | 0.2 h / 0.4 h | — |
| CompGUMBOSpec(*) † | — | CompContracts | gen 0 · auth 144 · unk 0 | — | 48 h / 120 h | — |
| CompGUMBOSpec(mavlinkfirewall) † | done | CompContracts | gen 0 · auth 125 · unk 0 | — | 42 h / 104 h | — |
| CompGUMBOSpec(modemanager) † | — | CompContracts | gen 0 · auth 23 · unk 0 | — | 7.7 h / 19 h | — |
| CompGUMBOSpec(rxfirewall) † | done | CompContracts | gen 0 · auth 105 · unk 0 | — | 35 h / 88 h | — |
| CompGUMBOSpec(txfirewall) † | — | CompContracts | gen 0 · auth 56 · unk 0 | — | 19 h / 47 h | — |
| SysGUMBOIntegrationCheck.1 | done | run Logika integration check and record vacuity/handshakes | — | handshakes N=0 — vacuous by design (notes) | 0.2 h / 0.4 h | — |
| CodeGen.2 | done | GenCode | gen 16,496 · auth 0 · unk 732 | 146 resources (94 regen / 52 once) (report) | 0.5 h / 0.8 h | — |
| CodeGen.4 | n | BuildScript | gen 0 · auth 523 · unk 0 | — | 0.5 h / 0.8 h | — |
| CompDev(*).2 | — | CompImpls | gen 0 · auth 0 · unk 1,116 | — | 0.0 h / 0.0 h | — |
| CompDev(*).3 | — | CompTests | gen 0 · auth 0 · unk 932 | — | 0.0 h / 0.0 h | — |
| CompDev(ArduPilot).4 | — | run tests with coverage instrumentation and read reports | — | — | 0.0 h / 0.0 h | — |
| CompDev(LowLevelEthernetDriver).4 | — | run tests with coverage instrumentation and read reports | — | — | 0.0 h / 0.0 h | — |
| CompDev(MAVLinkFirewall).4 | done | run tests with coverage instrumentation and read reports | — | — | 0.5 h / 0.8 h | — |
| CompDev(ModeManager).4 | — | run tests with coverage instrumentation and read reports | — | — | 0.0 h / 0.0 h | — |
| CompDev(RxFirewall).4 | done | run tests with coverage instrumentation and read reports | — | entrypoint lines 100% (notes) | 0.5 h / 0.8 h | — |
| CompDev(TxFirewall).4 | — | run tests with coverage instrumentation and read reports | — | — | 0.0 h / 0.0 h | — |
| CompDev(ArduPilot).6 | — | run Verus verification and triage results | — | — | 0.0 h / 0.0 h | — |
| CompDev(LowLevelEthernetDriver).6 | — | run Verus verification and triage results | — | — | 0.0 h / 0.0 h | — |
| CompDev(MAVLinkFirewall).6 | done | run Verus verification and triage results | — | Verus 14/0 (notes) | 0.5 h / 0.8 h | — |
| CompDev(ModeManager).6 | — | run Verus verification and triage results | — | — | 0.0 h / 0.0 h | — |
| CompDev(RxFirewall).6 | done | run Verus verification and triage results | — | Verus 27/0 (notes) | 0.5 h / 0.8 h | — |
| CompDev(TxFirewall).6 | — | run Verus verification and triage results | — | — | 0.0 h / 0.0 h | — |
| SysSchedDef.1 ‡ | done | Schedule | gen 0 · auth 0 · unk 14 | — | 0.2 h / 0.4 h | — |
| SysGUMBOSysSpecCheck.5 | — | run the system-proof crate through Verus and triage results | — | — | 0.0 h / 0.0 h | — |
| unattributed § | — | — | gen 0 · auth 198 · unk 339,513 | — | 16 h / 25 h | — |

† artifact produced across several steps of this workflow (workflow-only granularity) · ‡ inferred step (file generated earlier, edited here) · § unattributed accounting bucket. Modeled hours are UNCALIBRATED MODEL placeholders.

## 3. HAMR generation value detail (Modeled)

Generated SLOC (Observed): **16,496** across 7 artifact types. Modeled replacement estimates below are UNCALIBRATED MODEL placeholders; the conservative scenario halves the line-equivalence.

| Artifact type | SLOC | experienced_sel4 · nominal | experienced_sel4 · conservative | new_to_sel4 · nominal | new_to_sel4 · conservative |
|---|---|---|---|---|---|
| build_config | 54 | 4.5 h | 2.2 h | 6.8 h | 3.4 h |
| c_infra | 2,370 | 395 h | 198 h | 1,185 h | 592 h |
| makefile | 3,637 | 364 h | 182 h | 546 h | 273 h |
| microkit_config | 649 | 216 h | 108 h | 865 h | 433 h |
| rust_contract_woven | 842 | 210 h | 105 h | 526 h | 263 h |
| rust_infra | 7,383 | 923 h | 461 h | 1,384 h | 692 h |
| rust_test_infra | 1,561 | 130 h | 65 h | 195 h | 98 h |
| **Total** | **16,496** | **2,243 h ($336,449)** | **1,122 h ($168,224)** | **4,708 h ($517,915)** | **2,354 h ($258,958)** |

Generate-once templates: HAMR supplied **0** template SLOC in 0 files; **0** retained in the final system (counted above), **0** replaced by the developer/agent (counted conservatively as authored work, not as HAMR value).

Unknown-provenance bucket: **342,307 SLOC** — origin unresolved (no compatible template baseline); counted in neither headline story.

## 4. Common-mode artifacts (both counterfactual paths)

These artifacts are authored in both counterfactuals (fixed decision: SysMLv2 models
remain valuable as design/documentation without code generation). Their effort is
displayed for both paths and never counted toward the HAMR generation-value headline.

| Artifact type | SLOC | Modeled (experienced_sel4) | Modeled (new_to_sel4) |
|---|---|---|---|
| requirements_text | 453 | 38 h | 57 h |
| sysml_model | 421 | 70 h | 105 h |
| **Total** | **874** | **108 h ($16,188)** | **162 h ($17,806)** |

## 5. Assurance detail (Observed)

**Tests** (static counts are exact; runtime results derive from workflow-status Notes):

| Component | Defined (static) | Run (notes) | Static = runtime? |
|---|---|---|---|
| ArduPilot | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |
| LowLevelEthernetDriver | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |
| MAVLinkFirewall | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |
| ModeManager | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |
| RxFirewall | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |
| TxFirewall | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |

| Component | Entrypoint line coverage % | Confidence |
|---|---|---|
| ArduPilot | — | derived |
| LowLevelEthernetDriver | — | derived |
| MAVLinkFirewall | — | derived |
| ModeManager | — | derived |
| RxFirewall | 100 | derived |
| TxFirewall | — | derived |

| Component | Verus obligations verified | Errors | Confidence |
|---|---|---|---|
| ArduPilot | None | None | derived |
| LowLevelEthernetDriver | None | None | derived |
| MAVLinkFirewall | 14 | 0 | derived |
| ModeManager | None | None | derived |
| RxFirewall | 27 | 0 | derived |
| TxFirewall | None | None | derived |

System VCs: — (no SysGUMBOSysSpecCheck.5 row (step deferred or absent)).

Integration check: expected handshakes N=0 — **vacuous by design** (zero receiver-side integration assumes; hamr-sysml-patterns.md §2) (notes, derived).

Experiment assessment findings: 12 (1 blocker, 1 minor, 7 moderate, 3 positive) — assessment-summary.json, exact.

| Component | Contract-audit findings (CompGUMBOSpec.3) | Confidence |
|---|---|---|
| ArduPilot | None | derived |
| LowLevelEthernetDriver | None | derived |
| MAVLinkFirewall | 9 | derived |
| ModeManager | None | derived |
| RxFirewall | 9 | derived |
| TxFirewall | None | derived |

## 6. Agent session totals (Observed)

| Session metric (Observed) | Value |
|---|---|
| Harness | codex 0.154.0 |
| Models | gpt-6-astra |
| Metrics schema / adapter | schema-v2 |
| Tokens (in / out / cache r / cache w) | 2,345,891 / 295,554 / 82,935,168 / 0 |
| Cost | — (unavailable) |
| Active / wall-clock time | 18,143 s / 132,704 s |
| Friction (escalations / dynamic shell / sandbox failures) | 0 / 0 / 0 |

Caveats:

- 1267 parser coverage warning(s); friction counts undercounted
- actual billed cost unavailable
- subagents excluded

## 7. Assumptions and data quality

Estimator: `linear_sloc_v1` — calibration status **uncalibrated_placeholder**. Scope: Models coding and unit-debug effort only. Excludes deployment, CI, board bring-up, assurance-case authoring, requirements elicitation meetings, and team coordination overheads.

| Artifact type | SLOC/hour (placeholder) | Rationale |
|---|---|---|
| build_config | 12 | Cargo manifests and toolchain pins; mechanical. |
| build_script | 8 (fixed 0.5 h override) | bin/build.cmd is generated by the setup-build-script skill, which a non-agentic HAMR user also has; modeled as a fixed setup activity (fixed_hours_override), not hand-written SLOC. The SLOC rate is retained only for a manual-path sensitivity discussion. Conservative: understates human effort on the HAMR path. |
| c_infra | 6 | Hand-written seL4/Microkit C bridge, queue, and FFI code: low-level, shared-memory, concurrency-sensitive; embedded-C productivity literature supports single-digit SLOC/h for this class. |
| c_user_code | 8 | C-to-Rust user bridge shims; mechanical but FFI-sensitive. |
| docs | 20 | Narrative documentation. |
| gumbo_contract | 3 | GUMBO state/integration/initialize/compute clauses: formal contract authoring rate. |
| makefile | 10 | Build rules for cross-compiled Microkit images; mostly patterned but fragile. |
| microkit_config | 3 | Microkit system XML (protection domains, memory regions, channels) and schedule slots: dense, error-prone platform configuration. |
| proof_code | 4 | Verus system-proof code (state machines, VCs, lemmas): formal-methods authoring rate; seL4 proof-effort literature is the extreme upper bound. |
| requirements_text | 12 | ConOps/requirements prose and tables with stable IDs; lines-of-prose is a weak proxy — see requirement_items counts for a future per-item rate. |
| rust_app_code | 10 | Application entry-point logic against a typed port API; ordinary verified-application Rust. |
| rust_contract_woven | 4 | Verus contract clauses (requires/ensures) hand-authored on entry points; formal-specification authoring is slow per line. |
| rust_infra | 8 | Hand-written Rust port APIs, FFI bindings, and crate wiring for a no_std target; systems-Rust rate placeholder. |
| rust_test_code | 15 | Hand-written unit/contract/property test bodies; tests are faster per line than product code. |
| rust_test_infra | 12 | Test harness plumbing (port put/get shims, oracle wiring, generators) a manual path would hand-build. |
| sysml_model | 6 | SysMLv2 architecture and data modeling in the HAMR subset. |

| Profile | Rate | Default multiplier | Type multipliers |
|---|---|---|---|
| experienced_sel4 | $150/h | ×1.0 | — |
| new_to_sel4 | $110/h | ×1.5 | c_infra ×3.0, gumbo_contract ×2.5, microkit_config ×4.0, proof_code ×4.0, rust_contract_woven ×2.5 |

- Equivalence scenario **nominal** (factor 1.0): A manual build writes an equivalent of every generated line.
- Equivalence scenario **conservative** (factor 0.5): A hand-built system may need only about half the generated line count (less generality, fewer harness layers).

Counterfactual category assignments (Observed SLOC per category):

| Category | SLOC | By artifact type |
|---|---|---|
| common_both_paths | 874 | requirements_text 453, sysml_model 421 |
| hamr_generated_manual_replacement | 16,496 | build_config 54, c_infra 2,370, makefile 3,637, microkit_config 649, rust_contract_woven 842, rust_infra 7,383, rust_test_infra 1,561 |
| hamr_path_authored | 976 | build_script 523, gumbo_contract 453 |
| unknown_provenance | 342,307 | build_config 188, c_infra 125,303, c_user_code 78, docs 214,119, makefile 197, microkit_config 14, rust_app_code 1,116, rust_infra 350, rust_test_code 932, rust_test_infra 10 |

Manual-path effort deliberately **not** modeled (the honest asymmetry — these costs exist but are speculative):
- Manual-path behavioral specification and verification effort in place of GUMBO contracts and Verus proofs
- Manual-path test-harness construction and test authoring for hand-built infrastructure
- Manual acquisition of seL4/Microkit platform expertise (protection domains, channels, static schedules)
- Manual integration debugging of hand-written IPC/shared-memory plumbing
- Manual maintenance of consistency between architecture documentation and code

Attribution accounting: **5.8%** of measured SLOC attributed to workflow steps; granularity mix: exact_step 20,054, inferred_step 14, unattributed 339,711, workflow_only 874. 100% of measured SLOC sits in exactly one accounting bucket including `unattributed`.

Data-quality caveats:
- codegen report is PARTIAL: internal status 'Success' is not trusted as a completeness signal; 1243 generated-looking file(s) on disk are absent from the report
- no generation baseline supplied; non-marker content of generate-once files is preserved_origin_unknown
- all effort estimates are UNCALIBRATED MODEL placeholders (calibration_status=uncalibrated_placeholder)
- CHANGE-REQUEST SCOPE: Measurements cover the current project snapshot, including inherited CR-01 artifacts; they are not the CR-02 changed-line delta. The measured agent session covers CR-02 only, so modeled-vs-observed ratios must remain suppressed.
- The required concept_quote is retained verbatim from requirements/conops.md section 0, a retired CR-01 concept. CR-02 authority is action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md and its approved change plan.
- The greenfield-oriented measurer excludes action-requests wholesale, including the current CR-02 requirements, but measures the historical requirements directory. Requirements effort is therefore not a complete or current CR-02 estimate.
- Workflow status contains both CR-01 history and CR-02 records. Derived step and assurance observations may mix historical evidence; the authoritative final results are reports/CR-02/final-validation/CR-02-final-validation.md.
- Subagent activity is excluded by schema-v2 metrics. Nested-shell friction has 1267 parser warnings; top-level counts are not complete nested-command counts.
- Both actual billed cost and API list-price-equivalent cost are unavailable in this session; no agent dollar cost or measured savings can be inferred.
- No pristine generation baseline was prepared; generate-once non-marker provenance remains preserved_origin_unknown and is not silently counted as developer-authored.
- Coding and unit-debug placeholder rates exclude deployment, board bring-up, assurance-case writing, elicitation meetings and coordination. Hardware timing CR-02-HW-01 remains High/Open and deferred.
- The skill-referenced docs/effort-measurement-spec.md is absent at the repository root. Counting is performed by the bundled v1 scripts/configs with line-allocation validation; no missing specification content is assumed.
- Observed SLOC is a snapshot inventory, not newly produced session output. Vendor/VMM content contributes to the large unknown and unattributed buckets; the partial codegen report is not itself evidence of a failed generation.
- Workflow parsing misses some CR-02-qualified steps and retains older CR-01 assurance values (for example MAVLink 14/0 and Rx 27/0). A dash or zero modeled overhead does not mean CR-02 work was absent or incomplete. Final recorded proof counts are ModeManager 9/0, Rx 28/0, Tx 16/0, MAVLink 69/0, firewall_core 39/0, mavlink_core 38/0; 67 application/core tests and 2 driver helper tests pass. See reports/CR-02/final-validation/CR-02-final-validation.md.

Reproducibility:
- effort-model config SHA-256 `f4818bf48820c984fef3c5ecde38c9940543990e91816a3bbecce8e749618bb4`
- counterfactual config SHA-256 `39507c77cdeb96f70a04159e74beb9ec49a33772ec88b1893daa63265d1eebb9`
- session-metrics source `/home/robertvanvossen/dev/agentic-hamr-experiments-02/mavlink_02/experiment-reports/session-metrics.json` (schema-v2)
- generation baseline: none (generate-once provenance falls back to preserved_origin_unknown)
- codegen report status **partial** (146 raw / 146 deduped resources; classification mode report_plus_fallback)
- generated timestamp 2026-09-23T21:34:00-04:00 (injectable via --now)
