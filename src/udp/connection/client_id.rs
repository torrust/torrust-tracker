//! ClientId is a unique ID for the UDP tracker client.
//! Currently implemented with a hash of the socket, i.e the IP and port.
use std::hash::Hash;
use std::hash::Hasher;
use std::net::SocketAddr;

#[derive(PartialEq, Debug, Clone)]
pub struct ClientId {
    value: [u8; 4],
}

pub trait Make<T: Default + Hasher> {
    fn new(socket: &SocketAddr) -> Self;

    fn hash(socket: &SocketAddr) -> [u8;8] {
        let mut hasher = T::default();
        socket.hash(&mut hasher);

        hasher.finish().to_le_bytes()
    }
}

impl<T: Default + Hasher> Make<T> for ClientId {
    fn new(socket: &SocketAddr) -> Self {
        let hash = <ClientId as Make<T>>::hash(socket);

        let mut truncated_hash: [u8; 4] = [0u8; 4];
        truncated_hash.copy_from_slice(&hash[..4]);

        ClientId {
            value: truncated_hash,
        }
    }
}

impl ClientId {
    /// It generates the ID with a previously generated value
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut client_id = ClientId {
            value: [0u8; 4]
        };
        client_id.value.copy_from_slice(bytes);
        client_id
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let bytes: [u8; 4] = self.value.clone().try_into().unwrap();
        bytes
    }
}

#[cfg(test)]
mod tests {
    use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, collections::hash_map::DefaultHasher};

    use super::{ClientId, Make};

    #[test]
    fn it_should_be_a_hash_of_the_socket() {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let id: ClientId = Make::<DefaultHasher>::new(&socket);

        assert_eq!(id.value, [213, 195, 130, 185]);
    }

    #[test]
    fn id_should_be_converted_to_a_byte_array() {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let id: ClientId = Make::<DefaultHasher>::new(&socket);

        assert_eq!(id.to_bytes(), [213, 195, 130, 185]);
    }

    #[test]
    fn id_should_be_instantiate_from_a_previously_generated_value() {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let id: ClientId = Make::<DefaultHasher>::new(&socket);
        let bytes = id.to_bytes();

        assert_eq!(ClientId::from_bytes(&bytes), id);
    }

    #[test]
    fn it_should_be_unique_with_different_socket_ips() {
        let socket_1 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let socket_2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 8080);

        assert_ne!(
            <ClientId as Make::<DefaultHasher>>::new(&socket_1),
            <ClientId as Make::<DefaultHasher>>::new(&socket_2)
        );
    }

    #[test]
    fn it_should_be_unique_with_different_socket_ports() {
        let socket_1 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let socket_2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081);

        assert_ne!(
            <ClientId as Make::<DefaultHasher>>::new(&socket_1),
            <ClientId as Make::<DefaultHasher>>::new(&socket_2)
        );
    }
}