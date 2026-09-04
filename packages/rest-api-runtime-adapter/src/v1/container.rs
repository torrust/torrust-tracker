//! Dependency injection container for the REST API server.
//!
//! Wires all tracker internal components (swarm registry, HTTP/UDP cores, etc.)
//! into a single container that the Axum server uses to construct adapters.
//!
//! This was previously in `rest-api-core` and was moved here as part of SI-5
//! (deprecation of `rest-api-core`).
use std::sync::Arc;

use tokio::sync::RwLock;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_configuration::v3_0_0::tracker_api::HttpApi;
use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;
use torrust_tracker_configuration::v3_0_0::udp_tracker_server::UdpTrackerServer;
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_core::services::banning::BanService;
use torrust_tracker_udp_core::{self};
use torrust_tracker_udp_server::container::UdpTrackerServerContainer;

/// Container that holds all the internal tracker components needed by the
/// REST API server.
pub struct TrackerHttpApiCoreContainer {
    pub http_api_config: Arc<HttpApi>,

    // Swarm Coordination Registry Container
    pub swarm_coordination_registry_container: Arc<SwarmCoordinationRegistryContainer>,

    // Tracker core
    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // HTTP tracker core
    pub http_stats_repository: Arc<torrust_tracker_http_core::statistics::repository::Repository>,

    // UDP tracker core
    pub ban_service: Arc<RwLock<BanService>>,
    pub udp_core_stats_repository: Arc<torrust_tracker_udp_core::statistics::repository::Repository>,
    pub udp_server_stats_repository: Arc<torrust_tracker_udp_server::statistics::repository::Repository>,
}

impl TrackerHttpApiCoreContainer {
    /// # Panics
    ///
    /// Panics if the persistence-required tracker-core container cannot be
    /// composed from the configured database.
    #[must_use]
    pub async fn initialize(
        core_config: &Arc<Core>,
        http_tracker_config: &Arc<HttpTracker>,
        http_tracker_configuration_instance_id: ConfigurationInstanceId,
        udp_tracker_config: &Arc<UdpTracker>,
        udp_tracker_server_config: &UdpTrackerServer,
        udp_tracker_configuration_instance_id: ConfigurationInstanceId,
        http_api_config: &Arc<HttpApi>,
    ) -> Arc<Self> {
        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(
            TrackerCoreContainer::initialize_from(
                core_config,
                &swarm_coordination_registry_container,
                core_config.database.as_ref(),
            )
            .await
            .expect("REST API initialization requires persistence"),
        );

        let http_tracker_core_container = HttpTrackerCoreContainer::initialize_from_tracker_core(
            &tracker_core_container,
            http_tracker_config,
            http_tracker_configuration_instance_id,
        );

        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize_from_tracker_core(
            &tracker_core_container,
            udp_tracker_config,
            udp_tracker_server_config.max_connection_id_errors_per_ip,
            udp_tracker_configuration_instance_id,
        );

        let udp_tracker_server_container = UdpTrackerServerContainer::initialize(core_config);

        Self::initialize_from(
            &swarm_coordination_registry_container,
            &tracker_core_container,
            &http_tracker_core_container,
            &udp_tracker_core_container,
            &udp_tracker_server_container,
            http_api_config,
        )
    }

    #[must_use]
    pub fn initialize_from(
        swarm_coordination_registry_container: &Arc<SwarmCoordinationRegistryContainer>,
        tracker_core_container: &Arc<TrackerCoreContainer>,
        http_tracker_core_container: &Arc<HttpTrackerCoreContainer>,
        udp_tracker_core_container: &Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: &Arc<UdpTrackerServerContainer>,
        http_api_config: &Arc<HttpApi>,
    ) -> Arc<Self> {
        Arc::new(Self {
            http_api_config: http_api_config.clone(),

            // Swarm Coordination Registry Container
            swarm_coordination_registry_container: swarm_coordination_registry_container.clone(),

            // Tracker core
            tracker_core_container: tracker_core_container.clone(),

            // HTTP tracker core
            http_stats_repository: http_tracker_core_container.stats_repository.clone(),

            // UDP tracker core
            ban_service: udp_tracker_core_container.ban_service.clone(),
            udp_core_stats_repository: udp_tracker_core_container.stats_repository.clone(),
            udp_server_stats_repository: udp_tracker_server_container.stats_repository.clone(),
        })
    }
}
