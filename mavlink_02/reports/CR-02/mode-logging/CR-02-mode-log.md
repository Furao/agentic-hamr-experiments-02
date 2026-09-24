# CR-02 ModeManager transition diagnostic

Date: 2026-09-23. Developer requested logging only when the mode changes.
CompDev complete; AP1, AP2 and component completion approved. Loader rebuild passed.

ModeManager now emits the Info message `Mode changed: Normal -> Recovery` only
when its frozen ErrorStatus is true and retained_mode is Normal. It sets Recovery
and logs within that branch; both outputs continue to be published every dispatch.
Repeated true status in Recovery and unchanged Normal/Recovery are silent for this
diagnostic. Recovery is latched until initialization, so at most one transition
message occurs per boot. The existing initialization diagnostic remains unchanged.

The existing external log_info adapter is reused. Test-only capture records calls
inside that adapter; it adds no production state, allocation or new trust escape.
No model/contracts/generated files or requirements were changed, so no codegen is
needed. Hardware behavior was confirmed by the developer before this edit; that
statement has been recorded without inventing case-level or timing measurements.

All 7 ModeManager tests pass. Existing tests now assert exact transition messages
for all four state/input combinations and exactly one message across repeated
error/empty-status dispatches after latching Recovery. Existing publication,
initialization/reboot, notification, oracle and generated PropTests still pass.

Fresh isolated target target/cr02-mode-log, RUSTC_BOOTSTRAP=1,
CARGO_INCREMENTAL=0, RUSTFLAGS=-Cinstrument-coverage, absolute LLVM_PROFILE_FILE
under that target's verified-profiles directory for the final iteration, cargo test --offline. grcov reports:

| Source | Covered/executable lines |
|---|---:|
| ModeManager application (including test capture hook) | 41/41 |
| ModeManager GUMBOX | 67/67 |

BRF=0: no numerical branch percentage available. Four state/input cases cover the
transition guard outcomes. Host coverage does not establish physical logging timing.
Evidence: CR-02-mode-log-tests.txt and CR-02-mode-log-coverage.lcov.
Git diff --check passes. The approved diagnostic is included in the rebuilt loader
identified below.

## Verification and final iteration

Developer approved AP1. The initial target verification returned 8 verified/1 error:
the transition postcondition did not discharge when the new branch used executable
PartialEq on OperatingMode. The final implementation uses `if error_status` followed
by `if let OperatingMode::Normal = self.retained_mode`, matching the enum pattern
style already used in verified Rx code. It preserves exactly the tested transition
and once-per-transition diagnostic behavior. No generated contract, model, requirement,
proof assumption or external-body boundary was changed.

Fresh verification: **9 verified, 0 errors**, exit 0. Command from ModeManager crate:
`SYSTEM_MAKEFILE=custom.mk make verus`; AArch64 target, Rust 1.97.1,
Verus 0.2026.08.09.92f466f, rlimit 100, SMT seed 7. Evidence:
CR-02-mode-log-verification.txt; initial failure preserved separately in
CR-02-mode-log-verification-initial.txt.

After the equivalent branch rewrite, reran all tests (7/7) and coverage with a fresh
verified-profiles directory: app 41/41, GUMBOX 67/67, BRF=0. Exact logging checks and
all four state/input combinations pass. These refreshed results supersede the
initial app 39/39 coverage. The two existing external bodies remain log_info and
log_warn_channel; log delivery itself remains outside the state-machine proof and
is exercised through the existing test capture. No new trust escape was introduced.

Developer approved AP2 and component completion; the new loader build is complete.
The user's prior hardware confirmation applies to the pre-diagnostic build.

## Approved loader rebuild

Full custom ZCU102/debug build passed, exit 0. Command from project root:

```
SYSTEM_MAKEFILE=custom.mk make -C hamr/microkit \
 MICROKIT_SDK=/home/robertvanvossen/tools/microkit-sdk-2.2.0-dev \
 MICROKIT_BOARD=zcu102 MICROKIT_CONFIG=debug \
 R2U2_CLI=/tmp/cr02-r2u2-tools/bin/r2u2_cli
```

ModeManager rebuilt and linked into the loader, with release verification 9/0.
The build also recompiles the R2U2 specification and verifies MAVLink 69/0.
No source changes, new codegen invocation or new workaround were needed.
SHA-256 checks confirm custom.mk, the legacy schedule and patched R2U2 monitor
were preserved. XML comparison confirms the merged system retains the exact
schedule entries. The exact diagnostic string is present in both the linked
ModeManager ELF and the final loader. These are artifact checks, not a new
hardware observation.

Loader: hamr/microkit/build/loader.img, 155953132 bytes (148.73 MiB).
SHA-256: `cac588401b5a4853aa4a9d482f5a3743f5840063ee888f52027361893c7aba43`.
Evidence: CR-02-mode-log-build.txt and CR-02-mode-log-build-manifest.json.
The approved tests and coverage remain applicable; no source changed during build.

The user's prior hardware confirmation applies to the preceding bounds-fix image.
The diagnostic is ready for hardware use in this image; no board was flashed by
the agent. Broader CR-02 physical timing and documentation evidence remain tracked
separately and are not claimed complete by this logging change.
