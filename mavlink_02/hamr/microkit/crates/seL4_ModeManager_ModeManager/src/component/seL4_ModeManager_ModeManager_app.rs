// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_ModeManager_ModeManager_api::*;
use vstd::prelude::*;

verus! {

  pub struct seL4_ModeManager_ModeManager {
    // BEGIN MARKER STATE VARS
    pub retained_mode: open_platform_Data_Model::OperatingMode,
    // END MARKER STATE VARS
  }

  impl seL4_ModeManager_ModeManager {
    pub fn new() -> Self
    {
      Self {
        // BEGIN MARKER STATE VAR INIT
        retained_mode: open_platform_Data_Model::OperatingMode::default(),
        // END MARKER STATE VAR INIT
      }
    }

    pub fn initialize<API: seL4_ModeManager_ModeManager_Put_Api> (
      &mut self,
      api: &mut seL4_ModeManager_ModeManager_Application_Api<API>)
      ensures
        // BEGIN MARKER INITIALIZATION ENSURES
        // guarantee hlr_23_llr_11_initial_mode
        final(self).retained_mode == open_platform_Data_Model::OperatingMode::Normal,
        // guarantee hlr_26_llr_11_initial_rx_mode
        final(api).mode_to_rx == final(self).retained_mode,
        // guarantee hlr_26_llr_11_initial_mavlink_mode
        final(api).mode_to_mavlink == final(self).retained_mode,
        // END MARKER INITIALIZATION ENSURES
    {
      self.retained_mode = open_platform_Data_Model::OperatingMode::Normal;
      api.put_mode_to_rx(self.retained_mode);
      api.put_mode_to_mavlink(self.retained_mode);
      log_info("initialize entrypoint invoked");
    }

    pub fn timeTriggered<API: seL4_ModeManager_ModeManager_Full_Api> (
      &mut self,
      api: &mut seL4_ModeManager_ModeManager_Application_Api<API>)
      requires
        // PLACEHOLDER MARKER TIME TRIGGERED REQUIRES
      ensures
        // BEGIN MARKER TIME TRIGGERED ENSURES
        // guarantee hlr_27_llr_14_latched_transition
        final(self).retained_mode ==
          (if (final(api).error_status ||
            (old(self).retained_mode == open_platform_Data_Model::OperatingMode::Recovery)) {
            open_platform_Data_Model::OperatingMode::Recovery
          } else {
            open_platform_Data_Model::OperatingMode::Normal
          }),
        // guarantee hlr_26_27_llr_14_publish_rx_mode
        final(api).mode_to_rx == final(self).retained_mode,
        // guarantee hlr_26_27_llr_14_publish_mavlink_mode
        final(api).mode_to_mavlink == final(self).retained_mode,
        // END MARKER TIME TRIGGERED ENSURES
    {
      let error_status = api.get_error_status();
      if error_status {
        self.retained_mode = open_platform_Data_Model::OperatingMode::Recovery;
      }
      api.put_mode_to_rx(self.retained_mode);
      api.put_mode_to_mavlink(self.retained_mode);
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
