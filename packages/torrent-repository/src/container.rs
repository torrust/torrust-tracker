use std::sync::Arc;

use crate::event::bus::EventBus;
use crate::event::sender::Broadcaster;
use crate::event::{self};
use crate::statistics::repository::Repository;
use crate::{statistics, Swarms};

pub struct TorrentRepositoryContainer {
    pub swarms: Arc<Swarms>,
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
}

impl TorrentRepositoryContainer {
    #[must_use]
    pub fn initialize() -> Self {
        let swarms = Arc::new(Swarms::default());

        // Torrent repository stats
        let broadcaster = Broadcaster::default();
        let stats_repository = Arc::new(Repository::new());

        // todo: add a config option to enable/disable stats for this package
        let event_bus = Arc::new(EventBus::new(true, broadcaster.clone()));

        let stats_event_sender = event_bus.sender();

        Self {
            swarms,
            event_bus,
            stats_event_sender,
            stats_repository,
        }
    }
}
