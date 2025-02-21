use std::sync::Arc;

use bittorrent_tracker_core::announce_handler::AnnounceHandler;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_tracker_core::scrape_handler::ScrapeHandler;
use bittorrent_tracker_core::whitelist;
use tokio::sync::RwLock;
use torrust_tracker_configuration::{Core, UdpTracker};

use crate::services::banning::BanService;
use crate::{statistics, MAX_CONNECTION_ID_ERRORS_PER_IP};

pub struct UdpTrackerCoreContainer {
    // todo: replace with TrackerCoreContainer
    pub core_config: Arc<Core>,
    pub announce_handler: Arc<AnnounceHandler>,
    pub scrape_handler: Arc<ScrapeHandler>,
    pub whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,

    pub udp_tracker_config: Arc<UdpTracker>,
    pub udp_stats_event_sender: Arc<Option<Box<dyn statistics::event::sender::Sender>>>,
    pub udp_stats_repository: Arc<statistics::repository::Repository>,
    pub ban_service: Arc<RwLock<BanService>>,
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
        let (udp_stats_event_sender, udp_stats_repository) =
            statistics::setup::factory(tracker_core_container.core_config.tracker_usage_statistics);
        let udp_stats_event_sender = Arc::new(udp_stats_event_sender);
        let udp_stats_repository = Arc::new(udp_stats_repository);

        let ban_service = Arc::new(RwLock::new(BanService::new(MAX_CONNECTION_ID_ERRORS_PER_IP)));

        Arc::new(UdpTrackerCoreContainer {
            core_config: tracker_core_container.core_config.clone(),
            announce_handler: tracker_core_container.announce_handler.clone(),
            scrape_handler: tracker_core_container.scrape_handler.clone(),
            whitelist_authorization: tracker_core_container.whitelist_authorization.clone(),

            udp_tracker_config: udp_tracker_config.clone(),
            udp_stats_event_sender: udp_stats_event_sender.clone(),
            udp_stats_repository: udp_stats_repository.clone(),
            ban_service: ban_service.clone(),
        })
    }
}
