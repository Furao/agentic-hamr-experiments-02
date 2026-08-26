// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_MAVLinkFirewall_MAVLinkFirewall_api::*;
use vstd::prelude::*;

verus! {

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
        api.EthernetFramesIn0.is_some() && GumboLib::mavlink_allowed_spec(api.EthernetFramesIn0.unwrap()) ==>
          api.EthernetFramesOut0.is_some() &&
            (api.EthernetFramesOut0.unwrap() == api.EthernetFramesIn0.unwrap()),
        // guarantee hlr_24_lane0_deny_flash
        api.EthernetFramesIn0.is_some() && GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn0.unwrap()) &&
          GumboLib::mavlink_frame_valid_spec(api.EthernetFramesIn0.unwrap()) &&
          GumboLib::mavlink_firmware_flash_command_spec(api.EthernetFramesIn0.unwrap()) ==>
          api.EthernetFramesOut0.is_none(),
        // guarantee hlr_25_26_lane0_invalid_or_no_input
        !(api.EthernetFramesIn0.is_some()) ||
          api.EthernetFramesIn0.is_some() && !(GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn0.unwrap()) && GumboLib::mavlink_frame_valid_spec(api.EthernetFramesIn0.unwrap())) ==>
          api.EthernetFramesOut0.is_none(),
        // guarantee hlr_22_23_lane1_allow
        api.EthernetFramesIn1.is_some() && GumboLib::mavlink_allowed_spec(api.EthernetFramesIn1.unwrap()) ==>
          api.EthernetFramesOut1.is_some() &&
            (api.EthernetFramesOut1.unwrap() == api.EthernetFramesIn1.unwrap()),
        // guarantee hlr_24_lane1_deny_flash
        api.EthernetFramesIn1.is_some() && GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn1.unwrap()) &&
          GumboLib::mavlink_frame_valid_spec(api.EthernetFramesIn1.unwrap()) &&
          GumboLib::mavlink_firmware_flash_command_spec(api.EthernetFramesIn1.unwrap()) ==>
          api.EthernetFramesOut1.is_none(),
        // guarantee hlr_25_26_lane1_invalid_or_no_input
        !(api.EthernetFramesIn1.is_some()) ||
          api.EthernetFramesIn1.is_some() && !(GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn1.unwrap()) && GumboLib::mavlink_frame_valid_spec(api.EthernetFramesIn1.unwrap())) ==>
          api.EthernetFramesOut1.is_none(),
        // guarantee hlr_22_23_lane2_allow
        api.EthernetFramesIn2.is_some() && GumboLib::mavlink_allowed_spec(api.EthernetFramesIn2.unwrap()) ==>
          api.EthernetFramesOut2.is_some() &&
            (api.EthernetFramesOut2.unwrap() == api.EthernetFramesIn2.unwrap()),
        // guarantee hlr_24_lane2_deny_flash
        api.EthernetFramesIn2.is_some() && GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn2.unwrap()) &&
          GumboLib::mavlink_frame_valid_spec(api.EthernetFramesIn2.unwrap()) &&
          GumboLib::mavlink_firmware_flash_command_spec(api.EthernetFramesIn2.unwrap()) ==>
          api.EthernetFramesOut2.is_none(),
        // guarantee hlr_25_26_lane2_invalid_or_no_input
        !(api.EthernetFramesIn2.is_some()) ||
          api.EthernetFramesIn2.is_some() && !(GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn2.unwrap()) && GumboLib::mavlink_frame_valid_spec(api.EthernetFramesIn2.unwrap())) ==>
          api.EthernetFramesOut2.is_none(),
        // guarantee hlr_22_23_lane3_allow
        api.EthernetFramesIn3.is_some() && GumboLib::mavlink_allowed_spec(api.EthernetFramesIn3.unwrap()) ==>
          api.EthernetFramesOut3.is_some() &&
            (api.EthernetFramesOut3.unwrap() == api.EthernetFramesIn3.unwrap()),
        // guarantee hlr_24_lane3_deny_flash
        api.EthernetFramesIn3.is_some() && GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn3.unwrap()) &&
          GumboLib::mavlink_frame_valid_spec(api.EthernetFramesIn3.unwrap()) &&
          GumboLib::mavlink_firmware_flash_command_spec(api.EthernetFramesIn3.unwrap()) ==>
          api.EthernetFramesOut3.is_none(),
        // guarantee hlr_25_26_lane3_invalid_or_no_input
        !(api.EthernetFramesIn3.is_some()) ||
          api.EthernetFramesIn3.is_some() && !(GumboLib::valid_mavlink_carrier_spec(api.EthernetFramesIn3.unwrap()) && GumboLib::mavlink_frame_valid_spec(api.EthernetFramesIn3.unwrap())) ==>
          api.EthernetFramesOut3.is_none(),
        // END MARKER TIME TRIGGERED ENSURES
    {
      log_info("compute entrypoint invoked");
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

  // PLACEHOLDER MARKER GUMBO METHODS

}
