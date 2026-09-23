# CR-02 W1 — R2U2 reporting feasibility

Date: 2026-09-22. Status: blocked by generated verdict loss; W1 not approved/complete.

## Finding

The generated specification compiles, and R2U2 produces the HLR-30 failure during
D2. HAMR's generated reporting loop overwrites that failure with a later true
verdict from the same output buffer, then exposes only the final cached status.
Consequently the current generated status-log interface cannot support the required
same-D2 timeout diagnostic, even with a one-time reporting adapter.

For a first true ErrorStatus at absolute dispatch 0 and Recovery first at dispatch 3:

| Dispatch executing | Raw verdict time | Raw verdict truth |
|---|---:|---|
| D0 | 1 | true |
| D2 | 2 | false |
| D2 | 5 | true |

These are the actual runtime's timestamps, including its compressed future range
endpoints; they are not additional application dispatches. The false verdict is
available during D2, but `verdict_cache[out.spec_num] = Some(out.verdict)` stores
the subsequent true verdict in the same slot. The generated log consequently says
`hlr_30_llr_15_16_recovery_deadline is currently true` during D2. Delaying the first
assertion to absolute dispatch 1 or 4 reproduces the loss at dispatch 3 or 6.

## Reproducer and evidence

`tests/r2u2_monitor_probe/` compiles the current specification and includes the
generated monitor source unchanged. It supplies application/API shells to avoid
mixing this generator/runtime check with the unfinished W2 firewall implementation.
The real R2U2 4.2.4 runtime executes. Runtime configuration is compiled from the
generated spec/map; dependencies are locked. The compiler is r2u2_cli 4.2.4, and
the log facade version is 0.4.28.

```text
R2U2_CLI=/tmp/cr02-r2u2-tools/bin/r2u2_cli python3 tests/r2u2_monitor_probe/run.py
```

Result: exit 101; **1 passed, 1 failed**. Complete output is in
`reports/CR-02-r2u2-probe-output.txt`.

- Raw runtime check passes: first assertions at 0, 1 and 4 each yield exactly one
  false verdict at first-assertion dispatch + 2.
- Generated reporting check fails for late, absent and post-reboot missing Recovery,
  including a delayed first assertion. No timeout Error reaches the sink at D2.
- Timely D1/D2, no assertion and already-Recovery cases produce no timeout Errors.
- The candidate adapter itself ignores unrelated/true messages and suppresses
  repeated false-message diagnostics until reset, but cannot recover lost verdicts.

The probe's logger converts an exposed false record to an Error at the same call
site, recording dispatch and severity. This demonstrates the downstream adapter
shape only, not a working end-to-end path with the current generator. It is not
installed in the application. No deadline counter, weakened requirement, shifted
D3 deadline or manual generated-source patch was introduced.

## Required generator change

The relevant source is `GumboR2U2Util.processRustOutputs` in
`hamr/codegen/shared/src/main/scala/org/sireum/hamr/codegen/microkit/plugins/gumbo/`.

Expose **every raw verdict before it can be overwritten in the status cache**.
Prefer a developer-owned per-verdict callback carrying specification identity,
verdict timestamp and truth, invoked synchronously in the post-dispatch hook.
Provide a boot/reset integration point for the reporter. The existing latest-status
cache can remain for status presentation, but must not be the sole violation path.
Alert mappings using the same cache also need review for this loss mechanism.

After the generator change, regenerate and rerun the reproducer. Require a false
verdict delivery and one Error during D2, no Error for timely Recovery, no reset
of the first obligation on repeated assertions, and reset on reboot. W2 can then
integrate and verify the reporter in the application. Do not mark W1 complete based
on successful code generation or specification compilation alone.

## Startup and sampling inspection

- Generated C mode enum assigns Normal zero; retained C sampled-mode globals are
  zero-initialized, providing Normal before a new publication.
- ModeManager's retained C ErrorStatus starts false; generated Rust OperatingMode
  defaults to Normal. Generated application state initializes mode/count to Normal/0.
- Initialization constructs the monitor without stepping it. Each generated dispatch
  calls the monitor pre-hook, application, then monitor post-hook once, including
  empty-input dispatches.
- The pre-hook peeks mode; the post-hook peeks final ErrorStatus. The generated C
  peek/get access the sampled queues. W2 must use one dispatch mode snapshot, and
  W3 must validate peek/get consistency under the deployed domain schedule and
  input-freeze/publication ordering. This host probe supplies fixed snapshot values
  and does not validate those C queues or target scheduling.
- ModeManager's generated initialization body is still a scaffold; explicit startup
  publications remain a W2 implementation obligation. Source defaults alone are not
  evidence that those application calls have been implemented or tested.

## Build helper

Created executable `hamr/microkit/bin/build.cmd` using the setup-build-script
Microkit/Rust template. It lists five component crates and the two shared cores,
excluding data/GumboLib. Added the generated Makefile's RUSTC_BOOTSTRAP host-test
environment and nonzero exit on test-command failure. Usage invocation parsed and
exited 0 with all seven crate names. Existing application tests were not run during
this scaffold step. Monitor dependencies/spec.bin must be prepared before invoking
the template's direct Cargo test commands; generated Makefile rules provide that
preparation. Full build/verification is still W2/W3 work.
