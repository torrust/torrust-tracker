//! Error types for the UDP server.
use std::fmt::Display;
use std::panic::Location;

use aquatic_udp_protocol::{ConnectionId, RequestParseError, TransactionId};
use bittorrent_udp_tracker_core::services::announce::UdpAnnounceError;
use bittorrent_udp_tracker_core::services::scrape::UdpScrapeError;
use derive_more::derive::Display;
use thiserror::Error;

#[derive(Display, Debug)]
#[display(":?")]
pub struct ConnectionCookie(pub ConnectionId);

/// Error returned by the UDP server.
#[derive(Error, Debug, Clone)]
pub enum Error {
    /// Error returned when the request is invalid.
    #[error("error parsing request: {request_parse_error:?}")]
    InvalidRequest { request_parse_error: SendableRequestParseError },

    /// Error returned when the domain tracker returns an announce error.
    #[error("tracker announce error: {source}")]
    AnnounceFailed { source: UdpAnnounceError },

    /// Error returned when the domain tracker returns an scrape error.
    #[error("tracker scrape error: {source}")]
    ScrapeFailed { source: UdpScrapeError },

    /// Error returned from a third-party library (`aquatic_udp_protocol`).
    #[error("internal server error: {message}, {location}")]
    Internal {
        location: &'static Location<'static>,
        message: String,
    },

    /// Error returned when tracker requires authentication.
    #[error("domain tracker requires authentication but is not supported in current UDP implementation. Location: {location}")]
    AuthRequired { location: &'static Location<'static> },
}

impl From<RequestParseError> for Error {
    fn from(request_parse_error: RequestParseError) -> Self {
        Self::InvalidRequest {
            request_parse_error: request_parse_error.into(),
        }
    }
}

impl From<UdpAnnounceError> for Error {
    fn from(udp_announce_error: UdpAnnounceError) -> Self {
        Self::AnnounceFailed {
            source: udp_announce_error,
        }
    }
}

impl From<UdpScrapeError> for Error {
    fn from(udp_scrape_error: UdpScrapeError) -> Self {
        Self::ScrapeFailed {
            source: udp_scrape_error,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SendableRequestParseError {
    pub message: String,
    pub opt_connection_id: Option<ConnectionId>,
    pub opt_transaction_id: Option<TransactionId>,
}

impl Display for SendableRequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SendableRequestParseError: message: {}, connection_id: {:?}, transaction_id: {:?}",
            self.message, self.opt_connection_id, self.opt_transaction_id
        )
    }
}

impl From<RequestParseError> for SendableRequestParseError {
    fn from(request_parse_error: RequestParseError) -> Self {
        let (message, opt_connection_id, opt_transaction_id) = match request_parse_error {
            RequestParseError::Sendable {
                connection_id,
                transaction_id,
                err,
            } => ((*err).to_string(), Some(connection_id), Some(transaction_id)),
            RequestParseError::Unsendable { err } => (err.to_string(), None, None),
        };

        Self {
            message,
            opt_connection_id,
            opt_transaction_id,
        }
    }
}
