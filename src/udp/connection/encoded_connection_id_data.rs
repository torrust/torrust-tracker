use super::{client_id::ClientId, timestamp_32::Timestamp32, connection_id_data::ConnectionIdData};

/// The encoded version of ConnectionIdData to be use in the UPD tracker package field "connection_id"
pub struct EncodedConnectionIdData([u8; 8]);

impl EncodedConnectionIdData {
    pub fn from_bytes(bytes: &[u8; 8]) -> Self {
        let mut sized_bytes_arr = [0u8; 8];
        sized_bytes_arr.copy_from_slice(&bytes[..8]);
        Self(sized_bytes_arr)
    }

    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }

    fn extract_client_id(&self) -> ClientId {
        ClientId::from_bytes(&self.0[..4])
    }

    fn extract_expiration_timestamp(&self) -> Timestamp32 {
        let timestamp_bytes = &self.0[4..];
        let timestamp = Timestamp32::from_le_bytes(timestamp_bytes);
        timestamp
    }    
}

impl From<EncodedConnectionIdData> for ConnectionIdData {
    fn from(encoded_connection_id_data: EncodedConnectionIdData) -> Self {
        Self {
            client_id: encoded_connection_id_data.extract_client_id(),
            expiration_timestamp: encoded_connection_id_data.extract_expiration_timestamp()
        }
    }
}

impl From<ConnectionIdData> for EncodedConnectionIdData {
    fn from(connection_id_data: ConnectionIdData) -> Self {
        let byte_vec: Vec<u8> = [
            connection_id_data.client_id.to_bytes().as_slice(),
            connection_id_data.expiration_timestamp.to_le_bytes().as_slice(),
        ].concat();
        let bytes: [u8; 8] = byte_vec.try_into().unwrap();
        EncodedConnectionIdData::from_bytes(&bytes)
    }
}
