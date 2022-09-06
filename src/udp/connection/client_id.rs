//! ClientId is a unique ID for the UDP tracker client.
//! Currently implemented with a hash of the IP and port.
use std::net::{SocketAddr};

use blake3::{Hash, OUT_LEN};
use crate::ToBytesVec;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ClientId {
    pub value: [u8; 4],
}

impl ClientId {
    /// It generates the ID from the socket address (IP + port)
    pub fn from_socket_address(remote_address: &SocketAddr) -> Self {
        let unique_socket_id = generate_id_for_socket_address(remote_address);

        ClientId {
            value: unique_socket_id
        }
    }

    /// It generates the ID with a previously generated value
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut client_id = ClientId {
            value: [0u8; 4]
        };

        client_id.value.copy_from_slice(slice);
        client_id
    }
}

/// It generates an unique ID for a socket address (IP + port)
fn generate_id_for_socket_address(remote_address: &SocketAddr) -> [u8; 4] {
    let remote_address_as_bytes = remote_address.to_bytes_vec();

    let hashed_socket_addr = hash(&remote_address_as_bytes);

    get_first_four_bytes_from(hashed_socket_addr.as_bytes())
}

fn hash(bytes: &[u8]) -> Hash {
    blake3::hash(bytes)
}

fn get_first_four_bytes_from(bytes: &[u8; OUT_LEN]) -> [u8; 4] {
    let mut first_four_bytes: [u8; 4] = [0u8; 4]; // 4 bytes = 32 bits
    first_four_bytes.copy_from_slice(&bytes[..4]);
    first_four_bytes
}


#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, Ipv6Addr};
    use crate::ToBytesVec;

    use super::ClientId;

    #[test]
    fn client_id_should_generate_a_unique_four_byte_id_from_a_socket_address() {
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let client_id = ClientId::from_socket_address(&client_addr);

        assert_eq!(client_id.value, [58, 221, 231, 39]);
    }

    #[test]
    fn client_id_should_be_unique_for_clients_with_different_ip() {
        let client_1_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let client_2_with_different_socket_ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 8080);

        assert_ne!(ClientId::from_socket_address(&client_1_socket_addr), ClientId::from_socket_address(&client_2_with_different_socket_ip));
    }

    #[test]
    fn client_id_should_be_unique_for_clients_with_different_port() {
        let client_1_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let client_2_with_different_socket_port = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081);

        assert_ne!(ClientId::from_socket_address(&client_1_socket_addr), ClientId::from_socket_address(&client_2_with_different_socket_port));
    }

    #[test]
    fn ipv4_address_should_be_converted_to_a_byte_vector() {
        let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let bytes = ip_address.to_bytes_vec();

        assert_eq!(bytes, vec![127, 0, 0, 1]); // 4 bytes
    }

    #[test]
    fn ipv6_address_should_be_converted_to_a_byte_vector() {
        let ip_address = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        let bytes = ip_address.to_bytes_vec();

        assert_eq!(bytes, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]); // 16 bytes
    }
}
