use std::net::SocketAddr;

use torrust_tracker_metrics::label::{LabelName, LabelSet, LabelValue};
use torrust_tracker_primitives::service_binding::ServiceBinding;

pub mod sender;

/// A UDP core event.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    UdpConnect { context: ConnectionContext },
    UdpAnnounce { context: ConnectionContext },
    UdpScrape { context: ConnectionContext },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConnectionContext {
    pub client_socket_addr: SocketAddr,
    pub server_service_binding: ServiceBinding,
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
                LabelName::new("server_binding_protocol"),
                LabelValue::new(&connection_context.server_service_binding.protocol().to_string()),
            ),
            (
                LabelName::new("server_binding_ip"),
                LabelValue::new(&connection_context.server_service_binding.bind_address().ip().to_string()),
            ),
            (
                LabelName::new("server_binding_port"),
                LabelValue::new(&connection_context.server_service_binding.bind_address().port().to_string()),
            ),
        ])
    }
}
