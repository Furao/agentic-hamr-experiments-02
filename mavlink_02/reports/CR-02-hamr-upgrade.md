# CR-02 HAMR upgrade — generated R2U2 fix

Date: 2026-09-23. Result: generation and comparison complete; post-codegen patching
retired by explicit developer instruction.

## Comparison

The updated generator produces the same R2U2 monitor as the previously patched
version, byte-for-byte. It preserves a false verdict when a later true verdict
arrives during the same monitor step. This fixes the original reporting defect.

Before regeneration, 1,320 regular files under `hamr/` were snapshotted, excluding
build/target, Git, virtual-environment and cache directories and symlinks. Fresh
output was captured before the then-required patch attempt. There were no added,
removed or changed files. All 52 editable resources in the generator's 146-resource
report were included and unchanged, including application code and the legacy
schedule. No model elements were removed; no orphan cleanup was needed.

The patch helper returned `R2U2 workaround 4bc9a9a already applied.` and made no
changes. The comparison remained identical after the probe recompiled `spec.bin`.
Monitor SHA-256 before generation, immediately afterward and after validation:
`fcf457dc0b03f49084b387eab70977b992bc812703594cfcb634533d62a4d17b`.

## Validation and provenance

- Type-check: exit 0, `Well-formed!`.
- Codegen: exit 0; report `Success`, no warnings/errors; output in project
  `hamr/microkit/`, instance artifact in project `.slang/`.
- Isolated reporting probe: exit 0, 2/2 tests pass with unchanged generated source.
  Timely Recovery at D1/D2 produces no timeout; late/absent Recovery produces the
  D2 diagnostic. Delayed trigger, no trigger, repeated status, already-Recovery and
  reboot cases remain covered.
- Sireum checkout: `85c9cdfe04227ec2d3064b13c231943648c8431b`.
- HAMR codegen checkout: `5801330b0987f954f13e2188f523e24261ce2ba4`.
- Context: `/home/robertvanvossen/tools/r2u2-HAMR-agent-context`,
  `cf109a5e50b9208758c9f70dc5564d46a33537fc`.

Commands from `sysmlv2/open_platform`:

```sh
/home/robertvanvossen/tools/Sireum/bin/sireum hamr sysml tipe --sourcepath ../aadl-lib:. Platform.sysml
/home/robertvanvossen/tools/Sireum/bin/sireum hamr sysml codegen --sourcepath ../aadl-lib:. --platform Microkit --output-dir ../../hamr --workspace-root-dir ../.. --system-name Platform::ZCU102_Impl Platform.sysml
```

Probe command from project root:

```sh
R2U2_CLI=/tmp/cr02-r2u2-tools/bin/r2u2_cli python3 tests/r2u2_monitor_probe/run.py
```

The first sandboxed type-check could not write the updated Sireum launcher's AOT
log; the authorized rerun completed successfully. No model changes were needed.
Existing build scripts were retained; CodeGen.4 is not applicable to regeneration.
No full loader build or hardware retest was performed for this identical output.

Evidence: [comparison manifest](CR-02-hamr-upgrade-comparison.json),
[type-check](CR-02-hamr-upgrade-tipe.txt), [codegen](CR-02-hamr-upgrade-codegen.txt),
[last patch check](CR-02-hamr-upgrade-patch.txt),
[probe results](CR-02-hamr-upgrade-r2u2-probe.txt).

## Workflow disposition

The developer instructed removal of post-codegen workaround patching after the
successful comparison. AGENTS.md, the change plan and active probe/workaround
documentation now require validation of fresh output without invoking the patch
helper. The captured patch and helper remain historical artifacts. This direction
supersedes the earlier instruction to attempt the patch after every codegen run,
including failed runs.

The separate hardware timeout finding **CR-02-HW-01 remains High/Open**. Matching
the already-patched monitor does not resolve the observed Recovery deadline issue
or constitute overall Wave 3/final change approval.
