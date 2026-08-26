# Architecture
This is a virtualized UAS system that runs on the ZCU102 and is built on seL4, Microkit, Rust, and HAMR.

There are 4 components in this system: The Low-level ethernet driver, a network RxFirewall, a network TxFirewall, and a Virtual Machine Manager (VMM) hosting a Virtual Machine (VM) running Linux and the Ardupilot software.

Data received in the Ethernet device (via the driver) is sent to the RxFirewall and if it passes the policy according to the components requirements, then it is sent on to the VMM, which injects it into the VM through virtio-net.

When Data is transmitted from the VM to the virtio-net interface, the VMM sends it to the TxFirewall and if it passes the policy according to the components requirements, then it is sent on to the Ethernet Driver component and then sent through the device.

All of the components are Rust, except for the VMM, which is C.

# Other Notes
All makefile commands are to be prepended with `SYSTEM_MAKEFILE=custom.mk `.

The VMM is manually configured outside of HAMR and lives in the `hamr/microkit/vmm` directory.

# RX_Firewall requirements
## Rx_firewall: Copy through any ARP frame (RC_INSPECTA_00-HLR-5)
The RX firewall shall copy a frame from an input port to its output port if that frame has a wellformed ethheader, the ethernet type is ARP, and the ARP packet is wellformed.

- An ethernet header is wellformed if the ethernet type is valid and the destination
address is valid.
  - The ethernet type is valid if bytes 12-13 of the frame are 0x0800 or 0x0806 or 0x86DD.
  - The destination address is valid if bytes 0-5 of the frame are not 0x0000000000.
- An ethernet type is ARP if bytes 12-13 of the frame are 0x0806.
- An ARP packet is wellformed if the ARP Operation is valid and the ARP hardware type
is valid and the ARP protocol type is valid.
  - The ARP Operation is valid if bytes 20-21 of the frame are 0x0001 or 0x0002.
  - The ARP hardware type is valid if bytes 14-15 of the frame are 0x0001.
  - The ARP protocol type is valid if bytes 16-17 of the frame are 0x0800 or 0x86DD.

## Rx_firewall: Copy through allowed tcp port frames (RC_INSPECTA_00-HLR-6)))
The RX firewall shall copy a frame from an input port to its output port's message if that frame has a wellformed ethheader, the ethernet type is IPv4, the IPv4 packet is wellformed, the IPv4 packet uses the TCP protocol, and the TCP port is in the TCP port whitelist.

- An ethernet header is wellformed if the ethernet type is valid and the destination address is valid.
  - The ethernet type is valid if bytes 12-13 of the frame are 0x0800 or 0x0806 or 0x86DD.
  - The destination address is valid if bytes 0-5 of the frame are not 0x0000000000.
- An IPv4 packet is wellformed if the IPv4 protocol is valid, the IHL indicates no IPv4 options in the header, and the IPv4 length is valid.
  - The IPv4 protocol is valid if byte 23 of the frame is 0x00 or 0x01 or 0x02 or 0x06 or 0x11 or 0x2B or 0x2C or 0x3A or 0x3B or 0x3C.
  - The IPv4 length is valid if bytes 16-17 of the frame are <= 9000.
  - The IPv4 version is 4 and the IHL indicates no IPv4 options in the header if byte 14 is 0x45.
- An IPv4 packet uses the TCP protocol if byte 23 of the frame is 0x06.
- The TCP port is in the whitelist if bytes 36-37 of the frame are one of the following:
  - [5760]

A maximum IPv4 length of 9000 is selected since it is the standard JUMBO frame size for Maximum Transmission Unit (MTU). In most cases it will be closer to 1500.

## Rx_firewall: Copy through allowed udp port frames (RC_INSPECTA_00-HLR-13)))
The RX firewall shall copy a frame from an input port to its output port's message if that frame has a wellformed ethheader, the ethernet type is IPv4, the IPv4 packet is wellformed, the IPv4 packet uses the UDP protocol, and the UDP port is in the UDP port whitelist.

- An ethernet header is wellformed if the ethernet type is valid and the destination address is valid.
  - The ethernet type is valid if bytes 12-13 of the frame are 0x0800 or 0x0806 or 0x86DD.
  - The destination address is valid if bytes 0-5 of the frame are not 0x0000000000.
- An IPv4 packet is wellformed if the IPv4 protocol is valid, the IHL indicates no IPv4 options in the header, and the IPv4 length is valid.
  - The IPv4 protocol is valid if byte 23 of the frame is 0x00 or 0x01 or 0x02 or 0x06 or 0x11 or 0x2B or 0x2C or 0x3A or 0x3B or 0x3C.
  - The IPv4 length is valid if bytes 16-17 of the frame are <= 9000.
  - The IPv4 version is 4 and the IHL indicates no IPv4 options in the header if byte 14 is 0x45.
- An IPv4 packet uses the UDP protocol if byte 23 of the frame is 0x11.
- The UDP port is in the whitelist if bytes 36-37 of the frame are one of the following:
  - [68]

A maximum IPv4 length of 9000 is selected since it is the standard JUMBO frame size for Maximum Transmission Unit (MTU). In most cases it will be closer to 1500.

## Rx_firewall: Do not copy disallowed frame (RC_INSPECTA_00-HLR-15)))
The RX firewall shall not copy any frame originatingfrom an input port to its output port if it does not match a valid frame as defined in the other HLRs.

## Rx_firewall: No output on empty input (RC_INSPECTA_00-HLR-17)))
The RX firewall shall not place any data in an output port when its corresponding input port does not have any data available.

# TX_Firewall

## Tx_firewall: Copy through any ARP frame (RC_INSPECTA_00-HLR-7)))
The TX firewall shall copy a frame from an input port to its output port's message if that frame has a wellformed ethheader, the ethernet type is ARP, and the ARP packet is wellformed. A size of 64 is provided in the output port's size.

- An ethernet header is wellformed if the ethernet type is valid and the destination address is valid.
  - The ethernet type is valid if bytes 12-13 of the frame are 0x0800 or 0x0806 or 0x86DD.
  - The destination address is valid if bytes 0-5 of the frame are not 0x0000000000.
- An ethernet type is ARP if bytes 12-13 of the frame are 0x0806.
- An ARP packet is wellformed if the ARP Operation is valid and the ARP hardware type is valid and the ARP protocol type is valid.
  - The ARP Operation is valid if bytes 20-21 of the frame are 0x0001 or 0x0002.
  - The ARP hardware type is valid if bytes 14-15 of the frame are 0x0001.
  - The ARP protocol type is valid if bytes 16-17 of the frame are 0x0800 or 0x86DD.

## Tx_firewall: Copy through any IPv4 frame (RC_INSPECTA_00-HLR-12)))
The TX firewall shall copy a frame from an input port to its output port's message if that frame has a wellformed ethheader, the ethernet type is IPv4, and the IPv4 packet is wellformed. The sum of the total size provided by the IPv4 header and the 14 bytes of the ethernet header is provided in the output port's size.

- An ethernet header is wellformed if the ethernet type is valid and the destination address is valid.
  - The ethernet type is valid if bytes 12-13 of the frame are 0x0800 or 0x0806 or 0x86DD.
  - The destination address is valid if bytes 0-5 of the frame are not 0x0000000000.
- An ethernet type is IPv4 if bytes 12-13 of the frame are 0x0800.
- An IPv4 packet is wellformed if the IPv4 protocol is valid, the IHL indicates no IPv4 options in the header, and the IPv4 length is valid.
  - The IPv4 protocol is valid if byte 23 of the frame is 0x00 or 0x01 or 0x02 or 0x06 or 0x11 or 0x2B or 0x2C or 0x3A or 0x3B or 0x3C.
  - The IPv4 length is valid if bytes 16-17 of the frame are <= 9000.
  - The IPv4 version is 4 and the IHL indicates no IPv4 options in the header if byte 14 is 0x45.

Note: A maximum IPv4 length of 9000 is selected since it is the standard JUMBO frame size for Maximum Transmission Unit (MTU). In most cases it will be closer to 1500.

## Tx_firewall: Do not copy disallowed frame (RC_INSPECTA_00-HLR-14)))
The TX firewall shall not copy any frame originating from an input port to its output port if it does not match a valid copy frame as defined in other HLRs.

## Tx_firewall: No output on empty input (RC_INSPECTA_00-HLR-16)))
The TX firewall shall not place any data in an output port when its corresponding input port does not have any data available.
