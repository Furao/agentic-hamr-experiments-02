# Category 1 — Workflow and skill design

CR-02 only: requirements review through final approval. Original CR-01 development and post-closeout reporting work are excluded. Each finding states its relationship to this change.

Severity: 🔴 blocker · 🟠 moderate · 🟡 minor · 🟢 positive. Historical severity and current disposition are separate.

---

## WF-01 — Audited gates retained distinct evidence and acceptance decisions 🟢

**Scope.** `shared-workflow`. **Severity.** positive. **Disposition.** observed positive.

**CR-02 relationship.** CR-02 planning, approval and exception handling.

**Evidence.** [reports/workflow-status.md: ChangePlan(CR-02), ChangeExec(CR-02), W1/W2/W3 and AP2 rows only](../../reports/workflow-status.md); [reports/CR-02/CR-02-add-mode-manager.md §§5–7](../../reports/CR-02/CR-02-add-mode-manager.md); [experiment-reports/session-transcript.md:1282–1392, 9993, 10290, 10516](../../experiment-reports/session-transcript.md).

**Impact.** The three-wave change completed with explicit approvals, requirements ownership and scope amendments; acceptance did not silently close the hardware issue.

**Root cause.** Separate contract, development, wave and final review records made approvals and exceptions inspectable.

**Recommendation.** Target: ChangePlan/ChangeExec report templates. Retain explicit authority, scope-amendment and accepted-exception fields, including links to unresolved issues at final approval.

**Evidence limit.** This assesses one audited change. It does not establish that every invisible interaction was compliant or that the number of gates is optimal. Historical CR-01 workflow rows and later report organization are outside the assessed scope.

---

## WF-02 — CR-02 exposed and corrected an inherited carrier-capacity gap 🟠

**Scope.** `shared-workflow`. **Severity.** moderate. **Disposition.** resolved during CR-02.

**CR-02 relationship.** Inherited defect discovered and corrected during CR-02; not introduced by the change.

**Evidence.** [reports/CR-02/deployment/CR-02-w3-build.md: Remaining acceptance work](../../reports/CR-02/deployment/CR-02-w3-build.md); [reports/CR-02/bounds-correction/CR-02-bounds-development.md](../../reports/CR-02/bounds-correction/CR-02-bounds-development.md); [hamr/microkit/crates/seL4_LowLevelEthernetDriver_LowLevelEthernetDriver/src/component/seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_app/tx_bounds.rs](../../hamr/microkit/crates/seL4_LowLevelEthernetDriver_LowLevelEthernetDriver/src/component/seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_app/tx_bounds.rs); [sysmlv2/open_platform/GumboLib.sysml](../../sysmlv2/open_platform/GumboLib.sysml); `git show 043d574970d28ff172f7261ea0392ddd14ae50a8:mavlink_02/hamr/microkit/crates/firewall_core/src/net.rs:13`; `git show 043d574970d28ff172f7261ea0392ddd14ae50a8:mavlink_02/hamr/microkit/crates/seL4_LowLevelEthernetDriver_LowLevelEthernetDriver/src/component/seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_app.rs:106–114`.

**Impact.** The inherited oversized-Tx path required an authorized W3 scope amendment, a 1586-byte IPv4 limit, Tx integration guarantees and an independent driver guard. This records a limitation encountered and corrected by CR-02, not a regression introduced by ModeManager.

**Root cause.** The approved baseline already contained MAX_MTU=9000 and a driver slice sz_pkt.amessage[0..size] without a capacity check. CR-02 W3 exposed the cross-component mismatch; earlier successful contract proofs did not supply the missing capacity obligation.

**Recommendation.** Target: Contract audit and ChangePlan impact checklist. Before implementation, trace every producer length through fixed-capacity carriers and consumers; require boundary fixtures and receiver capacity obligations. Preserve the corrected shared bound and independent checked-slice defense.

**Evidence limit.** The source-level out-of-bounds path is documented; exploitation or occurrence on hardware is not demonstrated. The authorized correction passed tests/build and is resolved in this change. No finding is made about how the earlier CR-01 development was conducted.

---

## WF-03 — Deferred hardware timing evidence still needs an assigned follow-up 🟠

**Scope.** `shared-workflow`. **Severity.** moderate. **Disposition.** open; accepted deferral.

**CR-02 relationship.** Open CR-02 timing observation and accepted handoff.

**Evidence.** [open-issues/CR-02-HW-01-recovery-timeout.md](../../open-issues/CR-02-HW-01-recovery-timeout.md); [reports/CR-02/deployment/CR-02-manual-log-review-26_09_23_14_19.md](../../reports/CR-02/deployment/CR-02-manual-log-review-26_09_23_14_19.md); [sysmlv2/open_platform/open_platform_Software.sysml:324–326](../../sysmlv2/open_platform/open_platform_Software.sysml); [experiment-reports/session-transcript.md:9291–9345, 10290](../../experiment-reports/session-transcript.md).

**Impact.** HLR-30 timely Recovery observation remains unestablished after accepted CR-02 completion. The High/Open issue has neither an owner nor a follow-up CR ID.

**Root cause.** Unknown: the serial trace cannot distinguish propagation latency, sampling order or reporting error. The +200 ms budget experiment was ineffective and reverted.

**Recommendation.** Target: Issue handoff and SysSchedDef validation. Assign the future CR and owner; capture image-identified D0 threshold, D1/D2 frozen inputs, publication and monitor/log events. Retain a negative deadline regression and verify actual legacy timing units before proposing another budget change.

**Evidence limit.** Moderate denotes a remaining workflow/assurance gap, not a downgrade of the issue’s High criticality. The developer accepted deferral; this is not a current CR-02 approval blocker or proof of unwanted forwarding.
