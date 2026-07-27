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
//! See [ADR-20260727000000](../../../../../docs/adrs/20260727000000_events_are_objective_facts.md)
//! for the full rationale, the concrete counter-example, and naming heuristics.
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
#[derive(Debug, Clone, PartialEq)]
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
            UdpRequestKind::Connect => LabelValue::new("connect"),
            UdpRequestKind::Announce { .. } => LabelValue::new("announce"),
            UdpRequestKind::Scrape => LabelValue::new("scrape"),
        }
    }
}

impl fmt::Display for UdpRequestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let proto_str = match self {
            UdpRequestKind::Connect => "connect",
            UdpRequestKind::Announce { .. } => "announce",
            UdpRequestKind::Scrape => "scrape",
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

#[derive(Debug, Clone, PartialEq)]
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
            Error::Internal { location: _, message } => Self::InternalServer(message.clone()),
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
