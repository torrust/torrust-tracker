use std::sync::Arc;

use bittorrent_http_tracker_core::container::HttpTrackerCoreContainer;
use bittorrent_http_tracker_core::services::announce::AnnounceService;
use bittorrent_http_tracker_core::services::scrape::ScrapeService;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use bittorrent_udp_tracker_core::services::banning::BanService;
use bittorrent_udp_tracker_core::{self, MAX_CONNECTION_ID_ERRORS_PER_IP};
use tokio::sync::RwLock;
use torrust_rest_tracker_api_core::container::TrackerHttpApiCoreContainer;
use torrust_tracker_configuration::{Configuration, HttpApi, HttpTracker, UdpTracker};
use torrust_udp_tracker_server::container::UdpTrackerServerContainer;
use tracing::instrument;

/* todo: remove duplicate code.

   Use containers from packages as AppContainer fields:

   - bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer
   - bittorrent_http_tracker_core::container::HttpTrackerCoreContainer
   - torrust_udp_tracker_server::container::UdpTrackerServerContainer

   Container initialization is duplicated.
*/

pub struct AppContainer {
    pub tracker_core_container: TrackerCoreContainer,

    // UDP Tracker Core Services
    pub udp_core_stats_event_sender: Arc<Option<Box<dyn bittorrent_udp_tracker_core::event::sender::Sender>>>,
    pub udp_core_stats_repository: Arc<bittorrent_udp_tracker_core::statistics::repository::Repository>,
    pub udp_ban_service: Arc<RwLock<BanService>>,
    pub udp_connect_service: Arc<bittorrent_udp_tracker_core::services::connect::ConnectService>,
    pub udp_announce_service: Arc<bittorrent_udp_tracker_core::services::announce::AnnounceService>,
    pub udp_scrape_service: Arc<bittorrent_udp_tracker_core::services::scrape::ScrapeService>,

    // HTTP Tracker Core Services
    pub http_stats_event_sender: Arc<Option<Box<dyn bittorrent_http_tracker_core::event::sender::Sender>>>,
    pub http_stats_repository: Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,
    pub http_announce_service: Arc<bittorrent_http_tracker_core::services::announce::AnnounceService>,
    pub http_scrape_service: Arc<bittorrent_http_tracker_core::services::scrape::ScrapeService>,

    // UDP Tracker Server Services
    pub udp_server_stats_event_sender: Arc<Option<Box<dyn torrust_udp_tracker_server::event::sender::Sender>>>,
    pub udp_server_stats_repository: Arc<torrust_udp_tracker_server::statistics::repository::Repository>,
}

impl AppContainer {
    #[instrument(skip())]
    pub fn initialize(configuration: &Configuration) -> AppContainer {
        let core_config = Arc::new(configuration.core.clone());

        let tracker_core_container = TrackerCoreContainer::initialize(&core_config);

        // HTTP Tracker Core Services
        let (http_stats_event_sender, http_stats_repository) =
            bittorrent_http_tracker_core::statistics::setup::factory(configuration.core.tracker_usage_statistics);
        let http_stats_event_sender = Arc::new(http_stats_event_sender);
        let http_stats_repository = Arc::new(http_stats_repository);
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

        // UDP Tracker Core Services
        let (udp_core_stats_event_sender, udp_core_stats_repository) =
            bittorrent_udp_tracker_core::statistics::setup::factory(configuration.core.tracker_usage_statistics);
        let udp_core_stats_event_sender = Arc::new(udp_core_stats_event_sender);
        let udp_core_stats_repository = Arc::new(udp_core_stats_repository);
        let udp_ban_service = Arc::new(RwLock::new(BanService::new(MAX_CONNECTION_ID_ERRORS_PER_IP)));
        let udp_connect_service = Arc::new(bittorrent_udp_tracker_core::services::connect::ConnectService::new(
            udp_core_stats_event_sender.clone(),
        ));
        let udp_announce_service = Arc::new(bittorrent_udp_tracker_core::services::announce::AnnounceService::new(
            tracker_core_container.announce_handler.clone(),
            tracker_core_container.whitelist_authorization.clone(),
            udp_core_stats_event_sender.clone(),
        ));
        let udp_scrape_service = Arc::new(bittorrent_udp_tracker_core::services::scrape::ScrapeService::new(
            tracker_core_container.scrape_handler.clone(),
            udp_core_stats_event_sender.clone(),
        ));

        // UDP Tracker Server Services
        let (udp_server_stats_event_sender, udp_server_stats_repository) =
            torrust_udp_tracker_server::statistics::setup::factory(configuration.core.tracker_usage_statistics);
        let udp_server_stats_event_sender = Arc::new(udp_server_stats_event_sender);
        let udp_server_stats_repository = Arc::new(udp_server_stats_repository);

        AppContainer {
            tracker_core_container,

            // UDP Tracker Core Services
            udp_core_stats_event_sender,
            udp_core_stats_repository,
            udp_ban_service,
            udp_connect_service,
            udp_announce_service,
            udp_scrape_service,

            // HTTP Tracker Core Services
            http_stats_event_sender,
            http_stats_repository,
            http_announce_service,
            http_scrape_service,

            // UDP Tracker Server Services
            udp_server_stats_event_sender,
            udp_server_stats_repository,
        }
    }

    #[must_use]
    pub fn http_tracker_container(&self, http_tracker_config: &Arc<HttpTracker>) -> HttpTrackerCoreContainer {
        HttpTrackerCoreContainer {
            core_config: self.tracker_core_container.core_config.clone(),
            announce_handler: self.tracker_core_container.announce_handler.clone(),
            scrape_handler: self.tracker_core_container.scrape_handler.clone(),
            whitelist_authorization: self.tracker_core_container.whitelist_authorization.clone(),
            authentication_service: self.tracker_core_container.authentication_service.clone(),

            http_tracker_config: http_tracker_config.clone(),
            http_stats_event_sender: self.http_stats_event_sender.clone(),
            http_stats_repository: self.http_stats_repository.clone(),
            announce_service: self.http_announce_service.clone(),
            scrape_service: self.http_scrape_service.clone(),
        }
    }

    #[must_use]
    pub fn udp_tracker_container(&self, udp_tracker_config: &Arc<UdpTracker>) -> UdpTrackerCoreContainer {
        UdpTrackerCoreContainer {
            core_config: self.tracker_core_container.core_config.clone(),
            announce_handler: self.tracker_core_container.announce_handler.clone(),
            scrape_handler: self.tracker_core_container.scrape_handler.clone(),
            whitelist_authorization: self.tracker_core_container.whitelist_authorization.clone(),

            udp_tracker_config: udp_tracker_config.clone(),
            udp_core_stats_event_sender: self.udp_core_stats_event_sender.clone(),
            udp_core_stats_repository: self.udp_core_stats_repository.clone(),
            ban_service: self.udp_ban_service.clone(),
            connect_service: self.udp_connect_service.clone(),
            announce_service: self.udp_announce_service.clone(),
            scrape_service: self.udp_scrape_service.clone(),
        }
    }

    #[must_use]
    pub fn tracker_http_api_container(&self, http_api_config: &Arc<HttpApi>) -> TrackerHttpApiCoreContainer {
        TrackerHttpApiCoreContainer {
            core_config: self.tracker_core_container.core_config.clone(),
            in_memory_torrent_repository: self.tracker_core_container.in_memory_torrent_repository.clone(),
            keys_handler: self.tracker_core_container.keys_handler.clone(),
            whitelist_manager: self.tracker_core_container.whitelist_manager.clone(),

            http_api_config: http_api_config.clone(),
            ban_service: self.udp_ban_service.clone(),
            http_stats_repository: self.http_stats_repository.clone(),
            udp_core_stats_repository: self.udp_core_stats_repository.clone(),
            udp_server_stats_repository: self.udp_server_stats_repository.clone(),
        }
    }

    #[must_use]
    pub fn udp_tracker_server_container(&self) -> UdpTrackerServerContainer {
        UdpTrackerServerContainer {
            udp_server_stats_event_sender: self.udp_server_stats_event_sender.clone(),
            udp_server_stats_repository: self.udp_server_stats_repository.clone(),
        }
    }
}
