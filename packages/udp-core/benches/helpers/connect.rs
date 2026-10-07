use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_tracker_events::bus::SenderStatus;
use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
use torrust_tracker_udp_core::event::bus::EventBus;
use torrust_tracker_udp_core::event::sender::Broadcaster;
use torrust_tracker_udp_core::services::connect::ConnectService;
use torrust_tracker_udp_protocol::ConnectionId;

use crate::helpers::utils::{sample_ipv4_remote_addr, sample_issue_time};

/// A connect service built once, outside the measured loop.
///
/// Events are disabled, so a connect measures the client fingerprint and the
/// connection-cookie encryption.
pub struct ConnectBenchmarkContext {
    connect_service: ConnectService,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
}

impl ConnectBenchmarkContext {
    /// # Panics
    ///
    /// Panics if the fixed server address is not a valid UDP service binding.
    #[must_use]
    pub fn new() -> Self {
        let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
        let event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, Broadcaster::default()));

        Self {
            connect_service: ConnectService::new(event_bus.sender(), ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0)),
            client_socket_addr: sample_ipv4_remote_addr(),
            server_service_binding: ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap(),
        }
    }

    pub async fn connect_once(&self) -> ConnectionId {
        self.connect_service
            .handle_connect(
                self.client_socket_addr,
                self.server_service_binding.clone(),
                sample_issue_time(),
            )
            .await
    }
}
