//! Error types for the UDP server.
use std::fmt::Display;
use std::panic::Location;

use derive_more::derive::Display;
use thiserror::Error;
use torrust_tracker_udp_core::services::announce::UdpAnnounceError;
use torrust_tracker_udp_core::services::scrape::UdpScrapeError;
use torrust_tracker_udp_protocol::{ConnectionId, RequestParseError, TransactionId};

#[derive(Display, Debug)]
#[display(":?")]
pub struct ConnectionCookie(pub ConnectionId);

/// Error returned by the UDP server.
///
/// This internal type carries implementation details and must not be used as a
/// new event payload without the stable reason classification required by the
/// [general error-events EPIC](../../../docs/issues/drafts/generalize-error-events/EPIC.md).
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

    /// Error returned from the wire-protocol crate (`torrust_tracker_udp_protocol`).
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

#[cfg(test)]
mod tests {
    use torrust_tracker_udp_core::connection_cookie::ConnectionCookieError;
    use torrust_tracker_udp_core::services::announce::UdpAnnounceError;
    use torrust_tracker_udp_core::services::scrape::UdpScrapeError;
    use torrust_tracker_udp_protocol::{ConnectionId, RequestParseError, TransactionId};
    use zerocopy::byteorder::network_endian::{I32, I64};

    use super::{Error, SendableRequestParseError};

    #[test]
    fn it_should_preserve_response_routing_identifiers_for_a_sendable_parse_error() {
        // Arrange
        let connection_id = ConnectionId(I64::new(12));
        let transaction_id = TransactionId(I32::new(34));
        let parse_error = RequestParseError::sendable_text("invalid announce request", connection_id, transaction_id);
        let expected = SendableRequestParseError {
            message: "invalid announce request".to_string(),
            opt_connection_id: Some(connection_id),
            opt_transaction_id: Some(transaction_id),
        };

        // Act
        let actual = SendableRequestParseError::from(parse_error);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_should_clear_response_routing_identifiers_for_an_unsendable_parse_error() {
        // Arrange
        let parse_error = RequestParseError::unsendable_text("invalid request action");
        let expected = SendableRequestParseError {
            message: "invalid request action".to_string(),
            opt_connection_id: None,
            opt_transaction_id: None,
        };

        // Act
        let actual = SendableRequestParseError::from(parse_error);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_should_wrap_a_sendable_parse_error_as_an_invalid_request() {
        // Arrange
        let connection_id = ConnectionId(I64::new(12));
        let transaction_id = TransactionId(I32::new(34));
        let parse_error = RequestParseError::sendable_text("invalid scrape request", connection_id, transaction_id);

        // Act
        let actual = Error::from(parse_error);

        // Assert
        assert!(matches!(
            actual,
            Error::InvalidRequest {
                request_parse_error: SendableRequestParseError {
                    message,
                    opt_connection_id: Some(actual_connection_id),
                    opt_transaction_id: Some(actual_transaction_id),
                },
            } if message == "invalid scrape request"
                && actual_connection_id == connection_id
                && actual_transaction_id == transaction_id
        ));
    }

    #[test]
    fn it_should_wrap_an_announce_service_error_as_an_announce_failure() {
        // Arrange
        let source = ConnectionCookieError::ValueExpired {
            expired_value: 12.0,
            min_value: 13.0,
        };
        let announce_error = UdpAnnounceError::ConnectionCookieError { source: source.clone() };

        // Act
        let actual = Error::from(announce_error);

        // Assert
        assert!(matches!(
            actual,
            Error::AnnounceFailed {
                source: UdpAnnounceError::ConnectionCookieError { source: actual_source },
            } if actual_source == source
        ));
    }

    #[test]
    fn it_should_wrap_a_scrape_service_error_as_a_scrape_failure() {
        // Arrange
        let source = ConnectionCookieError::ValueFromFuture {
            future_value: 14.0,
            max_value: 13.0,
        };
        let scrape_error = UdpScrapeError::ConnectionCookieError { source: source.clone() };

        // Act
        let actual = Error::from(scrape_error);

        // Assert
        assert!(matches!(
            actual,
            Error::ScrapeFailed {
                source: UdpScrapeError::ConnectionCookieError { source: actual_source },
            } if actual_source == source
        ));
    }
}
