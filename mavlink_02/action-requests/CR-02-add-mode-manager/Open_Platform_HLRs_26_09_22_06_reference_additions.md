High Level Requirements  
C\_ROCKWELL\_COLLINS-INSPECTA

22 September 2026

Table of Contents
=================

[1.0 RX\_Firewall 4](#rx_firewall)

[1.1 Rx\_firewall: Copy through any ARP frame to VMM output
(*RC\_INSPECTA\_00-HLR-5*)))
4](#rx_firewall-copy-through-any-arp-frame-to-vmm-output-rc_inspecta_00-hlr-5)

[1.2 Rx\_firewall: Copy through allowed UDP port frames to VMM output
(*RC\_INSPECTA\_00-HLR-13*)))
4](#rx_firewall-copy-through-allowed-udp-port-frames-to-vmm-output-rc_inspecta_00-hlr-13)

[1.3 Rx\_firewall: Copy through mavlink UDP port frames to the
mavlink\_firewall output (*RC\_INSPECTA\_00-HLR-18*)))
5](#rx_firewall-copy-through-mavlink-udp-port-frames-to-the-mavlink_firewall-output-rc_inspecta_00-hlr-18)

[1.4 Rx\_firewall: Do not copy disallowed frame
(*RC\_INSPECTA\_00-HLR-15*)))
6](#rx_firewall-do-not-copy-disallowed-frame-rc_inspecta_00-hlr-15)

[1.5 Rx\_firewall: No ethernet output during Recovery Mode
(*RC\_INSPECTA\_00-HLR-29*)))
6](#rx_firewall-no-ethernet-output-during-recovery-mode-rc_inspecta_00-hlr-29)

[1.6 Rx\_firewall: No ethernet output on empty ethernet input
(*RC\_INSPECTA\_00-HLR-17*)))
6](#rx_firewall-no-ethernet-output-on-empty-ethernet-input-rc_inspecta_00-hlr-17)

[2.0 TX\_Firewall 6](#tx_firewall)

[2.1 Tx\_firewall: Copy through any ARP frame
(*RC\_INSPECTA\_00-HLR-7*)))
6](#tx_firewall-copy-through-any-arp-frame-rc_inspecta_00-hlr-7)

[2.2 Tx\_firewall: Copy through any IPv4 frame
(*RC\_INSPECTA\_00-HLR-12*)))
6](#tx_firewall-copy-through-any-ipv4-frame-rc_inspecta_00-hlr-12)

[2.3 Tx\_firewall: Do not copy disallowed frame
(*RC\_INSPECTA\_00-HLR-14*)))
7](#tx_firewall-do-not-copy-disallowed-frame-rc_inspecta_00-hlr-14)

[2.4 Tx\_firewall: No output on empty input
(*RC\_INSPECTA\_00-HLR-16*)))
7](#tx_firewall-no-output-on-empty-input-rc_inspecta_00-hlr-16)

[3.0 Mavlink\_Firewall 7](#mavlink_firewall)

[3.1 Mavlink\_Firewall: Drop flash\_bootloader mav\_command message
(*RC\_INSPECTA\_00-HLR-19*)))
7](#mavlink_firewall-drop-flash_bootloader-mav_command-message-rc_inspecta_00-hlr-19)

[3.2 Mavlink\_Firewall: Drop SECURE\_COMMAND\_FLASH\_BOOTLOADER messages
(*RC\_INSPECTA\_00-HLR-32*)))
8](#mavlink_firewall-drop-secure_command_flash_bootloader-messages-rc_inspecta_00-hlr-32)

[3.3 Mavlink\_Firewall: Drop malformed mavlink messages
(*RC\_INSPECTA\_00-HLR-20*)))
9](#mavlink_firewall-drop-malformed-mavlink-messages-rc_inspecta_00-hlr-20)

[3.4 Mavlink\_firewall: No ethernet output on corresponding empty
ethernet input (*RC\_INSPECTA\_00-HLR-21*)))
9](#mavlink_firewall-no-ethernet-output-on-corresponding-empty-ethernet-input-rc_inspecta_00-hlr-21)

[3.5 Mavlink\_firewall: No ethernet output during Recovery Mode
(*RC\_INSPECTA\_00-HLR-28*)))
9](#mavlink_firewall-no-ethernet-output-during-recovery-mode-rc_inspecta_00-hlr-28)

[3.6 Mavlink\_Firewall: Copy through well-formed, not-blacklisted
messages (*RC\_INSPECTA\_00-HLR-22*)))
9](#mavlink_firewall-copy-through-well-formed-not-blacklisted-messages-rc_inspecta_00-hlr-22)

[3.7 Mavlink\_Firewall: Send Error status to Mode\_Manager
(*RC\_INSPECTA\_00-HLR-25*)))
9](#mavlink_firewall-send-error-status-to-mode_manager-rc_inspecta_00-hlr-25)

[3.8 Mavlink\_Firewall: Monitor Mode Change
(*RC\_INSPECTA\_00-HLR-30*)))
9](#mavlink_firewall-monitor-mode-change-rc_inspecta_00-hlr-30)

[3.9 Mavlink\_Firewall: Track number of rejected messages
(*RC\_INSPECTA\_00-HLR-31*)))
10](#mavlink_firewall-track-number-of-rejected-messages-rc_inspecta_00-hlr-31)

[4.0 Mode\_Manager 10](#mode_manager)

[4.1 Mode\_Manager: Initialization (*RC\_INSPECTA\_00-HLR-23*)))
10](#mode_manager-initialization-rc_inspecta_00-hlr-23)

[4.2 Mode\_Manager: Modes (*RC\_INSPECTA\_00-HLR-24*)))
10](#mode_manager-modes-rc_inspecta_00-hlr-24)

[4.3 Mode\_Manager: Send current mode (*RC\_INSPECTA\_00-HLR-26*)))
10](#mode_manager-send-current-mode-rc_inspecta_00-hlr-26)

[4.4 Mode\_Manager: Transition to Recovery Mode
(*RC\_INSPECTA\_00-HLR-27*)))
10](#mode_manager-transition-to-recovery-mode-rc_inspecta_00-hlr-27)

RX\_Firewall 
============

Rx\_firewall: Copy through any ARP frame to VMM output ([*RC\_INSPECTA\_00-HLR-5*])
------------------------------------------------------------------------------------------------------------------------------------------------------------

During Normal Mode, the RX firewall shall copy a frame from an input
port to its VMM output port if that frame has a wellformed ethheader,
the ethernet type is ARP, and the ARP packet is wellformed.

 

-   An ethernet header is wellformed if the ethernet type is valid and
    > the destination address is valid.

    -   The ethernet type is valid if bytes 12-13 of the frame are
        > 0x0800 or 0x0806 or 0x86DD.

    -   The destination address is valid if bytes 0-5 of the frame are
        > not 0x0000000000.

-   An ethernet type is ARP if bytes 12-13 of the frame are 0x0806.

-   An ARP packet is wellformed if the ARP Operation is valid and the
    > ARP hardware type is valid and the ARP protocol type is valid.

    -   The ARP Operation is valid if bytes 20-21 of the frame are
        > 0x0001 or 0x0002.

    -   The ARP hardware type is valid if bytes 14-15 of the frame are
        > 0x0001.

    -   The ARP protocol type is valid if bytes 16-17 of the frame are
        > 0x0800 or 0x86DD.

Rx\_firewall: Copy through allowed UDP port frames to VMM output ([*RC\_INSPECTA\_00-HLR-13*])
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

During Normal Mode, the RX firewall shall copy a frame from an input
port to its VMM output port's message if that frame has a wellformed
ethheader, the ethernet type is IPv4, the IPv4 packet is wellformed, the
IPv4 packet uses the UDP protocol, the UDP source port is not from GCS,
the UDP destination port is not for ardupilot, and the UDP destination
port is in the UDP port whitelist.

 

-   An ethernet header is wellformed if the ethernet type is valid and
    > the destination address is valid.

    -   The ethernet type is valid if bytes 12-13 of the frame are
        > 0x0800 or 0x0806 or 0x86DD.

    -   The destination address is valid if bytes 0-5 of the frame are
        > not 0x0000000000.

-   An IPv4 packet is wellformed if the IPv4 protocol is valid, the
    > version is 4, the IHL indicates no IPv4 options in the header, and
    > the IPv4 length is valid.

    -   The IPv4 protocol is valid if byte 23 of the frame is 0x00 or
        > 0x01 or 0x02 or 0x06 or 0x11 or 0x2B or 0x2C or 0x3A or
        > 0x3B or 0x3C.

    -   The IPv4 length is valid if bytes 16-17 of the frame are
        > &lt;= 9000.

    -   The IPv4 version is 4 and the IHL indicates no IPv4 options in
        > the header if byte 14 is 0x45.

-   An IPv4 packet uses the UDP protocol if byte 23 of the frame is
    > 0x11.

-   The UDP source port is not from a GCS if bytes 34-35 of the frame
    > are not 14550.

-   The UDP destination port is not for ardupilot if bytes 36-37 of the
    > frame are not 14562.

-   The UDP destination port is in the whitelist if bytes 36-37 of the
    > frame are one of the following:

    -   \[68\]

 

A maximum IPv4 length of 9000 is selected since it is the standard JUMBO
frame size for Maximum Transmission Unit (MTU). In most cases it will be
closer to 1500. 

Rx\_firewall: Copy through mavlink UDP port frames to the mavlink\_firewall output ([*RC\_INSPECTA\_00-HLR-18*])
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

During Normal Mode, the RX firewall shall copy a frame from an input
port to its mavlink\_firewall output port's message if that frame has a
wellformed ethheader, the ethernet type is IPv4, the IPv4 packet is
wellformed, the IPv4 packet uses the UDP protocol, the UDP source port
is from GCS, and the UDP destination port is for ardupilot.

 

-   An ethernet header is wellformed if the ethernet type is valid and
    > the destination address is valid.

    -   The ethernet type is valid if bytes 12-13 of the frame are
        > 0x0800 or 0x0806 or 0x86DD.

    -   The destination address is valid if bytes 0-5 of the frame are
        > not 0x0000000000.

-   An IPv4 packet is wellformed if the IPv4 protocol is valid, the
    > version is 4, the IHL indicates no IPv4 options in the header, and
    > the IPv4 length is valid.

    -   The IPv4 protocol is valid if byte 23 of the frame is 0x00 or
        > 0x01 or 0x02 or 0x06 or 0x11 or 0x2B or 0x2C or 0x3A or
        > 0x3B or 0x3C.

    -   The IPv4 length is valid if bytes 16-17 of the frame are
        > &lt;= 9000.

    -   The IPv4 version is 4 and the IHL indicates no IPv4 options in
        > the header if byte 14 is 0x45.

-   An IPv4 packet uses the UDP protocol if byte 23 of the frame is
    > 0x11.

-   The UDP source port is from a GCS if bytes 34-35 of the frame
    > are 14550.

-   The UDP destination port is for ardupilot if bytes 36-37 of the
    > frame are 14562.

A maximum IPv4 length of 9000 is selected since it is the standard JUMBO
frame size for Maximum Transmission Unit (MTU). In most cases it will be
closer to 1500. 

Rx\_firewall: Do not copy disallowed frame ([*RC\_INSPECTA\_00-HLR-15*])
-------------------------------------------------------------------------------------------------------------------------------------------------

The RX firewall shall not copy any frame originating from an input port
to any of its output ports if it does not match a valid frame as defined
in the other HLRs.

Rx\_firewall: No ethernet output during Recovery Mode ([*RC\_INSPECTA\_00-HLR-29*])
------------------------------------------------------------------------------------------------------------------------------------------------------------

During Recovery Mode, the RX firewall shall not place any data on any
ethernet output ports. This includes the same dispatch that the mode
changes from Normal mode to Recovery mode.

Rx\_firewall: No ethernet output on empty ethernet input ([*RC\_INSPECTA\_00-HLR-17*])
---------------------------------------------------------------------------------------------------------------------------------------------------------------

The RX firewall shall not place any data in any ethernet output port
when its corresponding ethernet input port does not have any data
available.

TX\_Firewall 
============

Tx\_firewall: Copy through any ARP frame ([*RC\_INSPECTA\_00-HLR-7*])
----------------------------------------------------------------------------------------------------------------------------------------------

The TX firewall shall copy a frame from an input port to its output
port's message if that frame has a wellformed ethheader, the ethernet
type is ARP, and the ARP packet is wellformed. A size of 64 is provided
in the output port's size.

 

-   An ethernet header is wellformed if the ethernet type is valid and
    > the destination address is valid.

    -   The ethernet type is valid if bytes 12-13 of the frame are
        > 0x0800 or 0x0806 or 0x86DD.

    -   The destination address is valid if bytes 0-5 of the frame are
        > not 0x0000000000.

-   An ethernet type is ARP if bytes 12-13 of the frame are 0x0806.

-   An ARP packet is wellformed if the ARP Operation is valid and the
    > ARP hardware type is valid and the ARP protocol type is valid.

    -   The ARP Operation is valid if bytes 20-21 of the frame are
        > 0x0001 or 0x0002.

    -   The ARP hardware type is valid if bytes 14-15 of the frame are
        > 0x0001.

    -   The ARP protocol type is valid if bytes 16-17 of the frame are
        > 0x0800 or 0x86DD.

Tx\_firewall: Copy through any IPv4 frame ([*RC\_INSPECTA\_00-HLR-12*])
------------------------------------------------------------------------------------------------------------------------------------------------

The TX firewall shall copy a frame from an input port to its output
port's message if that frame has a wellformed ethheader, the ethernet
type is IPv4, and the IPv4 packet is wellformed. The sum of the total
size provided by the IPv4 header and the 14 bytes of the ethernet header
is provided in the output port's size.

 

-   An ethernet header is wellformed if the ethernet type is valid and
    > the destination address is valid.

    -   The ethernet type is valid if bytes 12-13 of the frame are
        > 0x0800 or 0x0806 or 0x86DD.

    -   The destination address is valid if bytes 0-5 of the frame are
        > not 0x0000000000.

-   An ethernet type is IPv4 if bytes 12-13 of the frame are 0x0800.

-   An IPv4 packet is wellformed if the IPv4 protocol is valid, the
    > version is 4, the IHL indicates no IPv4 options in the header, and
    > the IPv4 length is valid.

    -   The IPv4 protocol is valid if byte 23 of the frame is 0x00 or
        > 0x01 or 0x02 or 0x06 or 0x11 or 0x2B or 0x2C or 0x3A or
        > 0x3B or 0x3C.

    -   The IPv4 length is valid if bytes 16-17 of the frame are
        > &lt;= 9000.

    -   The IPv4 version is 4 and the IHL indicates no IPv4 options in
        > the header if byte 14 is 0x45.

 

A maximum IPv4 length of 9000 is selected since it is the standard JUMBO
frame size for Maximum Transmission Unit (MTU). In most cases it will be
closer to 1500. 

 

Tx\_firewall: Do not copy disallowed frame ([*RC\_INSPECTA\_00-HLR-14*])
-------------------------------------------------------------------------------------------------------------------------------------------------

The TX firewall shall not copy any frame originating from an input port
to its output port if it does not match a valid copy frame as defined in
other HLRs.  

Tx\_firewall: No output on empty input ([*RC\_INSPECTA\_00-HLR-16*])
---------------------------------------------------------------------------------------------------------------------------------------------

The TX firewall shall not place any data in an output port when its
corresponding input port does not have any data available.

Mavlink\_Firewall 
=================

Mavlink\_Firewall: Drop flash\_bootloader mav\_command message ([*RC\_INSPECTA\_00-HLR-19*])
---------------------------------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink firewall shall drop a frame from an input port if the
mavlink message of the payload has a message id of CommandInt or
CommandLong and the message payload's command is
MAV\_CMD\_FLASH\_BOOTLOADER.

 
\- The mavlink protocol is v2 if byte 0 is 0xFD

    - The mavlink message, bytes 7-9, has the ID of CommandInt: 75 or CommandLong: 76

        - The message payload's mav command, bytes 38-39, has
MAV\_CMD\_FLASH\_BOOTLOADER: 42650

 
OR


\- The mavlink protocol is v1 if byte 0 is 0xFE

    - The mavlink message, byte 5, has the ID of CommandInt: 75 or CommandLong: 76

        - The message payload's mav command, bytes 34-35, has
MAV\_CMD\_FLASH\_BOOTLOADER: 42650

 

Mavlink\_Firewall: Drop SECURE\_COMMAND\_FLASH\_BOOTLOADER messages ([*RC\_INSPECTA\_00-HLR-32*])
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink firewall shall drop a frame from an input port if the
mavlink message of the payload has a message id of SECURE\_COMMAND and
the message payload's operation is SECURE\_COMMAND\_FLASH\_BOOTLOADER.

 

\- The mavlink protocol is v2 if byte 0 is 0xFD

    - The mavlink message, bytes 7-9, has the ID of SECURE\_COMMAND:
11004

        - The message payload's operation, bytes 14-17, has
SECURE\_COMMAND\_FLASH\_BOOTLOADER: 7

Mavlink\_Firewall: Drop malformed mavlink messages ([*RC\_INSPECTA\_00-HLR-20*])
---------------------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink firewall shall drop an input frame if the mavlink message of
the payload is malformed.

Mavlink\_firewall: No ethernet output on corresponding empty ethernet input ([*RC\_INSPECTA\_00-HLR-21*])
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink firewall shall not place any data in an ethernet output port
when its corresponding ethernet input port does not have any data
available.

Mavlink\_firewall: No ethernet output during Recovery Mode ([*RC\_INSPECTA\_00-HLR-28*])
-----------------------------------------------------------------------------------------------------------------------------------------------------------------

During Recovery Mode, the Mavlink firewall shall not place any data in
any ethernet output ports. This includes the same dispatch that the mode
changes from Normal mode to Recovery mode.

Mavlink\_Firewall: Copy through well-formed, not-blacklisted messages ([*RC\_INSPECTA\_00-HLR-22*])
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink firewall shall copy through a mavlink message from an input
port to its corresponding output port if it is well-formed, it is not in
the blacklist, and the Mode is Normal Mode. 

Mavlink\_Firewall: Send Error status to Mode\_Manager ([*RC\_INSPECTA\_00-HLR-25*])
------------------------------------------------------------------------------------------------------------------------------------------------------------

If 5 messages or more have been rejected, then the Mavlink Firewall
shall set the Mode Manager error boolean to true, otherwise set it to
false. This shall be done last during each dispatch.

Mavlink\_Firewall: Monitor Mode Change ([*RC\_INSPECTA\_00-HLR-30*])
---------------------------------------------------------------------------------------------------------------------------------------------

The MAVLink Firewall shall monitor the transition to Recovery Mode
following its first assertion of the Mode Manager Error Status boolean
to true. The dispatch in which this assertion occurs shall be designated
D0; subsequent MAVLink Firewall dispatches shall be designated D1, D2,
D3, and so on.

Recovery Mode shall be considered timely if observed at the Mode input
by D2. If Recovery Mode has not been observed by the start of D2, the
MAVLink Firewall shall log a mode-transition timeout error during D2,
even if Recovery Mode is observed during D3 or is never received.

 

Repeated assertions of Error Status shall not restart the timer. The
timeout error shall be logged at most once per system boot.

Mavlink\_Firewall: Track number of rejected messages ([*RC\_INSPECTA\_00-HLR-31*])
-----------------------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink Firewall shall track the number of messages that have been
rejected, due to malformed messages and blacklisted messages, since boot
time. Counting shall start at 0 and stop at 20. A single message being
rejected adds one to the counter, even if it meets multiple rejection
criteria.

This includes all input channels at each dispatch, meaning that the
rejected message counter could increase up to 4 each dispatch. If
multiple messages are rejected in the same dispatch and that would bring
the count above 20, then counting shall stop in the middle of the
dispatch but not stop processing the remaining inputs.

Mode\_Manager 
=============

Mode\_Manager: Initialization ([*RC\_INSPECTA\_00-HLR-23*])
------------------------------------------------------------------------------------------------------------------------------------

The Mode manager shall start in Normal mode. 

Mode\_Manager: Modes ([*RC\_INSPECTA\_00-HLR-24*])
---------------------------------------------------------------------------------------------------------------------------

The Mode\_Manager shall support a Normal Mode and a Recovery Mode.

Mode\_Manager: Send current mode ([*RC\_INSPECTA\_00-HLR-26*])
---------------------------------------------------------------------------------------------------------------------------------------

On each dispatch, the Mode Manager shall send the current mode (an enum)
to the RX\_Firewall and to the Mavlink\_Firewall. All clients shall
assume Normal Mode if the Mode Manager has not run yet.

Mode\_Manager: Transition to Recovery Mode ([*RC\_INSPECTA\_00-HLR-27*])
-------------------------------------------------------------------------------------------------------------------------------------------------

If the Mavlink Firewall supplies true for the Error Status boolean, then
the Mode Manager shall set the Mode to Recovery Mode and shall publish
Recovery mode during that same dispatch. The Mode Manager keeps the
system in Recovery Mode until the system is rebooted. 


---

# Suggested supplemental requirements — CR-02 user reference

Date: 22 September 2026.

**Status: reference draft for developer review.** Everything before this appendix is
an exact copy of `Open_Platform_HLRs_26_09_22_06.md`. The additions below consolidate
missing baseline refinements and the approved change-plan choices. This reference
does not supersede the developer-supplied requirements or complete Wave 1's manual
requirements updates.

The proposed identifiers `CR02-DR-*` distinguish derived requirements from the
unchanged `RC_INSPECTA_00-HLR-*` identifiers. In this appendix, `HLR-n` always means
the supplied revision _06 requirement. Historical CR-01 identifiers are explicitly
qualified. R2U2, sampled control ports and Recovery counting were approved design
choices; their implementation and validation remain future work.

## A. Interfaces and receive-path behavior

### CR02-DR-01 — Four-lane capacity and correspondence

The Ethernet data interfaces between the driver, RxFirewall, MAVLinkFirewall and VMM
shall retain four corresponding event-data lanes, numbered 0 through 3, each with
queue capacity one. A frame forwarded from lane n shall appear only on the
corresponding output lane n. Each firewall shall process the available input on each
of its four lanes at each dispatch.

Trace: CR-02 sketch; HLR-17, HLR-18, HLR-21 and HLR-31. Retains the baseline capacity
obligation without reusing historical CR-01 HLR-30.

### CR02-DR-02 — Exclusive Rx routing

For each input frame, RxFirewall shall emit at most one corresponding frame output:
either the direct VMM output or the MAVLinkFirewall output. In Recovery, neither
output shall contain a frame. In Normal, a frame satisfying neither permitted route
shall be dropped. TCP frames shall not be forwarded under the supplied receive policy.

Trace: HLR-5, HLR-13, HLR-15, HLR-18 and HLR-29.

### CR02-DR-03 — Preserved bounded MAVLink carrier

For a frame routed under HLR-18, RxFirewall shall produce a
`MAVLinkUDPMessage_Impl` containing the unchanged Ethernet frame and the offset and
length of its validated UDP payload. The offset and length shall identify bytes
within the carrier and the containing IPv4 packet. MAVLinkFirewall shall forward an
allowed carrier unchanged, including its frame, offset and length.

Trace: HLR-18 and HLR-22; retained baseline interface refinement.

### CR02-DR-04 — Networking bounds and safe field access

RxFirewall shall validate the bounds needed for every Ethernet, IPv4 and UDP field
it reads. IPv4 total length shall be at least the IPv4 header length and shall fit
within the supported carrier after the Ethernet header. UDP length shall be at least
eight bytes and shall agree with the containing IPv4 payload length. No declared
length or arithmetic wraparound shall permit access beyond the carrier or inclusion
of bytes outside the validated UDP payload.

The existing Ethernet carrier shall remain 1600 bytes. The supplied 9000-byte IPv4
upper limit shall not be interpreted as authorizing expansion of that carrier.

Trace: HLR-13, HLR-15 and HLR-18; existing data dictionary and bounded-carrier contract.
This explicitly retains the baseline capacity refinement; it adds no jumbo-frame
support or new network authentication checks.

### CR02-DR-05 — Networking and MAVLink separation

RxFirewall and firewall_core shall determine network routing and validated payload
bounds without parsing MAVLink. MAVLinkFirewall shall use the supplied bounded
payload for MAVLink parsing and blacklist classification and shall not implement
Ethernet/IP/UDP routing policy. Shared carrier validity checks shall preserve this
separation of responsibilities.

Trace: CR-02 sketch; retained historical CR-01 HLR-31 design constraint.

## B. MAVLink validity and policy refinements

### CR02-DR-06 — Complete MAVLink framing

A supplied UDP payload shall be accepted as well-formed only when it contains exactly
one complete MAVLink v1 or v2 frame. Validation shall include version magic, header
and payload bounds, message identifier, supported v2 incompatibility flags, checksum
including dialect CRC extra, and complete signature bytes when the signature-present
flag is set. Incomplete frames and trailing bytes outside that single frame shall be
rejected under HLR-20.

Message metadata shall use the project's bundled ArduPilotMega dialect and its
included common, standard and minimal definitions. Unsupported message metadata or
framing shall not bypass validation. These checks establish structural validity;
cryptographic signature authentication is not added by this requirement.

Trace: HLR-20 and HLR-22; retained baseline structural-validity refinement.

### CR02-DR-07 — Legal MAVLink v2 truncation

MAVLink v2 validation shall permit legal omission of a trailing all-zero payload
suffix, subject to the supported message's maximum payload length and complete wire
framing. MAVLink v1 shall retain its fixed dialect payload length. Policy field reads
shall respect the transmitted payload bounds and shall not read checksum, signature,
padding or unrelated carrier bytes as command data.

Trace: HLR-19, HLR-20, HLR-22 and HLR-32; retained baseline truncation/bounds behavior.

### CR02-DR-08 — Explicit blacklist and allowed file transfer

The blacklist shall include COMMAND_INT and COMMAND_LONG carrying command 42650
(HLR-19), and SECURE_COMMAND message 11004 carrying operation 7 (HLR-32).
Well-formed messages shall not be rejected solely because they implement file transfer.
In particular, a well-formed FILE_TRANSFER_PROTOCOL message shall be forwarded in
Normal mode when no blacklist rule applies. Recovery suppression remains applicable.

Trace: HLR-19, HLR-22, HLR-28 and HLR-32; retained baseline allow-FTP behavior.

### CR02-DR-09 — Frame-output validity and failure handling

Each frame emitted by MAVLinkFirewall shall be a well-formed, nonblacklisted carrier
copied from the corresponding input lane while its frozen Mode input is Normal.
Absent input, failed validation or inability to process an input safely shall produce
no corresponding Ethernet output. These frame-output rules shall not suppress the
ErrorStatus output required by HLR-25.

Trace: HLR-20, HLR-21, HLR-22, HLR-25 and HLR-28.

## C. Modes, counting and dispatch semantics

### CR02-DR-10 — Frozen mode observation

Each firewall shall use the Mode value frozen immediately before its own dispatch
for all forwarding decisions during that dispatch. If that value is Recovery, it
shall suppress every Ethernet output for that dispatch, including outputs associated
with frames present in the same frozen input snapshot. A mode publication occurring
after input freezing shall not change the current dispatch's decisions.

RxFirewall suppression shall cover its four direct outputs and four MAVLink carrier
outputs; MAVLinkFirewall suppression shall cover its four Ethernet carrier outputs.

Trace: HLR-28 and HLR-29; developer's HAMR dispatch clarification.

### CR02-DR-11 — Sampled control ports and initialization

Mode and ErrorStatus shall be communicated through sampled data ports. The initial
Mode value available to each consumer shall be Normal. MAVLinkFirewall's initial
rejection count shall be zero and its initial ErrorStatus shall be false.
ModeManager shall initialize its retained mode to Normal and publish its current mode
to both consumers on each compute dispatch.

Trace: HLR-23, HLR-24, HLR-25, HLR-26 and HLR-31; approved sampled-port allocation.

### CR02-DR-12 — Post-count ErrorStatus

At each dispatch, MAVLinkFirewall shall process the available input on all four lanes,
update its rejection count, and then publish ErrorStatus as its final application
output update. ErrorStatus shall equal true exactly when the resulting count is at
least five. Publication shall occur even when all Ethernet inputs are empty or the
frozen Mode input is Recovery.

Trace: HLR-25 and HLR-31. Generated post-dispatch monitoring observes this final value.

### CR02-DR-13 — Saturation and Recovery counting

MAVLinkFirewall shall continue classifying available inputs during Recovery.
Malformed or blacklisted messages shall each contribute exactly one rejection,
including when multiple rejection criteria apply. Messages suppressed solely because
of Recovery, allowed messages and absent inputs shall contribute zero.

For each dispatch:

`new_count = min(20, previous_count + rejected_messages_this_dispatch)`

The count shall never decrease before reboot. Reaching twenty shall stop further
increments without stopping processing of remaining inputs. Reboot shall reset the
count to zero.

Trace: HLR-31; approved Recovery-counting interpretation.

### CR02-DR-14 — Mode transition and publication

ModeManager shall enter Recovery if its frozen ErrorStatus input is true or its
retained mode is already Recovery. It shall publish that resulting mode to both
firewalls during the same dispatch. A later false ErrorStatus shall not restore
Normal. Normal shall be restored only through system reboot initialization.

Trace: HLR-23, HLR-26 and HLR-27.

## D. R2U2 monitoring and diagnostics

### CR02-DR-15 — Generated temporal monitor allocation

HLR-30 shall be implemented using an R2U2 monitor generated by HAMR from a GUMBO
monitor specification associated with MAVLinkFirewall. One logical monitor sample
shall correspond to one MAVLinkFirewall compute dispatch, including dispatches with
no Ethernet input. The monitor shall observe the frozen Mode input and the final
ErrorStatus output for that dispatch. Initialization shall not shift the relative
D0/D1/D2 indexing.

Trace: HLR-30; approved R2U2 implementation choice.

### CR02-DR-16 — First-assertion deadline and one-time reporting

The monitor shall track the obligation triggered by the first true ErrorStatus
assertion at D0. Recovery present in the frozen Mode input at D1 or D2 shall satisfy
that obligation. If Recovery has not been observed by D2's frozen snapshot, the
monitor and its reporting integration shall emit a mode-transition timeout error
during D2. Repeated true assertions shall not restart the obligation. Recovery first
observed at D3 or never observed shall not suppress the timeout.

The timeout error shall be logged at most once per boot. Reboot shall reset the
monitor and reporting state. Routine per-step monitor status output shall not be
used as a substitute for the required one-time timeout diagnostic.

Trace: HLR-30; approved same-dispatch reporting integration.

### CR02-DR-17 — Monitor independence from forwarding

The monitor verdict and timeout reporting shall not enable forwarding, alter blacklist
classification, change the rejection count or clear Recovery. Monitoring shall not
replace application implementations with executable GUMBOX contract logic.

Trace: HLR-19, HLR-20, HLR-22, HLR-27, HLR-28 and HLR-30; CR-02 sketch.

### CR02-DR-18 — Reason logging

RxFirewall and MAVLinkFirewall shall retain diagnostic logging that identifies when
and why an input is dropped. Reasons shall distinguish relevant network rejection,
malformed MAVLink, blacklisted firmware activation and Recovery suppression.
RxFirewall shall retain its MAVLink-routing diagnostics. Logging shall not change
routing decisions. Repeated per-message rejection logs are distinct from HLR-30's
single timeout log per boot.

Trace: CR-02 sketch; retained baseline diagnostic obligations.

## E. Integration, deployment and verification constraints

### CR02-DR-19 — VMM receive integration

The VMM shall continue receiving four direct RawEthernetMessage lanes and four
MAVLinkUDPMessage_Impl lanes. It shall inject the direct frame or the carrier's
preserved ethernet_frame into the existing virtio-net receive path. The data layouts,
lane correspondence and VMM-to-TxFirewall path shall remain unchanged.

Trace: HLR-5, HLR-13, HLR-18 and HLR-22; retained baseline integration obligations.

### CR02-DR-20 — Transmit and driver non-regression

CR-02 shall preserve TxFirewall's existing model interfaces, application behavior
and contracts and the low-level driver's functionality. Changes to firewall_core or
other shared generated dependencies shall preserve successful TxFirewall verification.
Both Normal and Recovery modes shall leave transmit policy governed by the supplied
TxFirewall HLRs.

Trace: HLR-7, HLR-12, HLR-14 and HLR-16; CR-02 sketch and approved non-impact scope.

### CR02-DR-21 — Deployment and timing evidence

The system shall retain ZCU102/seL4/Microkit deployment and its manually integrated
VMM. Make invocations shall use `SYSTEM_MAKEFILE=custom.mk`. The build shall include
the generated R2U2 specification and runtime dependencies reproducibly, recording
compiler/runtime versions and generation options.

The deployed schedule shall provide mode propagation that meets HLR-30's D2 deadline
in normal operation. Validation shall account for input freezing, output publication,
monitor evaluation, verdict delivery and logging, including monitor runtime and memory
cost. The two-dispatch deadline shall not be represented as an independently specified
wall-clock bound.

RxFirewall and MAVLinkFirewall retain the baseline configured periodic dispatch of
1000 ms and compute-execution time of 300 ms unless a separately reviewed deployment
revision changes those settings. ModeManager's actual period, budget and domain
placement shall be recorded and checked against the required propagation behavior;
this reference does not invent numerical settings for the new component.

Trace: HLR-26 and HLR-30; baseline deployment refinement and approved schedule work.

### CR02-DR-22 — Independent implementation and regression evidence

Application logic shall implement the specified behavior directly rather than invoke
GUMBOX-generated executable contracts as its implementation. Changed components
shall have application and GUMBOX tests covering all branches, with any unavoidable
gaps documented and reviewed. Verus verification shall cover affected component
implementations and shared-core obligations, followed by final cross-crate checks.

Tests shall execute the generated R2U2 monitor and reporting integration for timely
D1/D2 Recovery, missing D2, late/never-arriving Recovery, repeated assertions, empty
inputs and reboot reset. Tests shall check actual verdict and log timing, not merely
eventual failure. Hardware acceptance shall exercise normal routing, strict UDP,
blacklist rejection, transition to Recovery, persistence until reboot and transmit
non-regression.

The assurance report shall distinguish verified application properties from the
R2U2 compiler/runtime trust boundary. Runtime monitoring shall not be described as a
formal proof of the whole system's temporal behavior.

Trace: CR-02 sketch and approved verification/acceptance scope.

## F. Suggested allocation and terminology

| Owner | Supplied HLRs | Supplemental requirements |
|---|---|---|
| RxFirewall | 5, 13, 15, 17, 18, 29 | DR-01–05, DR-10–11, DR-18 |
| MAVLinkFirewall | 19, 20, 21, 22, 25, 28, 30, 31, 32 | DR-01, DR-03, DR-05–13, DR-15–18 |
| ModeManager | 23, 24, 26, 27 | DR-11, DR-14 |
| VMM/integration | Receive-path HLR interfaces | DR-19 |
| TxFirewall/driver/shared core | 7, 12, 14, 16 and sketch constraints | DR-04–05, DR-20 |
| Deployment/verification | 26, 30 and approved plan | DR-21–22 |

`DR-n` in this table abbreviates `CR02-DR-n`.

| Term | Meaning |
|---|---|
| Ethernet carrier | Existing fixed 1600-byte storage; unused bytes are not MAVLink payload |
| UDP payload slice length | Length of the complete MAVLink wire message carried by UDP, including MAVLink framing |
| MAVLink transmitted payload length | MAVLink header field describing transmitted message payload bytes, excluding its framing |
| Mode | Sampled enum Normal/Recovery; consumer uses its frozen value |
| ErrorStatus | Sampled Boolean; initial false, post-count publication each dispatch |
| RejectionCount | Lifetime saturating count 0..20, threshold five |
| D0/D1/D2 | MAVLink dispatch indices relative to the first true ErrorStatus assertion |
| COMMAND_INT/LONG command | Little-endian 16-bit field at payload bytes 28–29; v1 message bytes 34–35, v2 bytes 38–39 |
| SECURE_COMMAND operation | Little-endian 32-bit field at payload bytes 4–7; v2 message bytes 14–17 |

## G. Historical requirement migration reference

The table preserves old meanings without reassigning the developer's current HLR IDs.

| Historical CR-01 HLR | Old obligation | Suggested destination |
|---|---|---|
| 19 | Exclusive Rx routing | CR02-DR-02 |
| 20 | Rx diagnostics | CR02-DR-18 |
| 21 | MAVLink structural validity | HLR-20/22 and CR02-DR-03/06/07 |
| 22 | Allowed forwarding | Current HLR-22 with Normal guard |
| 23 | Allow file transfer | HLR-22 and CR02-DR-08 |
| 24 | Deny firmware activation | Current HLR-19/32 and CR02-DR-08 |
| 25 | Malformed rejection | Current HLR-20 |
| 26 | No input/failure handling | Current HLR-21 and CR02-DR-09 |
| 27 | MAVLink diagnostics | CR02-DR-18 |
| 28 | VMM receive integration | CR02-DR-19 |
| 29 | Period/execution configuration | CR02-DR-21 |
| 30 | Four-lane capacity | CR02-DR-01 |
| 31 | Separation of concerns | CR02-DR-05 |
| 32 | Tx verification non-regression | CR02-DR-20 |
| 33 | Implementation independence | CR02-DR-17/22 |

For the developer's later ConOps update, these additions support scenarios for normal
forwarding, fifth rejection, first dispatch observing Recovery, D2 timeout, persistent
Recovery and reboot. Preserve the original developer-concept block and annotate
CR-02 changes separately. Requirements ownership remains with the developer.
