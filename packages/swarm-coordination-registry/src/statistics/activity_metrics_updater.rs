//! Job that runs a task on intervals to update peers' activity metrics.
use std::future::Future;
use std::sync::Arc;

use chrono::Utc;
use tokio_util::sync::CancellationToken;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_clock::clock::Time;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric_name;
use torrust_tracker_events::shutdown::Completion;
use tracing::instrument;

use super::repository::Repository;
use crate::statistics::{SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL, SWARM_COORDINATION_REGISTRY_TORRENTS_INACTIVE_TOTAL};
use crate::{CurrentClock, Registry};

#[must_use]
#[instrument(skip(swarms, stats_repository))]
pub fn run_job(
    swarms: Arc<Registry>,
    stats_repository: Arc<Repository>,
    inactivity_cutoff: DurationSinceUnixEpoch,
    cancellation_token: CancellationToken,
) -> impl Future<Output = Completion> + Send + 'static {
    async move {
        let weak_swarms = Arc::downgrade(&swarms);
        let weak_stats_repository = Arc::downgrade(&stats_repository);
        drop(swarms);
        drop(stats_repository);

        let interval_in_secs = 15; // todo: make this configurable
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
                        update_activity_metrics(interval_in_secs, &swarms, &stats_repository, inactivity_cutoff).await;
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
    inactivity_cutoff: DurationSinceUnixEpoch,
) {
    let start_time = Utc::now().time();

    tracing::debug!(
        "Updating peers and torrents activity metrics (executed every {} secs) ...",
        interval_in_secs
    );

    let activity_metadata = swarms.get_activity_metadata(inactivity_cutoff).await;

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
    use tokio_util::sync::CancellationToken;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_tracker_events::shutdown::Completion;

    use super::run_job;
    use crate::Registry;
    use crate::statistics::repository::Repository;

    #[tokio::test]
    async fn it_should_return_cancelled_when_the_token_is_cancelled() {
        // Arrange
        let swarms = Arc::new(Registry::new(None));
        let stats_repository = Arc::new(Repository::new());
        let cancellation_token = CancellationToken::new();
        let runner = run_job(
            swarms,
            stats_repository,
            DurationSinceUnixEpoch::default(),
            cancellation_token.clone(),
        );

        // Act
        cancellation_token.cancel();
        let completion = timeout(Duration::from_secs(1), runner)
            .await
            .expect("the activity metrics runner should stop after cancellation");

        // Assert
        assert_eq!(completion, Completion::Cancelled);
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_return_completed_when_the_collaborators_are_dropped() {
        // Arrange: the only strong collaborator references are dropped with
        // the block below, before the runner's first update tick.
        let runner = {
            let swarms = Arc::new(Registry::new(None));
            let stats_repository = Arc::new(Repository::new());

            run_job(
                swarms,
                stats_repository,
                DurationSinceUnixEpoch::default(),
                CancellationToken::new(),
            )
        };
        let runner = tokio::spawn(runner);
        tokio::task::yield_now().await;

        // Act
        tokio::time::advance(Duration::from_secs(15)).await;
        let completion = runner.await.expect("the activity metrics runner should not panic");

        // Assert
        assert_eq!(completion, Completion::Completed);
    }
}
