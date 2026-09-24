# CR-02-only effort estimate

[HTML report](EFFORT-CR-02-2026-09-23.html) · [Markdown report](EFFORT-CR-02-2026-09-23.md)

**UNCALIBRATED MODEL.** The human-with-HAMR counterfactual estimates the retained
CR-02 change at **199.02 hours / $29,853.00** for `experienced_sel4` or
**362.79 hours / $39,906.90** for `new_to_sel4`. These are alternative placeholder
profiles, not a confidence interval or measured savings. They include modeled
technical tool overhead (11 hours before profile multipliers), and exclude
inherited artifacts and generated-code authoring. Debugging iterations, deleted-only
or reverted work, meetings, board operations and assurance reports are not fully
priced. Observed session activity is 5.04 active hours; agent cost is unavailable.

## Scope and evidence

- Approved CR-02 baseline: `043d574970d28ff172f7261ea0392ddd14ae50a8`.
- Final side: current project working tree, with per-file hashes in
  [delta-audit.json](delta-audit.json).
- New ModeManager scaffold reference: first-generation commit
  `b68c70a2974b36db08f39835c0ae6a0dc300ad31`; not a pristine final-model baseline.
- Requirements revision comparison: initial supplied CR-02 `_26_09_22_01` to final
  `_26_09_23_02`. This counts developer/collaborative revisions, not all supplied
  requirements or seven repeated document versions as agent authorship.
- Ownership evidence: generator report, woven markers and
  [final artifact inventory](../../reports/CR-02/final-validation/CR-02-final-artifact-inventory.md).
- Workflow scope/status: only CR-02 rows from
  [workflow status](../../reports/workflow-status.md), including named correction
  and logging passes. No CR-01 step results enter the estimate.

Observed retained delta: **2,259 authored/revised SLOC** and **3,628 generated,
woven or retained-template SLOC**, across **141 changed text files**. Zero SLOC
remains unresolved under the recorded classification rules; this is an
evidence-based classification, not a universal proof of authorship. Generated
replacement sensitivities appear separately and are not added to the headline
human-with-HAMR estimate. Unchanged code is represented only to audit full-file
line partitions and contributes zero modeled hours.

## Machine-readable outputs

- [effort-manifest.json](effort-manifest.json): v1-compatible run identity and limits.
  Its concept quote remains the required historical ConOps §0; the actual CR-02
  concept and comparison references are in [delta-scope-v1.json](config/delta-scope-v1.json).
- [project-measures.json](project-measures.json): delta ownership buckets and
  CR-02-only workflow evidence. For v1 compatibility, excluded unchanged/input
  lines use `library` provenance with artifact type `inherited_baseline`; this does
  not assert they are third-party code.
- [line-allocation.json](line-allocation.json): exact final-file partitions;
  [delta-audit.json](delta-audit.json): new-side and deleted-side line ranges,
  source/config/script hashes and scaffold references.
- [effort-estimate.json](effort-estimate.json): the existing skill estimator's
  results using the local [rate configuration](config/effort-model-v1.json) and
  [counterfactual configuration](config/counterfactual-v1.json).

The local adapter extends measurement to a change request; shared skill scripts,
schemas and configurations remain unchanged. The baseline, rules and input hashes
are retained for review. Deleted lines are recorded but not charged twice for
replacements. Leading/trailing whitespace and identical moved lines are excluded.
Application/test code and non-woven Verus spec/proof functions use separate rates.
Custom Python tooling uses the base build-script SLOC rate without the template
setup override; the template build helper uses the existing fixed setup rate once.

## Reproduce and validate

From the project root:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 experiment-reports/CR-02-effort/test_measurement.py
python3 experiment-reports/CR-02-effort/refresh.py --now 2026-09-23T21:50:00-04:00
```

Validation passed: six focused measurement tests; all four bundled v1 JSON schemas;
exact partitions across all 141 final files; equality of priced/generated/unknown
line selections to the measured CR-02 additions/replacements; excluded baseline
hours zero; no CR-01 workflow keys; ratio suppression; self-contained HTML checks.
The current project snapshot reports outside this directory remain historical
alternative-scope outputs and are not the answer for CR-02-only effort.
