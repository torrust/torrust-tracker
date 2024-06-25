//! Error types for the UDP server.
use std::panic::Location;

use thiserror::Error;
use torrust_tracker_located_error::LocatedError;

/// Error returned by the UDP server.
#[derive(Error, Debug)]
pub enum Error {
    /// Error returned when the domain tracker returns an error.
    #[error("tracker server error: {source}")]
    TrackerError {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    /// Error returned from a third-party library (`aquatic_udp_protocol`).
    #[error("internal server error: {message}, {location}")]
    InternalServer {
        location: &'static Location<'static>,
        message: String,
    },

    /// Error returned when the connection id could not be verified.
    #[error("connection id could not be verified")]
    InvalidConnectionId { location: &'static Location<'static> },

    /// Error returned when the request is invalid.
    #[error("bad request: {source}")]
    BadRequest {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    /// Error returned when tracker requires authentication.
    #[error("domain tracker requires authentication but is not supported in current UDP implementation. Location: {location}")]
    TrackerAuthenticationRequired { location: &'static Location<'static> },
}
