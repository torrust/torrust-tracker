use super::{client_id::ClientId, timestamp_32::Timestamp32};

/// The data stored inside the connection id
#[derive(PartialEq, Debug, Clone)]
pub struct ConnectionIdData {
    pub client_id: ClientId,
    pub expiration_timestamp: Timestamp32
}

impl ConnectionIdData {
    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    pub fn expiration_timestamp(&self) -> &Timestamp32 {
        &self.expiration_timestamp
    }
}

#[cfg(test)]
mod tests {
    use crate::udp::connection::{connection_id_data::ConnectionIdData, client_id::ClientId};


    #[test]
    fn it_contains_a_client_id() {

        let connection_id = ConnectionIdData {
            client_id: ClientId::from_bytes(&[0u8; 4]),
            expiration_timestamp: 0u32.into(),
        };

        assert_eq!(connection_id.client_id, ClientId::from_bytes(&[0u8; 4]));
    }

    #[test]
    fn it_contains_an_expiration_timestamp() {

        let connection_id = ConnectionIdData {
            client_id: ClientId::from_bytes(&[0u8; 4]),
            expiration_timestamp: 0u32.into(),
        };

        assert_eq!(connection_id.expiration_timestamp, 0u32.into());
    }
}
