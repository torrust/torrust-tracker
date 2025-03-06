//! Error types for the UDP server.
use std::panic::Location;

use aquatic_udp_protocol::{ConnectionId, RequestParseError};
use bittorrent_udp_tracker_core::services::announce::UdpAnnounceError;
use bittorrent_udp_tracker_core::services::scrape::UdpScrapeError;
use derive_more::derive::Display;
use thiserror::Error;
use torrust_tracker_located_error::LocatedError;

#[derive(Display, Debug)]
#[display(":?")]
pub struct ConnectionCookie(pub ConnectionId);

/// Error returned by the UDP server.
#[derive(Error, Debug)]
pub enum Error {
    /// Error returned when the request is invalid.
    #[error("error when phrasing request: {request_parse_error:?}")]
    RequestParseError { request_parse_error: RequestParseError },

    /// Error returned when the domain tracker returns an announce error.
    #[error("tracker announce error: {source}")]
    UdpAnnounceError { source: UdpAnnounceError },

    /// Error returned when the domain tracker returns an scrape error.
    #[error("tracker scrape error: {source}")]
    UdpScrapeError { source: UdpScrapeError },

    /// Error returned from a third-party library (`aquatic_udp_protocol`).
    #[error("internal server error: {message}, {location}")]
    InternalServer {
        location: &'static Location<'static>,
        message: String,
    },

    /// Error returned when the request is invalid.
    #[error("bad request: {source}")]
    BadRequest {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    /// Error returned when tracker requires authentication.
    #[error("domain tracker requires authentication but is not supported in current UDP implementation. Location: {location}")]
    TrackerAuthenticationRequired { location: &'static Location<'static> },
}

impl From<RequestParseError> for Error {
    fn from(request_parse_error: RequestParseError) -> Self {
        Self::RequestParseError { request_parse_error }
    }
}

impl From<UdpAnnounceError> for Error {
    fn from(udp_announce_error: UdpAnnounceError) -> Self {
        Self::UdpAnnounceError {
            source: udp_announce_error,
        }
    }
}

impl From<UdpScrapeError> for Error {
    fn from(udp_scrape_error: UdpScrapeError) -> Self {
        Self::UdpScrapeError {
            source: udp_scrape_error,
        }
    }
}
