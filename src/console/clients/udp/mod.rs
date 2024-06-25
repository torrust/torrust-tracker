use std::net::SocketAddr;

use thiserror::Error;

use crate::shared::bit_torrent::tracker::udp;

pub mod app;
pub mod checker;
pub mod responses;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Failed to Connect to: {addr}, with error: {err}")]
    UnableToBindAndConnect { addr: SocketAddr, err: udp::Error },

    #[error("Failed to receive a response, with error: {err}")]
    UnableToReceiveResponse { err: udp::Error },

    #[error("Failed to send a request, with error: {err}")]
    UnableToSendRequest { err: udp::Error },

    #[error("Failed to get local address for connection: {err}")]
    UnableToGetLocalAddr { err: udp::Error },

    #[error("Failed to get a successful connection response: {err}")]
    UnexpectedConnectionResponse { err: udp::Error },
}
