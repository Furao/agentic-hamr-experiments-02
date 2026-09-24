# Project reports

Start with the completed change report or workflow status; use the topic indexes
for detailed reviews, test logs, coverage and build evidence.

| Area | Contents |
|---|---|
| [Workflow status](workflow-status.md) | Project-wide workflow and approval record; kept at this stable location |
| [CR-02 — Add ModeManager](CR-02/README.md) | Completed change, final validation and evidence grouped by topic |
| [CR-01 — MAVLink firewall](CR-01/README.md) | Completed change, hardware acceptance and original contract audits |
| [Maintenance](maintenance/README.md) | Parser consolidation and specification-refactoring evidence |
| [Open issues](../open-issues/README.md) | Canonical unresolved findings, separate from completed change reports |

## Organization

Keep future change reports under `reports/CR-<NN>/`, with the main change report
and a README index at that level. Group supporting reports and their evidence by
topic; keep related logs/manifests beside the report they substantiate.
`reports/workflow-status.md` remains the shared workflow entry point.

Existing filenames and stable change/issue IDs are retained. Narrative references
and manifest log/coverage paths were updated for this reorganization. Raw logs,
coverage, diff files and proof snippets were moved without changing their bytes.
Historical transcripts and raw command output retain original paths; use the
[old-to-new location map](report-locations.json) to find relocated artifacts.
