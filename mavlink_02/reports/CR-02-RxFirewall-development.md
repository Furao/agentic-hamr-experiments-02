# CR-02 Wave 2 — RxFirewall development

Date: 2026-09-23. Profile: audited. Status: coverage approved; verification 28/0; AP2 pending.
ModeManager completion and continuation to RxFirewall approved by developer.
Authority: developer-owned _07 requirements and approved Rx GUMBO contracts.

## Implementation

Compute reads current_mode exactly once before processing the four lanes. Every
present input is consumed; Recovery suppresses both corresponding outputs and
silently drops the frame per developer direction. Normal mode retains exclusive lane-preserving routing.

Both UDP routes now share the executable bounded-length check: UDP length >= 8,
IPv4 total length >= 20 and <= 1586, and UDP length equals IPv4 total length minus
20. The verified firewall_core parser continues to validate Ethernet/IP structure.
Subtraction is guarded; arithmetic cannot authorize carrier overflow. The exact
14550-to-14562 pair routes a preserved carrier with payload offset 42. Direct UDP
requires source != 14550, destination != 14562 and destination 68. ARP handling is
unchanged; other traffic drops. No executable GUMBOX calls implement routing.

Network rejection diagnostics are retained, and MAVLink-route trace
logging is explicit. Test-only capture in the existing external logging adapters
checks diagnostic calls without adding production state or new trust escapes.
No model, generated contracts, shared-core implementation or requirements changed.
Existing generated code matches the approved model; the prior clean tipe result
remains applicable. No regeneration was necessary.

## Tests and coverage

**12 tests passed, 0 failed**, including 100-case generated initialization and
compute PropTests. Legacy manual helpers now supply the generated Mode parameter.

- Both modes on all four lanes; direct and MAVLink routing, empty inputs and drops.
- Independent output assertions for lane correspondence, exclusivity, unchanged
  frame and payload bounds; source 14550/destination 68 is rejected.
- Empty UDP payloads, maximum carrier-fitting payload, one byte past carrier,
  inconsistent/short/oversized IPv4 and UDP lengths and unsupported IPv4 options.
- Initialization, ARP, IPv6, TCP, malformed frames and unhandled notifications.
- Captured rejection and MAVLink-route diagnostics, and absence of Recovery diagnostics.
- 480 constructed oracle combinations across lanes, modes, input partitions and
  candidate outputs; compute and top-level postcondition independently reject
  injection, duplicates, changed frame bytes, wrong offsets and wrong lengths.

An initial draft test incorrectly expected a fragmentation flag alone to cause a
drop. _07 and the approved predicate do not impose that rule, so that unrequested
test expectation was removed; implementation/contracts were not weakened to pass it.

Final coverage uses an isolated Cargo target and fresh quiet-profiles directory,
excluding historical binaries and failed-run profiles. grcov results:

| Source | Covered executable lines |
|---|---|
| Rx application | 118/118 (100%) |
| Rx GUMBOX | 382/382 (100%) |

LLVM reports no branch counters (BRF=0); no numerical branch percentage is claimed.
Manual partitions exercise both modes, present/absent lanes, each route and drop
class, bounds/port decisions, and accepted/rejected oracle outcomes. Dead legacy
`cfg(any())` examples and seL4-only log backend calls are excluded from host executable
coverage; target backend behavior remains W3 evidence.

Evidence: `CR-02-RxFirewall-tests.txt`, `CR-02-RxFirewall-coverage.lcov`.
HTML: component `target/cr02-current/report/index.html`.
Command: RUSTC_BOOTSTRAP=1, CARGO_TARGET_DIR=target/cr02-current,
CARGO_INCREMENTAL=0, RUSTFLAGS=-Cinstrument-coverage, absolute LLVM_PROFILE_FILE
under target/cr02-current/quiet-profiles, then `cargo test --offline`.

## Next gate

CompDev steps 1–5 complete. Await AP1 approval of tests and coverage, including the
branch-counter limitation. Verus verification follows that approval; no Rx proof
success for this implementation is claimed yet. The three existing external bodies
remain platform logging adapters. MAVLinkFirewall implementation follows Rx completion.

Developer follow-up: removed per-frame Recovery suppression logs to avoid excessive output. Tests now check that Recovery emits no such diagnostics. This instruction supersedes the per-message Recovery-reason logging portion of LLR-18; requirements remain developer-owned for manual reconciliation. Preserve this direction when implementing MAVLinkFirewall. Rerun: 12/12 tests pass.

## Verification iteration and sign-off

Developer approved the updated coverage after removing Recovery logs. Target
`make verus` initially reported 27 verified and one failed function: initialize
could not prove its empty-output postconditions for an arbitrary incoming API.
The generated initialization signature lacks the empty-event-output preconditions
that the compute signature already carries.

Added explicit requires clauses for all eight initially empty event outputs outside
HAMR-managed markers in the editable initialize signature. Generated `init_api()`
constructs all eight ghost outputs as None, and test initialization clears the
corresponding event slots. The application sends nothing during initialize.
This is an explicit platform-startup precondition: proof does not cover arbitrary
reinitialization with queued outputs. No assume/admit, ghost-output reset, external
body, runtime behavior change or weakening of postconditions was introduced.
The caller/platform establishment of this condition is inspected rather than proved
end-to-end through the extern-C lifecycle. Retain this qualification in the trust
boundary and reassess the local precondition when HAMR adds initialization framing.

Rerun: **28 verified, 0 errors**, exit 0, aarch64-unknown-none, Verus
0.2026.08.09.92f466f, Rust 1.97.1, rlimit 100 and SMT seed 7. No routing proof
hints or further implementation changes were needed. The three existing external
bodies remain info, trace and warn_channel logging adapters. Network parser and
routing/carrier helpers are verified, not external bodies.

Tests rerun after the signature change: **12/12 pass**. Fresh proof-profiles coverage
remains application **118/118**, GUMBOX **382/382**. The prior approved executable
behavior is unchanged; evidence files refreshed. See CR-02-RxFirewall-verification.txt.
AP2 and component completion await review of the proof result and startup precondition.
MAVLinkFirewall is the next Wave 2 component.
