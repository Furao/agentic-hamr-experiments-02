# ModeManager GUMBO specification audit — context rerun

Date: 2026-09-23  
Auditor: Codex, HAMR GUMBO Contract Audit  
Component: `ModeManager` (security-relevant mode control)  
Model: `sysmlv2/open_platform/open_platform_Software.sysml`, `ModeManager` (currently lines 321–360)  
Workflow: `CompGUMBOSpec(component=ModeManager)`, audited  
Context: `/home/robertvanvossen/tools/r2u2-HAMR-agent-context`  
Requirements authority: `action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_22_07.md`

## Executive summary

Reran the contract workflow against the user-selected context and unchanged _07 requirements. Zero AP-1–AP-9 findings; no contract revisions, waivers, or regeneration needed. This is a full-mode audit: generated GUMBOX, woven Verus contracts, and existing test source were inspected alongside the model. It supplements the earlier specification-only audit without replacing its approval history.

The existing developer approval of these unchanged contracts remains applicable. This rerun does not approve an implementation or clear the separate W1 R2U2 reporting blocker.

## Requirements and context review

| Contract obligation | Requirement | Result |
|---|---|---|
| Initialize retained mode to Normal | HLR-23, LLR-11 | Exact initialize guarantee |
| Initialize both sampled output modes to retained mode | HLR-26, LLR-11 | Both initialized to Normal |
| Support Normal and Recovery | HLR-24 | OperatingMode enum |
| Enter or retain Recovery on frozen ErrorStatus true or prior Recovery | HLR-27, LLR-14 | Exact unconditional next-state equation |
| Publish resulting mode to both firewalls each dispatch | HLR-26/27, LLR-11/14 | Both outputs equal post-state retained mode |

The new context's `doc/hamr-sysml-patterns.md` §§1, 2, 4, 6 supports sampled data-port initialization, reusable contracts, requirement-relevant retained state, and explicit modifies declarations. The existing contract satisfies these recommendations. `doc/sysmlv2-gumbo-quick-reference.md` §§12, 14–16, 18 confirms state, initialization, compute, and pre-state syntax.

ErrorStatus is sampled and frozen before dispatch. There is no absent-input or queued-event branch. `In(retained_mode)` denotes pre-state; unqualified retained mode in output guarantees denotes the resulting state. No compute assumptions restrict the four possible state/input combinations:

| Prior mode | Frozen ErrorStatus | Resulting mode and both outputs |
|---|---|---|
| Normal | false | Normal |
| Normal | true | Recovery |
| Recovery | false | Recovery |
| Recovery | true | Recovery |

This table is a specification review, not an executed test result. The initial-state guarantees and transition equation exclude a return to Normal during compute; reboot initialization restores Normal.

No integration handshake is necessary: both enum values are valid in the reusable component's context. State-dependent transition and cross-port equality are compute relations, not missing single-port invariants. No claim of whole-system assurance follows from absent integration assumptions. Initial client visibility before the manager runs and deployed scheduling remain integration/runtime obligations.

The new R2U2 guidance does not require adding a monitor to this component. The allocated HLR-30 monitor and its reporting problem remain separate MAVLinkFirewall work.

## Findings and catalog summary

No findings or severity ratings.

| Pattern | Assessment |
|---|---|
| AP-1 tautology | Guarantees constrain state and outputs; no tautological assumptions |
| AP-2 weak drop | Not applicable: sampled mode publication, no message filtering |
| AP-3 absent input | Not applicable: sampled Boolean input is always present |
| AP-4 ignored security field | ErrorStatus controls the exact transition |
| AP-5 incomplete partition | All four prior-state/input combinations covered |
| AP-6 missing integration invariant | No omitted nontrivial context-independent single-port invariant; reusable choice documented above |
| AP-7 contradiction | A unique consistent state/output assignment exists for each combination |
| AP-8 asymmetric handshake | No receiver-context assumptions introduced; no handshake required by this component contract |
| AP-9 missing variation | Deterministic latching required; no randomness requirement |

## Generated artifacts and demonstration tests

Reviewed `hamr/microkit/crates/seL4_ModeManager_ModeManager/src/bridge/seL4_ModeManager_ModeManager_GUMBOX.rs`: initialize composite conjoins all three initialization clauses; compute composite conjoins the exact transition and both publication equalities. Reviewed the matching woven ensures in `src/component/seL4_ModeManager_ModeManager_app.rs`; they preserve these model relations.

Reviewed `src/test/tests.rs`, including generated initialization, compute, and pre-state-aware property-test wrappers. Zero findings require zero demonstration tests; no test module was added. Cargo tests and Verus were not run for this specification audit. Application entrypoints currently only log, so this audit does not establish implemented transitions or actual output publication. Wave 2 must implement and test initialization, all four transition combinations, repeated Recovery, and both output publications, then verify the implementation.

## Validation and disposition

From `sysmlv2/open_platform`:

```text
/home/robertvanvossen/tools/Sireum/bin/sireum hamr sysml tipe --sourcepath ../aadl-lib:. Platform.sysml
Well-formed!
exit: 0
```

Steps 1–4 complete with unchanged contracts. The prior explicit ModeManager contract approval is retained; no new contract decision or waiver is requested. No generated files, requirements, or model source changed. The existing W1 R2U2 reporting blocker and W2/W3 status are unchanged.

## Repeat invocation (R3, 2026-09-23)

Repeated the review at the developer's request against context commit
`cf109a5e50b9208758c9f70dc5564d46a33537fc` (clean context working tree).
Re-read the workflow, decision rules, relevant _07 requirements, current ModeManager
model and generated GUMBOX. The generated component files and requirement document
have no working-tree changes from HEAD. All six ModeManager guarantees remain
unchanged; the AP-1–AP-9 assessment above still applies, with zero findings.
The same type-check command ran again and returned `Well-formed!`, exit 0.
No implementation tests or Verus verification were run.

The pre-existing model diff removes MAVLinkFirewall's monitor block; it does not
change ModeManager. This rerun preserved that edit. Consequently, earlier descriptions
of an active HLR-30 monitor in the model are historical: the generated MAVLink monitor
artifacts are now stale relative to that removal. HLR-30 monitoring remains outstanding
and is outside this component audit. No inference of system monitoring coverage or
resolution of the W1 blocker follows from this successful type-check.

R3 needs no ModeManager contract revision or regeneration. Prior approval remains
applicable to its unchanged contracts.
