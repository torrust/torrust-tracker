use crate::udp::connection::client_id::ClientId;
use crate::udp::connection::timestamp_32::Timestamp32;

/// The data stored inside the connection id
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ConnectionIdData(pub [u8; 8]);

impl ConnectionIdData {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut sized_bytes_arr = [0u8; 8];

        sized_bytes_arr.copy_from_slice(&bytes[..8]);

        Self(sized_bytes_arr)
    }

    pub fn from_client_id_and_timestamp(client_id: ClientId, timestamp: Timestamp32) -> Self {
        let bytes_vec = [client_id.value, timestamp.0.to_le_bytes()].concat();

        Self::from_bytes(&bytes_vec)
    }

    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }

    pub fn client_id(&self) -> &[u8] {
        &self.0[..4]
    }

    pub fn timestamp(&self) -> u32 {
        u32::from_le_bytes([self.0[4], self.0[5], self.0[6], self.0[7]])
    }

    pub fn timestamp_as_bytes(&self) -> &[u8] {
        &self.0[4..]
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use crate::protocol::clock::current_timestamp;
    use crate::udp::connection::{connection_id_data::ConnectionIdData};
    use crate::udp::connection::client_id::ClientId;
    use crate::udp::connection::timestamp_32::Timestamp32;

    #[test]
    fn it_should_be_instantiated_from_a_client_id_and_timestamp32() {
        let client_id = ClientId::from_socket_address(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081));

        let expiration_timestamp: Timestamp32 = (current_timestamp() + 120).into();

        let connection_id = ConnectionIdData::from_client_id_and_timestamp(client_id, expiration_timestamp);

        assert_eq!(connection_id.client_id(), client_id.value.as_slice());
        assert_eq!(connection_id.timestamp(), expiration_timestamp.0)
    }

    #[test]
    fn it_should_be_instantiated_from_a_byte_array() {
        let bytes = [0, 0, 0, 0, 255, 255, 255, 255];

        let connection_id = ConnectionIdData::from_bytes(&bytes);

        assert_eq!(connection_id.as_bytes(), &bytes);
    }

    #[test]
    fn it_should_have_a_timestamp_that_equals_u32max() {
        let bytes = [0, 0, 0, 0, 255, 255, 255, 255];

        let connection_id = ConnectionIdData::from_bytes(&bytes);

        assert_eq!(connection_id.timestamp(), u32::MAX);
    }
}
