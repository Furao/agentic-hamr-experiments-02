# CR-02 bounds correction — CodeGen review

Date: 2026-09-23. Profile: audited. Contract AP1 approved by developer.
Authoritative requirements: Open_Platform_HLRs_26_09_23_01.md.
Context: /home/robertvanvossen/tools/r2u2-HAMR-agent-context.

## Result

Generation exit 0; codegen_report_sysml.json reports Success with no warnings or
errors. Output is under the project hamr/microkit directory; the instance artifact
is under .slang. Existing configuration and type-clean model were reused.

Command, from sysmlv2/open_platform:

```
/home/robertvanvossen/tools/Sireum/bin/sireum hamr sysml codegen --sourcepath ../aadl-lib:. --platform Microkit --output-dir ../../hamr --workspace-root-dir ../.. --system-name Platform::ZCU102_Impl Platform.sysml
```

Immediately after generation, ran python3 bin/apply-codegen-workarounds.py from
project root (equivalent relative path from model directory). Exit 0: applied
4bc9a9a workaround. No other generated code was manually edited.
Log: CR-02-bounds-codegen.txt. CLI informational messages concern optional provisioning
and the ignored file argument when a system name is supplied.

## Generated changes

- GumboLib executable and spec predicates now use 1600 minus Ethernet header length
  (1586) as the IPv4 maximum.
- Tx GUMBOX now checks the four size integration guarantees in initialize and compute
  postconditions, with guards for absent event-data outputs.
- Tx output API setters require size <=1600. The generated initialization test helper
  retrieves the four outputs and calls the initialization oracle.
- Woven application files needed no textual changes: they already reference the shared
  predicates and generated APIs. Existing code remains for the next CompDev step.

Byte-for-byte comparison of 38 editable files before/after generation found no changes.
These include all five application files, component tests, driver helper/config,
component manifests/toolchains/Makefiles, custom.mk and microkit.schedule.xml.
Legacy domain XML is intact. No model elements were removed, so no orphan cleanup
was needed. Existing build helper retained (CodeGen.4 first-generation-only: n/a).
Git diff --check passes. This is regeneration evidence, not a full build result.

## Required R2U2 regression

```
R2U2_CLI=/tmp/cr02-r2u2-tools/bin/r2u2_cli python3 tests/r2u2_monitor_probe/run.py
```

Exit 0, 2/2 tests pass. The runner recompiles the generated specification with
compiler 4.2.4 and exercises actual generated pre/post hooks and verdict reporting.
Timely Recovery at D1/D2 passes; late/absent Recovery delivers the false verdict at
D2, with one diagnostic per boot in the probe. Delayed trigger, repeated status,
no trigger, already-Recovery and reboot scenarios remain covered.
Evidence: CR-02-bounds-r2u2-probe.txt. Production logger and target timing validation
remain covered by their separate component/hardware acceptance work.

## Handoff

CodeGen exit criteria met; audited composite boundary approval is pending before
CompDev. The shared core MAX_MTU and MAVLink executable maximum still contain 9000;
Tx regression fixtures still expect the former acceptance. These are the explicit
next implementation/test changes, not evidence of conformance to the new model.
Do not use prior W2 proofs or W3 loader as acceptance of this intermediate tree.
After boundary approval, align constants, refresh boundary/oracle tests and coverage,
then follow CompDev AP1 before Verus and AP2 afterward. Full loader rebuild follows.

Developer approved this CodeGen boundary. Implementation/test follow-up is recorded in CR-02-bounds-development.md; the pre-implementation handoff above is historical.
