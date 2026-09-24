# CR-02 bounds correction — full loader build

Date: 2026-09-23. Developer approved verification and affected component completion.
Status: build PASS; requested bounds correction implemented, tested and verified.
Wave 3 physical hardware and timing acceptance remains incomplete.

## Build result

From project root:

```
SYSTEM_MAKEFILE=custom.mk make -C hamr/microkit \
 MICROKIT_SDK=/home/robertvanvossen/tools/microkit-sdk-2.2.0-dev \
 MICROKIT_BOARD=zcu102 MICROKIT_CONFIG=debug \
 R2U2_CLI=/tmp/cr02-r2u2-tools/bin/r2u2_cli
```

Exit 0. Full log: CR-02-bounds-build.txt. The custom build compiled and linked the
updated driver, Rx/Tx shared parser and MAVLink application, preserved the VMM archive
integration, and successfully ran the Microkit loader tool. Release verification
reported ModeManager 9/0, Rx 28/0, Tx 16/0 and MAVLink 69/0; the changed firewall_core
dependency also verified 39/0. The driver has verify=false: its successful target
build is not a driver proof. No new build workaround or source change was needed.

Artifact: hamr/microkit/build/loader.img, **155953132 bytes (148.73 MiB)**.
SHA-256: `f524c83c893a10b0f20a638dc58a5d0b9784b9c0aaa4ee8f64ef3cbd5419cadf`.
This supersedes the earlier loader ending in 06ad0, which predates the bounds fix.
The equal image sizes do not imply equal contents; hashes differ.

SDK 2.2.0-dev, ZCU102/debug; Rust 1.97.1, Verus 0.2026.08.09.92f466f;
R2U2 compiler/runtime 4.2.4. Existing deprecation/unused-code warnings remain.
No compiler installation was required. No HAMR codegen ran during this build.

## Preservation and validation

SHA-256 comparisons confirm custom.mk, the legacy microkit.schedule.xml and the
patched generated R2U2 monitor remained unchanged across the build. XML comparison
confirms all twelve domain entries and lengths are included unchanged in the merged
system used by Microkit; domain_7 (ModeManager) is first among application slots.
Evidence: CR-02-bounds-build-manifest.json. The prior CapDL numbering analysis remains
historical evidence for this unchanged schedule; no new physical timing measurement
or fresh CapDL export is claimed.

The full build recompiles the R2U2 specification using the pinned installed compiler
and retains the authorized false-verdict reporting patch. The current generated
monitor passed its 2/2 probe after CodeGen; the MAVLink production suite passed 16/16,
including D2 logging. Those sources and behavior were not changed by this build.
Approved correction tests total 54 (Tx 8, Rx 12, MAVLink 16, firewall_core 18), plus
2 exhaustive driver-helper tests. See CR-02-bounds-development.md and
CR-02-bounds-verification.md. No redundant host test run followed this build.
Git diff --check passes.

## Remaining Wave 3 work

../deployment/CR-02-hardware-acceptance.md now identifies this exact image and includes explicit
IPv4 boundary/driver-defense cases. All physical cases remain NOT RUN; no board was
flashed. Operator/access, packet/serial captures, D0/D1/D2 traces and legacy schedule
unit/budget measurements remain outstanding. The configured model's 2080 ms is not
claimed as the measured physical schedule duration.

The current requirements authority is Open_Platform_HLRs_26_09_23_01.md. Legacy
requirements reconciliation and the user-directed suppression-log exception remain
developer-owned. No requirement file was edited. These outstanding broader CR-02
acceptance/documentation tasks do not reopen the completed bounds implementation.
