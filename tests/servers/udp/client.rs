use std::io::Cursor;
use std::sync::Arc;

use aquatic_udp_protocol::{Request, Response};
use tokio::net::UdpSocket;
use torrust_tracker::servers::udp::MAX_PACKET_SIZE;

use crate::servers::udp::source_address;

#[allow(clippy::module_name_repetitions)]
pub struct UdpClient {
    pub socket: Arc<UdpSocket>,
}

impl UdpClient {
    pub async fn bind(local_address: &str) -> Self {
        let socket = UdpSocket::bind(local_address).await.unwrap();
        Self {
            socket: Arc::new(socket),
        }
    }

    pub async fn connect(&self, remote_address: &str) {
        self.socket.connect(remote_address).await.unwrap();
    }

    pub async fn send(&self, bytes: &[u8]) -> usize {
        self.socket.writable().await.unwrap();
        self.socket.send(bytes).await.unwrap()
    }

    pub async fn receive(&self, bytes: &mut [u8]) -> usize {
        self.socket.readable().await.unwrap();
        self.socket.recv(bytes).await.unwrap()
    }
}

/// Creates a new `UdpClient` connected to a Udp server
pub async fn new_udp_client_connected(remote_address: &str) -> UdpClient {
    let port = 0; // Let OS choose an unused port.
    let client = UdpClient::bind(&source_address(port)).await;
    client.connect(remote_address).await;
    client
}

#[allow(clippy::module_name_repetitions)]
pub struct UdpTrackerClient {
    pub udp_client: UdpClient,
}

impl UdpTrackerClient {
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

/// Creates a new `UdpTrackerClient` connected to a Udp Tracker server
pub async fn new_udp_tracker_client_connected(remote_address: &str) -> UdpTrackerClient {
    let udp_client = new_udp_client_connected(remote_address).await;
    UdpTrackerClient { udp_client }
}
