# CR-02 ModeManager transition diagnostic

Date: 2026-09-23. Developer requested logging only when the mode changes.
CompDev steps 1–5 complete; audited AP1 review pending.

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
under that target's profiles directory, cargo test --offline. grcov reports:

| Source | Covered/executable lines |
|---|---:|
| ModeManager application (including test capture hook) | 39/39 |
| ModeManager GUMBOX | 67/67 |

BRF=0: no numerical branch percentage available. Four state/input cases cover the
transition guard outcomes. Host coverage does not establish physical logging timing.
Evidence: CR-02-mode-log-tests.txt and CR-02-mode-log-coverage.lcov.
Git diff --check passes. Await AP1 approval before Verus; the existing loader does
not yet contain this diagnostic. A new loader follows proof sign-off.
