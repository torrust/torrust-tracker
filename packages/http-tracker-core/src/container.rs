use std::sync::Arc;

use bittorrent_tracker_core::announce_handler::AnnounceHandler;
use bittorrent_tracker_core::authentication::service::AuthenticationService;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_tracker_core::scrape_handler::ScrapeHandler;
use bittorrent_tracker_core::whitelist;
use torrust_tracker_configuration::{Core, HttpTracker};

use crate::statistics;

pub struct HttpTrackerCoreContainer {
    // todo: replace with TrackerCoreContainer
    pub core_config: Arc<Core>,
    pub announce_handler: Arc<AnnounceHandler>,
    pub scrape_handler: Arc<ScrapeHandler>,
    pub whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    pub authentication_service: Arc<AuthenticationService>,

    pub http_tracker_config: Arc<HttpTracker>,
    pub http_stats_event_sender: Arc<Option<Box<dyn statistics::event::sender::Sender>>>,
    pub http_stats_repository: Arc<statistics::repository::Repository>,
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

        Arc::new(Self {
            core_config: tracker_core_container.core_config.clone(),
            announce_handler: tracker_core_container.announce_handler.clone(),
            scrape_handler: tracker_core_container.scrape_handler.clone(),
            whitelist_authorization: tracker_core_container.whitelist_authorization.clone(),
            authentication_service: tracker_core_container.authentication_service.clone(),

            http_tracker_config: http_tracker_config.clone(),
            http_stats_event_sender: http_stats_event_sender.clone(),
            http_stats_repository: http_stats_repository.clone(),
        })
    }
}
