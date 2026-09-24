# CR-01 MAVLink Firewall Hardware Test Record

| Field | Value |
|---|---|
| Change | CR-01 Add MAVLink Firewall |
| Target | ZCU102 running the generated Microkit loader and ArduPilot Linux guest |
| Operator | Robbie VanVossen |
| Date | 2026-08-27 |
| Status | **Passed** |
| Build | `SYSTEM_MAKEFILE=custom.mk`, ZCU102/debug |

## 1. Purpose

Confirm the receive path and rejection logging on physical hardware. This record does
not infer results from unit tests or verification; each observation must be completed
by the operator from the ZCU102 console and ground-station traffic.

## 2. Preconditions

- Flash or load `hamr/microkit/build/loader.img` on the ZCU102.
- Connect the UAS operator/ground station to the ArduPilot UDP network path.
- Capture the Microkit serial console and ground-station receive log.
- Record packet fixtures or capture files used for every case.
- Confirm the build was produced with `SYSTEM_MAKEFILE=custom.mk`.

## 3. Test Cases

| ID | Stimulus | Expected result | Required evidence | Result |
|---|---|---|---|---|
| HW-01 | Valid MAVLink v1 message over UDP 14550 → 14562 | Message reaches ArduPilot unchanged | Input/output capture and console log | Passed |
| HW-02 | Valid MAVLink v2 message over UDP 14550 → 14562 | Message reaches ArduPilot unchanged | Input/output capture and console log | Passed |
| HW-03 | Valid `FILE_TRANSFER_PROTOCOL` (message 110) | Message reaches ArduPilot unchanged | Input/output capture identifying message ID 110 | Passed |
| HW-04 | `COMMAND_INT` or `COMMAND_LONG` carrying command 42650 | No delivery; firmware-flash denial reason logged | Input capture, absence at guest, rejection log | Passed |
| HW-05 | `SECURE_COMMAND` carrying operation 7 | No delivery; firmware-flash denial reason logged | Input capture, absence at guest, rejection log | Passed |
| HW-06 | Bad MAVLink checksum | No delivery; malformed-frame reason logged | Corrupted input and rejection log | Passed |
| HW-07 | Truncated, extra-byte, or unsupported-v2-incompatibility frame | No delivery; malformed-frame reason logged | Input capture and rejection log | Passed |
| HW-08 | TCP receive traffic | No delivery; RxFirewall rejection reason logged | TCP capture and rejection log | Passed |
| HW-09 | Permitted non-ArduPilot direct traffic | Traffic reaches the VMM without entering MAVLinkFirewall | Input/output capture and routing log | Passed |
| HW-10 | One valid message on each of the four lanes | Each lane reaches its corresponding VMM input | Per-lane markers/captures | Passed |
| HW-11 | Simultaneous/burst traffic across all four lanes | No cross-lane delivery; drops, if any, are observable | Timestamped per-lane captures and logs | Passed |
| HW-12 | Guest-originated transmit traffic | Existing ArduPilot → TxFirewall path remains operational | Guest transmit and external receive capture | Passed |

## 4. Observations and Deviations

Initial execution exposed incorrect rejection of legal MAVLink v2 trailing-zero payload
truncation as an invalid frame length. The parser and its verified specification were
corrected to accept transmitted v2 payload lengths up to the dialect maximum while
retaining exact MAVLink v1 lengths. The follow-up capture contains 221 messages; all
221 validate under the corrected rule. Robbie VanVossen reported the complete manual
test procedure passing after the correction.

## 5. Artifacts

| Artifact | Location or identifier |
|---|---|
| Serial console log | `manual_test_results/26_08_27_9_11/open_platform.log` |
| Ground-station/MAVLink capture | `manual_test_results/9_37_mavlink.json` |
| Initial MAVLink capture | `manual_test_results/26_08_27_9_11/open_platform_mavlink_messages.json` |
| Loader image hash | SHA-256 `f0189ae8ea42ef2e46b1257510ccf2544c2c420cbe2e0c6cce7624be978156e9` |

## 6. Sign-off

| Role | Name | Decision | Date |
|---|---|---|---|
| Hardware-test operator | Robbie VanVossen | Pass | 2026-08-27 |
| Developer reviewer | Robbie VanVossen | Approved with W4 wave gate | 2026-08-27 |
