# CR-02 Verus toolchain migration

Date: 2026-09-23. Authorized by developer; verifier installed by developer.

## Configuration

Confirmed PATH selects `/home/robertvanvossen/tools/verus-x86-linux/verus` and
the matching `cargo-verus`. Verus reports **0.2026.08.09.92f466f**, Rust **1.97.1**.
Aligned all nine component/shared/core crate manifests to Verus dependencies
`0.0.0-2026-08-09-0044`. All crate toolchain files now pin 1.97.1, including an
explicit pin for mavlink_core. Cargo refreshed affected lockfiles and created
ModeManager's lockfile. Existing unrelated dependency constraints were preserved.

Existing editable component Makefiles now use the generated ModeManager scaffold's
`--rlimit 100 --smt-option smt.random_seed=7`. The default settings failed two vstd
proofs during the first target run; these settings discharge them. Cargo-verus
release arguments now precede target/build-std arguments. Updated the firewall_core
Makefile similarly. No generated contract or business-policy changes were required
for this migration. ModeManager implementation changes belong to the ongoing W2 task.

## Validation

| Target | Tests | Verification / limitation |
|---|---|---|
| ModeManager | 7/7 pass | Awaiting CompDev coverage approval before verification |
| firewall_core | 17/17 pass | Host cargo-verus: 39 verified, 0 errors |
| mavlink_core | 6/6 pass | Host cargo-verus: 38 verified, 0 errors |
| TxFirewall | 4/4 pass | Target make verus: 16 verified, 0 errors |
| RxFirewall | Test compile attempted | Existing generated test macro now expects current_mode; W2 test migration pending |
| MAVLinkFirewall | Test compile attempted | Same missing current_mode test-macro input; further known oracle signature updates remain W2 |
| LowLevelEthernetDriver | Host test compile attempted | seL4 dependency needs SEL4_INCLUDE_DIRS or SEL4_PREFIX; full target environment remains W3 |

Core verification command: `RUSTC_BOOTSTRAP=1 cargo-verus verify --offline -- --rlimit 100 --smt-option smt.random_seed=7`.
Tests use `RUSTC_BOOTSTRAP=1 cargo test`; ModeManager also uses coverage instrumentation.
Tx target verification uses `make verus`; a host-only attempt first hit the no_std
panic-unwinding limitation, resolved by using the existing target recipe.
Target sysroot dependencies required an escalated network fetch. vstd verifies
1861 obligations under the recorded solver settings. Shared data verifies 10;
GumboLib has 0 obligations, which is not an independent policy proof.

Evidence is in `CR-02-toolchain-*-verification.txt` and the ModeManager development
report. No full-system build, Rx/MAV implementation success or hardware assurance
is claimed. The remaining failures are at known scaffold/test/target-environment
boundaries, not unresolved Verus dependency selection or syntax errors.

## Follow-up

Resume CompDev(ModeManager) at AP1, then verify the implementation. Continue the
approved W2 component implementations and test migrations, with W3 full target
build/verification. The R2U2 compiler/runtime remains 4.2.4 and the authorized
post-codegen workaround remains required. This migration did not invoke codegen.
