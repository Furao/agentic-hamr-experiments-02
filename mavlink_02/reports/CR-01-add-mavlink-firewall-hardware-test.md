# CR-01 MAVLink Firewall Hardware Test Record

| Field | Value |
|---|---|
| Change | CR-01 Add MAVLink Firewall |
| Target | ZCU102 running the generated Microkit loader and ArduPilot Linux guest |
| Operator | Pending |
| Date | Pending |
| Status | **Pending manual execution** |
| Build | `SYSTEM_MAKEFILE=custom.mk`, ZCU102/debug, 147.98 MiB loader image |

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
| HW-01 | Valid MAVLink v1 message over UDP 14550 → 14562 | Message reaches ArduPilot unchanged | Input/output capture and console log | Pending |
| HW-02 | Valid MAVLink v2 message over UDP 14550 → 14562 | Message reaches ArduPilot unchanged | Input/output capture and console log | Pending |
| HW-03 | Valid `FILE_TRANSFER_PROTOCOL` (message 110) | Message reaches ArduPilot unchanged | Input/output capture identifying message ID 110 | Pending |
| HW-04 | `COMMAND_INT` or `COMMAND_LONG` carrying command 42650 | No delivery; firmware-flash denial reason logged | Input capture, absence at guest, rejection log | Pending |
| HW-05 | `SECURE_COMMAND` carrying operation 7 | No delivery; firmware-flash denial reason logged | Input capture, absence at guest, rejection log | Pending |
| HW-06 | Bad MAVLink checksum | No delivery; malformed-frame reason logged | Corrupted input and rejection log | Pending |
| HW-07 | Truncated, extra-byte, or unsupported-v2-incompatibility frame | No delivery; malformed-frame reason logged | Input capture and rejection log | Pending |
| HW-08 | TCP receive traffic | No delivery; RxFirewall rejection reason logged | TCP capture and rejection log | Pending |
| HW-09 | Permitted non-ArduPilot direct traffic | Traffic reaches the VMM without entering MAVLinkFirewall | Input/output capture and routing log | Pending |
| HW-10 | One valid message on each of the four lanes | Each lane reaches its corresponding VMM input | Per-lane markers/captures | Pending |
| HW-11 | Simultaneous/burst traffic across all four lanes | No cross-lane delivery; drops, if any, are observable | Timestamped per-lane captures and logs | Pending |
| HW-12 | Guest-originated transmit traffic | Existing ArduPilot → TxFirewall path remains operational | Guest transmit and external receive capture | Pending |

## 4. Observations and Deviations

Pending manual execution. Record unexpected output, dropped-event counters, timing
observations, and any deviation from the expected result here.

## 5. Artifacts

| Artifact | Location or identifier |
|---|---|
| Serial console log | Pending |
| Ground-station log | Pending |
| Packet captures/fixtures | Pending |
| Loader image hash | SHA-256 `9d8ad2303aa3391079b435ff7c7f55ab5155a7e93b86d9eb72bb597505aa0936` |

## 6. Sign-off

| Role | Name | Decision | Date |
|---|---|---|---|
| Hardware-test operator | Pending | Pending | Pending |
| Developer reviewer | Pending | Pending | Pending |
