use std::net::{IpAddr, SocketAddr};

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::label_name;
use torrust_tracker_primitives::peer::PeerAnnouncement;
use torrust_tracker_primitives::service_binding::ServiceBinding;

pub mod sender;

/// A HTTP core event.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    TcpAnnounce {
        connection: ConnectionContext,
        info_hash: InfoHash,
        announcement: PeerAnnouncement,
    },
    TcpScrape {
        connection: ConnectionContext,
    },
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
                label_name!("server_binding_protocol"),
                LabelValue::new(&connection_context.server.service_binding.protocol().to_string()),
            ),
            (
                label_name!("server_binding_ip"),
                LabelValue::new(&connection_context.server.service_binding.bind_address().ip().to_string()),
            ),
            (
                label_name!("server_binding_port"),
                LabelValue::new(&connection_context.server.service_binding.bind_address().port().to_string()),
            ),
        ])
    }
}

#[cfg(test)]
pub mod test {

    use torrust_tracker_primitives::peer::Peer;
    use torrust_tracker_primitives::service_binding::Protocol;

    use super::Event;
    use crate::tests::sample_info_hash;

    #[must_use]
    pub fn events_match(event: &Event, expected_event: &Event) -> bool {
        match (event, expected_event) {
            (
                Event::TcpAnnounce {
                    connection,
                    info_hash,
                    announcement,
                },
                Event::TcpAnnounce {
                    connection: expected_connection,
                    info_hash: expected_info_hash,
                    announcement: expected_announcement,
                },
            ) => {
                *connection == *expected_connection
                    && *info_hash == *expected_info_hash
                    && announcement.peer_addr == expected_announcement.peer_addr
            }
            _ => false,
        }
    }

    #[test]
    fn events_should_be_comparable() {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use torrust_tracker_primitives::service_binding::ServiceBinding;

        use crate::event::{ConnectionContext, Event};

        let remote_client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let info_hash = sample_info_hash();

        let event1 = Event::TcpAnnounce {
            connection: ConnectionContext::new(
                remote_client_ip,
                Some(8080),
                ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7070)).unwrap(),
            ),
            info_hash,
            announcement: Peer::default(),
        };

        let event2 = Event::TcpAnnounce {
            connection: ConnectionContext::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)),
                Some(8080),
                ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7070)).unwrap(),
            ),
            info_hash,
            announcement: Peer::default(),
        };

        let event1_clone = event1.clone();

        assert!(event1 == event1_clone);
        assert!(event1 != event2);
    }
}
