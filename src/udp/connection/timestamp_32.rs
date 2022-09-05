//! A UNIX 32-bit timestamp.

use std::num::TryFromIntError;

use super::timestamp_64::Timestamp64;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Timestamp32 {
    pub value: u32
}

impl Timestamp32 {
    pub fn from_le_bytes(timestamp_bytes: &[u8]) -> Self {
        // Little Endian
        let timestamp = u32::from_le_bytes([timestamp_bytes[0], timestamp_bytes[1], timestamp_bytes[2], timestamp_bytes[3]]);
        Self {
            value: timestamp
        }
    }

    pub fn to_le_bytes(self: Self) -> [u8; 4] {
        // Little Endian
        let mut bytes: [u8; 4] = [0u8; 4];
        bytes.copy_from_slice(&self.value.to_le_bytes()[..4]);
        bytes
    }
}

impl From<u32> for Timestamp32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl TryFrom<Timestamp64> for Timestamp32 {
    type Error = TryFromIntError;

    fn try_from(value: Timestamp64) -> Result<Self, Self::Error> {
        let timestamp32: u32 = u32::try_from(value)?;

        Ok(Self {
            value: timestamp32
        })
    }
}

impl Into<Timestamp64> for Timestamp32 {
    fn into(self) -> Timestamp64 {
        u64::from(self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::udp::connection::{timestamp_32::Timestamp32, timestamp_64::Timestamp64};

    #[test]
    fn it_should_be_instantiated_from_a_four_byte_array_in_little_indian() {

        let min_timestamp = Timestamp32::from_le_bytes(&[0u8, 0u8, 0u8, 0u8]);

        assert_eq!(min_timestamp, Timestamp32 { value: u32::MIN });

        let max_timestamp = Timestamp32::from_le_bytes(&[255u8, 255u8, 255u8, 255u8]);

        assert_eq!(max_timestamp, Timestamp32 { value: u32::MAX });        
    }

    #[test]
    fn it_should_be_converted_to_a_four_byte_array_in_little_indian() {

        let min_timestamp = Timestamp32 { value: u32::MIN };

        assert_eq!(min_timestamp.to_le_bytes(), [0u8, 0u8, 0u8, 0u8]);

        let max_timestamp = Timestamp32 { value: u32::MAX };

        assert_eq!(max_timestamp.to_le_bytes(), [255u8, 255u8, 255u8, 255u8]);        
    }

    #[test]
    fn it_should_be_converted_from_a_64_bit_unix_timestamp() {

        let timestamp32: Timestamp32 = 0u64.try_into().unwrap();

        assert_eq!(timestamp32, Timestamp32 { value: u32::MIN });
    }

    #[test]
    fn it_should_fail_trying_to_convert_it_from_a_64_bit_unix_timestamp_which_overflows_u32_range() {

        let out_of_range_value = (u32::MAX as u64) + 1;

        let timestamp32: Result<Timestamp32, _> = out_of_range_value.try_into();

        assert_eq!(timestamp32.is_err(), true);
    }

    #[test]
    fn it_should_be_converted_from_a_u32() {

        let timestamp32: Timestamp32 = u32::MIN.into();

        assert_eq!(timestamp32, Timestamp32 { value: u32::MIN });
    }

    #[test]
    fn it_should_be_converted_to_a_timestamp_64() {

        let min_timestamp_32 = Timestamp32 { value: u32::MIN };

        let min_timestamp_64: Timestamp64 = min_timestamp_32.into();

        assert_eq!(min_timestamp_64, u32::MIN as u64);


        let max_timestamp_32 = Timestamp32 { value: u32::MAX };

        let max_timestamp_64: Timestamp64 = max_timestamp_32.into();

        assert_eq!(max_timestamp_64, u32::MAX as u64);
    }
}