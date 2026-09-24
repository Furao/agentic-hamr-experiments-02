# Category 3 — Documentation and tool-use guidance

CR-02 only: requirements review through final approval. Original CR-01 development and post-closeout reporting work are excluded. Each finding states its relationship to this change.

Severity: 🔴 blocker · 🟠 moderate · 🟡 minor · 🟢 positive. Historical severity and current disposition are separate.

---

## DOC-01 — Initial requirements advice did not apply HAMR frozen-input semantics 🟠

**Scope.** `shared-workflow`. **Severity.** moderate. **Disposition.** corrected during requirements review.

**CR-02 relationship.** Advice error and user correction during CR-02 requirements review.

**Evidence.** [experiment-reports/session-transcript.md:300–356](../../experiment-reports/session-transcript.md); [action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md: LLR-10–16](../../action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md); [sysmlv2/open_platform/open_platform_Software.sysml:324–326](../../sysmlv2/open_platform/open_platform_Software.sysml).

**Impact.** The user had to correct advice framed around queued arrivals. That advice introduced an unnecessary consume-versus-retain ambiguity before the review was restated around the dispatch snapshot.

**Root cause.** The assistant’s initial explanation mixed arrival order with dispatch-visible input state; the transcript explicitly acknowledges the error.

**Recommendation.** Target: Requirements review and HAMR sampling guidance. Begin control-loop reviews with a D0/D1/D2 table covering input freezing, application execution, publication and monitor hooks. Derive deadline wording from the current authoritative requirements rather than reusing an earlier suggestion.

**Evidence limit.** The advice was corrected in the session. This is a demonstrated guidance-application gap, not evidence that the final implementation uses asynchronous inputs or that current HAMR documentation lacks the explanation.

---

## DOC-02 — CR-02 closeout reconciled logging requirements and retired conflicting authorities 🟢

**Scope.** `shared-workflow`. **Severity.** positive. **Disposition.** observed positive.

**CR-02 relationship.** Requirements authority and logging reconciliation completed before CR-02 approval.

**Evidence.** [reports/CR-02/CR-02-closeout-requirements-review.md](../../reports/CR-02/CR-02-closeout-requirements-review.md); [action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md: LLR-18](../../action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md); [requirements/updated_reqs.md: retirement banner only](../../requirements/updated_reqs.md); [requirements/component-requirements.md: retirement banner only](../../requirements/component-requirements.md); [experiment-reports/session-transcript.md:10112–10225](../../experiment-reports/session-transcript.md).

**Impact.** The developer-owned final LLR-18 matches transition-only logging and retained rejection diagnostics. Explicit retirement notices prevent older HLR identifiers and obsolete Tx/driver exclusions from conflicting with the accepted CR-02 scope.

**Root cause.** Closeout kept requirements ownership with the developer, who supplied revision _02 and explicitly authorized retirement of four legacy documents.

**Recommendation.** Target: ChangeExec requirements reconciliation and authority tracking. Retain a named authoritative revision, explicit developer disposition of logging changes and dated retirement/supersession notices for conflicting historical requirements.

**Evidence limit.** Only CR-02 reconciliation and added retirement notices are credited. The bodies of the CR-01 documents are inherited, and the later report-subfolder reorganization is excluded.
