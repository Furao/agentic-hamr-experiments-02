#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This file will not be overwritten if codegen is rerun

use crate::bridge::seL4_RxFirewall_RxFirewall_api::*;
use data::*;
#[cfg(feature = "sel4")]
// #[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use vstd::prelude::*;

use crate::open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0;
use firewall_core::{EthFrame, IpProtocol, Ipv4ProtoPacket, PacketType, TcpRepr, UdpRepr};

verus! {
    mod config {
        pub mod tcp {
            pub const ALLOWED_PORTS: [u16; 1] = [5760u16];
        }

        pub mod udp {
            const NUM_UDP_PORTS: usize = 1;
            pub const ALLOWED_PORTS: [u16; NUM_UDP_PORTS] = [68u16];
        }

    }

    const NUM_MSGS: usize = 4;

    #[verifier::external_body]
    fn info(s: &str) {
        #[cfg(feature = "sel4")]
        info!("{s}");
    }

    #[verifier::external_body]
    fn info_protocol(protocol: IpProtocol) {
        #[cfg(feature = "sel4")]
        info!("Not a TCP or UDP packet. ({:?}) Throw it away.", protocol);
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

    // fn eth_get<API: seL4_RxFirewall_RxFirewall_Get_Api>(
    //     idx: usize,
    //     api: &mut seL4_RxFirewall_RxFirewall_Application_Api<API>,
    // ) -> Option<open_platform_Data_Model::RawEthernetMessage> {
    //     match idx {
    //         0 => api.get_EthernetFramesRxIn0(),
    //         1 => api.get_EthernetFramesRxIn1(),
    //         2 => api.get_EthernetFramesRxIn2(),
    //         3 => api.get_EthernetFramesRxIn3(),
    //         _ => None,
    //     }
    // }

    // fn eth_put<API: seL4_RxFirewall_RxFirewall_Put_Api>(
    //     idx: usize,
    //     rx_buf: &mut open_platform_Data_Model::RawEthernetMessage,
    //     api: &mut seL4_RxFirewall_RxFirewall_Application_Api<API>,
    // ) {
    //     match idx {
    //         0 => api.put_EthernetFramesRxOut0(*rx_buf),
    //         1 => api.put_EthernetFramesRxOut1(*rx_buf),
    //         2 => api.put_EthernetFramesRxOut2(*rx_buf),
    //         3 => api.put_EthernetFramesRxOut3(*rx_buf),
    //         _ => (),
    //     }
    // }

    fn port_allowed(allowed_ports: &[u16], port: u16) -> (r: bool)
        ensures
            r == allowed_ports@.contains(port),
    {
        let mut i: usize = 0;
        while i < allowed_ports.len()
            invariant
                0 <= i <= allowed_ports@.len(),
                forall |j| 0 <= j < i ==> allowed_ports@[j] != port,
            decreases
                allowed_ports@.len() - i
        {
            if allowed_ports[i] == port {
                return true;
            }
            i += 1;
        }
        false
    }

    fn udp_port_allowed(port: u16) -> (r: bool)
        ensures
            r == config::udp::ALLOWED_PORTS@.contains(port),
    {
        port_allowed(&config::udp::ALLOWED_PORTS, port)
    }

    fn tcp_port_allowed(port: u16) -> (r: bool)
        ensures
            r == config::tcp::ALLOWED_PORTS@.contains(port),
    {
        port_allowed(&config::tcp::ALLOWED_PORTS, port)
    }

    pub open spec fn packet_is_whitelisted_tcp(packet: &PacketType) -> bool
    {
        packet is Ipv4 &&
            packet->Ipv4_0.protocol is Tcp &&
            seL4_RxFirewall_RxFirewall::ipv4_tcp_on_allowed_port_quant(packet->Ipv4_0.protocol->Tcp_0.dst_port)
    }


    pub open spec fn packet_is_whitelisted_udp(packet: &PacketType) -> bool
    {
        packet is Ipv4 &&
            packet->Ipv4_0.protocol is Udp &&
            seL4_RxFirewall_RxFirewall::ipv4_udp_on_allowed_port_quant(packet->Ipv4_0.protocol->Udp_0.dst_port)
    }

    fn can_send_packet(packet: &PacketType) -> (r: bool)
        requires
            config::udp::ALLOWED_PORTS =~= GumboLib::UDP_ALLOWED_PORTS_spec(),
            config::tcp::ALLOWED_PORTS =~= GumboLib::TCP_ALLOWED_PORTS_spec(),
        ensures
            ((packet is Arp) ||
                packet_is_whitelisted_tcp(packet) ||
                packet_is_whitelisted_udp(packet)
            ) == (r == true),
    {
        match packet {
            PacketType::Arp(_) => true,
            PacketType::Ipv4(ip) => match &ip.protocol {
                Ipv4ProtoPacket::Tcp(tcp) => {
                    let allowed = tcp_port_allowed(tcp.dst_port);
                    if !allowed {
                        info("TCP packet filtered out");
                    }
                    allowed
                }
                Ipv4ProtoPacket::Udp(udp) => {
                    let allowed = udp_port_allowed(udp.dst_port);
                    if !allowed {
                        info("UDP packet filtered out");
                    }
                    allowed
                }
                _ => {
                    info_protocol(ip.header.protocol);
                    false
                }
            },
            PacketType::Ipv6 => {
                info("Not an IPv4 or Arp packet. Throw it away.");
                false
            },
        }
    }

impl seL4_RxFirewall_RxFirewall {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn get_frame_packet(frame: &open_platform_Data_Model::RawEthernetMessage) -> (r: Option<EthFrame>)
        requires
            frame@.len() == open_platform_Data_Model_RawEthernetMessage_DIM_0
        ensures
            GumboLib::valid_arp_spec(*frame) == firewall_core::res_is_arp(r),
            GumboLib::valid_ipv4_udp_spec(*frame) == firewall_core::res_is_udp(r),
            GumboLib::valid_ipv4_tcp_spec(*frame) == firewall_core::res_is_tcp(r),
            GumboLib::valid_ipv4_tcp_spec(*frame) ==> firewall_core::tcp_port_bytes_match(frame, r),
            GumboLib::valid_ipv4_udp_spec(*frame) ==> firewall_core::udp_port_bytes_match(frame, r),
    {
        let eth = EthFrame::parse(frame);
        if eth.is_none() {
            info("Malformed packet. Throw it away.")
        }
        eth
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
        // END MARKER TIME TRIGGERED REQUIRES
      ensures
        // BEGIN MARKER TIME TRIGGERED ENSURES
        // guarantee hlr_05_rx0_can_send_arp
        api.EthernetFramesRxIn0.is_some() && GumboLib::valid_arp_spec(api.EthernetFramesRxIn0.unwrap()) ==>
          api.EthernetFramesRxOut0.is_some() &&
            (api.EthernetFramesRxIn0.unwrap() == api.EthernetFramesRxOut0.unwrap()),
        // guarantee hlr_06_rx0_can_send_ipv4_tcp
        api.EthernetFramesRxIn0.is_some() && GumboLib::valid_ipv4_tcp_port_spec(api.EthernetFramesRxIn0.unwrap()) ==>
          api.EthernetFramesRxOut0.is_some() &&
            (api.EthernetFramesRxIn0.unwrap() == api.EthernetFramesRxOut0.unwrap()),
        // guarantee hlr_13_rx0_can_send_ipv4_udp
        api.EthernetFramesRxIn0.is_some() && GumboLib::valid_ipv4_udp_port_spec(api.EthernetFramesRxIn0.unwrap()) ==>
          api.EthernetFramesRxOut0.is_some() &&
            (api.EthernetFramesRxIn0.unwrap() == api.EthernetFramesRxOut0.unwrap()),
        // guarantee hlr_15_rx0_disallow
        api.EthernetFramesRxIn0.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(api.EthernetFramesRxIn0.unwrap())) ==>
          api.EthernetFramesRxOut0.is_none(),
        // guarantee hlr_17_rx0_no_input
        api.EthernetFramesRxIn0.is_some() || api.EthernetFramesRxOut0.is_none(),
        // guarantee hlr_05_rx1_can_send_arp
        api.EthernetFramesRxIn1.is_some() && GumboLib::valid_arp_spec(api.EthernetFramesRxIn1.unwrap()) ==>
          api.EthernetFramesRxOut1.is_some() &&
            (api.EthernetFramesRxIn1.unwrap() == api.EthernetFramesRxOut1.unwrap()),
        // guarantee hlr_06_rx1_can_send_ipv4_tcp
        api.EthernetFramesRxIn1.is_some() && GumboLib::valid_ipv4_tcp_port_spec(api.EthernetFramesRxIn1.unwrap()) ==>
          api.EthernetFramesRxOut1.is_some() &&
            (api.EthernetFramesRxIn1.unwrap() == api.EthernetFramesRxOut1.unwrap()),
        // guarantee hlr_13_rx1_can_send_ipv4_udp
        api.EthernetFramesRxIn1.is_some() && GumboLib::valid_ipv4_udp_port_spec(api.EthernetFramesRxIn1.unwrap()) ==>
          api.EthernetFramesRxOut1.is_some() &&
            (api.EthernetFramesRxIn1.unwrap() == api.EthernetFramesRxOut1.unwrap()),
        // guarantee hlr_15_rx1_disallow
        api.EthernetFramesRxIn1.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(api.EthernetFramesRxIn1.unwrap())) ==>
          api.EthernetFramesRxOut1.is_none(),
        // guarantee hlr_17_rx1_no_input
        api.EthernetFramesRxIn1.is_some() || api.EthernetFramesRxOut1.is_none(),
        // guarantee hlr_05_rx2_can_send_arp
        api.EthernetFramesRxIn2.is_some() && GumboLib::valid_arp_spec(api.EthernetFramesRxIn2.unwrap()) ==>
          api.EthernetFramesRxOut2.is_some() &&
            (api.EthernetFramesRxIn2.unwrap() == api.EthernetFramesRxOut2.unwrap()),
        // guarantee hlr_06_rx2_can_send_ipv4_tcp
        api.EthernetFramesRxIn2.is_some() && GumboLib::valid_ipv4_tcp_port_spec(api.EthernetFramesRxIn2.unwrap()) ==>
          api.EthernetFramesRxOut2.is_some() &&
            (api.EthernetFramesRxIn2.unwrap() == api.EthernetFramesRxOut2.unwrap()),
        // guarantee hlr_13_rx2_can_send_ipv4_udp
        api.EthernetFramesRxIn2.is_some() && GumboLib::valid_ipv4_udp_port_spec(api.EthernetFramesRxIn2.unwrap()) ==>
          api.EthernetFramesRxOut2.is_some() &&
            (api.EthernetFramesRxIn2.unwrap() == api.EthernetFramesRxOut2.unwrap()),
        // guarantee hlr_15_rx2_disallow
        api.EthernetFramesRxIn2.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(api.EthernetFramesRxIn2.unwrap())) ==>
          api.EthernetFramesRxOut2.is_none(),
        // guarantee hlr_17_rx2_no_input
        api.EthernetFramesRxIn2.is_some() || api.EthernetFramesRxOut2.is_none(),
        // guarantee hlr_05_rx3_can_send_arp
        api.EthernetFramesRxIn3.is_some() && GumboLib::valid_arp_spec(api.EthernetFramesRxIn3.unwrap()) ==>
          api.EthernetFramesRxOut3.is_some() &&
            (api.EthernetFramesRxIn3.unwrap() == api.EthernetFramesRxOut3.unwrap()),
        // guarantee hlr_06_rx3_can_send_ipv4_tcp
        api.EthernetFramesRxIn3.is_some() && GumboLib::valid_ipv4_tcp_port_spec(api.EthernetFramesRxIn3.unwrap()) ==>
          api.EthernetFramesRxOut3.is_some() &&
            (api.EthernetFramesRxIn3.unwrap() == api.EthernetFramesRxOut3.unwrap()),
        // guarantee hlr_13_rx3_can_send_ipv4_udp
        api.EthernetFramesRxIn3.is_some() && GumboLib::valid_ipv4_udp_port_spec(api.EthernetFramesRxIn3.unwrap()) ==>
          api.EthernetFramesRxOut3.is_some() &&
            (api.EthernetFramesRxIn3.unwrap() == api.EthernetFramesRxOut3.unwrap()),
        // guarantee hlr_15_rx3_disallow
        api.EthernetFramesRxIn3.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(api.EthernetFramesRxIn3.unwrap())) ==>
          api.EthernetFramesRxOut3.is_none(),
        // guarantee hlr_17_rx3_no_input
        api.EthernetFramesRxIn3.is_some() || api.EthernetFramesRxOut3.is_none(),
        // END MARKER TIME TRIGGERED ENSURES
    {
        trace("compute entrypoint invoked");

        // Rx0 ports
        if let Some(frame) = api.get_EthernetFramesRxIn0() {
            if let Some(eth) = Self::get_frame_packet(&frame) {
                if can_send_packet(&eth.eth_type) {
                    api.put_EthernetFramesRxOut0(frame);
                }
            }
        }

        // Rx1 ports
        if let Some(frame) = api.get_EthernetFramesRxIn1() {
            if let Some(eth) = Self::get_frame_packet(&frame) {
                if can_send_packet(&eth.eth_type) {
                    api.put_EthernetFramesRxOut1(frame);
                }
            }
        }

        // Rx2 ports
        if let Some(frame) = api.get_EthernetFramesRxIn2() {
            if let Some(eth) = Self::get_frame_packet(&frame) {
                if can_send_packet(&eth.eth_type) {
                    api.put_EthernetFramesRxOut2(frame);
                }
            }
        }

        // Rx3 ports
        if let Some(frame) = api.get_EthernetFramesRxIn3() {
            if let Some(eth) = Self::get_frame_packet(&frame) {
                if can_send_packet(&eth.eth_type) {
                    api.put_EthernetFramesRxOut3(frame);
                }
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

#[test]
fn tcp_port_allowed_test() {
    assert!(tcp_port_allowed(5760));
    assert!(!tcp_port_allowed(42));
}

#[test]
fn udp_port_allowed_test() {
    assert!(udp_port_allowed(68));
    assert!(!udp_port_allowed(19));
}

#[cfg(test)]
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

#[cfg(test)]
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
