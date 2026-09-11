// Validation evidence: temporarily append these ghost lemmas inside verified.rs verus! block.
// All five passed against the refactored definitions; original formulas captured before editing.
proof fn crc_accumulate_spec_refactor_equivalent(byte: u8, crc: u16)
    ensures crc_accumulate_spec(byte, crc) == {
    let tmp0 = byte ^ (crc as u8);
    let tmp = tmp0 ^ (tmp0 << 4);
    (crc >> 8) ^ ((tmp as u16) << 8) ^ ((tmp as u16) << 3) ^ ((tmp as u16) >> 4)
}
{
}

proof fn u16_le_spec_refactor_equivalent(frame: Seq<u8>, at: int)
    ensures u16_le_spec(frame, at) == {
    (frame[at] as u16) | ((frame[at + 1] as u16) << 8)
}
{
}

proof fn u24_le_spec_refactor_equivalent(frame: Seq<u8>, at: int)
    ensures u24_le_spec(frame, at) == {
    (frame[at] as u32) | ((frame[at + 1] as u32) << 8) | ((frame[at + 2] as u32) << 16)
}
{
}

proof fn u32_le_spec_refactor_equivalent(frame: Seq<u8>, at: int)
    ensures u32_le_spec(frame, at) == {
    (frame[at] as u32) | ((frame[at + 1] as u32) << 8) |
      ((frame[at + 2] as u32) << 16) | ((frame[at + 3] as u32) << 24)
}
{
}

proof fn frame_valid_spec_refactor_equivalent(frame: Seq<u8>, offset: u16, length: u16)
    ensures frame_valid_spec(frame, offset, length) == {
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
{
}
