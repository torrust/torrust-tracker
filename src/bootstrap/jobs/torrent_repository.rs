use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_events::shutdown::Completion;

use crate::container::AppContainer;

/// Returns an unspawned listener for application-level supervision.
#[must_use]
pub fn run_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<impl Future<Output = Completion> + Send + 'static> {
    config.core.tracker_usage_statistics.then(|| {
        torrust_tracker_swarm_coordination_registry::statistics::event::listener::run_event_listener_unspawned(
            app_container.swarm_coordination_registry_container.event_bus.receiver(),
            cancellation_token,
            app_container.swarm_coordination_registry_container.stats_repository.clone(),
        )
    })
}
