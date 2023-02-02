use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use aquatic_udp_protocol::{Request, Response};
use rand::{thread_rng, Rng};
use torrust_tracker::udp::MAX_PACKET_SIZE;

use crate::common::udp::Client as UdpClient;

/// Creates a new generic UDP client connected to a generic UDP server
pub async fn new_udp_client_connected(remote_address: &SocketAddr) -> UdpClient {
    let local_address = loopback_socket_address(ephemeral_random_client_port());
    UdpClient::connected(remote_address, &local_address).await
}

/// Creates a new UDP tracker client connected to a UDP tracker server
pub async fn new_udp_tracker_client_connected(remote_address: &SocketAddr) -> Client {
    let udp_client = new_udp_client_connected(remote_address).await;
    Client { udp_client }
}

pub fn ephemeral_random_client_port() -> u16 {
    // todo: this may produce random test failures because two tests can try to bind the same port.
    // We could create a pool of available ports (with read/write lock)
    let mut rng = thread_rng();
    rng.gen_range(49152..65535)
}

fn loopback_socket_address(port: u16) -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
}

/// A UDP tracker client
pub struct Client {
    pub udp_client: UdpClient, // A generic UDP client
}

impl Client {
    pub async fn send(&self, request: Request) -> usize {
        // Write request into a buffer
        let request_buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(request_buffer);

        let request_data = match request.write(&mut cursor) {
            Ok(_) => {
                #[allow(clippy::cast_possible_truncation)]
                let position = cursor.position() as usize;
                let inner_request_buffer = cursor.get_ref();
                // Return slice which contains written request data
                &inner_request_buffer[..position]
            }
            Err(e) => panic!("could not write request to bytes: {e}."),
        };

        self.udp_client.send(request_data).await
    }

    pub async fn receive(&self) -> Response {
        let mut response_buffer = [0u8; MAX_PACKET_SIZE];

        let payload_size = self.udp_client.receive(&mut response_buffer).await;

        Response::from_bytes(&response_buffer[..payload_size], true).unwrap()
    }
}
