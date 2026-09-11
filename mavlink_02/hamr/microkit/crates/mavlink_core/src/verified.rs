use vstd::prelude::*;

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
    let mut tmp = byte ^ (crc as u8);
    tmp ^= tmp << 4;
    (crc >> 8) ^ ((tmp as u16) << 8) ^ ((tmp as u16) << 3) ^ ((tmp as u16) >> 4)
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
    if available == 0 || start > frame.len() || available > frame.len() - start { return Err(InvalidReason::CarrierBounds); }
    let magic = frame[start];
    let (header, signature, id) = if magic == 0xfe {
        if available < 8 { return Err(InvalidReason::TruncatedOrTrailingBytes); }
        (6usize, 0usize, frame[start + 5] as u32)
    } else if magic == 0xfd {
        if available < 12 { return Err(InvalidReason::TruncatedOrTrailingBytes); }
        let incompat = frame[start + 2];
        if incompat & !1u8 != 0 { return Err(InvalidReason::UnsupportedIncompatibilityFlags); }
        let id = frame[start + 7] as u32 | ((frame[start + 8] as u32) << 8) |
          ((frame[start + 9] as u32) << 16);
        (10usize, if incompat & 1u8 != 0 { 13 } else { 0 }, id)
    } else { return Err(InvalidReason::UnsupportedVersion); };
    let payload = frame[start + 1] as usize;
    if available != header + payload + 2 + signature { return Err(InvalidReason::TruncatedOrTrailingBytes); }
    let meta = match dialect::metadata(id) { Some(value) => value, None => return Err(InvalidReason::UnknownMessage) };
    if if magic == 0xfe { payload != meta.2 as usize }
       else { payload > meta.2 as usize } { return Err(InvalidReason::TruncatedOrTrailingBytes); }
    let checksum_at = start + header + payload;
    let crc = crc_accumulate_verified(meta.0, crc_fold(frame, start + 1, checksum_at, 0xffff));
    let received = frame[checksum_at] as u16 | ((frame[checksum_at + 1] as u16) << 8);
    if crc != received { return Err(InvalidReason::Checksum); }
    Ok(Message { message_id: id, payload_offset: start + header, payload_length: payload })
}

}
