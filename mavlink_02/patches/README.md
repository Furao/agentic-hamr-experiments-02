# Archived codegen workaround

Retired from the workflow by developer instruction on 2026-09-23. Updated HAMR
generates the fix directly: fresh output matches the formerly patched output
byte-for-byte and the reporting probe passes 2/2. See
`reports/CR-02/codegen/CR-02-hamr-upgrade.md`. Do not run the patch helper after codegen.

`4bc9a9a-r2u2-false-verdict.patch` captures the exact file diff from commit
`4bc9a9ae60daefad311bcf23546ee8d4e7468c1b` with project-relative paths.
It preserves visibility of a false verdict in the current monitor step even when
a later true verdict replaces the cached value. It modifies generated logging;
it does not provide the production one-time timeout reporter or fix alert routing.

The patch and `bin/apply-codegen-workarounds.py` are retained as historical
artifacts. They are no longer part of generation or validation. The captured fix
has a one-element per-specification array.

Continue validating fresh generated output with the isolated generated-monitor
probe in `tests/r2u2_monitor_probe/README.md`.
