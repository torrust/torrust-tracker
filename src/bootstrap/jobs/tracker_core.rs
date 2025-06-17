use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::Configuration;

use crate::container::AppContainer;

pub fn start_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<JoinHandle<()>> {
    if config.core.tracker_usage_statistics || config.core.tracker_policy.persistent_torrent_completed_stat {
        let job = bittorrent_tracker_core::statistics::event::listener::run_event_listener(
            app_container.swarm_coordination_registry_container.event_bus.receiver(),
            cancellation_token,
            &app_container.tracker_core_container.stats_repository,
            &app_container.tracker_core_container.db_downloads_metric_repository,
            app_container
                .tracker_core_container
                .core_config
                .tracker_policy
                .persistent_torrent_completed_stat,
        );

        Some(job)
    } else {
        tracing::info!("Tracker core event listener job is disabled.");
        None
    }
}
