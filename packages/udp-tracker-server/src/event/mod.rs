use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;

use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::label_name;
use torrust_tracker_primitives::service_binding::ServiceBinding;

pub mod sender;

/// A UDP server event.
#[derive(Debug, PartialEq, Eq, Clone)]
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
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UdpRequestKind {
    Connect,
    Announce,
    Scrape,
}

impl fmt::Display for UdpRequestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let proto_str = match self {
            UdpRequestKind::Connect => "connect",
            UdpRequestKind::Announce => "announce",
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
                label_name!("server_binding_port"),
                LabelValue::new(&connection_context.server_service_binding.bind_address().port().to_string()),
            ),
        ])
    }
}
