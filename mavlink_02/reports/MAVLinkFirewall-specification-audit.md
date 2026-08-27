# MAVLinkFirewall GUMBO Specification Audit Report

**Component**: `MAVLinkFirewall` (MAVLink validation and command firewall)  
**Model File**: `sysmlv2/open_platform/open_platform_Software.sysml`, lines 124–201  
**Date**: 2026-08-27
**Auditor**: HAMR GUMBO Contract Audit  
**Classification**: Security-Critical

## 1. Executive Summary

The MAVLinkFirewall contract was checked against anti-patterns AP-1 through AP-9.
No catalog findings remain. For each of four lanes, the compute contract partitions
present inputs into allowed, denied firmware-flash command, or invalid carrier/frame,
and separately constrains no-input behavior. An allowed output must equal the complete
input carrier, preserving the Ethernet frame and the payload boundary established by
RxFirewall. Integration guarantees export the context-independent `mavlink_allowed`
invariant on every output lane.

The component-local `mavlink_frame_valid` and
`mavlink_firmware_flash_command` predicates state the MAVLink security boundary
without duplicating variable-length wire offsets in GUMBO. Their developer Verus
definitions are now concrete predicates over `mavlink_core`'s verified framing,
dialect-metadata, X.25 checksum, and command-field specifications. The
component-local `mavlink_allowed` function composes those predicates with the reusable
`GumboLib::valid_mavlink_carrier` networking boundary.

## 2. Findings

No findings.

Catalog disposition:

- AP-1: no tautological clause.
- AP-2: every denied or invalid case requires `NoSend` on its corresponding output.
- AP-3: every lane explicitly requires no output when no input is present.
- AP-4: all security-relevant carrier fields participate in
  `valid_mavlink_carrier`; MAVLink structure, checksum, dialect metadata, and deny
  policy participate through the two component-local predicates and their concrete
  verified definitions.
- AP-5: allowed, valid-and-denied, and invalid cases cover every present input.
- AP-6: every output lane exports the `mavlink_allowed` integration invariant.
- AP-7: the three present-input partitions are disjoint.
- AP-8: the component introduces no receiver-side integration assumptions.
- AP-9: no randomness or freshness requirement applies.

## 3. Summary Table

| # | Finding | Severity | Anti-Pattern | Category |
|---|---|---|---|---|
| — | No findings | — | AP-1–AP-9 checked | — |

## 4. Demonstration Tests

No finding-demonstration tests are required because the model contract has zero
catalog findings. Existing CompDev tests cover allowed `FILE_TRANSFER_PROTOCOL`, both
firmware-flash deny encodings, malformed and checksum-failing MAVLink v1/v2 frames,
invalid payload bounds, all four lanes, carrier preservation, and no-input behavior.

## 5. Unscored Observations

The MAVLinkFirewall-local functions remain bodyless `@spec` declarations in SysML,
as required for component-owned developer definitions. The generated proof hooks now
delegate to concrete `mavlink_core` specifications rather than uninterpreted
predicates. The verified core covers MAVLink v1/v2 framing, signed-v2 length, complete
284-message dialect CRC/min/max metadata, X.25 checksum, command envelopes, command
42650, and secure-command operation 7. Standalone Verus verification passes 7/0.

The proof iteration also exposed a non-catalog arithmetic issue in
`valid_ardupilot_udp`: expressing IPv4 length as unsigned `udp_length +
ipv4_header_length` admitted modular wraparound. The model now first requires
`ipv4_header_length <= ipv4_length` and then compares `udp_length` with the guarded
subtraction `ipv4_length - ipv4_header_length`. This is outside AP-1–AP-9, is not
scored as a catalog finding, and is resolved in the current model. `sireum hamr sysml
tipe` reports `Well-formed!`.

HLR-27 requires rejection logging. GUMBO compute contracts describe port/state
behavior rather than logging side effects, and the catalog has no logging pattern.
Logging remains an implementation and test obligation in W2.
