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
    if config.core.tracker_usage_statistics {
        let job = torrust_tracker_swarm_coordination_registry::statistics::event::listener::run_event_listener(
            app_container.swarm_coordination_registry_container.event_bus.receiver(),
            cancellation_token,
            &app_container.swarm_coordination_registry_container.stats_repository,
        );

        Some(job)
    } else {
        tracing::info!("Torrent repository package event listener job is disabled.");
        None
    }
}
