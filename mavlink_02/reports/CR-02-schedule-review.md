# CR-02 Wave 3 — schedule review

Date: 2026-09-23. Draft SysSchedDef structural review complete; audited boundary
pending. Timing and deployment acceptance are not complete.

The developer directed use of the old domain XML format. Retained
`<domain_schedule><domain name="..." length="..."/></domain_schedule>` and every
existing slot length. Added domain 7 for ModeManager before ArduPilot, with an
intervening pacer as for every other component. Application order is:

ModeManager (7) → ArduPilot (2) → Tx (3) → driver (4) → Rx (5) → MAVLink (6).

ModeManager length is 10000, proportional to the unchanged firewall length 30000.
Pacer slots remain 3000 and ArduPilot remains 60000. Total is 208000 legacy length
units. Model budgets remain manager 100 ms, ArduPilot 600 ms, other components
300 ms, and generator pacer budget 30 ms. Their sum is the model's 2080 ms frame.
The relationship between the manually scaled legacy lengths and actual target time
is not yet established. The earlier new-format 2080000-us check and commentary
are superseded; do not infer that the legacy schedule has that duration. Likewise,
no measured 1000-ms periodic dispatch or WCET conformance is claimed.

Local generator history (codegen commit 4cb5bf60) confirms the transition from
legacy domain name/length entries to named domains and explicit schedule-entry
units. The installed SDK 2.2.0-dev rejects the new domains wrapper. A fresh probe of
the merged system using the legacy format gets past XML parsing and now stops at
missing seL4_ModeManager_ModeManager_MON.elf, as expected before custom.mk integration.
This establishes schema acceptance, not a successful loader build. The SDK's ZCU102
debug kernel advertises 256 domains and 100 schedule entries; this schedule uses
seven distinct domains and twelve entries. No generated system/bridge source changed.

The cyclic order preserves existing traffic dependencies: ArduPilot→Tx→driver→Rx→
MAVLink→ArduPilot on the following cycle. For the control loop, assuming each component
finishes and publishes within its slot, MAVLink's D0 true ErrorStatus is available
to the next cycle's first ModeManager dispatch. Manager publishes Recovery before
that cycle's Rx and MAVLink dispatches, so MAVLink observes it at D1. Input freezing
at each consumer dispatch, not producer publication alone, determines observation.
Generated C queues preserve the sampled value; MAVLink monitor peek precedes the
application getter and monitor evaluation follows final ErrorStatus publication.
This is a source/order argument; deployed dispatch traces, actual monitor cost and
physical one-time timeout reporting remain required Wave 3 evidence.

Checks: CR-02-schedule-checks.txt (legacy ordering/length total) and
CR-02-schedule-sdk-check.txt (SDK parser proceeds to missing manager image).
The source schedule remains editable and preserved by codegen. Keep the legacy
format on future runs as directed. No codegen was invoked here.

Next after boundary approval: reconcile custom.mk manager images/control types and
R2U2 prerequisites; build with the installed legacy SDK; determine legacy timing
units and close period/budget questions with target evidence. No newer SDK path is
needed for the resolved XML-format issue. The previously asked SDK-path question is
superseded by the developer's legacy-format instruction.
