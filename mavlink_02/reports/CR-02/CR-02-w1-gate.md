# CR-02 Wave 1 review

Date: 2026-09-23. Profile: audited. Status: developer approved Wave 1, including regenerated output, on 2026-09-23.
Requirements authority: `Open_Platform_HLRs_26_09_22_07.md` (developer-owned).
Context: `/home/robertvanvossen/tools/r2u2-HAMR-agent-context`.

## Gate evidence

| Approved-plan criterion | Evidence and disposition |
|---|---|
| Requirements reviewed and allocated | `action-requests/CR-02-add-mode-manager/w1-requirements-planning-07.md`; _07 review complete, legacy publication remains developer-owned |
| Model and contracts type-check | Latest revised MAVLink monitor run: Well-formed!, exit 0; no model changes since |
| Three contract audits reviewed | Rx and ModeManager approved; revised MAVLink future-time monitor explicitly approved; dated audit records retain refinement obligations |
| Integration result and handshake count | `contracts/CR-02-integration-check.md`: N=0, vacuous by design; subsequent monitor-only revision adds no integration assumptions |
| Code generation and specification compilation | `codegen/CR-02-codegen.md`: R2 generation Success without warnings/errors, R2U2 compiler 4.2.4 succeeds; 12 editable files preserved by hash |
| Sampling and startup inspection | Source inspection below complete |
| Same-dispatch reporting feasibility | `monitoring/CR-02-r2u2-probe-workaround-output.txt`: 2/2 tests pass with authorized 4bc9a9a patch and candidate reporter |
| Build helper | `hamr/microkit/bin/build.cmd` exists; recorded usage check lists five components and two cores; application tests remain W2 |

No codegen or duplicate runtime test was needed for this closure inspection: model,
generated monitor and probe are unchanged from the recorded passing runs. Working
tree diff check passed. Earlier unpatched reporting failures remain historical
evidence, superseded for this configuration by the patched probe.

## Startup and control-path inspection

Generated `types/include/sb_aadl_types.h` declares `enum {Normal, Recovery}`;
Normal is zero. Rust `crates/data/src/open_platform_Data_Model/OperatingMode.rs`
explicitly assigns Normal=0, Recovery=1 and Default=Normal. This checks the actual
generated mapping rather than assuming the enum's zero value.

Both firewall C bridges declare file-scope `last_current_mode_payload`, which C
zero-initializes to Normal. Their `get_current_mode` returns that retained value
when no fresh sample is available. MAVLink's `peek_current_mode` does the same.
ModeManager's C bridge similarly zero-initializes `last_error_status_payload` to
false. Its receiver initialization starts the receive cursor at zero. These
generated defaults provide the intended fallback before a fresh publication;
they do not replace the application initialization calls required in W2.

Inspected source paths beneath `hamr/microkit`:

- `components/seL4_RxFirewall_RxFirewall/src/seL4_RxFirewall_RxFirewall.c`
- `components/seL4_MAVLinkFirewall_MAVLinkFirewall/src/seL4_MAVLinkFirewall_MAVLinkFirewall.c`
- `components/seL4_ModeManager_ModeManager/src/seL4_ModeManager_ModeManager.c`
- `types/src/sb_queue_open_platform_Data_Model_OperatingMode_1.c`

The model assembly connects MAVLink ErrorStatus to ModeManager and the manager's
two mode outputs to the corresponding firewall inputs, including process delegation.
ModeManager's generated initialization remains a scaffold. W2 must explicitly
initialize/publish Normal and initialize MAVLink count/status, then test those calls.

## Sampling and monitor lifecycle

`seL4_MAVLinkFirewall_MAVLinkFirewall/src/lib.rs` initializes the application then
the monitor, without a monitor step. Each compute entrypoint executes pre-hook,
application compute, post-hook in that order, with no Ethernet-presence guard.
The monitor pre-hook loads pre-count, threshold and Mode; its post-hook reads the
latest ErrorStatus and invokes exactly one R2U2 step. The C status peek accesses
the latest output queue publication. W2 must publish status on every dispatch as
the final application output update.

The generated mode peek dequeues through a copied receiver cursor; it does not
consume the application's input. Peek and get address the same sampled queue and
retained fallback. Under HAMR's frozen-input semantics they observe the same
dispatch snapshot. This source inspection is not a deployed concurrency proof:
W3 must verify queue/publication visibility with the actual schedule.

The formula uses pre-count below five plus final true status as the first-assertion
trigger, relying on the specified count/status relation. The probe supplies
reachable threshold-crossing traces and confirms raw failure delivery at D2.
With the authorized patch, the candidate synchronous logger emits one timeout
Error during D2, including late/absent Recovery and reboot cases. Timely D1/D2
and no-trigger traces produce no timeout. Its test sink is not the production logger.

## Authorized workaround and limitations

The developer explicitly authorized commit `4bc9a9ae60daefad311bcf23546ee8d4e7468c1b`
as a generated-code workaround and instructed reapplication after every codegen.
This supersedes the original plan's prohibition on depending on overwrite-only
edits for this specific fix. `patches/4bc9a9a-r2u2-false-verdict.patch`, the checked
application helper, and AGENTS.md make that exception repeatable. Conflicts stop
for inspection. Forward application, idempotence, exact commit match and conflict
preservation were tested. No timing requirement was weakened.

This is an authorized local reporting path, not an upstream generator fix. The
patch covers one specification's logging and does not fix alert routing. W2 must
integrate the one-time reporter in editable application logging, reset it at boot,
suppress routine status messages, and ensure log facade filtering cannot suppress
the Info-level verdict before the reporter observes it. No production logging
integration or application correctness is claimed by the W1 probe.

## Handoff and review

W2: implement ModeManager and affected firewalls; resolve truncated SECURE_COMMAND
operation-7 hook semantics; update stale oracle/test signatures; implement one-time
reporting and initialization; run coverage, tests and Verus including shared-core
and Tx regressions. Record the R2U2 trust boundary.

W3: reconcile custom.mk and schedule, build the target, measure D0/D1/D2 observation
and logging with runtime budget impact, and perform hardware acceptance. No
whole-system temporal proof is claimed; the integration pass is vacuous by design.

The latest CodeGen output review and W1 review can be accepted together. W2 remains
not started until the audited Wave 1 approval is recorded. No audit or implementation
requirement has been waived.
