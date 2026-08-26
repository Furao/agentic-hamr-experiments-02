// This file will not be overwritten if HAMR codegen is rerun

pub(super) mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use crate::test::util::test_apis::*;
  use data::*;

  fn crc_accumulate(byte: u8, crc: u16) -> u16 {
    let mut tmp = byte ^ crc as u8; tmp ^= tmp << 4;
    (crc >> 8) ^ ((tmp as u16) << 8) ^ ((tmp as u16) << 3) ^ ((tmp as u16) >> 4)
  }

  pub(super) fn carrier(message_id: u32, payload: Vec<u8>, crc_extra: u8) -> open_platform_Data_Model::MAVLinkUDPMessage_Impl {
    let mut mav = vec![0u8; 10 + payload.len() + 2];
    mav[0] = 0xfd; mav[1] = payload.len() as u8;
    mav[7] = message_id as u8; mav[8] = (message_id >> 8) as u8; mav[9] = (message_id >> 16) as u8;
    mav[10..10 + payload.len()].copy_from_slice(&payload);
    let mut crc = 0xffff;
    for byte in &mav[1..10 + payload.len()] { crc = crc_accumulate(*byte, crc); }
    crc = crc_accumulate(crc_extra, crc);
    let checksum = 10 + payload.len(); mav[checksum] = crc as u8; mav[checksum + 1] = (crc >> 8) as u8;

    let mut ethernet_frame = [0u8; open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0];
    ethernet_frame[0] = 1; ethernet_frame[12] = 0x08; ethernet_frame[13] = 0x00; ethernet_frame[14] = 0x45;
    let udp_length = (8 + mav.len()) as u16; let ipv4_length = 20 + udp_length;
    ethernet_frame[16..18].copy_from_slice(&ipv4_length.to_be_bytes()); ethernet_frame[23] = 17;
    ethernet_frame[34..36].copy_from_slice(&14550u16.to_be_bytes()); ethernet_frame[36..38].copy_from_slice(&14562u16.to_be_bytes());
    ethernet_frame[38..40].copy_from_slice(&udp_length.to_be_bytes()); ethernet_frame[42..42 + mav.len()].copy_from_slice(&mav);
    open_platform_Data_Model::MAVLinkUDPMessage_Impl { ethernet_frame, payload_offset: 42, payload_length: mav.len() as u16 }
  }

  #[test]
  #[serial]
  fn test_initialization() {
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_initialize();
  }

  #[test]
  #[serial]
  fn test_compute() {
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_initialize();
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_timeTriggered();
  }

  #[test]
  #[serial]
  fn routes_allowed_and_drops_flash_and_malformed_per_lane() {
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_initialize();
    let ftp = carrier(110, vec![0; 254], 84);
    let mut command_payload = vec![0; 33]; command_payload[28..30].copy_from_slice(&42650u16.to_le_bytes());
    let command = carrier(76, command_payload, 152);
    let mut secure_payload = vec![0; 232]; secure_payload[4..8].copy_from_slice(&7u32.to_le_bytes());
    let secure = carrier(11004, secure_payload, 11);
    let mut malformed = ftp; malformed.ethernet_frame[52] ^= 1;

    put_concrete_inputs(Some(ftp), Some(command), Some(secure), Some(malformed));
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_timeTriggered();

    assert_eq!(get_EthernetFramesOut0(), Some(ftp));
    assert_eq!(get_EthernetFramesOut1(), None);
    assert_eq!(get_EthernetFramesOut2(), None);
    assert_eq!(get_EthernetFramesOut3(), None);
  }

  #[test]
  #[serial]
  fn allows_non_flash_command_and_handles_notification() {
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_initialize();
    let command = carrier(75, vec![0; 35], 158);
    put_concrete_inputs(None, Some(command), None, None);
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_timeTriggered();
    assert_eq!(get_EthernetFramesOut1(), Some(command));
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_notify(99);
  }
}

mod GUMBOX_tests {
  use serial_test::serial;
  use proptest::prelude::*;

  use crate::test::util::*;
  use crate::testInitializeCB_macro;
  use crate::testComputeCB_macro;
  use crate::bridge::seL4_MAVLinkFirewall_MAVLinkFirewall_GUMBOX as oracle;

  // number of valid (i.e., non-rejected) test cases that must be executed for the compute method.
  const numValidComputeTestCases: u32 = 100;

  // how many total test cases (valid + rejected) that may be attempted.
  //   0 means all inputs must satisfy the precondition (if present),
  //   5 means at most 5 rejected inputs are allowed per valid test case
  const computeRejectRatio: u32 = 5;

  const verbosity: u32 = 2;

  #[test]
  fn manual_allow_deny_invalid_and_no_input_partitions() {
    let ftp = super::tests::carrier(110, vec![0; 254], 84);
    let mut flash_payload = vec![0; 33];
    flash_payload[28..30].copy_from_slice(&42650u16.to_le_bytes());
    let flash = super::tests::carrier(76, flash_payload, 152);
    let mut invalid = ftp;
    invalid.ethernet_frame[52] ^= 1;
    for lane in 0..4 {
      for (input, output) in [(Some(ftp), Some(ftp)), (Some(flash), None), (Some(invalid), None), (None, None)] {
        let mut inputs = [None; 4]; let mut outputs = [None; 4];
        inputs[lane] = input; outputs[lane] = output;
        assert!(oracle::compute_CEP_Post(
          inputs[0], inputs[1], inputs[2], inputs[3],
          outputs[0], outputs[1], outputs[2], outputs[3]));
      }
    }
  }

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
    api_EthernetFramesIn0: generators::option_strategy_default(generators::open_platform_Data_Model_MAVLinkUDPMessage_Impl_strategy_default()),
    api_EthernetFramesIn1: generators::option_strategy_default(generators::open_platform_Data_Model_MAVLinkUDPMessage_Impl_strategy_default()),
    api_EthernetFramesIn2: generators::option_strategy_default(generators::open_platform_Data_Model_MAVLinkUDPMessage_Impl_strategy_default()),
    api_EthernetFramesIn3: generators::option_strategy_default(generators::open_platform_Data_Model_MAVLinkUDPMessage_Impl_strategy_default())
  }
}
