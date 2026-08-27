#![cfg_attr(not(test), no_std)]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#![allow(dead_code)]
#![allow(static_mut_refs)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_parens)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]

#![feature(proc_macro_hygiene)]
#![cfg_attr(not(verus_keep_ghost), feature(stmt_expr_attributes))]

// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use vstd::prelude::*;

macro_rules! implies {
  ($lhs: expr, $rhs: expr) => {
    !$lhs || $rhs
  };
}

macro_rules! impliesL {
  ($lhs: expr, $rhs: expr) => {
    !$lhs | $rhs
  };
}

// BEGIN MARKER GUMBO RUST MARKER
pub fn TCP_ALLOWED_PORTS() -> open_platform_Data_Model::u16Array
{
  [5760u16]
}

pub fn UDP_ALLOWED_PORTS() -> open_platform_Data_Model::u16Array
{
  [68u16]
}

pub fn two_bytes_to_u16_le(
  byte0: u8,
  byte1: u8) -> u16
{
  ((byte1) as u16) * 256u16 + ((byte0) as u16)
}

pub fn two_bytes_to_u16_be(
  byte0: u8,
  byte1: u8) -> u16
{
  ((byte0) as u16) * 256u16 + ((byte1) as u16)
}

pub fn three_bytes_to_u32(
  byte0: u8,
  byte1: u8,
  byte2: u8) -> u32
{
  ((byte2) as u32) * 65536u32 + ((byte1) as u32) * 256u32 + ((byte0) as u32)
}

pub fn frame_is_wellformed_eth2(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  valid_frame_ethertype(aframe) && valid_frame_dst_addr(aframe)
}

pub fn valid_frame_ethertype(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  frame_has_ipv4(aframe) | frame_has_arp(aframe) |
    frame_has_ipv6(aframe)
}

pub fn valid_frame_dst_addr(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    !((aframe[0] == 0u8) &&
      (aframe[1] == 0u8) &&
      (aframe[2] == 0u8) &&
      (aframe[3] == 0u8) &&
      (aframe[4] == 0u8) &&
      (aframe[5] == 0u8))
}

pub fn frame_has_ipv4(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    ((aframe[12] == 8u8) &&
      (aframe[13] == 0u8))
}

pub fn frame_has_ipv6(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    ((aframe[12] == 134u8) &&
      (aframe[13] == 221u8))
}

pub fn frame_has_arp(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    ((aframe[12] == 8u8) &&
      (aframe[13] == 6u8))
}

pub fn arp_has_ipv4(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    ((aframe[16] == 8u8) &&
      (aframe[17] == 0u8))
}

pub fn arp_has_ipv6(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    ((aframe[16] == 134u8) &&
      (aframe[17] == 221u8))
}

pub fn valid_arp_ptype(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  arp_has_ipv4(aframe) | arp_has_ipv6(aframe)
}

pub fn valid_arp_op(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    ((aframe[20] == 0u8) &&
      (aframe[21] == 1u8) |
        (aframe[21] == 2u8))
}

pub fn valid_arp_htype(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    ((aframe[14] == 0u8) &&
      (aframe[15] == 1u8))
}

pub fn wellformed_arp_frame(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  valid_arp_op(aframe) && valid_arp_htype(aframe) &&
    valid_arp_ptype(aframe)
}

pub fn valid_ipv4_length(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    (ipv4_length(aframe) <= 9000u16)
}

pub fn valid_ipv4_protocol(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    (aframe[23] == 0u8) |
      (aframe[23] == 1u8) |
      (aframe[23] == 2u8) |
      (aframe[23] == 6u8) |
      (aframe[23] == 17u8) |
      (aframe[23] == 43u8) |
      (aframe[23] == 44u8) |
      (aframe[23] == 58u8) |
      (aframe[23] == 59u8) |
      (aframe[23] == 60u8)
}

pub fn valid_ipv4_vers_ihl(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    (aframe[14] == 69u8)
}

pub fn wellformed_ipv4_frame(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  valid_ipv4_protocol(aframe) && valid_ipv4_length(aframe) &&
    valid_ipv4_vers_ihl(aframe)
}

pub fn valid_ipv6(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  frame_is_wellformed_eth2(aframe) && frame_has_ipv6(aframe)
}

pub fn valid_arp(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  frame_is_wellformed_eth2(aframe) && frame_has_arp(aframe) &&
    wellformed_arp_frame(aframe)
}

pub fn valid_ipv4(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  frame_is_wellformed_eth2(aframe) && frame_has_ipv4(aframe) &&
    wellformed_ipv4_frame(aframe)
}

pub fn ipv4_length(aframe: open_platform_Data_Model::RawEthernetMessage) -> u16
{
  two_bytes_to_u16_be(aframe[16], aframe[17])
}

pub fn valid_output_arp_size(output: open_platform_Data_Model::SizedEthernetMessage_Impl) -> bool
{
  output.sz == 64u16
}

pub fn valid_output_ipv4_size(
  input: open_platform_Data_Model::RawEthernetMessage,
  output: open_platform_Data_Model::SizedEthernetMessage_Impl) -> bool
{
  (input.len() == 1600) &&
    (output.sz == ipv4_length(input) + ethernet_header_length())
}

pub fn tx_allow_outbound_frame(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  valid_arp(aframe) | valid_ipv4(aframe)
}

pub fn ipv4_is_tcp(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    (aframe[23] == 6u8)
}

pub fn ipv4_is_udp(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    (aframe[23] == 17u8)
}

pub fn tcp_is_valid_port(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    (two_bytes_to_u16_be(aframe[36], aframe[37]) == TCP_ALLOWED_PORTS()[0])
}

pub fn udp_is_valid_port(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    (two_bytes_to_u16_be(aframe[36], aframe[37]) == UDP_ALLOWED_PORTS()[0])
}

pub fn frame_has_ipv4_tcp_on_allowed_port_quant(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    (0..=TCP_ALLOWED_PORTS().len() - 1).any(|i| TCP_ALLOWED_PORTS()[i] == two_bytes_to_u16_be(aframe[36], aframe[37]))
}

pub fn udp_is_valid_dst_port(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  (aframe.len() == 1600) &&
    (0..=UDP_ALLOWED_PORTS().len() - 1).any(|i| UDP_ALLOWED_PORTS()[i] == two_bytes_to_u16_be(aframe[36], aframe[37]))
}

pub fn valid_ipv4_tcp(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  frame_is_wellformed_eth2(aframe) && frame_has_ipv4(aframe) &&
    wellformed_ipv4_frame(aframe) &&
    ipv4_is_tcp(aframe)
}

pub fn valid_ipv4_udp(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  frame_is_wellformed_eth2(aframe) && frame_has_ipv4(aframe) &&
    wellformed_ipv4_frame(aframe) &&
    ipv4_is_udp(aframe)
}

pub fn valid_ipv4_tcp_port(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  valid_ipv4_tcp(aframe) && frame_has_ipv4_tcp_on_allowed_port_quant(aframe)
}

pub fn valid_ipv4_udp_port(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  valid_ipv4_udp(aframe) && udp_is_valid_dst_port(aframe)
}

pub fn udp_source_port(aframe: open_platform_Data_Model::RawEthernetMessage) -> u16
{
  two_bytes_to_u16_be(aframe[34], aframe[35])
}

pub fn udp_destination_port(aframe: open_platform_Data_Model::RawEthernetMessage) -> u16
{
  two_bytes_to_u16_be(aframe[36], aframe[37])
}

pub fn ethernet_header_length() -> u16
{
  14u16
}

pub fn ipv4_header_length() -> u16
{
  20u16
}

pub fn udp_header_length() -> u16
{
  8u16
}

pub fn udp_length(aframe: open_platform_Data_Model::RawEthernetMessage) -> u16
{
  two_bytes_to_u16_be(aframe[38], aframe[39])
}

pub fn udp_payload_length(aframe: open_platform_Data_Model::RawEthernetMessage) -> u16
{
  if (udp_header_length() <= udp_length(aframe)) {
    udp_length(aframe) - udp_header_length()
  } else {
    0u16
  }
}

pub fn udp_payload_offset() -> u16
{
  ethernet_header_length() + ipv4_header_length() + udp_header_length()
}

pub fn valid_ardupilot_udp(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  valid_ipv4_udp(aframe) &&
    (udp_source_port(aframe) == 14550u16) &&
    (udp_destination_port(aframe) == 14562u16) &&
    (udp_header_length() <= udp_length(aframe)) &&
    (ipv4_length(aframe) + ethernet_header_length() <= 1600u16) &&
    (ipv4_header_length() <= ipv4_length(aframe)) &&
    (udp_length(aframe) == ipv4_length(aframe) - ipv4_header_length())
}

pub fn rx_direct_frame(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  valid_arp(aframe) ||
    valid_ipv4_udp_port(aframe) && !(valid_ardupilot_udp(aframe))
}

pub fn valid_mavlink_carrier(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool
{
  valid_ardupilot_udp(msg.ethernet_frame) &&
    (msg.payload_offset == udp_payload_offset()) &&
    (msg.payload_length == udp_payload_length(msg.ethernet_frame)) &&
    (msg.payload_offset + msg.payload_length <= 1600u16)
}

pub fn rx_allow_outbound_frame(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
{
  rx_direct_frame(aframe) || valid_ardupilot_udp(aframe)
}
// END MARKER GUMBO RUST MARKER

verus! {

  // BEGIN MARKER GUMBO VERUS MARKER
  pub open spec fn TCP_ALLOWED_PORTS_spec() -> open_platform_Data_Model::u16Array
  {
    [5760u16]
  }

  pub open spec fn UDP_ALLOWED_PORTS_spec() -> open_platform_Data_Model::u16Array
  {
    [68u16]
  }

  pub open spec fn two_bytes_to_u16_le_spec(
    byte0: u8,
    byte1: u8) -> u16
  {
    (((byte1) as u16) * 256u16 + ((byte0) as u16)) as u16
  }

  pub open spec fn two_bytes_to_u16_be_spec(
    byte0: u8,
    byte1: u8) -> u16
  {
    (((byte0) as u16) * 256u16 + ((byte1) as u16)) as u16
  }

  pub open spec fn three_bytes_to_u32_spec(
    byte0: u8,
    byte1: u8,
    byte2: u8) -> u32
  {
    (((byte2) as u32) * 65536u32 + ((byte1) as u32) * 256u32 + ((byte0) as u32)) as u32
  }

  pub open spec fn frame_is_wellformed_eth2_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    valid_frame_ethertype_spec(aframe) && valid_frame_dst_addr_spec(aframe)
  }

  pub open spec fn valid_frame_ethertype_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (frame_has_ipv4_spec(aframe) || frame_has_arp_spec(aframe)) ||
      frame_has_ipv6_spec(aframe)
  }

  pub open spec fn valid_frame_dst_addr_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      !((aframe[0] == 0u8) &&
        (aframe[1] == 0u8) &&
        (aframe[2] == 0u8) &&
        (aframe[3] == 0u8) &&
        (aframe[4] == 0u8) &&
        (aframe[5] == 0u8))
  }

  pub open spec fn frame_has_ipv4_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      ((aframe[12] == 8u8) &&
        (aframe[13] == 0u8))
  }

  pub open spec fn frame_has_ipv6_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      ((aframe[12] == 134u8) &&
        (aframe[13] == 221u8))
  }

  pub open spec fn frame_has_arp_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      ((aframe[12] == 8u8) &&
        (aframe[13] == 6u8))
  }

  pub open spec fn arp_has_ipv4_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      ((aframe[16] == 8u8) &&
        (aframe[17] == 0u8))
  }

  pub open spec fn arp_has_ipv6_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      ((aframe[16] == 134u8) &&
        (aframe[17] == 221u8))
  }

  pub open spec fn valid_arp_ptype_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    arp_has_ipv4_spec(aframe) || arp_has_ipv6_spec(aframe)
  }

  pub open spec fn valid_arp_op_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      ((aframe[20] == 0u8) &&
        ((aframe[21] == 1u8) ||
          (aframe[21] == 2u8)))
  }

  pub open spec fn valid_arp_htype_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      ((aframe[14] == 0u8) &&
        (aframe[15] == 1u8))
  }

  pub open spec fn wellformed_arp_frame_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    valid_arp_op_spec(aframe) && valid_arp_htype_spec(aframe) &&
      valid_arp_ptype_spec(aframe)
  }

  pub open spec fn valid_ipv4_length_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      (ipv4_length_spec(aframe) <= 9000u16)
  }

  pub open spec fn valid_ipv4_protocol_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      ((((((((((aframe[23] == 0u8) ||
        (aframe[23] == 1u8)) ||
        (aframe[23] == 2u8)) ||
        (aframe[23] == 6u8)) ||
        (aframe[23] == 17u8)) ||
        (aframe[23] == 43u8)) ||
        (aframe[23] == 44u8)) ||
        (aframe[23] == 58u8)) ||
        (aframe[23] == 59u8)) ||
        (aframe[23] == 60u8))
  }

  pub open spec fn valid_ipv4_vers_ihl_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      (aframe[14] == 69u8)
  }

  pub open spec fn wellformed_ipv4_frame_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    valid_ipv4_protocol_spec(aframe) && valid_ipv4_length_spec(aframe) &&
      valid_ipv4_vers_ihl_spec(aframe)
  }

  pub open spec fn valid_ipv6_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    frame_is_wellformed_eth2_spec(aframe) && frame_has_ipv6_spec(aframe)
  }

  pub open spec fn valid_arp_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    frame_is_wellformed_eth2_spec(aframe) && frame_has_arp_spec(aframe) &&
      wellformed_arp_frame_spec(aframe)
  }

  pub open spec fn valid_ipv4_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    frame_is_wellformed_eth2_spec(aframe) && frame_has_ipv4_spec(aframe) &&
      wellformed_ipv4_frame_spec(aframe)
  }

  pub open spec fn ipv4_length_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> u16
  {
    two_bytes_to_u16_be_spec(aframe[16], aframe[17])
  }

  pub open spec fn valid_output_arp_size_spec(output: open_platform_Data_Model::SizedEthernetMessage_Impl) -> bool
  {
    output.sz == 64u16
  }

  pub open spec fn valid_output_ipv4_size_spec(
    input: open_platform_Data_Model::RawEthernetMessage,
    output: open_platform_Data_Model::SizedEthernetMessage_Impl) -> bool
  {
    (input.len() == 1600) &&
      (output.sz == ipv4_length_spec(input) + ethernet_header_length_spec())
  }

  pub open spec fn tx_allow_outbound_frame_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    valid_arp_spec(aframe) || valid_ipv4_spec(aframe)
  }

  pub open spec fn ipv4_is_tcp_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      (aframe[23] == 6u8)
  }

  pub open spec fn ipv4_is_udp_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      (aframe[23] == 17u8)
  }

  pub open spec fn tcp_is_valid_port_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      (two_bytes_to_u16_be_spec(aframe[36], aframe[37]) == TCP_ALLOWED_PORTS_spec()[0])
  }

  pub open spec fn udp_is_valid_port_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      (two_bytes_to_u16_be_spec(aframe[36], aframe[37]) == UDP_ALLOWED_PORTS_spec()[0])
  }

  pub open spec fn frame_has_ipv4_tcp_on_allowed_port_quant_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      exists|i:int| 0 <= i <= TCP_ALLOWED_PORTS_spec().len() - 1 && #[trigger] TCP_ALLOWED_PORTS_spec()[i] == two_bytes_to_u16_be_spec(aframe[36], aframe[37])
  }

  pub open spec fn udp_is_valid_dst_port_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    (aframe.len() == 1600) &&
      exists|i:int| 0 <= i <= UDP_ALLOWED_PORTS_spec().len() - 1 && #[trigger] UDP_ALLOWED_PORTS_spec()[i] == two_bytes_to_u16_be_spec(aframe[36], aframe[37])
  }

  pub open spec fn valid_ipv4_tcp_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    frame_is_wellformed_eth2_spec(aframe) && frame_has_ipv4_spec(aframe) &&
      wellformed_ipv4_frame_spec(aframe) &&
      ipv4_is_tcp_spec(aframe)
  }

  pub open spec fn valid_ipv4_udp_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    frame_is_wellformed_eth2_spec(aframe) && frame_has_ipv4_spec(aframe) &&
      wellformed_ipv4_frame_spec(aframe) &&
      ipv4_is_udp_spec(aframe)
  }

  pub open spec fn valid_ipv4_tcp_port_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    valid_ipv4_tcp_spec(aframe) && frame_has_ipv4_tcp_on_allowed_port_quant_spec(aframe)
  }

  pub open spec fn valid_ipv4_udp_port_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    valid_ipv4_udp_spec(aframe) && udp_is_valid_dst_port_spec(aframe)
  }

  pub open spec fn udp_source_port_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> u16
  {
    two_bytes_to_u16_be_spec(aframe[34], aframe[35])
  }

  pub open spec fn udp_destination_port_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> u16
  {
    two_bytes_to_u16_be_spec(aframe[36], aframe[37])
  }

  pub open spec fn ethernet_header_length_spec() -> u16
  {
    14u16
  }

  pub open spec fn ipv4_header_length_spec() -> u16
  {
    20u16
  }

  pub open spec fn udp_header_length_spec() -> u16
  {
    8u16
  }

  pub open spec fn udp_length_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> u16
  {
    two_bytes_to_u16_be_spec(aframe[38], aframe[39])
  }

  pub open spec fn udp_payload_length_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> u16
  {
    if (udp_header_length_spec() <= udp_length_spec(aframe)) {
      (udp_length_spec(aframe) - udp_header_length_spec()) as u16
    } else {
      0u16
    }
  }

  pub open spec fn udp_payload_offset_spec() -> u16
  {
    (ethernet_header_length_spec() + ipv4_header_length_spec() + udp_header_length_spec()) as u16
  }

  pub open spec fn valid_ardupilot_udp_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    valid_ipv4_udp_spec(aframe) &&
      (udp_source_port_spec(aframe) == 14550u16) &&
      (udp_destination_port_spec(aframe) == 14562u16) &&
      (udp_header_length_spec() <= udp_length_spec(aframe)) &&
      (ipv4_length_spec(aframe) + ethernet_header_length_spec() <= 1600u16) &&
      (ipv4_header_length_spec() <= ipv4_length_spec(aframe)) &&
      (udp_length_spec(aframe) == ipv4_length_spec(aframe) - ipv4_header_length_spec())
  }

  pub open spec fn rx_direct_frame_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    valid_arp_spec(aframe) ||
      valid_ipv4_udp_port_spec(aframe) && !(valid_ardupilot_udp_spec(aframe))
  }

  pub open spec fn valid_mavlink_carrier_spec(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool
  {
    valid_ardupilot_udp_spec(msg.ethernet_frame) &&
      (msg.payload_offset == udp_payload_offset_spec()) &&
      (msg.payload_length == udp_payload_length_spec(msg.ethernet_frame)) &&
      (msg.payload_offset + msg.payload_length <= 1600u16)
  }

  pub open spec fn rx_allow_outbound_frame_spec(aframe: open_platform_Data_Model::RawEthernetMessage) -> bool
  {
    rx_direct_frame_spec(aframe) || valid_ardupilot_udp_spec(aframe)
  }
  // END MARKER GUMBO VERUS MARKER

}
