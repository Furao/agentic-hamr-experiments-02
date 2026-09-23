# CR-02 bounds correction — verification review

Date: 2026-09-23. Profile: audited. Developer approved CompDev.AP1 coverage review.
Authority: Open_Platform_HLRs_26_09_23_01.md and approved regenerated contracts.

## Results

| Crate | Target | Verified | Errors | Exit |
|---|---|---:|---:|---:|
| firewall_core | host | 39 | 0 | 0 |
| TxFirewall | aarch64-unknown-none | 16 | 0 | 0 |
| RxFirewall | aarch64-unknown-none | 28 | 0 | 0 |
| MAVLinkFirewall | aarch64-unknown-none | 69 | 0 | 0 |

All runs actually executed verification and emitted the counts above. Rx/Tx builds
also verify firewall_core for AArch64 (39/0); MAVLink verifies its unchanged
mavlink_core dependency (38/0). These dependency counts are not added to component
counts. Toolchain: Verus 0.2026.08.09.92f466f, Rust 1.97.1, rlimit 100, SMT seed 7.
Existing deprecated ArrayAdditionalExecFns::set warnings remain in firewall_core.

No verification iteration was required. The shared parser's 1586 bound satisfies
Tx's generated size preconditions on all four output setters. No contracts were
weakened, no new proof hints/assumptions/external bodies added, and no application
or test source was edited during this verification phase. The accepted 54 tests
and recorded coverage remain applicable; no redundant test rerun was performed.

## Commands and evidence

Run sequentially from the respective crate directories:

```
RUSTC_BOOTSTRAP=1 cargo-verus verify --offline -- --rlimit 100 --smt-option smt.random_seed=7
SYSTEM_MAKEFILE=custom.mk make verus
```

The first command is for firewall_core; the second for Rx/Tx. Before MAVLink,
`python3 bin/compile-r2u2.py --cli /tmp/cr02-r2u2-tools/bin/r2u2_cli` from project
root successfully recompiled the current specification with version 4.2.4. It left
spec.bin and the application bounds file byte-identical. Then from the MAVLink crate:

```
SYSTEM_MAKEFILE=custom.mk make verus R2U2_BUILD_DEPS=
```

The override avoids the generated unconditional compiler installation, after the
required specification compilation has succeeded. All make invocations inherited
SYSTEM_MAKEFILE=custom.mk. Logs: CR-02-bounds-{firewall_core,TxFirewall,RxFirewall,
MAVLinkFirewall}-verification.txt. Git diff --check passes. No HAMR codegen ran.

## Trust boundary and remaining acceptance

The three existing external bodies per firewall are logging adapters. The changed
shared parser has no external_body, assume or admit. Rx/MAVLink retain their
previously reviewed empty-output startup preconditions; the platform establishment
of those conditions is inspected rather than proved end to end. The generated
R2U2 runtime and production logging adapter remain outside application proofs,
covered by the prior runtime tests and remaining target acceptance work.

The driver bounds helper retains its exhaustive tests and successful target build;
this is not a Verus proof of the driver. Physical hardware/timing evidence remains
outstanding. The prior full loader predates this correction and must be rebuilt.

CompDev.6 and .7 complete for all three affected firewalls. Await AP2 approval of
these proof results and component completion, then resume W3's full custom loader
build with the legacy domain XML and mandatory codegen workaround policy retained.

Developer approved AP2 and affected component completion. Full loader rebuild passed; current artifact/evidence and remaining physical acceptance are recorded in CR-02-bounds-build.md.
