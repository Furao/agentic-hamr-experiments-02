// Validation evidence: temporarily append inside the application verus! block.
// Both unrestricted equivalence lemmas verified against the refactored definitions.
  proof fn firmware_flash_refactor_equivalent(frame: Seq<u8>, offset: u16, length: u16)
    ensures firmware_flash_spec(frame, offset, length) == {
      mavlink_core::verified::frame_valid_spec(frame, offset, length) && {
        let start = offset as int;
        let header: int = if frame[start] == 0xfe { 6 } else { 10 };
        let payload = frame[start + 1] as int;
        let id: u32 = if frame[start] == 0xfe { frame[start + 5] as u32 } else { mavlink_core::verified::u24_le_spec(frame, start + 7) };
        ((id == 75 || id == 76) && payload >= 30 &&
          mavlink_core::verified::u16_le_spec(frame, start + header + 28) == 42650u16) ||
          (id == 11004 && payload >= 8 &&
            mavlink_core::verified::u32_le_spec(frame, start + header + 4) == 7u32)
      }
  }
  {}

  proof fn payload_field_refactor_equivalent(frame: Seq<u8>, payload_offset: int)
    ensures
      command_field_spec(frame, payload_offset) ==
        mavlink_core::verified::u16_le_spec(frame, payload_offset + 28),
      secure_operation_spec(frame, payload_offset) ==
        mavlink_core::verified::u32_le_spec(frame, payload_offset + 4),
  {}

