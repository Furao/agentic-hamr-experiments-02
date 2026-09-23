// This file will not be overwritten if HAMR codegen is rerun

mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use data::*;

  #[test]
  #[serial]
  fn test_initialization() {
    crate::seL4_TxFirewall_TxFirewall_initialize();
  }

  #[test]
  #[serial]
  fn test_compute() {
    crate::seL4_TxFirewall_TxFirewall_initialize();
    crate::seL4_TxFirewall_TxFirewall_timeTriggered();
  }
}

mod GUMBOX_tests {
  use serial_test::serial;
  use proptest::prelude::*;

  use crate::test::util::*;
  use crate::testInitializeCB_macro;
  use crate::testComputeCB_macro;

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
    api_EthernetFramesTxIn0: generators::option_strategy_default(generators::open_platform_Data_Model_RawEthernetMessage_strategy_default()),
    api_EthernetFramesTxIn1: generators::option_strategy_default(generators::open_platform_Data_Model_RawEthernetMessage_strategy_default()),
    api_EthernetFramesTxIn2: generators::option_strategy_default(generators::open_platform_Data_Model_RawEthernetMessage_strategy_default()),
    api_EthernetFramesTxIn3: generators::option_strategy_default(generators::open_platform_Data_Model_RawEthernetMessage_strategy_default())
  }
}

mod regression_tests {
  use data::open_platform_Data_Model::{RawEthernetMessage as Frame, SizedEthernetMessage_Impl as Output};
  use crate::test::util::test_apis::*;
  use crate::bridge::seL4_TxFirewall_TxFirewall_GUMBOX as oracle;

  fn ethernet(ether_type: u16) -> Frame {
    let mut frame = [0; 1600];
    frame[0] = 1;
    frame[12..14].copy_from_slice(&ether_type.to_be_bytes());
    frame
  }

  fn arp(operation: u16, protocol: u16) -> Frame {
    let mut frame = ethernet(0x0806);
    frame[14..16].copy_from_slice(&1u16.to_be_bytes());
    frame[16..18].copy_from_slice(&protocol.to_be_bytes());
    frame[20..22].copy_from_slice(&operation.to_be_bytes());
    frame
  }

  fn ipv4(length: u16, protocol: u8) -> Frame {
    let mut frame = ethernet(0x0800);
    frame[14] = 0x45;
    frame[16..18].copy_from_slice(&length.to_be_bytes());
    frame[23] = protocol;
    frame[34..36].copy_from_slice(&14550u16.to_be_bytes());
    frame[36..38].copy_from_slice(&68u16.to_be_bytes());
    frame
  }

  fn fixtures() -> Vec<(Frame, Option<u16>)> {
    let mut cases = vec![(ethernet(0x86dd), None), (ethernet(0), None), ([0; 1600], None)];
    for operation in [1, 2] {
      for protocol in [0x0800, 0x86dd] { cases.push((arp(operation, protocol), Some(64))); }
    }
    cases.push((arp(3, 0x0800), None));
    cases.push((arp(1, 0), None));
    let mut bad = arp(1, 0x0800); bad[15] = 2; cases.push((bad, None));
    for protocol in [0, 1, 2, 6, 17, 43, 44, 58, 59, 60] {
      for length in [0u16, 20, 28, 1585, 1586] {
        cases.push((ipv4(length, protocol), Some(length + 14)));
      }
      for length in [1587u16, 9000, 9001, u16::MAX] {
        cases.push((ipv4(length, protocol), None));
      }
    }
    cases.push((ipv4(9001, 17), None));
    cases.push((ipv4(u16::MAX, 17), None));
    cases.push((ipv4(28, 255), None));
    let mut bad = ipv4(28, 17); bad[14] = 0x46; cases.push((bad, None));
    let mut bad = ipv4(28, 17); bad[..6].fill(0); cases.push((bad, None));
    cases
  }

  fn outputs() -> [Option<Output>; 4] {
    [get_EthernetFramesTxOut0(), get_EthernetFramesTxOut1(),
     get_EthernetFramesTxOut2(), get_EthernetFramesTxOut3()]
  }

  #[test]
  #[serial_test::serial]
  fn all_lanes_preserve_frames_and_sizes() {
    for (frame, size) in fixtures() {
      for lane in 0..4 {
        crate::seL4_TxFirewall_TxFirewall_initialize();
        assert_eq!(outputs(), [None; 4]);
        let mut inputs = [None; 4]; inputs[lane] = Some(frame);
        put_concrete_inputs(inputs[0], inputs[1], inputs[2], inputs[3]);
        crate::seL4_TxFirewall_TxFirewall_timeTriggered();
        let mut expected = [None; 4];
        expected[lane] = size.map(|sz| Output { sz, amessage: frame });
        assert_eq!(outputs(), expected);
        assert!(outputs().iter().flatten().all(|output| usize::from(output.sz) <= output.amessage.len()));
        assert!(oracle::compute_CEP_Post(inputs[0], inputs[1], inputs[2], inputs[3],
          expected[0], expected[1], expected[2], expected[3]));
      }
    }
    crate::seL4_TxFirewall_TxFirewall_notify(99);
  }

  #[test]
  #[serial_test::serial]
  fn mixed_lanes_and_no_input_dispatch() {
    for shift in 0..4 {
      crate::seL4_TxFirewall_TxFirewall_initialize();
      let a = arp(1, 0x0800); let ip = ipv4(28, 17);
      let mut inputs = [Some(a), Some(ip), Some(ethernet(0x86dd)), None];
      let mut expected = [Some(Output { sz: 64, amessage: a }), Some(Output { sz: 42, amessage: ip }), None, None];
      inputs.rotate_right(shift); expected.rotate_right(shift);
      put_concrete_inputs(inputs[0], inputs[1], inputs[2], inputs[3]);
      crate::seL4_TxFirewall_TxFirewall_timeTriggered();
      assert_eq!(outputs(), expected);
    }
    crate::seL4_TxFirewall_TxFirewall_initialize();
    put_concrete_inputs(None, None, None, None);
    crate::seL4_TxFirewall_TxFirewall_timeTriggered();
    assert_eq!(outputs(), [None; 4]);
  }

  #[test]
  #[serial_test::serial]
  fn output_size_invariants_reject_oversized_values_on_every_lane() {
    for lane in 0..4 {
      for size in [0, 64, 1599, 1600, 1601, 9014, u16::MAX] {
        let mut outputs = [None; 4];
        outputs[lane] = Some(Output { sz: size, amessage: ipv4(1586, 17) });
        // Probe the generated integration guards independently of compute routing.
        assert_eq!(oracle::initialize_IEP_Post(outputs[0], outputs[1], outputs[2], outputs[3]), size <= 1600);
      }
    }
    assert!(oracle::initialize_IEP_Post(None, None, None, None));
  }

  #[test]
  fn oracle_rejects_injection_wrong_size_and_changed_bytes() {
    let mut cases: Vec<_> = fixtures().into_iter().map(|(f, size)| (Some(f), size)).collect();
    cases.push((None, None));
    for lane in 0..4 {
      for (input, size) in &cases {
        let mut inputs = [None; 4]; inputs[lane] = *input;
        let expected = size.map(|sz| Output { sz, amessage: input.unwrap() });
        let mut changed = input.unwrap_or([0; 1600]); changed[6] ^= 1;
        let candidates = [None, expected,
          Some(Output { sz: size.unwrap_or(64).wrapping_add(1), amessage: input.unwrap_or([0; 1600]) }),
          Some(Output { sz: size.unwrap_or(64), amessage: changed })];
        for candidate in candidates {
          let mut outputs = [None; 4]; outputs[lane] = candidate;
          assert_eq!(oracle::compute_CEP_Post(inputs[0], inputs[1], inputs[2], inputs[3],
            outputs[0], outputs[1], outputs[2], outputs[3]), candidate == expected);
        }
      }
    }
  }
}
