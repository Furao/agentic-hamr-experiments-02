//! Wire-format constants from the bundled MAVLink Packet Serialization guide.
//! Offsets are relative to the magic byte; header lengths include that byte.

use vstd::prelude::*;

verus! {

pub const MAVLINK_V1_MAGIC: u8 = 0xfe;
pub const MAVLINK_V2_MAGIC: u8 = 0xfd;
pub const MAVLINK_V2_SIGNED: u8 = 0x01;
pub const SUPPORTED_INCOMPAT_FLAGS: u8 = MAVLINK_V2_SIGNED;

pub const MAGIC_OFFSET: usize = 0;
pub const PAYLOAD_LENGTH_OFFSET: usize = 1;
pub const V2_INCOMPAT_FLAGS_OFFSET: usize = 2;
pub const V1_MESSAGE_ID_OFFSET: usize = 5;
pub const V2_MESSAGE_ID_OFFSET: usize = 7;
pub const V2_MESSAGE_ID_BYTES: usize = 3;
pub const V1_HEADER_BYTES: usize = 6;
pub const V2_HEADER_BYTES: usize = 10;
pub const CHECKSUM_BYTES: usize = 2;
pub const SIGNATURE_BYTES: usize = 13;
pub const V1_MIN_FRAME_BYTES: usize = V1_HEADER_BYTES + CHECKSUM_BYTES;
pub const V2_MIN_FRAME_BYTES: usize = V2_HEADER_BYTES + CHECKSUM_BYTES;

// The CRC covers the header after magic, then payload, then the dialect CRC_EXTRA.
pub const CRC_START_OFFSET: usize = PAYLOAD_LENGTH_OFFSET;
pub const CRC_INITIAL: u16 = 0xffff;
// Bit counts for shifting by whole bytes.
pub const ONE_BYTE_SHIFT: u32 = 8;
pub const TWO_BYTE_SHIFT: u32 = 16;
pub const THREE_BYTE_SHIFT: u32 = 24;
// Shifts in the byte-wise CRC-16/MCRF4XX recurrence (reflected polynomial 0x8408).
pub const CRC_NIBBLE_SHIFT: u32 = 4;
pub const CRC_POLYNOMIAL_MIX_SHIFT: u32 = 3;

}
