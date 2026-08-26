use std::sync::Arc;

use tokio::sync::RwLock;
use torrust_tracker_configuration::{Core, UdpTracker};
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_events::bus::SenderStatus;
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

use crate::event::bus::EventBus;
use crate::event::sender::Broadcaster;
use crate::services::announce::AnnounceService;
use crate::services::banning::BanService;
use crate::services::connect::ConnectService;
use crate::services::scrape::ScrapeService;
use crate::statistics::repository::Repository;
use crate::{event, services, statistics};

pub struct UdpTrackerCoreContainer {
    pub udp_tracker_config: Arc<UdpTracker>,
    pub configuration_instance_id: ConfigurationInstanceId,

    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // `UdpTrackerCoreServices`
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: crate::event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub ban_service: Arc<RwLock<BanService>>,
    pub connect_service: Arc<ConnectService>,
    pub announce_service: Arc<AnnounceService>,
    pub scrape_service: Arc<ScrapeService>,
}

impl UdpTrackerCoreContainer {
    /// # Panics
    ///
    /// Panics if the persistence-required tracker-core container cannot be
    /// composed from the active v2-compatible configuration.
    #[must_use]
    pub async fn initialize(
        core_config: &Arc<Core>,
        udp_tracker_config: &Arc<UdpTracker>,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<UdpTrackerCoreContainer> {
        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(
            TrackerCoreContainer::initialize_from(
                core_config,
                &swarm_coordination_registry_container,
                Some(&core_config.database),
            )
            .await
            .expect("UDP tracker core initialization requires persistence"),
        );

        Self::initialize_from_tracker_core(&tracker_core_container, udp_tracker_config, configuration_instance_id)
    }

    #[must_use]
    pub fn initialize_from_tracker_core(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        udp_tracker_config: &Arc<UdpTracker>,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<UdpTrackerCoreContainer> {
        let max_connection_id_errors_per_ip = udp_tracker_config.max_connection_id_errors_per_ip;
        let udp_tracker_core_services =
            UdpTrackerCoreServices::initialize_from(tracker_core_container, max_connection_id_errors_per_ip);

        Self::initialize_from_services(
            tracker_core_container,
            &udp_tracker_core_services,
            udp_tracker_config,
            configuration_instance_id,
        )
    }

    #[must_use]
    pub fn initialize_from_services(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        udp_tracker_core_services: &Arc<UdpTrackerCoreServices>,
        udp_tracker_config: &Arc<UdpTracker>,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<Self> {
        Arc::new(Self {
            udp_tracker_config: udp_tracker_config.clone(),
            configuration_instance_id,

            tracker_core_container: tracker_core_container.clone(),

            // `UdpTrackerCoreServices`
            event_bus: udp_tracker_core_services.event_bus.clone(),
            stats_event_sender: udp_tracker_core_services.stats_event_sender.clone(),
            stats_repository: udp_tracker_core_services.stats_repository.clone(),
            ban_service: udp_tracker_core_services.ban_service.clone(),
            connect_service: Arc::new(ConnectService::new(
                udp_tracker_core_services.stats_event_sender.clone(),
                configuration_instance_id,
            )),
            announce_service: Arc::new(AnnounceService::new(
                tracker_core_container.announce_handler.clone(),
                tracker_core_container.whitelist_authorization.clone(),
                udp_tracker_core_services.stats_event_sender.clone(),
                configuration_instance_id,
            )),
            scrape_service: Arc::new(ScrapeService::new(
                tracker_core_container.scrape_handler.clone(),
                udp_tracker_core_services.stats_event_sender.clone(),
                configuration_instance_id,
            )),
        })
    }
}

pub struct UdpTrackerCoreServices {
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: crate::event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub ban_service: Arc<RwLock<services::banning::BanService>>,
}

impl UdpTrackerCoreServices {
    #[must_use]
    pub fn initialize_from(
        _tracker_core_container: &Arc<TrackerCoreContainer>,
        max_connection_id_errors_per_ip: u32,
    ) -> Arc<Self> {
        let udp_core_broadcaster = Broadcaster::default();
        let udp_core_stats_repository = Arc::new(Repository::new());
        // issue: #2039
        // issue-spec: docs/issues/drafts/optimize-event-publication-without-consumers/ISSUE.md
        // Events are objective facts. Per-listener metrics policy is applied by
        // the shared statistics listener, so it must not suppress publication.
        // A future consumer-demand optimization needs an inventory and benchmark
        // evidence before this can become conditional.
        let event_bus = Arc::new(EventBus::new(SenderStatus::Enabled, udp_core_broadcaster.clone()));

        let udp_core_stats_event_sender = event_bus.sender();
        let ban_service = Arc::new(RwLock::new(BanService::new(max_connection_id_errors_per_ip)));
        Arc::new(Self {
            event_bus,
            stats_event_sender: udp_core_stats_event_sender,
            stats_repository: udp_core_stats_repository,
            ban_service,
        })
    }
}
