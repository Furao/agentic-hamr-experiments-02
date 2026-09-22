# CR-02 Wave 1 — requirements planning against revision _07

Date: 2026-09-22. Source: `Open_Platform_HLRs_26_09_22_07.md`.
Authority: developer explicitly directed requirements planning against this revision.
The supplied file is unchanged. This artifact allocates and reviews requirements;
it does not create a competing requirements baseline.

## Outcome and source precedence

Revision _07 supplies the high-level requirements and 21 derived requirements under
`RC_INSPECTA_00-LLR-1` through `RC_INSPECTA_00-LLR-21`. It replaces _06 and the
agent-authored reference additions as the active source for CR-02. Use the actual
HLR/LLR IDs in contracts, tests and reports; never the reference draft's CR02-DR IDs.

The consolidated source resolves the missing requirements that prompted the W1
requirements handoff: bounds/carriers, protocol validity, lane correspondence,
sampled control initialization, Recovery counting, R2U2 allocation, diagnostics,
integration and assurance expectations are now developer-supplied requirements.
No new blocking requirements inconsistency was found in the CR-02 change behavior.
The requirements input is ready for dependent modeling. This does not establish
model correctness, tool support, temporal-monitor feasibility or any W1 wave gate.

The old `requirements/updated_reqs.md`, `component-requirements.md`, ConOps and
DataDict remain CR-01 documents. They are contextual historical material and cannot
override _07. The earlier request to copy all changes into four separate documents
is not an additional prerequisite to modeling now that the developer has supplied
a consolidated HLR/LLR source. Their documentation reconciliation remains tracked
for the developer-owned back-propagation sweep; this report supplies the allocation
and ID map needed now. No such files were edited or claimed updated.

## Differences from the reference additions and approved plan

1. The reference draft's DR-1–19 correspond to actual LLR-1–19.
2. The reference draft's separate DR-20 (Tx/driver non-regression) is omitted.
   Its approved sketch/plan constraints remain traceable as such, not as a fictional
   LLR. LLR-21 still explicitly requires hardware transmit non-regression evidence.
3. Reference DR-21 becomes LLR-20; reference DR-22 becomes LLR-21.
4. LLR-20 retains Rx/MAVLink period 1000 ms and configured compute-execution time
   300 ms, and adds a preference for ModeManager first in the schedule with a
   compute-execution time of 100 ms. Update the planned candidate accordingly.
   Interpret first as first application component, with required pacer machinery
   preceding it as needed. Record the precise deployed sequence during SysSchedDef.
   ModeManager's period is still to be selected and recorded during modeling;
   do not mistake its compute budget for its period.
5. The new schedule preference refines the approved scheduling work; it does not add
   a wave, process beyond ModeManager, or formal-system-proof obligation. It is
   incorporated on the developer's explicit direction; no unrelated approval is inferred.

## HLR allocation

All numbers below abbreviate the supplied `RC_INSPECTA_00-HLR-*` IDs.

| Component | HLRs | Planned contract/implementation obligation |
|---|---|---|
| RxFirewall | 5, 13, 15, 17, 18, 29 | Normal-only ARP/direct UDP/MAVLink routing; strict UDP ports; exclusive/drop/no-input behavior; same-snapshot Recovery suppression |
| TxFirewall | 7, 12, 14, 16 | Existing ARP/IPv4 forwarding and disallowed/no-input behavior; shared-dependency verification regression |
| MAVLinkFirewall | 19, 20, 21, 22, 25, 28, 30, 31, 32 | Blacklist/validity and Normal-only forwarding; count/status; Recovery suppression; generated R2U2 and one-time D2 reporting |
| ModeManager | 23, 24, 26, 27 | Initial Normal, two-value mode, per-dispatch publication, same-dispatch transition and reboot latch |

## Complete LLR allocation and evidence plan

All numbers abbreviate `RC_INSPECTA_00-LLR-*`. W1 is model/contracts/generation;
W2 is implementation/tests/verification; W3 is deployment/hardware/final evidence.

| LLR | Owner / affected artifact | Planned realization | Evidence / wave |
|---|---|---|---|
| 1 | Ethernet interfaces; firewalls | Preserve four event-data lanes, capacity one, corresponding outputs and per-dispatch lane processing | Model connection/queue inspection W1; all-lane/empty/burst tests W2/W3 |
| 2 | Rx contracts/application | Exhaustive Direct/MAVLink/Drop decisions; no dual output; TCP drop; Recovery suppresses both outputs | Per-lane guarantees W1; branch/oracle tests and Verus W2 |
| 3 | Rx, MAVLink, carrier type | Preserve Ethernet frame and validated offset/length; unchanged carrier forwarding | Carrier invariants W1; preservation/bounds tests and proofs W2 |
| 4 | Rx, firewall_core, GumboLib | Bound every field read and IPv4/UDP arithmetic within 1600-byte carrier; no capacity expansion | Edge-length/overflow cases, verified helpers and Tx regression W2 |
| 5 | Rx/MAVLink/core boundaries | Network classification in Rx/core, bounded MAVLink parsing in MAVLink/core; preserve shared validity boundary | Dependency/specification review W1/W2; no new cross-layer parsing |
| 6 | MAVLink/mavlink_core | Exactly one complete v1/v2 frame, CRC/dialect and supported flags/signature structure | Retain parser proof; malformed/unsupported/trailing-byte tests W2 |
| 7 | MAVLink policy/parser | Legal v2 truncation; v1 fixed length; bounded policy reads | Truncated-field and framing fixtures, core/component proofs W2 |
| 8 | MAVLink policy | Command 42650 and secure operation 7 blacklist; allow valid FTP in Normal | Independent v1/v2 command/secure/FTP fixtures W2; hardware W3 |
| 9 | MAVLink contracts | Output validity applies to frame ports only; fail-closed/no-input does not stop ErrorStatus | Guarded output invariants W1; empty/invalid/status tests W2 |
| 10 | Rx/MAVLink frozen inputs | Use local frozen Mode for whole dispatch; Rx eight outputs and MAVLink four suppressed | Snapshot-based clauses W1; first-Recovery/mixed-lane tests W2/W3 |
| 11 | Mode/error DataPorts; three components | Explicit initialized Normal/false/zero; per-dispatch current mode | Initialization/codegen inspection W1; initial-value tests W2 |
| 12 | MAVLink application/status | Process all lanes, update count, then final ErrorStatus output iff count >=5 | Post-state contract; threshold/empty/Recovery tests and proof W2 |
| 13 | MAVLink counter state | Saturate at 20; classify in Recovery; count once per invalid/blacklisted message, never mode-only drops | Monotonic/bounded transition proof; 4→5, 19→20, overlap, reset tests W2 |
| 14 | ModeManager state/outputs | Recovery iff previous Recovery or current error; publish same dispatch; reboot reset | State and output contracts W1; latch/false-after-true tests/proof W2 |
| 15 | GUMBO/R2U2 generation | One sample per compute, frozen Mode and final ErrorStatus; correct initialization alignment | Generate/compile specification and inspect hooks W1; actual monitor traces W2 |
| 16 | R2U2/reporting integration | First assertion D0; accept D1/D2 Recovery; one timeout during D2; no restart; reset on reboot | Actual verdict/log timestamps, late/never/repeated/no-input traces W2/W3 |
| 17 | Application/monitor boundary | Verdict cannot alter forwarding/count/mode; no GUMBOX delegation | Code/data-flow review and application proofs W2; report trust boundary W3 |
| 18 | Rx/MAVLink diagnostics | Network/malformed/blacklist/Recovery reasons and routing diagnostics; independent one-time timeout | Diagnostic capture plus branch tests W2; hardware logs W3 |
| 19 | VMM/interfaces | Preserve four raw and four carrier receive lanes and virtio injection/transmit | Generated interface/source diff W1/W2; full custom build and traffic W3 |
| 20 | Model properties/schedule/custom.mk | ZCU102; R2U2 dependencies/versions; Rx/MAVLink 1000/300 ms; prefer manager first/100 ms; validate D2 | Model configuration W1; generated build W1/W2; slot/budget/sampling trace and target evidence W3 |
| 21 | Test/proof/report artifacts | Independent implementation; application/GUMBOX coverage; affected plus cross-crate proof; actual monitor/hardware evidence | W2 coverage/proof gates and W3 tests/build/hardware/report; distinguish R2U2 trust from application proof |

LLR-10's mode-observation obligation is allocated to RxFirewall and MAVLinkFirewall,
as its enumerated output sets and HLR-28/29 specify. TxFirewall has no Mode port
or Recovery forwarding restriction. This is consistent with the approved non-impact
boundary and LLR-21's transmit non-regression check.

## RF-1–RF-4 disposition

| Earlier feedback | _07 disposition | Remaining work |
|---|---|---|
| RF-1 source authority, missing refinements and ID collisions | HLRs plus LLR-1–21 supply the required source content; historical migration table below disambiguates old IDs | Contract aliases and tests must adopt actual IDs during W1/W2; legacy SysReqs publication remains developer-owned |
| RF-2 component allocation and Recovery/R2U2 detail | LLR-10–18 make those details explicit; allocation tables above cover all HLRs/LLRs | Apply model/contracts and review coverage; legacy component-document synchronization tracked in final sweep |
| RF-3 operational description | _07 defines Normal/Recovery, timeout, persistence and reboot; no missing behavior blocks modeling | ConOps dated annotations/domain/scenario updates remain a developer documentation task |
| RF-4 types, counts, timing and protocol bounds | LLR-3/4/7/10–16/20 provide these definitions; distinguish full UDP-carried message length from MAVLink transmitted payload length in model/spec names | DataDict naming/publication reconciliation remains developer-owned; no source requirement needs duplication before modeling |

## Historical ID migration

Historical IDs below mean `CR-01:RC_INSPECTA_00-HLR-n`, never the current HLR-n.

| Old HLR | Current trace target |
|---|---|
| 19 exclusive Rx routing | LLR-2; HLR-5/13/15/18 |
| 20 Rx logging | LLR-18 |
| 21 MAVLink structural validity | HLR-20/22; LLR-3/6/7 |
| 22 allowed forwarding | HLR-22, now Normal-guarded; LLR-9/10 |
| 23 FTP | LLR-8; HLR-22 |
| 24 firmware denial | HLR-19/32; LLR-8 |
| 25 malformed rejection | HLR-20; LLR-6/9 |
| 26 no-input/fail-closed | HLR-21; LLR-9 |
| 27 MAVLink logging | LLR-18 |
| 28 VMM integration | LLR-19 |
| 29 period/budget | LLR-20 |
| 30 four-lane capacity | LLR-1 |
| 31 separation | LLR-5 |
| 32 Tx verification non-regression | Sketch and approved plan; LLR-21 regression evidence (no separate replacement LLR) |
| 33 implementation independence | LLR-17/21 |

## Next step and retained limits

Requirements planning against _07 is complete. SysModeling is the next W1 action;
this turn has not changed model, code, requirements, or deployment artifacts.
Required audited model/contract/generation gates remain in force. R2U2 same-D2
reporting feasibility, generated startup values, ModeManager period and deployed
budget remain implementation/modeling checks, not unresolved requests to restate HLRs.
