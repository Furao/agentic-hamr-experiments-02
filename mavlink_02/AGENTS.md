# Open Platform — HAMR project

HAMR context (docs, workflows, examples) lives in the hamr-agent-context repo,
loaded as an additional working directory. Development follows the named
workflows in its process/README.md. Request workflows by name, use `/wf` and
per-workflow `/...` skills in Claude Code, or use `$wf` and per-workflow `$...`
skills in Codex. Progress: reports/workflow-status.md. Model source:
sysmlv2/open_platform/; generated code will land in hamr/microkit/.

Report navigation: `reports/README.md`. Store change reports under
`reports/CR-<NN>/`, with supporting evidence grouped by topic and indexed by README.
Keep the shared workflow record at `reports/workflow-status.md`.

Open findings are tracked in `open-issues/README.md`, with individual issue records
in `open-issues/`.

## Post-codegen R2U2 validation

HAMR now generates the R2U2 false-verdict reporting fix directly. The developer
retired post-codegen workaround patching on 2026-09-23; do not invoke the archived
patch helper as part of codegen, including after failed runs.
After successful generation, run the isolated reporting probe as documented in
`tests/r2u2_monitor_probe/README.md` and record its result. Report regressions
without automatically patching generated code. Upgrade evidence:
`reports/CR-02/codegen/CR-02-hamr-upgrade.md`.
