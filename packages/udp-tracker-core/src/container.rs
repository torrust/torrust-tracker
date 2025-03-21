use std::sync::Arc;

use bittorrent_tracker_core::container::TrackerCoreContainer;
use tokio::sync::RwLock;
use torrust_tracker_configuration::{Core, UdpTracker};

use crate::services::announce::AnnounceService;
use crate::services::banning::BanService;
use crate::services::connect::ConnectService;
use crate::services::scrape::ScrapeService;
use crate::{event, statistics, MAX_CONNECTION_ID_ERRORS_PER_IP};

pub struct UdpTrackerCoreContainer {
    pub tracker_core_container: Arc<TrackerCoreContainer>,
    pub udp_tracker_config: Arc<UdpTracker>,
    pub udp_core_stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub udp_core_stats_repository: Arc<statistics::repository::Repository>,
    pub ban_service: Arc<RwLock<BanService>>,
    pub connect_service: Arc<ConnectService>,
    pub announce_service: Arc<AnnounceService>,
    pub scrape_service: Arc<ScrapeService>,
}

impl UdpTrackerCoreContainer {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>, udp_tracker_config: &Arc<UdpTracker>) -> Arc<UdpTrackerCoreContainer> {
        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize(core_config));
        Self::initialize_from(&tracker_core_container, udp_tracker_config)
    }

    #[must_use]
    pub fn initialize_from(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        udp_tracker_config: &Arc<UdpTracker>,
    ) -> Arc<UdpTrackerCoreContainer> {
        let (udp_core_stats_event_sender, udp_core_stats_repository) =
            statistics::setup::factory(tracker_core_container.core_config.tracker_usage_statistics);
        let udp_core_stats_event_sender = Arc::new(udp_core_stats_event_sender);
        let udp_core_stats_repository = Arc::new(udp_core_stats_repository);
        let ban_service = Arc::new(RwLock::new(BanService::new(MAX_CONNECTION_ID_ERRORS_PER_IP)));
        let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender.clone()));
        let announce_service = Arc::new(AnnounceService::new(
            tracker_core_container.announce_handler.clone(),
            tracker_core_container.whitelist_authorization.clone(),
            udp_core_stats_event_sender.clone(),
        ));
        let scrape_service = Arc::new(ScrapeService::new(
            tracker_core_container.scrape_handler.clone(),
            udp_core_stats_event_sender.clone(),
        ));

        Arc::new(UdpTrackerCoreContainer {
            tracker_core_container: tracker_core_container.clone(),
            udp_tracker_config: udp_tracker_config.clone(),
            udp_core_stats_event_sender: udp_core_stats_event_sender.clone(),
            udp_core_stats_repository: udp_core_stats_repository.clone(),
            ban_service: ban_service.clone(),
            connect_service: connect_service.clone(),
            announce_service: announce_service.clone(),
            scrape_service: scrape_service.clone(),
        })
    }
}
