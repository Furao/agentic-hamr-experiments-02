// This file will not be overwritten if HAMR codegen is rerun

// Fixture metadata from the bundled MAVLink XML dialects.
use crate::component::seL4_MAVLinkFirewall_MAVLinkFirewall_app::*;
use mavlink_core::wire::*;
const COMMAND_INT_PAYLOAD_BYTES: usize = 35;
const COMMAND_LONG_PAYLOAD_BYTES: usize = 33;
const SECURE_COMMAND_PAYLOAD_BYTES: usize = 232;
const COMMAND_INT_CRC_EXTRA: u8 = 158;
const COMMAND_LONG_CRC_EXTRA: u8 = 152;
const SECURE_COMMAND_CRC_EXTRA: u8 = 11;
const FILE_TRANSFER_PROTOCOL_ID: u32 = 110;
const FILE_TRANSFER_PROTOCOL_PAYLOAD_BYTES: usize = 254;
const FILE_TRANSFER_PROTOCOL_CRC_EXTRA: u8 = 84;

pub(super) mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use crate::test::util::test_apis::*;
  use data::*;
  use super::*;

  fn crc_accumulate(byte: u8, crc: u16) -> u16 {
    let mut tmp = byte ^ crc as u8; tmp ^= tmp << CRC_NIBBLE_SHIFT;
    (crc >> BITS_PER_BYTE) ^ ((tmp as u16) << BITS_PER_BYTE)
      ^ ((tmp as u16) << CRC_POLYNOMIAL_MIX_SHIFT) ^ ((tmp as u16) >> CRC_NIBBLE_SHIFT)
  }

  pub(super) fn carrier(message_id: u32, payload: Vec<u8>, crc_extra: u8) -> open_platform_Data_Model::MAVLinkUDPMessage_Impl {
    let checksum_offset = V2_HEADER_BYTES + payload.len();
    let mut mav = vec![0u8; checksum_offset + CHECKSUM_BYTES];
    mav[MAGIC_OFFSET] = MAVLINK_V2_MAGIC;
    mav[PAYLOAD_LENGTH_OFFSET] = payload.len() as u8;
    mav[V2_MESSAGE_ID_OFFSET..V2_HEADER_BYTES]
      .copy_from_slice(&message_id.to_le_bytes()[..V2_MESSAGE_ID_BYTES]);
    mav[V2_HEADER_BYTES..checksum_offset].copy_from_slice(&payload);
    let mut crc = CRC_INITIAL;
    for byte in &mav[CRC_START_OFFSET..checksum_offset] { crc = crc_accumulate(*byte, crc); }
    crc = crc_accumulate(crc_extra, crc);
    mav[checksum_offset..checksum_offset + CHECKSUM_BYTES].copy_from_slice(&crc.to_le_bytes());

    let mut ethernet_frame = [0u8; open_platform_Data_Model::open_platform_Data_Model_RawEthernetMessage_DIM_0];
    ethernet_frame[0] = 1; // Nonzero destination MAC.
    ethernet_frame[ETHERTYPE_OFFSET..ETHERTYPE_OFFSET + 2].copy_from_slice(&ETHERTYPE_IPV4.to_be_bytes());
    ethernet_frame[IPV4_VERSION_IHL_OFFSET] = IPV4_NO_OPTIONS;
    let udp_length = UDP_HEADER_BYTES + mav.len() as u16;
    let ipv4_length = IPV4_HEADER_BYTES + udp_length;
    ethernet_frame[IPV4_TOTAL_LENGTH_OFFSET..IPV4_TOTAL_LENGTH_OFFSET + 2].copy_from_slice(&ipv4_length.to_be_bytes());
    ethernet_frame[IPV4_PROTOCOL_OFFSET] = IP_PROTOCOL_UDP;
    ethernet_frame[UDP_SOURCE_PORT_OFFSET..UDP_SOURCE_PORT_OFFSET + 2].copy_from_slice(&MAVLINK_SOURCE_PORT.to_be_bytes());
    ethernet_frame[UDP_DESTINATION_PORT_OFFSET..UDP_DESTINATION_PORT_OFFSET + 2].copy_from_slice(&MAVLINK_DESTINATION_PORT.to_be_bytes());
    ethernet_frame[UDP_LENGTH_OFFSET..UDP_LENGTH_OFFSET + 2].copy_from_slice(&udp_length.to_be_bytes());
    let payload_offset = MAVLINK_CARRIER_OFFSET as usize;
    ethernet_frame[payload_offset..payload_offset + mav.len()].copy_from_slice(&mav);
    open_platform_Data_Model::MAVLinkUDPMessage_Impl {
      ethernet_frame, payload_offset: MAVLINK_CARRIER_OFFSET, payload_length: mav.len() as u16,
    }
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
    let ftp = carrier(FILE_TRANSFER_PROTOCOL_ID, vec![0; FILE_TRANSFER_PROTOCOL_PAYLOAD_BYTES], FILE_TRANSFER_PROTOCOL_CRC_EXTRA);
    let mut command_payload = vec![0; COMMAND_LONG_PAYLOAD_BYTES]; command_payload[COMMAND_FIELD_OFFSET..COMMAND_FIELD_END].copy_from_slice(&MAV_CMD_FLASH_BOOTLOADER.to_le_bytes());
    let command = carrier(COMMAND_LONG_ID, command_payload, COMMAND_LONG_CRC_EXTRA);
    let mut secure_payload = vec![0; SECURE_COMMAND_PAYLOAD_BYTES]; secure_payload[SECURE_OPERATION_OFFSET..SECURE_OPERATION_END].copy_from_slice(&SECURE_COMMAND_FLASH_BOOTLOADER.to_le_bytes());
    let secure = carrier(SECURE_COMMAND_ID, secure_payload, SECURE_COMMAND_CRC_EXTRA);
    let mut malformed = ftp; malformed.ethernet_frame[MAVLINK_CARRIER_OFFSET as usize + V2_HEADER_BYTES] ^= 1;

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
    let command = carrier(COMMAND_INT_ID, vec![0; COMMAND_INT_PAYLOAD_BYTES], COMMAND_INT_CRC_EXTRA);
    put_concrete_inputs(None, Some(command), None, None);
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_timeTriggered();
    assert_eq!(get_EthernetFramesOut1(), Some(command));
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_notify(99);
  }
}

mod GUMBOX_tests {
  use super::*;
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
    let ftp = super::tests::carrier(FILE_TRANSFER_PROTOCOL_ID, vec![0; FILE_TRANSFER_PROTOCOL_PAYLOAD_BYTES], FILE_TRANSFER_PROTOCOL_CRC_EXTRA);
    let mut flash_payload = vec![0; COMMAND_LONG_PAYLOAD_BYTES];
    flash_payload[COMMAND_FIELD_OFFSET..COMMAND_FIELD_END].copy_from_slice(&MAV_CMD_FLASH_BOOTLOADER.to_le_bytes());
    let flash = super::tests::carrier(COMMAND_LONG_ID, flash_payload, COMMAND_LONG_CRC_EXTRA);
    let mut invalid = ftp;
    invalid.ethernet_frame[MAVLINK_CARRIER_OFFSET as usize + V2_HEADER_BYTES] ^= 1;
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
