use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;

use aquatic_udp_protocol::AnnounceRequest;
use bittorrent_tracker_core::error::{AnnounceError, ScrapeError};
use bittorrent_udp_tracker_core::services::announce::UdpAnnounceError;
use bittorrent_udp_tracker_core::services::scrape::UdpScrapeError;
use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::label_name;
use torrust_tracker_primitives::service_binding::ServiceBinding;

use crate::error::Error;

/// A UDP server event.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    UdpRequestReceived {
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConnectionContext {
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
}

impl ConnectionContext {
    #[must_use]
    pub fn new(client_socket_addr: SocketAddr, server_service_binding: ServiceBinding) -> Self {
        Self {
            client_socket_addr,
            server_service_binding,
        }
    }

    #[must_use]
    pub fn client_socket_addr(&self) -> SocketAddr {
        self.client_socket_addr
    }

    #[must_use]
    pub fn server_socket_addr(&self) -> SocketAddr {
        self.server_service_binding.bind_address()
    }
}

impl From<ConnectionContext> for LabelSet {
    fn from(connection_context: ConnectionContext) -> Self {
        LabelSet::from([
            (
                label_name!("server_binding_protocol"),
                LabelValue::new(&connection_context.server_service_binding.protocol().to_string()),
            ),
            (
                label_name!("server_binding_ip"),
                LabelValue::new(&connection_context.server_service_binding.bind_address().ip().to_string()),
            ),
            (
                label_name!("server_binding_address_ip_type"),
                LabelValue::new(&connection_context.server_service_binding.bind_address_ip_type().to_string()),
            ),
            (
                label_name!("server_binding_address_ip_family"),
                LabelValue::new(&connection_context.server_service_binding.bind_address_ip_family().to_string()),
            ),
            (
                label_name!("server_binding_port"),
                LabelValue::new(&connection_context.server_service_binding.bind_address().port().to_string()),
            ),
        ])
    }
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
            Error::Internal { location: _, message } => Self::InternalServer(message.to_string()),
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
