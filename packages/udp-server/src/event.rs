//! UDP tracker server events.
//!
//! # Design contract: events are objective facts
//!
//! Every variant in [`Event`] describes *what happened* — a neutral, observable
//! fact about a request or connection. Events must **not** be designed around
//! what a particular consumer should or should not do in response.
//!
//! **Wrong pattern**: creating a new event variant (e.g. `CookieErrorInLenientMode`)
//! so that a specific listener (e.g. the ban handler) silently ignores it.
//! That couples the event schema to one consumer's behaviour and hides policy
//! decisions inside the event layer.
//!
//! **Right pattern**: emit the same objective event (`UdpError { ConnectionCookie }`)
//! regardless of the active policy. Let the enforcement point (e.g. the `is_banned`
//! check in the main loop) gate on the policy and decide whether to act.
//!
//! Rule of thumb: if you are adding a new variant that is structurally identical
//! to an existing one but named differently so a listener ignores it — stop and
//! change the listener or the enforcement point instead.
//!
//! See [ADR-20260727000000](../../../docs/adrs/20260727000000_events_are_objective_facts.md)
//! for the full rationale, the concrete counter-example, and naming heuristics.
//!
//! The existing [`Event::UdpError`] and [`ErrorKind`] predate a general
//! rejected-request event contract. Do not add ad hoc error variants or reuse
//! internal error types as new payloads; see the [general error-events
//! EPIC](../../../docs/issues/drafts/generalize-error-events.md).
use std::fmt;
use std::time::Duration;

use torrust_metrics::label::LabelValue;
use torrust_tracker_core::error::{AnnounceError, ScrapeError};
use torrust_tracker_udp_core::event::ConnectionContext;
use torrust_tracker_udp_core::services::announce::UdpAnnounceError;
use torrust_tracker_udp_core::services::scrape::UdpScrapeError;
use torrust_tracker_udp_protocol::AnnounceRequest;

use crate::error::Error;

/// A UDP server event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    UdpRequestReceived {
        context: ConnectionContext,
    },
    UdpRequestDiscarded {
        context: ConnectionContext,
    },
    UdpRequestAborted {
        context: ConnectionContext,
    },
    UdpRequestBanned {
        context: ConnectionContext,
    },
    UdpRequestAccepted {
        context: ConnectionContext,
        kind: UdpRequestKind,
    },
    UdpResponseSent {
        context: ConnectionContext,
        kind: UdpResponseKind,
        req_processing_time: Duration,
    },
    UdpError {
        context: ConnectionContext,
        kind: Option<UdpRequestKind>,
        error: ErrorKind,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UdpRequestKind {
    Connect,
    Announce { announce_request: AnnounceRequest },
    Scrape,
}

impl From<UdpRequestKind> for LabelValue {
    fn from(kind: UdpRequestKind) -> Self {
        match kind {
            UdpRequestKind::Connect => Self::new("connect"),
            UdpRequestKind::Announce { .. } => Self::new("announce"),
            UdpRequestKind::Scrape => Self::new("scrape"),
        }
    }
}

impl fmt::Display for UdpRequestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let proto_str = match self {
            Self::Connect => "connect",
            Self::Announce { .. } => "announce",
            Self::Scrape => "scrape",
        };
        write!(f, "{proto_str}")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UdpResponseKind {
    Ok {
        req_kind: UdpRequestKind,
    },

    /// There was an error handling the request. The error contains the request
    /// kind if the request was parsed successfully.
    Error {
        opt_req_kind: Option<UdpRequestKind>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    RequestParse(String),
    ConnectionCookie(String),
    Whitelist(String),
    Database(String),
    InternalServer(String),
    BadRequest(String),
    TrackerAuthentication(String),
}

impl From<Error> for ErrorKind {
    fn from(error: Error) -> Self {
        match error {
            Error::InvalidRequest { request_parse_error } => Self::RequestParse(request_parse_error.to_string()),
            Error::AnnounceFailed { source } => match source {
                UdpAnnounceError::ConnectionCookieError { source } => Self::ConnectionCookie(source.to_string()),
                UdpAnnounceError::TrackerCoreAnnounceError { source } => match source {
                    AnnounceError::Whitelist(whitelist_error) => Self::Whitelist(whitelist_error.to_string()),
                    AnnounceError::Database(error) => Self::Database(error.to_string()),
                },
                UdpAnnounceError::TrackerCoreWhitelistError { source } => Self::Whitelist(source.to_string()),
            },
            Error::ScrapeFailed { source } => match source {
                UdpScrapeError::ConnectionCookieError { source } => Self::ConnectionCookie(source.to_string()),
                UdpScrapeError::TrackerCoreScrapeError { source } => match source {
                    ScrapeError::Whitelist(whitelist_error) => Self::Whitelist(whitelist_error.to_string()),
                },
                UdpScrapeError::TrackerCoreWhitelistError { source } => Self::Whitelist(source.to_string()),
            },
            Error::Internal { location: _, message } => Self::InternalServer(message),
            Error::AuthRequired { location } => Self::TrackerAuthentication(location.to_string()),
        }
    }
}

pub mod sender {
    use std::sync::Arc;

    use super::Event;

    pub type Sender = Option<Arc<dyn torrust_tracker_events::sender::Sender<Event = Event>>>;
    pub type Broadcaster = torrust_tracker_events::broadcaster::Broadcaster<Event>;
}

pub mod receiver {
    use super::Event;

    pub type Receiver = Box<dyn torrust_tracker_events::receiver::Receiver<Event = Event>>;
}

pub mod bus {
    use crate::event::Event;

    pub type EventBus = torrust_tracker_events::bus::EventBus<Event>;
}

#[cfg(test)]
mod tests {
    use std::panic::Location;
    use std::str::FromStr;

    use torrust_info_hash::InfoHash;
    use torrust_tracker_core::databases::error::Error as DatabaseError;
    use torrust_tracker_core::error::{AnnounceError, WhitelistError};
    use torrust_tracker_primitives::Driver;
    use torrust_tracker_udp_core::connection_cookie::ConnectionCookieError;
    use torrust_tracker_udp_core::services::announce::UdpAnnounceError;

    use super::ErrorKind;
    use crate::error::{Error, SendableRequestParseError};

    #[test]
    fn it_should_classify_an_invalid_request_as_a_request_parse_error() {
        // Arrange
        let error = Error::InvalidRequest {
            request_parse_error: SendableRequestParseError {
                message: "invalid request".to_string(),
                opt_connection_id: None,
                opt_transaction_id: None,
            },
        };

        // Act
        let actual = ErrorKind::from(error);

        // Assert
        assert_eq!(
            actual,
            ErrorKind::RequestParse(
                "SendableRequestParseError: message: invalid request, connection_id: None, transaction_id: None".to_string(),
            )
        );
    }

    #[test]
    fn it_should_classify_a_connection_cookie_error() {
        // Arrange
        let error = Error::AnnounceFailed {
            source: UdpAnnounceError::ConnectionCookieError {
                source: ConnectionCookieError::ValueExpired {
                    expired_value: 1.0,
                    min_value: 2.0,
                },
            },
        };

        // Act
        let actual = ErrorKind::from(error);

        // Assert
        assert_eq!(
            actual,
            ErrorKind::ConnectionCookie("cookie value is expired: 1, expected > 2".to_string())
        );
    }

    #[test]
    fn it_should_classify_a_whitelist_error() {
        // Arrange
        let info_hash = InfoHash::from_str("3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0") // DevSkim: ignore DS173237
            .expect("test info hash should be valid");
        let error = Error::AnnounceFailed {
            source: UdpAnnounceError::TrackerCoreWhitelistError {
                source: WhitelistError::TorrentNotWhitelisted {
                    info_hash,
                    location: Location::caller(),
                },
            },
        };

        // Act
        let actual = ErrorKind::from(error);

        // Assert
        assert!(
            matches!(actual, ErrorKind::Whitelist(message) if message.contains("The torrent: 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0, is not whitelisted"))
        );
    }

    #[test]
    fn it_should_classify_a_database_error() {
        // Arrange
        let error = Error::AnnounceFailed {
            source: UdpAnnounceError::TrackerCoreAnnounceError {
                source: AnnounceError::Database(DatabaseError::MalformedDatabaseRecord {
                    message: "corrupt record".to_string(),
                    driver: Driver::Sqlite3,
                }),
            },
        };

        // Act
        let actual = ErrorKind::from(error);

        // Assert
        assert_eq!(
            actual,
            ErrorKind::Database("Malformed Sqlite3 database record: corrupt record".to_string())
        );
    }

    #[test]
    fn it_should_classify_an_internal_error() {
        // Arrange
        let error = Error::Internal {
            location: Location::caller(),
            message: "internal failure".to_string(),
        };

        // Act
        let actual = ErrorKind::from(error);

        // Assert
        assert_eq!(actual, ErrorKind::InternalServer("internal failure".to_string()));
    }

    #[test]
    fn it_should_classify_an_authentication_error() {
        // Arrange
        let location = Location::caller();
        let error = Error::AuthRequired { location };

        // Act
        let actual = ErrorKind::from(error);

        // Assert
        assert_eq!(actual, ErrorKind::TrackerAuthentication(location.to_string()));
    }
}
