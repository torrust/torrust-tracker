use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use bittorrent_http_tracker_core::container::{HttpTrackerCoreContainer, HttpTrackerCoreServices};
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_udp_tracker_core::container::{UdpTrackerCoreContainer, UdpTrackerCoreServices};
use bittorrent_udp_tracker_core::{self};
use torrust_rest_tracker_api_core::container::TrackerHttpApiCoreContainer;
use torrust_server_lib::registar::Registar;
use torrust_tracker_configuration::{Configuration, HttpApi};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
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
    // Configuration
    pub http_api_config: Arc<Option<HttpApi>>,

    // Registar
    pub registar: Arc<Registar>,

    // Swarm Coordination Registry Container
    pub swarm_coordination_registry_container: Arc<SwarmCoordinationRegistryContainer>,

    // Core
    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // HTTP
    pub http_tracker_core_services: Arc<HttpTrackerCoreServices>,
    pub http_tracker_instance_containers: Arc<HashMap<SocketAddr, Arc<HttpTrackerCoreContainer>>>,

    // UDP
    pub udp_tracker_core_services: Arc<UdpTrackerCoreServices>,
    pub udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
    pub udp_tracker_instance_containers: Arc<HashMap<SocketAddr, Arc<UdpTrackerCoreContainer>>>,
}

impl AppContainer {
    #[instrument(skip(configuration))]
    pub fn initialize(configuration: &Configuration) -> AppContainer {
        // Configuration

        let core_config = Arc::new(configuration.core.clone());

        let http_api_config = Arc::new(configuration.http_api.clone());

        // Registar

        let registar = Arc::new(Registar::default());

        // Swarm Coordination Registry Container

        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        // Core

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize_from(
            &core_config,
            &swarm_coordination_registry_container,
        ));

        // HTTP

        let http_tracker_core_services = HttpTrackerCoreServices::initialize_from(&tracker_core_container);

        let http_tracker_instance_containers = Self::initialize_http_tracker_instance_containers(
            configuration,
            &tracker_core_container,
            &http_tracker_core_services,
        );

        // UDP

        let udp_tracker_core_services = UdpTrackerCoreServices::initialize_from(&tracker_core_container);

        let udp_tracker_server_container = UdpTrackerServerContainer::initialize(&core_config);

        let udp_tracker_instance_containers =
            Self::initialize_udp_tracker_instance_containers(configuration, &tracker_core_container, &udp_tracker_core_services);

        AppContainer {
            // Configuration
            http_api_config,

            // Registar
            registar,

            // Swarm Coordination Registry Container
            swarm_coordination_registry_container,

            // Core
            tracker_core_container,

            // HTTP
            http_tracker_core_services,
            http_tracker_instance_containers,

            // UDP
            udp_tracker_core_services,
            udp_tracker_server_container,
            udp_tracker_instance_containers,
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
        match self.http_tracker_instance_containers.get(&bind_address) {
            Some(http_tracker_container) => Ok(http_tracker_container.clone()),
            None => Err(Error::MissingHttpTrackerCoreContainer { bind_address }),
        }
    }

    /// # Errors
    ///
    /// Return an error if there is no UDP tracker server instance bound to the
    /// socket address.
    pub fn udp_tracker_container(&self, bind_address: SocketAddr) -> Result<Arc<UdpTrackerCoreContainer>, Error> {
        match self.udp_tracker_instance_containers.get(&bind_address) {
            Some(udp_tracker_container) => Ok(udp_tracker_container.clone()),
            None => Err(Error::MissingUdpTrackerCoreContainer { bind_address }),
        }
    }

    #[must_use]
    pub fn tracker_http_api_container(&self, http_api_config: &Arc<HttpApi>) -> Arc<TrackerHttpApiCoreContainer> {
        TrackerHttpApiCoreContainer {
            http_api_config: http_api_config.clone(),

            swarm_coordination_registry_container: self.swarm_coordination_registry_container.clone(),

            tracker_core_container: self.tracker_core_container.clone(),

            http_stats_repository: self.http_tracker_core_services.stats_repository.clone(),

            ban_service: self.udp_tracker_core_services.ban_service.clone(),
            udp_core_stats_repository: self.udp_tracker_core_services.stats_repository.clone(),
            udp_server_stats_repository: self.udp_tracker_server_container.stats_repository.clone(),
        }
        .into()
    }

    #[must_use]
    fn initialize_http_tracker_instance_containers(
        configuration: &Configuration,
        tracker_core_container: &Arc<TrackerCoreContainer>,
        http_tracker_core_services: &Arc<HttpTrackerCoreServices>,
    ) -> Arc<HashMap<SocketAddr, Arc<HttpTrackerCoreContainer>>> {
        let mut http_tracker_instance_containers = HashMap::new();

        if let Some(http_trackers) = &configuration.http_trackers {
            for http_tracker_config in http_trackers {
                http_tracker_instance_containers.insert(
                    http_tracker_config.bind_address,
                    HttpTrackerCoreContainer::initialize_from_services(
                        tracker_core_container,
                        http_tracker_core_services,
                        &Arc::new(http_tracker_config.clone()),
                    ),
                );
            }
        }

        Arc::new(http_tracker_instance_containers)
    }

    #[must_use]
    fn initialize_udp_tracker_instance_containers(
        configuration: &Configuration,
        tracker_core_container: &Arc<TrackerCoreContainer>,
        udp_tracker_core_services: &Arc<UdpTrackerCoreServices>,
    ) -> Arc<HashMap<SocketAddr, Arc<UdpTrackerCoreContainer>>> {
        let mut udp_tracker_instance_containers = HashMap::new();

        if let Some(udp_trackers) = &configuration.udp_trackers {
            for udp_tracker_config in udp_trackers {
                udp_tracker_instance_containers.insert(
                    udp_tracker_config.bind_address,
                    UdpTrackerCoreContainer::initialize_from_services(
                        tracker_core_container,
                        udp_tracker_core_services,
                        &Arc::new(udp_tracker_config.clone()),
                    ),
                );
            }
        }

        Arc::new(udp_tracker_instance_containers)
    }
}
