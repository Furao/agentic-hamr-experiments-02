// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_MAVLinkFirewall_MAVLinkFirewall_api::*;
use vstd::prelude::*;

verus! {
  #[derive(PartialEq, Eq)]
  enum Route { Allow, DenyFlash, Invalid }

  const COMMAND_INT: u32 = 75;
  const COMMAND_LONG: u32 = 76;
  const SECURE_COMMAND: u32 = 11004;
  const MAV_CMD_FLASH_BOOTLOADER: u16 = 42650;
  const SECURE_FLASH_BOOTLOADER: u32 = 7;

  fn firmware_flash(msg: &open_platform_Data_Model::MAVLinkUDPMessage_Impl, parsed: mavlink_core::Message) -> bool {
    match parsed.message_id {
      COMMAND_INT | COMMAND_LONG =>
        mavlink_core::payload_u16_le(&msg.ethernet_frame, parsed, 28) == Some(MAV_CMD_FLASH_BOOTLOADER),
      SECURE_COMMAND =>
        mavlink_core::payload_u32_le(&msg.ethernet_frame, parsed, 4) == Some(SECURE_FLASH_BOOTLOADER),
      _ => false,
    }
  }

  #[verifier::external_body]
  fn classify(msg: &open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> (route: Route)
    ensures
      (route is Allow) == mavlink_allowed(*msg),
      (route is DenyFlash) == (GumboLib::valid_mavlink_carrier_spec(*msg) && mavlink_frame_valid(*msg) && mavlink_firmware_flash_command(*msg)),
  {
    match mavlink_core::parse(&msg.ethernet_frame, msg.payload_offset, msg.payload_length) {
      Ok(parsed) => if firmware_flash(msg, parsed) { Route::DenyFlash } else { Route::Allow },
      Err(_) => Route::Invalid,
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

  // BEGIN MARKER GUMBO METHODS
  pub uninterp spec fn mavlink_frame_valid__developer_verus(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool;
  pub uninterp spec fn mavlink_firmware_flash_command__developer_verus(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool;

  #[verifier::external_body]
  pub fn mavlink_frame_valid__developer_gumbox(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool {
    mavlink_core::parse(&msg.ethernet_frame, msg.payload_offset, msg.payload_length).is_ok()
  }

  #[verifier::external_body]
  pub fn mavlink_firmware_flash_command__developer_gumbox(msg: open_platform_Data_Model::MAVLinkUDPMessage_Impl) -> bool {
    match mavlink_core::parse(&msg.ethernet_frame, msg.payload_offset, msg.payload_length) {
      Ok(parsed) => firmware_flash(&msg, parsed), Err(_) => false,
    }
  }

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
