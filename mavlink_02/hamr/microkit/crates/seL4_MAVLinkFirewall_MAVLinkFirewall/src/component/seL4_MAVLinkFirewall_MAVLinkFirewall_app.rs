// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_MAVLinkFirewall_MAVLinkFirewall_api::*;
use vstd::prelude::*;

fn firmware_flash_runtime(msg: &open_platform_Data_Model::MAVLinkUDPMessage_Impl, parsed: mavlink_core::Message) -> bool {
  match parsed.message_id {
    75 | 76 => mavlink_core::payload_u16_le(&msg.ethernet_frame, parsed, 28) == Some(42650),
    11004 => mavlink_core::payload_u32_le(&msg.ethernet_frame, parsed, 4) == Some(7),
    _ => false,
  }
}

pub fn mavlink_frame_valid__developer_gumbox(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool {
  mavlink_core::parse(&msg.ethernet_frame, msg.payload_offset, msg.payload_length).is_ok()
}

pub fn mavlink_firmware_flash_command__developer_gumbox(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool {
  match mavlink_core::parse(&msg.ethernet_frame, msg.payload_offset, msg.payload_length) {
    Ok(parsed) => firmware_flash_runtime(&msg, parsed), Err(_) => false,
  }
}

verus! {
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
    match mavlink_core::verified::classify(&msg.ethernet_frame, msg.payload_offset, msg.payload_length) {
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
      log_info("compute entrypoint invoked");
      if let Some(msg) = api.get_EthernetFramesIn0() { match classify(&msg) {
        Route::Allow => api.put_EthernetFramesOut0(msg), Route::DenyFlash => log_info("lane 0: firmware-flash command denied"), Route::Invalid => log_info("lane 0: malformed MAVLink frame dropped") } }
      if let Some(msg) = api.get_EthernetFramesIn1() { match classify(&msg) {
        Route::Allow => api.put_EthernetFramesOut1(msg), Route::DenyFlash => log_info("lane 1: firmware-flash command denied"), Route::Invalid => log_info("lane 1: malformed MAVLink frame dropped") } }
      if let Some(msg) = api.get_EthernetFramesIn2() { match classify(&msg) {
        Route::Allow => api.put_EthernetFramesOut2(msg), Route::DenyFlash => log_info("lane 2: firmware-flash command denied"), Route::Invalid => log_info("lane 2: malformed MAVLink frame dropped") } }
      if let Some(msg) = api.get_EthernetFramesIn3() { match classify(&msg) {
        Route::Allow => api.put_EthernetFramesOut3(msg), Route::DenyFlash => log_info("lane 3: firmware-flash command denied"), Route::Invalid => log_info("lane 3: malformed MAVLink frame dropped") } }
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
    mavlink_core::verified::firmware_flash_spec(
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
