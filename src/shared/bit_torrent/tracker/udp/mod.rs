use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use thiserror::Error;

mod client;

pub use client::Client;
use torrust_tracker_located_error::DynError;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Timed Out: \"{context}\".")]
    TimedOut { context: String },

    #[error("Failed to Get Writeable Socket: {err:?}")]
    UnableToGetWriteable { err: Arc<std::io::Error> },

    #[error("Error when sending to socket: {err:?}")]
    UnableToSendToSocket { err: Arc<std::io::Error> },

    #[error("Failed to Get Readable Socket: {err:?}")]
    UnableToGetReadable { err: Arc<std::io::Error> },

    #[error("Error when reading from socket: {err:?}")]
    UnableToReadFromSocket { err: Arc<std::io::Error> },

    #[error("Error when writing to buffer: {err:?}")]
    UnableToWriteToRequestBuffer { err: Arc<std::io::Error> },

    #[error("Error when writing to buffer: {err:?}")]
    UnableToGetResponseFromBuffer { err: Arc<std::io::Error> },

    #[error("Received an unexpected response: {response:?}")]
    UnexpectedResponse { response: aquatic_udp_protocol::Response },

    #[error("Received an unexpected TransactionId: Expected: {expected:?}, Received: {received:?}")]
    UnexpectedTransactionId {
        expected: aquatic_udp_protocol::TransactionId,
        received: aquatic_udp_protocol::TransactionId,
    },

    #[error("Failed to bind the Client: {err:?}")]
    ClientBuildingError { err: Arc<std::io::Error> },
    #[error("Failed to get the bound local socket: {err:?}")]
    UnableToGetLocalAddress { err: Arc<std::io::Error> },
    #[error("Failed to connect to remote: {err:?}")]
    UnableToConnectToRemote { err: Arc<std::io::Error> },
    #[error("Failed to get the connected socket: {err:?}")]
    UnableToGetRemoteAddress { err: Arc<std::io::Error> },
}

impl From<Error> for DynError {
    fn from(e: Error) -> Self {
        Arc::new(Box::new(e))
    }
}

/// Generates the source address for the UDP client
fn source_address(port: u16) -> SocketAddr {
    SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port)
}
