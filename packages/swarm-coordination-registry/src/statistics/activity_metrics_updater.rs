//! Job that runs a task on intervals to update peers' activity metrics.
use std::sync::Arc;

use chrono::Utc;
use tokio::task::JoinHandle;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_primitives::DurationSinceUnixEpoch;
use tracing::instrument;

use super::repository::Repository;
use crate::statistics::{SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL, SWARM_COORDINATION_REGISTRY_TORRENTS_INACTIVE_TOTAL};
use crate::{CurrentClock, Registry};

#[must_use]
#[instrument(skip(swarms, stats_repository))]
pub fn start_job(
    swarms: &Arc<Registry>,
    stats_repository: &Arc<Repository>,
    inactivity_cutoff: DurationSinceUnixEpoch,
) -> JoinHandle<()> {
    let weak_swarms = std::sync::Arc::downgrade(swarms);
    let weak_stats_repository = std::sync::Arc::downgrade(stats_repository);

    let interval_in_secs = 15; // todo: make this configurable

    tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(interval_in_secs);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await;

        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Stopping peers activity metrics update job (ctrl-c signal received) ...");
                    break;
                }
                _ = interval.tick() => {
                    if let (Some(swarms), Some(stats_repository)) = (weak_swarms.upgrade(), weak_stats_repository.upgrade()) {
                        update_activity_metrics(interval_in_secs, &swarms, &stats_repository, inactivity_cutoff).await;
                    } else {
                        tracing::info!("Stopping peers activity metrics update job (can't upgrade weak pointers) ...");
                        break;
                    }
                }
            }
        }
    })
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
