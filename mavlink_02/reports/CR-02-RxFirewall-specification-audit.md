# RxFirewall GUMBO Specification Audit Report — CR-02

Date: 2026-09-22. Component: `RxFirewall`. Classification: security-critical.
Model: `sysmlv2/open_platform/open_platform_Software.sysml`, RxFirewall GUMBO block;
predicates: `sysmlv2/open_platform/GumboLib.sysml`.
Authority: `action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_22_07.md`.
Status: developer approved contracts and audited CompGUMBOSpec.AP1 on 2026-09-22,
including the shared rx_bounded_udp refactor.

## 1. Executive summary

The revised contract requires Normal mode for forwarding and suppresses all eight
frame outputs when the dispatch-frozen mode is Recovery. Direct UDP requires both
source != 14550 and destination != 14562, destination whitelist membership, and
consistent carrier-bounded IPv4/UDP lengths. The exact 14550-to-14562 route retains
its bounded MAVLink carrier. All four lanes preserve frame content and routing
exclusivity. Initialization and absent inputs produce no frame events.

The model review found zero AP-1–AP-9 findings and requires no waivers. HAMR type
checking passed on the first attempt (exit 0, `Well-formed!`). This is a contract
review, not proof of implementation correctness. Existing generated GUMBOX and
woven application contracts predate these changes and have no current_mode input;
they cannot validate the revised behavior. The approved plan batches regeneration
after W1 contract reviews and integration checking, followed by W2 implementation,
fresh GUMBOX tests and verification.

## 2. Findings and requirement coverage

No unresolved catalog findings.

| Requirement | Contract coverage |
|---|---|
| HLR-5/13 | Normal-guarded direct forwarding; ARP or bounded UDP with whitelist and both independent exclusions |
| HLR-15; LLR-2 | Every present input outside the direct/MAVLink union suppresses both corresponding outputs, including TCP |
| HLR-17; LLR-1 | Four no-input clauses and four initialization no-send clauses |
| HLR-18; LLR-3/4 | Exact route, preserved Ethernet frame, payload offset 42, length UDP length minus 8, bounded consistent lengths |
| HLR-29; LLR-10 | Four unconditional Recovery implications suppress both outputs per lane using the frozen input mode |
| LLR-1/2 | Per-lane equality and paired no-send prevent substitution and duplicate routing |

LLR-11 startup Normal publication belongs to ModeManager's following contract
workflow. Rx consumes its sampled mode without imposing a system-context assumption.
LLR-18 reason logging is an implementation/test obligation; frame-output contracts
do not prove console side effects. Requirements remain developer-owned and unchanged.

For each lane the decision partition is:

| Frozen mode | Input | Required outputs |
|---|---|---|
| Normal | Absent | Neither |
| Normal | Valid direct ARP/UDP | Exact input on direct output only |
| Normal | Valid 14550-to-14562 carrier | Preserved frame and exact metadata on MAVLink output only |
| Normal | Any other present input | Neither |
| Recovery | Any input or absence | Neither |

Direct and MAVLink predicates are disjoint: ARP and IPv4 have distinct EtherTypes;
direct UDP excludes the MAVLink source and destination. The drop predicate is the
complement of their union. Recovery overlap with drop/no-input clauses agrees on
no-send. The two-value OperatingMode enum leaves no third mode uncovered.

## 3. Catalog summary

| Anti-pattern | Result and evidence |
|---|---|
| AP-1 tautological guarantee | None; guarantees constrain output presence, equality or carrier fields |
| AP-2 weak drop disjunction | None; each drop requires both outputs absent |
| AP-3 missing no-input behavior | None; all four lanes explicitly suppress both outputs |
| AP-4 unreferenced security field | None in allocated policy; frozen mode, route ports and declared lengths are checked |
| AP-5 incomplete partition | None; table above covers mode and input categories |
| AP-6 missing output invariant | None; all four direct and four MAVLink outputs retain integration guarantees |
| AP-7 contradictory overlap | None; route predicates are disjoint and suppression clauses agree |
| AP-8 assume/guarantee asymmetry | None; reusable component has no integration assumptions; no system handshake proof claimed |
| AP-9 missing required variation | Not applicable; requirements define deterministic routing |

Both UDP routes reuse rx_bounded_udp, which constrains IPv4 length before subtraction
and requires UDP length >= 8. Per developer review, valid_ardupilot_udp now adds only
the exact source/destination checks to that shared predicate. This preserves its
accepted frames: the former IPv4-plus-Ethernet bound was overflow-safe under
valid_ipv4_udp's 9000-byte maximum and is equivalent to the shared subtraction bound.
Carrier offset/length equality bounds the final carrier extent.

## 4. Accompanying tests and validation limits

No finding-demonstration tests were added because there are no catalog findings.
Existing RxFirewall application, GUMBOX and test artifacts were inspected for
generation status; they represent the previous contract and were not used as new
contract evidence. No Verus verification was run during this specification audit.

After planned CodeGen, W2 must cover all lanes in both modes, initialization,
no-input, frame preservation, routing exclusivity, strict source-port rejection
(including source 14550 to destination 68), declared-length boundaries and malformed
lengths. Tests must evaluate the refreshed GUMBOX as well as implementation behavior.

Type-check command, from `sysmlv2/open_platform/`:

```text
/home/robertvanvossen/tools/Sireum/bin/sireum hamr sysml tipe --sourcepath ../aadl-lib:. Platform.sysml
```

Result: exit 0, `Well-formed!`, including the rerun after the developer-requested
shared-predicate refactor. `git diff --check` also passed.
No requirements, application or generated-code edits
are included in this contract step.
