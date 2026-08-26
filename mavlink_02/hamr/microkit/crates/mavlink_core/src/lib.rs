#![cfg_attr(not(test), no_std)]

mod dialect;

pub const MAVLINK_V1_MAGIC: u8 = 0xfe;
pub const MAVLINK_V2_MAGIC: u8 = 0xfd;
pub const MAVLINK_V2_SIGNED: u8 = 0x01;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvalidReason {
    CarrierBounds,
    UnsupportedVersion,
    UnsupportedIncompatibilityFlags,
    TruncatedOrTrailingBytes,
    UnknownMessage,
    Checksum,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Message {
    pub message_id: u32,
    pub payload_offset: usize,
    pub payload_length: usize,
}

fn crc_accumulate(byte: u8, crc: u16) -> u16 {
    let mut tmp = byte ^ (crc as u8);
    tmp ^= tmp << 4;
    (crc >> 8) ^ ((tmp as u16) << 8) ^ ((tmp as u16) << 3) ^ ((tmp as u16) >> 4)
}

pub fn parse(frame: &[u8], carrier_offset: u16, carrier_length: u16) -> Result<Message, InvalidReason> {
    let start = carrier_offset as usize;
    let available = carrier_length as usize;
    let end = start.checked_add(available).ok_or(InvalidReason::CarrierBounds)?;
    if end > frame.len() || available == 0 { return Err(InvalidReason::CarrierBounds); }

    let (header_length, payload_length, message_id, signature_length) = match frame[start] {
        MAVLINK_V1_MAGIC => {
            if available < 8 { return Err(InvalidReason::TruncatedOrTrailingBytes); }
            (6usize, frame[start + 1] as usize, frame[start + 5] as u32, 0usize)
        }
        MAVLINK_V2_MAGIC => {
            if available < 12 { return Err(InvalidReason::TruncatedOrTrailingBytes); }
            let incompat = frame[start + 2];
            if incompat & !MAVLINK_V2_SIGNED != 0 {
                return Err(InvalidReason::UnsupportedIncompatibilityFlags);
            }
            let id = frame[start + 7] as u32
                | ((frame[start + 8] as u32) << 8)
                | ((frame[start + 9] as u32) << 16);
            (10usize, frame[start + 1] as usize, id,
                if incompat & MAVLINK_V2_SIGNED != 0 { 13 } else { 0 })
        }
        _ => return Err(InvalidReason::UnsupportedVersion),
    };

    let exact_length = header_length + payload_length + 2 + signature_length;
    if available != exact_length { return Err(InvalidReason::TruncatedOrTrailingBytes); }
    let (crc_extra, minimum_length, maximum_length) = dialect::crc_extra(message_id).ok_or(InvalidReason::UnknownMessage)?;
    let valid_payload_length = if header_length == 6 {
        payload_length == maximum_length
    } else {
        payload_length >= minimum_length && payload_length <= maximum_length
    };
    if !valid_payload_length { return Err(InvalidReason::TruncatedOrTrailingBytes); }
    let checksum_offset = start + header_length + payload_length;
    let mut crc = 0xffffu16;
    for byte in &frame[start + 1..checksum_offset] { crc = crc_accumulate(*byte, crc); }
    crc = crc_accumulate(crc_extra, crc);
    let received = frame[checksum_offset] as u16 | ((frame[checksum_offset + 1] as u16) << 8);
    if crc != received { return Err(InvalidReason::Checksum); }
    Ok(Message { message_id, payload_offset: start + header_length, payload_length })
}

pub fn payload_u16_le(frame: &[u8], message: Message, offset: usize) -> Option<u16> {
    if offset.checked_add(2)? > message.payload_length { return None; }
    let at = message.payload_offset + offset;
    Some(frame[at] as u16 | ((frame[at + 1] as u16) << 8))
}

pub fn payload_u32_le(frame: &[u8], message: Message, offset: usize) -> Option<u32> {
    if offset.checked_add(4)? > message.payload_length { return None; }
    let at = message.payload_offset + offset;
    Some(frame[at] as u32 | ((frame[at + 1] as u32) << 8)
        | ((frame[at + 2] as u32) << 16) | ((frame[at + 3] as u32) << 24))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let extra = dialect::crc_extra(message_id).unwrap().0;
        let mut crc = 0xffff;
        for byte in &frame[1..10 + payload.len()] { crc = crc_accumulate(*byte, crc); }
        crc = crc_accumulate(extra, crc);
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
        for byte in &frame[1..6 + payload.len()] { crc = crc_accumulate(*byte, crc); }
        crc = crc_accumulate(crc_extra, crc);
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
        let short_payload = v2(76, &[0; 32], 0);
        assert_eq!(parse(&short_payload, 0, short_payload.len() as u16), Err(InvalidReason::TruncatedOrTrailingBytes));
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
    fn accepts_v1_and_reads_bounded_payload_fields() {
        let mut payload = [0u8; 33];
        payload[4..8].copy_from_slice(&7u32.to_le_bytes());
        payload[28..30].copy_from_slice(&42650u16.to_le_bytes());
        let frame = v1(76, &payload, 152);
        let message = parse(&frame, 0, frame.len() as u16).unwrap();
        assert_eq!(payload_u16_le(&frame, message, 28), Some(42650));
        assert_eq!(payload_u32_le(&frame, message, 4), Some(7));
        assert_eq!(payload_u16_le(&frame, message, usize::MAX), None);
        assert_eq!(payload_u32_le(&frame, message, 31), None);
    }
}
