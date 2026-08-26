package GumboLib {
  private import HAMR::*;
  //private import open_platform_Data_Model::*;

  language "GUMBO" /*{
    library

    functions
      def TCP_ALLOWED_PORTS(): open_platform_Data_Model::u16Array := open_platform_Data_Model::u16Array(5760[u16]);

      def UDP_ALLOWED_PORTS(): open_platform_Data_Model::u16Array := open_platform_Data_Model::u16Array(68[u16]);

      def two_bytes_to_u16_le(byte0: Base_Types::Unsigned_8, byte1: Base_Types::Unsigned_8): Base_Types::Unsigned_16 :=
        byte1.toU16 * 256[u16] + byte0.toU16;

      def two_bytes_to_u16_be(byte0: Base_Types::Unsigned_8, byte1: Base_Types::Unsigned_8): Base_Types::Unsigned_16 :=
        byte0.toU16 * 256[u16] + byte1.toU16;

      def three_bytes_to_u32(byte0: Base_Types::Unsigned_8, byte1: Base_Types::Unsigned_8, byte2: Base_Types::Unsigned_8): Base_Types::Unsigned_32 :=
        byte2.toU32 * 65536[u32] + byte1.toU32 * 256[u32] + byte0.toU32;

      def frame_is_wellformed_eth2(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        valid_frame_ethertype(aframe) and valid_frame_dst_addr(aframe);

      def valid_frame_ethertype(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        frame_has_ipv4(aframe) | frame_has_arp(aframe) | frame_has_ipv6(aframe);

      def valid_frame_dst_addr(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        ( not ((aframe#(0) == 0[u8]) and
        (aframe#(1) == 0[u8]) and
        (aframe#(2) == 0[u8]) and
        (aframe#(3) == 0[u8]) and
        (aframe#(4) == 0[u8]) and
        (aframe#(5) == 0[u8])));

      def frame_has_ipv4(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        (aframe#(12) == 8[u8] and aframe#(13) == 0[u8]);

      def frame_has_ipv6(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        (aframe#(12) == 134[u8] and aframe#(13) == 221[u8]);

      def frame_has_arp(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        (aframe#(12) == 8[u8] and aframe#(13) == 6[u8]);

      def arp_has_ipv4(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        (aframe#(16) == 8[u8] and aframe#(17) == 0[u8]);

      def arp_has_ipv6(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        (aframe#(16) == 134[u8] and aframe#(17) == 221[u8]);

      def valid_arp_ptype(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        arp_has_ipv4(aframe) | arp_has_ipv6(aframe);

      def valid_arp_op(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        (aframe#(20) == 0[u8] and (aframe#(21) == 1[u8] | aframe#(21) == 2[u8]));

      def valid_arp_htype(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        (aframe#(14) == 0[u8] and aframe#(15) == 1[u8]);

      def wellformed_arp_frame(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        valid_arp_op(aframe) and valid_arp_htype(aframe) and valid_arp_ptype(aframe);

      def valid_ipv4_length(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        (two_bytes_to_u16_be(aframe#(16), aframe#(17)) <= 9000[u16]);

      def valid_ipv4_protocol(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        (
          (aframe#(23) == 0[u8])  |
          (aframe#(23) == 1[u8])  |
          (aframe#(23) == 2[u8])  |
          (aframe#(23) == 6[u8])  |
          (aframe#(23) == 17[u8]) |
          (aframe#(23) == 43[u8]) |
          (aframe#(23) == 44[u8]) |
          (aframe#(23) == 58[u8]) |
          (aframe#(23) == 59[u8]) |
          (aframe#(23) == 60[u8])
        );

      def valid_ipv4_vers_ihl(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        (aframe#(14) == 69[u8]);

      def wellformed_ipv4_frame(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        valid_ipv4_protocol(aframe) and valid_ipv4_length(aframe) and valid_ipv4_vers_ihl(aframe);

      // TxFirewall

      def valid_ipv6(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        frame_is_wellformed_eth2(aframe) and frame_has_ipv6(aframe);

      def valid_arp(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        frame_is_wellformed_eth2(aframe) and frame_has_arp(aframe) and wellformed_arp_frame(aframe);

      def valid_ipv4(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        frame_is_wellformed_eth2(aframe) and frame_has_ipv4(aframe) and wellformed_ipv4_frame(aframe);

      def ipv4_length(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Unsigned_16 :=
        two_bytes_to_u16_le(aframe#(16),aframe#(17));

      def valid_output_arp_size(output: open_platform_Data_Model::SizedEthernetMessage_Impl): Base_Types::Boolean :=
        output.sz == 64[u16];

      def valid_output_ipv4_size(input: open_platform_Data_Model::RawEthernetMessage, output: open_platform_Data_Model::SizedEthernetMessage_Impl): Base_Types::Boolean :=
        //output.sz == (ipv4_length(input)+14[u16]);
        (size(input) == 1600) and (output.sz == (two_bytes_to_u16_be(input#(16),input#(17))+14[u16]));

      def tx_allow_outbound_frame(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        valid_arp(aframe) | valid_ipv4(aframe);

      // RxFirewall

      def ipv4_is_tcp(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and (aframe#(23) == 6[u8]);

      def ipv4_is_udp(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and (aframe#(23) == 17[u8]);

      def tcp_is_valid_port(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and (two_bytes_to_u16_be(aframe#(36),aframe#(37)) == TCP_ALLOWED_PORTS()#(0));

      def udp_is_valid_port(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and (two_bytes_to_u16_be(aframe#(36),aframe#(37)) == UDP_ALLOWED_PORTS()#(0));

      def frame_has_ipv4_tcp_on_allowed_port_quant(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        ((0 .. (size(TCP_ALLOWED_PORTS()) - 1))
          -> exists {in i; TCP_ALLOWED_PORTS()#(i) == two_bytes_to_u16_be(aframe#(36), aframe#(37))});

      def udp_is_valid_dst_port(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        size(aframe) == 1600 and
        ((0 .. (size(UDP_ALLOWED_PORTS()) - 1))
          -> exists {in i; UDP_ALLOWED_PORTS()#(i) == two_bytes_to_u16_be(aframe#(36), aframe#(37))});

      def valid_ipv4_tcp(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        frame_is_wellformed_eth2(aframe) and frame_has_ipv4(aframe) and wellformed_ipv4_frame(aframe) and ipv4_is_tcp(aframe);

      def valid_ipv4_udp(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        frame_is_wellformed_eth2(aframe) and frame_has_ipv4(aframe) and wellformed_ipv4_frame(aframe) and ipv4_is_udp(aframe);

      def valid_ipv4_tcp_port(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        valid_ipv4_tcp(aframe) and frame_has_ipv4_tcp_on_allowed_port_quant(aframe);

      def valid_ipv4_udp_port(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        valid_ipv4_udp(aframe) and udp_is_valid_dst_port(aframe);

      def rx_allow_outbound_frame(aframe: open_platform_Data_Model::RawEthernetMessage): Base_Types::Boolean :=
        valid_arp(aframe)
        or valid_ipv4_tcp_port(aframe)
        or valid_ipv4_udp_port(aframe);

  }*/
}
