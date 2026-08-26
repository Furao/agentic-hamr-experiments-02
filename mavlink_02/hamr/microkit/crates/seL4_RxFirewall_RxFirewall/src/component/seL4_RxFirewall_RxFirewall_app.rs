#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This file will not be overwritten if codegen is rerun

use crate::bridge::seL4_RxFirewall_RxFirewall_api::*;
use data::*;
use firewall_core::{EthFrame, Ipv4ProtoPacket, PacketType};
#[cfg(feature = "sel4")]
// #[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use vstd::prelude::*;

verus! {
    const ARDUPILOT_SOURCE_PORT: u16 = 14550;
    const ARDUPILOT_DESTINATION_PORT: u16 = 14562;
    const DIRECT_UDP_DESTINATION_PORT: u16 = 68;
    const ETHERNET_HEADER_LENGTH: u16 = 14;
    const IPV4_HEADER_LENGTH: u16 = 20;
    const UDP_HEADER_LENGTH: u16 = 8;
    const ETHERNET_FRAME_LENGTH: u16 = 1600;

    #[derive(Debug, PartialEq, Eq)]
    enum RxRoute {
        Direct,
        MAVLink,
        Drop,
    }

    #[verifier::external_body]
    fn classify_frame(frame: &open_platform_Data_Model::RawEthernetMessage) -> (result: (RxRoute, u16))
        ensures
            (result.0 is Direct) == GumboLib::rx_direct_frame_spec(*frame),
            (result.0 is MAVLink) == GumboLib::valid_ardupilot_udp_spec(*frame),
            (result.0 is Drop) == !GumboLib::rx_allow_outbound_frame_spec(*frame),
            (result.0 is MAVLink) ==> result.1 == GumboLib::udp_payload_length_spec(*frame),
    {
        match EthFrame::parse(frame) {
            Some(parsed) => match parsed.eth_type {
                PacketType::Arp(_) => (RxRoute::Direct, 0),
                PacketType::Ipv4(ipv4) => match ipv4.protocol {
                    Ipv4ProtoPacket::Udp(udp) => {
                        let mavlink = udp.src_port == ARDUPILOT_SOURCE_PORT
                            && udp.dst_port == ARDUPILOT_DESTINATION_PORT
                            && UDP_HEADER_LENGTH <= udp.length
                            && ipv4.header.length + ETHERNET_HEADER_LENGTH <= ETHERNET_FRAME_LENGTH
                            && IPV4_HEADER_LENGTH <= ipv4.header.length
                            && udp.length == ipv4.header.length - IPV4_HEADER_LENGTH;
                        if mavlink {
                            (RxRoute::MAVLink, udp.length - UDP_HEADER_LENGTH)
                        } else if udp.dst_port == DIRECT_UDP_DESTINATION_PORT {
                            (RxRoute::Direct, 0)
                        } else {
                            (RxRoute::Drop, 0)
                        }
                    }
                    _ => (RxRoute::Drop, 0),
                },
                PacketType::Ipv6 => (RxRoute::Drop, 0),
            },
            None => (RxRoute::Drop, 0),
        }
    }

    #[verifier::external_body]
    fn make_mavlink_carrier(frame: open_platform_Data_Model::RawEthernetMessage, payload_length: u16) -> (carrier: open_platform_Data_Model::MAVLinkUDPMessage_Impl)
        requires
            GumboLib::valid_ardupilot_udp_spec(frame),
            payload_length == GumboLib::udp_payload_length_spec(frame),
        ensures
            carrier.ethernet_frame == frame,
            carrier.payload_offset == GumboLib::udp_payload_offset_spec(),
            carrier.payload_length == GumboLib::udp_payload_length_spec(frame),
            GumboLib::valid_mavlink_carrier_spec(carrier),
    {
        open_platform_Data_Model::MAVLinkUDPMessage_Impl {
            ethernet_frame: frame,
            payload_offset: ETHERNET_HEADER_LENGTH + IPV4_HEADER_LENGTH + UDP_HEADER_LENGTH,
            payload_length,
        }
    }

    #[verifier::external_body]
    fn info(s: &str) {
        #[cfg(feature = "sel4")]
        info!("{s}");
    }

    #[verifier::external_body]
    fn trace(s: &str) {
        #[cfg(feature = "sel4")]
        trace!("{s}");
    }

    #[verifier::external_body]
    fn warn_channel(channel: microkit_channel) {
        #[cfg(feature = "sel4")]
        warn!("Unexpected channel {}", channel)
    }

    pub struct seL4_RxFirewall_RxFirewall {}

impl seL4_RxFirewall_RxFirewall {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn initialize<API: seL4_RxFirewall_RxFirewall_Put_Api>(
      &mut self,
      api: &mut seL4_RxFirewall_RxFirewall_Application_Api<API>)
    {
      info("initialize entrypoint invoked");
    }

    pub fn timeTriggered<API: seL4_RxFirewall_RxFirewall_Full_Api> (
      &mut self,
      api: &mut seL4_RxFirewall_RxFirewall_Application_Api<API>)
      requires
        // BEGIN MARKER TIME TRIGGERED REQUIRES
        // assume AADL_Requirement
        //   All outgoing event ports must be empty
        old(api).EthernetFramesRxOut0.is_none(),
        old(api).EthernetFramesRxOut1.is_none(),
        old(api).EthernetFramesRxOut2.is_none(),
        old(api).EthernetFramesRxOut3.is_none(),
        old(api).MAVLinkFramesRxOut0.is_none(),
        old(api).MAVLinkFramesRxOut1.is_none(),
        old(api).MAVLinkFramesRxOut2.is_none(),
        old(api).MAVLinkFramesRxOut3.is_none(),
        // END MARKER TIME TRIGGERED REQUIRES
      ensures
        // BEGIN MARKER TIME TRIGGERED ENSURES
        // guarantee hlr_05_13_rx0_direct
        api.EthernetFramesRxIn0.is_some() && GumboLib::rx_direct_frame_spec(api.EthernetFramesRxIn0.unwrap()) ==>
          api.EthernetFramesRxOut0.is_some() &&
            (api.EthernetFramesRxOut0.unwrap() == api.EthernetFramesRxIn0.unwrap()) &&
            api.MAVLinkFramesRxOut0.is_none(),
        // guarantee hlr_18_rx0_mavlink
        api.EthernetFramesRxIn0.is_some() && GumboLib::valid_ardupilot_udp_spec(api.EthernetFramesRxIn0.unwrap()) ==>
          api.EthernetFramesRxOut0.is_none() && api.MAVLinkFramesRxOut0.is_some() &&
            (api.MAVLinkFramesRxOut0.unwrap().ethernet_frame == api.EthernetFramesRxIn0.unwrap()) &&
            (api.MAVLinkFramesRxOut0.unwrap().payload_offset == GumboLib::udp_payload_offset_spec()) &&
            (api.MAVLinkFramesRxOut0.unwrap().payload_length == GumboLib::udp_payload_length_spec(api.EthernetFramesRxIn0.unwrap())),
        // guarantee hlr_06_15_rx0_drop
        api.EthernetFramesRxIn0.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(api.EthernetFramesRxIn0.unwrap())) ==>
          api.EthernetFramesRxOut0.is_none() && api.MAVLinkFramesRxOut0.is_none(),
        // guarantee hlr_17_rx0_no_input
        !(api.EthernetFramesRxIn0.is_some()) ==>
          api.EthernetFramesRxOut0.is_none() && api.MAVLinkFramesRxOut0.is_none(),
        // guarantee hlr_05_13_rx1_direct
        api.EthernetFramesRxIn1.is_some() && GumboLib::rx_direct_frame_spec(api.EthernetFramesRxIn1.unwrap()) ==>
          api.EthernetFramesRxOut1.is_some() &&
            (api.EthernetFramesRxOut1.unwrap() == api.EthernetFramesRxIn1.unwrap()) &&
            api.MAVLinkFramesRxOut1.is_none(),
        // guarantee hlr_18_rx1_mavlink
        api.EthernetFramesRxIn1.is_some() && GumboLib::valid_ardupilot_udp_spec(api.EthernetFramesRxIn1.unwrap()) ==>
          api.EthernetFramesRxOut1.is_none() && api.MAVLinkFramesRxOut1.is_some() &&
            (api.MAVLinkFramesRxOut1.unwrap().ethernet_frame == api.EthernetFramesRxIn1.unwrap()) &&
            (api.MAVLinkFramesRxOut1.unwrap().payload_offset == GumboLib::udp_payload_offset_spec()) &&
            (api.MAVLinkFramesRxOut1.unwrap().payload_length == GumboLib::udp_payload_length_spec(api.EthernetFramesRxIn1.unwrap())),
        // guarantee hlr_06_15_rx1_drop
        api.EthernetFramesRxIn1.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(api.EthernetFramesRxIn1.unwrap())) ==>
          api.EthernetFramesRxOut1.is_none() && api.MAVLinkFramesRxOut1.is_none(),
        // guarantee hlr_17_rx1_no_input
        !(api.EthernetFramesRxIn1.is_some()) ==>
          api.EthernetFramesRxOut1.is_none() && api.MAVLinkFramesRxOut1.is_none(),
        // guarantee hlr_05_13_rx2_direct
        api.EthernetFramesRxIn2.is_some() && GumboLib::rx_direct_frame_spec(api.EthernetFramesRxIn2.unwrap()) ==>
          api.EthernetFramesRxOut2.is_some() &&
            (api.EthernetFramesRxOut2.unwrap() == api.EthernetFramesRxIn2.unwrap()) &&
            api.MAVLinkFramesRxOut2.is_none(),
        // guarantee hlr_18_rx2_mavlink
        api.EthernetFramesRxIn2.is_some() && GumboLib::valid_ardupilot_udp_spec(api.EthernetFramesRxIn2.unwrap()) ==>
          api.EthernetFramesRxOut2.is_none() && api.MAVLinkFramesRxOut2.is_some() &&
            (api.MAVLinkFramesRxOut2.unwrap().ethernet_frame == api.EthernetFramesRxIn2.unwrap()) &&
            (api.MAVLinkFramesRxOut2.unwrap().payload_offset == GumboLib::udp_payload_offset_spec()) &&
            (api.MAVLinkFramesRxOut2.unwrap().payload_length == GumboLib::udp_payload_length_spec(api.EthernetFramesRxIn2.unwrap())),
        // guarantee hlr_06_15_rx2_drop
        api.EthernetFramesRxIn2.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(api.EthernetFramesRxIn2.unwrap())) ==>
          api.EthernetFramesRxOut2.is_none() && api.MAVLinkFramesRxOut2.is_none(),
        // guarantee hlr_17_rx2_no_input
        !(api.EthernetFramesRxIn2.is_some()) ==>
          api.EthernetFramesRxOut2.is_none() && api.MAVLinkFramesRxOut2.is_none(),
        // guarantee hlr_05_13_rx3_direct
        api.EthernetFramesRxIn3.is_some() && GumboLib::rx_direct_frame_spec(api.EthernetFramesRxIn3.unwrap()) ==>
          api.EthernetFramesRxOut3.is_some() &&
            (api.EthernetFramesRxOut3.unwrap() == api.EthernetFramesRxIn3.unwrap()) &&
            api.MAVLinkFramesRxOut3.is_none(),
        // guarantee hlr_18_rx3_mavlink
        api.EthernetFramesRxIn3.is_some() && GumboLib::valid_ardupilot_udp_spec(api.EthernetFramesRxIn3.unwrap()) ==>
          api.EthernetFramesRxOut3.is_none() && api.MAVLinkFramesRxOut3.is_some() &&
            (api.MAVLinkFramesRxOut3.unwrap().ethernet_frame == api.EthernetFramesRxIn3.unwrap()) &&
            (api.MAVLinkFramesRxOut3.unwrap().payload_offset == GumboLib::udp_payload_offset_spec()) &&
            (api.MAVLinkFramesRxOut3.unwrap().payload_length == GumboLib::udp_payload_length_spec(api.EthernetFramesRxIn3.unwrap())),
        // guarantee hlr_06_15_rx3_drop
        api.EthernetFramesRxIn3.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(api.EthernetFramesRxIn3.unwrap())) ==>
          api.EthernetFramesRxOut3.is_none() && api.MAVLinkFramesRxOut3.is_none(),
        // guarantee hlr_17_rx3_no_input
        !(api.EthernetFramesRxIn3.is_some()) ==>
          api.EthernetFramesRxOut3.is_none() && api.MAVLinkFramesRxOut3.is_none(),
        // END MARKER TIME TRIGGERED ENSURES
    {
        trace("compute entrypoint invoked");

        // Rx0 ports
        if let Some(frame) = api.get_EthernetFramesRxIn0() {
            match classify_frame(&frame) {
                (RxRoute::Direct, _) => api.put_EthernetFramesRxOut0(frame),
                (RxRoute::MAVLink, payload_length) => api.put_MAVLinkFramesRxOut0(make_mavlink_carrier(frame, payload_length)),
                (RxRoute::Drop, _) => info("Rx lane 0 packet rejected"),
            }
        }

        // Rx1 ports
        if let Some(frame) = api.get_EthernetFramesRxIn1() {
            match classify_frame(&frame) {
                (RxRoute::Direct, _) => api.put_EthernetFramesRxOut1(frame),
                (RxRoute::MAVLink, payload_length) => api.put_MAVLinkFramesRxOut1(make_mavlink_carrier(frame, payload_length)),
                (RxRoute::Drop, _) => info("Rx lane 1 packet rejected"),
            }
        }

        // Rx2 ports
        if let Some(frame) = api.get_EthernetFramesRxIn2() {
            match classify_frame(&frame) {
                (RxRoute::Direct, _) => api.put_EthernetFramesRxOut2(frame),
                (RxRoute::MAVLink, payload_length) => api.put_MAVLinkFramesRxOut2(make_mavlink_carrier(frame, payload_length)),
                (RxRoute::Drop, _) => info("Rx lane 2 packet rejected"),
            }
        }

        // Rx3 ports
        if let Some(frame) = api.get_EthernetFramesRxIn3() {
            match classify_frame(&frame) {
                (RxRoute::Direct, _) => api.put_EthernetFramesRxOut3(frame),
                (RxRoute::MAVLink, payload_length) => api.put_MAVLinkFramesRxOut3(make_mavlink_carrier(frame, payload_length)),
                (RxRoute::Drop, _) => info("Rx lane 3 packet rejected"),
            }
        }

    }

    pub fn notify(
      &mut self,
      channel: microkit_channel)
    {
      // this method is called when the monitor does not handle the passed in channel
      match channel {
        _ => {
            warn_channel(channel);
        }
      }
    }

    pub open spec fn ipv4_udp_on_allowed_port_quant(port: u16) -> bool
    {
        exists|i:int| 0 <= i && i <= GumboLib::UDP_ALLOWED_PORTS_spec().len() - 1 && GumboLib::UDP_ALLOWED_PORTS_spec()[i] == port
    }

    pub open spec fn ipv4_tcp_on_allowed_port_quant(port: u16) -> bool
    {
        exists|i:int| 0 <= i && i <= GumboLib::TCP_ALLOWED_PORTS_spec().len() - 1 && GumboLib::TCP_ALLOWED_PORTS_spec()[i] == port
    }

    // BEGIN MARKER GUMBO METHODS
    // pub open spec fn TCP_ALLOWED_PORTS() -> open_platform_Data_Model::u16Array
    // {
    //   [5760u16]
    // }

    // pub open spec fn UDP_ALLOWED_PORTS() -> open_platform_Data_Model::u16Array
    // {
    //   [68u16]
    // }
    // END MARKER GUMBO METHODS
  }

}

#[cfg(any())]
#[test]
fn tcp_port_allowed_test() {
    assert!(tcp_port_allowed(5760));
    assert!(!tcp_port_allowed(42));
}

#[cfg(any())]
#[test]
fn udp_port_allowed_test() {
    assert!(udp_port_allowed(68));
    assert!(!udp_port_allowed(19));
}

#[cfg(any())]
mod parse_frame_tests {
    use super::*;

    #[test]
    fn parse_malformed_packet() {
        let mut frame = [0u8; 1600];
        let pkt = [
            0xffu8, 0xff, 0xff, 0xff, 0xff, 0xff, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x02, 0xC2,
        ];
        frame[0..14].copy_from_slice(&pkt);
        let res = seL4_RxFirewall_RxFirewall::get_frame_packet(&frame);
        assert!(res.is_none());
    }

    #[test]
    fn parse_valid_arp() {
        let mut frame = [0u8; 1600];
        let pkt = [
            0xffu8, 0xff, 0xff, 0xff, 0xff, 0xff, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x08, 0x06, 0x0,
            0x1, 0x8, 0x0, 0x6, 0x4, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0xc0, 0xa8, 0x0, 0x1,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xc0, 0xa8, 0x0, 0xce,
        ];
        frame[0..42].copy_from_slice(&pkt);
        let res = seL4_RxFirewall_RxFirewall::get_frame_packet(&frame);
        assert!(res.is_some());
    }
}

#[cfg(any())]
mod can_send_tests {
    use super::*;
    use firewall_core::{
        Address, Arp, ArpOp, EtherType, HardwareType, IpProtocol, Ipv4Address, Ipv4Packet, Ipv4Repr,
    };

    #[test]
    fn packet_valid_arp_request() {
        let packet = PacketType::Arp(Arp {
            htype: HardwareType::Ethernet,
            ptype: EtherType::Ipv4,
            hsize: 0x6,
            psize: 0x4,
            op: ArpOp::Request,
            src_addr: Address([0x2, 0x3, 0x4, 0x5, 0x6, 0x7]),
            src_protocol_addr: Ipv4Address([0xc0, 0xa8, 0x00, 0x01]),
            dest_addr: Address([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            dest_protocol_addr: Ipv4Address([0xc0, 0xa8, 0x0, 0xce]),
        });
        assert!(can_send_packet(&packet));
    }

    #[test]
    fn packet_valid_arp_reply() {
        let packet = PacketType::Arp(Arp {
            htype: HardwareType::Ethernet,
            ptype: EtherType::Ipv4,
            hsize: 0x6,
            psize: 0x4,
            op: ArpOp::Reply,
            src_addr: Address([0x18, 0x20, 0x22, 0x24, 0x26, 0x28]),
            src_protocol_addr: Ipv4Address([0xc0, 0xa8, 0x00, 0xce]),
            dest_addr: Address([0x2, 0x3, 0x4, 0x5, 0x6, 0x7]),
            dest_protocol_addr: Ipv4Address([0xc0, 0xa8, 0x0, 0x01]),
        });
        assert!(can_send_packet(&packet));
    }

    #[test]
    fn packet_invalid_ipv6() {
        let packet = PacketType::Ipv6;
        assert!(!can_send_packet(&packet));
    }

    #[test]
    fn invalid_ipv4_protocols() {
        // Hop by Hop
        let mut packet = PacketType::Ipv4(Ipv4Packet {
            header: Ipv4Repr {
                protocol: IpProtocol::HopByHop,
                length: 0x29,
            },
            protocol: Ipv4ProtoPacket::HopByHop,
        });
        assert!(!can_send_packet(&packet));

        // ICMP
        if let PacketType::Ipv4(ip) = &mut packet {
            ip.header.protocol = IpProtocol::Icmp;
        }
        assert!(!can_send_packet(&packet));

        // IGMP
        if let PacketType::Ipv4(ip) = &mut packet {
            ip.header.protocol = IpProtocol::Igmp;
        }
        assert!(!can_send_packet(&packet));

        // Ipv6 Route
        if let PacketType::Ipv4(ip) = &mut packet {
            ip.header.protocol = IpProtocol::Ipv6Route;
        }
        assert!(!can_send_packet(&packet));

        // Ipv6 Frag
        if let PacketType::Ipv4(ip) = &mut packet {
            ip.header.protocol = IpProtocol::Ipv6Frag;
        }
        assert!(!can_send_packet(&packet));

        // ICMPv6
        if let PacketType::Ipv4(ip) = &mut packet {
            ip.header.protocol = IpProtocol::Icmpv6;
        }
        assert!(!can_send_packet(&packet));

        // IPv6 No Nxt
        if let PacketType::Ipv4(ip) = &mut packet {
            ip.header.protocol = IpProtocol::Ipv6NoNxt;
        }
        assert!(!can_send_packet(&packet));

        // IPv6 Opts
        if let PacketType::Ipv4(ip) = &mut packet {
            ip.header.protocol = IpProtocol::Ipv6Opts;
        }
        assert!(!can_send_packet(&packet));
    }

    #[test]
    fn disallowed_tcp() {
        let packet = PacketType::Ipv4(Ipv4Packet {
            header: Ipv4Repr {
                protocol: IpProtocol::Tcp,
                length: 0x29,
            },
            protocol: Ipv4ProtoPacket::Tcp(TcpRepr { dst_port: 443 }),
        });
        assert!(!can_send_packet(&packet));
    }

    #[test]
    fn allowed_tcp() {
        let packet = PacketType::Ipv4(Ipv4Packet {
            header: Ipv4Repr {
                protocol: IpProtocol::Tcp,
                length: 0x29,
            },
            protocol: Ipv4ProtoPacket::Tcp(TcpRepr { dst_port: 5760 }),
        });
        assert!(can_send_packet(&packet));
    }

    #[test]
    fn disallowed_udp() {
        let packet = PacketType::Ipv4(Ipv4Packet {
            header: Ipv4Repr {
                protocol: IpProtocol::Udp,
                length: 0x29,
            },
            protocol: Ipv4ProtoPacket::Udp(UdpRepr { dst_port: 15 }),
        });
        assert!(!can_send_packet(&packet));
    }

    #[test]
    fn allowed_udp() {
        let packet = PacketType::Ipv4(Ipv4Packet {
            header: Ipv4Repr {
                protocol: IpProtocol::Udp,
                length: 0x29,
            },
            protocol: Ipv4ProtoPacket::Udp(UdpRepr { dst_port: 68 }),
        });
        assert!(can_send_packet(&packet));
    }
}

// #[cfg(test)]
// mod rx_ethernet_frames_tests {
//     use super::*;

//     mod get_in {
//         use super::*;

//         #[test]
//         fn valid() {
//             let mut rx_buf: open_platform_Data_Model::RawEthernetMessage = [0; open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0];

//             let res = eth_get(0, &mut rx_buf);
//             assert!(res);
//             let res = eth_get(1, &mut rx_buf);
//             assert!(res);
//             let res = eth_get(2, &mut rx_buf);
//             assert!(res);
//             let res = eth_get(3, &mut rx_buf);
//             assert!(res);
//             let res = eth_get(0, &mut rx_buf);
//             assert!(res);
//             let res = eth_get(1, &mut rx_buf);
//             assert!(res);
//         }

//         #[test]
//         #[should_panic]
//         fn out_of_bounds() {
//             let mut rx_buf: open_platform_Data_Model::RawEthernetMessage = [0; open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0];

//             let _ = eth_get(4, &mut rx_buf);
//         }
//     }

//     mod put_out {
//         use super::*;

//         #[test]
//         fn valid() {
//             let mut state = State::new();
//             let mut rx_buf: open_platform_Data_Model::RawEthernetMessage = [0; open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0];

//             assert_eq!(state.idx, 0);
//             eth_put(&mut state, &mut rx_buf);
//             assert_eq!(state.idx, 1);
//             eth_put(&mut state, &mut rx_buf);
//             assert_eq!(state.idx, 2);
//             eth_put(&mut state, &mut rx_buf);
//             assert_eq!(state.idx, 3);
//             eth_put(&mut state, &mut rx_buf);
//             assert_eq!(state.idx, 0);
//             eth_put(&mut state, &mut rx_buf);
//             assert_eq!(state.idx, 1);
//         }

//         #[test]
//         #[should_panic]
//         fn out_of_bounds() {
//             let mut state = State::new();
//             state.idx = 4;
//             let mut rx_buf: open_platform_Data_Model::RawEthernetMessage = [0; open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0];

//             eth_put(&mut state, &mut rx_buf);
//         }
//     }
// }

// #[cfg(test)]
// mod bindings {
//     use super::*;
//     pub fn get_EthernetFramesRxIn0(_value: *mut open_platform_Data_Model::RawEthernetMessage) -> bool {
//         true
//     }
//     pub fn get_EthernetFramesRxIn1(_value: *mut open_platform_Data_Model::RawEthernetMessage) -> bool {
//         true
//     }

//     pub fn get_EthernetFramesRxIn2(_value: *mut open_platform_Data_Model::RawEthernetMessage) -> bool {
//         true
//     }

//     pub fn get_EthernetFramesRxIn3(_value: *mut open_platform_Data_Model::RawEthernetMessage) -> bool {
//         true
//     }
//     pub fn EthernetFramesRxIn0_is_empty() -> bool {
//         false
//     }
//     pub fn EthernetFramesRxIn1_is_empty() -> bool {
//         false
//     }
//     pub fn EthernetFramesRxIn2_is_empty() -> bool {
//         false
//     }
//     pub fn EthernetFramesRxIn3_is_empty() -> bool {
//         false
//     }
//     pub fn put_EthernetFramesRxOut0(_value: *mut open_platform_Data_Model::RawEthernetMessage) -> bool {
//         true
//     }
//     pub fn put_EthernetFramesRxOut1(_value: *mut open_platform_Data_Model::RawEthernetMessage) -> bool {
//         true
//     }
//     pub fn put_EthernetFramesRxOut2(_value: *mut open_platform_Data_Model::RawEthernetMessage) -> bool {
//         true
//     }
//     pub fn put_EthernetFramesRxOut3(_value: *mut open_platform_Data_Model::RawEthernetMessage) -> bool {
//         true
//     }
//     pub struct EthChannelGet {
//         pub get: fn(*mut open_platform_Data_Model::RawEthernetMessage) -> bool,
//         pub empty: fn() -> bool,
//     }
//     pub struct EthChannelPut {
//         pub put: fn(*mut open_platform_Data_Model::RawEthernetMessage) -> bool,
//     }
// }
