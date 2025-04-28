pub mod bus;
pub mod receiver;
pub mod sender;

use std::net::SocketAddr;

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::label_name;
use torrust_tracker_primitives::peer::PeerAnnouncement;
use torrust_tracker_primitives::service_binding::ServiceBinding;

/// A UDP core event.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    UdpConnect {
        connection: ConnectionContext,
    },
    UdpAnnounce {
        connection: ConnectionContext,
        info_hash: InfoHash,
        announcement: PeerAnnouncement,
    },
    UdpScrape {
        connection: ConnectionContext,
    },
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
