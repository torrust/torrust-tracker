//! HTTP core events.
//!
//! # Design contract: events are objective facts
//!
//! Every variant in [`Event`] describes *what happened* — a neutral, observable
//! fact. Events must not be designed around what a particular consumer should or
//! should not do in response. Policy decisions belong in the consumer or the
//! enforcement point, never in the event definition.
//!
//! See [ADR-20260727000000](../../../docs/adrs/20260727000000_events_are_objective_facts.md)
//! for the full rationale, the concrete counter-example, and naming heuristics.
//!
//! Rejected-request/error events require an additional deliberate contract. Do
//! not add one-off variants solely to support a metric; see the deferred
//! [general error-events EPIC](../../../docs/issues/drafts/generalize-error-events.md)
//! and the [#1987 analysis](../../../docs/issues/closed/1987-add-config-option-to-use-ip-from-announce-query-string/error-event-observability-analysis.md).
use std::net::{IpAddr, SocketAddr};

use torrust_info_hash::InfoHash;
use torrust_metrics::label::{LabelSet, LabelValue};
use torrust_metrics::label_name;
use torrust_net_primitives::service_binding::{IpFamily, IpType, ServiceBinding};
use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::RemoteClientAddr;
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_primitives::peer::PeerAnnouncement;

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
// issue: #2039
// Carries canonical listener identity so shared metrics consumers can apply
// per-instance policy without deriving identity from a socket address.
pub struct ConnectionContext {
    configuration_instance_id: ConfigurationInstanceId,
    client: ClientConnectionContext,
    server: ServerConnectionContext,
    public_url: Option<String>,
}

impl ConnectionContext {
    #[must_use]
    pub fn new(
        configuration_instance_id: ConfigurationInstanceId,
        remote_client_addr: RemoteClientAddr,
        server_service_binding: ServiceBinding,
    ) -> Self {
        Self {
            configuration_instance_id,
            client: ClientConnectionContext { remote_client_addr },
            server: ServerConnectionContext {
                service_binding: server_service_binding,
            },
            public_url: None,
        }
    }

    #[must_use]
    pub fn with_public_url(mut self, public_url: Option<String>) -> Self {
        self.public_url = public_url;
        self
    }

    #[must_use]
    pub const fn configuration_instance_id(&self) -> ConfigurationInstanceId {
        self.configuration_instance_id
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

    #[must_use]
    pub fn public_url(&self) -> Option<&str> {
        self.public_url.as_deref()
    }

    #[must_use]
    pub fn client_address_ip_family(&self) -> IpFamily {
        self.client.ip_addr().into()
    }

    #[must_use]
    pub fn client_address_ip_type(&self) -> IpType {
        match self.client.ip_addr() {
            IpAddr::V6(v6) if v6.to_ipv4_mapped().is_some() => IpType::V4MappedV6,
            _ => IpType::Plain,
        }
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
        let mut label_set = LabelSet::from([
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
            (
                label_name!("client_address_ip_family"),
                LabelValue::new(&connection_context.client_address_ip_family().to_string()),
            ),
            (
                label_name!("client_address_ip_type"),
                LabelValue::new(&connection_context.client_address_ip_type().to_string()),
            ),
        ]);

        // Each configured public URL creates a distinct Prometheus series for
        // every combination of the existing per-service metric labels.
        if let Some(public_url) = connection_context.public_url() {
            label_set.upsert(label_name!("public_url"), LabelValue::new(public_url));
        }

        label_set
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

    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use torrust_metrics::label::{LabelSet, LabelValue};
    use torrust_metrics::label_name;
    use torrust_net_primitives::service_binding::{IpFamily, IpType, Protocol, ServiceBinding};
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{RemoteClientAddr, ResolvedIp};
    use torrust_tracker_primitives::peer::Peer;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

    use super::Event;
    use crate::event::ConnectionContext;
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
        let http_test_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let remote_client_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let info_hash = sample_info_hash();

        let event1 = Event::TcpAnnounce {
            connection: ConnectionContext::new(
                http_test_configuration_instance_id,
                RemoteClientAddr::new(ResolvedIp::FromSocketAddr(remote_client_ip), Some(8080)),
                ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
            ),
            info_hash,
            announcement: Peer::default(),
        };

        let event2 = Event::TcpAnnounce {
            connection: ConnectionContext::new(
                http_test_configuration_instance_id,
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

        assert_eq!(event1, event1_clone);
        assert_ne!(event1, event2);
    }

    #[test]
    fn connection_context_labels_should_include_the_configured_public_url_only_when_present() {
        let connection = ConnectionContext::new(
            ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(IpAddr::V4(Ipv4Addr::LOCALHOST)), Some(8080)),
            ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 7070)).unwrap(),
        )
        .with_public_url(Some("https://tracker.example.test/announce".to_string()));

        let configured_labels = LabelSet::from(connection);
        let absent_labels = LabelSet::from(ConnectionContext::new(
            ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(IpAddr::V4(Ipv4Addr::LOCALHOST)), Some(8080)),
            ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 7070)).unwrap(),
        ));
        let public_url_label = label_name!("public_url");
        let public_url = LabelValue::new("https://tracker.example.test/announce");

        assert!(configured_labels.contains_pair(&public_url_label, &public_url));
        assert!(!absent_labels.contains_pair(&public_url_label, &public_url));
    }

    #[test]
    fn client_address_ip_family_should_be_inet_for_ipv4() {
        let http_test_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let ctx = ConnectionContext::new(
            http_test_configuration_instance_id,
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(IpAddr::V4(Ipv4Addr::LOCALHOST)), Some(8080)),
            ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_family(), IpFamily::Inet);
    }

    #[test]
    fn client_address_ip_family_should_be_inet6_for_ipv6() {
        let http_test_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let ctx = ConnectionContext::new(
            http_test_configuration_instance_id,
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(IpAddr::V6(Ipv6Addr::LOCALHOST)), Some(8080)),
            ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_family(), IpFamily::Inet6);
    }

    #[test]
    fn client_address_ip_type_should_be_plain_for_direct_ipv4() {
        let http_test_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let ctx = ConnectionContext::new(
            http_test_configuration_instance_id,
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(IpAddr::V4(Ipv4Addr::LOCALHOST)), Some(8080)),
            ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_type(), IpType::Plain);
    }

    #[test]
    fn client_address_ip_type_should_be_plain_for_native_ipv6() {
        let http_test_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let ctx = ConnectionContext::new(
            http_test_configuration_instance_id,
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(IpAddr::V6(Ipv6Addr::LOCALHOST)), Some(8080)),
            ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_type(), IpType::Plain);
    }

    #[test]
    fn client_address_ip_type_should_be_v4_mapped_v6_for_ipv4_mapped_ipv6() {
        let http_test_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let v4_mapped_v6_addr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc0a8, 0x0101)); // ::ffff:192.168.1.1

        let ctx = ConnectionContext::new(
            http_test_configuration_instance_id,
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(v4_mapped_v6_addr), Some(8080)),
            ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_type(), IpType::V4MappedV6);
    }
}
