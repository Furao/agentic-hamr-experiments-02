//@ HAMR: --platform Microkit --output-dir ../../hamr

package open_platform {
    private import HAMR::*;
    private import open_platform_Data_Model::*;
    private import GumboLib::*;
    //private import Platform::*;
    //private import open_platform_Software::*;

    part def LowLevelEthernetDriver_seL4 :> Process {
        attribute :>> Domain = open_platform_Properties::Domain_LowLevelEthernetDriver; 

        part LowLevelEthernetDriver : open_platform_Software::LowLevelEthernetDriver; 

        port EthernetFramesRx0: EventDataPort {out :>> type : RawEthernetMessage; }
        port EthernetFramesRx1: EventDataPort {out :>> type : RawEthernetMessage; }
        port EthernetFramesRx2: EventDataPort {out :>> type : RawEthernetMessage; } 
        port EthernetFramesRx3: EventDataPort {out :>> type : RawEthernetMessage; }
        port EthernetFramesTx0: EventDataPort {in :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTx1: EventDataPort {in :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTx2: EventDataPort {in :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTx3: EventDataPort {in :>> type : SizedEthernetMessage_Impl; }

        connection c1 : PortConnection 
            connect EthernetFramesTx0 to LowLevelEthernetDriver.EthernetFramesTx0;
        connection c2 : PortConnection 
            connect EthernetFramesTx1 to LowLevelEthernetDriver.EthernetFramesTx1;
        connection c3 : PortConnection 
            connect EthernetFramesTx2 to LowLevelEthernetDriver.EthernetFramesTx2;
        connection c4 : PortConnection 
            connect EthernetFramesTx3 to LowLevelEthernetDriver.EthernetFramesTx3;
            
        connection c5 : PortConnection
            connect LowLevelEthernetDriver.EthernetFramesRx0 to EthernetFramesRx0;
        connection c6 : PortConnection
            connect LowLevelEthernetDriver.EthernetFramesRx1 to EthernetFramesRx1;
        connection c7 : PortConnection
            connect LowLevelEthernetDriver.EthernetFramesRx2 to EthernetFramesRx2;
        connection c8 : PortConnection
            connect LowLevelEthernetDriver.EthernetFramesRx3 to EthernetFramesRx3;
    }

    part def RxFirewall_seL4 :> Process {
        port EthernetFramesRxIn0 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesRxIn1 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesRxIn2 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesRxIn3 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesRxOut0 : EventDataPort { out :>> type : RawEthernetMessage; }
        port EthernetFramesRxOut1 : EventDataPort { out :>> type : RawEthernetMessage; }
        port EthernetFramesRxOut2 : EventDataPort { out :>> type : RawEthernetMessage; }
        port EthernetFramesRxOut3 : EventDataPort { out :>> type : RawEthernetMessage; }
        port MAVLinkFramesRxOut0 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port MAVLinkFramesRxOut1 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port MAVLinkFramesRxOut2 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port MAVLinkFramesRxOut3 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }

        part RxFirewall : open_platform_Software::RxFirewall;
        connection c1 : PortConnection 
            connect EthernetFramesRxIn0 to RxFirewall.EthernetFramesRxIn0;
        connection c2 : PortConnection 
            connect EthernetFramesRxIn1 to RxFirewall.EthernetFramesRxIn1;
        connection c3 : PortConnection 
            connect EthernetFramesRxIn2 to RxFirewall.EthernetFramesRxIn2;
        connection c4 : PortConnection 
            connect EthernetFramesRxIn3 to RxFirewall.EthernetFramesRxIn3;	
        connection c5 : PortConnection
            connect RxFirewall.EthernetFramesRxOut0 to EthernetFramesRxOut0;
        connection c6 : PortConnection
            connect RxFirewall.EthernetFramesRxOut1 to EthernetFramesRxOut1;
        connection c7 : PortConnection
            connect RxFirewall.EthernetFramesRxOut2 to EthernetFramesRxOut2;
        connection c8 : PortConnection
            connect RxFirewall.EthernetFramesRxOut3 to EthernetFramesRxOut3;
        connection c9 : PortConnection
            connect RxFirewall.MAVLinkFramesRxOut0 to MAVLinkFramesRxOut0;
        connection c10 : PortConnection
            connect RxFirewall.MAVLinkFramesRxOut1 to MAVLinkFramesRxOut1;
        connection c11 : PortConnection
            connect RxFirewall.MAVLinkFramesRxOut2 to MAVLinkFramesRxOut2;
        connection c12 : PortConnection
            connect RxFirewall.MAVLinkFramesRxOut3 to MAVLinkFramesRxOut3;

            
        attribute :>> Domain = open_platform_Properties::Domain_RxFirewall;
    }

    part def MAVLinkFirewall_seL4 :> Process {
        port EthernetFramesIn0 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesIn1 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesIn2 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesIn3 : EventDataPort { in :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesOut0 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesOut1 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesOut2 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }
        port EthernetFramesOut3 : EventDataPort { out :>> type : MAVLinkUDPMessage_Impl; }

        part MAVLinkFirewall : open_platform_Software::MAVLinkFirewall;

        connection c1 : PortConnection
            connect EthernetFramesIn0 to MAVLinkFirewall.EthernetFramesIn0;
        connection c2 : PortConnection
            connect EthernetFramesIn1 to MAVLinkFirewall.EthernetFramesIn1;
        connection c3 : PortConnection
            connect EthernetFramesIn2 to MAVLinkFirewall.EthernetFramesIn2;
        connection c4 : PortConnection
            connect EthernetFramesIn3 to MAVLinkFirewall.EthernetFramesIn3;
        connection c5 : PortConnection
            connect MAVLinkFirewall.EthernetFramesOut0 to EthernetFramesOut0;
        connection c6 : PortConnection
            connect MAVLinkFirewall.EthernetFramesOut1 to EthernetFramesOut1;
        connection c7 : PortConnection
            connect MAVLinkFirewall.EthernetFramesOut2 to EthernetFramesOut2;
        connection c8 : PortConnection
            connect MAVLinkFirewall.EthernetFramesOut3 to EthernetFramesOut3;

        attribute :>> Domain = open_platform_Properties::Domain_MAVLinkFirewall;
    }

    part def TxFirewall_seL4 :> Process {
        port EthernetFramesTxIn0 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesTxIn1 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesTxIn2 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesTxIn3 : EventDataPort { in :>> type : RawEthernetMessage; }
        port EthernetFramesTxOut0 : EventDataPort { out :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTxOut1 : EventDataPort { out :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTxOut2 : EventDataPort { out :>> type : SizedEthernetMessage_Impl; }
        port EthernetFramesTxOut3 : EventDataPort { out :>> type : SizedEthernetMessage_Impl; }

        part TxFirewall : open_platform_Software::TxFirewall;

        connection c1 : PortConnection 
            connect EthernetFramesTxIn0 to TxFirewall.EthernetFramesTxIn0;
        connection c2 : PortConnection 
            connect EthernetFramesTxIn1 to TxFirewall.EthernetFramesTxIn1;
        connection c3 : PortConnection 
            connect EthernetFramesTxIn2 to TxFirewall.EthernetFramesTxIn2;
        connection c4 : PortConnection 
            connect EthernetFramesTxIn3 to TxFirewall.EthernetFramesTxIn3;	
        connection c5 : PortConnection
            connect TxFirewall.EthernetFramesTxOut0 to EthernetFramesTxOut0;
        connection c6 : PortConnection
            connect TxFirewall.EthernetFramesTxOut1 to EthernetFramesTxOut1;
        connection c7 : PortConnection
            connect TxFirewall.EthernetFramesTxOut2 to EthernetFramesTxOut2;
        connection c8 : PortConnection
            connect TxFirewall.EthernetFramesTxOut3 to EthernetFramesTxOut3;
            
        attribute :>> Domain = open_platform_Properties::Domain_TxFirewall;
    }

    part def ArduPilot_seL4 :> Process {
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

        part ArduPilot : open_platform_Software::ArduPilot;

        connection c1 : PortConnection 
            connect FirewallRx0 to ArduPilot.FirewallRx0;
        connection c2 : PortConnection 
            connect FirewallRx1 to ArduPilot.FirewallRx1;
        connection c3 : PortConnection 
            connect FirewallRx2 to ArduPilot.FirewallRx2;
        connection c4 : PortConnection 
            connect FirewallRx3 to ArduPilot.FirewallRx3;
        connection c5 : PortConnection
            connect MAVLinkFirewallRx0 to ArduPilot.MAVLinkFirewallRx0;
        connection c6 : PortConnection
            connect MAVLinkFirewallRx1 to ArduPilot.MAVLinkFirewallRx1;
        connection c7 : PortConnection
            connect MAVLinkFirewallRx2 to ArduPilot.MAVLinkFirewallRx2;
        connection c8 : PortConnection
            connect MAVLinkFirewallRx3 to ArduPilot.MAVLinkFirewallRx3;
        connection c9 : PortConnection
            connect ArduPilot.EthernetFramesTx0 to EthernetFramesTx0;
        connection c10 : PortConnection
            connect ArduPilot.EthernetFramesTx1 to EthernetFramesTx1;
        connection c11 : PortConnection
            connect ArduPilot.EthernetFramesTx2 to EthernetFramesTx2;
        connection c12 : PortConnection
            connect ArduPilot.EthernetFramesTx3 to EthernetFramesTx3;
            
        attribute :>> Domain = open_platform_Properties::Domain_ArduPilot;
    }

    part def ArduPilot_MOCK :> open_platform_Software::ArduPilot {
        attribute :>> SMC = false;
    }

    part def ArduPilot_seL4_MOCK :> ArduPilot_seL4 {
        part :>> ArduPilot : ArduPilot_MOCK;
    }

    part def seL4 :> System {
        part RxFirewall : RxFirewall_seL4;
        part TxFirewall : TxFirewall_seL4;
        part LowLevelEthernetDriver : LowLevelEthernetDriver_seL4;
        part ArduPilot : ArduPilot_seL4;
        part MAVLinkFirewall : MAVLinkFirewall_seL4;

        attribute Domain : CASE_Scheduling::Domain = 2;
        connection c1 : PortConnection
        connect LowLevelEthernetDriver.EthernetFramesRx0 to RxFirewall.EthernetFramesRxIn0;
        connection c2 : PortConnection
        connect LowLevelEthernetDriver.EthernetFramesRx1 to RxFirewall.EthernetFramesRxIn1;
        connection c3 : PortConnection
        connect LowLevelEthernetDriver.EthernetFramesRx2 to RxFirewall.EthernetFramesRxIn2;
        connection c4 : PortConnection
        connect LowLevelEthernetDriver.EthernetFramesRx3 to RxFirewall.EthernetFramesRxIn3;
        
        connection c5 : PortConnection
        connect RxFirewall.EthernetFramesRxOut0 to ArduPilot.FirewallRx0;
        connection c6 : PortConnection
        connect RxFirewall.EthernetFramesRxOut1 to ArduPilot.FirewallRx1;
        connection c7 : PortConnection
        connect RxFirewall.EthernetFramesRxOut2 to ArduPilot.FirewallRx2;
        connection c8 : PortConnection
        connect RxFirewall.EthernetFramesRxOut3 to ArduPilot.FirewallRx3;
        
        connection c9 : PortConnection
        connect ArduPilot.EthernetFramesTx0 to TxFirewall.EthernetFramesTxIn0;
        connection c10 : PortConnection
        connect ArduPilot.EthernetFramesTx1 to TxFirewall.EthernetFramesTxIn1;
        connection c11 : PortConnection
        connect ArduPilot.EthernetFramesTx2 to TxFirewall.EthernetFramesTxIn2;
        connection c12 : PortConnection
        connect ArduPilot.EthernetFramesTx3 to TxFirewall.EthernetFramesTxIn3;
        
        connection c13 : PortConnection
        connect TxFirewall.EthernetFramesTxOut0 to LowLevelEthernetDriver.EthernetFramesTx0;
        connection c14 : PortConnection
        connect TxFirewall.EthernetFramesTxOut1 to LowLevelEthernetDriver.EthernetFramesTx1;
        connection c15 : PortConnection
        connect TxFirewall.EthernetFramesTxOut2 to LowLevelEthernetDriver.EthernetFramesTx2;
        connection c16 : PortConnection
        connect TxFirewall.EthernetFramesTxOut3 to LowLevelEthernetDriver.EthernetFramesTx3;

        connection c17 : PortConnection
        connect RxFirewall.MAVLinkFramesRxOut0 to MAVLinkFirewall.EthernetFramesIn0;
        connection c18 : PortConnection
        connect RxFirewall.MAVLinkFramesRxOut1 to MAVLinkFirewall.EthernetFramesIn1;
        connection c19 : PortConnection
        connect RxFirewall.MAVLinkFramesRxOut2 to MAVLinkFirewall.EthernetFramesIn2;
        connection c20 : PortConnection
        connect RxFirewall.MAVLinkFramesRxOut3 to MAVLinkFirewall.EthernetFramesIn3;

        connection c21 : PortConnection
        connect MAVLinkFirewall.EthernetFramesOut0 to ArduPilot.MAVLinkFirewallRx0;
        connection c22 : PortConnection
        connect MAVLinkFirewall.EthernetFramesOut1 to ArduPilot.MAVLinkFirewallRx1;
        connection c23 : PortConnection
        connect MAVLinkFirewall.EthernetFramesOut2 to ArduPilot.MAVLinkFirewallRx2;
        connection c24 : PortConnection
        connect MAVLinkFirewall.EthernetFramesOut3 to ArduPilot.MAVLinkFirewallRx3;
    }

    part def seL4_MOCK :> seL4 {
        part :>> ArduPilot : ArduPilot_seL4_MOCK;
    }

}
