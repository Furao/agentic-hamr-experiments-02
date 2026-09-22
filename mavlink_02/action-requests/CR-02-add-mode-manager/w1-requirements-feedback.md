# CR-02 Wave 1 — requirements review feedback

Update: developer-supplied revision _07 and `w1-requirements-planning-07.md`
supersede this initial feedback handoff. Source requirements are now supplied;
legacy document reconciliation remains tracked separately.

Date: 2026-09-22. Authoritative supplied requirements:
`Open_Platform_HLRs_26_09_22_06.md`. Approved plan: `change-plan.md`.
This is agent feedback for developer-owned edits, not a replacement requirements
document. Requirements and model/code files have not been changed.

## Execution entry and baseline reconciliation

- Plan is Approved; all eight Review Record items are resolved.
- Approved baseline: `043d574970d28ff172f7261ea0392ddd14ae50a8`.
- Execution-entry HEAD: `24ba011d4f0a49c727a9b6c9cd28941f536891b4`.
- Working tree was clean on entry. `git diff --stat <baseline> HEAD -- .` lists only
  six supplied HLR revisions, sketch, plan, intake record, and workflow status.
  No model, requirements/ engineering documents, generated code, implementation,
  schedule or build configuration drift occurred. The approved impact analysis
  remains applicable; source baseline is not repinned.
- ChangeExec is a draft workflow in shared context; its defined execution and audited
  gate mechanics are being followed. W1 requirements ownership follows the developer's
  explicit instruction and approved RD-7 instead of agent-authored SysPlanAndReq.

## Review outcome

No new blocking defect was identified in the mode behavior of supplied revision _06.
COMMAND_INT/LONG offsets now match implementation; HLR-32 explicitly preserves
SECURE_COMMAND operation 7 denial. Threshold five, saturation twenty, frozen-input
suppression, D2 timeout, one-time reporting and reboot latching remain consistent
with the approved plan. R2U2 is an approved implementation allocation of HLR-30;
the HLR need not name the tool to express its required behavior.

The remaining W1 requirements task is reconciliation of the four existing engineering
documents below. Their current contents are still CR-01-era, so they must not be
used as competing authority for the new contracts. The approved plan requires review
of developer-supplied updates before dependent model/contract work.

## RF-1 — requirements/updated_reqs.md

1. Identify _06 as the source of truth, with a dated CR-02 provenance reference.
2. Make HLR-5, HLR-13, HLR-18 and HLR-22 forwarding conditional on Normal mode.
   HLR-13 must require source !=14550 AND destination !=14562 AND destination in
   the whitelist. Example: 14550 → 68 is rejected even in Normal mode.
3. Incorporate the new authoritative HLR-19–32 meanings using the mapping below.
   Keep the developer's authoritative IDs unchanged. Archive old derived meanings
   with explicit `CR-01:` qualification and retain compatible details using distinct
   derived/component IDs, rather than silently treating the same number as equivalent.
4. Preserve compatible existing definitions of bounded carriers, exactly one MAVLink
   frame, CRC/dialect checking, and legal v2 truncation as refinements of HLR-20/22.
   Source conflicts must be explicitly dispositioned; the baseline cannot override _06.
5. Scope empty-input/Recovery suppression to Ethernet outputs, leaving ErrorStatus
   publication active on every dispatch. Include HLR-32 in blacklist/count traceability.

### Suggested migration map (old identifiers are historical, not new HLR assignments)

| CR-01 HLR | Old meaning | CR-02 disposition |
|---|---|---|
| 19 | Exclusive Rx routing | Retain as a named derived Rx obligation under 5/13/15/18; authoritative 19 now means COMMAND_INT/LONG flash denial |
| 20 | Rx logging | Retain as a sketch-traced component obligation; authoritative 20 means malformed MAVLink rejection |
| 21 | MAVLink structural validity | Retain compatible details under 20/22; authoritative 21 means no Ethernet output on corresponding empty input |
| 22 | Allowed MAVLink forwarding | Revise in place to include Normal mode |
| 23 | Allow FTP | Retain as an allowed-message example under 22; authoritative 23 initializes ModeManager |
| 24 | Deny firmware activation | Split trace to authoritative 19 and 32; authoritative 24 enumerates modes |
| 25 | Malformed rejection | Trace to authoritative 20; authoritative 25 publishes ErrorStatus for count >=5 |
| 26 | No MAVLink input/fail closed | Trace Ethernet empty-input behavior to 21 and compatible invalid handling to 20; authoritative 26 publishes current mode |
| 27 | MAVLink logging | Retain sketch-traced reason logging; authoritative 27 enters/latches Recovery and publishes in the same dispatch |
| 28 | VMM integration | Retain compatible VMM component/interface obligations; authoritative 28 suppresses MAVLink Ethernet outputs in Recovery |
| 29 | Dispatch period | Retain as separately identified deployment refinement; authoritative 29 suppresses Rx Ethernet outputs in Recovery |
| 30 | Four-lane capacity | Retain as sketch-traced interface requirement; authoritative 30 is the mode-transition monitor |
| 31 | Separation of concerns | Retain as sketch-traced architecture constraint; authoritative 31 is the rejection counter |
| 32 | Tx proof non-regression | Retain as sketch-traced verification constraint; authoritative 32 is SECURE_COMMAND operation 7 denial |
| 33 | Implementation independence | Retain as a distinct sketch-traced constraint; _06 supplies no HLR-33 |

Retired TCP HLR-6 remains historical; active no-TCP behavior can trace to the exhaustive
allowed receive paths and HLR-15. Do not invent a new authoritative HLR-6.

## RF-2 — requirements/component-requirements.md

Update individual rows and the summary table, not just document headings:

- Rx: guard ARP/direct-UDP/MAVLink forwarding by Normal; adopt the strict UDP rule;
  add Recovery suppression covering all eight frame outputs (HLR-29).
- MAVLink: remap Valid/Malformed to HLR-20, NoInput to HLR-21, Allow/FTP to HLR-22,
  and DenyFlash to HLR-19/32. Limit output-validity invariants to frame outputs.
  Add count (31), final ErrorStatus publication (25), Recovery suppression (28), and
  R2U2 monitoring/reporting allocation (30).
- ModeManager: add initialization (23), two modes (24), per-dispatch publication and
  startup Normal behavior (26), and same-dispatch Recovery transition/publication
  latched until reboot (27).
- VMM: retain current behavior but replace misleading HLR-28 references with named
  derived integration obligations and CR provenance.
- Libraries/frozen components: replace old HLR-31/32/33 references using RF-1's map;
  preserve Tx non-regression and parser/network separation.

Suggested derived text to make the approved implementation choices traceable:

> MAVLinkFirewall shall continue classifying available input messages during Recovery.
> Messages rejected as malformed or blacklisted contribute once to HLR-31's count;
> messages suppressed solely because of Recovery do not contribute.

> The generated R2U2 monitor shall evaluate HLR-30 once per MAVLinkFirewall dispatch,
> using that dispatch's frozen Mode input and final ErrorStatus output. Reporting
> integration shall emit a timeout log during D2 when required and at most once per boot.

These suggestions record already approved RD-5/RD-8 choices. Exact generator syntax,
extension points, and runtime feasibility are implementation evidence to establish
in W1/W2; they are not claimed verified by adding text here.

## RF-3 — requirements/conops.md

Preserve the original §0 concept block and archived CR-01 provenance. Add dated CR-02
annotations and update affected sections:

- §1–2: introduce ModeManager and rejection-triggered Recovery. Replace the blanket
  promise that allowed receive traffic retains its behavior with Normal-mode behavior,
  noting strict UDP and Recovery suppression. Transmit policy remains unchanged.
- §5: add scenarios for fifth rejection, Recovery publication/observation, persistent
  Recovery until reboot, timeout reporting, and permitted traffic during Normal.
  Update OPS-001/003 success conditions and the old no-automated-recovery statement.
- §6–7: allocate counting, mode coordination and runtime monitoring; describe that
  Recovery suppresses inbound forwarding and does not repair a component or resume
  automatically. Approved scope does not claim cryptographic authentication.
- §8–9: reference the three approved waves; distinguish HLR-30's dispatch deadline
  from a wall-clock latency bound. Preserve custom.mk and frozen functionality constraints.
- Appendix A: add ModeManager and Normal/Recovery state concepts and relationships.

## RF-4 — requirements/data-dictionary.md

Add or revise these entries, retaining existing carrier layouts:

| Item | Suggested definition |
|---|---|
| Mode | Enum Normal/Recovery; initialized Normal; current sampled value used from each component's frozen input snapshot |
| ErrorStatus | Boolean initialized false; published every MAVLink dispatch after count updates; true iff count >=5 |
| RejectionCount | Initialized 0; aggregates malformed/blacklisted inputs across four lanes; one per message; saturates at 20 |
| D0/D1/D2 | MAVLink compute-dispatch indices relative to first true assertion; Recovery present in D2 snapshot is timely; otherwise one error log in D2 |
| Recovery persistence | ModeManager remains Recovery until system reboot, regardless of later false status |
| Direct UDP eligibility | Whitelisted destination plus source !=14550 and destination !=14562, in Normal mode |
| COMMAND_INT/LONG command | Payload-relative bytes 28–29; MAVLink v1 bytes 34–35 or v2 bytes 38–39; little-endian value 42650 denied |
| SECURE_COMMAND operation | Message ID 11004; payload-relative bytes 4–7 / MAVLink v2 bytes 14–17; little-endian value 7 denied |

Clarify the two existing entries both called “MAVLink payload length”: one is the
UDP-carried complete MAVLink message slice length, the other the transmitted MAVLink
payload-field length. This is naming cleanup, not a carrier change. Keep the distinction
between the conceptual IPv4 limit and the 1600-byte carrier bound.

## Handoff and completion check

The developer edits the four requirements documents. The agent then checks that:

1. _06 remains authoritative (or a newer supplied revision is explicitly identified).
2. Old HLR meanings no longer collide with active authoritative IDs.
3. Normal/Recovery, strict UDP, blacklist, counter, ErrorStatus and D2 rules agree
   across documents, including the already approved implementation allocations.
4. Source provenance, manual ownership, original concept text and non-impact
   constraints are preserved.

RF-1–RF-4 are pending developer updates. No additional design approval is requested;
the approved W1 sequence is waiting for developer-owned input. SysModeling and
dependent contract/codegen work have not started.
