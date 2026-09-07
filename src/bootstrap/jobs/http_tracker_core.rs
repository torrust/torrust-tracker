use std::collections::BTreeMap;
use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_events::shutdown::Completion;

use crate::container::AppContainer;

/// Returns an unspawned listener for application-level supervision.
// issue: #2039
// The policy is immutable for this application lifetime and filters a shared
// aggregate repository; producers remain independent of this metrics decision.
pub fn run_event_listener(
    _config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> impl Future<Output = Completion> + Send + 'static {
    let metrics_policy = app_container
        .http_tracker_instance_containers
        .iter()
        .map(|(id, container)| (*id, container.http_tracker_config.tracker_usage_statistics))
        .collect::<BTreeMap<_, _>>();
    torrust_tracker_http_core::statistics::event::listener::run_event_listener_unspawned(
        app_container.http_tracker_core_services.event_bus.receiver(),
        cancellation_token,
        app_container.http_tracker_core_services.stats_repository.clone(),
        metrics_policy,
    )
}
