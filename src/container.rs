use std::sync::Arc;

use torrust_server_lib::registar::Registar;
use torrust_tracker_configuration::{Configuration, HttpApi};
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_http_core::container::{HttpTrackerCoreContainer, HttpTrackerCoreServices};
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
use torrust_tracker_udp_core::container::{UdpTrackerCoreContainer, UdpTrackerCoreServices};
use torrust_tracker_udp_core::{self};
use torrust_tracker_udp_server::container::UdpTrackerServerContainer;
use tracing::instrument;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("No HTTP tracker container at configuration index {index}")]
    MissingHttpTrackerCoreContainer { index: usize },

    #[error("No UDP tracker container at configuration index {index}")]
    MissingUdpTrackerCoreContainer { index: usize },
}

pub struct AppContainer {
    // Configuration
    pub http_api_config: Arc<Option<HttpApi>>,

    // Registar
    pub registar: Arc<Registar<torrust_tracker_primitives::RuntimeServiceMetadata>>,

    // Swarm Coordination Registry Container
    pub swarm_coordination_registry_container: Arc<SwarmCoordinationRegistryContainer>,

    // Core
    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // HTTP
    pub http_tracker_core_services: Arc<HttpTrackerCoreServices>,
    pub http_tracker_instance_containers: Vec<(ConfigurationInstanceId, Arc<HttpTrackerCoreContainer>)>,

    // UDP
    pub udp_tracker_core_services: Arc<UdpTrackerCoreServices>,
    pub udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
    pub udp_tracker_instance_containers: Vec<(ConfigurationInstanceId, Arc<UdpTrackerCoreContainer>)>,
}

impl AppContainer {
    #[instrument(skip(configuration))]
    pub async fn initialize(configuration: &Configuration) -> Self {
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

        let tracker_core_container =
            Arc::new(TrackerCoreContainer::initialize_from(&core_config, &swarm_coordination_registry_container).await);

        // HTTP

        let http_tracker_core_services = HttpTrackerCoreServices::initialize_from(&tracker_core_container);

        let http_tracker_instance_containers = Self::initialize_http_tracker_instance_containers(
            configuration,
            &tracker_core_container,
            &http_tracker_core_services,
        );

        // UDP

        use torrust_tracker_configuration::UdpTracker as UdpTrackerConfig;

        let default_max_connection_id_errors = UdpTrackerConfig::default().max_connection_id_errors_per_ip;

        let max_connection_id_errors = configuration
            .udp_trackers
            .as_ref()
            .and_then(|trackers| trackers.first())
            .map_or(default_max_connection_id_errors, |config| {
                config.max_connection_id_errors_per_ip
            });

        let udp_tracker_core_services =
            UdpTrackerCoreServices::initialize_from(&tracker_core_container, max_connection_id_errors);

        let udp_tracker_server_container = UdpTrackerServerContainer::initialize(&core_config);

        let udp_tracker_instance_containers =
            Self::initialize_udp_tracker_instance_containers(configuration, &tracker_core_container, &udp_tracker_core_services);

        Self {
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
    /// Return an error if there is no HTTP tracker container at the given
    /// configuration index.
    pub fn http_tracker_container(
        &self,
        index: usize,
    ) -> Result<(ConfigurationInstanceId, Arc<HttpTrackerCoreContainer>), Error> {
        self.http_tracker_instance_containers.get(index).map_or_else(
            || Err(Error::MissingHttpTrackerCoreContainer { index }),
            |(id, container)| Ok((*id, container.clone())),
        )
    }

    /// # Errors
    ///
    /// Return an error if there is no UDP tracker container at the given
    /// configuration index.
    pub fn udp_tracker_container(&self, index: usize) -> Result<(ConfigurationInstanceId, Arc<UdpTrackerCoreContainer>), Error> {
        self.udp_tracker_instance_containers.get(index).map_or_else(
            || Err(Error::MissingUdpTrackerCoreContainer { index }),
            |(id, container)| Ok((*id, container.clone())),
        )
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
    ) -> Vec<(ConfigurationInstanceId, Arc<HttpTrackerCoreContainer>)> {
        use torrust_tracker_primitives::ServiceRole;

        let mut containers = Vec::new();

        if let Some(http_trackers) = &configuration.http_trackers {
            for (index, http_tracker_config) in http_trackers.iter().enumerate() {
                let id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, index);
                let container = HttpTrackerCoreContainer::initialize_from_services(
                    tracker_core_container,
                    http_tracker_core_services,
                    &Arc::new(http_tracker_config.clone()),
                    id,
                );
                containers.push((id, container));
            }
        }

        containers
    }

    #[must_use]
    fn initialize_udp_tracker_instance_containers(
        configuration: &Configuration,
        tracker_core_container: &Arc<TrackerCoreContainer>,
        udp_tracker_core_services: &Arc<UdpTrackerCoreServices>,
    ) -> Vec<(ConfigurationInstanceId, Arc<UdpTrackerCoreContainer>)> {
        use torrust_tracker_primitives::ServiceRole;

        let mut containers = Vec::new();

        if let Some(udp_trackers) = &configuration.udp_trackers {
            for (index, udp_tracker_config) in udp_trackers.iter().enumerate() {
                let id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, index);
                let container = UdpTrackerCoreContainer::initialize_from_services(
                    tracker_core_container,
                    udp_tracker_core_services,
                    &Arc::new(udp_tracker_config.clone()),
                    id,
                );
                containers.push((id, container));
            }
        }

        containers
    }
}
