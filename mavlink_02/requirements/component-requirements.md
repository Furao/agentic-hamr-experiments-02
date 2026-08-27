# CR-01 Component Requirements

This document allocates `updated_reqs.md` requirements to component contracts and
implementation responsibilities.

## RxFirewall

| Component Req ID | Traces to | Requirement |
|---|---|---|
| RxFirewall_Req_ARP | HLR-5 | Forward each valid ARP input unchanged on the direct VMM output. |
| RxFirewall_Req_NoTCP | HLR-6, HLR-15 | Emit no output for TCP input and log TCP rejection. |
| RxFirewall_Req_DirectUDP | HLR-13 | Forward retained allowed UDP directly unless it is source 14550/destination 14562. |
| RxFirewall_Req_RouteMAVLink | HLR-18 | Route qualifying ArduPilot UDP to the MAVLinkFirewall output only as a carrier containing the preserved frame and validated payload offset/length. |
| RxFirewall_Req_Exclusive | HLR-19 | Never emit both direct and MAVLink outputs for one input event. |
| RxFirewall_Req_Drop | HLR-15, HLR-20 | Drop and reason-log malformed/disallowed networking traffic. |
| RxFirewall_Req_NoInput | HLR-17 | Emit neither output when no input event is present. |
| RxFirewall_Int_MAVLinkOut | HLR-18, HLR-31 | Every MAVLink-path output preserves a well-formed qualifying UDP frame and carries in-bounds payload metadata consistent with its IPv4/UDP headers; no MAVLink semantics are evaluated. |

## MAVLinkFirewall

| Component Req ID | Traces to | Requirement |
|---|---|---|
| MAVLinkFirewall_Req_Valid | HLR-21 | Validate supplied payload bounds, then recognize exactly one complete, checksum-valid MAVLink v1/v2 frame within that slice using bundled dialect metadata, accepting MAVLink v2 trailing-zero payload truncation up to the dialect maximum. |
| MAVLinkFirewall_Req_Allow | HLR-22 | Forward a valid non-denied input carrier unchanged, preserving both the Ethernet frame and validated bounds. |
| MAVLinkFirewall_Req_AllowFTP | HLR-23 | Allow valid `FILE_TRANSFER_PROTOCOL` messages unless an independent deny rule applies. |
| MAVLinkFirewall_Req_DenyFlash | HLR-24 | Drop command envelopes carrying command 42650 or secure flash operation 7. |
| MAVLinkFirewall_Req_Malformed | HLR-25, HLR-27 | Drop malformed/unsupported/checksum-failing frames and log a specific reason. |
| MAVLinkFirewall_Req_FailClosed | HLR-26 | Emit no output for absent input or any unprocessable input. |
| MAVLinkFirewall_Req_NoNetworking | HLR-31 | Consume the bounded UDP payload description supplied by RxFirewall without deriving Ethernet/IP/UDP offsets or implementing networking policy. |
| MAVLinkFirewall_Int_Output | HLR-21–HLR-26 | Every output is a structurally valid, allowed carrier copied from the same input lane. |

## ArduPilot/VMM

| Component Req ID | Traces to | Requirement |
|---|---|---|
| ArduPilotVMM_Req_DualReceive | HLR-28 | Consume four direct raw-frame inputs and four MAVLinkFirewall carrier inputs, preserving lane identity. |
| ArduPilotVMM_Req_Virtio | HLR-28 | Inject either the direct frame or the carrier's preserved `ethernet_frame` through the existing virtio-net path. |
| ArduPilotVMM_Req_TxUnchanged | HLR-28 | Preserve all existing VMM-to-TxFirewall behavior. |

## Shared libraries and frozen components

| Component Req ID | Traces to | Requirement |
|---|---|---|
| FirewallCore_Req_Classify | HLR-18, HLR-31 | Parse Ethernet/IPv4/UDP fields to return direct/drop/MAVLink routing classification including both UDP ports and validated payload offset/length. |
| FirewallCore_Req_TxProof | HLR-32 | Preserve the API/specification properties used by TxFirewall so that its existing implementation verifies unchanged. |
| TxFirewall_Req_Frozen | HLR-7, HLR-12, HLR-14, HLR-16, HLR-32 | No model, contract, or application-code delta; successful verification against changed `firewall_core` is mandatory. |
| LowLevelDriver_Req_Frozen | CR-01 non-impact | No model, contract, or application-code delta; existing interfaces remain unchanged. |

## Traceability summary

| System requirements | Allocation |
|---|---|
| HLR-5, HLR-6, HLR-13, HLR-15, HLR-17–HLR-20 | RxFirewall, `firewall_core` |
| HLR-21–HLR-27 | MAVLinkFirewall, MAVLink policy core |
| HLR-28 | ArduPilot/VMM and system connections |
| HLR-29–HLR-30 | MAVLinkFirewall model and static schedule |
| HLR-31 | RxFirewall/`firewall_core` and MAVLinkFirewall/MAVLink core boundary |
| HLR-32 | `firewall_core`, unchanged TxFirewall verification |
| HLR-33 | RxFirewall and MAVLinkFirewall application implementations |
