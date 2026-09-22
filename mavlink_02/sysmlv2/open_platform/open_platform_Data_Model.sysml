package open_platform_Data_Model {
    private import HAMR::*;

    // CR-02 HLR-24, HLR-26; LLR-10/11: sampled control mode.
    // Normal is first; explicit startup guarantees follow in CompGUMBOSpec.
    enum def OperatingMode {
        enum Normal;
        enum Recovery;
    }

    part def u16Array :> Array {
        part :>> Base_Type : Unsigned_16;
        attribute :>> Dimensions = 1;
        attribute :>> Data_Size = 2 [byte];
        attribute :>> Array_Size_Kind = Fixed;
    }

    part def RawEthernetMessage :> Array {
        part :>> Base_Type : Unsigned_8;
        attribute :>> Dimensions = 1600;
        attribute :>> Array_Size_Kind = Fixed; 
        attribute :>> Data_Size = 1600[byte];
    }

    part def SizedEthernetMessage_Impl :> Struct {
        part amessage : RawEthernetMessage;
        part sz : Unsigned_16;
    }

    part def MAVLinkUDPMessage_Impl :> Struct {
        part ethernet_frame : RawEthernetMessage;
        part payload_offset : Unsigned_16;
        part payload_length : Unsigned_16;
    }
}
