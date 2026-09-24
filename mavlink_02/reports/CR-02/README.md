# CR-02 — Add ModeManager

**Complete; final review approved.** The High/Open timing issue is deferred to a
future CR and is non-blocking for CR-02.

- [Approved final change report](CR-02-add-mode-manager.md)
- [Final validation summary](final-validation/CR-02-final-validation.md)
- [Full verification and target build](final-validation/CR-02-full-verify-build.md)
- [Requirements reconciliation](CR-02-closeout-requirements-review.md)
- [Wave 1 review](CR-02-w1-gate.md) and [Wave 2 review](CR-02-w2-gate.md)
- [Workflow status](../workflow-status.md)
- [Deferred timing issue](../../open-issues/CR-02-HW-01-recovery-timeout.md)

## Evidence by topic

| Folder | Contents |
|---|---|
| [Pre-ChangePlan requirements review](requirements-review/README.md) | Original review dialogue for requirements revisions `_01`–`_04`, before CR-02 ChangePlan began. |
| [Contracts and model checks](contracts/README.md) | Component contract audits, model type checks and the qualified integration result. |
| [Component development](components/README.md) | Implementation reviews and their component/core tests, coverage and verification evidence. |
| [Code generation and HAMR upgrade](codegen/README.md) | Generation reviews and the comparison confirming HAMR now emits the former R2U2 workaround directly. |
| [R2U2 monitor feasibility](monitoring/README.md) | Historical monitor/reporting investigations and isolated regression probes. |
| [Deployment and hardware acceptance](deployment/README.md) | Schedule, target integration, initial Wave 3 checks and hardware/manual-log acceptance. Final build evidence is in final-validation. |
| [Ethernet bounds correction](bounds-correction/README.md) | Approved 1586-byte IPv4 limit and defensive driver fix: contracts, generation, tests, proofs and build. |
| [Mode transition logging](mode-logging/README.md) | Once-per-transition diagnostic implementation, tests, coverage, proof and build. |
| [Reverted timing experiment](timing-experiment/README.md) | Historical +200 ms budget experiment. It was ineffective and reverted; the unresolved issue is tracked in open-issues. |
| [Verus toolchain migration](toolchain/README.md) | Approved Verus/Rust migration and validation evidence. |
| [Final validation](final-validation/README.md) | Final cross-crate tests and coverage, complete verification/build, manifests, commit history and impact inventory. CR-02 is approved complete. |

[All reports](../README.md)
