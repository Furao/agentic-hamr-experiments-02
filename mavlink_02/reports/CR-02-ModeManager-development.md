# CR-02 Wave 2 — ModeManager development

Date: 2026-09-23. Profile: audited. Wave 1 explicitly approved by developer.
Status: toolchain blocker resolved; 7/7 tests pass; coverage review AP1 pending.

## Implementation

Initialization explicitly resets retained mode to Normal and publishes both sampled
outputs. Compute reads ErrorStatus once, sets Recovery when true, otherwise retains
the current mode, and publishes that resulting mode to both firewalls on every
dispatch. No GUMBOX calls or new external-body escapes implement the behavior.
Generated contracts and model remain unchanged. Fresh model type-check: Well-formed!,
exit 0. No regeneration needed; installed reporting workaround remains intact.

## Tests authored

- Initialization and reboot restore Normal on state and both outputs.
- All four prior-mode/ErrorStatus combinations assert independently specified
  results. Test output slots are cleared before compute to require fresh publication.
- A multi-dispatch true/false sequence preserves latched Recovery; notification
  preserves state and outputs.
- Exhaustive direct GUMBOX checks enumerate 8 initialization and 32 compute
  state/output combinations, accepting only the required values.
- Existing generated initialization, compute and pre-state-aware PropTests retained.

## Build blocker

Attempted coverage-instrumented Cargo tests. The newly generated ModeManager manifest
pins Verus dependencies to `0.0.0-2026-08-09-0044` and Rust 1.97.1. Shared data and
GumboLib still pin `0.0.0-2026-01-25-0057`. Cargo cannot resolve these conflicting
verus_builtin versions in the same dependency graph. Initial toolchain/cache access
required escalation; Rust 1.97.1 installation and dependency fetching proceeded.

A diagnostic attempt aligning only ModeManager with the existing component baseline
resolved dependency selection but failed parsing `final(self)` in generated API
contracts and woven application ensures. The older Verus macros do not support the
new generator syntax. That attempted manifest/toolchain alignment was reverted;
original project pins remain intact. Generated specifications were not rewritten.

Installed `verus --version` reports `0.2026.01.23.1650a05`, toolchain 1.92.0.
Thus dependency changes alone cannot establish a working verification environment.
A coordinated migration to a matching newer verifier, shared/component/core Verus
dependencies and Rust toolchains, or compatible HAMR regeneration, is required.
This affects the project beyond ModeManager and must retain the planned shared-core
and unaffected-component regression checks.

No unit/property tests executed successfully; no coverage or verification success
is claimed. Coverage review AP1 and verification AP2 have not been reached.
The diagnostic instrumented build also emitted profile-write warnings for external
dependency build locations; a resumed coverage run should use an absolute writable
LLVM_PROFILE_FILE path under this component's target directory.

## Next action

Resolve the project toolchain direction, run ModeManager tests and coverage, then
present CompDev.AP1. Verus verification follows that audited gate. RxFirewall and
MAVLinkFirewall implementation work has not started.

## Migration resolution and coverage review

The developer installed Verus 0.2026.08.09.92f466f and authorized project migration.
All crate Verus pins now match 0.0.0-2026-08-09-0044 with Rust 1.97.1.
ModeManager's tests pass: **7 passed, 0 failed** (three generated PropTests, each
configured for 100 cases, plus four manual tests). Evidence: CR-02-ModeManager-tests.txt.

Coverage was collected with CARGO_INCREMENTAL=0, RUSTFLAGS=-Cinstrument-coverage
and an absolute LLVM_PROFILE_FILE under target/coverage-new, avoiding the earlier
external-directory profile warnings. grcov lcov and HTML generation passed.
Application: **37/37 executable lines**. GUMBOX: **67/67 executable lines**.
Filtered evidence: CR-02-ModeManager-coverage.lcov; HTML is under the component's
`target/coverage-new/report/index.html`.

The LLVM report has no branch counters (BRF=0), so no measured branch percentage
is claimed. Semantic branch coverage is established by the exhaustive test matrix:
all four prior-mode/input combinations, all eight initialization state/output
combinations and all 32 compute state/output combinations. Both outcomes of the
application's only compute conditional execute. Oracle clauses and conjunctions
are exercised with correct and incorrect state and each output. Initialization,
notification and reboot paths also execute. No application/contract line gap remains.

CompDev steps 1–5 are complete. AP1 awaits review of this evidence and the explicit
branch-metric limitation. No ModeManager Verus success is claimed; verification
follows coverage approval. Shared-core and Tx verification performed for toolchain
migration are separately recorded in CR-02-toolchain-migration.md.
