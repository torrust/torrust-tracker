use std::sync::Arc;

use bittorrent_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_configuration::{Core, HttpTracker};

use crate::services::announce::AnnounceService;
use crate::services::scrape::ScrapeService;
use crate::{event, services, statistics};

pub struct HttpTrackerCoreContainer {
    pub http_tracker_config: Arc<HttpTracker>,

    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // `HttpTrackerCoreServices`
    pub http_stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub http_stats_repository: Arc<statistics::repository::Repository>,
    pub announce_service: Arc<AnnounceService>,
    pub scrape_service: Arc<ScrapeService>,
}

impl HttpTrackerCoreContainer {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>, http_tracker_config: &Arc<HttpTracker>) -> Arc<Self> {
        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize(core_config));
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
            http_stats_event_sender: http_tracker_core_services.http_stats_event_sender.clone(),
            http_stats_repository: http_tracker_core_services.http_stats_repository.clone(),
            announce_service: http_tracker_core_services.http_announce_service.clone(),
            scrape_service: http_tracker_core_services.http_scrape_service.clone(),
        })
    }
}

pub struct HttpTrackerCoreServices {
    pub http_stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>>,
    pub http_stats_repository: Arc<statistics::repository::Repository>,
    pub http_announce_service: Arc<services::announce::AnnounceService>,
    pub http_scrape_service: Arc<services::scrape::ScrapeService>,
}

impl HttpTrackerCoreServices {
    #[must_use]
    pub fn initialize_from(tracker_core_container: &Arc<TrackerCoreContainer>) -> Arc<Self> {
        // HTTP core stats
        let http_core_stats_keeper = statistics::setup::factory(tracker_core_container.core_config.tracker_usage_statistics);
        let http_stats_event_sender = http_core_stats_keeper.sender();
        let http_stats_repository = http_core_stats_keeper.repository();

        if tracker_core_container.core_config.tracker_usage_statistics {
            // todo: this should be started like the other jobs during `app::start`
            // and keep the join handle in a list of jobs.
            let _unused = http_core_stats_keeper.run_event_listener();
        }

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
            http_stats_event_sender,
            http_stats_repository,
            http_announce_service,
            http_scrape_service,
        })
    }
}
