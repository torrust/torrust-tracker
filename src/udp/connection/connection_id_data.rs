use super::{client_id::ClientId, timestamp_32::Timestamp32};

/// The data stored inside the connection id
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ConnectionIdData {
    pub client_id: ClientId,
    pub expiration_timestamp: Timestamp32
}

impl ConnectionIdData {
    pub fn from_bytes(bytes: &[u8; 8]) -> Self {
        let client_id = Self::extract_client_id(bytes);
        let expiration_timestamp = Self::extract_timestamp(bytes);
        Self {
            client_id,
            expiration_timestamp
        }
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let connection_id: Vec<u8> = [
            self.client_id.to_bytes().as_slice(),
            self.expiration_timestamp.to_le_bytes().as_slice(),
        ].concat();
    
        let connection_as_array: [u8; 8] = connection_id.try_into().unwrap();
    
        connection_as_array
    }

    fn extract_timestamp(decrypted_connection_id: &[u8; 8]) -> Timestamp32 {
        let timestamp_bytes = &decrypted_connection_id[4..];
        let timestamp = Timestamp32::from_le_bytes(timestamp_bytes);
        timestamp
    }
    
    fn extract_client_id(decrypted_connection_id: &[u8; 8]) -> ClientId {
        ClientId::from_slice(&decrypted_connection_id[..4])
    }    
}

#[cfg(test)]
mod tests {
    use crate::udp::connection::{connection_id_data::ConnectionIdData, client_id::ClientId};


    #[test]
    fn it_contains_a_client_id() {

        let connection_id = ConnectionIdData {
            client_id: ClientId::from_slice(&[0u8; 4]),
            expiration_timestamp: 0u32.into(),
        };

        assert_eq!(connection_id.client_id, ClientId::from_slice(&[0u8; 4]));
    }

    #[test]
    fn it_contains_an_expiration_timestamp() {

        let connection_id = ConnectionIdData {
            client_id: ClientId::from_slice(&[0u8; 4]),
            expiration_timestamp: 0u32.into(),
        };

        assert_eq!(connection_id.expiration_timestamp, 0u32.into());
    }

    #[test]
    fn it_should_be_converted_to_a_byte_array() {

        let connection_id = ConnectionIdData {
            client_id: ClientId::from_slice(&[0u8; 4]),
            expiration_timestamp: (u32::MAX).into(),
        };

        assert_eq!(connection_id.to_bytes(), [0, 0, 0, 0, 255, 255, 255, 255]);
    }

    #[test]
    fn it_should_be_instantiated_from_a_byte_array() {

        let connection_id = ConnectionIdData::from_bytes(&[0, 0, 0, 0, 255, 255, 255, 255]);

        let expected_connection_id = ConnectionIdData {
            client_id: ClientId::from_slice(&[0, 0, 0, 0]),
            expiration_timestamp: (u32::MAX).into(),
        };

        assert_eq!(connection_id, expected_connection_id);
    }
}
