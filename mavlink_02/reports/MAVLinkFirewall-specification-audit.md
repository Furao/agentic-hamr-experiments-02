# MAVLinkFirewall GUMBO Specification Audit Report

**Component**: `MAVLinkFirewall` (MAVLink validation and command firewall)  
**Model File**: `sysmlv2/open_platform/open_platform_Software.sysml`, lines 124–201  
**Date**: 2026-08-26  
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

This is a specification-only audit because the MAVLinkFirewall generated crate does
not yet exist. The abstract `mavlink_frame_valid` and
`mavlink_firmware_flash_command` predicates deliberately state the security boundary
without duplicating a variable-length MAVLink parser in GUMBO. Their executable and
verified definitions remain W2 proof obligations, and CodeGen compatibility must be
confirmed in W1.

## 2. Findings

No findings.

Catalog disposition:

- AP-1: no tautological clause.
- AP-2: every denied or invalid case requires `NoSend` on its corresponding output.
- AP-3: every lane explicitly requires no output when no input is present.
- AP-4: all security-relevant carrier fields participate in
  `valid_mavlink_carrier`; MAVLink structure, checksum, and deny policy participate
  through the two abstract predicates.
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
catalog findings. CodeGen must first produce the MAVLinkFirewall GUMBOX functions.
CompDev.3 must then add functional tests covering allowed `FILE_TRANSFER_PROTOCOL`,
both firmware-flash deny encodings, malformed and checksum-failing MAVLink v1/v2
frames, invalid payload bounds, all four lanes, carrier preservation, and no-input
behavior.

## 5. Unscored Observations

`mavlink_frame_valid` and `mavlink_firmware_flash_command` are bodyless `@spec`
predicates. They make the intended contract readable and avoid replicated wire-format
offsets, but do not themselves prove that an implementation parses MAVLink framing,
dialect CRC-extra values, command envelopes, or secure-command operation 7 correctly.
The verified MAVLink core must refine these predicates, and its tests must bind them
to concrete byte-level cases. This limitation is outside AP-1–AP-9 and is therefore
not scored.

HLR-27 requires rejection logging. GUMBO compute contracts describe port/state
behavior rather than logging side effects, and the catalog has no logging pattern.
Logging remains an implementation and test obligation in W2.
