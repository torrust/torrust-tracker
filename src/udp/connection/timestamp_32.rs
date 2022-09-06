//! A UNIX 32-bit timestamp.

use super::timestamp_64::Timestamp64;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Timestamp32(pub u32);

impl Timestamp32 {
    pub fn from_le_bytes(timestamp_bytes: &[u8]) -> Self {
        let timestamp = u32::from_le_bytes([timestamp_bytes[0], timestamp_bytes[1], timestamp_bytes[2], timestamp_bytes[3]]);
        Self(timestamp)
    }
}

impl From<u32> for Timestamp32 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Timestamp64> for Timestamp32 {
    fn from(value: Timestamp64) -> Self {
        let mut bytes_array = [0u8; 4];

        bytes_array.copy_from_slice(&value.to_le_bytes()[..4]);

        Self(u32::from_le_bytes(bytes_array))
    }
}

impl Into<Timestamp64> for Timestamp32 {
    fn into(self) -> Timestamp64 {
        u64::from(self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::udp::connection::{timestamp_32::Timestamp32, timestamp_64::Timestamp64};

    #[test]
    fn it_should_be_instantiated_from_a_four_byte_array_in_little_indian() {

        let min_timestamp = Timestamp32::from_le_bytes(&[0u8, 0u8, 0u8, 0u8]);

        assert_eq!(min_timestamp, Timestamp32(u32::MIN));

        let max_timestamp = Timestamp32::from_le_bytes(&[255u8, 255u8, 255u8, 255u8]);

        assert_eq!(max_timestamp, Timestamp32(u32::MAX));
    }

    #[test]
    fn it_should_be_converted_to_a_four_byte_array_in_little_indian() {

        let min_timestamp = Timestamp32(u32::MIN);

        assert_eq!(min_timestamp.0.to_le_bytes(), [0u8, 0u8, 0u8, 0u8]);

        let max_timestamp = Timestamp32(u32::MAX);

        assert_eq!(max_timestamp.0.to_le_bytes(), [255u8, 255u8, 255u8, 255u8]);
    }

    #[test]
    fn it_should_be_converted_from_a_64_bit_unix_timestamp() {

        let timestamp32: Timestamp32 = 0u64.try_into().unwrap();

        assert_eq!(timestamp32, Timestamp32(u32::MIN));
    }

    #[test]
    fn it_should_be_converted_from_a_64_bit_unix_timestamp_with_u32_max() {

        let timestamp64 = u32::MAX as u64;
        let timestamp32: Timestamp32 = Timestamp32::from(timestamp64);

        assert_eq!(timestamp32, Timestamp32(u32::MAX));
    }

    #[test]
    fn it_should_be_converted_from_a_64_bit_unix_timestamp_with_u64_max() {

        let timestamp64 = u64::MAX;
        let timestamp32: Timestamp32 = Timestamp32::from(timestamp64);

        assert_eq!(timestamp32, Timestamp32(u32::MAX));
    }

    #[test]
    fn it_should_be_converted_from_a_u32() {

        let timestamp32: Timestamp32 = u32::MIN.into();

        assert_eq!(timestamp32, Timestamp32(u32::MIN));
    }

    #[test]
    fn it_should_be_converted_to_a_timestamp_64() {

        let min_timestamp_32 = Timestamp32(u32::MIN);

        let min_timestamp_64: Timestamp64 = min_timestamp_32.into();

        assert_eq!(min_timestamp_64, u32::MIN as u64);


        let max_timestamp_32 = Timestamp32(u32::MAX);

        let max_timestamp_64: Timestamp64 = max_timestamp_32.into();

        assert_eq!(max_timestamp_64, u32::MAX as u64);
    }
}
