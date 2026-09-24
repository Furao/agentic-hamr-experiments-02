# CR-02 Wave 2 review

Date: 2026-09-23. Status: technical work complete; Tx VerifyOnly AP2/boundary and
ChangeExec Wave 2 approval pending. No approval is inferred from earlier gates.
Authority: developer-owned _07 requirements and subsequent explicit instructions.

Wave 2 implements frozen-mode forwarding control, latched Recovery, strict receive
UDP policy, saturating rejection counts, final ErrorStatus publication, SECURE_COMMAND
operation-7 zero extension, and HAMR-generated R2U2 deadline reporting. Per-frame
Recovery suppression logs are omitted as directed. The production reporter emits
one timeout during D2 and resets on reboot. No handwritten deadline timer was added.

## Evidence consolidated for review

| Component/core | Tests passed | Verus verified / errors | Evidence |
|---|---:|---:|---|
| ModeManager | 7 | 9 / 0 | components/CR-02-ModeManager-development.md |
| RxFirewall | 12 | 28 / 0 | components/CR-02-RxFirewall-development.md |
| MAVLinkFirewall | 15 | 69 / 0 | components/CR-02-MAVLinkFirewall-development.md |
| TxFirewall | 7 | 16 / 0 | components/CR-02-TxFirewall-regression.md |
| firewall_core | 17 | 39 / 0 | components/CR-02-firewall_core-tests.txt and components/CR-02-firewall_core-verification.txt |
| mavlink_core | 6 | 38 / 0 | components/CR-02-mavlink_core-tests.txt and components/CR-02-mavlink_core-verification.txt |

Results are consolidated from component runs, not claimed as one simultaneous run.
Tx verification and both core regression suites were rerun for this gate. Previously
approved affected-component evidence remains applicable; no subsequent source changes
invalidate it. Components used the aarch64 target; standalone core verification used
the host target. No full-system or hardware pass is claimed.

All changed applications and generated GUMBOX have 100% active host line coverage.
Tx has 82/84 app lines covered (two unused trace-helper lines excluded with developer
approval) and GUMBOX 159/159. LLVM branch counters are unavailable (BRF=0); recorded
semantic partitions and explicit exclusions qualify these results. MAVLink logger
coverage is 39/39 host lines. The real generated monitor tests cover timely D1/D2,
late D3, never Recovery, repeated status, empty dispatches, reboot and forwarding
independence, with exact D2 diagnostic assertions.

Tx executable source and both core source trees retain baseline behavior. Tx's
baseline delta is generated final(api) post-state syntax plus separately authorized
toolchain configuration; this slice added only regression tests. There were no
verification fixes in Tx or either core and no new proof escapes. Tx retains three
logging external bodies; affected-component logging and platform-startup proof
boundaries are documented in their approved reports.

## Wave 3 handoff and limitations

- Reconcile custom.mk, ModeManager image/domain/type rules and actual schedule.
  Demonstrate frozen snapshot propagation and budget impact on the deployed target.
- Complete the ZCU102 build and hardware acceptance; the driver's known host-only
  seL4 dependency limitation is not a target-build result.
- Retain the authorized 4bc9a9a generated monitor workaround after every codegen
  invocation. The production reporting path depends on it. Temporal compiler/runtime
  correctness and physical logging remain outside the application Verus proofs.
- Resolve the default R2U2 CLI installation path's missing libpython3.14 link library
  or configure reuse of the established CLI. MAV verification used existing generated
  artifacts with `make -o r2u2_cli verus`; no codegen was invoked in this wave review.
- Inspect existing Tx downstream size handling: the baseline permits IPv4 length
  9000 and produces sz=9014 with a 1600-byte frame array. Regression preserves that
  contract; it does not establish safe target handling of oversized frames.
- Requirements remain developer-owned. Reconcile LLR-18's suppression-log wording
  and legacy documents during the final review; no requirement file was edited.
- Rx/MAV initialization proofs rely on explicitly empty outgoing event slots as
  supplied by HAMR startup. End-to-end extern-C framing is inspected, not proved.
- W1 integration checking was vacuous (N=0); no whole-system temporal proof is claimed.

Review requested: accept Tx verification/trust boundary and its slice completion,
then accept the consolidated Wave 2 gate and authorize the planned Wave 3 work.

Developer approved Tx VerifyOnly/AP2 and boundary, consolidated Wave 2 completion, and continuation to Wave 3.
