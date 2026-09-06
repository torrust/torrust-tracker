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
