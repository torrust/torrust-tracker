use core::result::Result::{Err, Ok};
use std::io::Cursor;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use tokio::net::UdpSocket;
use tokio::time;
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_udp_protocol::{ConnectRequest, Request, Response, TransactionId};
use zerocopy::byteorder::network_endian::I32;

use super::Error;

pub const UDP_CLIENT_LOG_TARGET: &str = "UDP CLIENT";

const DEFAULT_UDP_TIMEOUT: Duration = Duration::from_secs(5);

/// Fits any UDP datagram, so the client never truncates a tracker's reply. A
/// tracker's packet size (`MAX_PACKET_SIZE`) limits what it reads, not what a
/// client may receive.
const RECEIVE_BUFFER_SIZE: usize = 65_535;

#[allow(clippy::module_name_repetitions, reason = "public type identifies the UDP protocol client")]
#[derive(Debug, Clone)]
pub struct UdpClient {
    /// The socket to connect to
    pub socket: Arc<UdpSocket>,

    /// Timeout for sending and receiving packets
    pub timeout: Duration,
}

impl UdpClient {
    /// Creates a new `UdpClient` bound to the default port and ipv6 address
    ///
    /// # Errors
    ///
    /// Will return error if unable to bind to any port or ip address.
    ///
    async fn bound_to_default_ipv4(timeout: Duration) -> Result<Self, Error> {
        let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0);

        Self::bound(addr, timeout).await
    }

    /// Creates a new `UdpClient` bound to the default port and ipv6 address
    ///
    /// # Errors
    ///
    /// Will return error if unable to bind to any port or ip address.
    ///
    async fn bound_to_default_ipv6(timeout: Duration) -> Result<Self, Error> {
        let addr = SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 0);

        Self::bound(addr, timeout).await
    }

    /// Creates a new `UdpClient` connected to a Udp server
    ///
    /// # Errors
    ///
    /// Will return any errors present in the call stack
    ///
    pub async fn connected(remote_addr: SocketAddr, timeout: Duration) -> Result<Self, Error> {
        let client = if remote_addr.is_ipv4() {
            Self::bound_to_default_ipv4(timeout).await?
        } else {
            Self::bound_to_default_ipv6(timeout).await?
        };

        client.connect(remote_addr).await?;
        Ok(client)
    }

    /// Creates a `[UdpClient]` bound to a Socket.
    ///
    /// # Panics
    ///
    /// Panics if unable to get the `local_addr` of the bound socket.
    ///
    /// # Errors
    ///
    /// This function will return an error if the binding takes to long
    /// or if there is an underlying OS error.
    pub async fn bound(addr: SocketAddr, timeout: Duration) -> Result<Self, Error> {
        tracing::trace!(target: UDP_CLIENT_LOG_TARGET, "binding to socket: {addr:?} ...");

        let socket = time::timeout(timeout, UdpSocket::bind(addr))
            .await
            .map_err(|_| Error::TimeoutWhileBindingToSocket { addr })?
            .map_err(|e| Error::UnableToBindToSocket { err: e.into(), addr })?;

        let addr = socket.local_addr().expect("it should get the local address");

        tracing::debug!(target: UDP_CLIENT_LOG_TARGET, "bound to socket: {addr:?}.");

        let udp_client = Self {
            socket: Arc::new(socket),
            timeout,
        };

        Ok(udp_client)
    }

    /// # Errors
    ///
    /// Will return error if can't connect to the socket.
    pub async fn connect(&self, remote_addr: SocketAddr) -> Result<(), Error> {
        tracing::trace!(target: UDP_CLIENT_LOG_TARGET, "connecting to remote: {remote_addr:?} ...");

        let () = time::timeout(self.timeout, self.socket.connect(remote_addr))
            .await
            .map_err(|_| Error::TimeoutWhileConnectingToRemote { remote_addr })?
            .map_err(|e| Error::UnableToConnectToRemote {
                err: e.into(),
                remote_addr,
            })?;

        tracing::debug!(target: UDP_CLIENT_LOG_TARGET, "connected to remote: {remote_addr:?}.");

        Ok(())
    }

    /// # Errors
    ///
    /// Will return error if:
    ///
    /// - Can't write to the socket.
    /// - Can't send data.
    pub async fn send(&self, bytes: &[u8]) -> Result<usize, Error> {
        tracing::trace!(target: UDP_CLIENT_LOG_TARGET, "sending {bytes:?} ...");

        let () = time::timeout(self.timeout, self.socket.writable())
            .await
            .map_err(|_| Error::TimeoutWaitForWriteableSocket)?
            .map_err(|e| Error::UnableToGetWritableSocket { err: e.into() })?;

        let sent_bytes = time::timeout(self.timeout, self.socket.send(bytes))
            .await
            .map_err(|_| Error::TimeoutWhileSendingData { data: bytes.to_vec() })?
            .map_err(|e| Error::UnableToSendData {
                err: e.into(),
                data: bytes.to_vec(),
            })?;

        tracing::debug!(target: UDP_CLIENT_LOG_TARGET, "sent {sent_bytes} bytes to remote.");

        Ok(sent_bytes)
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
    pub async fn receive(&self) -> Result<Vec<u8>, Error> {
        tracing::trace!(target: UDP_CLIENT_LOG_TARGET, "receiving ...");

        let mut buffer = vec![0u8; RECEIVE_BUFFER_SIZE];

        let () = time::timeout(self.timeout, self.socket.readable())
            .await
            .map_err(|_| Error::TimeoutWaitForReadableSocket)?
            .map_err(|e| Error::UnableToGetReadableSocket { err: e.into() })?;

        let received_bytes = time::timeout(self.timeout, self.socket.recv(&mut buffer))
            .await
            .map_err(|_| Error::TimeoutWhileReceivingData)?
            .map_err(|e| Error::UnableToReceivingData { err: e.into() })?;

        buffer.truncate(received_bytes);
        let received = buffer;

        tracing::debug!(target: UDP_CLIENT_LOG_TARGET, "received {received_bytes} bytes: {received:?}");

        Ok(received)
    }
}

#[allow(
    clippy::module_name_repetitions,
    reason = "public type distinguishes the tracker-facing UDP client wrapper"
)]
#[derive(Debug)]
pub struct UdpTrackerClient {
    pub client: UdpClient,
}

impl UdpTrackerClient {
    /// Creates a new `UdpTrackerClient` connected to a Udp Tracker server
    ///
    /// # Errors
    ///
    /// If unable to connect to the remote address.
    ///
    pub async fn new(remote_addr: SocketAddr, timeout: Duration) -> Result<Self, Error> {
        let client = UdpClient::connected(remote_addr, timeout).await?;
        Ok(Self { client })
    }

    /// # Errors
    ///
    /// Will return error if can't write request to bytes.
    pub async fn send(&self, request: Request) -> Result<usize, Error> {
        tracing::trace!(target: UDP_CLIENT_LOG_TARGET, "sending request {request:?} ...");

        // Write request into a buffer
        // todo: optimize the pre-allocated amount based upon request type.
        let mut writer = Cursor::new(Vec::with_capacity(200));
        let () = request
            .write_bytes(&mut writer)
            .map_err(|e| Error::UnableToWriteDataFromRequest { err: e.into(), request })?;

        self.client.send(writer.get_ref()).await
    }

    /// # Errors
    ///
    /// Will return error if can't create response from the received payload (bytes buffer).
    pub async fn receive(&self) -> Result<Response, Error> {
        let response = self.client.receive().await?;

        tracing::debug!(target: UDP_CLIENT_LOG_TARGET, "received {} bytes: {response:?}", response.len());

        Response::parse_bytes(&response, true).map_err(|e| Error::UnableToParseResponse { err: e.into(), response })
    }
}

/// Helper Function to Check if a UDP Service is Connectable
///
/// It sends a connect request and reports `Ok("Connected")` when the service answers with a
/// connect response.
///
/// # Errors
///
/// It will return an error if it cannot bind or connect the client socket, if no reply arrives
/// within 2 seconds (`Timed Out`), if the reply is not a connect response (`Did not Connect`), or
/// if receiving the reply fails, for example because it is not a valid tracker response; that
/// error names the cause.
pub async fn check(service_binding: &ServiceBinding) -> Result<String, String> {
    let remote_addr = service_binding.bind_address();

    tracing::debug!("Checking Service (detail): {remote_addr:?}.");

    match UdpTrackerClient::new(remote_addr, DEFAULT_UDP_TIMEOUT).await {
        Ok(client) => {
            let connect_request = ConnectRequest {
                transaction_id: TransactionId(I32::new(123)),
            };

            // client.send() return usize, but doesn't use here
            match client.send(connect_request.into()).await {
                Ok(_) => (),
                Err(e) => tracing::debug!("Error: {e:?}."),
            }

            let process = move |response| {
                if matches!(response, Response::Connect(_connect_response)) {
                    Ok("Connected".to_string())
                } else {
                    Err("Did not Connect".to_string())
                }
            };

            let sleep = time::sleep(Duration::from_secs(2));
            tokio::pin!(sleep);

            tokio::select! {
                () = &mut sleep => {
                      Err("Timed Out".to_string())
                }
                response = client.receive() => {
                      // issue: #2867
                      match response {
                          Ok(response) => process(response),
                          Err(e) => Err(e.to_string()),
                      }
                }
            }
        }
        Err(e) => Err(format!("{e:?}")),
    }
}

#[cfg(test)]
mod tests {
    use tokio::net::UdpSocket;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};

    use super::check;

    /// A UDP peer that answers every datagram with bytes that are not a tracker response.
    async fn bind_a_junk_answering_udp_peer() -> ServiceBinding {
        let socket = UdpSocket::bind("127.0.0.1:0").await.expect("the junk peer should bind");
        let binding = ServiceBinding::new(
            Protocol::UDP,
            socket.local_addr().expect("the junk peer should have an address"),
        )
        .expect("the junk peer address should be a valid UDP binding");

        tokio::spawn(async move {
            let mut buffer = [0u8; 2048];
            while let Ok((_, from)) = socket.recv_from(&mut buffer).await {
                drop(socket.send_to(b"junk", from).await);
            }
        });

        binding
    }

    // issue: #2867
    #[tokio::test]
    async fn check_should_return_an_error_naming_the_cause_when_the_peer_answers_with_an_invalid_response() {
        // Arrange
        let junk_answering_peer = bind_a_junk_answering_udp_peer().await;

        // Act
        let result = check(&junk_answering_peer).await;

        // Assert
        let error = result.expect_err("an unparsable reply should be a failed check");
        assert!(
            error.contains("Unrecognized UDP tracker response"),
            "the error should name the unparsable response, got: {error}"
        );
    }
}
