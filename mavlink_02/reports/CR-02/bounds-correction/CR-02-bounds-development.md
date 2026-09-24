# CR-02 bounds correction — implementation and coverage review

Date: 2026-09-23. Profile: audited. Developer approved CodeGen boundary.
Authority: Open_Platform_HLRs_26_09_23_01.md; approved shared bound and Tx size
integration guarantees. HAMR context: /home/robertvanvossen/tools/r2u2-HAMR-agent-context.
CompDev steps 1–5 complete for the affected firewall correction; AP1 pending.

## Implementation

- firewall_core::net::MAX_MTU changes from 9000 to 1586. Its executable IPv4 parser
  and core specifications share that constant. Both Rx and Tx use this parser.
  Tx therefore drops an otherwise valid IPv4 frame whose declared total length is
  above 1586 before creating an output. Its valid upper boundary remains 1600 bytes
  including Ethernet header; accepted frame bytes are preserved exactly.
- MAVLinkFirewall's independent MAX_IPV4_TOTAL_LENGTH is derived from CARRIER_BYTES
  minus ETHERNET_HEADER_BYTES. Existing UDP consistency and carrier checks remain.
- No direct Rx or Tx application edit is required: their shared parsing dependency
  supplies the corrected limit. No new minimum IPv4 length, protocol policy, mode
  behavior, suppression logs or proof escapes were introduced.
- The independent driver checked-slice defense was already implemented, exhaustively
  tested and target-built; CodeGen preserved it. That evidence remains applicable.

Model and generated contracts are current from the approved CodeGen, and no model
or generated file was edited in this implementation step. No additional CodeGen,
Verus run or full loader build has occurred. Requirements remain developer-owned.

## Tests

| Suite | Passed | Failures |
|---|---:|---:|
| TxFirewall | 8 | 0 |
| RxFirewall | 12 | 0 |
| MAVLinkFirewall | 16 | 0 |
| firewall_core | 18 | 0 |

Tx fixtures now accept IPv4 1585/1586 and reject 1587/9000/9001/65535 for each
of the ten allowed protocols on every lane. Each emitted frame is independently
checked against the expected bytes and size and against its array capacity.
105 frame fixtures exercise 420 lane placements; the negative compute-oracle matrix
covers 1696 candidate outputs, including absent input, wrong size and changed bytes.
A new direct initialization-oracle test exercises the four generated integration
size guards at 0, 64, 1599, 1600, 1601, 9014 and 65535, plus absent outputs. The
integration invariant alone permits size zero; the full compute policy still fixes
sizes to 64 for ARP and total_length+14 for IPv4. No zero-size Tx forwarding is added.

Rx retains accepted maximum-size direct and MAVLink UDP cases (IPv4 1586), rejects
1587 and larger, and now includes 9001 in the four-lane/two-mode matrix. Its contract
harness checks routing, preservation, exclusivity and no-output behavior.

MAVLink adds a direct executable-versus-generated carrier predicate test with
consistent UDP lengths at 1585/1586 (accepted) and 1587/9000/9001/65535 (rejected).
This isolates the networking envelope from MAVLink framing/CRC policy: it does not
claim a 1586-byte MAVLink message is valid. The component carrier-malformation
matrix now includes 9000 and runs in both Normal and Recovery, confirming invalid
inputs count once per lane even during suppression. All existing command, counter,
monitor, D2 logging and reboot regression tests also pass.

The shared parser's new test checks both sides of the length boundary and retained
parsed lengths for all ten protocols. Prior parser tests remain passing.

## Coverage

Fresh isolated instrumented targets and fresh profile directories were used for
all three firewall components; no historical profiles were merged.

| Source | Covered / executable lines |
|---|---:|
| Tx application | 82 / 84 (82 / 82 reachable) |
| Tx GUMBOX, including new size guards | 205 / 205 |
| Rx application | 118 / 118 |
| Rx GUMBOX | 382 / 382 |
| MAVLink application, excluding its test module | 172 / 172 |
| MAVLink GUMBOX | 382 / 382 |
| MAVLink production logger, excluding its test module | 39 / 39 |

The two uncovered Tx lines are the unchanged unused trace helper's entry/exit,
previously reviewed in W2. MAVLink's entire app file including test fixtures is
253/254; the uncovered test-only unexpected-fixture-ID panic is excluded above.
Host coverage omits seL4-only logging backends. All LCOV records report BRF=0;
no numerical branch percentage is claimed. Deterministic partitions exercise the
changed bound's true/false outcomes, all protocol and lane cases, mode outcomes,
count preservation/increment and accepted/rejected integration/compute oracles.

Commands from each component crate (C is its absolute path):

```
RUSTC_BOOTSTRAP=1 CARGO_TARGET_DIR="$C/target/cr02-bounds" CARGO_INCREMENTAL=0 \
 RUSTFLAGS=-Cinstrument-coverage \
 LLVM_PROFILE_FILE="$C/target/cr02-bounds/profiles/test-%p-%m.profraw" cargo test --offline

grcov "$C/target/cr02-bounds/profiles" \
 --binary-path "$C/target/cr02-bounds/debug/deps" -s "$C" \
 -t lcov --branch --ignore-not-existing -o <report>
```

Shared parser: RUSTC_BOOTSTRAP=1 cargo test --offline from firewall_core.
Evidence: CR-02-bounds-{TxFirewall,RxFirewall,MAVLinkFirewall,firewall_core}-tests.txt;
CR-02-bounds-{TxFirewall,RxFirewall,MAVLinkFirewall}-coverage.lcov.

## Next gate

Await CompDev.AP1 approval of the tests/coverage and stated exclusions, then run
Verus for Tx, Rx, MAVLink and the changed shared parser. Existing earlier proof
results do not certify this correction. Address any proof obligations introduced by
Tx output API size preconditions without weakening the approved model. Full custom
loader rebuild and remaining hardware/timing acceptance follow the proof gates.

## Verification follow-up

Developer approved AP1. Fresh verification passes without source changes: firewall_core 39/0, Tx 16/0, Rx 28/0, MAVLink 69/0. See CR-02-bounds-verification.md for commands, trust boundaries and pending AP2/component completion review.
