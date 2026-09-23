# Open Platform — HAMR project

HAMR context (docs, workflows, examples) lives in the hamr-agent-context repo,
loaded as an additional working directory. Development follows the named
workflows in its process/README.md. Request workflows by name, use `/wf` and
per-workflow `/...` skills in Claude Code, or use `$wf` and per-workflow `$...`
skills in Codex. Progress: reports/workflow-status.md. Model source:
sysmlv2/open_platform/; generated code will land in hamr/microkit/.

## Required post-codegen workaround

After every HAMR codegen invocation (including failed runs that may have rewritten
files), attempt `python3 bin/apply-codegen-workarounds.py` from the project root.
This is explicitly authorized by the developer and is an exception to the general
rule against editing generated code. It reapplies the exact R2U2 reporting fix from
commit `4bc9a9ae60daefad311bcf23546ee8d4e7468c1b`, captured in
`patches/4bc9a9a-r2u2-false-verdict.patch`. An already-applied patch is a success.
If the patch conflicts, report the failure and inspect generator changes; do not
force it, discard other edits, or claim the workaround is installed.
After successful generation and patch application, run the isolated reporting probe
as documented in `tests/r2u2_monitor_probe/README.md` and record its result.
