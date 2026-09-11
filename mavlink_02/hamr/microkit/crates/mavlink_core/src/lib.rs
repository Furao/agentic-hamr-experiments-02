#![cfg_attr(not(test), no_std)]

pub mod verified;
pub use verified::{parse, InvalidReason, Message};

pub const MAVLINK_V1_MAGIC: u8 = 0xfe;
pub const MAVLINK_V2_MAGIC: u8 = 0xfd;
pub const MAVLINK_V2_SIGNED: u8 = 0x01;

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

    fn v2(message_id: u32, payload: &[u8], incompat: u8) -> Vec<u8> {
        let signature = if incompat & MAVLINK_V2_SIGNED != 0 { 13 } else { 0 };
        let mut frame = vec![0u8; 10 + payload.len() + 2 + signature];
        frame[0] = MAVLINK_V2_MAGIC;
        frame[1] = payload.len() as u8;
        frame[2] = incompat;
        frame[7] = message_id as u8;
        frame[8] = (message_id >> 8) as u8;
        frame[9] = (message_id >> 16) as u8;
        frame[10..10 + payload.len()].copy_from_slice(payload);
        let extra = verified::dialect::metadata(message_id).unwrap().0;
        let mut crc = 0xffff;
        for byte in &frame[1..10 + payload.len()] { crc = verified::crc_accumulate_verified(*byte, crc); }
        crc = verified::crc_accumulate_verified(extra, crc);
        let at = 10 + payload.len();
        frame[at] = crc as u8;
        frame[at + 1] = (crc >> 8) as u8;
        frame
    }

    fn v1(message_id: u8, payload: &[u8], crc_extra: u8) -> Vec<u8> {
        let mut frame = vec![0u8; 6 + payload.len() + 2];
        frame[0] = MAVLINK_V1_MAGIC; frame[1] = payload.len() as u8; frame[5] = message_id;
        frame[6..6 + payload.len()].copy_from_slice(payload);
        let mut crc = 0xffff;
        for byte in &frame[1..6 + payload.len()] { crc = verified::crc_accumulate_verified(*byte, crc); }
        crc = verified::crc_accumulate_verified(crc_extra, crc);
        let at = 6 + payload.len(); frame[at] = crc as u8; frame[at + 1] = (crc >> 8) as u8;
        frame
    }

    #[test]
    fn accepts_file_transfer_and_signed_frame() {
        let ftp = v2(110, &[0; 254], 0);
        assert_eq!(parse(&ftp, 0, ftp.len() as u16).unwrap().message_id, 110);
        let signed = v2(76, &[0; 33], MAVLINK_V2_SIGNED);
        assert!(parse(&signed, 0, signed.len() as u16).is_ok());
    }

    #[test]
    fn rejects_bad_flags_lengths_checksum_and_unknown_id() {
        let bad_flags = v2(76, &[0; 33], 0x02);
        assert_eq!(parse(&bad_flags, 0, bad_flags.len() as u16), Err(InvalidReason::UnsupportedIncompatibilityFlags));
        let truncated_zeros = v2(76, &[0; 32], 0);
        assert!(parse(&truncated_zeros, 0, truncated_zeros.len() as u16).is_ok());
        let overlong_payload = v2(76, &[0; 34], 0);
        assert_eq!(parse(&overlong_payload, 0, overlong_payload.len() as u16), Err(InvalidReason::TruncatedOrTrailingBytes));
        let mut checksum = v2(76, &[0; 33], 0);
        checksum[12] ^= 1;
        assert_eq!(parse(&checksum, 0, checksum.len() as u16), Err(InvalidReason::Checksum));
        let mut unknown = v2(76, &[0; 33], 0);
        unknown[7..10].copy_from_slice(&[0xff, 0xff, 0xff]);
        assert_eq!(parse(&unknown, 0, unknown.len() as u16), Err(InvalidReason::UnknownMessage));
        assert_eq!(parse(&[0], 1, 1), Err(InvalidReason::CarrierBounds));
        assert_eq!(parse(&[0; 8], 0, 8), Err(InvalidReason::UnsupportedVersion));
        assert_eq!(parse(&[MAVLINK_V1_MAGIC; 7], 0, 7), Err(InvalidReason::TruncatedOrTrailingBytes));
    }

    #[test]
    fn accepts_v1() {
        let frame = v1(76, &[0; 33], 152);
        assert_eq!(parse(&frame, 0, frame.len() as u16),
            Ok(Message { message_id: 76, payload_offset: 6, payload_length: 33 }));
    }
    #[test]
    fn preserves_all_diagnostic_reasons_and_carrier_bounds() {
        for (frame, offset, length, reason) in [
            (vec![], 0, 0, InvalidReason::CarrierBounds),
            (vec![0; 8], u16::MAX, 1, InvalidReason::CarrierBounds),
            (vec![0; 8], 0, u16::MAX, InvalidReason::CarrierBounds),
            (vec![0; 8], 8, 1, InvalidReason::CarrierBounds),
            (vec![MAVLINK_V2_MAGIC; 11], 0, 11, InvalidReason::TruncatedOrTrailingBytes),
        ] {
            assert_eq!(parse(&frame, offset, length), Err(reason));
        }
        let valid = v2(76, &[0; 33], 0);
        let mut trailing = valid.clone();
        trailing.push(0);
        assert_eq!(parse(&trailing, 0, trailing.len() as u16),
            Err(InvalidReason::TruncatedOrTrailingBytes));
        let short_v1 = v1(76, &[0; 32], 152);
        assert_eq!(parse(&short_v1, 0, short_v1.len() as u16),
            Err(InvalidReason::TruncatedOrTrailingBytes));
        let mut signed = v2(76, &[0; 33], MAVLINK_V2_SIGNED);
        signed.pop();
        assert_eq!(parse(&signed, 0, signed.len() as u16),
            Err(InvalidReason::TruncatedOrTrailingBytes));
    }

    #[test]
    fn returns_payload_locations_at_carrier_boundaries() {
        for (packet, id, header, payload_len) in [
            (v2(110, &[0; 254], 0), 110, 10, 254),
            (v2(76, &[0; 29], MAVLINK_V2_SIGNED), 76, 10, 29),
            (v1(76, &[0; 33], 152), 76, 6, 33),
        ] {
            for start in [0, 42, 1600 - packet.len()] {
                let mut carrier = [0; 1600];
                carrier[start..start + packet.len()].copy_from_slice(&packet);
                assert_eq!(parse(&carrier, start as u16, packet.len() as u16),
                    Ok(Message { message_id: id, payload_offset: start + header,
                                 payload_length: payload_len }));
            }
        }
    }
}
