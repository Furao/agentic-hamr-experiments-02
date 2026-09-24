# Category 4 — Agent harness and environment

CR-02 only: requirements review through final approval. Original CR-01 development and post-closeout reporting work are excluded. Each finding states its relationship to this change.

Severity: 🔴 blocker · 🟠 moderate · 🟡 minor · 🟢 positive. Historical severity and current disposition are separate.

---

## ENV-02 — Restricted network and external cache writes caused recoverable execution friction 🟠

**Scope.** `codex`. **Severity.** moderate. **Disposition.** recovered in run; setup improvement remains.

**CR-02 relationship.** Observed harness execution friction within the CR-02 transcript interval.

**Evidence.** [experiment-reports/session-metrics.json: friction.observed_policy_settings](../../experiment-reports/session-metrics.json); [experiment-reports/session-transcript.md:2823, 4320, 4456, 4764, 4990, 6177, 9495](../../experiment-reports/session-transcript.md); [reports/CR-02/deployment/CR-02-w3-build.md](../../reports/CR-02/deployment/CR-02-w3-build.md); [reports/CR-02/codegen/CR-02-hamr-upgrade.md](../../reports/CR-02/codegen/CR-02-hamr-upgrade.md).

**Impact.** Dependency preparation, coverage and type-check/build operations required retries or elevated execution; the inventory below separates confirmed filesystem denials from network failures and requests.

**Root cause.** Observed on-request/workspace-write policy disables network access and excludes tool-installation/cache paths. Relative LLVM profile paths additionally make write location dependent on subprocess working directory.

**Recommendation.** Target: Codex workspace setup and project preparation scripts. Preinstall pinned compiler/sysroot dependencies in an authorized setup step; configure supported caches and absolute coverage output under writable project/tmp paths. Preflight external launcher/cache writes and use narrowly scoped escalation when needed.

**Evidence limit.** DNS failures are consistent with disabled sandbox networking but do not independently exclude external DNS causes. Relative-profile failure location and all UI approval interactions are not recoverable from truncated output. These are harness/environment constraints, not HAMR or model-quality failures. All seven manually enumerated requests and all three explicit read-only failures fall inside the CR-02 transcript window. Metrics-wide warning counts and the unlinked saved approval cannot be restricted to CR-02.

---

## ENV-03 — Driver host checks and standalone target checks require different configurations 🟡

**Scope.** `environment`. **Severity.** minor. **Disposition.** documented limitation.

**CR-02 relationship.** Inherited platform limitation re-observed in CR-02 driver amendment and final validation.

**Evidence.** [reports/CR-02/final-validation/CR-02-full-verify-build.md](../../reports/CR-02/final-validation/CR-02-full-verify-build.md); [reports/CR-02/final-validation/CR-02-final-validation.md](../../reports/CR-02/final-validation/CR-02-final-validation.md); [reports/CR-02/deployment/CR-02-w3-build.md](../../reports/CR-02/deployment/CR-02-w3-build.md).

**Impact.** CR-02 driver-bounds validation and the final sweep encountered an unavailable full host suite and a standalone target recipe missing logging features. The deployed-feature target retry succeeded; the new bounds helper passed 2 tests. These observations constrain acceptance of the CR-02 driver amendment.

**Root cause.** seL4 headers/features and AArch64 dependencies do not match the generic x86 host-test path; standalone target flags omit the sel4 feature. Driver application verification is explicitly disabled.

**Recommendation.** Target: Driver test/verify wrapper and setup documentation. Provide separate named host-helper and deployed-target recipes, checking SDK headers/features first. Label the latter compilation-only while verify=false; keep full driver hardware acceptance separate.

**Evidence limit.** The host/target distinction and verify=false are inherited limitations re-observed during CR-02, not new ModeManager defects. Two helper tests and a target build do not establish a full driver suite or application proof.

---

## Friction inventory and reconciliation

The starting point is the existing schema-v2 [session-metrics.json](../session-metrics.json). It covers the whole normalized primary session, not an isolated CR-02 slice. The event supplement below is restricted to [session-transcript.md](../session-transcript.md) lines 136–10560. Extracted `sandbox_failures`, `escalation_requests` and
`dynamic_shell_commands` each contain **0 entries**. These are incomplete arrays:
1267 session-wide parser warnings cover 506 distinct orchestration call IDs. These are evidence-quality metadata, not a CR-02 development finding. The following
manual supplement uses visible normalized transcript text only; it does not alter
the metrics or claim an exhaustive reconstruction.

### Confirmed filesystem failures, separate from escalation requests

| Transcript evidence | Observable failure | Cause and concrete fix |
|---|---|---|
| Line 4320, `call_545kxp3l4cR1ABoETv9TvZT1` | Rustup cannot create `/home/robertvanvossen/.rustup/tmp/...`: `Read-only file system (os error 30)` | Toolchain synchronization writes outside workspace roots. Prepare the pinned toolchain in an authorized setup step before running component checks. |
| Line 4456, `call_oZFG2BsGMSkRXh6ARFuMkznf` | `LLVM Profile Error: Failed to write file "target/coverage/cargo-test-...profraw": Read-only file system` | The relative profile path resolves in a non-writable context for at least one instrumented process. Use an absolute, precreated writable profile directory and a separate Cargo target directory. The exact process cwd is unavailable. |
| Line 9495, `call_QCF35sh9aFJShEpPLkm547Nq` | Sireum launcher line 67 cannot write `bin/.aot/train.log`: `Read-only file system` | Updated launcher writes into the external installation. Preflight launcher write requirements; prepare it with authorized permissions or use a supported writable log/cache configuration. The subsequent escalated type-check succeeds. |

These are **three visible filesystem-denial events**, a lower bound. They are not
three inferred UI prompts. The build report additionally records external Rust
cache writes requiring an approved rerun, but its normalized failure output is
truncated; no fourth exact error is invented.

Network failures are listed separately because an error string alone cannot prove
the sandbox was the sole cause:

| Transcript lines | Operation / visible error | Attribution and fix |
|---|---|---|
| 2823, 2844 | R2U2 CLI installation; cannot resolve `index.crates.io` | Consistent with recorded `network_access: false`; install pinned tools/dependencies in an authorized preparation step. Repeated retries belong to the same episode. |
| 4764 | Target sysroot dependency `wasip1`; cannot resolve `index.crates.io` | Same policy constraint; prepare Rust 1.97.1 target/sysroot dependencies before offline verification. |
| 4990 | ModeManager target dependency fetch; cannot resolve `static.crates.io` | Same constraint; populate the target dependency cache, then use the locked/offline verification path where supported. |
| 6177 | Generated `cargo +stable install r2u2_cli --version 4.2.4`; cannot resolve `index.crates.io` | Network restriction plus install-on-verification prerequisite; separate installation from compilation (CG-02). |

The probe escalation justification at line 3332 also reports blocked dependency
resolution, but that request is not counted here as an independently visible error
result. DNS infrastructure faults cannot be excluded from the normalized record.

### Observable requested escalations

There are **seven visible `require_escalated` requests** below, despite the empty
extracted array. This is a lower bound over truncated normalized calls, not a count
of dialogs displayed or clicked. Network/cache setup and narrow command recipes
are the recommended fixes; broad shell permission is not necessary.

| Transcript line / call ID | Requested operation | Observable outcome / evidence limit |
|---|---|---|
| 2833 / `call_0YgGimvQM0Zfbv4PCPKVg6Yj` | Install R2U2 CLI 4.2.4 under `/tmp/cr02-r2u2-tools` | Subsequent result shows crate downloads; later reports use the installed compiler. No UI click or approval latency is visible. |
| 3332 / `call_XILewCU0K8rh1fK6N3L8FZxf` | Run isolated monitor tests with dependency download | Subsequent probe report records real runtime execution, initially 1 pass/1 failure for the generator defect. A failing assertion is not an approval denial. |
| 4770 / `call_LRGbBhUGj87VIt7pGO9Sn1nO` | Tx target `make verus` with sysroot fetch | Toolchain migration report records target verification 16/0. It does not expose how approval was granted. |
| 4996 / `call_xEaY2yG5iWfL2lIflwgCx91O` | ModeManager `make verus` with missing target dependencies | Later component/final reports record successful verification; no per-request UI decision is supplied. |
| 6183 / `call_r5ZlH9QluGqvqaELiqfc7Bzk` | MAVLink `make verus` | Request flag and command are visible; justification is truncated. Later verification succeeds; exact immediate permission disposition is unavailable. |
| 7161 / `call_dSt4loRf0vrhJxGVyFFIkWex` | Full custom ZCU102 build | Result starts process session 15232; W3 build report explicitly records an approved escalated rerun and exit 0. Exact UI mechanism is unknown. |
| 9501 / `call_xiE4SQgahRZuxBsthVkSOvdm` | Sireum type-check after external AOT log denial | Subsequent result and upgrade report record `Well-formed!` / success. No approval-dialog observation is claimed. |


### Approval observations and command-shape heuristics

`friction.observable_approval_outcomes` contains **one** `approval_granted`
observation, `call_id: null`. Its evidence is a saved `/usr/bin/zsh -lc` prefix for
instrumented Cargo tests using `RUSTC_BOOTSTRAP=1`, `CARGO_INCREMENTAL=0`,
`RUSTFLAGS=-Cinstrument-coverage` and `LLVM_PROFILE_FILE`. This supports one session-wide recorded saved approval, not one total prompt. It cannot be joined to the seven requests or confidently attributed to the CR-02 interval, so it is not counted as a CR-02-specific approval outcome.
Successful execution does not reveal whether a user clicked a prompt, an existing
prefix rule matched, or another authorization mechanism applied. No denied approval
is established here. Workflow audit approvals are not sandbox approvals.

The metrics dynamic-command array is empty. The following visible command forms
are **heuristic prompt candidates only**; they are neither confirmed prompts nor
confirmed sandbox failures:

- Transcript `call_duI7d48UWtSnYnehGs0zsR3o` uses an inline Python heredoc and
  programmatic file edits. Such bodies are difficult to represent as a narrow
  reusable command prefix. Prefer a reviewable project script with explicit
  arguments when the same operation recurs.
- `call_dSt4loRf0vrhJxGVyFFIkWex` combines environment assignment, Make arguments and
  output redirection. The escalation flag is directly visible, but syntax alone
  does not establish why approval was needed. A stable build wrapper and explicit
  writable log path would simplify review; external cache needs remain separate.
- The saved coverage prefix contains shell environment assignments. Instrumentation
  wrappers with absolute output directories can improve repeatability. The saved
  rule is evidence of authorization, not evidence that every similar invocation
  prompted or that arbitrary shell execution should be allowed.

### Epistemic limits

The recorded policy is `on-request`, `workspace-write`, network disabled, with the
additional HAMR context root writable. It is an observed configuration, not proof
that every command used identical settings. Parser warnings consist of **761
nonliteral-argument warnings and 506 undecoded orchestration warnings**. Warnings
can repeat for one call. Zero malformed top-level records does not imply complete
nested-shell decoding. The 527 top-level tool calls are not a denominator for a
reliable nested-command failure rate.

All five metrics limitations apply: outcomes are observable only when represented
in the source record; nested decoding requires JSON literals; subagent sessions
are excluded from tokens/time/dialog/tools; reasoning records are omitted and
reasoning tokens are a subset of output tokens; billed costs and subscription
credit values are not inferred. Normalized call/result truncation imposes another
limit. No friction duration, prompt latency, exhaustive event total or unseen
subagent outcome is claimed. The existing transcript/metrics were not regenerated
or modified for this assessment.


### Boundary audit

All three listed filesystem-denial events and seven escalation requests occur
between the initial CR-02 requirements review (line 136 user block) and the final
completion response (through line 10560). The four network episodes and named
heredoc/build heuristic candidates also occur in that interval. These are separate
inventories; requests and failures must not be added together as unique incidents.
The saved coverage-prefix observation is unlinked session metadata; its command
shape is an unscoped heuristic example only. No post-closeout event is counted as
CR-02 friction.

The earlier assessment's ENV-01 parser finding is retained here only as a data-quality
limitation: the transcription adapter emits its warning inventory after the
assessed development. Concrete improvement remains to capture nested calls
structurally or safely parse supported object literals without evaluating code.
Do not label the empty arrays as zero CR-02 friction. No transcript/metrics refresh
or adapter modification was performed for this assessment.
