use std::net::{IpAddr, SocketAddr};

use torrust_tracker_metrics::label::{LabelName, LabelSet, LabelValue};
use torrust_tracker_primitives::service_binding::ServiceBinding;

pub mod sender;

/// A HTTP core event.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    TcpAnnounce { connection: ConnectionContext },
    TcpScrape { connection: ConnectionContext },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConnectionContext {
    client: ClientConnectionContext,
    server: ServerConnectionContext,
}

impl ConnectionContext {
    #[must_use]
    pub fn new(client_ip_addr: IpAddr, opt_client_port: Option<u16>, server_service_binding: ServiceBinding) -> Self {
        Self {
            client: ClientConnectionContext {
                ip_addr: client_ip_addr,
                port: opt_client_port,
            },
            server: ServerConnectionContext {
                service_binding: server_service_binding,
            },
        }
    }

    #[must_use]
    pub fn client_ip_addr(&self) -> IpAddr {
        self.client.ip_addr
    }

    #[must_use]
    pub fn client_port(&self) -> Option<u16> {
        self.client.port
    }

    #[must_use]
    pub fn server_socket_addr(&self) -> SocketAddr {
        self.server.service_binding.bind_address()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ClientConnectionContext {
    ip_addr: IpAddr,

    /// It's provided if you use the `torrust-axum-http-tracker-server` crate.
    port: Option<u16>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ServerConnectionContext {
    service_binding: ServiceBinding,
}

impl From<ConnectionContext> for LabelSet {
    fn from(connection_context: ConnectionContext) -> Self {
        LabelSet::from([
            (
                LabelName::new("server_binding_protocol"),
                LabelValue::new(&connection_context.server.service_binding.protocol().to_string()),
            ),
            (
                LabelName::new("server_binding_ip"),
                LabelValue::new(&connection_context.server.service_binding.bind_address().ip().to_string()),
            ),
            (
                LabelName::new("server_binding_port"),
                LabelValue::new(&connection_context.server.service_binding.bind_address().port().to_string()),
            ),
        ])
    }
}
