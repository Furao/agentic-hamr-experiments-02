// This file will not be overwritten if HAMR codegen is rerun

mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use crate::test::util::test_apis::*;
  use data::*;

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
  }

  #[test]
  #[serial]
  fn test_compute() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
    crate::seL4_RxFirewall_RxFirewall_timeTriggered();
  }

  #[test]
  #[serial]
  fn test_unhandled_notification() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
    crate::seL4_RxFirewall_RxFirewall_notify(99);
  }

  #[test]
  #[serial]
  fn routes_each_lane_without_cross_lane_output() {
    crate::seL4_RxFirewall_RxFirewall_initialize();

    let direct = ipv4_udp_frame(67, 68, 4);
    let mavlink = ipv4_udp_frame(14550, 14562, 12);
    let mut tcp = direct;
    tcp[23] = 6;

    put_concrete_inputs(Some(direct), Some(mavlink), Some(tcp), None);
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
    let mut frame = ipv4_udp_frame(14550, 14562, 12);
    frame[17] += 1;

    put_concrete_inputs(Some(frame), None, None, None);
    crate::seL4_RxFirewall_RxFirewall_timeTriggered();

    assert_eq!(get_EthernetFramesRxOut0(), None);
    assert_eq!(get_MAVLinkFramesRxOut0(), None);
  }

  #[test]
  #[serial]
  fn disallowed_udp_port_fails_closed() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
    let frame = ipv4_udp_frame(1000, 1001, 4);

    put_concrete_inputs(Some(frame), None, None, None);
    crate::seL4_RxFirewall_RxFirewall_timeTriggered();

    assert_eq!(get_EthernetFramesRxOut0(), None);
    assert_eq!(get_MAVLinkFramesRxOut0(), None);
  }

  #[test]
  #[serial]
  fn routes_arp_and_drops_ipv6() {
    crate::seL4_RxFirewall_RxFirewall_initialize();
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

    put_concrete_inputs(Some(arp), Some(ipv6), None, None);
    crate::seL4_RxFirewall_RxFirewall_timeTriggered();

    assert_eq!(get_EthernetFramesRxOut0(), Some(arp));
    assert_eq!(get_MAVLinkFramesRxOut0(), None);
    assert_eq!(get_EthernetFramesRxOut1(), None);
    assert_eq!(get_MAVLinkFramesRxOut1(), None);
  }
}

mod GUMBOX_tests {
  use serial_test::serial;
  use proptest::prelude::*;

  use crate::test::util::*;
  use crate::testInitializeCB_macro;
  use crate::testComputeCB_macro;
  use data::*;

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
        cb_apis::testComputeCB(inputs[0], inputs[1], inputs[2], inputs[3]),
        cb_apis::HarnessResult::Passed
      ));
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
    api_EthernetFramesRxIn3: generators::option_strategy_default(generators::open_platform_Data_Model_RawEthernetMessage_strategy_default())
  }
}
