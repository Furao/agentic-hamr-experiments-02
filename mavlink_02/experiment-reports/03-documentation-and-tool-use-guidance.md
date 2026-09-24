# Category 3 — Documentation and tool-use guidance

Assessment of CR-02 in mavlink_02 only. Historical findings and current dispositions are distinguished below.

Severity: 🔴 blocker · 🟠 moderate · 🟡 minor · 🟢 positive.

---

## DOC-01 — Initial requirements advice did not apply HAMR frozen-input semantics 🟠

**Scope.** `shared-workflow`. **Severity.** moderate. **Disposition.** corrected during requirements review.

**Evidence.** [experiment-reports/session-transcript.md:300–356](../experiment-reports/session-transcript.md); [action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md: LLR-10–16](../action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md); [sysmlv2/open_platform/open_platform_Software.sysml:324–326](../sysmlv2/open_platform/open_platform_Software.sysml).

**Impact.** The user had to correct advice framed around queued arrivals. That advice introduced an unnecessary consume-versus-retain ambiguity before the review was restated around the dispatch snapshot.

**Root cause.** The assistant’s initial explanation mixed arrival order with dispatch-visible input state; the transcript explicitly acknowledges the error.

**Recommendation.** Target: Requirements review and HAMR sampling guidance. Begin control-loop reviews with a D0/D1/D2 table covering input freezing, application execution, publication and monitor hooks. Derive deadline wording from the current authoritative requirements rather than reusing an earlier suggestion.

**Evidence limit.** The advice was corrected in the session. This is a demonstrated guidance-application gap, not evidence that the final implementation uses asynchronous inputs or that current HAMR documentation lacks the explanation.

---

## DOC-02 — Requirements retirement and report relocation preserve authority and provenance 🟢

**Scope.** `shared-workflow`. **Severity.** positive. **Disposition.** observed positive.

**Evidence.** [requirements/updated_reqs.md: retirement banner](../requirements/updated_reqs.md); [requirements/component-requirements.md: retirement banner](../requirements/component-requirements.md); [reports/README.md](../reports/README.md); [reports/report-locations.json](../reports/report-locations.json); [reports/CR-02/CR-02-closeout-requirements-review.md](../reports/CR-02/CR-02-closeout-requirements-review.md); [experiment-reports/session-transcript.md:10179, 10562](../experiment-reports/session-transcript.md).

**Impact.** Explicit historical labels prevent reused HLR numbers from silently changing meaning. Topic indexes and the location map make relocated evidence discoverable while keeping workflow status stable.

**Root cause.** Developer-directed closeout reconciled logging into revision _02 and explicitly retired four legacy requirements documents.

**Recommendation.** Target: Report and requirements templates. Retain authority/supersession banners and old-to-new location maps; distinguish historical pending notes from the final status in each topic index.

**Evidence limit.** This is a current-artifact navigation review, not an exhaustive audit of every historical hyperlink. Historical transcripts retain original paths.
