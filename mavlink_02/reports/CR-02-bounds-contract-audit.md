# CR-02 firewall bounds contract review

Date: 2026-09-23. Auditor: HAMR GUMBO Contract Audit. Components: TxFirewall,
RxFirewall, MAVLinkFirewall (security-critical). Mode: full, with generated
contracts explicitly stale relative to the revised model. Context:
`/home/robertvanvossen/tools/r2u2-HAMR-agent-context`.

## Result and authority

The source of truth is developer-supplied
`action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_01.md`.
Compared with _07, HLR-12/13/18 replace 9000 with 1586 and explain the Ethernet
header allowance; LLR-4 retains the 1600-byte carrier. No requirements were edited.
The developer also explicitly authorized the independent driver bounds check.

No unresolved AP-1–AP-9 findings in the revised compute contracts. One missing
Tx integration export (AP-6) is addressed in the proposed model. Contract sign-off
is pending. This review does not claim regenerated code, new firewall tests or new
Verus results. Earlier accepted W2 evidence describes the older requirements.

## Concrete model changes and traceability

- `sysmlv2/open_platform/GumboLib.sysml`, `valid_ipv4_length`: require
  `ipv4_length(aframe) <= 1600[u16] - ethernet_header_length()` (1586).
  Traces to HLR-12/13/18 and LLR-4. All existing lane contracts use this predicate
  directly or through the shared UDP/carrier predicates.
- `sysmlv2/open_platform/open_platform_Software.sysml`, TxFirewall integration:
  four `hlr_12_llr_4_txN_size_bound` guarantees export `EthernetFramesTxOutN.sz
  <= 1600[u16]`. HLR-7 ARP output is 64; HLR-12 IPv4 output is at most 1586+14.
  These are consequences of the revised compute contract, not new filtering policy.
- Rx's direct and MAVLink routes still reuse `rx_bounded_udp`; MAVLink independently
  validates its carrier. Existing length relationships already excluded 1587 and
  larger, so the shared bound makes the requirements alignment explicit without
  expanding either accepted UDP input set.

Tx's complete partition per lane is unchanged: valid ARP sends the identical array
with size 64; valid IPv4 sends the identical array with size total_length+14;
other input drops; no input produces no output. At 1586, IPv4 size is 1600;
at 1587, forwarding is forbidden. Tx still has no new minimum-length requirement.
Rx partitions Normal into ARP/direct UDP/MAVLink UDP/drop; Recovery suppresses all
outputs. MAVLink partitions allowed/flash/invalid/no-input with a frozen mode gate,
counts each rejected present input once in either mode, saturates at 20 and emits
post-count ErrorStatus. Counter, SECURE_COMMAND policy and R2U2 formula are unchanged.

## Catalog review

| Pattern | Tx | Rx | MAVLink |
|---|---|---|---|
| AP-1 tautology | None; input-or-no-output is a meaningful implication | None | Inductive count bound is meaningful |
| AP-2 weak drop | Strict NoSend | Both route outputs NoSend | Strict NoSend |
| AP-3 absent input | Covered on all four lanes | Covered on all eight outputs | Covered on all four lanes |
| AP-4 ignored field | IPv4 length is inspected; output size constrained | Mode, length and UDP fields inspected | Carrier, mode, policy and count inspected |
| AP-5 partition gaps | ARP/IPv4/other/absent exhaustive | Normal/Recovery and disjoint routes exhaustive | Valid/flash/invalid/absent in both modes exhaustive |
| AP-6 output invariant export | New size exports resolve missing bound export | Existing direct/carrier exports include bound | Existing allowed-carrier exports include bound |
| AP-7 contradiction | ARP and IPv4 disjoint; new size bounds follow compute | Direct/MAVLink ports disjoint; Normal guards exclude Recovery | Allow requires Normal; all overlapping drop conditions agree |
| AP-8 handshake asymmetry | No receiver integration assumes introduced | Existing reusable design retained | Existing reusable design retained |
| AP-9 variation | Not required | Not required | Not required |

AP-6 disposition: pre-change Tx had no integration block. The current source has
four output-size guarantees, resolving export of the safety property being reviewed.
The generated oracle still has no corresponding integration guards pending CodeGen.
No downstream integration assumption or non-vacuous connection proof is claimed.

## Generated artifacts and tests

Reviewed model compute blocks, shared predicates, component generated GUMBOX
postconditions, woven ensures and existing boundary/negative-oracle tests. Generated
GumboLib still uses 9000; existing Tx fixtures intentionally accept 9000 and size
9014. The existing direct-oracle regression is rerun as evidence of the OLD contract
only (CR-02-bounds-old-tx-oracle.txt): 1/1 passed using `RUSTC_BOOTSTRAP=1
cargo test oracle_rejects_injection_wrong_size_and_changed_bytes` in the Tx crate.
The initial run without the required bootstrap setting failed before tests. The
fixture expectations must be updated after generation. There is
no unresolved revised-compute finding needing a new permissiveness demonstration.
AP-6 is an integration export observation; the current compute oracle cannot test
an integration guarantee that has not been generated.

After sign-off: regenerate (never manually alter generated contracts), attempt the
required reporting patch and run its probe. Change firewall_core's MAX_MTU and the
MAVLink executable maximum to 1586; refresh Tx fixture expectations and every-lane
negative-oracle checks at 1587/9000/9001/65535. Retain Rx/MAVLink malformed-length,
mode/counter and monitor coverage; run affected tests/coverage and Verus at the
existing audited gates. Rebuild the full target and update loader evidence.

Final model type-check: `sireum hamr sysml tipe --sourcepath ../aadl-lib:.
Platform.sysml` from `sysmlv2/open_platform`, exit 0, `Well-formed!`.
Evidence: CR-02-bounds-tipe.txt. One attempted invocation from the project root
failed to locate the relative sourcepath; the corrected final run covers both edits.

## Independent driver fix

The driver calls application-owned `tx_bounds::bounded_payload` before requesting
a transmit token. Empty and oversized requests yield None; other requests produce
a checked slice used for both token length and copy. The loop continues to later
lanes and existing interrupt handling. No truncation or extra per-frame logging.
The 1600-byte carrier matches the current DMA MTU (core/src/dma.rs).

The helper is tested directly, without requiring the driver crate's seL4 host
configuration: 2/2 tests pass, exhausting all 65,536 u16 sizes and checking mixed
invalid/valid requests. Test output: CR-02-driver-bounds-tests.txt. Reproduce with
an edition-2021 `rustc --test` harness that includes the production tx_bounds.rs
as a module via `#[path = "<absolute helper path>"] mod tx_bounds;`.

Target driver build passes with:

```
SYSTEM_MAKEFILE=custom.mk make build-release \
  MICROKIT_SDK=/home/robertvanvossen/tools/microkit-sdk-2.2.0-dev \
  MICROKIT_BOARD=zcu102 MICROKIT_CONFIG=debug
```

Run in the driver crate. Evidence: CR-02-driver-bounds-build.txt. This builds
AArch64 release code, not a Verus proof or hardware test. Full loader rebuild remains
pending the firewall changes. Legacy schedule and custom build integration retained.

## Approval and regeneration follow-up

Developer approved the revised contracts on 2026-09-23. CodeGen then succeeded,
including mandatory reporting patch and 2/2 probe tests; see CR-02-bounds-codegen.md.
The stale-artifact observations above describe the pre-generation review, not the
current generated output. Application/test updates and fresh proofs remain pending.
