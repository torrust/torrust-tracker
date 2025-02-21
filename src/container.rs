use std::sync::Arc;

use bittorrent_http_tracker_core::container::HttpTrackerCoreContainer;
use bittorrent_tracker_core::announce_handler::AnnounceHandler;
use bittorrent_tracker_core::authentication::handler::KeysHandler;
use bittorrent_tracker_core::authentication::service::AuthenticationService;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_tracker_core::databases::Database;
use bittorrent_tracker_core::scrape_handler::ScrapeHandler;
use bittorrent_tracker_core::torrent::manager::TorrentsManager;
use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use bittorrent_tracker_core::torrent::repository::persisted::DatabasePersistentTorrentRepository;
use bittorrent_tracker_core::whitelist;
use bittorrent_tracker_core::whitelist::manager::WhitelistManager;
use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use bittorrent_udp_tracker_core::services::banning::BanService;
use bittorrent_udp_tracker_core::{self, MAX_CONNECTION_ID_ERRORS_PER_IP};
use tokio::sync::RwLock;
use torrust_tracker_api_core::container::TrackerHttpApiCoreContainer;
use torrust_tracker_configuration::{Configuration, Core, HttpApi, HttpTracker, UdpTracker};
use tracing::instrument;

pub struct AppContainer {
    // Tracker Core Services
    pub core_config: Arc<Core>,
    pub database: Arc<Box<dyn Database>>,
    pub announce_handler: Arc<AnnounceHandler>,
    pub scrape_handler: Arc<ScrapeHandler>,
    pub keys_handler: Arc<KeysHandler>,
    pub authentication_service: Arc<AuthenticationService>,
    pub in_memory_whitelist: Arc<InMemoryWhitelist>,
    pub whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    pub whitelist_manager: Arc<WhitelistManager>,
    pub in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    pub db_torrent_repository: Arc<DatabasePersistentTorrentRepository>,
    pub torrents_manager: Arc<TorrentsManager>,

    // UDP Tracker Core Services
    pub ban_service: Arc<RwLock<BanService>>,
    pub udp_stats_event_sender: Arc<Option<Box<dyn bittorrent_udp_tracker_core::statistics::event::sender::Sender>>>,

    // HTTP Tracker Core Services
    pub http_stats_event_sender: Arc<Option<Box<dyn bittorrent_http_tracker_core::statistics::event::sender::Sender>>>,
    pub http_stats_repository: Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,
    pub udp_stats_repository: Arc<bittorrent_udp_tracker_core::statistics::repository::Repository>,
}

impl AppContainer {
    #[instrument(skip())]
    pub fn initialize(configuration: &Configuration) -> AppContainer {
        let core_config = Arc::new(configuration.core.clone());

        let tracker_core_container = TrackerCoreContainer::initialize(&core_config);

        // HTTP stats
        let (http_stats_event_sender, http_stats_repository) =
            bittorrent_http_tracker_core::statistics::setup::factory(configuration.core.tracker_usage_statistics);
        let http_stats_event_sender = Arc::new(http_stats_event_sender);
        let http_stats_repository = Arc::new(http_stats_repository);

        // UDP stats
        let (udp_stats_event_sender, udp_stats_repository) =
            bittorrent_udp_tracker_core::statistics::setup::factory(configuration.core.tracker_usage_statistics);
        let udp_stats_event_sender = Arc::new(udp_stats_event_sender);
        let udp_stats_repository = Arc::new(udp_stats_repository);

        let ban_service = Arc::new(RwLock::new(BanService::new(MAX_CONNECTION_ID_ERRORS_PER_IP)));

        AppContainer {
            core_config,
            database: tracker_core_container.database,
            announce_handler: tracker_core_container.announce_handler,
            scrape_handler: tracker_core_container.scrape_handler,
            keys_handler: tracker_core_container.keys_handler,
            authentication_service: tracker_core_container.authentication_service,
            in_memory_whitelist: tracker_core_container.in_memory_whitelist,
            whitelist_authorization: tracker_core_container.whitelist_authorization,
            whitelist_manager: tracker_core_container.whitelist_manager,
            in_memory_torrent_repository: tracker_core_container.in_memory_torrent_repository,
            db_torrent_repository: tracker_core_container.db_torrent_repository,
            torrents_manager: tracker_core_container.torrents_manager,
            ban_service,
            http_stats_event_sender,
            udp_stats_event_sender,
            http_stats_repository,
            udp_stats_repository,
        }
    }

    #[must_use]
    pub fn http_tracker_container(&self, http_tracker_config: &Arc<HttpTracker>) -> HttpTrackerCoreContainer {
        HttpTrackerCoreContainer {
            core_config: self.core_config.clone(),
            announce_handler: self.announce_handler.clone(),
            scrape_handler: self.scrape_handler.clone(),
            whitelist_authorization: self.whitelist_authorization.clone(),
            authentication_service: self.authentication_service.clone(),

            http_tracker_config: http_tracker_config.clone(),
            http_stats_event_sender: self.http_stats_event_sender.clone(),
            http_stats_repository: self.http_stats_repository.clone(),
        }
    }

    #[must_use]
    pub fn udp_tracker_container(&self, udp_tracker_config: &Arc<UdpTracker>) -> UdpTrackerCoreContainer {
        UdpTrackerCoreContainer {
            core_config: self.core_config.clone(),
            announce_handler: self.announce_handler.clone(),
            scrape_handler: self.scrape_handler.clone(),
            whitelist_authorization: self.whitelist_authorization.clone(),

            udp_tracker_config: udp_tracker_config.clone(),
            udp_stats_event_sender: self.udp_stats_event_sender.clone(),
            udp_stats_repository: self.udp_stats_repository.clone(),
            ban_service: self.ban_service.clone(),
        }
    }

    #[must_use]
    pub fn tracker_http_api_container(&self, http_api_config: &Arc<HttpApi>) -> TrackerHttpApiCoreContainer {
        TrackerHttpApiCoreContainer {
            http_api_config: http_api_config.clone(),
            core_config: self.core_config.clone(),
            in_memory_torrent_repository: self.in_memory_torrent_repository.clone(),
            keys_handler: self.keys_handler.clone(),
            whitelist_manager: self.whitelist_manager.clone(),
            ban_service: self.ban_service.clone(),
            http_stats_repository: self.http_stats_repository.clone(),
            udp_stats_repository: self.udp_stats_repository.clone(),
        }
    }
}
