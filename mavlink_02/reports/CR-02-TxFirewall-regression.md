# CR-02 Wave 2 — TxFirewall regression

Date: 2026-09-23. Profile: audited. TestOnly steps 3–5 complete; review pending.
MAVLinkFirewall verification 69/0 and component completion were approved by the
developer. This is the approved plan's TxFirewall TestOnly slice. VerifyOnly and
core regressions remain next; Wave 2 is not yet complete.

## Scope and non-impact

Only editable Tx tests were extended. No Tx implementation or contract changes were
made. Compared with baseline 043d574970d28ff172f7261ea0392ddd14ae50a8, the app differs
only within the generated ensures marker, where HAMR now spells post-state references
as final(api). A comparison with that marker removed confirms the remainder of the
file is identical. Both shared cores' src directories are unchanged from baseline.
Previously authorized toolchain/dependency updates remain in effect.

The strict source/destination UDP exclusions apply to Rx, not Tx. Regression fixtures
include source 14550/destination 68 IPv4 traffic and confirm Tx still forwards it.
No Mode input or new transmit restriction was introduced.

## Tests and coverage

**7 tests passed, zero failed**, including the retained 100-case initialization and
compute PropTests and three new deterministic regression tests.

- 65 frame fixtures on every lane: ARP request/reply with IPv4/IPv6 protocol type;
  malformed ARP hardware/protocol/operation; IPv4's ten accepted protocol values;
  lengths 0, 20, 28, 1586 and 9000; rejected 9001/65535; invalid protocol/options,
  zero destination, unknown EtherType and IPv6 rejection.
- Each accepted output preserves all frame bytes and has the expected ARP size 64
  or IPv4 total length + 14. Absent and rejected lanes emit nothing. Initialization,
  mixed traffic rotated across all lanes, empty dispatch and notify are exercised.
- 1,056 constructed oracle combinations cover every lane and fixture plus absent
  input, rejecting injected outputs, incorrect sizes and changed frame bytes.
  Expected output policy is provided by independent fixture metadata.

Coverage from an isolated instrumented target and fresh profiles:

| Source | Covered executable lines |
|---|---|
| Tx application | 82/84 (97.62%) |
| Tx GUMBOX | 159/159 (100%) |

The two uncovered app lines (26 and 29) are the entry/exit of the existing unused
trace logging helper. It has no callers in the component. All reachable host app
lines are covered (82/82). No implementation change was made merely to remove that
coverage gap. Production seL4 logging macro bodies are excluded from host builds;
they remain part of Wave 3 target validation. LLVM reports BRF=0, so no numerical
branch percentage is claimed. Deterministic fixtures exercise present/absent input,
parse success/failure, all three packet variants, and all output-oracle outcomes.

These tests preserve the existing Tx length contract: an IPv4 length of 9000 yields
output sz=9014 even though the stored frame array has 1600 bytes. This is existing
behavior, not a claim of safe physical transmission for oversized frames. Inspect
downstream size handling during Wave 3; changing Tx policy is outside this slice.

Evidence: CR-02-TxFirewall-tests.txt and CR-02-TxFirewall-coverage.lcov.
HTML: component target/cr02-current/report/index.html.
Commands run from hamr/microkit/crates/seL4_TxFirewall_TxFirewall:

```
RUSTC_BOOTSTRAP=1 CARGO_TARGET_DIR=target/cr02-current CARGO_INCREMENTAL=0 \
RUSTFLAGS=-Cinstrument-coverage LLVM_PROFILE_FILE=<absolute-crate-path>/target/cr02-current/profiles/test-%p-%m.profraw \
cargo test --offline

grcov target/cr02-current/profiles --binary-path target/cr02-current/debug/deps \
  -s . -t lcov --branch --ignore-not-existing -o <coverage-report>
```

Await approval of this TestOnly result and exclusions before its audited slice
boundary and continuation to Tx VerifyOnly. No fresh Tx verification is claimed by
this report. Earlier migration verification remains historical evidence.
