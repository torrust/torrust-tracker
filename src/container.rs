use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use bittorrent_http_tracker_core::container::{HttpTrackerCoreContainer, HttpTrackerCoreServices};
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use bittorrent_udp_tracker_core::services::banning::BanService;
use bittorrent_udp_tracker_core::{self, MAX_CONNECTION_ID_ERRORS_PER_IP};
use tokio::sync::RwLock;
use torrust_rest_tracker_api_core::container::TrackerHttpApiCoreContainer;
use torrust_tracker_configuration::{Configuration, HttpApi};
use torrust_udp_tracker_server::container::UdpTrackerServerContainer;
use tracing::instrument;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("There is not a HTTP tracker server instance bound to the socket address: {bind_address}")]
    MissingHttpTrackerCoreContainer { bind_address: SocketAddr },

    #[error("There is not a UDP tracker server instance bound to the socket address: {bind_address}")]
    MissingUdpTrackerCoreContainer { bind_address: SocketAddr },
}

pub struct AppContainer {
    pub tracker_core_container: Arc<TrackerCoreContainer>,
    pub http_api_config: Arc<Option<HttpApi>>,
    pub http_tracker_core_services: Arc<HttpTrackerCoreServices>,

    // UDP Tracker Core Services
    pub udp_core_stats_event_sender: Arc<Option<Box<dyn bittorrent_udp_tracker_core::event::sender::Sender>>>,
    pub udp_core_stats_repository: Arc<bittorrent_udp_tracker_core::statistics::repository::Repository>,
    pub udp_ban_service: Arc<RwLock<BanService>>,
    pub udp_connect_service: Arc<bittorrent_udp_tracker_core::services::connect::ConnectService>,
    pub udp_announce_service: Arc<bittorrent_udp_tracker_core::services::announce::AnnounceService>,
    pub udp_scrape_service: Arc<bittorrent_udp_tracker_core::services::scrape::ScrapeService>,

    // UDP Tracker Server Services
    pub udp_server_stats_event_sender: Arc<Option<Box<dyn torrust_udp_tracker_server::event::sender::Sender>>>,
    pub udp_server_stats_repository: Arc<torrust_udp_tracker_server::statistics::repository::Repository>,

    // UDP Tracker Server Container
    pub udp_tracker_server_container: Arc<UdpTrackerServerContainer>,

    // Tracker Instance Containers
    pub http_tracker_containers: Arc<HashMap<SocketAddr, Arc<HttpTrackerCoreContainer>>>,
    pub udp_tracker_containers: Arc<HashMap<SocketAddr, Arc<UdpTrackerCoreContainer>>>,
}

impl AppContainer {
    #[instrument(skip())]
    pub fn initialize(configuration: &Configuration) -> AppContainer {
        let core_config = Arc::new(configuration.core.clone());

        let http_api_config = Arc::new(configuration.http_api.clone());

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize(&core_config));

        let http_tracker_core_services = HttpTrackerCoreServices::initialize_from(&tracker_core_container);

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

        // UDP Tracker Server Container
        let udp_tracker_server_container = Arc::new(UdpTrackerServerContainer {
            udp_server_stats_event_sender: udp_server_stats_event_sender.clone(),
            udp_server_stats_repository: udp_server_stats_repository.clone(),
        });

        // Tracker Instance Containers

        let mut http_tracker_containers = HashMap::new();

        if let Some(http_trackers) = &configuration.http_trackers {
            for http_tracker_config in http_trackers {
                http_tracker_containers.insert(
                    http_tracker_config.bind_address,
                    HttpTrackerCoreContainer::initialize_from_services(
                        &tracker_core_container,
                        &http_tracker_core_services,
                        &Arc::new(http_tracker_config.clone()),
                    ),
                );
            }
        }

        let http_tracker_containers = Arc::new(http_tracker_containers);

        let mut udp_tracker_containers = HashMap::new();

        if let Some(udp_trackers) = &configuration.udp_trackers {
            for udp_tracker_config in udp_trackers {
                udp_tracker_containers.insert(
                    udp_tracker_config.bind_address,
                    Arc::new(UdpTrackerCoreContainer {
                        tracker_core_container: tracker_core_container.clone(),
                        udp_tracker_config: Arc::new(udp_tracker_config.clone()),
                        udp_core_stats_event_sender: udp_core_stats_event_sender.clone(),
                        udp_core_stats_repository: udp_core_stats_repository.clone(),
                        ban_service: udp_ban_service.clone(),
                        connect_service: udp_connect_service.clone(),
                        announce_service: udp_announce_service.clone(),
                        scrape_service: udp_scrape_service.clone(),
                    }),
                );
            }
        }

        let udp_tracker_containers = Arc::new(udp_tracker_containers);

        AppContainer {
            tracker_core_container,
            http_api_config,
            http_tracker_core_services,

            // UDP Tracker Core Services
            udp_core_stats_event_sender,
            udp_core_stats_repository,
            udp_ban_service,
            udp_connect_service,
            udp_announce_service,
            udp_scrape_service,

            // UDP Tracker Server Services
            udp_server_stats_event_sender,
            udp_server_stats_repository,

            // UDP Tracker Server Container
            udp_tracker_server_container,

            // Tracker Instance Containers
            http_tracker_containers,
            udp_tracker_containers,
        }
    }

    #[must_use]
    pub fn udp_tracker_server_container(&self) -> Arc<UdpTrackerServerContainer> {
        self.udp_tracker_server_container.clone()
    }

    /// # Errors
    ///
    /// Return an error if there is no HTTP tracker server instance bound to the
    /// socket address.
    pub fn http_tracker_container(&self, bind_address: SocketAddr) -> Result<Arc<HttpTrackerCoreContainer>, Error> {
        match self.http_tracker_containers.get(&bind_address) {
            Some(http_tracker_container) => Ok(http_tracker_container.clone()),
            None => Err(Error::MissingHttpTrackerCoreContainer { bind_address }),
        }
    }

    /// # Errors
    ///
    /// Return an error if there is no UDP tracker server instance bound to the
    /// socket address.
    pub fn udp_tracker_container(&self, bind_address: SocketAddr) -> Result<Arc<UdpTrackerCoreContainer>, Error> {
        match self.udp_tracker_containers.get(&bind_address) {
            Some(udp_tracker_container) => Ok(udp_tracker_container.clone()),
            None => Err(Error::MissingUdpTrackerCoreContainer { bind_address }),
        }
    }

    #[must_use]
    pub fn tracker_http_api_container(&self, http_api_config: &Arc<HttpApi>) -> Arc<TrackerHttpApiCoreContainer> {
        TrackerHttpApiCoreContainer {
            tracker_core_container: self.tracker_core_container.clone(),
            http_api_config: http_api_config.clone(),
            ban_service: self.udp_ban_service.clone(),
            http_stats_repository: self.http_tracker_core_services.http_stats_repository.clone(),
            udp_core_stats_repository: self.udp_core_stats_repository.clone(),
            udp_server_stats_repository: self.udp_server_stats_repository.clone(),
        }
        .into()
    }
}
