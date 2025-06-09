use std::sync::Arc;

use bittorrent_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_configuration::{Core, HttpTracker};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

use crate::event::bus::EventBus;
use crate::event::sender::Broadcaster;
use crate::services::announce::AnnounceService;
use crate::services::scrape::ScrapeService;
use crate::statistics::repository::Repository;
use crate::{event, services, statistics};

pub struct HttpTrackerCoreContainer {
    pub http_tracker_config: Arc<HttpTracker>,

    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // `HttpTrackerCoreServices`
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub announce_service: Arc<AnnounceService>,
    pub scrape_service: Arc<ScrapeService>,
}

impl HttpTrackerCoreContainer {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>, http_tracker_config: &Arc<HttpTracker>) -> Arc<Self> {
        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize_from(
            core_config,
            &swarm_coordination_registry_container,
        ));

        Self::initialize_from_tracker_core(&tracker_core_container, http_tracker_config)
    }

    #[must_use]
    pub fn initialize_from_tracker_core(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        http_tracker_config: &Arc<HttpTracker>,
    ) -> Arc<Self> {
        let http_tracker_core_services = HttpTrackerCoreServices::initialize_from(tracker_core_container);

        Self::initialize_from_services(tracker_core_container, &http_tracker_core_services, http_tracker_config)
    }

    #[must_use]
    pub fn initialize_from_services(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        http_tracker_core_services: &Arc<HttpTrackerCoreServices>,
        http_tracker_config: &Arc<HttpTracker>,
    ) -> Arc<Self> {
        Arc::new(Self {
            tracker_core_container: tracker_core_container.clone(),
            http_tracker_config: http_tracker_config.clone(),
            event_bus: http_tracker_core_services.event_bus.clone(),
            stats_event_sender: http_tracker_core_services.stats_event_sender.clone(),
            stats_repository: http_tracker_core_services.stats_repository.clone(),
            announce_service: http_tracker_core_services.announce_service.clone(),
            scrape_service: http_tracker_core_services.scrape_service.clone(),
        })
    }
}

pub struct HttpTrackerCoreServices {
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub announce_service: Arc<services::announce::AnnounceService>,
    pub scrape_service: Arc<services::scrape::ScrapeService>,
}

impl HttpTrackerCoreServices {
    #[must_use]
    pub fn initialize_from(tracker_core_container: &Arc<TrackerCoreContainer>) -> Arc<Self> {
        // HTTP core stats
        let http_core_broadcaster = Broadcaster::default();
        let http_stats_repository = Arc::new(Repository::new());
        let http_stats_event_bus = Arc::new(EventBus::new(
            tracker_core_container.core_config.tracker_usage_statistics.into(),
            http_core_broadcaster.clone(),
        ));

        let http_stats_event_sender = http_stats_event_bus.sender();

        let http_announce_service = Arc::new(AnnounceService::new(
            tracker_core_container.core_config.clone(),
            tracker_core_container.announce_handler.clone(),
            tracker_core_container.authentication_service.clone(),
            tracker_core_container.whitelist_authorization.clone(),
            http_stats_event_sender.clone(),
        ));

        let http_scrape_service = Arc::new(ScrapeService::new(
            tracker_core_container.core_config.clone(),
            tracker_core_container.scrape_handler.clone(),
            tracker_core_container.authentication_service.clone(),
            http_stats_event_sender.clone(),
        ));

        Arc::new(Self {
            event_bus: http_stats_event_bus,
            stats_event_sender: http_stats_event_sender,
            stats_repository: http_stats_repository,
            announce_service: http_announce_service,
            scrape_service: http_scrape_service,
        })
    }
}
