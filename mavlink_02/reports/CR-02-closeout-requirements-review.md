# CR-02 closeout requirements review

Date: 2026-09-23. Wave 3 approved by developer. Current source of truth:
`action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md`.
Requirements remain developer-owned under change-plan RD-7 and §7. The developer
subsequently authorized retirement notices on the four legacy documents. Their
historical bodies and the supplied current requirements are unchanged.

**Disposition: requirements reconciliation complete.** The developer supplied _02
for LLR-18 and explicitly retired the four legacy CR-01 documents on 2026-09-23.
The initial observations below are retained as review history, not outstanding work.

## Initial reconciliation review (now resolved)

| Item | Observation | Suggested developer update |
|---|---|---|
| LLR-18 reason logging — resolved | Developer supplied _26_09_23_02 with exactly the suggested replacement below. The requirement now matches the approved implementation. | No further logging reconciliation required. |
| `requirements/conops.md` | Describes CR-01 and excludes Tx/driver changes; lacks the implemented mode-control scenarios. | Preserve §0 verbatim. Add a dated CR-02 annotation citing the change request/sketch: Normal startup, first error threshold, same-dispatch publication, frozen Recovery suppression, reboot-only return, and the accepted High/Open timeout finding. Include the approved Tx bounds and defensive driver scope amendment. |
| `requirements/updated_reqs.md` | CR-01 HLR identifiers collide with current identities; direct UDP says "unless both" ports match instead of the stricter current exclusions. | Mark it explicitly as a superseded CR-01 baseline or reconcile it to the supplied current source. Use the historical-ID mapping in `w1-requirements-planning-07.md`; old HLR-30/31/32 are not current HLR-30/31/32. |
| `requirements/component-requirements.md` | CR-01 allocations, no ModeManager, old direct UDP rule, and frozen Tx/driver claims. | Add current HLR/LLR allocations from the W1 planning report, refine with the 1586-byte bounds amendment, and identify the approved driver bounds check. Alternatively explicitly retire this legacy allocation in favor of a named current replacement. |
| `requirements/data-dictionary.md` | Still CR-01; includes a conceptual 9000-byte IPv4 bound and omits mode, error status and counter definitions. | Record IPv4 maximum 1586 within the 1600-byte Ethernet carrier; Normal/Recovery, initialized false ErrorStatus, counter 0–20 with threshold 5, D0/D1/D2 frozen observations, and the restored manager 100 ms / firewalls 300 ms configured budgets. Record the actual legacy schedule separately without treating its raw units as measured physical time. |

The legacy documents remain historical context; they do not override the supplied
current requirements. The developer's subsequent retirement instruction resolves
the back-propagation obligation through explicit retirement rather than rewriting
these historical documents.

## LLR-18 wording adopted in _26_09_23_02

> RxFirewall and MAVLinkFirewall shall retain diagnostic logging that identifies
> relevant network rejection, malformed MAVLink messages, and blacklisted firmware
> activation commands. RxFirewall shall retain its MAVLink-routing diagnostics.
> Neither firewall shall emit a per-message diagnostic solely because forwarding
> is suppressed by Recovery Mode. Logging shall not change routing decisions.
> ModeManager shall log its transition from Normal Mode to Recovery Mode once when
> that transition occurs. Repeated mode publications shall not repeat that message.
> Rejection and mode-transition diagnostics are distinct from HLR-30's single
> timeout error per system boot.

This wording records both developer-directed logging changes. It does not remove
malformed/blacklist diagnostics or counting in Recovery, or change the D2 deadline.

## Acceptance and remaining closeout

All three waves are approved. Wave 3 approval includes the full target build and
fresh verification in `CR-02-full-verify-build.md`, manual testing accepted with
High/Open `CR-02-HW-01`, and the documented driver/tool/coverage limitations.
The HAMR reporting fix is now generated directly; post-codegen patching is retired.

Active-source logging reconciliation and legacy-document retirement are complete.
Final cross-crate
test consolidation and the required CR-02 change report/completion review remain
separate closeout steps. The latest full verification/build evidence can be reused
while its source remains unchanged; no redundant verification rerun is required
solely because Wave 3 approval was recorded.

## Revision _26_09_23_02 review

The developer reported requirements reconciliation and supplied _02. A complete
diff against _01 confirms that only LLR-18 changed, adopting the wording above.
ModeManager's transition guard emits its diagnostic once, the firewalls omit
mode-only suppression diagnostics, and the independent D2 timeout reporter remains
unchanged. Existing logging tests and the fresh full verification/build remain
applicable to unchanged code. No regeneration or redundant validation run is needed.

At the time of the _02 review, no edits were found in the four legacy CR-01
documents. The developer subsequently instructed their retirement as recorded below.

## Developer-directed retirement — 2026-09-23

The developer explicitly instructed: "retire the legacy documents". Added visible
retirement notices to `requirements/conops.md`, `requirements/updated_reqs.md`,
`requirements/component-requirements.md` and `requirements/data-dictionary.md`.
Each notice links to authoritative _26_09_23_02 and the CR-02 sketch/approved plan,
and qualifies the historical requirement identifiers as CR-01. ConOps metadata now
also says retired. Original document bodies, including the verbatim developer
concept in ConOps §0, were checked for exact preservation. Files remain at their
existing paths so historical links continue to work.

This explicit retirement supersedes the plan's proposed updates to these four
documents. ChangeExec.3 is complete. No current requirement, model, code, generated
output or build artifact changed. CR-02-HW-01 remains High/Open; final test
consolidation, change report and completion approval remain separate steps.

Reviewed _02 SHA-256: `feaf8facf23e0a22a1ae99ac17a9447810d45f9db43b5da3953080b9ac125646`.

## Final disposition — 2026-09-23

Final test consolidation and the change report are complete. The developer approved
ChangeExec.AP2; CR-02 is complete. Earlier pending closeout statements above are
historical. CR-02-HW-01 remains High/Open, explicitly deferred to a future CR.
