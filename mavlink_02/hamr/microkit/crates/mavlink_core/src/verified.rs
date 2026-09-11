use vstd::prelude::*;
use crate::wire::*;

pub(crate) mod dialect {
    include!("dialect_verified.rs");
}

verus! {

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

pub open spec fn crc_accumulate_spec(byte: u8, crc: u16) -> u16 {
    let tmp0 = byte ^ (crc as u8);
    let tmp = tmp0 ^ (tmp0 << 4);
    (crc >> 8) ^ ((tmp as u16) << 8) ^ ((tmp as u16) << 3) ^ ((tmp as u16) >> 4)
}

pub open spec fn crc_fold_spec(frame: Seq<u8>, start: int, end: int, crc: u16) -> u16
    decreases end - start
{
    if start >= end { crc }
    else { crc_fold_spec(frame, start + 1, end, crc_accumulate_spec(frame[start], crc)) }
}

proof fn crc_fold_append(frame: Seq<u8>, start: int, end: int, initial: u16)
    requires 0 <= start <= end < frame.len()
    ensures crc_fold_spec(frame, start, end + 1, initial) ==
        crc_accumulate_spec(frame[end], crc_fold_spec(frame, start, end, initial))
    decreases end - start
{
    reveal_with_fuel(crc_fold_spec, 2);
    if start < end {
        crc_fold_append(frame, start + 1, end, crc_accumulate_spec(frame[start], initial));
    }
}

pub fn crc_accumulate_verified(byte: u8, crc: u16) -> (result: u16)
    ensures result == crc_accumulate_spec(byte, crc)
{
    let mut mixed_byte = byte ^ (crc as u8);
    mixed_byte ^= mixed_byte << CRC_NIBBLE_SHIFT;
    (crc >> BITS_PER_BYTE)
        ^ ((mixed_byte as u16) << BITS_PER_BYTE)
        ^ ((mixed_byte as u16) << CRC_POLYNOMIAL_MIX_SHIFT)
        ^ ((mixed_byte as u16) >> CRC_NIBBLE_SHIFT)
}

pub fn crc_fold(frame: &[u8], start: usize, end: usize, initial: u16) -> (result: u16)
    requires start <= end <= frame@.len()
    ensures result == crc_fold_spec(frame@, start as int, end as int, initial)
{
    let mut index = start;
    let mut crc = initial;
    while index < end
        invariant
            start <= index <= end,
            end <= frame@.len(),
            crc == crc_fold_spec(frame@, start as int, index as int, initial),
        decreases end - index
    {
        proof { crc_fold_append(frame@, start as int, index as int, initial); }
        crc = crc_accumulate_verified(frame[index], crc);
        index += 1;
    }
    crc
}

pub open spec fn u16_le_spec(frame: Seq<u8>, at: int) -> u16 {
    (frame[at] as u16) | ((frame[at + 1] as u16) << 8)
}

pub open spec fn u24_le_spec(frame: Seq<u8>, at: int) -> u32 {
    (frame[at] as u32) | ((frame[at + 1] as u32) << 8) | ((frame[at + 2] as u32) << 16)
}

pub open spec fn u32_le_spec(frame: Seq<u8>, at: int) -> u32 {
    (frame[at] as u32) | ((frame[at + 1] as u32) << 8) |
      ((frame[at + 2] as u32) << 16) | ((frame[at + 3] as u32) << 24)
}

pub open spec fn frame_valid_spec(frame: Seq<u8>, offset: u16, length: u16) -> bool {
    let start = offset as int;
    let available = length as int;
    &&& available > 0
    &&& start + available <= frame.len()
    &&& (frame[start] == 0xfe || frame[start] == 0xfd)
    &&& (frame[start] == 0xfe ==> available >= 8)
    &&& (frame[start] == 0xfd ==> available >= 12 && (frame[start + 2] & !1u8) == 0)
    &&& {
      let header: int = if frame[start] == 0xfe { 6 } else { 10 };
      let payload = frame[start + 1] as int;
      let signature: int = if frame[start] == 0xfd && (frame[start + 2] & 1u8) != 0 { 13 } else { 0 };
      let id: u32 = if frame[start] == 0xfe { frame[start + 5] as u32 } else { u24_le_spec(frame, start + 7) };
      let meta = dialect::metadata_spec(id);
      &&& available == header + payload + 2 + signature
      &&& meta.is_some()
      &&& (if frame[start] == 0xfe { payload == meta.unwrap().2 as int }
           else { payload <= meta.unwrap().2 as int })
      &&& {
        let checksum_at = start + header + payload;
        let computed = crc_accumulate_spec(meta.unwrap().0,
          crc_fold_spec(frame, start + 1, checksum_at, 0xffffu16));
        computed == u16_le_spec(frame, checksum_at)
      }
    }
}

// Field getters are called only after the parser has checked the carrier/header bounds.
fn get_magic(frame: &[u8], start: usize) -> (value: u8)
    requires start < frame.len()
    ensures value == frame[start as int]
{
    frame[start + MAGIC_OFFSET]
}

fn get_payload_length(frame: &[u8], start: usize) -> (value: usize)
    requires start + PAYLOAD_LENGTH_OFFSET < frame.len()
    ensures value == frame[start as int + 1] as int
{
    frame[start + PAYLOAD_LENGTH_OFFSET] as usize
}

fn get_v2_incompat_flags(frame: &[u8], start: usize) -> (value: u8)
    requires start + V2_INCOMPAT_FLAGS_OFFSET < frame.len()
    ensures value == frame[start as int + 2]
{
    frame[start + V2_INCOMPAT_FLAGS_OFFSET]
}

fn get_v1_message_id(frame: &[u8], start: usize) -> (value: u32)
    requires start + V1_MESSAGE_ID_OFFSET < frame.len()
    ensures value == frame[start as int + 5] as u32
{
    frame[start + V1_MESSAGE_ID_OFFSET] as u32
}

fn get_v2_message_id(frame: &[u8], start: usize) -> (value: u32)
    requires start + V2_HEADER_BYTES <= frame.len()
    ensures value == u24_le_spec(frame@, start as int + 7)
{
    let at = start + V2_MESSAGE_ID_OFFSET;
    (frame[at] as u32)
        | ((frame[at + 1] as u32) << BITS_PER_BYTE)
        | ((frame[at + 2] as u32) << (2 * BITS_PER_BYTE))
}

fn get_checksum(frame: &[u8], at: usize) -> (value: u16)
    requires at + CHECKSUM_BYTES <= frame.len()
    ensures value == u16_le_spec(frame@, at as int)
{
    (frame[at] as u16) | ((frame[at + 1] as u16) << BITS_PER_BYTE)
}

/// Validate one complete MAVLink frame and return its policy-neutral payload location.
pub fn parse(frame: &[u8], offset: u16, length: u16) -> (result: Result<Message, InvalidReason>)
    ensures
      result.is_ok() == frame_valid_spec(frame@, offset, length),
      result.is_ok() ==> {
        let message = result.unwrap();
        let start = offset as int;
        &&& message.message_id == (if frame[start] == 0xfe { frame[start + 5] as u32 }
                                   else { u24_le_spec(frame@, start + 7) })
        &&& message.payload_offset == start + (if frame[start] == 0xfe { 6int } else { 10int })
        &&& message.payload_length == frame[start + 1] as int
        &&& message.payload_offset + message.payload_length <= frame.len()
      },
{
    let start = offset as usize;
    let available = length as usize;
    if available == 0 || start > frame.len() || available > frame.len() - start {
        return Err(InvalidReason::CarrierBounds);
    }

    let magic = get_magic(frame, start);
    let (header_length, signature_length, message_id) = if magic == MAVLINK_V1_MAGIC {
        if available < V1_MIN_FRAME_BYTES {
            return Err(InvalidReason::TruncatedOrTrailingBytes);
        }
        (V1_HEADER_BYTES, 0usize, get_v1_message_id(frame, start))
    } else if magic == MAVLINK_V2_MAGIC {
        if available < V2_MIN_FRAME_BYTES {
            return Err(InvalidReason::TruncatedOrTrailingBytes);
        }
        let incompat_flags = get_v2_incompat_flags(frame, start);
        if incompat_flags & !SUPPORTED_INCOMPAT_FLAGS != 0 {
            return Err(InvalidReason::UnsupportedIncompatibilityFlags);
        }
        // Account for the signature trailer; this parser does not authenticate it.
        let signature_length = if incompat_flags & MAVLINK_V2_SIGNED != 0 {
            SIGNATURE_BYTES
        } else { 0 };
        (V2_HEADER_BYTES, signature_length, get_v2_message_id(frame, start))
    } else {
        return Err(InvalidReason::UnsupportedVersion);
    };

    let payload_length = get_payload_length(frame, start);
    let expected_frame_length = header_length + payload_length + CHECKSUM_BYTES + signature_length;
    if available != expected_frame_length {
        return Err(InvalidReason::TruncatedOrTrailingBytes);
    }
    let (crc_extra, _minimum_payload_length, maximum_payload_length) = match dialect::metadata(message_id) {
        Some(metadata) => metadata,
        None => return Err(InvalidReason::UnknownMessage),
    };
    // Preserve the existing v1 length rule; v2 permits truncated trailing zero bytes.
    let invalid_payload_length = if magic == MAVLINK_V1_MAGIC {
        payload_length != maximum_payload_length as usize
    } else {
        payload_length > maximum_payload_length as usize
    };
    if invalid_payload_length {
        return Err(InvalidReason::TruncatedOrTrailingBytes);
    }

    let payload_offset = start + header_length;
    let checksum_offset = payload_offset + payload_length;
    let header_and_payload_crc = crc_fold(frame, start + CRC_START_OFFSET, checksum_offset, CRC_INITIAL);
    let expected_checksum = crc_accumulate_verified(crc_extra, header_and_payload_crc);
    if expected_checksum != get_checksum(frame, checksum_offset) {
        return Err(InvalidReason::Checksum);
    }
    Ok(Message { message_id, payload_offset, payload_length })
}

}
