use std::net::SocketAddr;
use std::sync::Arc;

use thiserror::Error;
use torrust_located_error::DynError;
use torrust_tracker_udp_tracker_protocol::Request;

pub mod client;

/// The maximum number of bytes in a UDP packet.
pub const MAX_PACKET_SIZE: usize = 1496;
/// A magic 64-bit integer constant defined in the protocol that is used to
/// identify the protocol.
pub const PROTOCOL_ID: i64 = 0x0417_2710_1980;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Timeout while waiting for socket to bind: {addr:?}")]
    TimeoutWhileBindingToSocket { addr: SocketAddr },

    #[error("Failed to bind to socket: {addr:?}, with error: {err:?}")]
    UnableToBindToSocket { err: Arc<std::io::Error>, addr: SocketAddr },

    #[error("Timeout while waiting for connection to remote: {remote_addr:?}")]
    TimeoutWhileConnectingToRemote { remote_addr: SocketAddr },

    #[error("Failed to connect to remote: {remote_addr:?}, with error: {err:?}")]
    UnableToConnectToRemote {
        err: Arc<std::io::Error>,
        remote_addr: SocketAddr,
    },

    #[error("Timeout while waiting for the socket to become writable.")]
    TimeoutWaitForWriteableSocket,

    #[error("Failed to get writable socket: {err:?}")]
    UnableToGetWritableSocket { err: Arc<std::io::Error> },

    #[error("Timeout while trying to send data: {data:?}")]
    TimeoutWhileSendingData { data: Vec<u8> },

    #[error("Failed to send data: {data:?}, with error: {err:?}")]
    UnableToSendData { err: Arc<std::io::Error>, data: Vec<u8> },

    #[error("Timeout while waiting for the socket to become readable.")]
    TimeoutWaitForReadableSocket,

    #[error("Failed to get readable socket: {err:?}")]
    UnableToGetReadableSocket { err: Arc<std::io::Error> },

    #[error("Timeout while trying to receive data.")]
    TimeoutWhileReceivingData,

    #[error("Failed to receive data: {err:?}")]
    UnableToReceivingData { err: Arc<std::io::Error> },

    #[error("Failed to get data from request: {request:?}, with error: {err:?}")]
    UnableToWriteDataFromRequest { err: Arc<std::io::Error>, request: Request },

    #[error("Unrecognized UDP tracker response. Expected a valid UDP response, got: {response:?}")]
    UnableToParseResponse {
        #[source]
        err: Arc<std::io::Error>,
        response: Vec<u8>,
    },
}

impl From<Error> for DynError {
    fn from(e: Error) -> Self {
        Arc::new(Box::new(e))
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::Arc;

    use super::Error;

    #[test]
    fn it_should_display_unrecognized_udp_tracker_response_without_debug_noise() {
        // Arrange
        let error = Error::UnableToParseResponse {
            err: Arc::new(io::Error::other("failed to fill whole buffer")),
            response: vec![0, 0, 0, 1],
        };

        // Act
        let message = error.to_string();

        // Assert
        assert_eq!(
            message,
            "Unrecognized UDP tracker response. Expected a valid UDP response, got: [0, 0, 0, 1]"
        );
    }
}
