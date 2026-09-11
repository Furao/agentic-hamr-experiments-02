// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_MAVLinkFirewall_MAVLinkFirewall_api::*;
use vstd::prelude::*;

pub fn mavlink_frame_valid__developer_gumbox(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool {
  mavlink_core::parse(&msg.ethernet_frame, msg.payload_offset, msg.payload_length).is_ok()
}

pub fn mavlink_firmware_flash_command__developer_gumbox(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool {
  classify_mavlink(&msg.ethernet_frame, msg.payload_offset, msg.payload_length) == CLASS_FLASH
}

verus! {
  // Message and operation names from the bundled common.xml and ardupilotmega.xml.
  pub(crate) const COMMAND_INT_ID: u32 = 75;
  pub(crate) const COMMAND_LONG_ID: u32 = 76;
  pub(crate) const SECURE_COMMAND_ID: u32 = 11004;
  pub(crate) const MAV_CMD_FLASH_BOOTLOADER: u16 = 42650;
  pub(crate) const SECURE_COMMAND_FLASH_BOOTLOADER: u32 = 7;
  // Payload fields are serialized by width, not XML declaration order.
  pub(crate) const COMMAND_FIELD_OFFSET: usize = 28;
  pub(crate) const COMMAND_FIELD_END: usize = COMMAND_FIELD_OFFSET + 2;
  pub(crate) const SECURE_OPERATION_OFFSET: usize = 4;
  pub(crate) const SECURE_OPERATION_END: usize = SECURE_OPERATION_OFFSET + 4;
  const CLASS_INVALID: u8 = 0;
  const CLASS_ALLOWED: u8 = 1;
  const CLASS_FLASH: u8 = 2;

  // Existing project carrier constraints; offsets are from the Ethernet frame start.
  pub(crate) const ETHERNET_HEADER_BYTES: u16 = 14;
  pub(crate) const IPV4_HEADER_BYTES: u16 = 20;
  pub(crate) const UDP_HEADER_BYTES: u16 = 8;
  pub(crate) const CARRIER_BYTES: usize = 1600;
  const MAX_IPV4_TOTAL_LENGTH: u16 = 9000;
  pub(crate) const ETHERTYPE_OFFSET: usize = 12;
  pub(crate) const ETHERTYPE_IPV4: u16 = 0x0800;
  pub(crate) const IPV4_VERSION_IHL_OFFSET: usize = 14;
  pub(crate) const IPV4_NO_OPTIONS: u8 = 0x45;
  pub(crate) const IPV4_TOTAL_LENGTH_OFFSET: usize = 16;
  pub(crate) const IPV4_PROTOCOL_OFFSET: usize = 23;
  pub(crate) const IP_PROTOCOL_UDP: u8 = 17;
  pub(crate) const UDP_SOURCE_PORT_OFFSET: usize = 34;
  pub(crate) const UDP_DESTINATION_PORT_OFFSET: usize = 36;
  pub(crate) const UDP_LENGTH_OFFSET: usize = 38;
  pub(crate) const MAVLINK_SOURCE_PORT: u16 = 14550;
  pub(crate) const MAVLINK_DESTINATION_PORT: u16 = 14562;
  pub(crate) const MAVLINK_CARRIER_OFFSET: u16 = ETHERNET_HEADER_BYTES + IPV4_HEADER_BYTES + UDP_HEADER_BYTES;

  fn get_command(frame: &[u8], payload_offset: usize) -> (value: u16)
    requires payload_offset + COMMAND_FIELD_END <= frame.len()
    ensures value == mavlink_core::verified::u16_le_spec(frame@, payload_offset as int + 28)
  {
    let at = payload_offset + COMMAND_FIELD_OFFSET;
    (frame[at] as u16) | ((frame[at + 1] as u16) << mavlink_core::wire::BITS_PER_BYTE)
  }

  fn get_secure_operation(frame: &[u8], payload_offset: usize) -> (value: u32)
    requires payload_offset + SECURE_OPERATION_END <= frame.len()
    ensures value == mavlink_core::verified::u32_le_spec(frame@, payload_offset as int + 4)
  {
    let at = payload_offset + SECURE_OPERATION_OFFSET;
    (frame[at] as u32) | ((frame[at + 1] as u32) << mavlink_core::wire::BITS_PER_BYTE)
      | ((frame[at + 2] as u32) << (2 * mavlink_core::wire::BITS_PER_BYTE))
      | ((frame[at + 3] as u32) << (3 * mavlink_core::wire::BITS_PER_BYTE))
  }

  const NETWORK_BYTE_RADIX: u16 = 256;

  fn get_network_u16(frame: &[u8], at: usize) -> (value: u16)
    requires at + 2 <= frame.len()
    ensures value == (frame[at as int] as u16) * 256 + frame[at as int + 1] as u16
  {
    (frame[at] as u16) * NETWORK_BYTE_RADIX + frame[at + 1] as u16
  }

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
          Err(_) => return CLASS_INVALID,
      };
      let payload_offset = parsed.payload_offset;
      let payload_length = parsed.payload_length;
      let message_id = parsed.message_id;
      if (message_id == COMMAND_INT_ID || message_id == COMMAND_LONG_ID)
          && payload_length >= COMMAND_FIELD_END
          && get_command(frame, payload_offset) == MAV_CMD_FLASH_BOOTLOADER {
          CLASS_FLASH
      } else if message_id == SECURE_COMMAND_ID
          && payload_length >= SECURE_OPERATION_END
          && get_secure_operation(frame, payload_offset) == SECURE_COMMAND_FLASH_BOOTLOADER {
          CLASS_FLASH
      } else { CLASS_ALLOWED }
  }

  #[derive(PartialEq, Eq)]
  enum Route { Allow, DenyFlash, Invalid }

  fn carrier_valid(msg: &open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> (valid: bool)
    ensures valid == GumboLib::valid_mavlink_carrier_spec(*msg)
  {
    let frame = &msg.ethernet_frame;
    let destination_valid = frame[0] != 0 || frame[1] != 0 || frame[2] != 0 ||
      frame[3] != 0 || frame[4] != 0 || frame[5] != 0;
    let ipv4_length = get_network_u16(frame, IPV4_TOTAL_LENGTH_OFFSET);
    let udp_length = get_network_u16(frame, UDP_LENGTH_OFFSET);
    let lengths_valid = ipv4_length <= MAX_IPV4_TOTAL_LENGTH && udp_length >= UDP_HEADER_BYTES &&
      ipv4_length + ETHERNET_HEADER_BYTES <= CARRIER_BYTES as u16 && ipv4_length >= IPV4_HEADER_BYTES &&
      udp_length == ipv4_length - IPV4_HEADER_BYTES;
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
    destination_valid && get_network_u16(frame, ETHERTYPE_OFFSET) == ETHERTYPE_IPV4 &&
      frame[IPV4_VERSION_IHL_OFFSET] == IPV4_NO_OPTIONS &&
      frame[IPV4_PROTOCOL_OFFSET] == IP_PROTOCOL_UDP &&
      get_network_u16(frame, UDP_SOURCE_PORT_OFFSET) == MAVLINK_SOURCE_PORT &&
      get_network_u16(frame, UDP_DESTINATION_PORT_OFFSET) == MAVLINK_DESTINATION_PORT &&
      lengths_valid &&
      msg.payload_offset == MAVLINK_CARRIER_OFFSET && msg.payload_length == udp_length - UDP_HEADER_BYTES &&
      msg.payload_offset + msg.payload_length <= CARRIER_BYTES as u16
  }

  fn classify(msg: &open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> (route: Route)
    ensures
      (route is Allow) == mavlink_allowed(*msg),
      (route is DenyFlash) == (GumboLib::valid_mavlink_carrier_spec(*msg) && mavlink_frame_valid(*msg) && mavlink_firmware_flash_command(*msg)),
  {
    if !carrier_valid(msg) { return Route::Invalid; }
    match classify_mavlink(&msg.ethernet_frame, msg.payload_offset, msg.payload_length) {
      CLASS_FLASH => Route::DenyFlash,
      CLASS_ALLOWED => Route::Allow,
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
      if let Some(msg) = api.get_EthernetFramesIn0() {
        match classify(&msg) {
          Route::Allow => api.put_EthernetFramesOut0(msg),
          Route::DenyFlash => log_info("lane 0: firmware-flash command denied"),
          Route::Invalid => log_invalid_mavlink(0, &msg),
        }
      }
      if let Some(msg) = api.get_EthernetFramesIn1() {
        match classify(&msg) {
          Route::Allow => api.put_EthernetFramesOut1(msg),
          Route::DenyFlash => log_info("lane 1: firmware-flash command denied"),
          Route::Invalid => log_invalid_mavlink(1, &msg),
        }
      }
      if let Some(msg) = api.get_EthernetFramesIn2() {
        match classify(&msg) {
          Route::Allow => api.put_EthernetFramesOut2(msg),
          Route::DenyFlash => log_info("lane 2: firmware-flash command denied"),
          Route::Invalid => log_invalid_mavlink(2, &msg),
        }
      }
      if let Some(msg) = api.get_EthernetFramesIn3() {
        match classify(&msg) {
          Route::Allow => api.put_EthernetFramesOut3(msg),
          Route::DenyFlash => log_info("lane 3: firmware-flash command denied"),
          Route::Invalid => log_invalid_mavlink(3, &msg),
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
    use super::*;
    use mavlink_core::wire::*;
    const COMMAND_INT_CRC_EXTRA: u8 = 158;
    const COMMAND_LONG_CRC_EXTRA: u8 = 152;
    const SECURE_COMMAND_CRC_EXTRA: u8 = 11;
    const COMMAND_LONG_PAYLOAD_BYTES: usize = 33;
    const SECURE_COMMAND_PREFIX_BYTES: usize = 12;
    fn v2(message_id: u32, payload: &[u8], incompat_flags: u8) -> Vec<u8> {
        let signature_length = if incompat_flags & MAVLINK_V2_SIGNED != 0 { SIGNATURE_BYTES } else { 0 };
        let checksum_offset = V2_HEADER_BYTES + payload.len();
        let mut frame = vec![0u8; checksum_offset + CHECKSUM_BYTES + signature_length];
        frame[MAGIC_OFFSET] = MAVLINK_V2_MAGIC;
        frame[PAYLOAD_LENGTH_OFFSET] = payload.len() as u8;
        frame[V2_INCOMPAT_FLAGS_OFFSET] = incompat_flags;
        frame[V2_MESSAGE_ID_OFFSET..V2_HEADER_BYTES]
            .copy_from_slice(&message_id.to_le_bytes()[..V2_MESSAGE_ID_BYTES]);
        frame[V2_HEADER_BYTES..checksum_offset].copy_from_slice(payload);
        let crc_extra = match message_id {
            COMMAND_INT_ID => COMMAND_INT_CRC_EXTRA,
            COMMAND_LONG_ID => COMMAND_LONG_CRC_EXTRA,
            SECURE_COMMAND_ID => SECURE_COMMAND_CRC_EXTRA,
            _ => panic!("unexpected fixture ID"),
        };
        write_checksum(&mut frame, checksum_offset, crc_extra);
        frame
    }

    fn v1(message_id: u8, payload: &[u8], crc_extra: u8) -> Vec<u8> {
        let checksum_offset = V1_HEADER_BYTES + payload.len();
        let mut frame = vec![0u8; checksum_offset + CHECKSUM_BYTES];
        frame[MAGIC_OFFSET] = MAVLINK_V1_MAGIC;
        frame[PAYLOAD_LENGTH_OFFSET] = payload.len() as u8;
        frame[V1_MESSAGE_ID_OFFSET] = message_id;
        frame[V1_HEADER_BYTES..checksum_offset].copy_from_slice(payload);
        write_checksum(&mut frame, checksum_offset, crc_extra);
        frame
    }

    fn write_checksum(frame: &mut [u8], checksum_offset: usize, crc_extra: u8) {
        let mut crc = CRC_INITIAL;
        for byte in &frame[CRC_START_OFFSET..checksum_offset] {
            crc = mavlink_core::verified::crc_accumulate_verified(*byte, crc);
        }
        crc = mavlink_core::verified::crc_accumulate_verified(crc_extra, crc);
        frame[checksum_offset..checksum_offset + CHECKSUM_BYTES].copy_from_slice(&crc.to_le_bytes());
    }

    #[test]
    fn firmware_policy_handles_versions_truncation_and_carrier_offsets() {
        let mut command = [0; COMMAND_LONG_PAYLOAD_BYTES];
        command[COMMAND_FIELD_OFFSET..COMMAND_FIELD_END].copy_from_slice(&MAV_CMD_FLASH_BOOTLOADER.to_le_bytes());
        let mut remote = [0; SECURE_COMMAND_PREFIX_BYTES];
        remote[SECURE_OPERATION_OFFSET..SECURE_OPERATION_END].copy_from_slice(&SECURE_COMMAND_FLASH_BOOTLOADER.to_le_bytes());
        let mut bad_crc = v2(COMMAND_LONG_ID, &command, 0);
        bad_crc[V2_HEADER_BYTES] ^= 1;
        for (packet, expected) in [
            (v2(COMMAND_LONG_ID, &command, 0), CLASS_FLASH),
            (v2(COMMAND_INT_ID, &command[..COMMAND_FIELD_END], 0), CLASS_FLASH),
            (v2(COMMAND_LONG_ID, &command[..COMMAND_FIELD_END - 1], 0), CLASS_ALLOWED),
            (v2(COMMAND_LONG_ID, &[0; COMMAND_LONG_PAYLOAD_BYTES], 0), CLASS_ALLOWED),
            (v2(SECURE_COMMAND_ID, &remote, 0), CLASS_FLASH),
            (v2(SECURE_COMMAND_ID, &remote[..SECURE_OPERATION_END - 1], 0), CLASS_ALLOWED),
            (v2(SECURE_COMMAND_ID, &[0; SECURE_COMMAND_PREFIX_BYTES], 0), CLASS_ALLOWED),
            (v2(COMMAND_LONG_ID, &command, MAVLINK_V2_SIGNED), CLASS_FLASH),
            (bad_crc, CLASS_INVALID),
        ] {
            for start in [0, MAVLINK_CARRIER_OFFSET as usize, CARRIER_BYTES - packet.len()] {
                let mut carrier = [0; CARRIER_BYTES];
                carrier[start..start + packet.len()].copy_from_slice(&packet);
                assert_eq!(classify_mavlink(&carrier, start as u16, packet.len() as u16),
                    expected);
            }
        }
        let mut command = [0; COMMAND_LONG_PAYLOAD_BYTES];
        command[COMMAND_FIELD_OFFSET..COMMAND_FIELD_END].copy_from_slice(&MAV_CMD_FLASH_BOOTLOADER.to_le_bytes());
        let packet = v1(COMMAND_LONG_ID as u8, &command, COMMAND_LONG_CRC_EXTRA);
        assert_eq!(classify_mavlink(&packet, 0, packet.len() as u16), CLASS_FLASH);
        let packet = v1(COMMAND_LONG_ID as u8, &[0; COMMAND_LONG_PAYLOAD_BYTES], COMMAND_LONG_CRC_EXTRA);
        assert_eq!(classify_mavlink(&packet, 0, packet.len() as u16), CLASS_ALLOWED);
    }
}
