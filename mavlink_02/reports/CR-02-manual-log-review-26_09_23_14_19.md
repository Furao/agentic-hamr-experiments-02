# CR-02 manual hardware log review — 2026-09-23 14:19

Source: manual_test_results/26_09_23_14_19_open_platform.log
SHA-256: cd9c505763f68649075f4afef4b9ad3979024fe6b621b9c8fce5c7667871eada.
The raw log was inspected without modification; line numbers use LF delimiters.
It contains CRLF, interleaved serial output and a non-UTF-8 byte near shutdown.

## Requested sequence: confirmed in captured diagnostics

Exactly five MAVLinkFirewall firmware-flash denial records occur, all on lane 0:

| Denial | Log line | Diagnostic |
|---|---:|---|
| 1 | 673 | lane 0: firmware-flash command denied |
| 2 | 691 | lane 0: firmware-flash command denied |
| 3 | 714 | lane 0: firmware-flash command denied |
| 4 | 735 | lane 0: firmware-flash command denied |
| 5 | 766 | lane 0: firmware-flash command denied |

The next two physical lines, 767–768, contain one ModeManager record:
`Mode changed: Normal -> Recovery`. There are no other transition records in the
capture. Other RxFirewall packet rejection logs belong to the upstream network
filter and were excluded from this five-message count.

Source correspondence: each MAVLink DenyFlash branch increments rejections once,
sends no frame and emits this denial record. The final accumulated count produces
ErrorStatus at >=5. The ModeManager message occurs only after changing retained
Normal to Recovery. Thus the captured five-denial-then-transition sequence agrees
with the requested threshold behavior. The log does not directly print the counter
or packet IDs, so it does not distinguish COMMAND_INT, COMMAND_LONG and SECURE_COMMAND
or supply independent packet-capture proof of non-forwarding.

## Additional finding: D2 timeout reported

Lines 767–768 are character-interleaved output from two records. Removing only
CR/LF separators yields exactly 220 bytes, which were checked by an order-preserving
two-string interleaving matcher against these exact records:

```
INFO  [seL4_ModeManager_ModeManager::component::seL4_ModeManager_ModeManager_app] Mode changed: Normal -> Recovery
ERROR [seL4_MAVLinkFirewall_MAVLinkFirewall::logging] Mode-transition timeout: Recovery not observed by D2
```

The exact interleaving check passes: every byte is accounted for, with each record's
character order preserved. The timeout is present despite a literal text search for
its full message failing on the interleaved raw line.

The monitor therefore reported failure to observe Recovery by D2 in this run.
The state transition is confirmed, but timely publication/observation is not a pass.
ModeManager emits its message before its two output writes; serial interleaving alone
cannot establish exact dispatch boundaries, publication timing or the root cause.
No wall-clock deadline, scheduling diagnosis or monitor false-positive is inferred.
Further timing/dispatch investigation is needed to resolve this hardware finding.

## Disposition

Update hardware evidence with five denials followed by one actual transition log.
Developer subsequently accepted manual testing with high-criticality finding
CR-02-HW-01 retained open; see CR-02-HW-01-recovery-timeout.md. The manual testing
acceptance step is complete. The timeout investigation remains outstanding; overall
Wave 3 approval is separate.
This capture does not contain a build hash, so it demonstrates use of transition
logging but does not independently identify the exact loader image. No source,
requirements, test or generated code was changed for this review.
