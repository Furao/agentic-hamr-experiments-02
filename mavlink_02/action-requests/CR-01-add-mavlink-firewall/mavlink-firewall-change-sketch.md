# Reduce potential cyber attacks by adding a mavlink firewall that can blacklist certain mavlink commands/messages

## Goals for updated system functionality
Increase the cyber resilience of the system by adding a protocol specific firewall which can reject specific or malformed mavlink messages before they can make it to the Ardupilot software running in the VM. THe mavlink firewall should reject any messages that can update firmware.

## Background

### Architecture
This is a virtualized UAS system that runs on the ZCU102 and is built on seL4, Microkit, Rust, and HAMR.

There are 4 components in this system: The Low-level ethernet driver, a network RxFirewall, a network TxFirewall, and a Virtual Machine Manager (VMM) hosting a Virtual Machine (VM) running Linux and the Ardupilot software.

Data received in the Ethernet device (via the driver) is sent to the RxFirewall and if it passes the policy according to the components requirements, then it is sent on to the VMM, which injects it into the VM through virtio-net.

When Data is transmitted from the VM to the virtio-net interface, the VMM sends it to the TxFirewall and if it passes the policy according to the components requirements, then it is sent on to the Ethernet Driver component and then sent through the device.

All of the components are Rust, except for the VMM, which is C.

### Other Notes
All makefile commands are to be prepended with `SYSTEM_MAKEFILE=custom.mk `.

The VMM is manually configured outside of HAMR and lives in the `hamr/microkit/vmm` directory.

All connections between components have 4 separate corresponding input and output ports. This is a workaround to address that HAMR does not support a queue size larger than 1. 


## Suggested approach

### RxFirewall
To identify network packets bound for Ardupilot and facilitate identifying single messages, the Ardupilot VM was updated to use UDP instead of TCP. A packet is bound for Ardupilot if it is UDP, the source port is 14550, and the destination port is 14562.

The Rxfirewall should no longer allow any TCP packets through. If a UDP packet is bound for ardupilot, then it should be routed to the new Mavlink firewall component. The previously allowed packets should still route directly to the VMM (Ardupilot component).

### Mavlink Firewall
The mavlink firewall should parse the payload of the UDP packet, since it is a single, serialized mavlink message.

Use `mavlink_spec/Packet Serialization _ MAVLink Guide.html` as the specification for how Mavlink messages are serialized and should be deserialized. Handle both versions of the Mavlink protocol.

Disallow messages that are malformed.
Disallow messages that can update the ardupilot firmware.

We are using the ArduPilotMega Dialect, which means all of the following message definitions need to be considered:
- `mavlink_spec/ardupilotmega.xml`
- `mavlink_spec/common.xml`
- `mavlink_spec/standard.xml`
- `mavlink_spec/minimal.xml`

All allowed messages should be routed to the VMM.

### VMM
The VMM needs to be updated to read the network packets coming from the Mavlink firewall.

## Guidelines
When updating the RxFirewall, utilize and update the local firewall_core dependency instead of starting from scratch. Also make sure that any changes to

When writing specs or code, avoid combining many vague statements and prefer abstractions that make the code easier to read by humans.

When developing code, do not just use the generated executable code generated from GUMBO, ie. GUMBOX.

Maintain separation of concerns. The mavlink firewall should not need to understand the networking stack and the RxFirewall should not need to understand the mavlink protocol.

Retain the naming scheme of the requirements defined in `requirements/manual_reqs.md` and link requirement ID to related contracts for traceability.

Retain logging of components that describes when and why messages are dropped. Do the same for the new Mavlink firewall.