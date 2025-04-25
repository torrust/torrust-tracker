use std::sync::Arc;

use bittorrent_tracker_core::container::TrackerCoreContainer;
use tokio::sync::RwLock;
use torrust_tracker_configuration::{Core, UdpTracker};

use crate::services::announce::AnnounceService;
use crate::services::banning::BanService;
use crate::services::connect::ConnectService;
use crate::services::scrape::ScrapeService;
use crate::{event, services, statistics, MAX_CONNECTION_ID_ERRORS_PER_IP};

pub struct UdpTrackerCoreContainer {
    pub udp_tracker_config: Arc<UdpTracker>,

    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // `UdpTrackerCoreServices`
    pub stats_keeper: Arc<statistics::keeper::Keeper>,
    pub stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub ban_service: Arc<RwLock<BanService>>,
    pub connect_service: Arc<ConnectService>,
    pub announce_service: Arc<AnnounceService>,
    pub scrape_service: Arc<ScrapeService>,
}

impl UdpTrackerCoreContainer {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>, udp_tracker_config: &Arc<UdpTracker>) -> Arc<UdpTrackerCoreContainer> {
        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize(core_config));
        Self::initialize_from_tracker_core(&tracker_core_container, udp_tracker_config)
    }

    #[must_use]
    pub fn initialize_from_tracker_core(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        udp_tracker_config: &Arc<UdpTracker>,
    ) -> Arc<UdpTrackerCoreContainer> {
        let udp_tracker_core_services = UdpTrackerCoreServices::initialize_from(tracker_core_container);
        Self::initialize_from_services(tracker_core_container, &udp_tracker_core_services, udp_tracker_config)
    }

    #[must_use]
    pub fn initialize_from_services(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        udp_tracker_core_services: &Arc<UdpTrackerCoreServices>,
        udp_tracker_config: &Arc<UdpTracker>,
    ) -> Arc<Self> {
        Arc::new(Self {
            udp_tracker_config: udp_tracker_config.clone(),

            tracker_core_container: tracker_core_container.clone(),

            // `UdpTrackerCoreServices`
            stats_keeper: udp_tracker_core_services.stats_keeper.clone(),
            stats_event_sender: udp_tracker_core_services.stats_event_sender.clone(),
            stats_repository: udp_tracker_core_services.stats_repository.clone(),
            ban_service: udp_tracker_core_services.ban_service.clone(),
            connect_service: udp_tracker_core_services.connect_service.clone(),
            announce_service: udp_tracker_core_services.announce_service.clone(),
            scrape_service: udp_tracker_core_services.scrape_service.clone(),
        })
    }
}

pub struct UdpTrackerCoreServices {
    pub stats_keeper: Arc<statistics::keeper::Keeper>,
    pub stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub ban_service: Arc<RwLock<services::banning::BanService>>,
    pub connect_service: Arc<services::connect::ConnectService>,
    pub announce_service: Arc<services::announce::AnnounceService>,
    pub scrape_service: Arc<services::scrape::ScrapeService>,
}

impl UdpTrackerCoreServices {
    #[must_use]
    pub fn initialize_from(tracker_core_container: &Arc<TrackerCoreContainer>) -> Arc<Self> {
        let keeper = statistics::setup::factory(tracker_core_container.core_config.tracker_usage_statistics);
        let udp_core_stats_event_sender = keeper.sender();
        let udp_core_stats_repository = keeper.repository();
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

        Arc::new(Self {
            stats_keeper: keeper,
            stats_event_sender: udp_core_stats_event_sender,
            stats_repository: udp_core_stats_repository,
            ban_service,
            connect_service,
            announce_service,
            scrape_service,
        })
    }
}
