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

        attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
        attribute :>> Period = 1000[ms]; 
        attribute :>> Compute_Execution_Time =  300[ms] .. 300[ms];
        attribute :>> Stack_Size = 1048576[byte];
        attribute :>> Implementation_Language = Rust;			

        language "GUMBO" /*{

        compute

            guarantee hlr_05_rx0_can_send_arp:
            ((HasEvent(EthernetFramesRxIn0) and GumboLib::GUMBO__Library::valid_arp(EthernetFramesRxIn0)) implies 
                (HasEvent(EthernetFramesRxOut0) and (EthernetFramesRxIn0 == EthernetFramesRxOut0)));
            guarantee hlr_06_rx0_can_send_ipv4_tcp:
            ((HasEvent(EthernetFramesRxIn0) and GumboLib::GUMBO__Library::valid_ipv4_tcp_port(EthernetFramesRxIn0)) implies 
                (HasEvent(EthernetFramesRxOut0) and (EthernetFramesRxIn0 == EthernetFramesRxOut0)));
            guarantee hlr_13_rx0_can_send_ipv4_udp:
            ((HasEvent(EthernetFramesRxIn0) and GumboLib::GUMBO__Library::valid_ipv4_udp_port(EthernetFramesRxIn0)) implies 
                (HasEvent(EthernetFramesRxOut0) and (EthernetFramesRxIn0 == EthernetFramesRxOut0)));
            guarantee hlr_15_rx0_disallow:
            (HasEvent(EthernetFramesRxIn0) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn0)) implies 
                (NoSend(EthernetFramesRxOut0));
            guarantee hlr_17_rx0_no_input:
            (HasEvent(EthernetFramesRxIn0) | NoSend(EthernetFramesRxOut0));

            guarantee hlr_05_rx1_can_send_arp:
            ((HasEvent(EthernetFramesRxIn1) and GumboLib::GUMBO__Library::valid_arp(EthernetFramesRxIn1)) implies 
                (HasEvent(EthernetFramesRxOut1) and (EthernetFramesRxIn1 == EthernetFramesRxOut1)));
            guarantee hlr_06_rx1_can_send_ipv4_tcp:
            ((HasEvent(EthernetFramesRxIn1) and GumboLib::GUMBO__Library::valid_ipv4_tcp_port(EthernetFramesRxIn1)) implies 
                (HasEvent(EthernetFramesRxOut1) and (EthernetFramesRxIn1 == EthernetFramesRxOut1)));
            guarantee hlr_13_rx1_can_send_ipv4_udp:
            ((HasEvent(EthernetFramesRxIn1) and GumboLib::GUMBO__Library::valid_ipv4_udp_port(EthernetFramesRxIn1)) implies 
                (HasEvent(EthernetFramesRxOut1) and (EthernetFramesRxIn1 == EthernetFramesRxOut1)));
            guarantee hlr_15_rx1_disallow:
            (HasEvent(EthernetFramesRxIn1) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn1)) implies 
                (NoSend(EthernetFramesRxOut1));
            guarantee hlr_17_rx1_no_input:
            (HasEvent(EthernetFramesRxIn1) | NoSend(EthernetFramesRxOut1));

            guarantee hlr_05_rx2_can_send_arp:
            ((HasEvent(EthernetFramesRxIn2) and GumboLib::GUMBO__Library::valid_arp(EthernetFramesRxIn2)) implies 
                (HasEvent(EthernetFramesRxOut2) and (EthernetFramesRxIn2 == EthernetFramesRxOut2)));
            guarantee hlr_06_rx2_can_send_ipv4_tcp:
            ((HasEvent(EthernetFramesRxIn2) and GumboLib::GUMBO__Library::valid_ipv4_tcp_port(EthernetFramesRxIn2)) implies 
                (HasEvent(EthernetFramesRxOut2) and (EthernetFramesRxIn2 == EthernetFramesRxOut2)));
            guarantee hlr_13_rx2_can_send_ipv4_udp:
            ((HasEvent(EthernetFramesRxIn2) and GumboLib::GUMBO__Library::valid_ipv4_udp_port(EthernetFramesRxIn2)) implies 
                (HasEvent(EthernetFramesRxOut2) and (EthernetFramesRxIn2 == EthernetFramesRxOut2)));
            guarantee hlr_15_rx2_disallow:
            (HasEvent(EthernetFramesRxIn2) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn2)) implies 
                (NoSend(EthernetFramesRxOut2));
            guarantee hlr_17_rx2_no_input:
            (HasEvent(EthernetFramesRxIn2) | NoSend(EthernetFramesRxOut2));

            guarantee hlr_05_rx3_can_send_arp:
            ((HasEvent(EthernetFramesRxIn3) and GumboLib::GUMBO__Library::valid_arp(EthernetFramesRxIn3)) implies 
                (HasEvent(EthernetFramesRxOut3) and (EthernetFramesRxIn3 == EthernetFramesRxOut3)));
            guarantee hlr_06_rx3_can_send_ipv4_tcp:
            ((HasEvent(EthernetFramesRxIn3) and GumboLib::GUMBO__Library::valid_ipv4_tcp_port(EthernetFramesRxIn3)) implies 
                (HasEvent(EthernetFramesRxOut3) and (EthernetFramesRxIn3 == EthernetFramesRxOut3)));
            guarantee hlr_13_rx3_can_send_ipv4_udp:
            ((HasEvent(EthernetFramesRxIn3) and GumboLib::GUMBO__Library::valid_ipv4_udp_port(EthernetFramesRxIn3)) implies 
                (HasEvent(EthernetFramesRxOut3) and (EthernetFramesRxIn3 == EthernetFramesRxOut3)));
            guarantee hlr_15_rx3_disallow:
            (HasEvent(EthernetFramesRxIn3) and not GumboLib::GUMBO__Library::rx_allow_outbound_frame(EthernetFramesRxIn3)) implies 
                (NoSend(EthernetFramesRxOut3));
            guarantee hlr_17_rx3_no_input:
            (HasEvent(EthernetFramesRxIn3) | NoSend(EthernetFramesRxOut3));
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
