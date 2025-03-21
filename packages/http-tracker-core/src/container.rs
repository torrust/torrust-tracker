use std::sync::Arc;

use bittorrent_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_configuration::{Core, HttpTracker};

use crate::services::announce::AnnounceService;
use crate::services::scrape::ScrapeService;
use crate::{event, statistics};

pub struct HttpTrackerCoreContainer {
    pub tracker_core_container: Arc<TrackerCoreContainer>,
    pub http_tracker_config: Arc<HttpTracker>,
    pub http_stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub http_stats_repository: Arc<statistics::repository::Repository>,
    pub announce_service: Arc<AnnounceService>,
    pub scrape_service: Arc<ScrapeService>,
}

impl HttpTrackerCoreContainer {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>, http_tracker_config: &Arc<HttpTracker>) -> Arc<Self> {
        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize(core_config));
        Self::initialize_from(&tracker_core_container, http_tracker_config)
    }

    #[must_use]
    pub fn initialize_from(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        http_tracker_config: &Arc<HttpTracker>,
    ) -> Arc<Self> {
        let (http_stats_event_sender, http_stats_repository) =
            statistics::setup::factory(tracker_core_container.core_config.tracker_usage_statistics);
        let http_stats_event_sender = Arc::new(http_stats_event_sender);
        let http_stats_repository = Arc::new(http_stats_repository);

        let announce_service = Arc::new(AnnounceService::new(
            tracker_core_container.core_config.clone(),
            tracker_core_container.announce_handler.clone(),
            tracker_core_container.authentication_service.clone(),
            tracker_core_container.whitelist_authorization.clone(),
            http_stats_event_sender.clone(),
        ));

        let scrape_service = Arc::new(ScrapeService::new(
            tracker_core_container.core_config.clone(),
            tracker_core_container.scrape_handler.clone(),
            tracker_core_container.authentication_service.clone(),
            http_stats_event_sender.clone(),
        ));

        Arc::new(Self {
            tracker_core_container: tracker_core_container.clone(),
            http_tracker_config: http_tracker_config.clone(),
            http_stats_event_sender: http_stats_event_sender.clone(),
            http_stats_repository: http_stats_repository.clone(),
            announce_service: announce_service.clone(),
            scrape_service: scrape_service.clone(),
        })
    }
}
