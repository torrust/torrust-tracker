use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_events::shutdown::Completion;

use crate::container::AppContainer;

/// Errors encountered while starting tracker-core background jobs.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Persistent completed statistics require a persistence container")]
    PersistentStatisticsRequirePersistence,
}

/// Returns an unspawned in-memory listener for application-level supervision.
#[must_use]
pub fn run_in_memory_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<impl Future<Output = Completion> + Send + 'static> {
    config.core.tracker_usage_statistics.then(|| {
        torrust_tracker_core::statistics::event::listener::run_in_memory_event_listener_unspawned(
            app_container.swarm_coordination_registry_container.event_bus.receiver(),
            cancellation_token,
            app_container.tracker_core_container.stats_repository.clone(),
        )
    })
}

/// Returns an unspawned persistent-statistics listener for application-level supervision.
///
/// # Errors
///
/// Returns an error if persistent completed statistics are enabled but
/// persistence was not composed.
pub fn run_persistent_completed_statistics_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Result<Option<impl Future<Output = Completion> + Send + 'static>, Error> {
    if !config.core.tracker_policy.persistent_torrent_completed_stat {
        return Ok(None);
    }

    let persistence = app_container
        .tracker_core_container
        .persistence
        .as_ref()
        .ok_or(Error::PersistentStatisticsRequirePersistence)?;
    Ok(Some(
        torrust_tracker_core::statistics::event::listener::run_persistent_completed_statistics_event_listener_unspawned(
            app_container.swarm_coordination_registry_container.event_bus.receiver(),
            cancellation_token,
            persistence.db_downloads_metric_repository.clone(),
            app_container.tracker_core_container.stats_repository.clone(),
        ),
    ))
}
