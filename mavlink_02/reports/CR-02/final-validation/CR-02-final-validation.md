# CR-02 final validation summary

Date: 2026-09-23. Requirements: Open_Platform_HLRs_26_09_23_02.md.
Result: final application/core tests pass; prior full verification/build applies
to unchanged source. Driver host-test and proof limitations remain explicitly
qualified. The hardware timeout is deferred by developer to a future CR.

## Fresh cross-crate tests and entry-point line coverage

| Crate | Tests | initialize | timeTriggered |
|---|---|---|---|
| ModeManager | PASS, 7 | 8/8 | 13/13 |
| RxFirewall | PASS, 12 | 5/5 | 49/49 |
| TxFirewall | PASS, 8 | 5/5 | 48/48 |
| MAVLinkFirewall | PASS, 16 | 7/7 | 66/66 |
| firewall_core | PASS, 18 | n/a | n/a |
| mavlink_core | PASS, 6 | n/a | n/a |
| LowLevelEthernetDriver full host suite | BLOCKED before tests by seL4 build dependency | not measured | not measured |
| Driver bounds helper, isolated host harness | PASS, 2 | n/a | n/a |

The six application/core suites pass **67 tests**, with another **2** tests passing
for the driver helper. No application/core test failed. The helper exercises all
65,536 u16 sizes and continued handling of valid requests after invalid ones; it
does not replace the unavailable full driver host suite.

Every selected crate was attempted sequentially with `cargo test --locked --offline`,
`RUSTC_BOOTSTRAP=1`, `CARGO_INCREMENTAL=0`, `RUSTFLAGS=-Cinstrument-coverage`.
A fresh temporary shared Cargo target and separate initially empty profile
directories per crate prevented stale coverage counts. grcov generated each lcov
report from that crate's profiles. The common binary directory also exposes
unrelated zero-hit source records; coverage summaries select only the named crate's
own source. Entry-point ranges are cross-referenced to application functions.

Measured application/GUMBOX lines: ModeManager 41/41 and 67/67; Rx 118/118 and
382/382; Tx 82/84 and 205/205; MAVLink app including fixtures 253/254 and GUMBOX
382/382. The Tx misses are the previously approved unused trace helper; MAVLink's
miss is the unexpected-fixture panic at line 617. MAVLink logging.rs, including
its tests, is 59/59. All four initialize/timeTriggered entry points have 100% line
coverage. LLVM branch counters are unavailable (BRF=0); these are line coverage
and semantic-case results, not numerical proof of complete branch coverage.

The full driver host test failed in `sel4-config-data` before tests because
`SEL4_INCLUDE_DIRS or SEL4_PREFIX must be set` (upstream build-env/src/lib.rs:55).
This is the established host/platform limitation, not an assertion failure.
The recent driver target check and full image build pass with SDK headers and the
`sel4` feature; its application manifest still disables Verus verification.

Evidence: [test runs and logs](CR-02-final-test-results.json),
[entry-point coverage](CR-02-final-entrypoint-coverage.json),
[driver helper](CR-02-final-driver-bounds-tests.txt). Per-crate lcov files are named
`CR-02-final-<crate>-coverage.lcov` beside this report.

## Verification and build retained for identical source

All 1,347 source/configuration files captured for the immediately preceding full
verification/build were checked again by SHA-256 and remain unchanged. Requirements
_02 only reconciles logging already implemented and tested. No verification or
build rerun is needed for this documentation/acceptance update.

| Crate | Verified | Errors |
|---|---:|---:|
| ModeManager | 9 | 0 |
| RxFirewall | 28 | 0 |
| TxFirewall | 16 | 0 |
| MAVLinkFirewall | 69 | 0 |
| firewall_core | 39 | 0 |
| mavlink_core | 38 | 0 |

Toolchain: Verus 0.2026.08.09.92f466f / Rust 1.97.1. Driver compilation passes with
the deployed feature configuration, but no driver application proof is claimed.
There are no sys_proof crates or separately specified whole-system proof obligations
in this change. The historical integration check is explicitly vacuous (N=0).

Full build: `SYSTEM_MAKEFILE=custom.mk`, ZCU102/debug,
`/home/robertvanvossen/tools/microkit-sdk-2.2.0-dev`, R2U2 CLI 4.2.4. Loader size
155,953,132 bytes (148.73 MiB), SHA-256
`cac588401b5a4853aa4a9d482f5a3743f5840063ee888f52027361893c7aba43`.
The restored legacy schedule matches the merged system. The generated monitor
matches the old patched output and needs no workaround. No codegen or patch ran
during this final review.

Evidence: [full verification/build](CR-02-full-verify-build.md),
[artifact manifest](CR-02-full-build-manifest.json),
[HAMR upgrade and probe](../codegen/CR-02-hamr-upgrade.md).

## Hardware disposition

Manual testing and all three waves are developer-approved. Serial evidence shows
five flash-command denials followed by Recovery and an interleaved D2 timeout;
case-specific captures are not available for every acceptance procedure. No new
hardware run is claimed. The developer explicitly deferred the unresolved timing
issue to a future CR, making it non-blocking for CR-02 final review. The issue
remains High/Open with unchanged evidence and closure criteria in
[CR-02-HW-01](../../../open-issues/CR-02-HW-01-recovery-timeout.md).
