// This file will not be overwritten if HAMR codegen is rerun

// Fixture metadata from the bundled MAVLink XML dialects.
use crate::component::seL4_MAVLinkFirewall_MAVLinkFirewall_app::*;
use mavlink_core::wire::*;
use data::open_platform_Data_Model::OperatingMode::{Normal, Recovery};
use crate::component::seL4_MAVLinkFirewall_MAVLinkFirewall_app::ONE_BYTE_SHIFT;
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
    (crc >> ONE_BYTE_SHIFT) ^ ((tmp as u16) << ONE_BYTE_SHIFT)
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
    put_concrete_inputs(Normal, None, None, None, None);
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

    put_concrete_inputs(Normal, Some(ftp), Some(command), Some(secure), Some(malformed));
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
    put_concrete_inputs(Normal, None, Some(command), None, None);
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_timeTriggered();
    assert_eq!(get_EthernetFramesOut1(), Some(command));
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_notify(99);
  }
  type Message = open_platform_Data_Model::MAVLinkUDPMessage_Impl;

  fn outputs() -> [Option<Message>; 4] {
    [get_EthernetFramesOut0(), get_EthernetFramesOut1(), get_EthernetFramesOut2(), get_EthernetFramesOut3()]
  }

  fn dispatch(mode: open_platform_Data_Model::OperatingMode, inputs: [Option<Message>; 4]) {
    // HAMR clears outgoing event slots before each component dispatch.
    use crate::bridge::extern_c_api as ports;
    for port in [&*ports::OUT_EthernetFramesOut0, &*ports::OUT_EthernetFramesOut1,
                 &*ports::OUT_EthernetFramesOut2, &*ports::OUT_EthernetFramesOut3] {
      *port.lock().unwrap() = None;
    }
    put_concrete_inputs(mode, inputs[0], inputs[1], inputs[2], inputs[3]);
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_timeTriggered();
  }

  #[test]
  #[serial]
  fn count_modes_threshold_saturation_and_empty_dispatches() {
    let allowed = carrier(COMMAND_INT_ID, vec![0; 35], COMMAND_INT_CRC_EXTRA);
    let mut payload = vec![0; 33];
    payload[28..30].copy_from_slice(&42650u16.to_le_bytes());
    let flash = carrier(COMMAND_LONG_ID, payload, COMMAND_LONG_CRC_EXTRA);
    let mut bad = allowed; bad.ethernet_frame[52] ^= 1;
    let mut bad_carrier = allowed; bad_carrier.ethernet_frame[UDP_SOURCE_PORT_OFFSET] = 0;
    for mode in [Normal, Recovery] {
      for before in [0, 3, 4, 5, 18, 19, 20] {
        for lane in 0..4 {
          crate::seL4_MAVLinkFirewall_MAVLinkFirewall_initialize();
          assert_eq!(get_rejected_count(), 0);
          assert!(!get_error_status());
          assert_eq!(outputs(), [None; 4]);
          put_rejected_count(before);
          let mut inputs = [Some(flash), Some(bad), Some(bad_carrier), Some(allowed)];
          inputs.rotate_right(lane);
          dispatch(mode, inputs);
          assert_eq!(get_rejected_count(), (before + 3).min(20));
          assert_eq!(get_error_status(), before + 3 >= 5);
          for (i, output) in outputs().iter().enumerate() {
            assert_eq!(*output, if mode == Normal && inputs[i] == Some(allowed) { Some(allowed) } else { None });
          }
          let logs = crate::logging::take_records();
          assert!(logs.iter().any(|(_, m)| m.contains("firmware-flash command denied")));
          assert!(logs.iter().any(|(_, m)| m.contains("invalid Ethernet/IPv4/UDP carrier")));
          assert!(!logs.iter().any(|(_, m)| m.contains("suppressed") || m.contains("supressed")));
          dispatch(mode, [None; 4]);
          assert_eq!(get_rejected_count(), (before + 3).min(20));
          assert_eq!(get_error_status(), before + 3 >= 5);
          assert_eq!(outputs(), [None; 4]);
        }
      }
    }
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_initialize();
    crate::logging::take_records();
    dispatch(Recovery, [Some(allowed); 4]);
    assert_eq!(get_rejected_count(), 0);
    assert_eq!(outputs(), [None; 4]);
    assert!(crate::logging::take_records().is_empty());
  }

  #[test]
  #[serial]
  fn secure_operation_zero_extension_and_upper_bytes() {
    for length in [1, 4, 5, 6, 7, 8, 12, 232] {
      for operation in [0u32, 6, 7, 8, 0x107, 0x10007, 0x1000007] {
        let mut payload = vec![0; length];
        for (i, byte) in operation.to_le_bytes().iter().enumerate() {
          if 4 + i < length { payload[4 + i] = *byte; }
        }
        let decoded = payload.iter().skip(4).take(4).enumerate()
          .fold(0u32, |acc, (i, b)| acc | ((*b as u32) << (8 * i)));
        let msg = carrier(SECURE_COMMAND_ID, payload, SECURE_COMMAND_CRC_EXTRA);
        crate::seL4_MAVLinkFirewall_MAVLinkFirewall_initialize();
        dispatch(Normal, [Some(msg); 4]);
        assert_eq!(outputs(), if decoded == 7 { [None; 4] } else { [Some(msg); 4] });
        assert_eq!(get_rejected_count(), if decoded == 7 { 4 } else { 0 });
      }
    }
  }

  #[test]
  #[serial]
  fn production_monitor_reports_at_d2_once_and_resets_on_boot() {
    let mut bad = carrier(COMMAND_INT_ID, vec![0; 35], COMMAND_INT_CRC_EXTRA);
    bad.ethernet_frame[52] ^= 1;
    for trigger in [1usize, 2, 4] {
      for recovery_delay in [Some(1usize), Some(2), Some(3), None] {
        for _boot in 0..2 {
          crate::seL4_MAVLinkFirewall_MAVLinkFirewall_initialize();
          crate::logging::take_records();
          let mut errors = 0;
          for tick in 0..trigger + 10 {
            let inputs = if tick == trigger - 1 { [Some(bad); 4] }
              else if tick == trigger { [Some(bad), None, None, None] }
              else { [None; 4] };
            let mode = if recovery_delay.is_some_and(|d| tick >= trigger + d) { Recovery } else { Normal };
            dispatch(mode, inputs);
            let logs = crate::logging::take_records();
            let timeout_count = logs.iter().filter(|(level, message)|
              *level == log::Level::Error && message == crate::logging::TIMEOUT_MESSAGE).count();
            let missed = recovery_delay.is_none_or(|d| d > 2);
            assert_eq!(timeout_count, usize::from(missed && tick == trigger + 2),
              "trigger={trigger}, recovery={recovery_delay:?}, tick={tick}, logs={logs:?}");
            assert!(!logs.iter().any(|(_, message)| message.contains("is currently")));
            errors += timeout_count;
          }
          assert_eq!(errors, usize::from(recovery_delay.is_none_or(|d| d > 2)));
        }
      }
    }
  }

  #[test]
  #[serial]
  fn carrier_boundaries_and_destination_octets() {
    let valid = carrier(COMMAND_INT_ID, vec![0; 35], COMMAND_INT_CRC_EXTRA);
    let mut fixtures = Vec::new();
    for octet in 0..6 {
      let mut msg = valid; msg.ethernet_frame[..6].fill(0); msg.ethernet_frame[octet] = 1;
      fixtures.push((msg, true));
    }
    let mut msg = valid; msg.ethernet_frame[..6].fill(0); fixtures.push((msg, false));
    for (at, value) in [(ETHERTYPE_OFFSET, 0x0806u16), (UDP_SOURCE_PORT_OFFSET, 68),
      (UDP_DESTINATION_PORT_OFFSET, 68), (IPV4_TOTAL_LENGTH_OFFSET, 19),
      (IPV4_TOTAL_LENGTH_OFFSET, 1587), (IPV4_TOTAL_LENGTH_OFFSET, 9000), (IPV4_TOTAL_LENGTH_OFFSET, 9001),
      (IPV4_TOTAL_LENGTH_OFFSET, u16::MAX), (UDP_LENGTH_OFFSET, 7), (UDP_LENGTH_OFFSET, 9)] {
      let mut msg = valid; msg.ethernet_frame[at..at+2].copy_from_slice(&value.to_be_bytes());
      fixtures.push((msg, false));
    }
    for at in [IPV4_VERSION_IHL_OFFSET, IPV4_PROTOCOL_OFFSET] {
      let mut msg = valid; msg.ethernet_frame[at] = 0; fixtures.push((msg, false));
    }
    let mut msg = valid; msg.payload_offset = 41; fixtures.push((msg, false));
    let mut msg = valid; msg.payload_length += 1; fixtures.push((msg, false));
    for (msg, allowed) in fixtures {
      for mode in [Normal, Recovery] {
        crate::seL4_MAVLinkFirewall_MAVLinkFirewall_initialize();
        dispatch(mode, [Some(msg); 4]);
        assert_eq!(outputs(), if allowed && mode == Normal { [Some(msg); 4] } else { [None; 4] });
        assert_eq!(get_rejected_count(), if allowed { 0 } else { 4 });
      }
    }
  }

  #[test]
  #[serial]
  fn timeout_does_not_change_routing_or_count() {
    let allowed = carrier(COMMAND_INT_ID, vec![0; 35], COMMAND_INT_CRC_EXTRA);
    let mut bad = allowed; bad.ethernet_frame[52] ^= 1;
    crate::seL4_MAVLinkFirewall_MAVLinkFirewall_initialize();
    dispatch(Normal, [Some(bad); 4]);
    dispatch(Normal, [Some(bad), None, None, None]); // D0
    crate::logging::take_records();
    for tick in 1..=4 {
      dispatch(Normal, [Some(allowed); 4]);
      assert_eq!(outputs(), [Some(allowed); 4]);
      assert_eq!(get_rejected_count(), 5);
      assert!(get_error_status());
      assert_eq!(crate::logging::take_records().iter().filter(|(level, _)| *level == log::Level::Error).count(), usize::from(tick == 2));
    }
    dispatch(Recovery, [Some(allowed); 4]);
    assert_eq!(outputs(), [None; 4]);
    assert_eq!(get_rejected_count(), 5);
    assert!(get_error_status());
  }

}

mod GUMBOX_tests {
  use super::*;
  use serial_test::serial;
  use proptest::prelude::*;

  use crate::test::util::*;
  use crate::testInitializeCB_macro;
  use crate::testComputeCBwGSV_macro;
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
          0, if input.is_some() && output.is_none() { 1 } else { 0 },
          inputs[0], inputs[1], inputs[2], inputs[3], Normal,
          outputs[0], outputs[1], outputs[2], outputs[3], false));
      }
    }
  }

  #[test]
  fn oracle_rejects_wrong_state_status_and_each_lane_output() {
    let allowed = super::tests::carrier(COMMAND_INT_ID, vec![0; 35], COMMAND_INT_CRC_EXTRA);
    let mut payload = vec![0; 5]; payload[4] = 7;
    let flash = super::tests::carrier(SECURE_COMMAND_ID, payload, SECURE_COMMAND_CRC_EXTRA);
    let mut invalid = allowed; invalid.ethernet_frame[52] ^= 1;
    let mut bad_carrier = allowed; bad_carrier.payload_offset = 41;
    let mut altered = allowed; altered.ethernet_frame[6] ^= 1;
    for lane in 0..4 {
      for mode in [Normal, Recovery] {
        for input in [None, Some(allowed), Some(flash), Some(invalid), Some(bad_carrier)] {
          let mut inputs = [None; 4]; inputs[lane] = input;
          for before in [0u16, 4, 5, 19, 20] {
            let expected_count = (before + u16::from(input.is_some() && input != Some(allowed))).min(20);
            let expected_output = if mode == Normal && input == Some(allowed) { input } else { None };
            for output in [None, Some(allowed), Some(altered), Some(flash), Some(invalid), Some(bad_carrier)] {
              let mut outputs = [None; 4]; outputs[lane] = output;
              for after in [expected_count, 0, 5, 20, 21] {
                for status in [false, true] {
                  let expected = after == expected_count && status == (after >= 5) && output == expected_output;
                  assert_eq!(oracle::compute_CEP_Post(before, after,
                    inputs[0], inputs[1], inputs[2], inputs[3], mode,
                    outputs[0], outputs[1], outputs[2], outputs[3], status), expected);
                }
              }
            }
          }
        }
      }
    }
    assert!(!oracle::compute_CEP_Pre(21, None, None, None, None, Normal));
    for count in [0, 1] {
      for status in [false, true] {
        for lane in 0..4 {
          for output in [None, Some(allowed), Some(flash)] {
            let mut outputs = [None; 4]; outputs[lane] = output;
            assert_eq!(oracle::initialize_IEP_Post(count, outputs[0], outputs[1], outputs[2], outputs[3], status),
              count == 0 && !status && output.is_none());
          }
        }
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

  testComputeCBwGSV_macro! {
    prop_testComputeCB_macro, // test name
    config: ProptestConfig { // proptest configuration, built by overriding fields from default config
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    },
    In_rejected_count: 0u16..=20,
    // strategies for generating each component input
    api_EthernetFramesIn0: generators::option_strategy_default(generators::open_platform_Data_Model_MAVLinkUDPMessage_Impl_strategy_default()),
    api_EthernetFramesIn1: generators::option_strategy_default(generators::open_platform_Data_Model_MAVLinkUDPMessage_Impl_strategy_default()),
    api_EthernetFramesIn2: generators::option_strategy_default(generators::open_platform_Data_Model_MAVLinkUDPMessage_Impl_strategy_default()),
    api_EthernetFramesIn3: generators::option_strategy_default(generators::open_platform_Data_Model_MAVLinkUDPMessage_Impl_strategy_default()),
    api_current_mode: prop_oneof![Just(Normal), Just(Recovery)]
  }
}
