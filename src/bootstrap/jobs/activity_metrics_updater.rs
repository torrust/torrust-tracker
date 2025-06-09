//! Job that runs a task on intervals to update peers' activity metrics.
use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_configuration::Configuration;

use crate::container::AppContainer;
use crate::CurrentClock;

#[must_use]
pub fn start_job(config: &Configuration, app_container: &Arc<AppContainer>) -> JoinHandle<()> {
    torrust_tracker_swarm_coordination_registry::statistics::activity_metrics_updater::start_job(
        &app_container.swarm_coordination_registry_container.swarms.clone(),
        &app_container.swarm_coordination_registry_container.stats_repository.clone(),
        peer_inactivity_cutoff_timestamp(config.core.tracker_policy.max_peer_timeout),
    )
}

/// Returns the timestamp of the cutoff for inactive peers.
///
/// Peers that has not been updated for more than `max_peer_timeout` seconds are
/// considered inactive.
fn peer_inactivity_cutoff_timestamp(max_peer_timeout: u32) -> Duration {
    CurrentClock::now_sub(&Duration::from_secs(u64::from(max_peer_timeout))).unwrap_or_default()
}
