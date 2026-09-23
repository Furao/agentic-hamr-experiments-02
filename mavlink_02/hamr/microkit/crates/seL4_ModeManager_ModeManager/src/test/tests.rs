// This file will not be overwritten if HAMR codegen is rerun

mod tests {
  use serial_test::serial;
  use crate::test::util::*;
  use data::open_platform_Data_Model::OperatingMode::{self, Normal, Recovery};
  use crate::bridge::seL4_ModeManager_ModeManager_GUMBOX as oracle;
  use crate::component::seL4_ModeManager_ModeManager_app::INFO_MESSAGES;

  fn assert_mode(expected: OperatingMode) {
    assert_eq!(test_apis::get_retained_mode(), expected);
    assert_eq!(test_apis::get_mode_to_rx(), expected);
    assert_eq!(test_apis::get_mode_to_mavlink(), expected);
  }

  #[test]
  #[serial]
  fn initialization_and_reboot_reset() {
    crate::seL4_ModeManager_ModeManager_initialize();
    assert_mode(Normal);
    test_apis::put_error_status(true);
    crate::seL4_ModeManager_ModeManager_timeTriggered();
    assert_mode(Recovery);
    crate::seL4_ModeManager_ModeManager_initialize();
    assert_mode(Normal);
    test_apis::put_error_status(false);
    crate::seL4_ModeManager_ModeManager_timeTriggered();
    assert_mode(Normal);
  }

  #[test]
  #[serial]
  fn all_transitions_publish_both_outputs_each_dispatch() {
    for (prior, error, expected) in [
      (Normal, false, Normal), (Normal, true, Recovery),
      (Recovery, false, Recovery), (Recovery, true, Recovery),
    ] {
      crate::seL4_ModeManager_ModeManager_initialize();
      INFO_MESSAGES.lock().unwrap().clear();
      test_apis::put_concrete_inputs_wGSV(prior, error);
      // Empty the test output slots so stale values cannot masquerade as publication.
      *crate::bridge::extern_c_api::OUT_mode_to_rx.lock().unwrap() = None;
      *crate::bridge::extern_c_api::OUT_mode_to_mavlink.lock().unwrap() = None;
      crate::seL4_ModeManager_ModeManager_timeTriggered();
      assert_mode(expected);
      assert_eq!(*INFO_MESSAGES.lock().unwrap(), if prior != expected {
        vec!["Mode changed: Normal -> Recovery".to_string()]
      } else { vec![] });
      assert!(oracle::compute_CEP_Post(prior, expected, error,
        test_apis::get_mode_to_mavlink(), test_apis::get_mode_to_rx()));
    }
  }

  #[test]
  #[serial]
  fn recovery_stays_latched_and_notifications_preserve_state() {
    crate::seL4_ModeManager_ModeManager_initialize();
    INFO_MESSAGES.lock().unwrap().clear();
    for error in [false, true, false, false, true, false] {
      test_apis::put_error_status(error);
      crate::seL4_ModeManager_ModeManager_timeTriggered();
    }
    assert_mode(Recovery);
    assert_eq!(*INFO_MESSAGES.lock().unwrap(), vec!["Mode changed: Normal -> Recovery".to_string()]);
    crate::seL4_ModeManager_ModeManager_notify(99);
    assert_mode(Recovery);
  }

  #[test]
  #[serial]
  fn exhaustive_contract_oracle_accepts_only_required_results() {
    for state in [Normal, Recovery] {
      for rx in [Normal, Recovery] {
        for mav in [Normal, Recovery] {
          assert_eq!(oracle::initialize_IEP_Post(state, mav, rx),
            state == Normal && rx == Normal && mav == Normal);
          for (prior, error, expected) in [
            (Normal, false, Normal), (Normal, true, Recovery),
            (Recovery, false, Recovery), (Recovery, true, Recovery),
          ] {
            assert_eq!(oracle::compute_CEP_Post(prior, state, error, mav, rx),
              state == expected && rx == expected && mav == expected);
          }
        }
      }
    }
  }
}

mod GUMBOX_tests {
  use serial_test::serial;
  use proptest::prelude::*;

  use crate::test::util::*;
  use crate::testInitializeCB_macro;
  use crate::testComputeCB_macro;
    use crate::testComputeCBwGSV_macro;

  // number of valid (i.e., non-rejected) test cases that must be executed for the compute method.
  const numValidComputeTestCases: u32 = 100;

  // how many total test cases (valid + rejected) that may be attempted.
  //   0 means all inputs must satisfy the precondition (if present),
  //   5 means at most 5 rejected inputs are allowed per valid test case
  const computeRejectRatio: u32 = 5;

  const verbosity: u32 = 2;

  testInitializeCB_macro! {
    prop_testInitializeCB_macro, // test name
    config: ProptestConfig { // proptest configuration, built by overriding fields from default config
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    }
  }

  testComputeCB_macro! {
    prop_testComputeCB_macro, // test name
    config: ProptestConfig { // proptest configuration, built by overriding fields from default config
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    },
    // strategies for generating each component input
    api_error_status: generators::bool_strategy_default()
  }

  testComputeCBwGSV_macro! {
    prop_testComputeCBwGSV_macro, // test name
    config: ProptestConfig { // proptest configuration, built by overriding fields from default config
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    },
    // strategies for generating each component input
    In_retained_mode: generators::open_platform_Data_Model_OperatingMode_strategy_default(),
    api_error_status: generators::bool_strategy_default()
  }
}
