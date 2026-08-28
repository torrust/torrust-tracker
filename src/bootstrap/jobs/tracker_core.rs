use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::v3_0_0::Configuration;

use crate::container::AppContainer;

pub fn start_in_memory_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<JoinHandle<()>> {
    if config.core.tracker_usage_statistics {
        let job = torrust_tracker_core::statistics::event::listener::run_in_memory_event_listener(
            app_container.swarm_coordination_registry_container.event_bus.receiver(),
            cancellation_token,
            &app_container.tracker_core_container.stats_repository,
        );

        Some(job)
    } else {
        tracing::info!("Tracker core event listener job is disabled.");
        None
    }
}

/// # Panics
///
/// Panics if persistent completed statistics are enabled but persistence was
/// not composed. Bootstrap configuration validation prevents this state.
pub fn start_persistent_completed_statistics_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<JoinHandle<()>> {
    if config.core.tracker_policy.persistent_torrent_completed_stat {
        let persistence = app_container
            .tracker_core_container
            .persistence
            .as_ref()
            .expect("persistent completed statistics require persistence");
        let job = torrust_tracker_core::statistics::event::listener::run_persistent_completed_statistics_event_listener(
            app_container.swarm_coordination_registry_container.event_bus.receiver(),
            cancellation_token,
            &persistence.db_downloads_metric_repository,
        );

        Some(job)
    } else {
        tracing::info!("Tracker core persistent completed statistics event listener job is disabled.");
        None
    }
}
