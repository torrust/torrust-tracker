use std::net::{IpAddr, SocketAddr};

use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::RemoteClientAddr;
use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::label_name;
use torrust_tracker_primitives::peer::PeerAnnouncement;
use torrust_tracker_primitives::service_binding::ServiceBinding;

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
    pub fn new(remote_client_addr: RemoteClientAddr, server_service_binding: ServiceBinding) -> Self {
        Self {
            client: ClientConnectionContext { remote_client_addr },
            server: ServerConnectionContext {
                service_binding: server_service_binding,
            },
        }
    }

    #[must_use]
    pub fn client_ip_addr(&self) -> IpAddr {
        self.client.ip_addr()
    }

    #[must_use]
    pub fn client_port(&self) -> Option<u16> {
        self.client.port()
    }

    #[must_use]
    pub fn server_socket_addr(&self) -> SocketAddr {
        self.server.service_binding.bind_address()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ClientConnectionContext {
    remote_client_addr: RemoteClientAddr,
}

impl ClientConnectionContext {
    #[must_use]
    pub fn ip_addr(&self) -> IpAddr {
        self.remote_client_addr.ip()
    }

    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.remote_client_addr.port()
    }
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
                label_name!("server_binding_address_ip_type"),
                LabelValue::new(&connection_context.server.service_binding.bind_address_ip_type().to_string()),
            ),
            (
                label_name!("server_binding_address_ip_family"),
                LabelValue::new(&connection_context.server.service_binding.bind_address_ip_family().to_string()),
            ),
            (
                label_name!("server_binding_port"),
                LabelValue::new(&connection_context.server.service_binding.bind_address().port().to_string()),
            ),
        ])
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
pub mod test {

    use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::{RemoteClientAddr, ResolvedIp};
    use torrust_tracker_primitives::peer::Peer;
    use torrust_tracker_primitives::service_binding::Protocol;

    use super::Event;
    use crate::tests::sample_info_hash;

    #[must_use]
    pub fn announce_events_match(event: &Event, expected_event: &Event) -> bool {
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
                    && announcement.peer_id == expected_announcement.peer_id
                    && announcement.peer_addr == expected_announcement.peer_addr
                    // Events can't be compared due to the `updated` field.
                    // The `announcement.uploaded` contains the current time
                    // when the test is executed.
                    // todo: mock time
                    //&& announcement.updated == expected_announcement.updated
                    && announcement.uploaded == expected_announcement.uploaded
                    && announcement.downloaded == expected_announcement.downloaded
                    && announcement.left == expected_announcement.left
                    && announcement.event == expected_announcement.event
            }
            _ => false,
        }
    }

    #[test]
    fn events_should_be_comparable() {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use torrust_tracker_primitives::service_binding::ServiceBinding;

        use crate::event::{ConnectionContext, Event};

        let remote_client_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let info_hash = sample_info_hash();

        let event1 = Event::TcpAnnounce {
            connection: ConnectionContext::new(
                RemoteClientAddr::new(ResolvedIp::FromSocketAddr(remote_client_ip), Some(8080)),
                ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
            ),
            info_hash,
            announcement: Peer::default(),
        };

        let event2 = Event::TcpAnnounce {
            connection: ConnectionContext::new(
                RemoteClientAddr::new(
                    ResolvedIp::FromSocketAddr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2))),
                    Some(8080),
                ),
                ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
            ),
            info_hash,
            announcement: Peer::default(),
        };

        let event1_clone = event1.clone();

        assert!(event1 == event1_clone);
        assert!(event1 != event2);
    }
}
