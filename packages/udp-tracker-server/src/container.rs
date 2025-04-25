use std::sync::Arc;

use torrust_tracker_configuration::Core;

use crate::{event, statistics};

pub struct UdpTrackerServerContainer {
    pub udp_server_stats_keeper: Arc<statistics::keeper::Keeper>,
    pub udp_server_stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub udp_server_stats_repository: Arc<statistics::repository::Repository>,
}

impl UdpTrackerServerContainer {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>) -> Arc<Self> {
        let udp_tracker_server_services = UdpTrackerServerServices::initialize(core_config);

        Arc::new(Self {
            udp_server_stats_keeper: udp_tracker_server_services.udp_server_stats_keeper.clone(),
            udp_server_stats_event_sender: udp_tracker_server_services.udp_server_stats_event_sender.clone(),
            udp_server_stats_repository: udp_tracker_server_services.udp_server_stats_repository.clone(),
        })
    }
}

pub struct UdpTrackerServerServices {
    pub udp_server_stats_keeper: Arc<statistics::keeper::Keeper>,
    pub udp_server_stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub udp_server_stats_repository: Arc<statistics::repository::Repository>,
}

impl UdpTrackerServerServices {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>) -> Arc<Self> {
        let udp_server_stats_keeper = statistics::setup::factory(core_config.tracker_usage_statistics);
        let udp_server_stats_event_sender = udp_server_stats_keeper.sender();
        let udp_server_stats_repository = udp_server_stats_keeper.repository();

        Arc::new(Self {
            udp_server_stats_keeper: udp_server_stats_keeper.clone(),
            udp_server_stats_event_sender: udp_server_stats_event_sender.clone(),
            udp_server_stats_repository: udp_server_stats_repository.clone(),
        })
    }
}
