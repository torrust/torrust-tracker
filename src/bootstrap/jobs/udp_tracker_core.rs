use std::collections::BTreeMap;
use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::Configuration;

use crate::container::AppContainer;

#[must_use]
// issue: #2039
// The policy is immutable for this application lifetime and filters a shared
// aggregate repository; producers remain independent of this metrics decision.
pub fn start_event_listener(
    _config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<JoinHandle<()>> {
    let metrics_policy = app_container
        .udp_tracker_instance_containers
        .iter()
        .map(|(id, container)| (*id, container.udp_tracker_config.tracker_usage_statistics))
        .collect::<BTreeMap<_, _>>();
    let job = torrust_tracker_udp_core::statistics::event::listener::run_event_listener(
        app_container.udp_tracker_core_services.event_bus.receiver(),
        cancellation_token,
        &app_container.udp_tracker_core_services.stats_repository,
        metrics_policy,
    );
    Some(job)
}
