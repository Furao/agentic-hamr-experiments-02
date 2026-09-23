# Open Platform — HAMR project

HAMR context (docs, workflows, examples) lives in the hamr-agent-context repo,
loaded as an additional working directory. Development follows the named
workflows in its process/README.md. Request workflows by name, use `/wf` and
per-workflow `/...` skills in Claude Code, or use `$wf` and per-workflow `$...`
skills in Codex. Progress: reports/workflow-status.md. Model source:
sysmlv2/open_platform/; generated code will land in hamr/microkit/.

## Post-codegen R2U2 validation

HAMR now generates the R2U2 false-verdict reporting fix directly. The developer
retired post-codegen workaround patching on 2026-09-23; do not invoke the archived
patch helper as part of codegen, including after failed runs.
After successful generation, run the isolated reporting probe as documented in
`tests/r2u2_monitor_probe/README.md` and record its result. Report regressions
without automatically patching generated code. Upgrade evidence:
`reports/CR-02-hamr-upgrade.md`.
