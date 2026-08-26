---
title: "ConOps – Open Platform MAVLink Firewall"
version: "0.1"
date: "2026-08-26"
authors: "Open Platform development team"
status: "Draft"
security_classification: "Project internal"
---

# ConOps – Open Platform MAVLink Firewall

## 0. Developer Concept (Verbatim)

The complete developer concept is archived at
`action-requests/CR-01-add-mavlink-firewall/mavlink-firewall-change-sketch.md`.
The mission statement supplied there is reproduced verbatim:

> # Reduce potential cyber attacks by adding a mavlink firewall that can blacklist certain mavlink commands/messages
>
> ## Goals for updated system functionality
> Increase the cyber resilience of the system by adding a protocol specific firewall which can reject specific or malformed mavlink messages before they can make it to the Ardupilot software running in the VM. THe mavlink firewall should reject any messages that can update firmware.

The sketch, bundled MAVLink references, and the audited ChangeScope/SystemConcept
answers are the provenance anchors for this change. The sketch was amended on
2026-08-26 to complete the `firewall_core`/TxFirewall verification constraint; the
pre-amendment text remains at commit `7a99f23e`.

## 1. Purpose and Scope

The system reduces the attack surface of a virtualized UAS by inserting a
protocol-aware, fail-closed MAVLink inspection component into the inbound ArduPilot
network path. It separates Ethernet/IP/UDP classification from MAVLink parsing and
blocks malformed frames and firmware-flash activation commands before they reach the
ArduPilot VM.

In scope:

- Receive-side classification of ArduPilot UDP traffic (source 14550, destination 14562).
- MAVLink v1/v2 structural validation using the bundled ArduPilotMega dialect sources.
- Allowing valid `FILE_TRANSFER_PROTOCOL` messages while denying firmware-flash
  activation commands.
- RxFirewall, a new MAVLinkFirewall, VMM receive integration, scheduling, logging,
  verification, and ZCU102 hardware validation.

Out of scope:

- Changes to LowLevelEthernetDriver or TxFirewall model, contracts, or application code.
- MAVLink cryptographic authentication or semantic authorization beyond the approved
  blacklist.
- Transmit-path policy changes, new safety/regulatory modes, or recovery after a
  firewall component failure.

## 2. System Overview

Inbound Ethernet frames flow from LowLevelEthernetDriver to RxFirewall. RxFirewall
drops TCP; sends permitted traffic not addressed to ArduPilot directly to the VMM;
and sends UDP source port 14550/destination port 14562 through MAVLinkFirewall.
MAVLinkFirewall validates exactly one MAVLink frame in the UDP payload and either
forwards the original packet to the VMM or emits no output and logs the rejection.
The VMM injects accepted traffic into the Linux/ArduPilot guest through virtio-net.
The outbound VMM → TxFirewall → LowLevelEthernetDriver route is unchanged.

Primary objectives:

- No malformed MAVLink frame or approved firmware-flash activation command reaches
  the ArduPilot VM through the inspected inbound path.
- Valid allowed MAVLink, including `FILE_TRANSFER_PROTOCOL`, reaches the VMM unchanged.
- Existing allowed non-ArduPilot receive traffic and the complete transmit path retain
  their behavior.
- TxFirewall continues to verify after the shared `firewall_core` change.

Key stakeholders are the UAS operator and the connected ground station.

## 3. Operational Environment

The system runs on a ZCU102 using seL4, Microkit, HAMR-generated integration code,
Rust components, and a manually maintained C VMM hosting Linux and ArduPilot. The
ground station exchanges MAVLink over UDP with ArduPilot. Each logical component
connection is represented by four event-data port lanes, each with queue size 1.

Security boundary: untrusted inbound Ethernet/UDP/MAVLink traffic is outside the
trusted ArduPilot VM boundary until it passes RxFirewall and MAVLinkFirewall policy.
The primary threats are malformed-parser inputs and remote firmware modification.
No additional regulatory or safety standard was specified for CR-01.

## 4. Users and External Actors

- **UAS operator (DM-001):** operates the aircraft through the ground station and
  expects allowed control/telemetry traffic to continue.
- **Ground station (DM-002):** external network peer sending MAVLink UDP traffic from
  port 14550 to ArduPilot port 14562, including permitted file transfer.
- **Ethernet environment (DM-003):** source of other network traffic evaluated by the
  existing RxFirewall policy.

No direct human interaction with the firewall components is introduced.

## 5. Operational Scenarios

### OPS-001 – Allowed MAVLink traffic

1. Ground station DM-002 sends one well-formed MAVLink v1/v2 message in a qualifying
   UDP packet.
2. RxFirewall DM-004 routes the packet to MAVLinkFirewall DM-005.
3. RxFirewall supplies the preserved Ethernet frame plus validated UDP-payload offset
   and length metadata. MAVLinkFirewall validates only that bounded payload and policy,
   then forwards the original frame unchanged.
4. VMM/ArduPilot DM-006 receives the packet.

Success: allowed traffic, including valid `FILE_TRANSFER_PROTOCOL`, reaches ArduPilot.

### OPS-002 – Malformed or denied MAVLink traffic

1. A qualifying UDP payload is malformed or carries an approved firmware-flash
   activation command.
2. MAVLinkFirewall emits no output for that lane and logs why it dropped the message.

Success: the prohibited input does not reach ArduPilot; failure behavior is fail-closed.

### OPS-003 – Non-ArduPilot receive traffic

RxFirewall applies its updated network policy. Still-permitted non-ArduPilot traffic
goes directly to the VMM; TCP and otherwise disallowed traffic is dropped and logged.

### OPS-004 – Component failure

If MAVLinkFirewall cannot validate or process an input, it emits no output. No
fail-open bypass or automated recovery mode is provided.

### OPS-005 – Maintenance and hardware validation

Developers deploy to ZCU102 and exercise allowed, malformed, denied, TCP,
non-ArduPilot UDP, and four-lane/burst cases. No separate operational maintenance mode
is introduced.

## 6. System Functions

| ID | Function | Description | Security relevance |
|---|---|---|---|
| FUNC-1 | Network classification | Parse Ethernet/IPv4/UDP and distinguish the ArduPilot flow. | High |
| FUNC-1A | Payload boundary handoff | Supply a validated payload offset/length with the preserved frame. | High |
| FUNC-2 | MAVLink validation | Validate exactly one MAVLink v1/v2 frame against framing and dialect metadata. | High |
| FUNC-3 | Firmware policy | Allow file transfer but deny approved firmware-flash activation commands. | High |
| FUNC-4 | Fail-closed routing | Emit no output for invalid, denied, or unprocessable input. | High |
| FUNC-5 | Audit logging | Record when and why RxFirewall or MAVLinkFirewall drops traffic. | Medium |
| FUNC-6 | VMM delivery | Deliver allowed direct and MAVLink-inspected traffic to virtio-net. | High |

## 7. Safety and Security Operational Concepts

The security posture is deny-on-invalid and fail-closed. Networking and MAVLink
concerns remain in separate components/libraries. Generated GUMBOX executable logic
is not used as the application implementation. TxFirewall is a protected regression
boundary: shared-library changes must preserve its verification result.

No extra safety standard, degraded mode, emergency mode, or regulatory constraint was
identified. Loss of allowed traffic is an availability risk addressed through tests,
verification, logging, and hardware validation; bypassing inspection is not an
accepted recovery response.

## 8. Impacts and Transition

Transition occurs through four reviewed waves: requirements/model/contracts and code
generation; component implementation and verification; VMM/schedule integration; and
ZCU102 hardware/final verification. Existing traffic is not cut over until each wave's
gate passes. Rollback is by reverting the CR-01 implementation commits and restoring
the prior generated system/schedule together.

## 9. Assumptions, Constraints, and Open Issues

Constraints:

1. MAVLinkFirewall uses the same 1000 ms dispatch period and 300 ms configured
   compute-execution time as RxFirewall; no separate end-to-end latency bound applies.
2. Capacity matches RxFirewall: four event-data lanes with queue size 1 per lane.
3. VMM remains manually maintained C code under `hamr/microkit/vmm/`.
4. Make invocations use `SYSTEM_MAKEFILE=custom.mk`.
5. `requirements/manual_reqs.md` remains unchanged.
6. LowLevelEthernetDriver and TxFirewall source/model/contracts remain unchanged.
7. MAVLinkFirewall trusts only payload-boundary metadata established by RxFirewall
   contracts and never derives Ethernet/IP/UDP offsets itself.

Open issues: none at requirements drafting. Newly discovered firmware activation
paths or timing infeasibility require plan review rather than silent policy expansion.

## 10. References and Traceability

- CR-01 sketch and bundled MAVLink packet-serialization/dialect files.
- Approved `action-requests/CR-01-add-mavlink-firewall/change-plan.md`.
- `requirements/updated_reqs.md`, `component-requirements.md`, and
  `data-dictionary.md`.

## Appendix A: Domain Model

| ID | Type | Name | Definition | Relationships |
|---|---|---|---|---|
| DM-001 | Entity | UAS operator | Human stakeholder operating the UAS. | uses DM-002 |
| DM-002 | Entity | Ground station | External MAVLink UDP peer. | sends to DM-004; communicates with DM-006 |
| DM-003 | Entity | Ethernet environment | Other inbound/outbound network peers and traffic. | sends to driver/RxFirewall |
| DM-004 | Entity | RxFirewall | Ethernet/IP/UDP classifier and receive policy enforcement point. | routes qualifying traffic to DM-005 |
| DM-005 | Entity | MAVLinkFirewall | MAVLink validation and blacklist enforcement point operating on a bounded payload supplied by DM-004. | forwards the preserved allowed frame to DM-006 |
| DM-006 | Entity | VMM/ArduPilot | Linux guest and flight software receiving accepted traffic. | receives from DM-004 and DM-005 |
| DM-007 | Condition | InvalidOrDenied | Frame is malformed, prohibited, or cannot be processed. | triggers fail-closed no-output behavior |
