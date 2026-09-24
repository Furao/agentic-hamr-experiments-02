# Category 2 — HAMR codegen and tooling

Assessment of CR-02 in mavlink_02 only. Historical findings and current dispositions are distinguished below.

Severity: 🔴 blocker · 🟠 moderate · 🟡 minor · 🟢 positive.

---

## CG-01 — Generated verdict caching lost the same-dispatch timeout failure 🔴

**Scope.** `hamr`. **Severity.** blocker. **Disposition.** resolved by HAMR upgrade.

**Evidence.** [reports/CR-02/monitoring/CR-02-w1-r2u2-reporting.md](../reports/CR-02/monitoring/CR-02-w1-r2u2-reporting.md); [reports/CR-02/codegen/CR-02-hamr-upgrade.md](../reports/CR-02/codegen/CR-02-hamr-upgrade.md); [hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall/src/component/r2u2_monitor.rs:69–90](../hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall/src/component/r2u2_monitor.rs); [tests/r2u2_monitor_probe/README.md](../tests/r2u2_monitor_probe/README.md).

**Impact.** Historically blocked W1: the real runtime returned false then true in D2, but the generated log exposed only true. The initial probe passed 1 test and failed 1 despite a compiled specification.

**Root cause.** The generated latest-verdict cache overwrote the false verdict within a single monitor step. Current code retains false_verdict_seen for reporting.

**Recommendation.** Target: HAMR GumboR2U2Util.processRustOutputs regression suite. Promote the isolated raw-runtime versus generated-reporting probe into generator regression tests; cover delayed triggers, false-then-true, timely/late/absent Recovery and reboot. Continue validation after successful generation without invoking the retired patch.

**Evidence limit.** Blocker is historical severity: the upgraded generator is byte-identical to the patched monitor and the probe passes 2/2. The tested unmapped reporting path does not establish correctness of every alert mapping or resolve the separate hardware timeout.

---

## CG-02 — Generated build prerequisites couple verification to compiler installation 🟠

**Scope.** `hamr`. **Severity.** moderate. **Disposition.** mitigated locally; generator follow-up.

**Evidence.** [reports/CR-02/deployment/CR-02-w3-build.md: Changes](../reports/CR-02/deployment/CR-02-w3-build.md); [reports/CR-02/final-validation/CR-02-full-verify-build.md](../reports/CR-02/final-validation/CR-02-full-verify-build.md); [hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall/Makefile](../hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall/Makefile); [bin/compile-r2u2.py](../bin/compile-r2u2.py); [experiment-reports/session-transcript.md:6177–6183](../experiment-reports/session-transcript.md).

**Impact.** Standalone verification attempted a crates.io compiler install, adding network dependence even when an installed R2U2 4.2.4 compiler was available. The custom build needed an override.

**Root cause.** Generated component prerequisites retain installation behavior; the project custom path separates installed-tool validation and specification compilation.

**Recommendation.** Target: HAMR Microkit R2U2 Makefile generation. Make an explicit R2U2_CLI path/version check the normal prerequisite, compile the specification reproducibly, and move installation into a separate setup target. Preserve locked dependency preparation.

**Evidence limit.** The project custom build already mitigates this. No clean-cache, fully offline rebuild was demonstrated; future availability of the compiler under /tmp is not guaranteed.

---

## CG-03 — Generator schedule format and installed SDK required compatibility handling 🟠

**Scope.** `environment`. **Severity.** moderate. **Disposition.** compatibility resolved; timing remains qualified.

**Evidence.** [reports/CR-02/deployment/CR-02-schedule-review.md](../reports/CR-02/deployment/CR-02-schedule-review.md); [reports/CR-02/deployment/CR-02-w3-build.md: Schedule and memory inspection](../reports/CR-02/deployment/CR-02-w3-build.md); [hamr/microkit/microkit.schedule.xml](../hamr/microkit/microkit.schedule.xml); [sysmlv2/open_platform/Platform.sysml](../sysmlv2/open_platform/Platform.sysml).

**Impact.** The installed SDK rejected the new domains wrapper. Retaining the developer-selected legacy XML enabled the loader build, but raw schedule units do not demonstrate physical periods or D2 behavior.

**Root cause.** Generator and installed microkit-sdk-2.2.0-dev expected different schedule schemas; later CapDL checks established order, not complete timing conversion.

**Recommendation.** Target: Project toolchain preflight and SysSchedDef. Record generator/SDK schema compatibility before schedule edits. Validate the merged schedule with the installed SDK and retain domain mapping, timer-unit conversion and measured target timing as separate checks.

**Evidence limit.** Schema/build compatibility is resolved for this installation. The evidence does not diagnose CR-02-HW-01 or support a universal SDK incompatibility claim.

---

## CG-04 — Final regeneration and assurance evidence preserve explicit trust boundaries 🟢

**Scope.** `hamr`. **Severity.** positive. **Disposition.** observed positive.

**Evidence.** [reports/CR-02/codegen/CR-02-hamr-upgrade-comparison.json](../reports/CR-02/codegen/CR-02-hamr-upgrade-comparison.json); [reports/CR-02/final-validation/CR-02-final-validation.md](../reports/CR-02/final-validation/CR-02-final-validation.md); [reports/CR-02/contracts/CR-02-integration-check.md](../reports/CR-02/contracts/CR-02-integration-check.md); [hamr/microkit/crates/seL4_ModeManager_ModeManager/src/component/seL4_ModeManager_ModeManager_app.rs](../hamr/microkit/crates/seL4_ModeManager_ModeManager/src/component/seL4_ModeManager_ModeManager_app.rs).

**Impact.** The upgraded generation preserved 1320 compared files and all 52 reported editable resources. Final evidence records 67 application/core tests, 2 isolated driver tests, six proof-enabled crates and a successful target build without conflating these with temporal system proof.

**Root cause.** Snapshot comparison, independent application implementations, actual-runtime monitor probes and qualified coverage/verification reports provide complementary evidence.

**Recommendation.** Target: CodeGen and final assurance templates. Keep preservation manifests, per-crate counts and negative monitor cases; state N=0 integration checks, logging extern boundaries, driver verify=false and BRF=0 coverage limitations beside positive results.

**Evidence limit.** These are inspected historical results, not newly rerun tests. Source/configuration manifests exclude build caches and symlinks. No whole-system temporal proof, full driver host suite or numerical branch coverage is established.
