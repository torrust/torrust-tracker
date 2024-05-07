use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use aquatic_udp_protocol::{ConnectRequest, ConnectResponse, Request, Response, TransactionId};
use derive_more::{AsRef, Constructor, From, Into};
use tokio::net::UdpSocket;
use tokio::time;
use torrust_tracker_configuration::{CLIENT_TIMEOUT_DEFAULT, MAX_PACKET_SIZE, PORT_ASSIGNED_BY_OS};
use tracing::debug;

use super::{source_address, Error};

#[derive(From, Into, AsRef, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct LocalSocketAddr(SocketAddr);
impl Default for LocalSocketAddr {
    fn default() -> Self {
        Self(source_address(PORT_ASSIGNED_BY_OS))
    }
}

#[derive(From, Into, AsRef, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Timeout(Duration);

impl Default for Timeout {
    fn default() -> Self {
        Self(CLIENT_TIMEOUT_DEFAULT)
    }
}

#[derive(Constructor, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Launcher {
    bind_to: LocalSocketAddr,
    timeout: Duration,
}

#[derive(Debug)]
pub struct Bound {
    launcher: Launcher,
    sock: UdpSocket,
}

#[derive(Debug, Clone)]
pub struct Client {
    launcher: Launcher,
    sock: Arc<UdpSocket>,
}

impl Launcher {
    /// # Errors
    ///
    /// Will error if the local address can't be bound.
    pub async fn bind(&self) -> Result<Bound, Error> {
        let sock = UdpSocket::bind(self.bind_to.as_ref())
            .await
            .map_err(|e| Error::ClientBuildingError { err: e.into() })?;

        Ok(Bound::new(*self, sock))
    }
}

impl Bound {
    fn new(launcher: Launcher, sock: UdpSocket) -> Self {
        Self { launcher, sock }
    }

    /// Returns the local address of this [`Bound`].
    ///
    /// # Errors
    ///
    /// This function errors if underlying function fails.
    #[allow(dead_code)]
    pub fn local_addr(&self) -> Result<SocketAddr, Error> {
        self.sock
            .local_addr()
            .map_err(|e| Error::UnableToGetLocalAddress { err: e.into() })
    }

    /// # Errors
    ///
    /// Will error if can't connect to the socket.
    pub async fn connect(self, addr: SocketAddr) -> Result<Client, Error> {
        let () = self
            .sock
            .connect(addr)
            .await
            .map_err(|e| Error::UnableToConnectToRemote { err: e.into() })?;

        Ok(Client::new(self))
    }
}

impl Client {
    /// Creates a new `UdpTrackerClient` connected a remote `addr`.
    ///
    /// # Errors
    ///
    /// This function returns and error if the the binding fails.
    pub async fn connect(addr: SocketAddr, timeout: Duration) -> Result<Self, Error> {
        let launcher = Launcher {
            timeout,
            ..Default::default()
        };
        let bound = launcher.bind().await?;

        bound.connect(addr).await
    }

    fn new(bound: Bound) -> Self {
        Self {
            sock: bound.sock.into(),
            launcher: bound.launcher,
        }
    }

    #[must_use]
    pub fn timeout(&self) -> Duration {
        self.launcher.timeout
    }

    /// Returns the local address of this [`Client`].
    ///
    /// # Errors
    ///
    /// This function errors if underlying function fails.
    pub fn local_addr(&self) -> Result<SocketAddr, Error> {
        self.sock
            .local_addr()
            .map_err(|e| Error::UnableToGetLocalAddress { err: e.into() })
    }

    /// Returns the peer address of this [`Client`].
    ///
    /// # Errors
    ///
    /// This function errors if underlying function fails.
    pub fn peer_addr(&self) -> Result<SocketAddr, Error> {
        self.sock
            .peer_addr()
            .map_err(|e| Error::UnableToGetRemoteAddress { err: e.into() })
    }

    /// # Errors
    ///
    /// Will error if:
    ///
    /// - Can't write to the socket.
    /// - Can't send data.
    pub async fn send(&self, bytes: &[u8]) -> Result<usize, Error> {
        debug!(target: "UDP client", "sending {bytes:?} ...");

        time::timeout(self.timeout(), self.sock.writable())
            .await
            .map_err(|_| Error::TimedOut {
                context: "Get Writable Socket".into(),
            })?
            .map_err(|e| Error::UnableToGetWriteable { err: e.into() })?;

        time::timeout(self.timeout(), self.sock.send(bytes))
            .await
            .map_err(|_| Error::TimedOut {
                context: "Send To Socket".into(),
            })?
            .map_err(|e| Error::UnableToSendToSocket { err: e.into() })
    }

    /// # Errors
    ///
    /// Will error if:
    ///
    /// - Can't read from the socket.
    /// - Can't receive data.
    pub async fn receive(&self, bytes: &mut [u8]) -> Result<usize, Error> {
        debug!(target: "UDP client", "receiving ...");

        let () = time::timeout(self.timeout(), self.sock.readable())
            .await
            .map_err(|_| Error::TimedOut {
                context: "Get Readable Socket".into(),
            })?
            .map_err(|e| Error::UnableToGetReadable { err: e.into() })?;

        let size = time::timeout(self.timeout(), self.sock.recv(bytes))
            .await
            .map_err(|_| Error::TimedOut {
                context: "Read From Socket".into(),
            })?
            .map_err(|e| Error::UnableToReadFromSocket { err: e.into() })?;

        debug!(target: "UDP client", "{size} bytes received {bytes:?}");

        Ok(size)
    }

    /// # Errors
    ///
    /// Will errors if can't write request to bytes.
    pub async fn send_request(&self, request: Request) -> Result<usize, Error> {
        debug!(target: "UDP tracker client", "send request {request:?}");

        // Write request into a buffer
        let request_buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(request_buffer);

        request
            .write_bytes(&mut cursor)
            .map_err(|e| Error::UnableToWriteToRequestBuffer { err: e.into() })?;

        let request_data = {
            #[allow(clippy::cast_possible_truncation)]
            let position = cursor.position() as usize;
            let inner_request_buffer = cursor.get_ref();
            // Return slice which contains written request data
            &inner_request_buffer[..position]
        };

        self.send(request_data).await
    }

    /// # Errors
    ///
    /// Will error if can't create response from the received payload (bytes buffer).
    pub async fn receive_response(&self) -> Result<Response, Error> {
        let mut response_buffer = [0u8; MAX_PACKET_SIZE];

        let payload_size = self.receive(&mut response_buffer).await?;

        debug!(target: "UDP tracker client", "received {payload_size} bytes. Response {response_buffer:?}");

        Response::parse_bytes(&response_buffer[..payload_size], true)
            .map_err(|e| Error::UnableToGetResponseFromBuffer { err: e.into() })
    }

    /// Completes a connection request to the UDP Tracker server.
    ///
    /// # Errors
    ///
    /// Will return and error if
    ///
    /// - It can't connect to the remote UDP socket.
    /// - It can't make a connection request successfully to the remote UDP
    /// server (after successfully connecting to the remote UDP socket).
    ///
    pub async fn do_connection_request(&self, transaction_id: TransactionId) -> Result<ConnectResponse, Error> {
        debug!("Sending connection request with transaction id: {transaction_id:#?}");

        let connect_request = ConnectRequest { transaction_id };

        let _ = self.send_request(connect_request.into()).await?;

        let response = self.receive_response().await?;

        debug!("connection request response:\n{response:#?}");

        check_connect_response(&response, connect_request)
    }

    /// Helper Function to Check if a UDP Service is Connectable
    ///
    /// # Errors
    ///
    /// It will return an error if unable to connect to the UDP service.
    ///
    /// # Panics
    pub async fn check(self) -> Result<String, Error> {
        let connect_request = ConnectRequest {
            transaction_id: TransactionId::new(rand::Rng::gen(&mut rand::thread_rng())),
        };

        let _ = self.send_request(connect_request.into()).await?;

        let process = move |response: Result<Response, Error>, connect_request: ConnectRequest| -> Result<String, Error> {
            check_connect_response(&response?, connect_request).map(|id| {
                format!(
                    "Connected with, transaction_id: {} and connection_id: {}",
                    id.transaction_id.0, id.connection_id.0
                )
            })
        };

        let sleep = time::sleep(Duration::from_millis(2000));
        tokio::pin!(sleep);

        tokio::select! {
            () = &mut sleep => {
                  Err(Error::TimedOut { context: "Receive Connect Response".into() })
            }
            response = self.receive_response() => {
                  process(response, connect_request)
            }
        }
    }
}

/// Checks the Connect Response Against the Request
///
/// # Errors
///
/// If the [`Response`] is not a [`ConnectResponse`]
/// or if the [`TransactionId`] dose not match.
///
pub fn check_connect_response(response: &Response, connect_request: ConnectRequest) -> Result<ConnectResponse, Error> {
    match response {
        Response::Connect(connect) => {
            if connect.transaction_id == connect_request.transaction_id {
                Ok(*connect)
            } else {
                Err(Error::UnexpectedTransactionId {
                    expected: connect.transaction_id,
                    received: connect_request.transaction_id,
                })
            }
        }
        response => Err(Error::UnexpectedResponse {
            response: response.clone(),
        }),
    }
}
