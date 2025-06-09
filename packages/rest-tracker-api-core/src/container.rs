use std::sync::Arc;

use bittorrent_http_tracker_core::container::HttpTrackerCoreContainer;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use bittorrent_udp_tracker_core::services::banning::BanService;
use bittorrent_udp_tracker_core::{self};
use tokio::sync::RwLock;
use torrust_tracker_configuration::{Core, HttpApi, HttpTracker, UdpTracker};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
use torrust_udp_tracker_server::container::UdpTrackerServerContainer;

pub struct TrackerHttpApiCoreContainer {
    pub http_api_config: Arc<HttpApi>,

    // Swarm Coordination Registry Container
    pub swarm_coordination_registry_container: Arc<SwarmCoordinationRegistryContainer>,

    // Tracker core
    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // HTTP tracker core
    pub http_stats_repository: Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,

    // UDP tracker core
    pub ban_service: Arc<RwLock<BanService>>,
    pub udp_core_stats_repository: Arc<bittorrent_udp_tracker_core::statistics::repository::Repository>,
    pub udp_server_stats_repository: Arc<torrust_udp_tracker_server::statistics::repository::Repository>,
}

impl TrackerHttpApiCoreContainer {
    #[must_use]
    pub fn initialize(
        core_config: &Arc<Core>,
        http_tracker_config: &Arc<HttpTracker>,
        udp_tracker_config: &Arc<UdpTracker>,
        http_api_config: &Arc<HttpApi>,
    ) -> Arc<TrackerHttpApiCoreContainer> {
        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize_from(
            core_config,
            &swarm_coordination_registry_container,
        ));

        let http_tracker_core_container =
            HttpTrackerCoreContainer::initialize_from_tracker_core(&tracker_core_container, http_tracker_config);

        let udp_tracker_core_container =
            UdpTrackerCoreContainer::initialize_from_tracker_core(&tracker_core_container, udp_tracker_config);

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
    ) -> Arc<TrackerHttpApiCoreContainer> {
        Arc::new(TrackerHttpApiCoreContainer {
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
