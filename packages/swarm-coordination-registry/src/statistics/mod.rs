pub mod activity_metrics_updater;
pub mod event;
pub mod metrics;
pub mod repository;

use metrics::Metrics;
use torrust_metrics::metric::description::MetricDescription;
use torrust_metrics::metric_name;
use torrust_metrics::unit::Unit;

// Torrent metrics

const SWARM_COORDINATION_REGISTRY_TORRENTS_ADDED_TOTAL: &str = "swarm_coordination_registry_torrents_added_total";
const SWARM_COORDINATION_REGISTRY_TORRENTS_REMOVED_TOTAL: &str = "swarm_coordination_registry_torrents_removed_total";

const SWARM_COORDINATION_REGISTRY_TORRENTS_TOTAL: &str = "swarm_coordination_registry_torrents_total";
const SWARM_COORDINATION_REGISTRY_TORRENTS_DOWNLOADS_TOTAL: &str = "swarm_coordination_registry_torrents_downloads_total";
const SWARM_COORDINATION_REGISTRY_TORRENTS_INACTIVE_TOTAL: &str = "swarm_coordination_registry_torrents_inactive_total";

// Peers metrics

const SWARM_COORDINATION_REGISTRY_PEERS_ADDED_TOTAL: &str = "swarm_coordination_registry_peers_added_total";
const SWARM_COORDINATION_REGISTRY_PEERS_REMOVED_TOTAL: &str = "swarm_coordination_registry_peers_removed_total";
const SWARM_COORDINATION_REGISTRY_PEERS_UPDATED_TOTAL: &str = "swarm_coordination_registry_peers_updated_total";

const SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL: &str = "swarm_coordination_registry_peer_connections_total";
const SWARM_COORDINATION_REGISTRY_UNIQUE_PEERS_TOTAL: &str = "swarm_coordination_registry_unique_peers_total"; // todo: not implemented yet
const SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL: &str = "swarm_coordination_registry_peers_inactive_total";
const SWARM_COORDINATION_REGISTRY_PEERS_COMPLETED_STATE_REVERTED_TOTAL: &str =
    "swarm_coordination_registry_peers_completed_state_reverted_total";

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    // Torrent metrics

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_ADDED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of torrents added.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_REMOVED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of torrents removed.")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of torrents.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_DOWNLOADS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of torrent downloads.")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_INACTIVE_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of inactive torrents.")),
    );

    // Peers metrics

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_ADDED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of peers added.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_REMOVED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of peers removed.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_UPDATED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of peers updated.")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new(
            "The total number of peer connections (one connection per torrent).",
        )),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(SWARM_COORDINATION_REGISTRY_UNIQUE_PEERS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of unique peers.")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of inactive peers.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_COMPLETED_STATE_REVERTED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new(
            "The total number of peers whose completed state was reverted.",
        )),
    );

    metrics
}

#[cfg(test)]
mod tests {
    //! Collaboration tests: the activity metrics job running against a real
    //! `Registry` and `Repository`. They pin the observable outcome of the
    //! job (the published gauges), not the internals of any single module.
    //
    // Missing tests, tracked by the package coverage review (#1347):
    // - a peer that re-announces after going inactive lowers the gauge again
    // - the torrents gauge follows the same before/after boundary

    use std::sync::Arc;
    use std::time::Duration;

    use tokio_util::sync::{CancellationToken, DropGuard};
    use torrust_clock::clock::stopped::Stopped as _;
    use torrust_clock::{DurationSinceUnixEpoch, clock};
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;

    use super::activity_metrics_updater::{ACTIVITY_METRICS_UPDATE_INTERVAL_SECS, run_job};
    use super::repository::Repository;
    use crate::Registry;
    use crate::statistics::SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL;
    use crate::tests::{sample_info_hash, sample_peer};

    /// A running activity metrics job whose registry holds one peer announced
    /// at job startup. Owns the runner and its collaborators so tests only
    /// state the peer timeout, the elapsed domain time, and the expected gauge.
    ///
    /// Two clocks are involved: Tokio's paused time drives the update interval,
    /// while `clock::Stopped` drives the domain timestamps the cutoff compares.
    struct JobWithOnePeerAnnouncedAtStartup {
        startup_time: DurationSinceUnixEpoch,
        _swarms: Arc<Registry>,
        stats_repository: Arc<Repository>,
        _cancel_on_drop: DropGuard,
    }

    impl JobWithOnePeerAnnouncedAtStartup {
        async fn start(max_peer_timeout_in_secs: u32) -> Self {
            let startup_time = DurationSinceUnixEpoch::new(1_000, 0);
            clock::Stopped::local_set(&startup_time);

            let swarms = Arc::new(Registry::new(None));
            let stats_repository = Arc::new(Repository::new());
            let cancellation_token = CancellationToken::new();

            tokio::spawn(run_job(
                swarms.clone(),
                stats_repository.clone(),
                max_peer_timeout_in_secs,
                cancellation_token.clone(),
            ));
            tokio::task::yield_now().await;

            let mut peer = sample_peer();
            peer.updated = startup_time;
            swarms.handle_announcement(&sample_info_hash(), &peer, None).await.unwrap();

            Self {
                startup_time,
                _swarms: swarms,
                stats_repository,
                _cancel_on_drop: cancellation_token.drop_guard(),
            }
        }

        fn set_domain_time_elapsed_since_startup(&self, elapsed: Duration) {
            clock::Stopped::local_set(&(self.startup_time + elapsed));
        }

        async fn run_next_update(&self) {
            tokio::time::advance(Duration::from_secs(ACTIVITY_METRICS_UPDATE_INTERVAL_SECS)).await;
            tokio::task::yield_now().await;
        }

        async fn inactive_peers_total(&self) -> usize {
            let gauge_value = self
                .stats_repository
                .get_metrics()
                .await
                .metric_collection
                .get_gauge_value(
                    &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL),
                    &LabelSet::default(),
                )
                .expect("the update tick should set the inactive peers gauge")
                .value();

            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "the gauge is set from a small non-negative peer count"
            )]
            let inactive_peers_total = gauge_value as usize;

            inactive_peers_total
        }
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_keep_a_peer_announced_after_startup_active_before_the_timeout_elapses() {
        // Arrange
        let max_peer_timeout_in_secs = 2;
        let job = JobWithOnePeerAnnouncedAtStartup::start(max_peer_timeout_in_secs).await;

        // Act
        job.set_domain_time_elapsed_since_startup(Duration::from_secs(1));
        job.run_next_update().await;

        // Assert
        assert_eq!(job.inactive_peers_total().await, 0);
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_count_a_peer_announced_after_startup_inactive_after_the_timeout_elapses() {
        // Arrange
        let max_peer_timeout_in_secs = 2;
        let job = JobWithOnePeerAnnouncedAtStartup::start(max_peer_timeout_in_secs).await;

        // Act
        job.set_domain_time_elapsed_since_startup(Duration::from_secs(3));
        job.run_next_update().await;

        // Assert
        assert_eq!(job.inactive_peers_total().await, 1);
    }
}
