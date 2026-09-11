use std::sync::Arc;

use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_events::bus::SenderStatus;

use crate::event::bus::EventBus;
use crate::event::sender::Broadcaster;
use crate::event::{self};
use crate::statistics;
use crate::statistics::repository::Repository;

pub struct UdpTrackerServerContainer {
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: crate::event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
}

impl UdpTrackerServerContainer {
    #[must_use]
    pub fn initialize(_core_config: &Arc<Core>) -> Arc<Self> {
        let udp_tracker_server_services = UdpTrackerServerServices::initialize();

        Arc::new(Self {
            event_bus: udp_tracker_server_services.event_bus.clone(),
            stats_event_sender: udp_tracker_server_services.stats_event_sender.clone(),
            stats_repository: udp_tracker_server_services.stats_repository.clone(),
        })
    }
}

pub struct UdpTrackerServerServices {
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: crate::event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
}

impl UdpTrackerServerServices {
    #[must_use]
    pub fn initialize() -> Arc<Self> {
        let udp_server_broadcaster = Broadcaster::default();
        let udp_server_stats_repository = Arc::new(Repository::new());
        // issue: #2039
        // issue-spec: docs/issues/drafts/optimize-event-publication-without-consumers/ISSUE.md
        // Always publish UDP-server facts: metrics filtering is consumer-side,
        // and the banning listener also requires cookie-error facts regardless
        // of the originating listener's metrics policy. Any future demand-based
        // optimization must first prove that no required consumer is active.
        let udp_server_stats_event_bus = Arc::new(EventBus::new(SenderStatus::Enabled, udp_server_broadcaster));

        let udp_server_stats_event_sender = udp_server_stats_event_bus.sender();

        Arc::new(Self {
            event_bus: udp_server_stats_event_bus.clone(),
            stats_event_sender: udp_server_stats_event_sender.clone(),
            stats_repository: udp_server_stats_repository,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::time::Duration;

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_udp_core::event::ConnectionContext;

    use super::UdpTrackerServerServices;
    use crate::event::Event;

    const EVENT_PUBLICATION_TIMEOUT: Duration = Duration::from_secs(1);

    fn sample_udp_request_received_event() -> Event {
        Event::UdpRequestReceived {
            context: ConnectionContext::new(
                ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                ServiceBinding::new(
                    Protocol::UDP,
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                )
                .expect("sample UDP service binding should be valid"),
            ),
        }
    }

    #[tokio::test]
    async fn should_publish_events_through_the_enabled_server_event_bus() {
        // Arrange
        let services = UdpTrackerServerServices::initialize();
        let event = sample_udp_request_received_event();
        let mut event_receiver = services.event_bus.receiver();
        let event_sender = services
            .stats_event_sender
            .as_deref()
            .expect("UDP-server services should enable event publication");

        // Act
        event_sender
            .send(event.clone())
            .await
            .expect("event sender should be active")
            .expect("event should be delivered to the connected receiver");

        // Assert
        assert_eq!(
            tokio::time::timeout(EVENT_PUBLICATION_TIMEOUT, event_receiver.recv())
                .await
                .expect("event should be received before the test deadline")
                .expect("event receiver should remain connected"),
            event
        );
    }
}
