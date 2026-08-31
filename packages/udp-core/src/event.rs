//! UDP core events.
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
//! Error-event coverage is intentionally deferred until the [general
//! error-events EPIC](../../../docs/issues/drafts/generalize-error-events.md)
//! defines a stable cross-service contract.
use std::net::{IpAddr, SocketAddr};

use torrust_info_hash::InfoHash;
use torrust_metrics::label::{LabelSet, LabelValue};
use torrust_metrics::label_name;
use torrust_net_primitives::service_binding::{IpFamily, IpType, ServiceBinding};
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_primitives::peer::PeerAnnouncement;

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
// issue: #2039
// Carries canonical listener identity so shared metrics consumers can apply
// per-instance policy without deriving identity from a socket address.
pub struct ConnectionContext {
    configuration_instance_id: ConfigurationInstanceId,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    public_url: Option<String>,
}

impl ConnectionContext {
    #[must_use]
    pub fn new(
        configuration_instance_id: ConfigurationInstanceId,
        client_socket_addr: SocketAddr,
        server_service_binding: ServiceBinding,
    ) -> Self {
        Self {
            configuration_instance_id,
            client_socket_addr,
            server_service_binding,
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
    pub fn client_socket_addr(&self) -> SocketAddr {
        self.client_socket_addr
    }

    #[must_use]
    pub fn server_socket_addr(&self) -> SocketAddr {
        self.server_service_binding.bind_address()
    }

    #[must_use]
    pub fn public_url(&self) -> Option<&str> {
        self.public_url.as_deref()
    }

    #[must_use]
    pub fn client_address_ip_family(&self) -> IpFamily {
        self.client_socket_addr.ip().into()
    }

    #[must_use]
    pub fn client_address_ip_type(&self) -> IpType {
        match self.client_socket_addr.ip() {
            IpAddr::V6(v6) if v6.to_ipv4_mapped().is_some() => IpType::V4MappedV6,
            _ => IpType::Plain,
        }
    }
}

impl From<ConnectionContext> for LabelSet {
    fn from(connection_context: ConnectionContext) -> Self {
        let mut label_set = LabelSet::from([
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
pub(crate) mod tests {

    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use torrust_metrics::label::{LabelSet, LabelValue};
    use torrust_metrics::label_name;
    use torrust_net_primitives::service_binding::{IpFamily, IpType, Protocol, ServiceBinding};
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

    use super::ConnectionContext;

    #[test]
    fn client_address_ip_family_should_be_inet_for_ipv4() {
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let ctx = ConnectionContext::new(
            configuration_instance_id,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_family(), IpFamily::Inet);
    }

    #[test]
    fn it_should_retain_an_optional_configured_public_url() {
        let ctx = ConnectionContext::new(
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        )
        .with_public_url(Some("udp://tracker.example.test:6969/announce".to_string()));

        assert_eq!(ctx.public_url(), Some("udp://tracker.example.test:6969/announce"));
    }

    #[test]
    fn connection_context_labels_should_include_the_configured_public_url_only_when_present() {
        let connection = ConnectionContext::new(
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 6969)).unwrap(),
        )
        .with_public_url(Some("udp://tracker.example.test:6969/announce".to_string()));

        let configured_labels = LabelSet::from(connection);
        let absent_labels = LabelSet::from(ConnectionContext::new(
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 6969)).unwrap(),
        ));
        let public_url_label = label_name!("public_url");
        let public_url = LabelValue::new("udp://tracker.example.test:6969/announce");

        assert!(configured_labels.contains_pair(&public_url_label, &public_url));
        assert!(!absent_labels.contains_pair(&public_url_label, &public_url));
    }

    #[test]
    fn client_address_ip_family_should_be_inet6_for_ipv6() {
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let ctx = ConnectionContext::new(
            configuration_instance_id,
            SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_family(), IpFamily::Inet6);
    }

    #[test]
    fn client_address_ip_type_should_be_plain_for_direct_ipv4() {
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let ctx = ConnectionContext::new(
            configuration_instance_id,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_type(), IpType::Plain);
    }

    #[test]
    fn client_address_ip_type_should_be_plain_for_native_ipv6() {
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let ctx = ConnectionContext::new(
            configuration_instance_id,
            SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_type(), IpType::Plain);
    }

    #[test]
    fn client_address_ip_type_should_be_v4_mapped_v6_for_ipv4_mapped_ipv6() {
        let v4_mapped_v6_addr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc0a8, 0x0101)); // ::ffff:192.168.1.1
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);

        let ctx = ConnectionContext::new(
            configuration_instance_id,
            SocketAddr::new(v4_mapped_v6_addr, 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_type(), IpType::V4MappedV6);
    }
}
