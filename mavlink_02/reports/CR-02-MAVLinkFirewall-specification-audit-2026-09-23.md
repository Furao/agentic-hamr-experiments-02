# MAVLinkFirewall GUMBO audit — new-context rerun

Date: 2026-09-23. Auditor: Codex / HAMR GUMBO Contract Audit.
Classification: security-critical. Mode: full (generated crate exists).
Source: `sysmlv2/open_platform/open_platform_Software.sysml`, `MAVLinkFirewall`.
Authority: `action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_22_07.md`.
Context: `/home/robertvanvossen/tools/r2u2-HAMR-agent-context`, commit
`cf109a5e50b9208758c9f70dc5564d46a33537fc`.
Status: revised monitor explicitly approved by the developer on 2026-09-23; AP1 complete.

## Summary

The working model had no MAVLinkFirewall monitor on entry. Added a future-time
monitor for HLR-30 / LLR-15–16, following the new context's
`doc/sysmlv2-r2u2-monitors.md` §§7–8.3. Existing initialization, routing, count,
status and integration clauses remain unchanged. Fresh HAMR type-check passed.

No new AP-1–AP-9 weakness was identified in those model compute clauses, conditional
on correct refinement of the abstract framing/blacklist hooks. This is not a clean
end-to-end security verdict: the previously recorded truncated SECURE_COMMAND hook
concern and the one-time D2 reporting obligation remain unresolved.

## Revised temporal contract

```text
guarantee hlr_30_llr_15_16_recovery_deadline:
    (In(rejected_count) < error_threshold() and error_status) implies
    Eventually [1,2] (current_mode == open_platform_Data_Model::OperatingMode.Recovery);
```

The pre-count is captured before application compute, final ErrorStatus afterward,
and Mode from the frozen input snapshot. Under HLR-25/31 and LLR-12/13, count starts
at zero, never decreases, and final ErrorStatus equals post-count >= 5. Therefore
pre-count < 5 and true final status identifies exactly the first assertion at D0,
including a multi-message jump across the threshold. On subsequent dispatches the
antecedent is false; repeated true status cannot restart the obligation. No added
state variable, handwritten timer or mixed past/future operators are needed.

The closed [1,2] window is precisely LLR-16's D1/D2 window. R2U2 already evaluates
the formula at each index; an outer Globally is unnecessary. No initialization
sample may be inserted. This trigger depends on the count/status compute contracts;
it is not an independent monitor of their implementation correctness. If that
relationship is violated, first-assertion detection is not assured.

| Trace after D0 trigger | Semantic obligation |
|---|---|
| Recovery at D1 | Satisfied |
| Recovery first at D2 | Satisfied |
| Recovery first at D3 | False for D0; D3 cannot repair the expired window |
| Recovery never arrives | False for D0 |
| ErrorStatus never true | No triggered obligation |
| ErrorStatus remains true | Original obligation only |

These are semantic review cases, not executed generated-monitor tests. Regeneration
and runtime tests must establish that the D0 verdict is available during D2, despite
future-time verdict timestamps and multiple outputs in a single monitor step.

The context's §6 explicitly documents newest-verdict caching. Later vacuous true
verdicts may obscure a false D0 verdict. Neither a default status log nor adding an
alert mapping establishes the required one-time timeout diagnostic. No alert port
was added: current requirements request logging, and cached-verdict alerts do not
resolve this concern. Existing W1 reporting blocker remains; its prior reproducer
used the old past-time formula and must be rerun against this new formula after
generation. Do not claim the old result proves the new formula's runtime behavior.

## Compute audit and traceability

| Requirement | Existing coverage |
|---|---|
| HLR-19/32, LLR-6–9 | Four-lane blacklist/validity checks via developer hooks |
| HLR-20/21 | Invalid and absent input require no output on each lane |
| HLR-22, LLR-3/9 | Allowed Normal-mode forwarding preserves complete carrier and lane |
| HLR-28, LLR-10 | Frozen Recovery suppresses every Ethernet output |
| HLR-31, LLR-11/13 | Initialize zero; exact per-lane rejection sum; saturate at 20; monotonic |
| HLR-25, LLR-11/12 | Initialize false; unconditional final threshold equality |
| LLR-17 | Monitor supplies no routing, count, or mode update |

| Catalog item | Assessment |
|---|---|
| AP-1 | No tautological model clauses; pre-count bound is an inductive state condition |
| AP-2 | Deny clauses require NoSend without an escape disjunct |
| AP-3 | Every absent input has a no-send clause |
| AP-4 | Carrier/frame validity, blacklist and frozen mode all influence routing |
| AP-5 | Absent/invalid/blacklisted/allowed covered for both modes on all four lanes |
| AP-6 | Each output exports its own allowed-message invariant; sampled Boolean status has no nontrivial standalone value restriction |
| AP-7 | Recovery suppression and Normal forwarding disjoint; exact count/status relations consistent |
| AP-8 | No context-specific receiver handshake; no new assumption on external traffic |
| AP-9 | No random/fresh-value requirement |

One Boolean rejection per present disallowed lane prevents double counting. The sum
is at most four. Saturation uses remaining capacity before addition, avoiding u16
overflow. Saturation does not remove routing obligations on later lanes. Existing
integration clauses describe context-independent output validity, consistent with
`hamr-sysml-patterns.md` §2. They do not establish system-wide timing assurance.

## Generated artifacts and remaining obligations

Inspected the generated `seL4_MAVLinkFirewall_MAVLinkFirewall_GUMBOX.rs`, application
woven requires/ensures, developer parser/blacklist hooks, and `src/test/tests.rs`.
Compute translation includes the mode/count/status clauses and conjunction of all
four lane partitions. Generated temporal artifacts still describe the old monitor
and require CodeGen after approval of this revision.

The developer hook still requires at least eight transmitted payload bytes before
reading SECURE_COMMAND operation 7. The previously recorded legal v2 trailing-zero
truncation concern therefore remains: a five-byte payload ending in 07 can represent
operation 7 but evade that hook. Both executable and formal definitions require
refinement against _07 plus a CRC-valid rejection/counting test in W2. This concern
is not waived by the absence of new model-clause catalog findings.

Existing tests also retain the old eight-argument compute_CEP_Post call while the
current generated oracle takes twelve arguments, including count, mode and status.
Their compute property-test input list omits current_mode. These are known-generation
migration work for W2, not passing validation evidence. Implementation, tests and
generated files were not modified in this contract rerun.

No new catalog finding-demonstration module was required or added. No Cargo tests
or Verus were run. Remaining implementation tests must cover both modes, all lanes,
empty inputs, threshold crossing, saturation, hook refinement, final-output ordering,
reason logging, D1/D2/D3/never Recovery, repeated status and reboot reporting reset.

## Validation and approval

Executed from `sysmlv2/open_platform`:

```text
/home/robertvanvossen/tools/Sireum/bin/sireum hamr sysml tipe --sourcepath ../aadl-lib:. Platform.sysml
Well-formed!
exit 0
```

Requirements remain user-owned and unchanged. AP1 must review the new future-time
monitor and its explicit count/status dependency. Prior approval of the old monitor
does not cover this revision. After approval, run CodeGen and regenerate/retest the
reporting probe; W1 remains blocked until actual D2 one-time logging is established.

Approval recorded: developer approved the revised monitor on 2026-09-23. CodeGen and reporting validation follow; outstanding obligations above are not waived.
