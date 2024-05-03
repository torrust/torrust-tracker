use core::result::Result::{Err, Ok};
use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use aquatic_udp_protocol::{ConnectRequest, Request, Response, TransactionId};
use log::debug;
use tokio::net::UdpSocket;
use tokio::time;
use zerocopy::network_endian::I32;

use crate::console::clients::udp::checker::ClientError;
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
    /// # Errors
    ///
    /// Will return error if the local address can't be bound.
    ///
    pub async fn bind(local_address: &str) -> Result<Self> {
        let socket_addr = local_address
            .parse::<SocketAddr>()
            .context(format!("{local_address} is not a valid socket address"))?;

        let socket = UdpSocket::bind(socket_addr).await?;

        let udp_client = Self {
            socket: Arc::new(socket),
            timeout: DEFAULT_TIMEOUT,
        };
        Ok(udp_client)
    }

    /// # Errors
    ///
    /// Will return error if can't connect to the socket.
    pub async fn connect(&self, remote_address: &str) -> Result<()> {
        let socket_addr = remote_address
            .parse::<SocketAddr>()
            .context(format!("{remote_address} is not a valid socket address"))?;

        match self.socket.connect(socket_addr).await {
            Ok(()) => {
                debug!("Connected successfully");
                Ok(())
            }
            Err(e) => Err(anyhow!("Failed to connect: {e:?}")),
        }
    }

    /// # Errors
    ///
    /// Will return error if:
    ///
    /// - Can't write to the socket.
    /// - Can't send data.
    pub async fn send(&self, bytes: &[u8]) -> Result<usize> {
        debug!(target: "UDP client", "sending {bytes:?} ...");

        match time::timeout(self.timeout, self.socket.writable()).await {
            Ok(writable_result) => {
                match writable_result {
                    Ok(()) => (),
                    Err(e) => return Err(anyhow!("IO error waiting for the socket to become readable: {e:?}")),
                };
            }
            Err(_) => bail!(ClientError::ConnectionTimeout),
        };

        match time::timeout(self.timeout, self.socket.send(bytes)).await {
            Ok(send_result) => match send_result {
                Ok(size) => Ok(size),
                Err(e) => Err(anyhow!("IO error during send: {e:?}")),
            },
            Err(_) => bail!(ClientError::ConnectionTimeout),
        }
    }

    /// # Errors
    ///
    /// Will return error if:
    ///
    /// - Can't read from the socket.
    /// - Can't receive data.
    ///
    /// # Panics
    ///
    pub async fn receive(&self, bytes: &mut [u8]) -> Result<usize> {
        debug!(target: "UDP client", "receiving ...");

        match time::timeout(self.timeout, self.socket.readable()).await {
            Ok(readable_result) => {
                match readable_result {
                    Ok(()) => (),
                    Err(e) => return Err(anyhow!("IO error waiting for the socket to become readable: {e:?}")),
                };
            }
            Err(_) => bail!(ClientError::ConnectionTimeout),
        };

        let size_result = match time::timeout(self.timeout, self.socket.recv(bytes)).await {
            Ok(recv_result) => match recv_result {
                Ok(size) => Ok(size),
                Err(e) => Err(anyhow!("IO error during send: {e:?}")),
            },
            Err(_) => bail!(ClientError::ConnectionTimeout),
        };

        if size_result.is_ok() {
            let size = size_result.as_ref().unwrap();
            debug!(target: "UDP client", "{size} bytes received {bytes:?}");
            size_result
        } else {
            size_result
        }
    }
}

/// Creates a new `UdpClient` connected to a Udp server
///
/// # Errors
///
/// Will return any errors present in the call stack
///
pub async fn new_udp_client_connected(remote_address: &str) -> Result<UdpClient> {
    let port = 0; // Let OS choose an unused port.
    let client = UdpClient::bind(&source_address(port)).await?;
    client.connect(remote_address).await?;
    Ok(client)
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct UdpTrackerClient {
    pub udp_client: UdpClient,
}

impl UdpTrackerClient {
    /// # Errors
    ///
    /// Will return error if can't write request to bytes.
    pub async fn send(&self, request: Request) -> Result<usize> {
        debug!(target: "UDP tracker client", "send request {request:?}");

        // Write request into a buffer
        let request_buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(request_buffer);

        let request_data_result = match request.write_bytes(&mut cursor) {
            Ok(()) => {
                #[allow(clippy::cast_possible_truncation)]
                let position = cursor.position() as usize;
                let inner_request_buffer = cursor.get_ref();
                // Return slice which contains written request data
                Ok(&inner_request_buffer[..position])
            }
            Err(e) => Err(anyhow!("could not write request to bytes: {e}.")),
        };

        let request_data = request_data_result?;

        self.udp_client.send(request_data).await
    }

    /// # Errors
    ///
    /// Will return error if can't create response from the received payload (bytes buffer).
    pub async fn receive(&self) -> Result<Response> {
        let mut response_buffer = [0u8; MAX_PACKET_SIZE];

        let payload_size = self.udp_client.receive(&mut response_buffer).await?;

        debug!(target: "UDP tracker client", "received {payload_size} bytes. Response {response_buffer:?}");

        let response = Response::parse_bytes(&response_buffer[..payload_size], true)?;

        Ok(response)
    }
}

/// Creates a new `UdpTrackerClient` connected to a Udp Tracker server
///
/// # Errors
///
/// Will return any errors present in the call stack
///
pub async fn new_udp_tracker_client_connected(remote_address: &str) -> Result<UdpTrackerClient> {
    let udp_client = new_udp_client_connected(remote_address).await?;
    let udp_tracker_client = UdpTrackerClient { udp_client };
    Ok(udp_tracker_client)
}

/// Helper Function to Check if a UDP Service is Connectable
///
/// # Panics
///
/// It will return an error if unable to connect to the UDP service.
///
/// # Errors
///
pub async fn check(binding: &SocketAddr) -> Result<String, String> {
    debug!("Checking Service (detail): {binding:?}.");

    match new_udp_tracker_client_connected(binding.to_string().as_str()).await {
        Ok(client) => {
            let connect_request = ConnectRequest {
                transaction_id: TransactionId(I32::new(123)),
            };

            // client.send() return usize, but doesn't use here
            match client.send(connect_request.into()).await {
                Ok(_) => (),
                Err(e) => debug!("Error: {e:?}."),
            };

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
                      process(response.unwrap())
                }
            }
        }
        Err(e) => Err(format!("{e:?}")),
    }
}
