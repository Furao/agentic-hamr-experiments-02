# Add cyber resilience by detecting user attacks and changing behavior

## Goals for updated system functionality
Increase the cyber resilience of the system by counting the number of rejected mavlink messages and updating the mode of the system when a specified threshold is reached.

## Background

### Architecture
This is a virtualized UAS system that runs on the ZCU102 and is built on seL4, Microkit, Rust, and HAMR.

There are 5 components in this system: The Low-level ethernet driver, a network RxFirewall, a network TxFirewall, a Mavlink Firewall, and a Virtual Machine Manager (VMM) hosting a Virtual Machine (VM) running Linux and the Ardupilot software.

Data received in the Ethernet device (via the driver) is sent to the RxFirewall and if it passes the policy according to the components requirements, then it is sent on to either the Mavlink Firewall or directly to the VMM. The Mavlink Firewall determines whether the messages are well-formed and if they are not in a blacklist, if so it sends on to the the VMM. The VMM injects it into the VM through virtio-net.

When Data is transmitted from the VM to the virtio-net interface, the VMM sends it to the TxFirewall and if it passes the policy according to the components requirements, then it is sent on to the Ethernet Driver component and then sent through the device.

All of the components are Rust, except for the VMM, which is C.

### Other Notes
All makefile commands are to be prepended with `SYSTEM_MAKEFILE=custom.mk `.

The VMM is manually configured outside of HAMR and lives in the `hamr/microkit/vmm` directory.

All ethernet data connections between components have 4 separate corresponding input and output ports. This is a workaround to address that HAMR does not support a queue size larger than 1. 

## Suggested approach

Refer to supplied requirements.

## Guidelines
When updating the RxFirewall, utilize and update the local firewall_core dependency instead of starting from scratch. Also make sure that any changes to the firewall_core does not affect verification of the TxFirewall.

When writing specs or code, avoid combining many vague statements and prefer abstractions that make the code easier to read by humans.

When developing code, do not just use the generated executable code generated from GUMBO, ie. GUMBOX.

Maintain separation of concerns. The mavlink firewall should not need to understand the networking stack and the RxFirewall should not need to understand the mavlink protocol.

Retain the naming scheme of the requirements defined in `requirements/manual_reqs.md` and link requirement ID to related contracts for traceability.

Retain logging of components that describes when and why messages are dropped. Do the same for the new Mavlink firewall.
