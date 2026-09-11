// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_MAVLinkFirewall_MAVLinkFirewall_api::*;
use vstd::prelude::*;

pub fn mavlink_frame_valid__developer_gumbox(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool {
  mavlink_core::parse(&msg.ethernet_frame, msg.payload_offset, msg.payload_length).is_ok()
}

pub fn mavlink_firmware_flash_command__developer_gumbox(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool {
  classify_mavlink(&msg.ethernet_frame, msg.payload_offset, msg.payload_length) == 2
}

verus! {
  pub open spec fn firmware_flash_spec(frame: Seq<u8>, offset: u16, length: u16) -> bool {
      mavlink_core::verified::frame_valid_spec(frame, offset, length) && {
        let start = offset as int;
        let header: int = if frame[start] == 0xfe { 6 } else { 10 };
        let payload = frame[start + 1] as int;
        let id: u32 = if frame[start] == 0xfe { frame[start + 5] as u32 } else { mavlink_core::verified::u24_le_spec(frame, start + 7) };
        ((id == 75 || id == 76) && payload >= 30 &&
          mavlink_core::verified::u16_le_spec(frame, start + header + 28) == 42650u16) ||
          (id == 11004 && payload >= 8 &&
            mavlink_core::verified::u32_le_spec(frame, start + header + 4) == 7u32)
      }
  }

  // Policy belongs to this component; all callers use the same verified parser.
  fn classify_mavlink(frame: &[u8], offset: u16, length: u16) -> (result: u8)
      ensures
        result <= 2,
        (result != 0) == mavlink_core::verified::frame_valid_spec(frame@, offset, length),
        (result == 2) == firmware_flash_spec(frame@, offset, length),
  {
      let parsed = match mavlink_core::verified::parse(frame, offset, length) {
          Ok(message) => message,
          Err(_) => return 0,
      };
      let payload_at = parsed.payload_offset;
      let payload = parsed.payload_length;
      let id = parsed.message_id;
      if (id == 75 || id == 76) && payload >= 30 &&
         (frame[payload_at + 28] as u16 | ((frame[payload_at + 29] as u16) << 8)) == 42650 {
          2
      } else if id == 11004 && payload >= 8 &&
         (frame[payload_at + 4] as u32 | ((frame[payload_at + 5] as u32) << 8) |
          ((frame[payload_at + 6] as u32) << 16) | ((frame[payload_at + 7] as u32) << 24)) == 7 {
          2
      } else { 1 }
  }

  #[derive(PartialEq, Eq)]
  enum Route { Allow, DenyFlash, Invalid }

  fn carrier_valid(msg: &open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> (valid: bool)
    ensures valid == GumboLib::valid_mavlink_carrier_spec(*msg)
  {
    let frame = &msg.ethernet_frame;
    let destination_valid = frame[0] != 0 || frame[1] != 0 || frame[2] != 0 ||
      frame[3] != 0 || frame[4] != 0 || frame[5] != 0;
    let ipv4_length = (frame[16] as u16) * 256 + frame[17] as u16;
    let udp_length = (frame[38] as u16) * 256 + frame[39] as u16;
    let lengths_valid = ipv4_length <= 9000 && udp_length >= 8 &&
      ipv4_length + 14 <= 1600 && ipv4_length >= 20 &&
      udp_length == ipv4_length - 20;
    proof {
      assert(ipv4_length == GumboLib::ipv4_length_spec(*frame));
      assert(udp_length == GumboLib::udp_length_spec(*frame));
      assert(destination_valid == GumboLib::valid_frame_dst_addr_spec(*frame));
      assert(lengths_valid == (GumboLib::valid_ipv4_length_spec(*frame) &&
        GumboLib::udp_header_length_spec() <= GumboLib::udp_length_spec(*frame) &&
        GumboLib::ipv4_length_spec(*frame) + GumboLib::ethernet_header_length_spec() <= 1600u16 &&
        GumboLib::ipv4_header_length_spec() <= GumboLib::ipv4_length_spec(*frame) &&
        GumboLib::udp_length_spec(*frame) == GumboLib::ipv4_length_spec(*frame) - GumboLib::ipv4_header_length_spec()));
    }
    destination_valid && frame[12] == 8 && frame[13] == 0 && frame[14] == 0x45 &&
      frame[23] == 17 &&
      frame[34] == 0x38 && frame[35] == 0xd6 && frame[36] == 0x38 && frame[37] == 0xe2 &&
      lengths_valid &&
      msg.payload_offset == 42 && msg.payload_length == udp_length - 8 &&
      msg.payload_offset + msg.payload_length <= 1600
  }

  fn classify(msg: &open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> (route: Route)
    ensures
      (route is Allow) == mavlink_allowed(*msg),
      (route is DenyFlash) == (GumboLib::valid_mavlink_carrier_spec(*msg) && mavlink_frame_valid(*msg) && mavlink_firmware_flash_command(*msg)),
  {
    if !carrier_valid(msg) { return Route::Invalid; }
    match classify_mavlink(&msg.ethernet_frame, msg.payload_offset, msg.payload_length) {
      2 => Route::DenyFlash,
      1 => Route::Allow,
      _ => Route::Invalid,
    }
  }

  pub struct seL4_MAVLinkFirewall_MAVLinkFirewall {
    // PLACEHOLDER MARKER STATE VARS
  }

  impl seL4_MAVLinkFirewall_MAVLinkFirewall {
    pub fn new() -> Self
    {
      Self {
        // PLACEHOLDER MARKER STATE VAR INIT
      }
    }

    pub fn initialize<API: seL4_MAVLinkFirewall_MAVLinkFirewall_Put_Api> (
      &mut self,
      api: &mut seL4_MAVLinkFirewall_MAVLinkFirewall_Application_Api<API>)
      ensures
        // PLACEHOLDER MARKER INITIALIZATION ENSURES
    {
      log_info("initialize entrypoint invoked");
    }

    pub fn timeTriggered<API: seL4_MAVLinkFirewall_MAVLinkFirewall_Full_Api> (
      &mut self,
      api: &mut seL4_MAVLinkFirewall_MAVLinkFirewall_Application_Api<API>)
      requires
        // BEGIN MARKER TIME TRIGGERED REQUIRES
        // assume AADL_Requirement
        //   All outgoing event ports must be empty
        old(api).EthernetFramesOut0.is_none(),
        old(api).EthernetFramesOut1.is_none(),
        old(api).EthernetFramesOut2.is_none(),
        old(api).EthernetFramesOut3.is_none(),
        // END MARKER TIME TRIGGERED REQUIRES
      ensures
        // BEGIN MARKER TIME TRIGGERED ENSURES
        // guarantee hlr_22_23_lane0_allow
        api.EthernetFramesIn0.is_some() && mavlink_allowed(api.EthernetFramesIn0.unwrap()) ==>
          api.EthernetFramesOut0.is_some() &&
            (api.EthernetFramesOut0.unwrap() == api.EthernetFramesIn0.unwrap()),
        // guarantee hlr_24_lane0_deny_flash
        api.EthernetFramesIn0.is_some() && GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn0.unwrap()) &&
          mavlink_frame_valid(api.EthernetFramesIn0.unwrap()) &&
          mavlink_firmware_flash_command(api.EthernetFramesIn0.unwrap()) ==>
          api.EthernetFramesOut0.is_none(),
        // guarantee hlr_25_26_lane0_invalid_or_no_input
        !(api.EthernetFramesIn0.is_some()) ||
          api.EthernetFramesIn0.is_some() && !(GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn0.unwrap()) && mavlink_frame_valid(api.EthernetFramesIn0.unwrap())) ==>
          api.EthernetFramesOut0.is_none(),
        // guarantee hlr_22_23_lane1_allow
        api.EthernetFramesIn1.is_some() && mavlink_allowed(api.EthernetFramesIn1.unwrap()) ==>
          api.EthernetFramesOut1.is_some() &&
            (api.EthernetFramesOut1.unwrap() == api.EthernetFramesIn1.unwrap()),
        // guarantee hlr_24_lane1_deny_flash
        api.EthernetFramesIn1.is_some() && GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn1.unwrap()) &&
          mavlink_frame_valid(api.EthernetFramesIn1.unwrap()) &&
          mavlink_firmware_flash_command(api.EthernetFramesIn1.unwrap()) ==>
          api.EthernetFramesOut1.is_none(),
        // guarantee hlr_25_26_lane1_invalid_or_no_input
        !(api.EthernetFramesIn1.is_some()) ||
          api.EthernetFramesIn1.is_some() && !(GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn1.unwrap()) && mavlink_frame_valid(api.EthernetFramesIn1.unwrap())) ==>
          api.EthernetFramesOut1.is_none(),
        // guarantee hlr_22_23_lane2_allow
        api.EthernetFramesIn2.is_some() && mavlink_allowed(api.EthernetFramesIn2.unwrap()) ==>
          api.EthernetFramesOut2.is_some() &&
            (api.EthernetFramesOut2.unwrap() == api.EthernetFramesIn2.unwrap()),
        // guarantee hlr_24_lane2_deny_flash
        api.EthernetFramesIn2.is_some() && GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn2.unwrap()) &&
          mavlink_frame_valid(api.EthernetFramesIn2.unwrap()) &&
          mavlink_firmware_flash_command(api.EthernetFramesIn2.unwrap()) ==>
          api.EthernetFramesOut2.is_none(),
        // guarantee hlr_25_26_lane2_invalid_or_no_input
        !(api.EthernetFramesIn2.is_some()) ||
          api.EthernetFramesIn2.is_some() && !(GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn2.unwrap()) && mavlink_frame_valid(api.EthernetFramesIn2.unwrap())) ==>
          api.EthernetFramesOut2.is_none(),
        // guarantee hlr_22_23_lane3_allow
        api.EthernetFramesIn3.is_some() && mavlink_allowed(api.EthernetFramesIn3.unwrap()) ==>
          api.EthernetFramesOut3.is_some() &&
            (api.EthernetFramesOut3.unwrap() == api.EthernetFramesIn3.unwrap()),
        // guarantee hlr_24_lane3_deny_flash
        api.EthernetFramesIn3.is_some() && GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn3.unwrap()) &&
          mavlink_frame_valid(api.EthernetFramesIn3.unwrap()) &&
          mavlink_firmware_flash_command(api.EthernetFramesIn3.unwrap()) ==>
          api.EthernetFramesOut3.is_none(),
        // guarantee hlr_25_26_lane3_invalid_or_no_input
        !(api.EthernetFramesIn3.is_some()) ||
          api.EthernetFramesIn3.is_some() && !(GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn3.unwrap()) && mavlink_frame_valid(api.EthernetFramesIn3.unwrap())) ==>
          api.EthernetFramesOut3.is_none(),
        // END MARKER TIME TRIGGERED ENSURES
    {
      if let Some(msg) = api.get_EthernetFramesIn0() { match classify(&msg) {
        Route::Allow => api.put_EthernetFramesOut0(msg), Route::DenyFlash => log_info("lane 0: firmware-flash command denied"), Route::Invalid => log_invalid_mavlink(0, &msg) } }
      if let Some(msg) = api.get_EthernetFramesIn1() { match classify(&msg) {
        Route::Allow => api.put_EthernetFramesOut1(msg), Route::DenyFlash => log_info("lane 1: firmware-flash command denied"), Route::Invalid => log_invalid_mavlink(1, &msg) } }
      if let Some(msg) = api.get_EthernetFramesIn2() { match classify(&msg) {
        Route::Allow => api.put_EthernetFramesOut2(msg), Route::DenyFlash => log_info("lane 2: firmware-flash command denied"), Route::Invalid => log_invalid_mavlink(2, &msg) } }
      if let Some(msg) = api.get_EthernetFramesIn3() { match classify(&msg) {
        Route::Allow => api.put_EthernetFramesOut3(msg), Route::DenyFlash => log_info("lane 3: firmware-flash command denied"), Route::Invalid => log_invalid_mavlink(3, &msg) } }
    }

    pub fn notify(
      &mut self,
      channel: microkit_channel)
    {
      // this method is called when the monitor does not handle the passed in channel
      match channel {
        _ => {
          log_warn_channel(channel)
        }
      }
    }
  }

  #[verifier::external_body]
  pub fn log_info(msg: &str)
  {
    log::info!("{0}", msg);
  }

  #[verifier::external_body]
  pub fn log_invalid_mavlink(
    lane: u8,
    msg: &open_platform_Data_Model::MAVLinkUDPMessage_Impl)
  {
    match mavlink_core::parse(&msg.ethernet_frame, msg.payload_offset, msg.payload_length) {
      Err(reason) => log::info!("lane {0}: MAVLink frame dropped: {1}", lane, reason.as_str()),
      Ok(_) => log::info!("lane {0}: MAVLink frame dropped: invalid Ethernet/IPv4/UDP carrier", lane),
    }
  }

  #[verifier::external_body]
  pub fn log_warn_channel(channel: u32)
  {
    log::warn!("Unexpected channel: {0}", channel);
  }

  pub open spec fn mavlink_frame_valid__developer_verus(
    msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl
  ) -> bool {
    mavlink_core::verified::frame_valid_spec(
      msg.ethernet_frame@, msg.payload_offset, msg.payload_length)
  }

  pub open spec fn mavlink_firmware_flash_command__developer_verus(
    msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl
  ) -> bool {
    firmware_flash_spec(
      msg.ethernet_frame@, msg.payload_offset, msg.payload_length)
  }

  // BEGIN MARKER GUMBO METHODS
  /// Verus wrapper for the GUMBO spec function `test` that delegates to the developer-supplied Verus
  /// specification function that must have the following signature:
  /// 
  ///   pub open spec fn mavlink_frame_valid__developer_verus(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> (res: bool) { ... }
  /// 
  /// The semantics of the GUMBO spec function are entirely defined by the developer-supplied implementation.
  pub open spec fn mavlink_frame_valid(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool
  {
    mavlink_frame_valid__developer_verus(msg)
  }

  /// Verus wrapper for the GUMBO spec function `test` that delegates to the developer-supplied Verus
  /// specification function that must have the following signature:
  /// 
  ///   pub open spec fn mavlink_firmware_flash_command__developer_verus(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> (res: bool) { ... }
  /// 
  /// The semantics of the GUMBO spec function are entirely defined by the developer-supplied implementation.
  pub open spec fn mavlink_firmware_flash_command(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool
  {
    mavlink_firmware_flash_command__developer_verus(msg)
  }

  pub open spec fn mavlink_allowed(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool
  {
    GumboLib::valid_mavlink_carrier_spec(msg) && mavlink_frame_valid(msg) &&
      !(mavlink_firmware_flash_command(msg))
  }
  // END MARKER GUMBO METHODS

}

#[cfg(test)]
mod policy_tests {
    use super::classify_mavlink;
    use mavlink_core::{InvalidReason, MAVLINK_V1_MAGIC, MAVLINK_V2_MAGIC, MAVLINK_V2_SIGNED};
    fn v2(message_id: u32, payload: &[u8], incompat: u8) -> Vec<u8> {
        let signature = if incompat & MAVLINK_V2_SIGNED != 0 { 13 } else { 0 };
        let mut frame = vec![0u8; 10 + payload.len() + 2 + signature];
        frame[0] = MAVLINK_V2_MAGIC;
        frame[1] = payload.len() as u8;
        frame[2] = incompat;
        frame[7] = message_id as u8;
        frame[8] = (message_id >> 8) as u8;
        frame[9] = (message_id >> 16) as u8;
        frame[10..10 + payload.len()].copy_from_slice(payload);
        let extra = match message_id { 75 => 158, 76 => 152, 11004 => 11, _ => panic!("unexpected fixture ID") };
        let mut crc = 0xffff;
        for byte in &frame[1..10 + payload.len()] { crc = mavlink_core::verified::crc_accumulate_verified(*byte, crc); }
        crc = mavlink_core::verified::crc_accumulate_verified(extra, crc);
        let at = 10 + payload.len();
        frame[at] = crc as u8;
        frame[at + 1] = (crc >> 8) as u8;
        frame
    }

    fn v1(message_id: u8, payload: &[u8], crc_extra: u8) -> Vec<u8> {
        let mut frame = vec![0u8; 6 + payload.len() + 2];
        frame[0] = MAVLINK_V1_MAGIC; frame[1] = payload.len() as u8; frame[5] = message_id;
        frame[6..6 + payload.len()].copy_from_slice(payload);
        let mut crc = 0xffff;
        for byte in &frame[1..6 + payload.len()] { crc = mavlink_core::verified::crc_accumulate_verified(*byte, crc); }
        crc = mavlink_core::verified::crc_accumulate_verified(crc_extra, crc);
        let at = 6 + payload.len(); frame[at] = crc as u8; frame[at + 1] = (crc >> 8) as u8;
        frame
    }

    #[test]
    fn firmware_policy_handles_versions_truncation_and_carrier_offsets() {
        let mut command = [0; 33];
        command[28..30].copy_from_slice(&42650u16.to_le_bytes());
        let mut remote = [0; 12];
        remote[4..8].copy_from_slice(&7u32.to_le_bytes());
        let mut bad_crc = v2(76, &command, 0);
        bad_crc[10] ^= 1;
        for (packet, expected) in [
            (v2(76, &command, 0), Ok(2)),
            (v2(75, &command[..30], 0), Ok(2)),
            (v2(76, &command[..29], 0), Ok(1)),
            (v2(76, &[0; 33], 0), Ok(1)),
            (v2(11004, &remote, 0), Ok(2)),
            (v2(11004, &remote[..7], 0), Ok(1)),
            (v2(11004, &[0; 12], 0), Ok(1)),
            (v2(76, &command, MAVLINK_V2_SIGNED), Ok(2)),
            (bad_crc, Err(InvalidReason::Checksum)),
        ] {
            for start in [0, 42, 1600 - packet.len()] {
                let mut carrier = [0; 1600];
                carrier[start..start + packet.len()].copy_from_slice(&packet);
                assert_eq!(classify_mavlink(&carrier, start as u16, packet.len() as u16),
                    expected.unwrap_or(0));
            }
        }
        let mut command = [0; 33];
        command[28..30].copy_from_slice(&42650u16.to_le_bytes());
        let packet = v1(76, &command, 152);
        assert_eq!(classify_mavlink(&packet, 0, packet.len() as u16), 2);
        let packet = v1(76, &[0; 33], 152);
        assert_eq!(classify_mavlink(&packet, 0, packet.len() as u16), 1);
    }
}
