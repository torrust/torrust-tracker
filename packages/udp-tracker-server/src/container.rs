use std::sync::Arc;

use torrust_tracker_configuration::Core;

use crate::{event, statistics};

pub struct UdpTrackerServerContainer {
    pub udp_server_stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub udp_server_stats_repository: Arc<statistics::repository::Repository>,
}

impl UdpTrackerServerContainer {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>) -> Arc<Self> {
        let (udp_server_stats_event_sender, udp_server_stats_repository) =
            statistics::setup::factory(core_config.tracker_usage_statistics);
        let udp_server_stats_event_sender = Arc::new(udp_server_stats_event_sender);
        let udp_server_stats_repository = Arc::new(udp_server_stats_repository);

        Arc::new(Self {
            udp_server_stats_event_sender: udp_server_stats_event_sender.clone(),
            udp_server_stats_repository: udp_server_stats_repository.clone(),
        })
    }
}
