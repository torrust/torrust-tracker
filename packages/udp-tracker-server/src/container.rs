use std::sync::Arc;

use torrust_tracker_configuration::Core;

use crate::{event, statistics};

pub struct UdpTrackerServerContainer {
    pub stats_keeper: Arc<statistics::keeper::Keeper>,
    pub stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub stats_repository: Arc<statistics::repository::Repository>,
}

impl UdpTrackerServerContainer {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>) -> Arc<Self> {
        let udp_tracker_server_services = UdpTrackerServerServices::initialize(core_config);

        Arc::new(Self {
            stats_keeper: udp_tracker_server_services.stats_keeper.clone(),
            stats_event_sender: udp_tracker_server_services.stats_event_sender.clone(),
            stats_repository: udp_tracker_server_services.stats_repository.clone(),
        })
    }
}

pub struct UdpTrackerServerServices {
    pub stats_keeper: Arc<statistics::keeper::Keeper>,
    pub stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub stats_repository: Arc<statistics::repository::Repository>,
}

impl UdpTrackerServerServices {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>) -> Arc<Self> {
        let (udp_server_stats_keeper, udp_server_stats_repository) =
            statistics::setup::factory(core_config.tracker_usage_statistics);
        let udp_server_stats_event_sender = udp_server_stats_keeper.sender();

        Arc::new(Self {
            stats_keeper: udp_server_stats_keeper.clone(),
            stats_event_sender: udp_server_stats_event_sender.clone(),
            stats_repository: udp_server_stats_repository.clone(),
        })
    }
}
