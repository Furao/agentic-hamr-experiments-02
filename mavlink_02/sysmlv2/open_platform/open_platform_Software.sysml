package open_platform_Software {
    private import HAMR::*;
    private import open_platform_Data_Model::*;

    part def LowLevelEthernetDriver :> Thread {
        attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
        attribute :>> Period = 1000 [ms]; 
        attribute :>> Compute_Execution_Time = 300[ms] .. 300[ms];
        attribute :>> Implementation_Language = Rust;
        attribute :>> Stack_Size = 1048576[byte];

        port EthernetFramesRx0: EventDataPort {out :>> type : RawEthernetMessage; }
        port EthernetFramesRx1: EventDataPort {out :>> type : RawEthernetMessage; }
        port EthernetFramesRx2: EventDataPort {out :>> type : RawEthernetMessage; } 
        port EthernetFramesRx3: EventDataPort {out :>> type : RawEthernetMessage; }
        port EthernetFramesTx0: EventDataPort {in :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTx1: EventDataPort {in :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTx2: EventDataPort {in :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTx3: EventDataPort {in :>> type : SizedEthernetMessage_Impl; }
    }

    part def RxFirewall :> Thread {
        port current_mode : DataPort { in :>> type : OperatingMode; }
        port EthernetFramesRxIn0: EventDataPort {in :>> type : RawEthernetMessage; }
        port EthernetFramesRxIn1: EventDataPort {in :>> type : RawEthernetMessage; }
        port EthernetFramesRxIn2: EventDataPort {in :>> type : RawEthernetMessage; } 
        port EthernetFramesRxIn3: EventDataPort {in :>> type : RawEthernetMessage; }
        port EthernetFramesRxOut0 : EventDataPort { out :>> type : RawEthernetMessage; }
        port EthernetFramesRxOut1 : EventDataPort { out :>> type : RawEthernetMessage; }
        port EthernetFramesRxOut2 : EventDataPort { out :>> type : RawEthernetMessage; }
        port EthernetFramesRxOut3 : EventDataPort { out :>> type : RawEthernetMessage; }
        port MAVLinkFramesRxOut0 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port MAVLinkFramesRxOut1 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port MAVLinkFramesRxOut2 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port MAVLinkFramesRxOut3 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }

        attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
        attribute :>> Period = 1000[ms]; 
        attribute :>> Compute_Execution_Time =  300[ms] .. 300[ms];
        attribute :>> Stack_Size = 1048576[byte];
        attribute :>> Implementation_Language = Rust;			

        language "GUMBO" /*{
        integration
            guarantee hlr_05_13_direct_out0:
                GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxOut0);
            guarantee hlr_05_13_direct_out1:
                GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxOut1);
            guarantee hlr_05_13_direct_out2:
                GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxOut2);
            guarantee hlr_05_13_direct_out3:
                GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxOut3);
            guarantee hlr_18_mavlink_out0:
                GumboLib::GUMBO__Library::valid_mavlink_carrier(MAVLinkFramesRxOut0);
            guarantee hlr_18_mavlink_out1:
                GumboLib::GUMBO__Library::valid_mavlink_carrier(MAVLinkFramesRxOut1);
            guarantee hlr_18_mavlink_out2:
                GumboLib::GUMBO__Library::valid_mavlink_carrier(MAVLinkFramesRxOut2);
            guarantee hlr_18_mavlink_out3:
                GumboLib::GUMBO__Library::valid_mavlink_carrier(MAVLinkFramesRxOut3);

        // CR-02 HLR-17 / LLR-1: initialization fabricates no frame events.
        initialize
            guarantee hlr_17_initialize_lane0:
                NoSend(EthernetFramesRxOut0) and NoSend(MAVLinkFramesRxOut0);
            guarantee hlr_17_initialize_lane1:
                NoSend(EthernetFramesRxOut1) and NoSend(MAVLinkFramesRxOut1);
            guarantee hlr_17_initialize_lane2:
                NoSend(EthernetFramesRxOut2) and NoSend(MAVLinkFramesRxOut2);
            guarantee hlr_17_initialize_lane3:
                NoSend(EthernetFramesRxOut3) and NoSend(MAVLinkFramesRxOut3);

        compute
            // CR-02 HLR-29 / LLR-10: frozen Recovery mode suppresses every lane.
            guarantee hlr_29_llr_10_rx0_recovery:
                (current_mode == open_platform_Data_Model::OperatingMode.Recovery) implies
                (NoSend(EthernetFramesRxOut0) and NoSend(MAVLinkFramesRxOut0));
            guarantee hlr_29_llr_10_rx1_recovery:
                (current_mode == open_platform_Data_Model::OperatingMode.Recovery) implies
                (NoSend(EthernetFramesRxOut1) and NoSend(MAVLinkFramesRxOut1));
            guarantee hlr_29_llr_10_rx2_recovery:
                (current_mode == open_platform_Data_Model::OperatingMode.Recovery) implies
                (NoSend(EthernetFramesRxOut2) and NoSend(MAVLinkFramesRxOut2));
            guarantee hlr_29_llr_10_rx3_recovery:
                (current_mode == open_platform_Data_Model::OperatingMode.Recovery) implies
                (NoSend(EthernetFramesRxOut3) and NoSend(MAVLinkFramesRxOut3));

            guarantee hlr_05_13_rx0_direct:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesRxIn0) and GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxIn0)) implies
                (HasEvent(EthernetFramesRxOut0) and EthernetFramesRxOut0 == EthernetFramesRxIn0 and NoSend(MAVLinkFramesRxOut0));
            guarantee hlr_18_rx0_mavlink:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesRxIn0) and GumboLib::GUMBO__Library::valid_ardupilot_udp(EthernetFramesRxIn0)) implies
                (NoSend(EthernetFramesRxOut0) and HasEvent(MAVLinkFramesRxOut0) and
                 MAVLinkFramesRxOut0.ethernet_frame == EthernetFramesRxIn0 and
                 MAVLinkFramesRxOut0.payload_offset == GumboLib::GUMBO__Library::udp_payload_offset() and
                 MAVLinkFramesRxOut0.payload_length == GumboLib::GUMBO__Library::udp_payload_length(EthernetFramesRxIn0));
            guarantee hlr_15_llr_2_rx0_drop:
                (HasEvent(EthernetFramesRxIn0) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn0)) implies
                (NoSend(EthernetFramesRxOut0) and NoSend(MAVLinkFramesRxOut0));
            guarantee hlr_17_rx0_no_input:
                (not HasEvent(EthernetFramesRxIn0)) implies (NoSend(EthernetFramesRxOut0) and NoSend(MAVLinkFramesRxOut0));

            guarantee hlr_05_13_rx1_direct:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesRxIn1) and GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxIn1)) implies
                (HasEvent(EthernetFramesRxOut1) and EthernetFramesRxOut1 == EthernetFramesRxIn1 and NoSend(MAVLinkFramesRxOut1));
            guarantee hlr_18_rx1_mavlink:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesRxIn1) and GumboLib::GUMBO__Library::valid_ardupilot_udp(EthernetFramesRxIn1)) implies
                (NoSend(EthernetFramesRxOut1) and HasEvent(MAVLinkFramesRxOut1) and
                 MAVLinkFramesRxOut1.ethernet_frame == EthernetFramesRxIn1 and
                 MAVLinkFramesRxOut1.payload_offset == GumboLib::GUMBO__Library::udp_payload_offset() and
                 MAVLinkFramesRxOut1.payload_length == GumboLib::GUMBO__Library::udp_payload_length(EthernetFramesRxIn1));
            guarantee hlr_15_llr_2_rx1_drop:
                (HasEvent(EthernetFramesRxIn1) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn1)) implies
                (NoSend(EthernetFramesRxOut1) and NoSend(MAVLinkFramesRxOut1));
            guarantee hlr_17_rx1_no_input:
                (not HasEvent(EthernetFramesRxIn1)) implies (NoSend(EthernetFramesRxOut1) and NoSend(MAVLinkFramesRxOut1));

            guarantee hlr_05_13_rx2_direct:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesRxIn2) and GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxIn2)) implies
                (HasEvent(EthernetFramesRxOut2) and EthernetFramesRxOut2 == EthernetFramesRxIn2 and NoSend(MAVLinkFramesRxOut2));
            guarantee hlr_18_rx2_mavlink:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesRxIn2) and GumboLib::GUMBO__Library::valid_ardupilot_udp(EthernetFramesRxIn2)) implies
                (NoSend(EthernetFramesRxOut2) and HasEvent(MAVLinkFramesRxOut2) and
                 MAVLinkFramesRxOut2.ethernet_frame == EthernetFramesRxIn2 and
                 MAVLinkFramesRxOut2.payload_offset == GumboLib::GUMBO__Library::udp_payload_offset() and
                 MAVLinkFramesRxOut2.payload_length == GumboLib::GUMBO__Library::udp_payload_length(EthernetFramesRxIn2));
            guarantee hlr_15_llr_2_rx2_drop:
                (HasEvent(EthernetFramesRxIn2) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn2)) implies
                (NoSend(EthernetFramesRxOut2) and NoSend(MAVLinkFramesRxOut2));
            guarantee hlr_17_rx2_no_input:
                (not HasEvent(EthernetFramesRxIn2)) implies (NoSend(EthernetFramesRxOut2) and NoSend(MAVLinkFramesRxOut2));

            guarantee hlr_05_13_rx3_direct:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesRxIn3) and GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxIn3)) implies
                (HasEvent(EthernetFramesRxOut3) and EthernetFramesRxOut3 == EthernetFramesRxIn3 and NoSend(MAVLinkFramesRxOut3));
            guarantee hlr_18_rx3_mavlink:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesRxIn3) and GumboLib::GUMBO__Library::valid_ardupilot_udp(EthernetFramesRxIn3)) implies
                (NoSend(EthernetFramesRxOut3) and HasEvent(MAVLinkFramesRxOut3) and
                 MAVLinkFramesRxOut3.ethernet_frame == EthernetFramesRxIn3 and
                 MAVLinkFramesRxOut3.payload_offset == GumboLib::GUMBO__Library::udp_payload_offset() and
                 MAVLinkFramesRxOut3.payload_length == GumboLib::GUMBO__Library::udp_payload_length(EthernetFramesRxIn3));
            guarantee hlr_15_llr_2_rx3_drop:
                (HasEvent(EthernetFramesRxIn3) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn3)) implies
                (NoSend(EthernetFramesRxOut3) and NoSend(MAVLinkFramesRxOut3));
            guarantee hlr_17_rx3_no_input:
                (not HasEvent(EthernetFramesRxIn3)) implies (NoSend(EthernetFramesRxOut3) and NoSend(MAVLinkFramesRxOut3));
        }*/
    }

    part def MAVLinkFirewall :> Thread {
        // CR-02 HLR-25/26/28/30; LLR-11/12/15: monitor observes these ports.
        port current_mode : DataPort { in :>> type : OperatingMode; }
        port error_status : DataPort { out :>> type : Base_Types::Boolean; }
        port EthernetFramesIn0 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesIn1 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesIn2 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesIn3 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesOut0 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesOut1 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesOut2 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesOut3 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }

        attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
        attribute :>> Period = 1000[ms];
        attribute :>> Compute_Execution_Time = 300[ms] .. 300[ms];
        attribute :>> Stack_Size = 1048576[byte];
        attribute :>> Implementation_Language = Rust;

        language "GUMBO" /*{
        state
            rejected_count: Base_Types::Unsigned_16;

        functions
            // LLR-6/7: refined by the existing component-owned verified parser hook.
            @spec def mavlink_frame_valid(msg: MAVLinkUDPMessage_Impl): Base_Types::Boolean;

            // HLR-19/32, LLR-8: COMMAND_INT/LONG command 42650 or
            // SECURE_COMMAND operation 7; bounded payload reads in the developer hook.
            @spec def mavlink_firmware_flash_command(msg: MAVLinkUDPMessage_Impl): Base_Types::Boolean;

            def mavlink_allowed(msg: MAVLinkUDPMessage_Impl): Base_Types::Boolean :=
                GumboLib::GUMBO__Library::valid_mavlink_carrier(msg) and
                mavlink_frame_valid(msg) and
                not mavlink_firmware_flash_command(msg);

            def rejection_limit(): Base_Types::Unsigned_16 := 20[u16];

            def error_threshold(): Base_Types::Unsigned_16 := 5[u16];

            def rejection_total(lane0: Base_Types::Boolean, lane1: Base_Types::Boolean,
                                lane2: Base_Types::Boolean, lane3: Base_Types::Boolean): Base_Types::Unsigned_16 :=
                (if lane0 ? 1[u16] else 0[u16]) + (if lane1 ? 1[u16] else 0[u16]) +
                (if lane2 ? 1[u16] else 0[u16]) + (if lane3 ? 1[u16] else 0[u16]);

            def saturated_rejection_count(count: Base_Types::Unsigned_16,
                                          increment: Base_Types::Unsigned_16): Base_Types::Unsigned_16 :=
                if count >= rejection_limit() ? rejection_limit()
                else if increment >= rejection_limit() - count ? rejection_limit()
                else count + increment;

        integration
            guarantee hlr_22_llr_9_output0_allowed:
                mavlink_allowed(EthernetFramesOut0);
            guarantee hlr_22_llr_9_output1_allowed:
                mavlink_allowed(EthernetFramesOut1);
            guarantee hlr_22_llr_9_output2_allowed:
                mavlink_allowed(EthernetFramesOut2);
            guarantee hlr_22_llr_9_output3_allowed:
                mavlink_allowed(EthernetFramesOut3);

        initialize
            modifies rejected_count;
            guarantee hlr_31_llr_11_initial_count:
                rejected_count == 0[u16];
            guarantee hlr_25_llr_11_initial_error_status:
                error_status == false;
            guarantee hlr_21_initialize_lane0:
                NoSend(EthernetFramesOut0);
            guarantee hlr_21_initialize_lane1:
                NoSend(EthernetFramesOut1);
            guarantee hlr_21_initialize_lane2:
                NoSend(EthernetFramesOut2);
            guarantee hlr_21_initialize_lane3:
                NoSend(EthernetFramesOut3);

        compute
            modifies rejected_count;
            // Inductive state bound: established by initialization and preserved below.
            assume hlr_31_count_pre_bound:
                rejected_count <= rejection_limit();
            guarantee hlr_31_count_post_bound:
                rejected_count <= rejection_limit();
            guarantee hlr_31_llr_13_count_monotonic:
                rejected_count >= In(rejected_count);
            // Each present malformed/blacklisted carrier contributes once, in either mode.
            guarantee hlr_31_llr_13_count_update:
                rejected_count == saturated_rejection_count(In(rejected_count), rejection_total(
                    HasEvent(EthernetFramesIn0) and not mavlink_allowed(EthernetFramesIn0),
                    HasEvent(EthernetFramesIn1) and not mavlink_allowed(EthernetFramesIn1),
                    HasEvent(EthernetFramesIn2) and not mavlink_allowed(EthernetFramesIn2),
                    HasEvent(EthernetFramesIn3) and not mavlink_allowed(EthernetFramesIn3)));
            // Final value; LLR-12's last-output-update ordering is checked in CompDev.
            guarantee hlr_25_llr_12_post_count_error_status:
                error_status == (rejected_count >= error_threshold());

            guarantee hlr_22_llr_3_lane0_allow:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesIn0) and mavlink_allowed(EthernetFramesIn0)) implies
                (HasEvent(EthernetFramesOut0) and EthernetFramesOut0 == EthernetFramesIn0);
            guarantee hlr_19_32_lane0_deny_flash:
                (HasEvent(EthernetFramesIn0) and GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn0) and
                 mavlink_frame_valid(EthernetFramesIn0) and
                 mavlink_firmware_flash_command(EthernetFramesIn0)) implies NoSend(EthernetFramesOut0);
            guarantee hlr_20_lane0_invalid:
                (HasEvent(EthernetFramesIn0) and not (GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn0) and
                 mavlink_frame_valid(EthernetFramesIn0))) implies NoSend(EthernetFramesOut0);
            guarantee hlr_21_lane0_no_input:
                (not HasEvent(EthernetFramesIn0)) implies NoSend(EthernetFramesOut0);
            guarantee hlr_28_llr_10_lane0_recovery:
                (current_mode == open_platform_Data_Model::OperatingMode.Recovery) implies NoSend(EthernetFramesOut0);

            guarantee hlr_22_llr_3_lane1_allow:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesIn1) and mavlink_allowed(EthernetFramesIn1)) implies
                (HasEvent(EthernetFramesOut1) and EthernetFramesOut1 == EthernetFramesIn1);
            guarantee hlr_19_32_lane1_deny_flash:
                (HasEvent(EthernetFramesIn1) and GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn1) and
                 mavlink_frame_valid(EthernetFramesIn1) and
                 mavlink_firmware_flash_command(EthernetFramesIn1)) implies NoSend(EthernetFramesOut1);
            guarantee hlr_20_lane1_invalid:
                (HasEvent(EthernetFramesIn1) and not (GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn1) and
                 mavlink_frame_valid(EthernetFramesIn1))) implies NoSend(EthernetFramesOut1);
            guarantee hlr_21_lane1_no_input:
                (not HasEvent(EthernetFramesIn1)) implies NoSend(EthernetFramesOut1);
            guarantee hlr_28_llr_10_lane1_recovery:
                (current_mode == open_platform_Data_Model::OperatingMode.Recovery) implies NoSend(EthernetFramesOut1);

            guarantee hlr_22_llr_3_lane2_allow:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesIn2) and mavlink_allowed(EthernetFramesIn2)) implies
                (HasEvent(EthernetFramesOut2) and EthernetFramesOut2 == EthernetFramesIn2);
            guarantee hlr_19_32_lane2_deny_flash:
                (HasEvent(EthernetFramesIn2) and GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn2) and
                 mavlink_frame_valid(EthernetFramesIn2) and
                 mavlink_firmware_flash_command(EthernetFramesIn2)) implies NoSend(EthernetFramesOut2);
            guarantee hlr_20_lane2_invalid:
                (HasEvent(EthernetFramesIn2) and not (GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn2) and
                 mavlink_frame_valid(EthernetFramesIn2))) implies NoSend(EthernetFramesOut2);
            guarantee hlr_21_lane2_no_input:
                (not HasEvent(EthernetFramesIn2)) implies NoSend(EthernetFramesOut2);
            guarantee hlr_28_llr_10_lane2_recovery:
                (current_mode == open_platform_Data_Model::OperatingMode.Recovery) implies NoSend(EthernetFramesOut2);

            guarantee hlr_22_llr_3_lane3_allow:
                (current_mode == open_platform_Data_Model::OperatingMode.Normal and
                 HasEvent(EthernetFramesIn3) and mavlink_allowed(EthernetFramesIn3)) implies
                (HasEvent(EthernetFramesOut3) and EthernetFramesOut3 == EthernetFramesIn3);
            guarantee hlr_19_32_lane3_deny_flash:
                (HasEvent(EthernetFramesIn3) and GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn3) and
                 mavlink_frame_valid(EthernetFramesIn3) and
                 mavlink_firmware_flash_command(EthernetFramesIn3)) implies NoSend(EthernetFramesOut3);
            guarantee hlr_20_lane3_invalid:
                (HasEvent(EthernetFramesIn3) and not (GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn3) and
                 mavlink_frame_valid(EthernetFramesIn3))) implies NoSend(EthernetFramesOut3);
            guarantee hlr_21_lane3_no_input:
                (not HasEvent(EthernetFramesIn3)) implies NoSend(EthernetFramesOut3);
            guarantee hlr_28_llr_10_lane3_recovery:
                (current_mode == open_platform_Data_Model::OperatingMode.Recovery) implies NoSend(EthernetFramesOut3);

        monitor
            // HLR-25/31 and LLR-12/13 make the first true ErrorStatus coincide
            // with crossing the threshold. In(count) is captured before compute;
            // ErrorStatus is final output and Mode is the frozen input snapshot.
            // LLR-15/16: check D1/D2 from D0, without restarting on later true status.
            // One-time D2 logging remains a separate reporting integration obligation.
            guarantee hlr_30_llr_15_16_recovery_deadline:
                (In(rejected_count) < error_threshold() and error_status) implies
                Eventually [1,2] (current_mode == open_platform_Data_Model::OperatingMode.Recovery);
        }*/
    }

    // CR-02 HLR-23/24/26/27; LLR-11/14/20.
    part def ModeManager :> Thread {
        port error_status : DataPort { in :>> type : Base_Types::Boolean; }
        port mode_to_rx : DataPort { out :>> type : OperatingMode; }
        port mode_to_mavlink : DataPort { out :>> type : OperatingMode; }

        attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
        // Align with the existing firewall period; validate visibility in W3.
        attribute :>> Period = 1000[ms];
        attribute :>> Compute_Execution_Time = 100[ms] .. 100[ms];
        attribute :>> Stack_Size = 1048576[byte];
        attribute :>> Implementation_Language = Rust;

        language "GUMBO" /*{
        state
            retained_mode: OperatingMode;

        initialize
            modifies retained_mode;
            guarantee hlr_23_llr_11_initial_mode:
                retained_mode == open_platform_Data_Model::OperatingMode.Normal;
            guarantee hlr_26_llr_11_initial_rx_mode:
                mode_to_rx == retained_mode;
            guarantee hlr_26_llr_11_initial_mavlink_mode:
                mode_to_mavlink == retained_mode;

        compute
            modifies retained_mode;
            // Sampled ErrorStatus is frozen immediately before this dispatch.
            guarantee hlr_27_llr_14_latched_transition:
                retained_mode ==
                    (if error_status or In(retained_mode) == open_platform_Data_Model::OperatingMode.Recovery
                     ? open_platform_Data_Model::OperatingMode.Recovery
                     else open_platform_Data_Model::OperatingMode.Normal);
            // Both consumers receive the resulting mode in this same dispatch.
            guarantee hlr_26_27_llr_14_publish_rx_mode:
                mode_to_rx == retained_mode;
            guarantee hlr_26_27_llr_14_publish_mavlink_mode:
                mode_to_mavlink == retained_mode;
        }*/
    }

    part def TxFirewall :> Thread {
        port EthernetFramesTxIn0 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesTxIn1 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesTxIn2 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesTxIn3 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesTxOut0 : EventDataPort { out :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTxOut1 : EventDataPort { out :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTxOut2 : EventDataPort { out :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTxOut3 : EventDataPort { out :>> type : SizedEthernetMessage_Impl; }
    
        attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
        attribute :>> Period = 1000 [ms];
        attribute :>> Compute_Execution_Time = 300[ms] .. 300[ms];
        attribute :>> Stack_Size = 1048576[byte];
        attribute :>> Implementation_Language = Rust;
        
        language "GUMBO" /*{

        // HLR-7/12 and LLR-4: every emitted frame fits the driver carrier.
        integration
            guarantee hlr_12_llr_4_tx0_size_bound:
                EthernetFramesTxOut0.sz <= 1600[u16];
            guarantee hlr_12_llr_4_tx1_size_bound:
                EthernetFramesTxOut1.sz <= 1600[u16];
            guarantee hlr_12_llr_4_tx2_size_bound:
                EthernetFramesTxOut2.sz <= 1600[u16];
            guarantee hlr_12_llr_4_tx3_size_bound:
                EthernetFramesTxOut3.sz <= 1600[u16];

        compute
    
            guarantee hlr_07_tx0_can_send_valid_arp:
            ((HasEvent(EthernetFramesTxIn0) and GumboLib::GUMBO__Library::valid_arp(EthernetFramesTxIn0)) implies
                (HasEvent(EthernetFramesTxOut0) and (EthernetFramesTxIn0 == EthernetFramesTxOut0.amessage) and GumboLib::GUMBO__Library::valid_output_arp_size(EthernetFramesTxOut0)));
            guarantee hlr_12_tx0_can_send_valid_ipv4:
            ((HasEvent(EthernetFramesTxIn0) and GumboLib::GUMBO__Library::valid_ipv4(EthernetFramesTxIn0)) implies
                (HasEvent(EthernetFramesTxOut0) and (EthernetFramesTxIn0 == EthernetFramesTxOut0.amessage) and GumboLib::GUMBO__Library::valid_output_ipv4_size(EthernetFramesTxIn0, EthernetFramesTxOut0) ));
            guarantee hlr_14_tx0_disallow:
            (HasEvent(EthernetFramesTxIn0) and not GumboLib::GUMBO__Library::tx_allow_outbound_frame(EthernetFramesTxIn0)) implies
            (NoSend(EthernetFramesTxOut0));
            guarantee hlr_16_tx0_no_input:
            (HasEvent(EthernetFramesTxIn0) | NoSend(EthernetFramesTxOut0));
    
            guarantee hlr_07_tx1_can_send_valid_arp:
            ((HasEvent(EthernetFramesTxIn1) and GumboLib::GUMBO__Library::valid_arp(EthernetFramesTxIn1)) implies
                (HasEvent(EthernetFramesTxOut1) and (EthernetFramesTxIn1 == EthernetFramesTxOut1.amessage) and GumboLib::GUMBO__Library::valid_output_arp_size(EthernetFramesTxOut1) ));
            guarantee hlr_12_tx1_can_send_valid_ipv4:
            ((HasEvent(EthernetFramesTxIn1) and GumboLib::GUMBO__Library::valid_ipv4(EthernetFramesTxIn1)) implies
                (HasEvent(EthernetFramesTxOut1) and (EthernetFramesTxIn1 == EthernetFramesTxOut1.amessage) and GumboLib::GUMBO__Library::valid_output_ipv4_size(EthernetFramesTxIn1, EthernetFramesTxOut1) ));
            guarantee hlr_14_tx1_disallow:
            (HasEvent(EthernetFramesTxIn1) and not GumboLib::GUMBO__Library::tx_allow_outbound_frame(EthernetFramesTxIn1)) implies
            (NoSend(EthernetFramesTxOut1));
            guarantee hlr_16_tx1_no_input:
            (HasEvent(EthernetFramesTxIn1) | NoSend(EthernetFramesTxOut1));
    
            guarantee hlr_07_tx2_can_send_valid_arp:
            ((HasEvent(EthernetFramesTxIn2) and GumboLib::GUMBO__Library::valid_arp(EthernetFramesTxIn2)) implies
                (HasEvent(EthernetFramesTxOut2) and (EthernetFramesTxIn2 == EthernetFramesTxOut2.amessage) and GumboLib::GUMBO__Library::valid_output_arp_size(EthernetFramesTxOut2) ));
            guarantee hlr_12_tx2_can_send_valid_ipv4:
            ((HasEvent(EthernetFramesTxIn2) and GumboLib::GUMBO__Library::valid_ipv4(EthernetFramesTxIn2)) implies
                (HasEvent(EthernetFramesTxOut2) and (EthernetFramesTxIn2 == EthernetFramesTxOut2.amessage) and GumboLib::GUMBO__Library::valid_output_ipv4_size(EthernetFramesTxIn2, EthernetFramesTxOut2) ));
            guarantee hlr_14_tx2_disallow:
            (HasEvent(EthernetFramesTxIn2) and not GumboLib::GUMBO__Library::tx_allow_outbound_frame(EthernetFramesTxIn2)) implies
                (NoSend(EthernetFramesTxOut2));
            guarantee hlr_16_tx2_no_input:
            (HasEvent(EthernetFramesTxIn2) | NoSend(EthernetFramesTxOut2));
    
            guarantee hlr_07_tx3_can_send_valid_arp:
            ((HasEvent(EthernetFramesTxIn3) and GumboLib::GUMBO__Library::valid_arp(EthernetFramesTxIn3)) implies
                (HasEvent(EthernetFramesTxOut3) and (EthernetFramesTxIn3 == EthernetFramesTxOut3.amessage) and GumboLib::GUMBO__Library::valid_output_arp_size(EthernetFramesTxOut3) ));
            guarantee hlr_12_tx3_can_send_valid_ipv4:
            ((HasEvent(EthernetFramesTxIn3) and GumboLib::GUMBO__Library::valid_ipv4(EthernetFramesTxIn3)) implies
                (HasEvent(EthernetFramesTxOut3) and (EthernetFramesTxIn3 == EthernetFramesTxOut3.amessage) and GumboLib::GUMBO__Library::valid_output_ipv4_size(EthernetFramesTxIn3, EthernetFramesTxOut3) ));
            guarantee hlr_14_tx3_disallow:
            (HasEvent(EthernetFramesTxIn3) and not GumboLib::GUMBO__Library::tx_allow_outbound_frame(EthernetFramesTxIn3)) implies
                (NoSend(EthernetFramesTxOut3));
            guarantee hlr_16_tx3_no_input:
            (HasEvent(EthernetFramesTxIn3) | NoSend(EthernetFramesTxOut3));
                    
        }*/
    }

    part def ArduPilot :> Thread {
        port FirewallRx0 : EventDataPort { in :>> type : RawEthernetMessage; }
        port FirewallRx1 : EventDataPort { in :>> type : RawEthernetMessage; }
        port FirewallRx2 : EventDataPort { in :>> type : RawEthernetMessage; }
        port FirewallRx3 : EventDataPort { in :>> type : RawEthernetMessage; }
        port MAVLinkFirewallRx0 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port MAVLinkFirewallRx1 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port MAVLinkFirewallRx2 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port MAVLinkFirewallRx3 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesTx0 : EventDataPort { out :>> type : RawEthernetMessage; }
        port EthernetFramesTx1 : EventDataPort { out :>> type : RawEthernetMessage; }
        port EthernetFramesTx2 : EventDataPort { out :>> type : RawEthernetMessage; }
        port EthernetFramesTx3 : EventDataPort { out :>> type : RawEthernetMessage; }
    
        attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
        attribute :>> Period = 1000 [ms]; 
        attribute :>> Compute_Execution_Time = 600[ms] .. 600 [ms];
        attribute :>> Stack_Size = 1048576[byte];

        attribute :>> SMC = true;
    }
}
