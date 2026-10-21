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
//! [general error-events EPIC](../../../docs/issues/drafts/generalize-error-events/EPIC.md)
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
    pub const fn new(
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
    pub const fn client_ip_addr(&self) -> IpAddr {
        self.client.ip_addr()
    }

    #[must_use]
    pub const fn client_port(&self) -> Option<u16> {
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
    pub const fn client_address_ip_type(&self) -> IpType {
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
    pub const fn ip_addr(&self) -> IpAddr {
        self.remote_client_addr.ip()
    }

    #[must_use]
    pub const fn port(&self) -> Option<u16> {
        self.remote_client_addr.port()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ServerConnectionContext {
    service_binding: ServiceBinding,
}

impl From<ConnectionContext> for LabelSet {
    fn from(connection_context: ConnectionContext) -> Self {
        let mut label_set = Self::from([
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
mod tests {
    use std::net::{IpAddr, SocketAddr};

    use rstest::rstest;
    use torrust_metrics::label::LabelSet;
    use torrust_net_primitives::service_binding::{IpFamily, IpType, Protocol, ServiceBinding};
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{RemoteClientAddr, ResolvedIp};
    use torrust_tracker_primitives::peer::Peer;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

    use super::{ConnectionContext, Event};
    use crate::tests::sample_info_hash;

    const CLIENT_PORT: u16 = 54_321;
    const SERVER_PORT: u16 = 7070;

    fn ip(text: &str) -> IpAddr {
        text.parse().expect("the test IP address should be valid")
    }

    fn instance_id() -> ConfigurationInstanceId {
        ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0)
    }

    fn server_binding(bind_ip: &str) -> ServiceBinding {
        ServiceBinding::new(Protocol::HTTP, SocketAddr::new(ip(bind_ip), SERVER_PORT)).unwrap()
    }

    /// A context for a client at `client_ip`, behind a server bound to `bind_ip`.
    fn connection_between(client_ip: &str, bind_ip: &str) -> ConnectionContext {
        ConnectionContext::new(
            instance_id(),
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(ip(client_ip)), Some(CLIENT_PORT)),
            server_binding(bind_ip),
        )
    }

    /// A context for a client at `client_ip`, behind an IPv4 server.
    fn connection_from(client_ip: &str) -> ConnectionContext {
        connection_between(client_ip, "127.0.0.1")
    }

    fn labels(pairs: &[(&str, &str)]) -> LabelSet {
        LabelSet::from(pairs.to_vec())
    }

    #[rstest]
    #[case::direct_ipv4(connection_from("126.0.0.1"), IpFamily::Inet)]
    #[case::direct_ipv6(connection_from("2001:db8::1"), IpFamily::Inet6)]
    #[case::ipv4_mapped_ipv6(connection_from("::ffff:192.168.1.1"), IpFamily::Inet6)]
    #[case::ipv6_loopback(connection_from("::1"), IpFamily::Inet6)]
    fn it_should_derive_the_client_ip_family_from_the_client_address_not_the_server_binding(
        #[case] connection: ConnectionContext,
        #[case] expected_family: IpFamily,
    ) {
        // Act
        let family = connection.client_address_ip_family();

        // Assert: the server is bound to an IPv4 address in every case.
        assert_eq!(family, expected_family);
    }

    #[test]
    fn it_should_derive_the_client_ip_family_from_the_forwarded_address() {
        // Arrange
        let connection = ConnectionContext::new(
            instance_id(),
            RemoteClientAddr::new(ResolvedIp::FromXForwardedFor(ip("2001:db8::1")), None),
            server_binding("127.0.0.1"),
        );

        // Act
        let family = connection.client_address_ip_family();

        // Assert
        assert_eq!(family, IpFamily::Inet6);
    }

    #[rstest]
    #[case::direct_ipv4(connection_from("126.0.0.1"), IpType::Plain)]
    #[case::native_ipv6(connection_from("2001:db8::1"), IpType::Plain)]
    #[case::ipv6_loopback(connection_from("::1"), IpType::Plain)]
    #[case::ipv4_compatible_ipv6(connection_from("::c0a8:101"), IpType::Plain)]
    #[case::ipv4_mapped_ipv6(connection_from("::ffff:192.168.1.1"), IpType::V4MappedV6)]
    fn it_should_derive_the_client_ip_type_from_the_client_address(
        #[case] connection: ConnectionContext,
        #[case] expected_type: IpType,
    ) {
        // Act
        let ip_type = connection.client_address_ip_type();

        // Assert
        assert_eq!(ip_type, expected_type);
    }

    #[rstest]
    #[case::ipv4_client_and_server(
        connection_between("126.0.0.1", "0.0.0.0"),
        &[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "0.0.0.0"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet"),
            ("client_address_ip_type", "plain"),
        ]
    )]
    #[case::ipv6_client_and_server(
        connection_between("2001:db8::1", "::1"),
        &[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "::1"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet6"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet6"),
            ("client_address_ip_type", "plain"),
        ]
    )]
    #[case::ipv4_mapped_client_of_an_ipv4_server(
        connection_between("::ffff:192.168.1.1", "127.0.0.1"),
        &[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "127.0.0.1"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet6"),
            ("client_address_ip_type", "v4_mapped_v6"),
        ]
    )]
    #[case::configured_public_url(
        connection_between("126.0.0.1", "0.0.0.0").with_public_url(Some("https://tracker.example.test/announce".to_string())),
        &[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "0.0.0.0"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet"),
            ("client_address_ip_type", "plain"),
            ("public_url", "https://tracker.example.test/announce"),
        ]
    )]
    #[case::https_server_on_another_port(
        ConnectionContext::new(
            instance_id(),
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(ip("126.0.0.1")), Some(CLIENT_PORT)),
            ServiceBinding::new(Protocol::HTTPS, SocketAddr::new(ip("0.0.0.0"), 443)).unwrap(),
        ),
        &[
            ("server_binding_protocol", "https"),
            ("server_binding_ip", "0.0.0.0"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet"),
            ("server_binding_port", "443"),
            ("client_address_ip_family", "inet"),
            ("client_address_ip_type", "plain"),
        ]
    )]
    #[case::server_bound_to_an_ipv4_mapped_ipv6_address(
        connection_between("126.0.0.1", "::ffff:127.0.0.1"),
        &[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "::ffff:127.0.0.1"),
            ("server_binding_address_ip_type", "v4_mapped_v6"),
            ("server_binding_address_ip_family", "inet6"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet"),
            ("client_address_ip_type", "plain"),
        ]
    )]
    fn it_should_project_a_connection_context_to_exactly_these_metric_labels(
        #[case] connection: ConnectionContext,
        #[case] expected_labels: &[(&str, &str)],
    ) {
        // Act
        let label_set = LabelSet::from(connection);

        // Assert: the whole set, so a missing, extra or wrong-valued label fails.
        assert_eq!(label_set, labels(expected_labels));
    }

    #[rstest]
    #[case::configured(Some("https://tracker.example.test/announce"))]
    #[case::absent(None)]
    fn it_should_expose_the_public_url_only_when_it_is_configured(#[case] public_url: Option<&str>) {
        // Arrange
        let connection = connection_from("126.0.0.1").with_public_url(public_url.map(str::to_string));

        // Act
        let exposed = connection.public_url();

        // Assert
        assert_eq!(exposed, public_url);
    }

    #[test]
    fn it_should_expose_the_configuration_instance_id_it_was_created_with() {
        // Arrange
        let other_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 3);
        let connection = ConnectionContext::new(
            other_instance_id,
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(ip("126.0.0.1")), Some(CLIENT_PORT)),
            server_binding("127.0.0.1"),
        );

        // Act
        let exposed = connection.configuration_instance_id();

        // Assert
        assert_eq!(exposed, other_instance_id);
    }

    #[rstest]
    #[case::from_the_socket_address(ResolvedIp::FromSocketAddr(ip("126.0.0.1")), Some(CLIENT_PORT), ip("126.0.0.1"))]
    #[case::from_x_forwarded_for_without_a_port(ResolvedIp::FromXForwardedFor(ip("203.0.113.195")), None, ip("203.0.113.195"))]
    fn it_should_expose_the_client_address_and_port(
        #[case] resolved_ip: ResolvedIp,
        #[case] port: Option<u16>,
        #[case] expected_ip: IpAddr,
    ) {
        // Arrange
        let connection = ConnectionContext::new(
            instance_id(),
            RemoteClientAddr::new(resolved_ip, port),
            server_binding("127.0.0.1"),
        );

        // Act
        let (client_ip, client_port) = (connection.client_ip_addr(), connection.client_port());

        // Assert
        assert_eq!((client_ip, client_port), (expected_ip, port));
    }

    #[test]
    fn it_should_expose_the_server_socket_address() {
        // Arrange
        let connection = connection_between("126.0.0.1", "0.0.0.0");

        // Act
        let server_socket_addr = connection.server_socket_addr();

        // Assert
        assert_eq!(server_socket_addr, SocketAddr::new(ip("0.0.0.0"), SERVER_PORT));
    }

    #[rstest]
    #[case::another_instance(ConnectionContext::new(
        ConfigurationInstanceId::new(ServiceRole::HttpTracker, 1),
        RemoteClientAddr::new(ResolvedIp::FromSocketAddr(ip("126.0.0.1")), Some(CLIENT_PORT)),
        server_binding("127.0.0.1"),
    ))]
    #[case::another_client_address(connection_from("126.0.0.2"))]
    #[case::another_client_port(ConnectionContext::new(
        instance_id(),
        RemoteClientAddr::new(ResolvedIp::FromSocketAddr(ip("126.0.0.1")), Some(CLIENT_PORT + 1)),
        server_binding("127.0.0.1"),
    ))]
    #[case::another_way_of_resolving_the_address(ConnectionContext::new(
        instance_id(),
        RemoteClientAddr::new(ResolvedIp::FromXForwardedFor(ip("126.0.0.1")), Some(CLIENT_PORT)),
        server_binding("127.0.0.1"),
    ))]
    #[case::another_server_address(connection_between("126.0.0.1", "127.0.0.2"))]
    #[case::a_public_url(connection_from("126.0.0.1").with_public_url(Some("https://tracker.example.test/".to_string())))]
    fn it_should_not_consider_two_connection_contexts_equal_when_any_part_differs(#[case] other: ConnectionContext) {
        // Arrange
        let connection = connection_from("126.0.0.1");

        // Act
        let equal = connection == other;

        // Assert
        assert!(!equal, "{connection:?} should differ from {other:?}");
    }

    #[test]
    fn it_should_consider_a_cloned_event_equal_to_the_original() {
        // Arrange
        let event = Event::TcpAnnounce {
            connection: connection_from("126.0.0.1"),
            info_hash: sample_info_hash(),
            announcement: Peer::default(),
        };

        // Act
        let cloned = event.clone();

        // Assert
        assert_eq!(event, cloned);
    }
}
