//! A UNIX 32-bit timestamp.

use std::num::TryFromIntError;

use super::timestamp_64::Timestamp64;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Timestamp32 {
    value: u32
}

impl Timestamp32 {
    pub fn from_timestamp_64(timestamp64: Timestamp64) -> Result<Self, TryFromIntError> {
        let timestamp32: u32 = u32::try_from(timestamp64)?;

        Ok(Self {
            value: timestamp32
        })
    }
    
    fn from_le_bytes(timestamp_bytes: &[u8]) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::udp::connection::timestamp_32::Timestamp32;

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
    fn it_should_be_instantiated_from_a_64_bit_unix_timestamp() {

        let timestamp = Timestamp32::from_timestamp_64(0u64);

        assert_eq!(timestamp.unwrap(), Timestamp32 { value: u32::MIN });
    }

    #[test]
    fn it_should_fail_trying_to_instantiate_from_a_64_bit_unix_timestamp_which_overflows_u32_range() {

        let timestamp = Timestamp32::from_timestamp_64((u32::MAX as u64) + 1u64);

        assert_eq!(timestamp.is_err(), true);
    }    
}