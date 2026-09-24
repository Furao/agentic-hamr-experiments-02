# CR-02 execution budget increase

Status: reverted by developer; did not resolve the hardware problem.

Date: 2026-09-23. Developer explicitly requested an additional 200 ms for both
MAVLinkFirewall and ModeManager after the manual log showed five flash denials,
a Normal-to-Recovery transition, and an interleaved D2 timeout.

## Authorized changes

| Setting | Before | After |
|---|---:|---:|
| ModeManager Compute_Execution_Time (both bounds) | 100 ms | 300 ms |
| MAVLinkFirewall Compute_Execution_Time (both bounds) | 300 ms | 500 ms |
| Processor Frame_Period | 2080 ms | 2480 ms |
| Legacy domain_7 length | 10000 | 30000 |
| Legacy domain_6 length | 30000 | 50000 |
| Legacy total length | 208000 | 248000 |

Model edits are in open_platform_Software.sysml and Platform.sysml. Major frame
is 2300 ms component budgets plus six 30 ms pacer budgets. The editable legacy XML
retains its domain elements, order, other slots and existing proportional mapping
of 100 length units per modeled millisecond. No periodic-dispatch attributes,
application logic, GUMBO contracts, D2 formula or requirements files were changed.
The current request supersedes the earlier 100 ms manager preference; developer-owned
requirements reconciliation should reflect it.

SysSchedDef remains draft. The legacy loader/kernel unit conversion and measured
physical budget duration are not established by these edits. SDK documentation
names timer ticks for MCS, but a complete loader/initializer/kernel conversion has
not been validated. These changes implement the requested model-budget increase
and its established legacy schedule scaling, not proof of an extra measured 200 ms.
The timeout root cause and its resolution still require hardware evidence.

## Validation

- Model type-check: exit 0, Well-formed (CR-02-budget-increase-tipe.txt).
- HAMR codegen reused the recorded model-directory invocation, exit 0, Success,
  no report warnings/errors (CR-02-budget-increase-codegen.txt).
- Immediately attempted the mandatory 4bc9a9a reporting workaround: applied cleanly.
  Isolated generated-monitor probe passes 2/2 (CR-02-budget-increase-r2u2-probe.txt).
- All 43 snapshotted editable files were byte-identical before/after regeneration,
  including the newly adjusted legacy schedule and existing application/test code.
  No generated source content delta remained after the authorized patch; existing
  application/contract approvals remain applicable. No new codegen sign-off on
  unchanged code was requested. The requested exact budget change authorized this
  schedule update and its rebuild.
- Full custom ZCU102/debug build exit 0. ModeManager 9/0, Rx 28/0, Tx 16/0,
  MAVLink 69/0 verification printed in the release build. The driver compiled;
  its manifest still uses verify=false. Build log: CR-02-budget-increase-build.txt.
- XML assertions confirm all twelve entries, both changed slots and the 248000 sum;
  the merged system used for the loader matches the editable schedule exactly.
- Git diff --check passes. No source/proof changes or redundant application test
  reruns were needed; existing approved host tests remain applicable to unchanged code.

Build command from project root:

```
SYSTEM_MAKEFILE=custom.mk make -C hamr/microkit \
 MICROKIT_SDK=/home/robertvanvossen/tools/microkit-sdk-2.2.0-dev \
 MICROKIT_BOARD=zcu102 MICROKIT_CONFIG=debug \
 R2U2_CLI=/tmp/cr02-r2u2-tools/bin/r2u2_cli
```

Loader: hamr/microkit/build/loader.img, 155953132 bytes (148.73 MiB).
SHA-256: `b55ce4b99309e469e413e9fa77bf34aee034c8a0df88c5b3467dc30b7c349079`.
Artifact/schedule manifest: CR-02-budget-increase-build-manifest.json.

## Hardware follow-up

Repeat the five-rejection scenario on this image. Expect exactly one ModeManager
Normal-to-Recovery diagnostic and no R2U2 D2 timeout. Capture serial output long
enough to observe subsequent dispatches. The prior timeout remains an open hardware
finding until a new run establishes the outcome; no board execution is claimed here.

## Developer rollback — 2026-09-23

The developer reports that the +200 ms schedule/budget increase did not resolve
the problem and has reverted it. Source inspection confirms ModeManager 100 ms,
MAVLinkFirewall 300 ms, Frame_Period 2080 ms, and legacy domain_7/domain_6 lengths
10000/30000. The D2 timeout remains unresolved. Preserve this failed mitigation as
historical evidence; do not reapply it as an accepted fix. No codegen or rebuild was
performed while recording this update, so no new loader identity is claimed.
