# CR-02 Wave 3 — custom build integration

Date: 2026-09-23. Status: historical build passed; Wave 3 acceptance incomplete.
The subsequent 26_09_23_01 requirements/driver amendment requires a fresh loader;
see ../bounds-correction/CR-02-bounds-contract-audit.md. This image predates that correction.
Developer approved the legacy schedule review and continuation to this integration.

## Changes

custom.mk now includes ModeManager application/monitor images, C compilation and
Rust linking, test/verify/clean entries, and OperatingMode/bool queue objects. Manual
VMM archive construction and linking are preserved. Recursive calls use $(MAKE).
The loader depends explicitly on the retained microkit.schedule.xml.

The custom build calls bin/compile-r2u2.py using R2U2_CLI, requires CLI version 4.2.4,
strips generated map comments in a temporary file and compiles spec.c2po to spec.bin
and .cargo/config.toml. The subsequent component make overrides R2U2_BUILD_DEPS to
avoid its unconditional cargo install. Compilation still occurs before build/test/
verify through the explicit custom-build prerequisite. Missing/wrong compiler fails
the build. No compiler installation is performed by this path. The standalone
component Makefile retains HAMR's original installation behavior.

No HAMR codegen was invoked. Generated monitor source and the authorized workaround
are unchanged; compiler-produced spec/bounds have no content diff. The checked-in
build/Makefile changed automatically when the root build copied custom.mk.

## Build evidence

Command from project root:

```
SYSTEM_MAKEFILE=custom.mk make -C hamr/microkit \
  MICROKIT_SDK=/home/robertvanvossen/tools/microkit-sdk-2.2.0-dev \
  MICROKIT_BOARD=zcu102 MICROKIT_CONFIG=debug \
  R2U2_CLI=/tmp/cr02-r2u2-tools/bin/r2u2_cli
```

Exit 0. Missing Rust dependency cache writes initially required an approved escalated
run. The compiler was already installed at the recorded temporary location; future
builds must retain it or supply another installed 4.2.4 executable via R2U2_CLI.

Loader: hamr/microkit/build/loader.img, **155953132 bytes (148.73 MiB)**.
SHA-256: `8de75f30c22239f03a721e7dbceb2a2b55714d9070bb64bda3ae024ed7d06ad0`.
Full log: CR-02-w3-build.txt. SDK version 2.2.0-dev, ZCU102/debug; Rust 1.97.1,
Verus 0.2026.08.09.92f466f, rlimit 100 / SMT seed 7. R2U2 compiler/runtime 4.2.4.

The release builds verify ModeManager 9/0, Rx 28/0, Tx 16/0 and MAVLink 69/0.
The driver compiles and links for the real target. Its manifest has verify=false;
that successful build is **not** a driver application proof. The VMM archive build
completed and its existing manual integration remains linked.

Fresh post-build host tests pass: ModeManager 7, Rx 12, Tx 7, MAVLink 15,
firewall_core 17 and mavlink_core 6. Driver host tests fail before tests execute:
without configuration they need SEL4_INCLUDE_DIRS; with the target SDK includes,
AArch64 register names are invalid for the x86 host compiler. Both results are
recorded; target compilation does not replace physical driver tests. The host
MAVLink suite includes the real monitor and production reporter after recompilation.

## Schedule and memory inspection

An additional loader-tool run exported CapDL to inspect the actual emitted schedule.
The legacy SDK assigns kernel domain numbers in first-appearance order. Source
names domain_1,domain_7,domain_2,... map to kernel IDs 0,1,2,... respectively.
Assertions confirm each application and its parent monitor have the expected same
domain, with ModeManager first among applications. Compact evidence:
CR-02-w3-schedule-capdl.json. All twelve lengths survive unchanged; start index is 0.

SDK sel4_client.h documents ScheduleConfigure duration as timer ticks for MCS,
and this kernel enables MCS. CapDL retains time values 3000,10000,60000,30000,...;
further loader/kernel conversion and timer-frequency validation remain necessary
before assigning physical milliseconds. No 2080-ms actual cycle is claimed.

llvm-size reports MAVLink text/data/bss = 505975/1088/344 bytes and ModeManager =
393458/28/200. The linked R2U2_MONITOR symbol occupies 0x428 (1064) bytes, and the
reporting latch occupies one byte. These are static ELF measurements, not runtime
stack/WCET measurements or an isolated incremental code-size comparison. Each
component's configured stack is retained. Hardware cost/timing remains pending.

## Remaining acceptance work

Hardware procedure: CR-02-hardware-acceptance.md (all cases NOT RUN). Operator/access
was requested; no device execution or permission to flash a board is inferred.
Need actual mode propagation, D2 diagnostic timing, reboot and traffic observations,
plus timing/budget validation. No new wave approval is requested before these results.

An existing Tx safety gap was confirmed by source inspection: a declared IPv4
length above 1586 can produce sz>1600; LowLevelEthernetDriver_app.rs passes sz to the
transmit token and slices amessage[0..size] without an upper-bound check. If that
callback is reached, the slice is out of bounds. Tx regression/proof confirms the
current contract, not safety against such oversized lengths. Developer subsequently authorized including the correction in CR-02 and supplied
26_09_23_01 with a 1586-byte IPv4 limit. The driver guard now passes exhaustive
helper tests and target build; revised contracts await sign-off before regeneration
and firewall implementation/verification. This resolves the scope question, not yet
the full firewall/loader acceptance. Requirements remain developer-owned.

Legacy requirements files still carry CR-01 meanings for HLR-23 onward. The 26_09_23_01
requirements are now authoritative. Developer-owned reconciliation, including the
LLR-18 suppression-log exception, remains outstanding for the final sweep.

## Bounds correction rebuild follow-up

The subsequent correction is implemented, approved and rebuilt successfully. Current loader and preservation evidence: ../bounds-correction/CR-02-bounds-build.md. Earlier bounds scope/contract/proof pending notes above are historical; physical acceptance and requirements reconciliation remain outstanding.
