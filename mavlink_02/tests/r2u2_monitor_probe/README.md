# CR-02 generated-monitor reporting probe

This is W1 feasibility evidence, not a test of the firewall application or target
schedule. `build.rs` copies HAMR's `r2u2_monitor.rs` and `spec.bin` unchanged into
the build directory. The harness supplies minimal application/API/type shells;
R2U2 4.2.4 and HAMR's actual pre/post hooks, verdict cache and logging execute.
The probe has no Ethernet ports: samples represent dispatches independently of
Ethernet input availability.

Run from the project root with r2u2_cli 4.2.4 available:

```sh
R2U2_CLI=/path/to/r2u2_cli python3 tests/r2u2_monitor_probe/run.py
```

The runner recompiles the current generated specification, refreshes bounds and
runs locked, offline Cargo tests from this directory (so Cargo reads those bounds).
Run `cargo +stable fetch --locked` here first if dependencies are not cached.
The lockfile pins the runtime and logging dependencies. Compilation replaces the
generated component's spec.bin with its current compiled specification.

Current expected outcome: **two tests pass with fresh HAMR output**, without any
post-codegen patch. The 2026-09-23 upgrade was checked against the previously patched
output; see `reports/CR-02/codegen/CR-02-hamr-upgrade.md`.

Historical generator outcome before the fix: **one test passed and one failed**.
The raw runtime test confirmed a false verdict at D2, but HAMR's last-verdict cache
hid it behind a later true verdict in the same step. The generated reporting test
requires that failure to reach a one-time Error diagnostic during D2. Preserve
this regression assertion.

`src/reporter.rs` is a candidate allocation-free adapter for the current generated
log interface, kept outside production code. It recognizes the named monitor's
false record and requests one Error emission per boot. The harness sink records
that Error synchronously. It cannot recover a verdict the generator never exposes.
This is not an accepted production solution: a typed per-verdict callback would
avoid coupling to message text and log-level configuration.

All generated-monitor scenarios execute in one serialized test because HAMR uses
a global mutable monitor. The raw-runtime test owns a separate monitor. Tests cover
Recovery at D1/D2/D3, never arriving, repeated true status, delayed first assertion,
never asserted status, already-Recovery and reboot reset. The raw test additionally
checks first assertions at absolute dispatches 1, 2 and 4. Initializing does not
step the monitor. No generated source is patched to make this probe pass.

The revised future-time formula also samples pre-count and the threshold. The shell supplies threshold 5, matching the generated helper. Count starts at zero, reaches four on the first dispatch, then five at the trigger. The raw false verdict is timestamped D0 but delivered during D2.

The developer retired the `4bc9a9a` post-codegen workaround on 2026-09-23.
Run this probe directly after successful generation; report regressions without
automatically applying the archived patch. The candidate reporter remains outside
production.
