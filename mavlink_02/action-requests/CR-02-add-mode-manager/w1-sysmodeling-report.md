# CR-02 Wave 1 — SysModeling review

Date: 2026-09-22. Source: `Open_Platform_HLRs_26_09_22_07.md` and
`w1-requirements-planning-07.md`. Profile: audited.
Status: SysModeling.AP1 and audited sub-workflow boundary approved by the developer on 2026-09-22.

## Architecture changes

| Element | Model choice | Requirements |
|---|---|---|
| OperatingMode | Enum Normal, Recovery; Normal first | HLR-24/26; LLR-10/11 |
| ModeManager | One Rust periodic thread wrapped by ModeManager_seL4 | HLR-23/24/26/27; LLR-11/14 |
| ErrorStatus | Boolean DataPort from MAVLinkFirewall to ModeManager, with wrapper delegation | HLR-25/27; LLR-11/12 |
| Mode to Rx | OperatingMode DataPort from ModeManager.mode_to_rx to RxFirewall.current_mode | HLR-26/29; LLR-10/11 |
| Mode to MAVLink | OperatingMode DataPort from ModeManager.mode_to_mavlink to MAVLinkFirewall.current_mode | HLR-26/28/30; LLR-10/11/15 |
| Deployment | ModeManager domain 7; existing component domains 2–6 retained; Max_Domain 8 | LLR-20 |
| Manager timing | Period 1000 ms; configured Compute_Execution_Time 100–100 ms | LLR-20; modeling choice for unspecified manager period |

ModeManager's 1000 ms period aligns with the existing Rx/MAVLink period. Its
100 ms compute budget follows the supplied preference; neither constitutes measured
execution time. Stack size follows the existing Rust firewall allocation of 1 MiB.
The manager is declared first among application instances; actual first-in-schedule
placement is a W3 obligation, not a consequence of declaration or domain numbering.

Both Platform::ZCU102_Impl and its mock inherit the extended seL4 assembly. Processor
binding remains unchanged. Ethernet carriers, four-lane interfaces, and existing
system connections remain intact. ModeManager gets two separate mode outputs so
both publications can be constrained to its post-state in the following contract work.

## Requirement homes and following work

- Rx owns Normal-mode network routing, strict UDP and Recovery suppression; its
  new current_mode port supplies the architectural input for HLR-29/LLR-10.
- MAVLink owns count, final ErrorStatus publication and mode-gated forwarding.
  HLR-30/LLR-15–17 are allocated to an embedded generated R2U2 monitor observing
  frozen current_mode and final error_status; no extra monitor process is introduced.
- ModeManager owns initial mode, same-dispatch transition/publication and reboot latch.
- Existing Tx, driver, VMM and library architecture retain the requirements allocated
  in the _07 requirements planning report.

SysModeling establishes the structural homes only. Existing GUMBO blocks are preserved;
they are not yet the CR-02 contracts. CompGUMBOSpec must next introduce Normal guards,
counter/latch states, explicit Normal/false/zero initialization, mode-output equality,
updated HLR/LLR traceability and the R2U2 monitor specification. Normal's enum order
alone is not evidence that generated startup publication satisfies LLR-11.

The static schedule and Platform Frame_Period remain unchanged at this step. W3 must
reconcile slot units, frame capacity, the additional manager/pacer slots and actual
input-freeze/publication/monitor timing. No claim of D2 schedulability or R2U2 verdict
feasibility is made by this type-check.

## Validation

Command, executed from `sysmlv2/open_platform/`:

```text
/home/robertvanvossen/tools/Sireum/bin/sireum hamr sysml tipe --sourcepath ../aadl-lib:. Platform.sysml
```

Result: exit 0, `Well-formed!` (first attempt).

Additional structural checks confirmed existing GUMBO blocks are unchanged, existing
Ethernet EventDataPort declarations are unchanged, and each new system-level mode/error
connection occurs exactly once. `git diff --check` passed. Five model files changed;
no code generation, application edits, schedule edits, or requirements edits occurred.

## Audit handoff

SysModeling.AP1 asks whether each requirement has an architectural home and each
component/connection traces to a requirement. This report and the _07 allocation map
provide that review surface. The developer approved the audited model gate and
ChangeExec's SysModeling sub-workflow boundary on 2026-09-22.

Approval update: developer replied “approved”; SysModeling is complete and RxFirewall CompGUMBOSpec follows.
