# ModeManager GUMBO Specification Audit Report — CR-02

Date: 2026-09-22. Auditor: HAMR GUMBO Contract Audit.
Component: `ModeManager`; classification: safety/security-critical mode control.
Model: `sysmlv2/open_platform/open_platform_Software.sysml`, ModeManager block.
Authority: `action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_22_07.md`.
Mode: specification-only; no ModeManager generated crate exists yet.
Status: critique complete; audited CompGUMBOSpec.AP1 approval pending.

## 1. Executive summary

The contract initializes retained mode and both sampled mode outputs to Normal.
Each compute dispatch sets retained mode to Recovery if frozen ErrorStatus is true
or the pre-state mode is already Recovery; otherwise it sets Normal. Both outputs
must equal that resulting state during the same dispatch. No input assumptions or
bodyless specification hooks are required.

All four combinations of retained mode and Boolean input have a unique specified
result. The AP-1–AP-9 review found zero findings and requires no waivers. The full
model type-check passed on the first attempt. This is specification evidence;
generated initialization, runtime publication, implementation and scheduling remain
subject to the approved W1/W2/W3 checks.

## 2. Findings and traceability

No findings.

| Requirement | Contract or model element |
|---|---|
| HLR-23; LLR-11 | `hlr_23_llr_11_initial_mode` fixes retained_mode to Normal |
| HLR-24 | OperatingMode enum restricts state and both outputs to Normal/Recovery |
| HLR-26; LLR-11 | Initialization clauses set both outputs to retained Normal; sampled DataPort declarations retained |
| HLR-27; LLR-14 | `hlr_27_llr_14_latched_transition` uses frozen error_status and In(retained_mode) |
| HLR-26/27; LLR-14 | Both unconditional publication guarantees equate outputs to post-state retained_mode |

Truth-table review (not an executed implementation test):

| Pre-state mode | Frozen ErrorStatus | Post-state mode | Both output values |
|---|---|---|---|
| Normal | false | Normal | Normal |
| Normal | true | Recovery | Recovery |
| Recovery | false | Recovery | Recovery |
| Recovery | true | Recovery | Recovery |

Initialization is separate from compute and sets all three mode values to Normal,
regardless of the sampled input. Compute cannot return Recovery to Normal. Reboot
must invoke initialization and reset the generated runtime state, as checked later.
ErrorStatus is sampled data, so there is no absent-event case; before the producer
runs, its required initial value is false. The contract does not assume monotonic
ErrorStatus and remains latched for any later Boolean sequence.

## 3. Catalog summary

| Pattern | Disposition |
|---|---|
| AP-1 | No tautologies: initial state, transition and each output value are constrained |
| AP-2 | Not applicable: no frame/drop outputs |
| AP-3 | Not applicable to sampled DataPort input; outputs are constrained on every compute dispatch |
| AP-4 | Both security-relevant inputs to the transition, ErrorStatus and prior mode, are used |
| AP-5 | Complete two-mode by two-Boolean partition shown above |
| AP-6 | No omitted nontrivial port-local invariant: both enum values are legitimate; output equality and latching are relational/temporal properties expressed in compute |
| AP-7 | Transition fixes one post-state; both publication guarantees agree with it |
| AP-8 | Reusable contract has no receiver integration assumptions or system-context constraints |
| AP-9 | No randomness/freshness requirement; deterministic repeated publication is intended |

An integration guarantee enumerating Normal or Recovery would only repeat the type
and add a tautology. No such clause is added. Equality between the outputs and the
state-dependent transition are specified directly by compute guarantees. This does
not establish the system-level mode-propagation deadline.

## 4. Demonstration tests and validation

The component has no generated crate, so the audit runs in specification-only mode.
There are no finding IDs awaiting demonstration and no finding-demonstration tests
are needed. W2 CompDev.3 must nevertheless test initialization, all four table rows,
repeated publication, a true-then-false input sequence and reboot reset using the
application and generated GUMBOX. No implementation tests or Verus were run here.

Executed from `sysmlv2/open_platform/`:

```text
/home/robertvanvossen/tools/Sireum/bin/sireum hamr sysml tipe --sourcepath ../aadl-lib:. Platform.sysml
```

Result: exit 0, `Well-formed!`, first attempt. `git diff --check` passed.

W1 CodeGen must inspect generated enum defaults, startup publication and consumer
sampled-port initialization so consumers see Normal even before ModeManager first
computes. Postcondition equality specifies resulting values; actual per-dispatch
publication calls and reset paths require W2 implementation inspection/tests.
W3 must check scheduling and propagation against the existing HLR-30 monitor
deadline. No new schedule, generated code, application code or requirements edits
were made in this contract step.
