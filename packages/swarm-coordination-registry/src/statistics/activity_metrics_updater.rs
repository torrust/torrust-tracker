//! Job that runs a task on intervals to update peers' activity metrics.
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric_name;
use torrust_tracker_events::shutdown::Completion;
use tracing::instrument;

use super::repository::Repository;
use crate::statistics::{SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL, SWARM_COORDINATION_REGISTRY_TORRENTS_INACTIVE_TOTAL};
use crate::{CurrentClock, Registry};

const ACTIVITY_METRICS_UPDATE_INTERVAL_SECS: u64 = 15;

#[must_use]
#[instrument(skip(swarms, stats_repository))]
pub fn run_job(
    swarms: Arc<Registry>,
    stats_repository: Arc<Repository>,
    max_peer_timeout: u32,
    cancellation_token: CancellationToken,
) -> impl Future<Output = Completion> + Send + 'static {
    async move {
        let weak_swarms = Arc::downgrade(&swarms);
        let weak_stats_repository = Arc::downgrade(&stats_repository);
        drop(swarms);
        drop(stats_repository);

        let interval_in_secs = ACTIVITY_METRICS_UPDATE_INTERVAL_SECS; // todo: make this configurable
        let interval = std::time::Duration::from_secs(interval_in_secs);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await;

        loop {
            tokio::select! {
                biased;
                () = cancellation_token.cancelled() => {
                    tracing::info!("Stopping peers activity metrics update job ...");
                    return Completion::Cancelled;
                }
                _ = interval.tick() => {
                    if let (Some(swarms), Some(stats_repository)) = (weak_swarms.upgrade(), weak_stats_repository.upgrade()) {
                        update_activity_metrics(interval_in_secs, &swarms, &stats_repository, max_peer_timeout).await;
                    } else {
                        tracing::info!("Stopping peers activity metrics update job (can't upgrade weak pointers) ...");
                        return Completion::Completed;
                    }
                }
            }
        }
    }
}

async fn update_activity_metrics(
    interval_in_secs: u64,
    swarms: &Arc<Registry>,
    stats_repository: &Arc<Repository>,
    max_peer_timeout: u32,
) {
    let start_time = Utc::now().time();

    tracing::debug!(
        "Updating peers and torrents activity metrics (executed every {} secs) ...",
        interval_in_secs
    );

    // Follow-up timestamp naming and API audit: see
    // docs/issues/open/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/follow-up-issue-draft.md.
    let inactivity_cutoff_timestamp =
        CurrentClock::now_sub(&Duration::from_secs(u64::from(max_peer_timeout))).unwrap_or_default();
    let activity_metadata = swarms.get_activity_metadata(inactivity_cutoff_timestamp).await;

    activity_metadata.log();

    update_inactive_peers_total(stats_repository, activity_metadata.inactive_peers_total).await;
    update_inactive_torrents_total(stats_repository, activity_metadata.inactive_torrents_total).await;

    tracing::debug!(
        "Peers and torrents activity metrics updated in {} ms",
        (Utc::now().time() - start_time).num_milliseconds()
    );
}

async fn update_inactive_peers_total(stats_repository: &Arc<Repository>, inactive_peers_total: usize) {
    #[allow(clippy::cast_precision_loss)]
    let inactive_peers_total = inactive_peers_total as f64;

    let _unused = stats_repository
        .set_gauge(
            &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL),
            &LabelSet::default(),
            inactive_peers_total,
            CurrentClock::now(),
        )
        .await;
}

async fn update_inactive_torrents_total(stats_repository: &Arc<Repository>, inactive_torrents_total: usize) {
    #[allow(clippy::cast_precision_loss)]
    let inactive_torrents_total = inactive_torrents_total as f64;

    let _unused = stats_repository
        .set_gauge(
            &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_INACTIVE_TOTAL),
            &LabelSet::default(),
            inactive_torrents_total,
            CurrentClock::now(),
        )
        .await;
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::time::timeout;
    use tokio_util::sync::{CancellationToken, DropGuard};
    use torrust_clock::clock::stopped::Stopped as _;
    use torrust_clock::{DurationSinceUnixEpoch, clock};
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;
    use torrust_tracker_events::shutdown::Completion;

    use super::{ACTIVITY_METRICS_UPDATE_INTERVAL_SECS, run_job};
    use crate::Registry;
    use crate::statistics::SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL;
    use crate::statistics::repository::Repository;
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

    #[tokio::test]
    async fn it_should_return_cancelled_when_the_token_is_cancelled() {
        // Arrange
        let swarms = Arc::new(Registry::new(None));
        let stats_repository = Arc::new(Repository::new());
        let cancellation_token = CancellationToken::new();
        let runner = run_job(swarms, stats_repository, 0, cancellation_token.clone());

        // Act
        cancellation_token.cancel();
        let completion = timeout(Duration::from_secs(1), runner)
            .await
            .expect("the activity metrics runner should stop after cancellation");

        // Assert
        assert_eq!(completion, Completion::Cancelled);
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_return_completed_when_the_registry_is_dropped() {
        // Arrange: the only strong registry reference is dropped with the
        // block below, before the runner's first update tick.
        let stats_repository = Arc::new(Repository::new());
        let runner = {
            let swarms = Arc::new(Registry::new(None));

            run_job(swarms, stats_repository.clone(), 0, CancellationToken::new())
        };
        let runner = tokio::spawn(runner);
        tokio::task::yield_now().await;

        // Act
        tokio::time::advance(Duration::from_secs(15)).await;
        let completion = runner.await.expect("the activity metrics runner should not panic");

        // Assert
        assert_eq!(completion, Completion::Completed);
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_return_completed_when_the_statistics_repository_is_dropped() {
        // Arrange: the only strong statistics repository reference is dropped
        // with the block below, before the runner's first update tick.
        let swarms = Arc::new(Registry::new(None));
        let runner = {
            let stats_repository = Arc::new(Repository::new());

            run_job(swarms.clone(), stats_repository, 0, CancellationToken::new())
        };
        let runner = tokio::spawn(runner);
        tokio::task::yield_now().await;

        // Act
        tokio::time::advance(Duration::from_secs(15)).await;
        let completion = runner.await.expect("the activity metrics runner should not panic");

        // Assert
        assert_eq!(completion, Completion::Completed);
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
