# CR-02 full verification and build

Date: 2026-09-23. Developer requested one more full verify/build after the HAMR
upgrade and retirement of workaround patching. Completed successfully with the
driver proof limitation below. Source HEAD: `9cf01d9b9cebb31c94265c082a2ea257ca8dbe52`.

## Verification

| Crate | Result | Fresh verification / check |
|---|---|---|
| ModeManager | PASS | 9 verified, 0 errors; AArch64 |
| RxFirewall | PASS | 28 verified, 0 errors; AArch64 |
| TxFirewall | PASS | 16 verified, 0 errors; AArch64 |
| MAVLinkFirewall | PASS | 69 verified, 0 errors; AArch64 |
| firewall_core | PASS | 39 verified, 0 errors; host, also during target build |
| mavlink_core | PASS | 38 verified, 0 errors; host, also during target build |
| LowLevelEthernetDriver | CHECK/BUILD PASS; proof disabled | `verify=false` in its existing manifest; no application proof claimed |

Supporting data and GumboLib crates also ran as dependencies (10/0 and 0/0).
Component counts above exclude dependency counts. Verus version
0.2026.08.09.92f466f, Rust 1.97.1, rlimit 100, SMT seed 7.

Every thread component crate's `make verus` was attempted sequentially. The two
shared libraries used `RUSTC_BOOTSTRAP=1 cargo-verus verify --offline -- --rlimit 100
--smt-option smt.random_seed=7`. Initial core checks reused cached results; touching
only their `src/lib.rs` modification times forced fresh verification with the
counts above. Source contents were unchanged.

The driver's first standalone target failed with missing logging macros because
the target omits `--features sel4`: `info` at application line 88, `debug` at
100/111, and `trace` at 113/116. Its imports are gated on the `sel4` feature.
Retrying with the deployed feature configuration and SDK headers succeeded:

```sh
SYSTEM_MAKEFILE=custom.mk SEL4_INCLUDE_DIRS=/home/robertvanvossen/tools/microkit-sdk-2.2.0-dev/board/zcu102/debug/include make verus 'CARGO_FLAGS=--features sel4 -Z build-std=core,alloc,compiler_builtins -Z build-std-features=compiler-builtins-mem --target aarch64-unknown-none'
```

No code or makefile edits were needed. The default standalone driver target retains
that configuration limitation. The retry and full build check compilation, not
driver application proofs. Existing deprecated array-set warnings in firewall_core
remain; all proof-enabled crate checks report zero errors.

The current generated R2U2 specification was compiled with CLI 4.2.4 before MAVLink
verification. `make verus R2U2_BUILD_DEPS=` then used that specification without
installing tools. Every make invocation inherited `SYSTEM_MAKEFILE=custom.mk`.

Evidence: [verification runs and log paths](CR-02-full-verification-results.json),
[R2U2 compilation](CR-02-full-r2u2-compile.txt). Initial failed/cached runs and
successful retries are retained separately.

## Full target build

From the project root:

```sh
SYSTEM_MAKEFILE=custom.mk make -C hamr/microkit \
 MICROKIT_SDK=/home/robertvanvossen/tools/microkit-sdk-2.2.0-dev \
 MICROKIT_BOARD=zcu102 MICROKIT_CONFIG=debug \
 R2U2_CLI=/tmp/cr02-r2u2-tools/bin/r2u2_cli
```

Exit 0. The normal custom build compiled/linked the target components and VMM,
reran release component verification (9/28/16/69, zero errors), merged the system
configuration and regenerated the loader. This used existing dependency caches;
it was not a clean-from-scratch toolchain/dependency build.

Loader: `hamr/microkit/build/loader.img`, **155,953,132 bytes (148.73 MiB)**.
SHA-256: `cac588401b5a4853aa4a9d482f5a3743f5840063ee888f52027361893c7aba43`.
This matches the prior ModeManager logging image, consistent with the restored
schedule and byte-identical generated monitor.

All 12 legacy schedule entries match the merged loader input, including
domain_7=10000 and domain_6=30000; total 208000 legacy units. No physical timing
conversion is inferred. The transition diagnostic is present in both ModeManager
ELF and loader. All 1,347 compared source/configuration files under `hamr/` and
`sysmlv2/` are byte-identical before and after; build/target/cache directories and
symlinks were excluded. No codegen or workaround patching ran.

Evidence: [build log](CR-02-full-build.txt),
[artifact hashes and preservation checks](CR-02-full-build-manifest.json).

## Disposition

The requested verification/build is complete. The prior R2U2 upgrade probe (2/2)
still applies to unchanged generated output; no new test suite or hardware run was
requested or performed here. Manual testing remains accepted with **High/Open
CR-02-HW-01**. This build does not resolve the hardware timeout finding or grant
overall Wave 3/final change approval.
