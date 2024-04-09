use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use aquatic_udp_protocol::{ConnectRequest, Request, Response, TransactionId};
use tokio::net::UdpSocket;
use tokio::time;
use tracing::debug;

use crate::shared::bit_torrent::tracker::udp::{source_address, MAX_PACKET_SIZE};

/// Default timeout for sending and receiving packets. And waiting for sockets
/// to be readable and writable.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct UdpClient {
    /// The socket to connect to
    pub socket: Arc<UdpSocket>,

    /// Timeout for sending and receiving packets
    pub timeout: Duration,
}

impl UdpClient {
    /// # Panics
    ///
    /// Will panic if the local address can't be bound.
    pub async fn bind(local_address: &str) -> Self {
        let valid_socket_addr = local_address
            .parse::<SocketAddr>()
            .unwrap_or_else(|_| panic!("{local_address} is not a valid socket address"));

        let socket = UdpSocket::bind(valid_socket_addr).await.unwrap();

        Self {
            socket: Arc::new(socket),
            timeout: DEFAULT_TIMEOUT,
        }
    }

    /// # Panics
    ///
    /// Will panic if can't connect to the socket.
    pub async fn connect(&self, remote_address: &str) {
        let valid_socket_addr = remote_address
            .parse::<SocketAddr>()
            .unwrap_or_else(|_| panic!("{remote_address} is not a valid socket address"));

        match self.socket.connect(valid_socket_addr).await {
            Ok(()) => debug!("Connected successfully"),
            Err(e) => panic!("Failed to connect: {e:?}"),
        }
    }

    /// # Panics
    ///
    /// Will panic if:
    ///
    /// - Can't write to the socket.
    /// - Can't send data.
    pub async fn send(&self, bytes: &[u8]) -> usize {
        debug!(target: "UDP client", "sending {bytes:?} ...");

        match time::timeout(self.timeout, self.socket.writable()).await {
            Ok(writable_result) => match writable_result {
                Ok(()) => (),
                Err(e) => panic!("{}", format!("IO error waiting for the socket to become readable: {e:?}")),
            },
            Err(e) => panic!("{}", format!("Timeout waiting for the socket to become readable: {e:?}")),
        };

        match time::timeout(self.timeout, self.socket.send(bytes)).await {
            Ok(send_result) => match send_result {
                Ok(size) => size,
                Err(e) => panic!("{}", format!("IO error during send: {e:?}")),
            },
            Err(e) => panic!("{}", format!("Send operation timed out: {e:?}")),
        }
    }

    /// # Panics
    ///
    /// Will panic if:
    ///
    /// - Can't read from the socket.
    /// - Can't receive data.
    pub async fn receive(&self, bytes: &mut [u8]) -> usize {
        debug!(target: "UDP client", "receiving ...");

        match time::timeout(self.timeout, self.socket.readable()).await {
            Ok(readable_result) => match readable_result {
                Ok(()) => (),
                Err(e) => panic!("{}", format!("IO error waiting for the socket to become readable: {e:?}")),
            },
            Err(e) => panic!("{}", format!("Timeout waiting for the socket to become readable: {e:?}")),
        };

        let size = match time::timeout(self.timeout, self.socket.recv(bytes)).await {
            Ok(recv_result) => match recv_result {
                Ok(size) => size,
                Err(e) => panic!("{}", format!("IO error during send: {e:?}")),
            },
            Err(e) => panic!("{}", format!("Receive operation timed out: {e:?}")),
        };

        debug!(target: "UDP client", "{size} bytes received {bytes:?}");

        size
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
#[derive(Debug)]
pub struct UdpTrackerClient {
    pub udp_client: UdpClient,
}

impl UdpTrackerClient {
    /// # Panics
    ///
    /// Will panic if can't write request to bytes.
    pub async fn send(&self, request: Request) -> usize {
        debug!(target: "UDP tracker client", "send request {request:?}");

        // Write request into a buffer
        let request_buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(request_buffer);

        let request_data = match request.write(&mut cursor) {
            Ok(()) => {
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

    /// # Panics
    ///
    /// Will panic if can't create response from the received payload (bytes buffer).
    pub async fn receive(&self) -> Response {
        let mut response_buffer = [0u8; MAX_PACKET_SIZE];

        let payload_size = self.udp_client.receive(&mut response_buffer).await;

        debug!(target: "UDP tracker client", "received {payload_size} bytes. Response {response_buffer:?}");

        Response::from_bytes(&response_buffer[..payload_size], true).unwrap()
    }
}

/// Creates a new `UdpTrackerClient` connected to a Udp Tracker server
pub async fn new_udp_tracker_client_connected(remote_address: &str) -> UdpTrackerClient {
    let udp_client = new_udp_client_connected(remote_address).await;
    UdpTrackerClient { udp_client }
}

/// Helper Function to Check if a UDP Service is Connectable
///
/// # Errors
///
/// It will return an error if unable to connect to the UDP service.
///
/// # Panics
pub async fn check(binding: &SocketAddr) -> Result<String, String> {
    debug!("Checking Service (detail): {binding:?}.");

    let client = new_udp_tracker_client_connected(binding.to_string().as_str()).await;

    let connect_request = ConnectRequest {
        transaction_id: TransactionId(123),
    };

    client.send(connect_request.into()).await;

    let process = move |response| {
        if matches!(response, Response::Connect(_connect_response)) {
            Ok("Connected".to_string())
        } else {
            Err("Did not Connect".to_string())
        }
    };

    let sleep = time::sleep(Duration::from_millis(2000));
    tokio::pin!(sleep);

    tokio::select! {
        () = &mut sleep => {
              Err("Timed Out".to_string())
        }
        response = client.receive() => {
              process(response)
        }
    }
}
