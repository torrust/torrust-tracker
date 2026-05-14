use std::net::SocketAddr;

use bittorrent_tracker_client::udp;
use bittorrent_udp_tracker_protocol::Response;
use serde::Serialize;
use thiserror::Error;

pub mod app;
pub mod checker;
pub mod responses;

#[derive(Error, Debug, Clone, Serialize)]
#[serde(into = "String")]
pub enum Error {
    #[error("Failed to Connect to: {remote_addr}, with error: {err}")]
    UnableToBindAndConnect { remote_addr: SocketAddr, err: udp::Error },

    #[error("Failed to send a connection request, with error: {err}")]
    UnableToSendConnectionRequest { err: udp::Error },

    #[error("{err}")]
    UnableToReceiveConnectResponse {
        #[source]
        err: udp::Error,
    },

    #[error("Failed to send a announce request, with error: {err}")]
    UnableToSendAnnounceRequest { err: udp::Error },

    #[error("{err}")]
    UnableToReceiveAnnounceResponse {
        #[source]
        err: udp::Error,
    },

    #[error("Failed to send a scrape request, with error: {err}")]
    UnableToSendScrapeRequest { err: udp::Error },

    #[error("{err}")]
    UnableToReceiveScrapeResponse {
        #[source]
        err: udp::Error,
    },

    #[error("{err}")]
    UnableToReceiveResponse {
        #[source]
        err: udp::Error,
    },

    #[error("Failed to get local address for connection: {err}")]
    UnableToGetLocalAddr { err: udp::Error },

    #[error("Failed to get a connection response: {response:?}")]
    UnexpectedConnectionResponse { response: Response },
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::Arc;

    use bittorrent_tracker_client::udp;

    use super::Error;

    #[test]
    fn it_should_display_the_inner_udp_parse_error_for_announce_responses() {
        // Arrange
        let inner_error = udp::Error::UnableToParseResponse {
            err: Arc::new(io::Error::other("failed to fill whole buffer")),
            response: vec![0, 0, 0, 1],
        };

        let error = Error::UnableToReceiveAnnounceResponse { err: inner_error };

        // Act
        let message = error.to_string();

        // Assert
        assert_eq!(
            message,
            "Unrecognized UDP tracker response. Expected a valid UDP response, got: [0, 0, 0, 1]"
        );
    }
}
