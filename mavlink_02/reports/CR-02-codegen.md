# CR-02 Wave 1 — Code generation

Date: 2026-09-22. Profile: audited.

## Result

The rerun with the developer-updated HAMR installation completed with exit 0.
`hamr/microkit/reporting/codegen_report_sysml.json` reports `Success`, with empty
warning and error lists. The attestation report was regenerated successfully.
The previous reporting-parser failure on `last_error_status_payload` is resolved.
No agent-authored patch to the external Sireum installation was needed for this run.

Executed from `sysmlv2/open_platform/`:

```text
/home/robertvanvossen/tools/Sireum/bin/sireum hamr sysml codegen --sourcepath ../aadl-lib:. --platform Microkit --output-dir ../../hamr --workspace-root-dir ../.. --system-name Platform::ZCU102_Impl Platform.sysml
```

The updated launcher required permission to write its external `.aot/train.log`;
the first sandboxed launch failed before generation, and the authorized rerun passed.
The CLI also printed informational messages about the unimplemented SysML readme
generator and optional provisioning (`AM_REPOS_ROOT` not set).

## Output and preservation

Output is under this project's `hamr/microkit/`; the instance model is under
`.slang/ZCU102_Impl_Instance.json`. Generated artifacts include ModeManager's crate
and C bridge, OperatingMode, sampled control ports, revised GUMBOX/Verus contracts,
and MAVLinkFirewall's R2U2 specification and pre/post-dispatch hooks.

Comparisons against the pre-generation backups confirmed:

- `custom.mk`, `microkit.schedule.xml` and existing component `tests.rs` files are
  byte-for-byte unchanged.
- Driver application code is unchanged. Tx changes are confined to managed markers.
- Rx and MAVLink application bodies are preserved. Changes comprise regenerated
  contract/state markers, plus the Rx initialization `ensures` section introduced
  while reconciling the generator-requested markers.
- Eight stale `_fixme` files from earlier failed attempts were removed after the
  successful rerun. No removed components or data types require orphan cleanup.

`git diff --check` identifies two generator-emitted trailing spaces, in the Rx and
MAVLink GUMBOX initialization conjunctions. They were left in generated output;
this is a formatting issue, not a clean diff-check result.

## Earlier prerequisite changes

The original frame period of 1950 ms failed HAMR's configured-budget check after
adding ModeManager. `Platform.sysml` now uses the required minimum of 2080 ms:
1900 ms component budgets plus 180 ms pacer budget. Type checking passed after that
change. This advances one model prerequisite from the planned scheduling work; it
does not validate or update the deployed schedule. W3 must reconcile that schedule
with the model and establish mode-propagation timing.

The new generator-requested initialization, monitor-module, R2U2 dependency/build
and ModeManager system markers were reconciled during the earlier attempts.
Existing dependency versions and manual application/custom build content were
preserved. New generated scaffolds still require W2 implementation and build checks.

## R2U2 compilation and remaining W1 obligations

The freshly generated `spec.c2po` was compiled again with `r2u2_cli 4.2.4`, exit 0,
producing `spec.bin` and `.cargo/config.toml` bounds. The compiler was installed
under `/tmp/cr02-r2u2-tools` during the previous attempt; its build required a local
linker search path for the installed versioned Python 3.14 shared library.

```text
/tmp/cr02-r2u2-tools/bin/r2u2_cli compile -o hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall/src/component -b hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall/.cargo/config.toml hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall/src/component/spec.c2po /tmp/cr02-spec.map
```

`/tmp/cr02-spec.map` is the generated `spec.map` with comment lines removed, as in
the generated build rule. The formula remains a past-time D2 check. The generated
hooks sample mode before compute and final ErrorStatus after compute, then invoke
one monitor step. Initialization creates a fresh monitor without stepping it.

The generated reporter still emits routine info-level verdict status; it is not
the required one-time timeout error reporter. Demonstrating a supported,
regeneration-safe same-dispatch reporting path remains an explicit W1 prerequisite
before W2. Startup control values and frozen-snapshot consistency also require the
planned runtime integration checks. Specification compilation alone does not prove
verdict delivery or log timing.

CodeGen's artifact-generation exit criteria are met; the developer approved its
audited boundary on 2026-09-22. The separately planned setup-build-script task follows
(this is regeneration, so CodeGen.4's first-generation-only condition does not apply).
W1 is not complete. No application tests, Verus verification, target build or R2U2
runtime trace tests are claimed by this report.
