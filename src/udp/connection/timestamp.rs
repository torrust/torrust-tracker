/// Connection id contains a timestamp in 4 bytes due to its 64 bits limit.
pub type Timestamp = u64;

pub fn timestamp_to_le_bytes(current_timestamp: Timestamp) -> [u8; 4] {
    // Little Endian
    let mut bytes: [u8; 4] = [0u8; 4];
    bytes.copy_from_slice(&current_timestamp.to_le_bytes()[..4]);
    bytes
}

pub fn timestamp_from_le_bytes(timestamp_bytes: &[u8]) -> Timestamp {
    // Little Endian
    let timestamp = u64::from_le_bytes([timestamp_bytes[0], timestamp_bytes[1], timestamp_bytes[2], timestamp_bytes[3], 0, 0, 0, 0]);
    timestamp
}

