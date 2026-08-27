# Effort report — mavlink_02 (2026-08-27)

**UNCALIBRATED MODEL** — modeled values use uncalibrated placeholder rates; every headline is tagged Observed (measured), Modeled (estimated under a named profile/scenario/counterfactual), or Compared (scope-gated). Experiment `mavlink_02`, workflow ChangeExec(CR-01) (audited profile), harness codex.

## 1. Executive summary

**HAMRvsManual (Modeled).** HAMR generated **13,850 SLOC** of infrastructure, configuration, and assurance artifacts. Hand-building a manual equivalent is modeled at **941 h–3,935 h** (**$141,116–$432,822**) across profiles (experienced_sel4, new_to_sel4) and equivalence scenarios (nominal, conservative). This is a *modeled manual-equivalent replacement value*, never a measured net saving.

**HAMR-AGENTvsHAMR (Compared).** Observed agent session: **$50** (list-price equivalent), **3.28 active hours** (220,658 output tokens). Modeled human effort for the same authored artifacts plus tool-running overheads: **209 h–423 h** (**$31,401–$46,495**) by profile. Scope alignment is `partial`; no ratio is rendered.

> **UNCALIBRATED MODEL.** Every modeled number above uses uncalibrated placeholder productivity rates under a named profile, scenario, and counterfactual (§7). Observed values are measured; modeled values are estimates; no calibrated prediction accuracy is claimed.

Authored-artifact composition (Observed SLOC; modeled per profile in §2/§4):

| Artifact type | Counterfactual category | SLOC |
|---|---|---|
| gumbo_contract | HAMR-path authored | 326 |
| requirements_text | common (both paths) | 421 |
| sysml_model | common (both paths) | 374 |

## 2. Per-workflow-step view

Modeled hours cover developer/agent-authored work plus tool-running overheads on the
HAMR path; generated SLOC appears per step for context but is costed only in §3.
Per-step agent actuals do not exist in Wave 1 — the session total appears once in §6.

| Step | Status | Artifacts | SLOC by provenance | Assurance (Observed) | Modeled hours (experienced_sel4 / new_to_sel4) | Agent actuals |
|---|---|---|---|---|---|---|
| SysPlanAndReq.1 | done | ConOps | gen 0 · auth 152 · unk 0 | — | 13 h / 19 h | — |
| SysPlanAndReq.3 | done | CompReqs | gen 0 · auth 48 · unk 0 | — | 4.0 h / 6.0 h | — |
| SysPlanAndReq.4 | done | DataDict | gen 0 · auth 31 · unk 0 | — | 2.6 h / 3.9 h | — |
| SysModeling † | done | SysModel | gen 0 · auth 374 · unk 0 | — | 62 h / 94 h | — |
| SysModeling.5 | done | run sireum tipe and fix trivial findings | — | — | 0.2 h / 0.4 h | — |
| CompGUMBOSpec(*) † | — | CompContracts | gen 0 · auth 140 · unk 0 | — | 47 h / 117 h | — |
| CompGUMBOSpec(mavlinkfirewall) † | done | CompContracts | gen 0 · auth 63 · unk 0 | — | 21 h / 52 h | — |
| CompGUMBOSpec(rxfirewall) † | done | CompContracts | gen 0 · auth 76 · unk 0 | — | 25 h / 63 h | — |
| CompGUMBOSpec(txfirewall) † | — | CompContracts | gen 0 · auth 47 · unk 0 | — | 16 h / 39 h | — |
| SysGUMBOIntegrationCheck.1 | done | run Logika integration check and record vacuity/handshakes | — | handshakes N=0 — vacuous by design (notes) | 0.2 h / 0.4 h | — |
| CodeGen.2 | done | GenCode | gen 13,850 · auth 0 · unk 533 | 118 resources (75 regen / 43 once) (report) | 0.5 h / 0.8 h | — |
| CodeGen.4 | n | generate the build script via the setup-build-script skill and verify usage | — | — | 0.0 h / 0.0 h | — |
| CompDev(*).2 | — | CompImpls | gen 0 · auth 0 · unk 722 | — | 0.0 h / 0.0 h | — |
| CompDev(*).3 | — | CompTests | gen 0 · auth 0 · unk 350 | — | 0.0 h / 0.0 h | — |
| CompDev(ArduPilot).4 | — | run tests with coverage instrumentation and read reports | — | — | 0.0 h / 0.0 h | — |
| CompDev(LowLevelEthernetDriver).4 | — | run tests with coverage instrumentation and read reports | — | — | 0.0 h / 0.0 h | — |
| CompDev(MAVLinkFirewall).4 | done | run tests with coverage instrumentation and read reports | — | — | 0.5 h / 0.8 h | — |
| CompDev(RxFirewall).4 | done | run tests with coverage instrumentation and read reports | — | entrypoint lines 100% (notes) | 0.5 h / 0.8 h | — |
| CompDev(TxFirewall).4 | — | run tests with coverage instrumentation and read reports | — | — | 0.0 h / 0.0 h | — |
| CompDev(ArduPilot).6 | — | run Verus verification and triage results | — | — | 0.0 h / 0.0 h | — |
| CompDev(LowLevelEthernetDriver).6 | — | run Verus verification and triage results | — | — | 0.0 h / 0.0 h | — |
| CompDev(MAVLinkFirewall).6 | done | run Verus verification and triage results | — | Verus 14/0 (notes) | 0.5 h / 0.8 h | — |
| CompDev(RxFirewall).6 | done | run Verus verification and triage results | — | Verus 27/0 (notes) | 0.5 h / 0.8 h | — |
| CompDev(TxFirewall).6 | — | run Verus verification and triage results | — | — | 0.0 h / 0.0 h | — |
| SysSchedDef.1 ‡ | done | Schedule | gen 0 · auth 0 · unk 12 | — | 0.2 h / 0.4 h | — |
| SysGUMBOSysSpecCheck.5 | — | run the system-proof crate through Verus and triage results | — | — | 0.0 h / 0.0 h | — |
| unattributed § | — | — | gen 0 · auth 190 · unk 339,513 | — | 16 h / 24 h | — |

† artifact produced across several steps of this workflow (workflow-only granularity) · ‡ inferred step (file generated earlier, edited here) · § unattributed accounting bucket. Modeled hours are UNCALIBRATED MODEL placeholders.

## 3. HAMR generation value detail (Modeled)

Generated SLOC (Observed): **13,850** across 7 artifact types. Modeled replacement estimates below are UNCALIBRATED MODEL placeholders; the conservative scenario halves the line-equivalence.

| Artifact type | SLOC | experienced_sel4 · nominal | experienced_sel4 · conservative | new_to_sel4 · nominal | new_to_sel4 · conservative |
|---|---|---|---|---|---|
| build_config | 41 | 3.4 h | 1.7 h | 5.1 h | 2.6 h |
| c_infra | 1,834 | 306 h | 153 h | 917 h | 458 h |
| makefile | 3,598 | 360 h | 180 h | 540 h | 270 h |
| microkit_config | 572 | 191 h | 95 h | 763 h | 381 h |
| rust_contract_woven | 709 | 177 h | 89 h | 443 h | 222 h |
| rust_infra | 6,082 | 760 h | 380 h | 1,140 h | 570 h |
| rust_test_infra | 1,014 | 84 h | 42 h | 127 h | 63 h |
| **Total** | **13,850** | **1,882 h ($282,232)** | **941 h ($141,116)** | **3,935 h ($432,822)** | **1,967 h ($216,411)** |

Generate-once templates: HAMR supplied **0** template SLOC in 0 files; **0** retained in the final system (counted above), **0** replaced by the developer/agent (counted conservatively as authored work, not as HAMR value).

Unknown-provenance bucket: **341,130 SLOC** — origin unresolved (no compatible template baseline); counted in neither headline story.

## 4. Common-mode artifacts (both counterfactual paths)

These artifacts are authored in both counterfactuals (fixed decision: SysMLv2 models
remain valuable as design/documentation without code generation). Their effort is
displayed for both paths and never counted toward the HAMR generation-value headline.

| Artifact type | SLOC | Modeled (experienced_sel4) | Modeled (new_to_sel4) |
|---|---|---|---|
| requirements_text | 421 | 35 h | 53 h |
| sysml_model | 374 | 62 h | 94 h |
| **Total** | **795** | **97 h ($14,612)** | **146 h ($16,074)** |

## 5. Assurance detail (Observed)

**Tests** (static counts are exact; runtime results derive from workflow-status Notes):

| Component | Defined (static) | Run (notes) | Static = runtime? |
|---|---|---|---|
| ArduPilot | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |
| LowLevelEthernetDriver | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |
| MAVLinkFirewall | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |
| RxFirewall | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |
| TxFirewall | 0 (0 tests.rs + 0 cb_apis.rs) | None passed / None failed | — |

| Component | Entrypoint line coverage % | Confidence |
|---|---|---|
| ArduPilot | — | derived |
| LowLevelEthernetDriver | — | derived |
| MAVLinkFirewall | — | derived |
| RxFirewall | 100 | derived |
| TxFirewall | — | derived |

| Component | Verus obligations verified | Errors | Confidence |
|---|---|---|---|
| ArduPilot | None | None | derived |
| LowLevelEthernetDriver | None | None | derived |
| MAVLinkFirewall | 14 | 0 | derived |
| RxFirewall | 27 | 0 | derived |
| TxFirewall | None | None | derived |

System VCs: — (no SysGUMBOSysSpecCheck.5 row (step deferred or absent)).

Integration check: expected handshakes N=0 — **vacuous by design** (zero receiver-side integration assumes; hamr-sysml-patterns.md §2) (notes, derived).

Experiment assessment findings: not available (no assessment-summary.json for this experiment).

| Component | Contract-audit findings (CompGUMBOSpec.3) | Confidence |
|---|---|---|
| ArduPilot | None | derived |
| LowLevelEthernetDriver | None | derived |
| MAVLinkFirewall | 9 | derived |
| RxFirewall | 9 | derived |
| TxFirewall | None | derived |

## 6. Agent session totals (Observed)

| Session metric (Observed) | Value |
|---|---|
| Harness | codex 0.149.0 |
| Models | gpt-5.6-sol |
| Metrics schema / adapter | schema-v2 |
| Tokens (in / out / cache r / cache w) | 1,842,495 / 220,658 / 67,979,264 / 0 |
| Cost | $50 (list_price_equivalent) |
| Active / wall-clock time | 11,825 s / 71,026 s |
| Friction (escalations / dynamic shell / sandbox failures) | 0 / 35 / 0 |

Caveats:

- 608 parser coverage warning(s); friction counts undercounted
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
| common_both_paths | 795 | requirements_text 421, sysml_model 374 |
| hamr_generated_manual_replacement | 13,850 | build_config 41, c_infra 1,834, makefile 3,598, microkit_config 572, rust_contract_woven 709, rust_infra 6,082, rust_test_infra 1,014 |
| hamr_path_authored | 326 | gumbo_contract 326 |
| unknown_provenance | 341,130 | build_config 157, c_infra 125,303, c_user_code 65, docs 214,119, makefile 144, microkit_config 12, rust_app_code 722, rust_infra 250, rust_test_code 350, rust_test_infra 8 |

Manual-path effort deliberately **not** modeled (the honest asymmetry — these costs exist but are speculative):
- Manual-path behavioral specification and verification effort in place of GUMBO contracts and Verus proofs
- Manual-path test-harness construction and test authoring for hand-built infrastructure
- Manual acquisition of seL4/Microkit platform expertise (protection domains, channels, static schedules)
- Manual integration debugging of hand-written IPC/shared-memory plumbing
- Manual maintenance of consistency between architecture documentation and code

Attribution accounting: **4.6%** of measured SLOC attributed to workflow steps; granularity mix: exact_step 15,686, inferred_step 12, unattributed 339,703, workflow_only 700. 100% of measured SLOC sits in exactly one accounting bucket including `unattributed`.

Data-quality caveats:
- codegen report contains 42 duplicate resource path(s); deduplicated (flags agreed)
- codegen report is PARTIAL: internal status 'Success' is not trusted as a completeness signal; 1238 generated-looking file(s) on disk are absent from the report
- no generation baseline supplied; non-marker content of generate-once files is preserved_origin_unknown
- all effort estimates are UNCALIBRATED MODEL placeholders (calibration_status=uncalibrated_placeholder)

Reproducibility:
- effort-model config SHA-256 `f4818bf48820c984fef3c5ecde38c9940543990e91816a3bbecce8e749618bb4`
- counterfactual config SHA-256 `39507c77cdeb96f70a04159e74beb9ec49a33772ec88b1893daa63265d1eebb9`
- session-metrics source `/home/robertvanvossen/dev/agentic-hamr-experiments-02/mavlink_02/experiment-reports/session-metrics.json` (schema-v2)
- generation baseline: none (generate-once provenance falls back to preserved_origin_unknown)
- codegen report status **partial** (160 raw / 118 deduped resources; classification mode report_plus_fallback)
- generated timestamp 2026-08-27T10:09:50-04:00 (injectable via --now)
