# CR-02 Wave 2 — MAVLinkFirewall development

Date: 2026-09-23. Profile: audited. Status: steps 1–5 complete, AP1 pending.
Developer approved RxFirewall completion and continuation to this component.
Authority: developer-owned Open_Platform_HLRs_26_09_22_07.md, approved contracts,
and subsequent instruction to omit per-frame Recovery suppression logs.

## Implementation

Initialize resets rejected_count to zero and publishes ErrorStatus=false. The
editable initialization signature now explicitly requires four empty event outputs,
matching the RxFirewall startup precondition accepted by the developer. Generated
init_api constructs those outputs as None; the application emits no initialization
events. This platform framing condition is inspected, not proved end-to-end through
the extern-C lifecycle.

Each compute reads frozen Mode once and classifies all four lanes, including during
Recovery and at count saturation. Invalid carriers, malformed MAVLink and flash
commands each contribute one rejection. Allowed and absent inputs contribute zero;
Recovery suppresses allowed output without adding rejection counts or suppression
logs. Rejection diagnostics remain. The count saturates at twenty. ErrorStatus is
published from the resulting count (>=5) after all lane processing, including empty
and Recovery dispatches. Its final application-output ordering is checked by source
inspection; host tests assert the resulting value.

SECURE_COMMAND operation uses payload bytes 4–7 with zero extension for missing
trailing bytes. It never reads checksum/signature bytes as omitted operation data.
The executable helper and developer-supplied Verus predicate were updated together.
This discharges the deferred W1 refinement obligation for operation 7, without
changing the approved model clauses. COMMAND_INT and COMMAND_LONG retain command
bytes 28–29. Both nonzero command bytes are needed for command 42650.

The production logger in editable logging.rs consumes the actual generated R2U2
verdict record. It matches the exact component monitor target, Info level and exact
false-verdict message, then emits one Error directly to the backend, avoiding
recursive logging. An atomic latch resets at initialization. The matcher uses
streamed formatting without allocation. The facade retains Info so the verdict can
reach the adapter; routine monitor Info statuses are discarded. Other normal logs
and monitor warnings/errors retain their levels. The seL4 sink remains sel4_logging.
No handwritten deadline timer or forwarding control is introduced by the reporter.

Generated lib.rs invokes pre/app/post hooks for every dispatch; initialization resets
the monitor without advancing time. The existing authorized 4bc9a9a workaround is
still required to expose a false verdict when a later true verdict appears in the
same R2U2 step. Production tests exercise that patched generated monitor and its
compiled specification, not a replacement monitor.

Fresh model check: **Well-formed**, exit 0. Model and generated contracts are current;
no codegen was needed. No generated files, shared cores or requirements were edited.

## Tests and coverage

**15 tests passed, zero failed**, including generated 100-case initialization and
compute PropTests. Compute randomizes pre-count over 0–20 and both frozen modes.

- All four lanes: allowed, flash, malformed, invalid carrier, absent input; original
  output bytes preserved in Normal and all outputs empty in Recovery.
- Counts at 0, 3, 4, 5, 18, 19 and 20; simultaneous rejections, threshold crossing,
  saturation, continued processing, empty publication and reboot reset.
- SECURE_COMMAND payload lengths 1, 4, 5, 6, 7, 8, 12 and 232; operation 7,
  adjacent operations, and each nonzero upper operation byte. Existing v1, signed
  v2, command truncation, checksum and carrier-offset cases retained.
- Nonzero destination MAC at each octet, zero destination, wrong EtherType/protocol/
  ports/options, short/oversized/inconsistent lengths and carrier offset/length.
- 12,000 constructed compute-oracle combinations vary lane, mode, input class,
  pre/post count, output and status. They reject injected/altered/disallowed outputs,
  incorrect counters and status. Additional initialization negatives reject nonzero
  count, true status and any emitted event; pre-count 21 fails the precondition.
- Real generated monitor timelines trigger at dispatches 1, 2 and 4. Recovery at D1
  or D2 succeeds; D3 or never produces exactly one Error during D2. Each timeline
  runs twice across reboot, then continues for multiple empty dispatches. Separate
  traffic tests show timeout does not change forwarding/count/status.
- Logger tests check wrong source/level, suffix/prefix mismatch, fragmented formatting,
  duplicate false verdicts and ordinary logging. Recovery-only suppression is silent.

Fresh isolated instrumented build, fresh final-profiles directory, grcov:

| Active host source | Covered executable lines |
|---|---|
| Application (excluding cfg(test) policy fixture module) | 172/172 (100%) |
| Generated GUMBOX | 382/382 (100%) |
| Logger (excluding its unit-test module) | 39/39 (100%) |

The raw app file includes test fixture code: 236/237 lines. The only uncovered line
is its unreachable unexpected-fixture-ID panic, excluded from application coverage.
Logger host coverage includes test capture plumbing; seL4-only backend setup/output
and flush are not compiled on the host and remain target evidence for Wave 3.
LLVM emits no branch counters (BRF=0), so no numerical branch percentage is claimed.
The listed semantic partitions cover the active policy/control/oracle decisions;
line coverage alone is not treated as proof of exhaustive path coverage.

Evidence: CR-02-MAVLinkFirewall-tests.txt, CR-02-MAVLinkFirewall-coverage.lcov,
CR-02-MAVLinkFirewall-tipe.txt. HTML: component target/cr02-current/report/index.html.
Run from the component crate so .cargo/config.toml supplies generated R2U2 bounds:

```
RUSTC_BOOTSTRAP=1 CARGO_TARGET_DIR=target/cr02-current CARGO_INCREMENTAL=0 \
RUSTFLAGS=-Cinstrument-coverage LLVM_PROFILE_FILE=<absolute-crate-path>/target/cr02-current/final-profiles/test-%p-%m.profraw \
cargo test --offline

grcov target/cr02-current/final-profiles --binary-path target/cr02-current/debug/deps \
  -s . -t lcov --branch --ignore-not-existing -o <coverage-report>
```

## Next gate and proof boundary

Await CompDev.AP1 approval of tests/coverage and stated exclusions, then run Verus.
No verification success is claimed for this revised implementation yet. The three
existing external bodies remain logging adapters; no new proof escape was added.
The generated R2U2 runtime and Rust logging integration are outside application
Verus proofs and rely on the recorded runtime tests and later target evidence.
Requirements reconciliation for the user-directed LLR-18 suppression-log exception
remains developer-owned. No requirement file was modified.

Developer follow-up: the SECURE_COMMAND specification now reuses SECURE_COMMAND_ID
and SECURE_COMMAND_FLASH_BOOTLOADER, matching the executable classifier instead of
duplicating their literals in local spec bindings. Component tests remain 15/15.
This specification-only refactor does not change executable coverage; recorded LCOV
line numbers precede the two-line removal. Verus remains pending AP1 approval.
