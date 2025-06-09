use std::sync::Arc;

use torrust_tracker_events::bus::SenderStatus;

use crate::event::bus::EventBus;
use crate::event::sender::Broadcaster;
use crate::event::{self};
use crate::statistics::repository::Repository;
use crate::{statistics, Registry};

pub struct SwarmCoordinationRegistryContainer {
    pub swarms: Arc<Registry>,
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
}

impl SwarmCoordinationRegistryContainer {
    #[must_use]
    pub fn initialize(sender_status: SenderStatus) -> Self {
        // // Swarm Coordination Registry Container stats
        let broadcaster = Broadcaster::default();
        let stats_repository = Arc::new(Repository::new());

        let event_bus = Arc::new(EventBus::new(sender_status, broadcaster.clone()));

        let stats_event_sender = event_bus.sender();

        let swarms = Arc::new(Registry::new(stats_event_sender.clone()));

        Self {
            swarms,
            event_bus,
            stats_event_sender,
            stats_repository,
        }
    }
}
