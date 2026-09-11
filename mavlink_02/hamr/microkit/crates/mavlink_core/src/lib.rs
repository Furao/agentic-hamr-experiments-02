#![cfg_attr(not(test), no_std)]

pub mod verified;
pub mod wire;
pub use verified::{parse, InvalidReason, Message};

pub use wire::{MAVLINK_V1_MAGIC, MAVLINK_V2_MAGIC, MAVLINK_V2_SIGNED};

impl InvalidReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CarrierBounds => "carrier bounds are invalid",
            Self::UnsupportedVersion => "unsupported MAVLink version/magic",
            Self::UnsupportedIncompatibilityFlags => "unsupported MAVLink 2 incompatibility flags",
            Self::TruncatedOrTrailingBytes => "MAVLink frame length is invalid",
            Self::UnknownMessage => "message ID is absent from the configured MAVLink dialect",
            Self::Checksum => "MAVLink checksum is invalid",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::*;

    // Fixture definitions from the bundled common.xml dialect.
    const COMMAND_LONG_ID: u32 = 76;
    const COMMAND_LONG_PAYLOAD_BYTES: usize = 33;
    const COMMAND_LONG_CRC_EXTRA: u8 = 152;
    const FILE_TRANSFER_PROTOCOL_ID: u32 = 110;
    const FILE_TRANSFER_PROTOCOL_PAYLOAD_BYTES: usize = 254;
    const UNSUPPORTED_INCOMPAT_FLAG: u8 = 0x02;
    const TEST_CARRIER_BYTES: usize = 1600;
    const TEST_PAYLOAD_OFFSET: usize = 42;

    #[test]
    fn invalid_reasons_have_distinct_messages() {
        let reasons = [
            InvalidReason::CarrierBounds,
            InvalidReason::UnsupportedVersion,
            InvalidReason::UnsupportedIncompatibilityFlags,
            InvalidReason::TruncatedOrTrailingBytes,
            InvalidReason::UnknownMessage,
            InvalidReason::Checksum,
        ];
        for (index, reason) in reasons.iter().enumerate() {
            assert!(!reason.as_str().is_empty());
            for other in &reasons[index + 1..] {
                assert_ne!(reason.as_str(), other.as_str());
            }
        }
    }

    fn v2(message_id: u32, payload: &[u8], incompat_flags: u8) -> Vec<u8> {
        let signature_length = if incompat_flags & MAVLINK_V2_SIGNED != 0 { SIGNATURE_BYTES } else { 0 };
        let checksum_offset = V2_HEADER_BYTES + payload.len();
        let mut frame = vec![0u8; checksum_offset + CHECKSUM_BYTES + signature_length];
        frame[MAGIC_OFFSET] = MAVLINK_V2_MAGIC;
        frame[PAYLOAD_LENGTH_OFFSET] = payload.len() as u8;
        frame[V2_INCOMPAT_FLAGS_OFFSET] = incompat_flags;
        frame[V2_MESSAGE_ID_OFFSET..V2_HEADER_BYTES]
            .copy_from_slice(&message_id.to_le_bytes()[..V2_MESSAGE_ID_BYTES]);
        frame[V2_HEADER_BYTES..checksum_offset].copy_from_slice(payload);
        let (crc_extra, _, _) = verified::dialect::metadata(message_id).unwrap();
        write_checksum(&mut frame, checksum_offset, crc_extra);
        frame
    }

    fn v1(message_id: u8, payload: &[u8], crc_extra: u8) -> Vec<u8> {
        let checksum_offset = V1_HEADER_BYTES + payload.len();
        let mut frame = vec![0u8; checksum_offset + CHECKSUM_BYTES];
        frame[MAGIC_OFFSET] = MAVLINK_V1_MAGIC;
        frame[PAYLOAD_LENGTH_OFFSET] = payload.len() as u8;
        frame[V1_MESSAGE_ID_OFFSET] = message_id;
        frame[V1_HEADER_BYTES..checksum_offset].copy_from_slice(payload);
        write_checksum(&mut frame, checksum_offset, crc_extra);
        frame
    }

    fn write_checksum(frame: &mut [u8], checksum_offset: usize, crc_extra: u8) {
        let mut crc = CRC_INITIAL;
        for byte in &frame[CRC_START_OFFSET..checksum_offset] {
            crc = verified::crc_accumulate_verified(*byte, crc);
        }
        crc = verified::crc_accumulate_verified(crc_extra, crc);
        frame[checksum_offset..checksum_offset + CHECKSUM_BYTES].copy_from_slice(&crc.to_le_bytes());
    }

    #[test]
    fn accepts_file_transfer_and_signed_frame() {
        let ftp = v2(FILE_TRANSFER_PROTOCOL_ID, &[0; FILE_TRANSFER_PROTOCOL_PAYLOAD_BYTES], 0);
        assert_eq!(parse(&ftp, 0, ftp.len() as u16).unwrap().message_id, FILE_TRANSFER_PROTOCOL_ID);
        let signed = v2(COMMAND_LONG_ID, &[0; COMMAND_LONG_PAYLOAD_BYTES], MAVLINK_V2_SIGNED);
        assert!(parse(&signed, 0, signed.len() as u16).is_ok());
    }

    #[test]
    fn rejects_bad_flags_lengths_checksum_and_unknown_id() {
        let bad_flags = v2(COMMAND_LONG_ID, &[0; COMMAND_LONG_PAYLOAD_BYTES], UNSUPPORTED_INCOMPAT_FLAG);
        assert_eq!(parse(&bad_flags, 0, bad_flags.len() as u16), Err(InvalidReason::UnsupportedIncompatibilityFlags));
        let truncated_zeros = v2(COMMAND_LONG_ID, &[0; COMMAND_LONG_PAYLOAD_BYTES - 1], 0);
        assert!(parse(&truncated_zeros, 0, truncated_zeros.len() as u16).is_ok());
        let overlong_payload = v2(COMMAND_LONG_ID, &[0; COMMAND_LONG_PAYLOAD_BYTES + 1], 0);
        assert_eq!(parse(&overlong_payload, 0, overlong_payload.len() as u16), Err(InvalidReason::TruncatedOrTrailingBytes));
        let mut checksum = v2(COMMAND_LONG_ID, &[0; COMMAND_LONG_PAYLOAD_BYTES], 0);
        checksum[V2_HEADER_BYTES] ^= 1;
        assert_eq!(parse(&checksum, 0, checksum.len() as u16), Err(InvalidReason::Checksum));
        let mut unknown = v2(COMMAND_LONG_ID, &[0; COMMAND_LONG_PAYLOAD_BYTES], 0);
        unknown[V2_MESSAGE_ID_OFFSET..V2_HEADER_BYTES].copy_from_slice(&[u8::MAX; V2_MESSAGE_ID_BYTES]);
        assert_eq!(parse(&unknown, 0, unknown.len() as u16), Err(InvalidReason::UnknownMessage));
        assert_eq!(parse(&[0], 1, 1), Err(InvalidReason::CarrierBounds));
        assert_eq!(parse(&[0; V1_MIN_FRAME_BYTES], 0, V1_MIN_FRAME_BYTES as u16), Err(InvalidReason::UnsupportedVersion));
        assert_eq!(parse(&[MAVLINK_V1_MAGIC; V1_MIN_FRAME_BYTES - 1], 0, (V1_MIN_FRAME_BYTES - 1) as u16), Err(InvalidReason::TruncatedOrTrailingBytes));
    }

    #[test]
    fn accepts_v1() {
        let frame = v1(COMMAND_LONG_ID as u8, &[0; COMMAND_LONG_PAYLOAD_BYTES], COMMAND_LONG_CRC_EXTRA);
        assert_eq!(parse(&frame, 0, frame.len() as u16),
            Ok(Message { message_id: COMMAND_LONG_ID, payload_offset: V1_HEADER_BYTES, payload_length: COMMAND_LONG_PAYLOAD_BYTES }));
    }
    #[test]
    fn preserves_all_diagnostic_reasons_and_carrier_bounds() {
        for (frame, offset, length, reason) in [
            (vec![], 0, 0, InvalidReason::CarrierBounds),
            (vec![0; V1_MIN_FRAME_BYTES], u16::MAX, 1, InvalidReason::CarrierBounds),
            (vec![0; V1_MIN_FRAME_BYTES], 0, u16::MAX, InvalidReason::CarrierBounds),
            (vec![0; V1_MIN_FRAME_BYTES], V1_MIN_FRAME_BYTES as u16, 1, InvalidReason::CarrierBounds),
            (vec![MAVLINK_V2_MAGIC; V2_MIN_FRAME_BYTES - 1], 0, (V2_MIN_FRAME_BYTES - 1) as u16, InvalidReason::TruncatedOrTrailingBytes),
        ] {
            assert_eq!(parse(&frame, offset, length), Err(reason));
        }
        let valid = v2(COMMAND_LONG_ID, &[0; COMMAND_LONG_PAYLOAD_BYTES], 0);
        let mut trailing = valid.clone();
        trailing.push(0);
        assert_eq!(parse(&trailing, 0, trailing.len() as u16),
            Err(InvalidReason::TruncatedOrTrailingBytes));
        let short_v1 = v1(COMMAND_LONG_ID as u8, &[0; COMMAND_LONG_PAYLOAD_BYTES - 1], COMMAND_LONG_CRC_EXTRA);
        assert_eq!(parse(&short_v1, 0, short_v1.len() as u16),
            Err(InvalidReason::TruncatedOrTrailingBytes));
        let mut signed = v2(COMMAND_LONG_ID, &[0; COMMAND_LONG_PAYLOAD_BYTES], MAVLINK_V2_SIGNED);
        signed.pop();
        assert_eq!(parse(&signed, 0, signed.len() as u16),
            Err(InvalidReason::TruncatedOrTrailingBytes));
    }

    #[test]
    fn returns_payload_locations_at_carrier_boundaries() {
        for (packet, id, header, payload_len) in [
            (v2(FILE_TRANSFER_PROTOCOL_ID, &[0; FILE_TRANSFER_PROTOCOL_PAYLOAD_BYTES], 0), FILE_TRANSFER_PROTOCOL_ID, V2_HEADER_BYTES, FILE_TRANSFER_PROTOCOL_PAYLOAD_BYTES),
            (v2(COMMAND_LONG_ID, &[0; COMMAND_LONG_PAYLOAD_BYTES - 4], MAVLINK_V2_SIGNED), COMMAND_LONG_ID, V2_HEADER_BYTES, COMMAND_LONG_PAYLOAD_BYTES - 4),
            (v1(COMMAND_LONG_ID as u8, &[0; COMMAND_LONG_PAYLOAD_BYTES], COMMAND_LONG_CRC_EXTRA), COMMAND_LONG_ID, V1_HEADER_BYTES, COMMAND_LONG_PAYLOAD_BYTES),
        ] {
            for start in [0, TEST_PAYLOAD_OFFSET, TEST_CARRIER_BYTES - packet.len()] {
                let mut carrier = [0; TEST_CARRIER_BYTES];
                carrier[start..start + packet.len()].copy_from_slice(&packet);
                assert_eq!(parse(&carrier, start as u16, packet.len() as u16),
                    Ok(Message { message_id: id, payload_offset: start + header,
                                 payload_length: payload_len }));
            }
        }
    }
}
