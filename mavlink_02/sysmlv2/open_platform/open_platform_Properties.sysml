package open_platform_Properties {
    private import HAMR::*;


    // Microkit domains 0 and 1 are reserved. Each process receives a unique domain.
    attribute Domain_ArduPilot : CASE_Scheduling::Domain = 2;
    attribute Domain_TxFirewall : CASE_Scheduling::Domain = 3;
    attribute Domain_LowLevelEthernetDriver : CASE_Scheduling::Domain = 4;
    attribute Domain_RxFirewall : CASE_Scheduling::Domain = 5;
    attribute Domain_MAVLinkFirewall : CASE_Scheduling::Domain = 6;
    attribute Domain_ModeManager : CASE_Scheduling::Domain = 7;
}
