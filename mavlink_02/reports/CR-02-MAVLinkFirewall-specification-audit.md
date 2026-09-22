# MAVLinkFirewall GUMBO Specification Audit Report — CR-02

Date: 2026-09-22. Auditor: HAMR GUMBO Contract Audit.
Component: `MAVLinkFirewall`; classification: security-critical.
Model: `sysmlv2/open_platform/open_platform_Software.sysml`, MAVLinkFirewall block.
Authority: `action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_22_07.md`.
Status: model critique complete; audited CompGUMBOSpec.AP1 approval pending.

## 1. Executive summary

The contract now initializes rejection count to zero and ErrorStatus to false,
counts each present invalid or blacklisted carrier once across all four lanes,
saturates at twenty, and sets final ErrorStatus exactly when the post-count is at
least five. Classification/counting remain active during Recovery. Forwarding
requires the frozen Normal mode; Recovery and absent inputs suppress frame outputs.
Allowed outputs preserve the complete corresponding input carrier.

The new GUMBO monitor uses a past-time formula to evaluate the first-assertion
deadline at D2. No hand-written deadline counter is introduced. Type checking passes.
The AP-1–AP-9 review found no new model-clause findings. This result is conditional
on the existing component-owned parser/blacklist predicate refinement boundary;
the truncated SECURE_COMMAND observation below requires attention in W2.

This review does not establish generated-monitor timing, logging correctness or
implementation conformance. W1 CodeGen and W2 implementation/test gates remain open.

## 2. Contract coverage and findings

No unresolved AP-1–AP-9 findings in the revised model clauses; no catalog waivers.

| Requirements | Contract |
|---|---|
| HLR-19/32; LLR-6/7/8 | Existing component-local validity/firmware-blacklist hooks, with corrected traceability; deny clauses cover all four lanes |
| HLR-20/21; LLR-9 | Separate invalid/no-input no-send clauses on every frame output |
| HLR-22; LLR-1/3/9 | Normal-only allowed forwarding with whole-carrier equality on the corresponding lane |
| HLR-28; LLR-10 | Recovery suppression for all four frame outputs using the frozen input mode |
| HLR-31; LLR-11/13 | Zero initialization; exact four-lane rejection sum; saturation at 20; nondecreasing count |
| HLR-25; LLR-11/12 | Initial false status; unconditional post-count threshold equality, including empty-input and Recovery dispatches |
| HLR-30; LLR-15/16 | GUMBO temporal monitor over final ErrorStatus and frozen mode; reporting integration still required |
| LLR-17 | Monitor is separate from compute clauses and contributes no forwarding/count decision |

For each lane:

| Input | Normal output | Recovery output | Rejection increment |
|---|---|---|---|
| Absent | None | None | 0 |
| Valid and nonblacklisted | Exact input carrier | None | 0 |
| Valid and blacklisted | None | None | 1 |
| Invalid carrier or malformed frame | None | None | 1 |

The rejection sum uses one Boolean per present lane, independent of mode. Thus a
message cannot count twice for overlapping rejection criteria. Its maximum is four.
The saturation helper compares against remaining capacity before adding, avoiding
unsigned overflow. Saturation does not weaken any lane's routing obligation.

HAMR does not allow thread-level `invariants`. The initial draft was rejected for
that reason; the revision expresses the count bound as a compute assumption on
pre-state and a guarantee on post-state, supported by zero initialization. This is
an inductive local-state condition, not an assumption restricting external traffic.
W2 must prove its preservation across all entry points and reboot.

## 3. Temporal monitor review

The model contains:

```text
(Once [2,2] (error_status and not (Once [1,1] error_status))) implies
(Once [0,1] (current_mode == OperatingMode.Recovery))
```

`Once [a,b]` examines samples a through b dispatches in the past. At D2, the
antecedent examines the ErrorStatus rising edge at D0; the consequent examines
Recovery at D1 or D2. The count is nondecreasing and status equals its threshold,
so there is only one rising edge per boot on contract-conforming executions.
Repeated true samples cannot restart or extend that obligation. Initialization
resets the application and must reset the monitor/reporting state without inserting
an extra compute sample.

Expected traces below are semantic review cases, not results from executing R2U2.
Each assumes Normal at D0 and ErrorStatus stays true from D0 onward.

| Recovery observation | D2 deadline verdict | Required diagnostic |
|---|---|---|
| D1 | True | None |
| D2 | True | None |
| D3 | False | Once during D2; D3 cannot retract it |
| Never | False | Once during D2 |
| ErrorStatus never asserted | No triggered obligation | None |

The formula relies on the separately specified monotonic ErrorStatus property; it
does not independently diagnose erroneous false/true oscillation of that output.
It uses the LLR-16 D1/D2 window. On conforming ModeManager executions, Recovery
already present at D0 persists through that window and satisfies the deadline.

Installed HAMR source was inspected:

- `GumboRustPlugin.scala` creates pre/post compute hooks, loads input-only signals
  before compute and output signals afterward, then invokes one `monitor_step`.
- `GumboC2POUtil.scala` accepts pure past-time formulas; mixed future/past formulas
  are rejected. `SlangExpUtil.scala` lowers `Once` to C2PO's `O` operator.
- `GumboR2U2Util.scala` processes verdicts after the step. Unmapped specifications
  receive routine info-level status logging, which does not meet LLR-16.

W1 CodeGen must demonstrate a regeneration-safe, same-dispatch reporter consuming
the generated verdict after the monitor step. It must emit the required timeout
error once per boot, without changing application routing/counting. Check actual
startup history semantics, sample timestamps, verdict-buffer/cache behavior and
hook ordering. W2 must execute the generated monitor and reporter for the traces
above, repeated assertions, empty inputs and reboot. A passing type-check does not
prove any of those runtime properties; the R2U2 compiler/runtime remains a trust
boundary. No generated files or generator sources were edited in this step.

## 4. Catalog summary

| Pattern | Disposition |
|---|---|
| AP-1 | No tautological clauses; state range, update, status and outputs are constrained |
| AP-2 | All deny/invalid/Recovery consequents require no-send |
| AP-3 | Each lane has an explicit absent-input clause |
| AP-4 | Frozen mode, carrier fields and parser/blacklist hooks participate in forwarding; predicate refinement caveat below |
| AP-5 | Table covers both modes and every input category; count/status apply unconditionally |
| AP-6 | All four filtered frame outputs export allowed-carrier invariants; Boolean status admits both values, with its state-dependent relation specified in compute |
| AP-7 | Normal allow and Recovery suppress are disjoint; invalid/blacklisted/absent partitions do not require conflicting outputs |
| AP-8 | No receiver integration assumptions; count pre-bound is a local inductive state condition |
| AP-9 | No randomness or freshness requirement |

## 5. Refinement observations and follow-up obligations

The two bodyless `@spec` declarations retain the existing component-owned developer
Verus/GUMBOX definitions. They are not independent model-level definitions of
framing or blacklist bytes. W2 must revalidate their semantics against _07, including
legal truncation, fixed v1 lengths, exact framing/CRC/dialect checks and allowed FTP.

In particular, the existing `secure_command_requests_bootloader_flash_spec` and
runtime classifier require at least eight transmitted payload bytes before checking
operation 7. A v2 payload whose bytes are `00 00 00 00 07` can represent that operation
with its trailing zero bytes omitted. The parser permits shortened v2 payloads,
but those current blacklist checks skip this case. This is a concrete developer-hook
refinement concern for HLR-32/LLR-7/8, outside the new mode/count clause partition.
W2 must add a CRC-valid fixture, reconcile omitted-zero field semantics in both
executable and Verus/GUMBOX hook definitions, and demonstrate denial/counting.
Do not carry forward the previous hook verification result as evidence that this
requirement is satisfied. No payload/checksum bytes outside the transmitted payload
may be read as the missing operation bytes.

GUMBO postconditions constrain final port/state values, not intra-dispatch write
order or console effects. LLR-12's final application output update, LLR-18 reason
logging and LLR-16's one-time timeout diagnostic require implementation inspection
and tests. The monitor must remain independent of forwarding and must not become
the application implementation via GUMBOX calls.

## 6. Validation and demonstration tests

Executed from `sysmlv2/open_platform/`:

```text
/home/robertvanvossen/tools/Sireum/bin/sireum hamr sysml tipe --sourcepath ../aadl-lib:. Platform.sysml
```

First attempt reported the unsupported thread invariant (despite process exit 0).
After revision, the command reported `Well-formed!`, exit 0. `git diff --check`
passed. No Verus was run for this specification audit.

Existing MAVLinkFirewall application, generated GUMBOX and tests were inspected.
Their generated contracts still use the old IDs and lack count/mode/status clauses.
They cannot test the new contract until planned W1 regeneration. No catalog-finding
demonstration tests were added because there are no new model-clause findings.
W2 must add fresh application/GUMBOX coverage for both modes, all lane categories,
threshold crossing, count 19 with multiple rejects, saturation without skipped lane
processing, empty inputs and initialization, plus the hook refinement and actual
monitor/reporting tests above. This report does not claim those tests passed.
