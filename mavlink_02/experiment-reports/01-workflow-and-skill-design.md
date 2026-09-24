# Category 1 — Workflow and skill design

Assessment of CR-02 in mavlink_02 only. Historical findings and current dispositions are distinguished below.

Severity: 🔴 blocker · 🟠 moderate · 🟡 minor · 🟢 positive.

---

## WF-01 — Audited gates retained distinct evidence and acceptance decisions 🟢

**Scope.** `shared-workflow`. **Severity.** positive. **Disposition.** observed positive.

**Evidence.** [reports/workflow-status.md](../reports/workflow-status.md); [reports/CR-02/CR-02-add-mode-manager.md §§5–7](../reports/CR-02/CR-02-add-mode-manager.md); [experiment-reports/session-transcript.md:1282–1392, 9993, 10290, 10516](../experiment-reports/session-transcript.md).

**Impact.** The three-wave change completed with explicit approvals, requirements ownership and scope amendments; acceptance did not silently close the hardware issue.

**Root cause.** Separate contract, development, wave and final review records made approvals and exceptions inspectable.

**Recommendation.** Target: ChangePlan/ChangeExec report templates. Retain explicit authority, scope-amendment and accepted-exception fields, including links to unresolved issues at final approval.

**Evidence limit.** This assesses one audited change. It does not establish that every invisible interaction was compliant or that the number of gates is optimal.

---

## WF-02 — Component proofs initially omitted a critical carrier-capacity boundary 🟠

**Scope.** `shared-workflow`. **Severity.** moderate. **Disposition.** resolved during CR-02.

**Evidence.** [reports/CR-02/deployment/CR-02-w3-build.md: Remaining acceptance work](../reports/CR-02/deployment/CR-02-w3-build.md); [reports/CR-02/bounds-correction/CR-02-bounds-development.md](../reports/CR-02/bounds-correction/CR-02-bounds-development.md); [hamr/microkit/crates/seL4_LowLevelEthernetDriver_LowLevelEthernetDriver/src/component/seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_app/tx_bounds.rs](../hamr/microkit/crates/seL4_LowLevelEthernetDriver_LowLevelEthernetDriver/src/component/seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_app/tx_bounds.rs); [sysmlv2/open_platform/GumboLib.sysml](../sysmlv2/open_platform/GumboLib.sysml).

**Impact.** A declared IPv4 length above 1586 could yield a transmit size above the 1600-byte carrier despite passing the earlier Tx contract/proof. The correction expanded scope late in W3.

**Root cause.** The earlier shared 9000-byte limit and unchecked driver slice were inconsistent with carrier capacity; proof of the existing specification did not supply the missing bound.

**Recommendation.** Target: Contract audit and ChangePlan impact checklist. Before implementation, trace every producer length through fixed-capacity carriers and consumers; require boundary fixtures and receiver capacity obligations. Preserve the corrected shared bound and independent checked-slice defense.

**Evidence limit.** The source-level out-of-bounds path is documented; exploitation or occurrence on hardware is not demonstrated. The authorized correction passed tests/build and is resolved in this change.

---

## WF-03 — Deferred hardware timing evidence still needs an assigned follow-up 🟠

**Scope.** `shared-workflow`. **Severity.** moderate. **Disposition.** open; accepted deferral.

**Evidence.** [open-issues/CR-02-HW-01-recovery-timeout.md](../open-issues/CR-02-HW-01-recovery-timeout.md); [reports/CR-02/deployment/CR-02-manual-log-review-26_09_23_14_19.md](../reports/CR-02/deployment/CR-02-manual-log-review-26_09_23_14_19.md); [sysmlv2/open_platform/open_platform_Software.sysml:324–326](../sysmlv2/open_platform/open_platform_Software.sysml); [experiment-reports/session-transcript.md:9291–9345, 10290](../experiment-reports/session-transcript.md).

**Impact.** HLR-30 timely Recovery observation remains unestablished after accepted CR-02 completion. The High/Open issue has neither an owner nor a follow-up CR ID.

**Root cause.** Unknown: the serial trace cannot distinguish propagation latency, sampling order or reporting error. The +200 ms budget experiment was ineffective and reverted.

**Recommendation.** Target: Issue handoff and SysSchedDef validation. Assign the future CR and owner; capture image-identified D0 threshold, D1/D2 frozen inputs, publication and monitor/log events. Retain a negative deadline regression and verify actual legacy timing units before proposing another budget change.

**Evidence limit.** Moderate denotes a remaining workflow/assurance gap, not a downgrade of the issue’s High criticality. The developer accepted deferral; this is not a current CR-02 approval blocker or proof of unwanted forwarding.
