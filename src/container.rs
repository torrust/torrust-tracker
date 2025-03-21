use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use bittorrent_http_tracker_core::container::HttpTrackerCoreContainer;
use bittorrent_http_tracker_core::services::announce::AnnounceService;
use bittorrent_http_tracker_core::services::scrape::ScrapeService;
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
use torrust_rest_tracker_api_core::container::TrackerHttpApiCoreContainer;
use torrust_tracker_configuration::{Configuration, Core, HttpApi, HttpTracker, UdpTracker};
use torrust_udp_tracker_server::container::UdpTrackerServerContainer;
use tracing::instrument;

/* todo: remove duplicate code.

   Use containers from packages as AppContainer fields:

   - bittorrent_tracker_core::container::TrackerCoreContainer
   - bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer
   - bittorrent_http_tracker_core::container::HttpTrackerCoreContainer
   - torrust_udp_tracker_server::container::UdpTrackerServerContainer

   Container initialization is duplicated.
*/

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

    // HTTP Tracker Server Containers (one container per HTTP Tracker)
    pub http_server_instance_containers: Arc<RwLock<HttpTrackerInstanceContainers>>,

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

        // HTTP Tracker Server Containers (one container per HTTP Tracker)
        let http_server_instance_containers = Arc::new(RwLock::new(HttpTrackerInstanceContainers::default()));

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
            // Tracker Core Services
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

            // HTTP Tracker Server Containers
            http_server_instance_containers,

            // UDP Tracker Server Services
            udp_server_stats_event_sender,
            udp_server_stats_repository,
        }
    }

    #[must_use]
    pub async fn http_tracker_container(&mut self, http_tracker_config: &Arc<HttpTracker>) -> HttpTrackerCoreContainer {
        let http_tracker_instance_container = if let Some(http_tracker_instance_container) = self
            .http_server_instance_containers
            .read()
            .await
            .get(&http_tracker_config.bind_address)
            .await
        {
            http_tracker_instance_container
        } else {
            let http_server_instance_container = Arc::new(HttpTrackerInstanceContainer::initialize(http_tracker_config));

            self.http_server_instance_containers
                .write()
                .await
                .insert(http_tracker_config, http_server_instance_container.clone())
                .await;

            http_server_instance_container
        };

        HttpTrackerCoreContainer {
            core_config: self.core_config.clone(),
            announce_handler: self.announce_handler.clone(),
            scrape_handler: self.scrape_handler.clone(),
            whitelist_authorization: self.whitelist_authorization.clone(),
            authentication_service: self.authentication_service.clone(),

            http_tracker_config: http_tracker_config.clone(),
            http_stats_event_sender: http_tracker_instance_container.http_core_stats_event_sender.clone(),
            http_stats_repository: http_tracker_instance_container.http_core_stats_repository.clone(),
            announce_service: self.http_announce_service.clone(),
            scrape_service: self.http_scrape_service.clone(),
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
            http_api_config: http_api_config.clone(),
            core_config: self.core_config.clone(),
            in_memory_torrent_repository: self.in_memory_torrent_repository.clone(),
            keys_handler: self.keys_handler.clone(),
            whitelist_manager: self.whitelist_manager.clone(),
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

/// Container for each HTTP Tracker Server instance.
///
/// Each instance runs on a different socket address. These services are not
/// shared between instances.
#[derive(Default)]
pub struct HttpTrackerInstanceContainers {
    instances: RwLock<HashMap<SocketAddr, Arc<HttpTrackerInstanceContainer>>>,
}

impl HttpTrackerInstanceContainers {
    pub async fn insert(
        &mut self,
        http_tracker_config: &Arc<HttpTracker>,
        http_server_instance_container: Arc<HttpTrackerInstanceContainer>,
    ) {
        self.instances
            .write()
            .await
            .insert(http_tracker_config.bind_address, http_server_instance_container);
    }

    #[must_use]
    pub async fn get(&self, socket_addr: &SocketAddr) -> Option<Arc<HttpTrackerInstanceContainer>> {
        self.instances.read().await.get(socket_addr).cloned()
    }
}

/// Container for HTTP Tracker Server instances.
#[derive(Clone, Default)]
pub struct HttpTrackerInstanceContainer {
    pub http_core_stats_event_sender: Arc<Option<Box<dyn bittorrent_http_tracker_core::event::sender::Sender>>>,
    pub http_core_stats_repository: Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,
}

impl HttpTrackerInstanceContainer {
    #[must_use]
    pub fn initialize(configuration: &HttpTracker) -> Self {
        let (http_core_stats_event_sender, http_core_stats_repository) =
            bittorrent_http_tracker_core::statistics::setup::factory(configuration.tracker_usage_statistics);

        let http_core_stats_event_sender = Arc::new(http_core_stats_event_sender);
        let http_core_stats_repository = Arc::new(http_core_stats_repository);

        Self {
            http_core_stats_event_sender,
            http_core_stats_repository,
        }
    }
}
