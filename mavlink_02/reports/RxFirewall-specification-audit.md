# RxFirewall GUMBO Specification Audit Report

**Component**: `RxFirewall` (receive-side network firewall)  
**Model File**: `sysmlv2/open_platform/open_platform_Software.sysml`, lines 22–122  
**Date**: 2026-08-26  
**Auditor**: HAMR GUMBO Contract Audit  
**Classification**: Security-Critical

## 1. Executive Summary

The revised RxFirewall contract was checked against anti-patterns AP-1 through AP-9.
No catalog findings remain. For each of four lanes, the compute contract partitions
present inputs into exactly one of direct forwarding, MAVLink routing, or drop, and
separately constrains no-input behavior. Every path constrains both output ports, so
the component cannot duplicate, fabricate, or route one input on both paths while
satisfying the contract.

Output integration guarantees export the context-independent invariants established by
the component: direct outputs satisfy `rx_direct_frame`, and MAVLink-path outputs
satisfy `valid_mavlink_carrier`. The carrier preserves the Ethernet frame and fixes the
validated payload offset and length. Packet layout is centralized in getter/spec
functions (`ipv4_length`, `udp_length`, `udp_payload_offset`, and
`udp_payload_length`); the per-lane contracts do not replicate byte offsets.

The existing generated RxFirewall crate predates this model revision. Its GUMBOX,
bridge, application-marker, and test infrastructure remain stale until W1 CodeGen and
must not be treated as evidence for this audit pass.

## 2. Findings

No findings.

Catalog disposition:

- AP-1: no tautological clause.
- AP-2: every drop consequent requires `NoSend` on both outputs.
- AP-3: each lane explicitly requires no output when no input is present.
- AP-4: the input carrier has no unreferenced security flag; networking validity is
  expressed through the referenced library predicates.
- AP-5: `rx_direct_frame`, `valid_ardupilot_udp`, and the complement captured through
  `not rx_allow_outbound_frame` cover every present input.
- AP-6: both output families have integration guarantees.
- AP-7: direct and MAVLink predicates are disjoint, and the drop predicate is their
  complement.
- AP-8: no receiver-side integration assumptions are introduced.
- AP-9: no randomness or freshness requirement applies.

## 3. Summary Table

| # | Finding | Severity | Anti-Pattern | Category |
|---|---|---|---|---|
| — | No findings | — | AP-1–AP-9 checked | — |

## 4. Accompanying Test Cases

No demonstration tests are required because the revised model contract has zero audit
findings. W1 CodeGen must regenerate the GUMBOX functions before CompDev tests use
them. Functional tests are still required for every routing partition, all four lanes,
payload metadata, exclusivity, and no-input behavior.

## 5. Unscored Observation

HLR-20 requires decision logging. GUMBO compute contracts describe port/state behavior,
not logging side effects, and the current anti-pattern catalog has no logging pattern.
Logging therefore remains an implementation and test obligation in W2 rather than a
scored contract finding.
