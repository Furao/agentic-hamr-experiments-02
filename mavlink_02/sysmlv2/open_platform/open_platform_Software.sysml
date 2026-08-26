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

        compute
            guarantee hlr_05_13_rx0_direct:
                (HasEvent(EthernetFramesRxIn0) and GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxIn0)) implies
                (HasEvent(EthernetFramesRxOut0) and EthernetFramesRxOut0 == EthernetFramesRxIn0 and NoSend(MAVLinkFramesRxOut0));
            guarantee hlr_18_rx0_mavlink:
                (HasEvent(EthernetFramesRxIn0) and GumboLib::GUMBO__Library::valid_ardupilot_udp(EthernetFramesRxIn0)) implies
                (NoSend(EthernetFramesRxOut0) and HasEvent(MAVLinkFramesRxOut0) and
                 MAVLinkFramesRxOut0.ethernet_frame == EthernetFramesRxIn0 and
                 MAVLinkFramesRxOut0.payload_offset == GumboLib::GUMBO__Library::udp_payload_offset() and
                 MAVLinkFramesRxOut0.payload_length == GumboLib::GUMBO__Library::udp_payload_length(EthernetFramesRxIn0));
            guarantee hlr_06_15_rx0_drop:
                (HasEvent(EthernetFramesRxIn0) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn0)) implies
                (NoSend(EthernetFramesRxOut0) and NoSend(MAVLinkFramesRxOut0));
            guarantee hlr_17_rx0_no_input:
                (not HasEvent(EthernetFramesRxIn0)) implies (NoSend(EthernetFramesRxOut0) and NoSend(MAVLinkFramesRxOut0));

            guarantee hlr_05_13_rx1_direct:
                (HasEvent(EthernetFramesRxIn1) and GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxIn1)) implies
                (HasEvent(EthernetFramesRxOut1) and EthernetFramesRxOut1 == EthernetFramesRxIn1 and NoSend(MAVLinkFramesRxOut1));
            guarantee hlr_18_rx1_mavlink:
                (HasEvent(EthernetFramesRxIn1) and GumboLib::GUMBO__Library::valid_ardupilot_udp(EthernetFramesRxIn1)) implies
                (NoSend(EthernetFramesRxOut1) and HasEvent(MAVLinkFramesRxOut1) and
                 MAVLinkFramesRxOut1.ethernet_frame == EthernetFramesRxIn1 and
                 MAVLinkFramesRxOut1.payload_offset == GumboLib::GUMBO__Library::udp_payload_offset() and
                 MAVLinkFramesRxOut1.payload_length == GumboLib::GUMBO__Library::udp_payload_length(EthernetFramesRxIn1));
            guarantee hlr_06_15_rx1_drop:
                (HasEvent(EthernetFramesRxIn1) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn1)) implies
                (NoSend(EthernetFramesRxOut1) and NoSend(MAVLinkFramesRxOut1));
            guarantee hlr_17_rx1_no_input:
                (not HasEvent(EthernetFramesRxIn1)) implies (NoSend(EthernetFramesRxOut1) and NoSend(MAVLinkFramesRxOut1));

            guarantee hlr_05_13_rx2_direct:
                (HasEvent(EthernetFramesRxIn2) and GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxIn2)) implies
                (HasEvent(EthernetFramesRxOut2) and EthernetFramesRxOut2 == EthernetFramesRxIn2 and NoSend(MAVLinkFramesRxOut2));
            guarantee hlr_18_rx2_mavlink:
                (HasEvent(EthernetFramesRxIn2) and GumboLib::GUMBO__Library::valid_ardupilot_udp(EthernetFramesRxIn2)) implies
                (NoSend(EthernetFramesRxOut2) and HasEvent(MAVLinkFramesRxOut2) and
                 MAVLinkFramesRxOut2.ethernet_frame == EthernetFramesRxIn2 and
                 MAVLinkFramesRxOut2.payload_offset == GumboLib::GUMBO__Library::udp_payload_offset() and
                 MAVLinkFramesRxOut2.payload_length == GumboLib::GUMBO__Library::udp_payload_length(EthernetFramesRxIn2));
            guarantee hlr_06_15_rx2_drop:
                (HasEvent(EthernetFramesRxIn2) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn2)) implies
                (NoSend(EthernetFramesRxOut2) and NoSend(MAVLinkFramesRxOut2));
            guarantee hlr_17_rx2_no_input:
                (not HasEvent(EthernetFramesRxIn2)) implies (NoSend(EthernetFramesRxOut2) and NoSend(MAVLinkFramesRxOut2));

            guarantee hlr_05_13_rx3_direct:
                (HasEvent(EthernetFramesRxIn3) and GumboLib::GUMBO__Library::rx_direct_frame(EthernetFramesRxIn3)) implies
                (HasEvent(EthernetFramesRxOut3) and EthernetFramesRxOut3 == EthernetFramesRxIn3 and NoSend(MAVLinkFramesRxOut3));
            guarantee hlr_18_rx3_mavlink:
                (HasEvent(EthernetFramesRxIn3) and GumboLib::GUMBO__Library::valid_ardupilot_udp(EthernetFramesRxIn3)) implies
                (NoSend(EthernetFramesRxOut3) and HasEvent(MAVLinkFramesRxOut3) and
                 MAVLinkFramesRxOut3.ethernet_frame == EthernetFramesRxIn3 and
                 MAVLinkFramesRxOut3.payload_offset == GumboLib::GUMBO__Library::udp_payload_offset() and
                 MAVLinkFramesRxOut3.payload_length == GumboLib::GUMBO__Library::udp_payload_length(EthernetFramesRxIn3));
            guarantee hlr_06_15_rx3_drop:
                (HasEvent(EthernetFramesRxIn3) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn3)) implies
                (NoSend(EthernetFramesRxOut3) and NoSend(MAVLinkFramesRxOut3));
            guarantee hlr_17_rx3_no_input:
                (not HasEvent(EthernetFramesRxIn3)) implies (NoSend(EthernetFramesRxOut3) and NoSend(MAVLinkFramesRxOut3));
        }*/
    }

    part def MAVLinkFirewall :> Thread {
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
        integration
            guarantee hlr_22_output0_allowed:
                GumboLib::GUMBO__Library::mavlink_allowed(EthernetFramesOut0);
            guarantee hlr_22_output1_allowed:
                GumboLib::GUMBO__Library::mavlink_allowed(EthernetFramesOut1);
            guarantee hlr_22_output2_allowed:
                GumboLib::GUMBO__Library::mavlink_allowed(EthernetFramesOut2);
            guarantee hlr_22_output3_allowed:
                GumboLib::GUMBO__Library::mavlink_allowed(EthernetFramesOut3);

        compute
            guarantee hlr_22_23_lane0_allow:
                (HasEvent(EthernetFramesIn0) and GumboLib::GUMBO__Library::mavlink_allowed(EthernetFramesIn0)) implies
                (HasEvent(EthernetFramesOut0) and EthernetFramesOut0 == EthernetFramesIn0);
            guarantee hlr_24_lane0_deny_flash:
                (HasEvent(EthernetFramesIn0) and GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn0) and
                 GumboLib::GUMBO__Library::mavlink_frame_valid(EthernetFramesIn0) and
                 GumboLib::GUMBO__Library::mavlink_firmware_flash_command(EthernetFramesIn0)) implies NoSend(EthernetFramesOut0);
            guarantee hlr_25_26_lane0_invalid_or_no_input:
                ((not HasEvent(EthernetFramesIn0)) or
                 (HasEvent(EthernetFramesIn0) and not (GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn0) and
                  GumboLib::GUMBO__Library::mavlink_frame_valid(EthernetFramesIn0)))) implies NoSend(EthernetFramesOut0);

            guarantee hlr_22_23_lane1_allow:
                (HasEvent(EthernetFramesIn1) and GumboLib::GUMBO__Library::mavlink_allowed(EthernetFramesIn1)) implies
                (HasEvent(EthernetFramesOut1) and EthernetFramesOut1 == EthernetFramesIn1);
            guarantee hlr_24_lane1_deny_flash:
                (HasEvent(EthernetFramesIn1) and GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn1) and
                 GumboLib::GUMBO__Library::mavlink_frame_valid(EthernetFramesIn1) and
                 GumboLib::GUMBO__Library::mavlink_firmware_flash_command(EthernetFramesIn1)) implies NoSend(EthernetFramesOut1);
            guarantee hlr_25_26_lane1_invalid_or_no_input:
                ((not HasEvent(EthernetFramesIn1)) or
                 (HasEvent(EthernetFramesIn1) and not (GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn1) and
                  GumboLib::GUMBO__Library::mavlink_frame_valid(EthernetFramesIn1)))) implies NoSend(EthernetFramesOut1);

            guarantee hlr_22_23_lane2_allow:
                (HasEvent(EthernetFramesIn2) and GumboLib::GUMBO__Library::mavlink_allowed(EthernetFramesIn2)) implies
                (HasEvent(EthernetFramesOut2) and EthernetFramesOut2 == EthernetFramesIn2);
            guarantee hlr_24_lane2_deny_flash:
                (HasEvent(EthernetFramesIn2) and GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn2) and
                 GumboLib::GUMBO__Library::mavlink_frame_valid(EthernetFramesIn2) and
                 GumboLib::GUMBO__Library::mavlink_firmware_flash_command(EthernetFramesIn2)) implies NoSend(EthernetFramesOut2);
            guarantee hlr_25_26_lane2_invalid_or_no_input:
                ((not HasEvent(EthernetFramesIn2)) or
                 (HasEvent(EthernetFramesIn2) and not (GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn2) and
                  GumboLib::GUMBO__Library::mavlink_frame_valid(EthernetFramesIn2)))) implies NoSend(EthernetFramesOut2);

            guarantee hlr_22_23_lane3_allow:
                (HasEvent(EthernetFramesIn3) and GumboLib::GUMBO__Library::mavlink_allowed(EthernetFramesIn3)) implies
                (HasEvent(EthernetFramesOut3) and EthernetFramesOut3 == EthernetFramesIn3);
            guarantee hlr_24_lane3_deny_flash:
                (HasEvent(EthernetFramesIn3) and GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn3) and
                 GumboLib::GUMBO__Library::mavlink_frame_valid(EthernetFramesIn3) and
                 GumboLib::GUMBO__Library::mavlink_firmware_flash_command(EthernetFramesIn3)) implies NoSend(EthernetFramesOut3);
            guarantee hlr_25_26_lane3_invalid_or_no_input:
                ((not HasEvent(EthernetFramesIn3)) or
                 (HasEvent(EthernetFramesIn3) and not (GumboLib::GUMBO__Library::valid_mavlink_carrier(EthernetFramesIn3) and
                  GumboLib::GUMBO__Library::mavlink_frame_valid(EthernetFramesIn3)))) implies NoSend(EthernetFramesOut3);
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
