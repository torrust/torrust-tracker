/// Connection id contains a timestamp in 4 bytes due to its 64 bits limit.
pub type Timestamp64 = u64;

pub fn timestamp_from_le_bytes(timestamp_bytes: &[u8]) -> Timestamp64 {
    // Little Endian
    let timestamp = u64::from_le_bytes([timestamp_bytes[0], timestamp_bytes[1], timestamp_bytes[2], timestamp_bytes[3], 0, 0, 0, 0]);
    timestamp
}

