// This file will not be overwritten if HAMR codegen is rerun

mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use crate::test::util::test_apis::*;
  use data::*;
  use data::open_platform_Data_Model::OperatingMode::{Normal, Recovery};

  fn ipv4_udp_frame(source_port: u16, destination_port: u16, payload_length: u16) -> open_platform_Data_Model::RawEthernetMessage {
    let mut frame = [0u8; open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0];
    frame[0] = 1;
    frame[12] = 0x08;
    frame[13] = 0x00;
    frame[14] = 0x45;
    let udp_length = payload_length + 8;
    let ipv4_length = udp_length + 20;
    frame[16..18].copy_from_slice(&ipv4_length.to_be_bytes());
    frame[23] = 17;
    frame[34..36].copy_from_slice(&source_port.to_be_bytes());
    frame[36..38].copy_from_slice(&destination_port.to_be_bytes());
    frame[38..40].copy_from_slice(&udp_length.to_be_bytes());
    frame
  }

  #[test]
  #[serial]
  fn test_initialization() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
    put_current_mode(Normal);
  }

  #[test]
  #[serial]
  fn test_compute() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
    put_current_mode(Normal);
    crate::seL4_RxFirewall_RxFirewall_timeTriggered();
  }

  #[test]
  #[serial]
  fn test_unhandled_notification() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
    put_current_mode(Normal);
    crate::seL4_RxFirewall_RxFirewall_notify(99);
  }

  #[test]
  #[serial]
  fn routes_each_lane_without_cross_lane_output() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
    put_current_mode(Normal);

    let direct = ipv4_udp_frame(67, 68, 4);
    let mavlink = ipv4_udp_frame(14550, 14562, 12);
    let mut tcp = direct;
    tcp[23] = 6;

    put_concrete_inputs(Normal, Some(direct), Some(mavlink), Some(tcp), None);
    crate::seL4_RxFirewall_RxFirewall_timeTriggered();

    assert_eq!(get_EthernetFramesRxOut0(), Some(direct));
    assert_eq!(get_MAVLinkFramesRxOut0(), None);

    assert_eq!(get_EthernetFramesRxOut1(), None);
    let carrier = get_MAVLinkFramesRxOut1().expect("ArduPilot UDP must be routed to MAVLinkFirewall");
    assert_eq!(carrier.ethernet_frame, mavlink);
    assert_eq!(carrier.payload_offset, 42);
    assert_eq!(carrier.payload_length, 12);

    assert_eq!(get_EthernetFramesRxOut2(), None);
    assert_eq!(get_MAVLinkFramesRxOut2(), None);
    assert_eq!(get_EthernetFramesRxOut3(), None);
    assert_eq!(get_MAVLinkFramesRxOut3(), None);
  }

  #[test]
  #[serial]
  fn malformed_ardupilot_length_fails_closed() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
    put_current_mode(Normal);
    let mut frame = ipv4_udp_frame(14550, 14562, 12);
    frame[17] += 1;

    put_concrete_inputs(Normal, Some(frame), None, None, None);
    crate::seL4_RxFirewall_RxFirewall_timeTriggered();

    assert_eq!(get_EthernetFramesRxOut0(), None);
    assert_eq!(get_MAVLinkFramesRxOut0(), None);
  }

  #[test]
  #[serial]
  fn disallowed_udp_port_fails_closed() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
    put_current_mode(Normal);
    let frame = ipv4_udp_frame(1000, 1001, 4);

    put_concrete_inputs(Normal, Some(frame), None, None, None);
    crate::seL4_RxFirewall_RxFirewall_timeTriggered();

    assert_eq!(get_EthernetFramesRxOut0(), None);
    assert_eq!(get_MAVLinkFramesRxOut0(), None);
  }

  #[test]
  #[serial]
  fn routes_arp_and_drops_ipv6() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
    put_current_mode(Normal);
    let mut arp = [0u8; open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0];
    let arp_header = [
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 2, 3, 4, 5, 6, 7, 0x08, 0x06,
      0, 1, 0x08, 0, 6, 4, 0, 1, 2, 3, 4, 5, 6, 7, 192, 168, 0, 1,
      0, 0, 0, 0, 0, 0, 192, 168, 0, 206,
    ];
    arp[..arp_header.len()].copy_from_slice(&arp_header);

    let mut ipv6 = [0u8; open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0];
    ipv6[0] = 1;
    ipv6[12] = 0x86;
    ipv6[13] = 0xdd;

    put_concrete_inputs(Normal, Some(arp), Some(ipv6), None, None);
    crate::seL4_RxFirewall_RxFirewall_timeTriggered();

    assert_eq!(get_EthernetFramesRxOut0(), Some(arp));
    assert_eq!(get_MAVLinkFramesRxOut0(), None);
    assert_eq!(get_EthernetFramesRxOut1(), None);
    assert_eq!(get_MAVLinkFramesRxOut1(), None);
  }
  #[test]
  #[serial]
  fn strict_udp_bounds_modes_and_lane_matrix() {
    use crate::component::seL4_RxFirewall_RxFirewall_app::DIAGNOSTICS;
    let direct = ipv4_udp_frame(67, 68, 4);
    let mav = ipv4_udp_frame(14550, 14562, 12);
    let mut cases = vec![
      (direct, 1), (mav, 2),
      (ipv4_udp_frame(14550, 68, 4), 0),
      (ipv4_udp_frame(67, 14562, 4), 0),
      (ipv4_udp_frame(67, 1000, 4), 0),
      (ipv4_udp_frame(67, 68, 0), 1),
      (ipv4_udp_frame(14550, 14562, 0), 2),
      (ipv4_udp_frame(67, 68, 1558), 1),
      (ipv4_udp_frame(14550, 14562, 1558), 2),
      (ipv4_udp_frame(67, 68, 1559), 0),
      (ipv4_udp_frame(14550, 14562, 1559), 0),
    ];
    for base in [direct, mav] {
      for length in [0u16, 19, 20, 27, 1587, 9000, 65535] {
        let mut bad = base; bad[16..18].copy_from_slice(&length.to_be_bytes());
        cases.push((bad, 0));
      }
      for length in [0u16, 7, 8, 65535] {
        let mut bad = base; bad[38..40].copy_from_slice(&length.to_be_bytes());
        cases.push((bad, 0));
      }
      let mut options = base; options[14] = 0x46; cases.push((options, 0));
    }
    for mode in [Normal, Recovery] {
      for lane in 0..4 {
        for (frame, route) in &cases {
          let mut inputs = [None; 4]; inputs[lane] = Some(*frame);
          DIAGNOSTICS.lock().unwrap().clear();
          assert!(matches!(cb_apis::testComputeCB(inputs[0], inputs[1], inputs[2], inputs[3], mode), cb_apis::HarnessResult::Passed));
          let direct_outputs = [get_EthernetFramesRxOut0(), get_EthernetFramesRxOut1(), get_EthernetFramesRxOut2(), get_EthernetFramesRxOut3()];
          let mav_outputs = [get_MAVLinkFramesRxOut0(), get_MAVLinkFramesRxOut1(), get_MAVLinkFramesRxOut2(), get_MAVLinkFramesRxOut3()];
          for out_lane in 0..4 {
            let want_direct = mode == Normal && out_lane == lane && *route == 1;
            let want_mav = mode == Normal && out_lane == lane && *route == 2;
            assert_eq!(direct_outputs[out_lane], if want_direct { Some(*frame) } else { None });
            assert_eq!(mav_outputs[out_lane].is_some(), want_mav);
            if let Some(carrier) = mav_outputs[out_lane] {
              assert_eq!(carrier.ethernet_frame, *frame);
              assert_eq!(carrier.payload_offset, 42);
              assert_eq!(carrier.payload_length, u16::from_be_bytes([frame[38],frame[39]]) - 8);
            }
          }
          let logs = DIAGNOSTICS.lock().unwrap();
          if mode == Recovery { assert!(!logs.iter().any(|s| s.contains("Recovery"))); }
          else if *route == 0 { assert!(logs.iter().any(|s| s.contains("rejected"))); }
          else if *route == 2 { assert!(logs.iter().any(|s| s.contains("routing bounded UDP"))); }
        }
      }
      assert!(matches!(cb_apis::testComputeCB(None, None, None, None, mode), cb_apis::HarnessResult::Passed));
    }
  }

}

mod GUMBOX_tests {
  use serial_test::serial;
  use proptest::prelude::*;

  use crate::test::util::*;
  use crate::testInitializeCB_macro;
  use crate::testComputeCB_macro;
  use data::*;
  use data::open_platform_Data_Model::OperatingMode::{Normal, Recovery};

  fn ipv4_udp_frame(source_port: u16, destination_port: u16, payload_length: u16) -> open_platform_Data_Model::RawEthernetMessage {
    let mut frame = [0u8; open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0];
    frame[0] = 1;
    frame[12] = 0x08;
    frame[13] = 0x00;
    frame[14] = 0x45;
    let udp_length = payload_length + 8;
    let ipv4_length = udp_length + 20;
    frame[16..18].copy_from_slice(&ipv4_length.to_be_bytes());
    frame[23] = 17;
    frame[34..36].copy_from_slice(&source_port.to_be_bytes());
    frame[36..38].copy_from_slice(&destination_port.to_be_bytes());
    frame[38..40].copy_from_slice(&udp_length.to_be_bytes());
    frame
  }

  #[test]
  #[serial]
  fn every_lane_and_contract_partition_passes_oracle() {
    let direct = ipv4_udp_frame(67, 68, 4);
    let mavlink = ipv4_udp_frame(14550, 14562, 12);
    let mut drop = direct;
    drop[23] = 6;

    for inputs in [
      [Some(direct); 4],
      [Some(mavlink); 4],
      [Some(drop); 4],
      [None; 4],
    ] {
      assert!(matches!(
        cb_apis::testComputeCB(inputs[0], inputs[1], inputs[2], inputs[3], Normal),
        cb_apis::HarnessResult::Passed
      ));
    }
  }

  #[test]
  #[serial]
  fn oracle_rejects_injection_duplication_and_modified_carriers() {
    use crate::bridge::seL4_RxFirewall_RxFirewall_GUMBOX as oracle;
    let direct = ipv4_udp_frame(67, 68, 4);
    let mut changed_direct = direct; changed_direct[0] ^= 2;
    let mav = ipv4_udp_frame(14550, 14562, 12);
    let carrier = open_platform_Data_Model::MAVLinkUDPMessage_Impl {
      ethernet_frame: mav, payload_offset: 42, payload_length: 12,
    };
    let mut bad_offset = carrier; bad_offset.payload_offset = 43;
    let mut bad_length = carrier; bad_length.payload_length = 11;
    let mut bad_frame = carrier; bad_frame.ethernet_frame[0] ^= 2;
    for lane in 0..4 {
      for mode in [Normal, Recovery] {
        for (input, required_route) in [(None, 0), (Some(direct), 1), (Some(mav), 2), (Some([0u8; 1600]), 0)] {
          let mut inputs = [None; 4]; inputs[lane] = input;
          for output in [None, Some(direct), Some(changed_direct)] {
            for mout in [None, Some(carrier), Some(bad_offset), Some(bad_length), Some(bad_frame)] {
              let mut outputs = [None; 4]; outputs[lane] = output;
              let mut carriers = [None; 4]; carriers[lane] = mout;
              let route = if mode == Recovery { 0 } else { required_route };
              let expected = match route {
                1 => output == Some(direct) && mout.is_none(),
                2 => output.is_none() && mout == Some(carrier),
                _ => output.is_none() && mout.is_none(),
              };
              // Call compute directly as well as the top-level postcondition so an
              // integration failure cannot hide a permissive compute guarantee.
              assert_eq!(oracle::compute_CEP_T_Guar(inputs[0], inputs[1], inputs[2], inputs[3], mode,
                outputs[0], outputs[1], outputs[2], outputs[3], carriers[0], carriers[1], carriers[2], carriers[3]), expected);
              assert_eq!(oracle::compute_CEP_Post(inputs[0], inputs[1], inputs[2], inputs[3], mode,
                outputs[0], outputs[1], outputs[2], outputs[3], carriers[0], carriers[1], carriers[2], carriers[3]), expected);
            }
          }
        }
      }
    }
  }

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
    api_EthernetFramesRxIn0: generators::option_strategy_default(generators::open_platform_Data_Model_RawEthernetMessage_strategy_default()),
    api_EthernetFramesRxIn1: generators::option_strategy_default(generators::open_platform_Data_Model_RawEthernetMessage_strategy_default()),
    api_EthernetFramesRxIn2: generators::option_strategy_default(generators::open_platform_Data_Model_RawEthernetMessage_strategy_default()),
    api_EthernetFramesRxIn3: generators::option_strategy_default(generators::open_platform_Data_Model_RawEthernetMessage_strategy_default()),
    api_current_mode: generators::open_platform_Data_Model_OperatingMode_strategy_default()
  }
}
