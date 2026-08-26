# Open Platform Updated System Requirements – CR-01

This document is the evolving requirements baseline for CR-01. It preserves the
`RC_INSPECTA_00-HLR-*` naming scheme from `manual_reqs.md`; that source remains
unchanged. Requirements are evaluated independently for each of the four queue-size-1
lanes unless stated otherwise.

## Baseline requirements retained without behavioral change

| ID | Requirement |
|---|---|
| RC_INSPECTA_00-HLR-5 | RxFirewall shall copy a well-formed ARP frame unchanged to the corresponding direct VMM output. ARP validity remains defined by `manual_reqs.md`. |
| RC_INSPECTA_00-HLR-7 | TxFirewall shall copy a well-formed ARP frame and correct size to its corresponding driver output. |
| RC_INSPECTA_00-HLR-12 | TxFirewall shall copy a well-formed IPv4 frame and correct size to its corresponding driver output. |
| RC_INSPECTA_00-HLR-14 | TxFirewall shall emit no output for an input not permitted by its retained requirements. |
| RC_INSPECTA_00-HLR-16 | TxFirewall shall emit no output on a lane with no input event. |

## Revised and retired RxFirewall requirements

### RC_INSPECTA_00-HLR-6 – Retired TCP forwarding

The former TCP destination-port-5760 forwarding requirement is retired by CR-01.
RxFirewall shall not forward TCP traffic on either output path.

### RC_INSPECTA_00-HLR-13 – Direct allowed UDP traffic

For a well-formed IPv4 UDP frame allowed by the retained UDP whitelist (including
destination port 68), RxFirewall shall copy the frame unchanged to the corresponding
direct VMM output unless both source port is 14550 and destination port is 14562.

### RC_INSPECTA_00-HLR-15 – Disallowed receive frame

RxFirewall shall emit neither a direct-VMM output nor a MAVLinkFirewall output for an
input that is neither valid ARP, retained allowed UDP, nor qualifying ArduPilot UDP.

### RC_INSPECTA_00-HLR-17 – No receive input

When a lane has no RxFirewall input event, RxFirewall shall emit no event on either
corresponding output path.

## New RxFirewall requirements

### RC_INSPECTA_00-HLR-18 – Route ArduPilot UDP to MAVLinkFirewall

For a well-formed IPv4 UDP frame whose source port is 14550 and destination port is
14562, RxFirewall shall send a `MAVLinkUDPMessage_Impl` on the corresponding
MAVLinkFirewall output and shall emit no direct-VMM output on that lane. The carrier
shall preserve the original frame and provide the validated UDP payload offset and
payload length.

### RC_INSPECTA_00-HLR-19 – Exclusive routing

For one RxFirewall input event, at most one corresponding output path—direct VMM or
MAVLinkFirewall—shall contain an event.

### RC_INSPECTA_00-HLR-20 – RxFirewall decision logging

RxFirewall shall log every dropped input and every ArduPilot-routing decision with a
reason that distinguishes malformed/disallowed networking traffic, TCP rejection, and
MAVLink-path routing. Logging shall not alter routing behavior.

## MAVLinkFirewall requirements

### RC_INSPECTA_00-HLR-21 – MAVLink v1/v2 structural validity

For a qualifying `MAVLinkUDPMessage_Impl`, MAVLinkFirewall shall first require
`payload_offset + payload_length` to remain within the preserved frame and the
metadata to agree with the validated IPv4/UDP layout established by RxFirewall. It
shall accept only when that bounded payload is exactly one complete MAVLink v1 or v2
frame. Validity includes the version magic,
payload length, complete header and payload, message-ID width, checksum including the
dialect CRC extra, supported v2 incompatibility flags, complete optional signature
when indicated, and no truncation or trailing bytes. Message metadata shall cover the
bundled ArduPilotMega, common, standard, and minimal dialect definitions.

### RC_INSPECTA_00-HLR-22 – Forward allowed MAVLink

When HLR-21 holds and no deny rule holds, MAVLinkFirewall shall copy the original
Ethernet frame unchanged to the corresponding VMM output.

### RC_INSPECTA_00-HLR-23 – Allow file-transfer protocol

A structurally valid MAVLink `FILE_TRANSFER_PROTOCOL` message shall not be denied
solely because its message ID or payload implements file transfer.

### RC_INSPECTA_00-HLR-24 – Deny firmware-flash activation

MAVLinkFirewall shall emit no output for a structurally valid command envelope that
requests `MAV_CMD_FLASH_BOOTLOADER` (command value 42650) or secure-command
flash-bootloader operation 7. This rule takes precedence over HLR-22.

### RC_INSPECTA_00-HLR-25 – Reject malformed MAVLink

If HLR-21 does not hold, MAVLinkFirewall shall emit no output for the corresponding
lane.

### RC_INSPECTA_00-HLR-26 – No MAVLink input / fail closed

With no input event, or when processing cannot complete safely, MAVLinkFirewall shall
emit no corresponding output. No bypass path is permitted.

### RC_INSPECTA_00-HLR-27 – MAVLink decision logging

MAVLinkFirewall shall log every rejected input with a reason distinguishing structural
malformation, unsupported framing feature, checksum failure, and denied firmware
activation. Logging shall not expose a bypass or alter forwarding.

## Integration, timing, and capacity requirements

### RC_INSPECTA_00-HLR-28 – VMM receive integration

For each of four lanes, the VMM shall accept frames from both the direct RxFirewall
path and the MAVLinkFirewall path and inject accepted frames into the existing
virtio-net receive path without changing the transmit route.

### RC_INSPECTA_00-HLR-29 – Dispatch period

MAVLinkFirewall shall use the same 1000 ms periodic dispatch and 300 ms configured
compute-execution time as RxFirewall. CR-01 adds no end-to-end latency constraint
beyond execution in the configured static-schedule order.

### RC_INSPECTA_00-HLR-30 – Capacity

MAVLinkFirewall shall provide four corresponding input/output event-data lanes, each
with queue size 1, matching RxFirewall capacity. Lane identity shall be preserved.

### RC_INSPECTA_00-HLR-31 – Separation of concerns

RxFirewall and `firewall_core` shall classify Ethernet/IP/UDP without parsing MAVLink
and shall produce validated payload offset/length metadata. MAVLinkFirewall and its
MAVLink policy core shall parse only the indicated bounded payload and shall not derive
or interpret Ethernet/IP/UDP headers.

### RC_INSPECTA_00-HLR-32 – TxFirewall verification non-regression

Changes to shared `firewall_core` shall require no TxFirewall model, contract, or
application-code change and shall preserve successful TxFirewall Verus verification.

### RC_INSPECTA_00-HLR-33 – Implementation independence

Application behavior shall be implemented directly and shall not delegate execution
to code generated from GUMBO/GUMBOX contracts.

## Verification and acceptance

- Application logic and GUMBOX oracles for changed/added components shall achieve full
  branch coverage.
- RxFirewall and MAVLinkFirewall shall verify with Verus; TxFirewall shall re-verify
  against the changed shared library; the final cross-crate verification pass shall
  succeed.
- Manual ZCU102 tests shall cover allowed MAVLink, valid file transfer, both denied
  firmware activation forms, malformed v1/v2 frames, TCP, retained direct traffic,
  no-input/fail-closed behavior, and all four lanes including bursts.

## Traceability

All requirements above trace to CR-01 and the approved change plan. Component-level
allocation is defined in `component-requirements.md`; terms and field ranges are in
`data-dictionary.md`.
