# CR-02 ZCU102 acceptance record

Status: **ACCEPTED WITH OPEN HIGH-CRITICALITY FINDING CR-02-HW-01**.
Accepted by developer on 2026-09-23. See CR-02-HW-01-recovery-timeout.md.
This acceptance covers the supplied manual testing; it does not resolve the timeout
or claim that every individual procedure below has captured evidence.

Earlier general confirmation: developer reported hardware behavior correct (2026-09-23).
Source: user statement, "hardware behavior is correct." No case-by-case captures or
timing measurements were supplied; rows below retain their evidence status. This
confirmation precedes the subsequent ModeManager transition-log edit.
Loader at time of hardware confirmation: f524c83c893a10b0f20a638dc58a5d0b9784b9c0aaa4ee8f64ef3cbd5419cadf.
Prior rebuilt loader with transition diagnostic: cac588401b5a4853aa4a9d482f5a3743f5840063ee888f52027361893c7aba43.
The diagnostic build is not yet covered by a separate hardware confirmation;
see CR-02-mode-log.md for its tests, proof and build evidence.
This image includes the approved 26_09_23_01 firewall bounds and driver defense.
Build evidence: CR-02-bounds-build.md. This record does not inherit CR-01 hardware results.

Use the ZCU102/debug loader built with SYSTEM_MAKEFILE=custom.mk, the legacy schedule,
and the authorized R2U2 false-verdict workaround. Record serial output and input/output
packet captures. Reboot between tests that must begin with count zero and Normal.
Use a controlled bench for blacklisted-command fixtures; acceptance expects no guest
delivery. Record actual fixture bytes, including valid MAVLink CRCs.

| Case | Stimulus and expected result | Result/evidence |
|---|---|---|
| Startup | Reboot: manager publishes Normal, count=0, ErrorStatus=false; no unsolicited Ethernet output | No case-specific evidence supplied |
| Normal routing | ARP and permitted direct UDP reach corresponding VMM lanes; valid v1/v2 MAVLink and FTP 110 traverse MAVLink firewall unchanged | No case-specific evidence supplied |
| Strict UDP | Other-source→68 forwards; 14550→68 drops; 14550→14562 takes MAVLink path; other prohibited pairs drop | No case-specific evidence supplied |
| Blacklist | COMMAND_INT and COMMAND_LONG 42650, and SECURE_COMMAND operation 7, are rejected; include v2 secure payload lengths 5, 6 and 7 | No case-specific evidence supplied |
| Malformed MAVLink | Valid UDP carriers with invalid MAVLink CRC/framing are rejected and counted once each | No case-specific evidence supplied |
| Transition | Cross count 4→5 or cross five with multiple lanes; final ErrorStatus=true; manager latches Recovery; both firewalls stop forwarding on their first frozen Recovery snapshot | No case-specific evidence supplied |
| Recovery | Allowed frames cause no suppression logs; malformed/blacklisted follow-up inputs reaching MAVLink still count; no Ethernet forwarding | No case-specific evidence supplied |
| Saturation | Count reaches 20 without wrapping; all lanes continue processing; ErrorStatus stays true, including empty dispatches | No case-specific evidence supplied |
| Latch/reset | Later false input to manager cannot restore Normal; reboot resets mode, count, monitor and reporting state | No case-specific evidence supplied |
| Tx regression | Normal guest-originated ARP/IPv4 traffic still transmits; mode changes do not introduce a Tx policy gate | No case-specific evidence supplied |
| Tx IPv4 bounds | Valid total length 1586 transmits as a 1600-byte Ethernet frame; 1587, 9000, 9001 and 65535 drop on every lane; subsequent valid traffic still transmits | No case-specific evidence supplied |
| Driver defense | Separately identified driver-input harness injects sizes 0, 1601 and 65535: no transmit/panic; sizes 1, 1599 and 1600 preserve bytes; later valid lanes still process. Normal Tx rejects oversized IPv4 before this interface, so exercise the guard directly | No case-specific evidence supplied |
| Rx carrier bounds | Both direct and MAVLink UDP routes accept structurally valid IPv4 length 1586 and reject 1587; MAVLink payload policy remains a separate check | No case-specific evidence supplied |
| Lane isolation | Distinct traffic on all four lanes preserves corresponding output lane and frame bytes | No case-specific evidence supplied |
| Nominal deadline | Trace D0 ErrorStatus and each frozen mode; Recovery arrives by D2 (expected D1 under approved order), with no timeout | Timeout reported at lines 767–768 in supplied 14:19 log; investigate dispatch/publication/observation timing |
| Fault-injected deadline | Controlled test harness delays mode to D3 or never: exactly one Error during D2, including empty dispatches, no repeated errors; reboot permits a new diagnostic | No case-specific evidence supplied |
| Mode transition diagnostic | Exactly one Info message `Mode changed: Normal -> Recovery` on the transition | Observed once at lines 767–768 in supplied 14:19 log, after five MAVLink denials; reboot rearming not demonstrated by this capture |
| Budget/units | Determine legacy schedule-length units, measure dispatch spacing and component+monitor execution/memory cost; reconcile model periods/budgets | No case-specific evidence supplied |

D0/D1/D2 and count assertions require a suitable dispatch/state trace or debugger;
ordinary packet captures alone cannot establish them. A stopped debugger can inspect
state but cannot establish execution-time bounds. Fault injection must be confined to
a separately identified test image or harness; do not claim it was exercised on the
production loader without evidence. Physical log timing is distinct from passing
host R2U2 timeline tests. The two-dispatch deadline is not a separate wall-clock bound.

The baseline oversized-Tx issue is corrected in this image: firewall IPv4 lengths
are bounded to 1586 and the driver independently checks its buffer before requesting
a token. Host tests, Verus and target build pass; manual testing is now accepted with the open finding, while case-specific evidence
remains limited as shown above. Identify any direct-driver injection harness separately from
the production loader.

## Supplied manual log — 26_09_23_14_19

Reviewed manual_test_results/26_09_23_14_19_open_platform.log. Exactly five lane-0
MAVLink firmware-flash denials occur at lines 673, 691, 714, 735 and 766, followed by
one ModeManager Normal-to-Recovery record at 767–768. This supplies direct transition
logging evidence beyond the earlier general hardware confirmation.

The same two lines also contain an interleaved ERROR record: `Mode-transition timeout:
Recovery not observed by D2`. Exact byte-level interleaving validation confirms both
records. Nominal deadline acceptance is therefore not established; the monitor
reports a timeout requiring investigation. No exact loaded-image hash or dispatch
trace is present. See CR-02-manual-log-review-26_09_23_14_19.md for evidence and limits.

## Budget-increase retest image (subsequently reverted)

Developer requested +200 ms each for manager and MAVLink. Historical test-image SHA-256: `b55ce4b99309e469e413e9fa77bf34aee034c8a0df88c5b3467dc30b7c349079`. Model budgets are now 300/500 ms; legacy slot lengths are 30000/50000 using the retained scale. Full build/proofs and monitor probe pass. Repeat the five-rejection scenario and check for one Recovery diagnostic without the D2 timeout. This new run and physical legacy-time conversion remain unverified; see CR-02-budget-increase.md.

## Developer rollback — 2026-09-23

The developer reports that the +200 ms schedule/budget increase did not resolve
the problem and has reverted it. Source inspection confirms ModeManager 100 ms,
MAVLinkFirewall 300 ms, Frame_Period 2080 ms, and legacy domain_7/domain_6 lengths
10000/30000. The D2 timeout remains unresolved. Preserve this failed mitigation as
historical evidence; do not reapply it as an accepted fix. No codegen or rebuild was
performed while recording this update, so no new loader identity is claimed.

## Developer acceptance — 2026-09-23

Manual testing accepted with **CR-02-HW-01 (High, Open)** for the D2 timeout.
The five-denial-then-Recovery sequence is confirmed; the accompanying timeout is
retained as an unresolved finding. The unsuccessful +200 ms mitigation was reverted.
The table records evidence coverage, not a request to repeat acceptance. Requirement
conformance for the timeout and a root-cause resolution are not claimed.
