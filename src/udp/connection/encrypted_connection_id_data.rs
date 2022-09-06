#[derive(PartialEq, Debug, Copy, Clone)]
pub struct EncryptedConnectionIdData {
    pub bytes: [u8; 8]
}

impl EncryptedConnectionIdData {
    pub fn from_encrypted_bytes(encrypted_bytes: [u8; 8]) -> Self {
        Self { bytes: encrypted_bytes }
    }
}

impl Into<i64> for EncryptedConnectionIdData {
    fn into(self) -> i64 {
        i64::from_le_bytes(self.bytes)
    }
}

impl From<i64> for EncryptedConnectionIdData {
    fn from(value: i64) -> Self {
        Self { bytes: value.to_le_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use crate::udp::connection::encrypted_connection_id_data::EncryptedConnectionIdData;


    #[test]
    fn it_should_be_generated_from_the_encrypted_connection_id_data() {

        let encrypted_data = EncryptedConnectionIdData::from_encrypted_bytes([0u8; 8]);

        assert_eq!(encrypted_data, EncryptedConnectionIdData { bytes: [0u8; 8]});
    }

    #[test]
    fn it_should_be_converted_into_a_i64() {

        let encrypted_data: i64 = EncryptedConnectionIdData::from_encrypted_bytes([0u8; 8]).into();

        assert_eq!(encrypted_data, 0i64);
    }

    #[test]
    fn it_should_be_converted_from_a_i64() {

        let encrypted_data: EncryptedConnectionIdData = 0i64.into();

        assert_eq!(encrypted_data, EncryptedConnectionIdData { bytes: [0u8; 8]});
    }
}

