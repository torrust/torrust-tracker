//! Job that runs a task on intervals to update peers' activity metrics.
use std::future::Future;
use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_events::shutdown::Completion;

use crate::container::AppContainer;

pub fn run_job(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> impl Future<Output = Completion> + Send + 'static {
    torrust_tracker_swarm_coordination_registry::statistics::activity_metrics_updater::run_job(
        app_container.swarm_coordination_registry_container.swarms.clone(),
        app_container.swarm_coordination_registry_container.stats_repository.clone(),
        config.core.tracker_policy.max_peer_timeout,
        cancellation_token,
    )
}
