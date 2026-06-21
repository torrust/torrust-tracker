use std::net::{IpAddr, SocketAddr};

use torrust_info_hash::InfoHash;
use torrust_metrics::label::{LabelSet, LabelValue};
use torrust_metrics::label_name;
use torrust_net_primitives::service_binding::{IpFamily, IpType, ServiceBinding};
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
            (
                label_name!("client_address_ip_family"),
                LabelValue::new(&connection_context.client_address_ip_family().to_string()),
            ),
            (
                label_name!("client_address_ip_type"),
                LabelValue::new(&connection_context.client_address_ip_type().to_string()),
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
pub(crate) mod tests {

    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use torrust_net_primitives::service_binding::{IpFamily, IpType, Protocol, ServiceBinding};

    use super::ConnectionContext;

    #[test]
    fn client_address_ip_family_should_be_inet_for_ipv4() {
        let ctx = ConnectionContext::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_family(), IpFamily::Inet);
    }

    #[test]
    fn client_address_ip_family_should_be_inet6_for_ipv6() {
        let ctx = ConnectionContext::new(
            SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_family(), IpFamily::Inet6);
    }

    #[test]
    fn client_address_ip_type_should_be_plain_for_direct_ipv4() {
        let ctx = ConnectionContext::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_type(), IpType::Plain);
    }

    #[test]
    fn client_address_ip_type_should_be_plain_for_native_ipv6() {
        let ctx = ConnectionContext::new(
            SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_type(), IpType::Plain);
    }

    #[test]
    fn client_address_ip_type_should_be_v4_mapped_v6_for_ipv4_mapped_ipv6() {
        let v4_mapped_v6_addr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc0a8, 0x0101)); // ::ffff:192.168.1.1

        let ctx = ConnectionContext::new(
            SocketAddr::new(v4_mapped_v6_addr, 6969),
            ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
        );

        assert_eq!(ctx.client_address_ip_type(), IpType::V4MappedV6);
    }
}
