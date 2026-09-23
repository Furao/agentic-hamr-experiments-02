# CR-02 ZCU102 acceptance record

Status: developer reports hardware behavior correct (2026-09-23).
Source: user statement, "hardware behavior is correct." No case-by-case captures or
timing measurements were supplied; rows below retain their evidence status. This
confirmation precedes the subsequent ModeManager transition-log edit.
Loader SHA-256: f524c83c893a10b0f20a638dc58a5d0b9784b9c0aaa4ee8f64ef3cbd5419cadf.
This image includes the approved 26_09_23_01 firewall bounds and driver defense.
Build evidence: CR-02-bounds-build.md. This record does not inherit CR-01 hardware results.

Use the ZCU102/debug loader built with SYSTEM_MAKEFILE=custom.mk, the legacy schedule,
and the authorized R2U2 false-verdict workaround. Record serial output and input/output
packet captures. Reboot between tests that must begin with count zero and Normal.
Use a controlled bench for blacklisted-command fixtures; acceptance expects no guest
delivery. Record actual fixture bytes, including valid MAVLink CRCs.

| Case | Stimulus and expected result | Result/evidence |
|---|---|---|
| Startup | Reboot: manager publishes Normal, count=0, ErrorStatus=false; no unsolicited Ethernet output | Pending |
| Normal routing | ARP and permitted direct UDP reach corresponding VMM lanes; valid v1/v2 MAVLink and FTP 110 traverse MAVLink firewall unchanged | Pending |
| Strict UDP | Other-source→68 forwards; 14550→68 drops; 14550→14562 takes MAVLink path; other prohibited pairs drop | Pending |
| Blacklist | COMMAND_INT and COMMAND_LONG 42650, and SECURE_COMMAND operation 7, are rejected; include v2 secure payload lengths 5, 6 and 7 | Pending |
| Malformed MAVLink | Valid UDP carriers with invalid MAVLink CRC/framing are rejected and counted once each | Pending |
| Transition | Cross count 4→5 or cross five with multiple lanes; final ErrorStatus=true; manager latches Recovery; both firewalls stop forwarding on their first frozen Recovery snapshot | Pending |
| Recovery | Allowed frames cause no suppression logs; malformed/blacklisted follow-up inputs reaching MAVLink still count; no Ethernet forwarding | Pending |
| Saturation | Count reaches 20 without wrapping; all lanes continue processing; ErrorStatus stays true, including empty dispatches | Pending |
| Latch/reset | Later false input to manager cannot restore Normal; reboot resets mode, count, monitor and reporting state | Pending |
| Tx regression | Normal guest-originated ARP/IPv4 traffic still transmits; mode changes do not introduce a Tx policy gate | Pending |
| Tx IPv4 bounds | Valid total length 1586 transmits as a 1600-byte Ethernet frame; 1587, 9000, 9001 and 65535 drop on every lane; subsequent valid traffic still transmits | Pending |
| Driver defense | Separately identified driver-input harness injects sizes 0, 1601 and 65535: no transmit/panic; sizes 1, 1599 and 1600 preserve bytes; later valid lanes still process. Normal Tx rejects oversized IPv4 before this interface, so exercise the guard directly | Pending |
| Rx carrier bounds | Both direct and MAVLink UDP routes accept structurally valid IPv4 length 1586 and reject 1587; MAVLink payload policy remains a separate check | Pending |
| Lane isolation | Distinct traffic on all four lanes preserves corresponding output lane and frame bytes | Pending |
| Nominal deadline | Trace D0 ErrorStatus and each frozen mode; Recovery arrives by D2 (expected D1 under approved order), with no timeout | Pending |
| Fault-injected deadline | Controlled test harness delays mode to D3 or never: exactly one Error during D2, including empty dispatches, no repeated errors; reboot permits a new diagnostic | Pending |
| Budget/units | Determine legacy schedule-length units, measure dispatch spacing and component+monitor execution/memory cost; reconcile model periods/budgets | Pending |

D0/D1/D2 and count assertions require a suitable dispatch/state trace or debugger;
ordinary packet captures alone cannot establish them. A stopped debugger can inspect
state but cannot establish execution-time bounds. Fault injection must be confined to
a separately identified test image or harness; do not claim it was exercised on the
production loader without evidence. Physical log timing is distinct from passing
host R2U2 timeline tests. The two-dispatch deadline is not a separate wall-clock bound.

The baseline oversized-Tx issue is corrected in this image: firewall IPv4 lengths
are bounded to 1586 and the driver independently checks its buffer before requesting
a token. Host tests, Verus and target build pass; physical acceptance of the cases
above remains pending. Identify any direct-driver injection harness separately from
the production loader.
