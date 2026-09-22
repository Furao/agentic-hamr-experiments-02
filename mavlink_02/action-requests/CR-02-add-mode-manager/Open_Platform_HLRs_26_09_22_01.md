High Level Requirements  
C\_ROCKWELL\_COLLINS-INSPECTA

22 September 2026

Table of Contents
=================

[1.0 RX\_Firewall 3](#rx_firewall)

[1.1 Rx\_firewall: Copy through any ARP frame to VMM output
(*RC\_INSPECTA\_00-HLR-5*)))
3](#rx_firewall-copy-through-any-arp-frame-to-vmm-output-rc_inspecta_00-hlr-5)

[1.2 Rx\_firewall: Copy through allowed UDP port frames to VMM output
(*RC\_INSPECTA\_00-HLR-13*)))
3](#rx_firewall-copy-through-allowed-udp-port-frames-to-vmm-output-rc_inspecta_00-hlr-13)

[1.3 Rx\_firewall: Copy through mavlink UDP port frames to the
mavlink\_firewall output (*RC\_INSPECTA\_00-HLR-18*)))
4](#rx_firewall-copy-through-mavlink-udp-port-frames-to-the-mavlink_firewall-output-rc_inspecta_00-hlr-18)

[1.4 Rx\_firewall: Do not copy disallowed frame
(*RC\_INSPECTA\_00-HLR-15*)))
5](#rx_firewall-do-not-copy-disallowed-frame-rc_inspecta_00-hlr-15)

[1.5 Rx\_firewall: No output during Recovery Mode
(*RC\_INSPECTA\_00-HLR-29*)))
5](#rx_firewall-no-output-during-recovery-mode-rc_inspecta_00-hlr-29)

[1.6 Rx\_firewall: No output on empty input
(*RC\_INSPECTA\_00-HLR-17*)))
5](#rx_firewall-no-output-on-empty-input-rc_inspecta_00-hlr-17)

[2.0 TX\_Firewall 5](#tx_firewall)

[2.1 Tx\_firewall: Copy through any ARP frame
(*RC\_INSPECTA\_00-HLR-7*)))
5](#tx_firewall-copy-through-any-arp-frame-rc_inspecta_00-hlr-7)

[2.2 Tx\_firewall: Copy through any IPv4 frame
(*RC\_INSPECTA\_00-HLR-12*)))
5](#tx_firewall-copy-through-any-ipv4-frame-rc_inspecta_00-hlr-12)

[2.3 Tx\_firewall: Do not copy disallowed frame
(*RC\_INSPECTA\_00-HLR-14*)))
6](#tx_firewall-do-not-copy-disallowed-frame-rc_inspecta_00-hlr-14)

[2.4 Tx\_firewall: No output on empty input
(*RC\_INSPECTA\_00-HLR-16*)))
6](#tx_firewall-no-output-on-empty-input-rc_inspecta_00-hlr-16)

[3.0 Mavlink\_Firewall 6](#mavlink_firewall)

[3.1 Mavlink\_Firewall: Drop flash\_bootloader mav\_command message
(*RC\_INSPECTA\_00-HLR-19*)))
6](#mavlink_firewall-drop-flash_bootloader-mav_command-message-rc_inspecta_00-hlr-19)

[3.2 Mavlink\_Firewall: Drop malformed mavlink messages
(*RC\_INSPECTA\_00-HLR-20*)))
7](#mavlink_firewall-drop-malformed-mavlink-messages-rc_inspecta_00-hlr-20)

[3.3 Mavlink\_firewall: No output on empty input
(*RC\_INSPECTA\_00-HLR-21*)))
7](#mavlink_firewall-no-output-on-empty-input-rc_inspecta_00-hlr-21)

[3.4 Mavlink\_firewall: No output during Recovery Mode
(*RC\_INSPECTA\_00-HLR-28*)))
7](#mavlink_firewall-no-output-during-recovery-mode-rc_inspecta_00-hlr-28)

[3.5 Mavlink\_Firewall: Copy through well-formed, not-blacklisted
messages (*RC\_INSPECTA\_00-HLR-22*)))
8](#mavlink_firewall-copy-through-well-formed-not-blacklisted-messages-rc_inspecta_00-hlr-22)

[3.6 Mavlink\_Firewall: Send Error status to Mode\_Manager
(*RC\_INSPECTA\_00-HLR-25*)))
8](#mavlink_firewall-send-error-status-to-mode_manager-rc_inspecta_00-hlr-25)

[3.7 Mavlink\_Firewall: Monitor Mode Change
(*RC\_INSPECTA\_00-HLR-30*)))
8](#mavlink_firewall-monitor-mode-change-rc_inspecta_00-hlr-30)

[3.8 Mavlink\_Firewall: Track number of rejected messages
(*RC\_INSPECTA\_00-HLR-31*)))
8](#mavlink_firewall-track-number-of-rejected-messages-rc_inspecta_00-hlr-31)

[4.0 Mode\_Manager 8](#mode_manager)

[4.1 Mode\_Manager: Initialization (*RC\_INSPECTA\_00-HLR-23*)))
8](#mode_manager-initialization-rc_inspecta_00-hlr-23)

[4.2 Mode\_Manager: Modes (*RC\_INSPECTA\_00-HLR-24*)))
8](#mode_manager-modes-rc_inspecta_00-hlr-24)

[4.3 Mode\_Manager: Send current mode (*RC\_INSPECTA\_00-HLR-26*)))
8](#mode_manager-send-current-mode-rc_inspecta_00-hlr-26)

[4.4 Mode\_Manager: Transition to Recovery Mode
(*RC\_INSPECTA\_00-HLR-27*)))
8](#mode_manager-transition-to-recovery-mode-rc_inspecta_00-hlr-27)

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

Rx\_firewall: No output during Recovery Mode ([*RC\_INSPECTA\_00-HLR-29*])
---------------------------------------------------------------------------------------------------------------------------------------------------

During Recovery Mode, the RX firewall shall not place any data on any
output ports.

Rx\_firewall: No output on empty input ([*RC\_INSPECTA\_00-HLR-17*])
---------------------------------------------------------------------------------------------------------------------------------------------

The RX firewall shall not place any data in any output port when its
corresponding input port does not have any data available.

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

 

he mavlink protocol is v2 if byte 0 is 0xFD
 - The mavlink message, bytes 7-9, has the ID of CommandInt: 75

    - The message payload's mav command, bytes 37-38, has
_CMD\_FLASH\_BOOTLOADER: 42650

 

OR

 

\- The mavlink protocol is v2 if byte 0 is 0xFD

    - The mavlink message, bytes 7-9, has the ID of CommandLong: 76

        - The message payload's mav command, bytes 38-39, has
MAV\_CMD\_FLASH\_BOOTLOADER: 42650

 

OR

 

\- The mavlink protocol is v1 if byte 0 is 0xFE

    - The mavlink message, byte 5, has the ID of CommandInt: 75

        - The message payload's mav command, bytes 33-34, has
MAV\_CMD\_FLASH\_BOOTLOADER: 42650

 

OR

 

\- The mavlink protocol is v1 if byte 0 is 0xFE

    - The mavlink message, byte 5, has the ID of CommandLong: 76

        - The message payload's mav command, bytes 34-35, has
MAV\_CMD\_FLASH\_BOOTLOADER: 42650

 

Mavlink\_Firewall: Drop malformed mavlink messages ([*RC\_INSPECTA\_00-HLR-20*])
---------------------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink firewall shall drop an input frame if the mavlink message of
the payload is malformed.

Mavlink\_firewall: No output on empty input ([*RC\_INSPECTA\_00-HLR-21*])
--------------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink firewall shall not place any data in an output port when its
corresponding input port does not have any data available.

Mavlink\_firewall: No output during Recovery Mode ([*RC\_INSPECTA\_00-HLR-28*])
--------------------------------------------------------------------------------------------------------------------------------------------------------

During Recovery Mode, the Mavlink firewall shall not place any data in
any output port.

Mavlink\_Firewall: Copy through well-formed, not-blacklisted messages ([*RC\_INSPECTA\_00-HLR-22*])
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink firewall shall copy through a mavlink message from an input
port to its corresponding output port if it is well-formed, it is not in
the blacklist, and the Mode is Normal Mode. 

Mavlink\_Firewall: Send Error status to Mode\_Manager ([*RC\_INSPECTA\_00-HLR-25*])
------------------------------------------------------------------------------------------------------------------------------------------------------------

If more than 5 messages have been rejected, then the Mavlink Firewall
shall set the Mode Manager error boolean to true, otherwise set it to
false.

Mavlink\_Firewall: Monitor Mode Change ([*RC\_INSPECTA\_00-HLR-30*])
---------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink Firewall shall monitor the Mode change and shall print an
error if it takes more than 2 dispatches from setting the Mode Manager
error boolean to its Mode input being changed. 

Mavlink\_Firewall: Track number of rejected messages ([*RC\_INSPECTA\_00-HLR-31*])
-----------------------------------------------------------------------------------------------------------------------------------------------------------

The Mavlink Firewall shall track the number of messages that have been
rejected since boot time.

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

The Mode Manager should always send the current mode to the RX\_Firewall
and to the Mavlink\_Firewall. 

Mode\_Manager: Transition to Recovery Mode ([*RC\_INSPECTA\_00-HLR-27*])
-------------------------------------------------------------------------------------------------------------------------------------------------

If the Mavlink Filter supplies true for the Error Status boolean, then
the Mode Manager shall set the Mode to Recovery Mode.
