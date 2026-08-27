use std::sync::Arc;

use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_events::bus::SenderStatus;
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

use crate::event::bus::EventBus;
use crate::event::sender::Broadcaster;
use crate::services::announce::AnnounceService;
use crate::services::scrape::ScrapeService;
use crate::statistics::repository::Repository;
use crate::{event, statistics};

pub struct HttpTrackerCoreContainer {
    pub http_tracker_config: Arc<HttpTracker>,

    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // `HttpTrackerCoreServices`
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub announce_service: Arc<AnnounceService>,
    pub scrape_service: Arc<ScrapeService>,
}

impl HttpTrackerCoreContainer {
    /// # Panics
    ///
    /// Panics if the persistence-required tracker-core container cannot be
    /// composed from the configured database or SQLite fallback.
    #[must_use]
    pub async fn initialize(
        core_config: &Arc<Core>,
        http_tracker_config: &Arc<HttpTracker>,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<Self> {
        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let database_compatibility_bridge = core_config.database.clone().unwrap_or_default();
        let tracker_core_container = Arc::new(
            TrackerCoreContainer::initialize_from(
                core_config,
                &swarm_coordination_registry_container,
                Some(&database_compatibility_bridge),
            )
            .await
            .expect("HTTP tracker core initialization requires a configured database or SQLite fallback"),
        );

        Self::initialize_from_tracker_core(&tracker_core_container, http_tracker_config, configuration_instance_id)
    }

    #[must_use]
    pub fn initialize_from_tracker_core(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        http_tracker_config: &Arc<HttpTracker>,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<Self> {
        let http_tracker_core_services = HttpTrackerCoreServices::initialize_from(tracker_core_container);

        Self::initialize_from_services(
            tracker_core_container,
            &http_tracker_core_services,
            http_tracker_config,
            configuration_instance_id,
        )
    }

    #[must_use]
    pub fn initialize_from_services(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        http_tracker_core_services: &Arc<HttpTrackerCoreServices>,
        http_tracker_config: &Arc<HttpTracker>,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<Self> {
        Arc::new(Self {
            tracker_core_container: tracker_core_container.clone(),
            http_tracker_config: http_tracker_config.clone(),
            event_bus: http_tracker_core_services.event_bus.clone(),
            stats_event_sender: http_tracker_core_services.stats_event_sender.clone(),
            stats_repository: http_tracker_core_services.stats_repository.clone(),
            announce_service: Arc::new(AnnounceService::new_with_http_tracker_config(
                tracker_core_container.core_config.clone(),
                tracker_core_container.announce_handler.clone(),
                tracker_core_container.authentication_service.clone(),
                tracker_core_container.whitelist_authorization.clone(),
                http_tracker_core_services.stats_event_sender.clone(),
                http_tracker_config,
                configuration_instance_id,
            )),
            scrape_service: Arc::new(ScrapeService::new_with_http_tracker_config(
                tracker_core_container.core_config.clone(),
                tracker_core_container.scrape_handler.clone(),
                tracker_core_container.authentication_service.clone(),
                http_tracker_core_services.stats_event_sender.clone(),
                http_tracker_config,
                configuration_instance_id,
            )),
        })
    }
}

pub struct HttpTrackerCoreServices {
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
}

impl HttpTrackerCoreServices {
    #[must_use]
    pub fn initialize_from(_tracker_core_container: &Arc<TrackerCoreContainer>) -> Arc<Self> {
        // HTTP core stats
        let http_core_broadcaster = Broadcaster::default();
        let http_stats_repository = Arc::new(Repository::new());
        // issue: #2039
        // issue-spec: docs/issues/drafts/optimize-event-publication-without-consumers/ISSUE.md
        // Events are objective facts. Per-listener metrics policy is applied by
        // the shared statistics listener, so it must not suppress publication.
        // A future consumer-demand optimization needs an inventory and benchmark
        // evidence before this can become conditional.
        let http_stats_event_bus = Arc::new(EventBus::new(SenderStatus::Enabled, http_core_broadcaster.clone()));

        let http_stats_event_sender = http_stats_event_bus.sender();

        Arc::new(Self {
            event_bus: http_stats_event_bus,
            stats_event_sender: http_stats_event_sender,
            stats_repository: http_stats_repository,
        })
    }
}
