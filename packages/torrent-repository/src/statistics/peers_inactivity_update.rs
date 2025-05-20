//! Job that runs a task on intervals to update peers' inactivity metrics.
use std::sync::Arc;

use chrono::Utc;
use tokio::task::JoinHandle;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_primitives::DurationSinceUnixEpoch;
use tracing::instrument;

use super::repository::Repository;
use crate::statistics::TORRENT_REPOSITORY_PEERS_INACTIVE_TOTAL;
use crate::{CurrentClock, Swarms};

#[must_use]
#[instrument(skip(swarms, stats_repository))]
pub fn start_job(
    swarms: &Arc<Swarms>,
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
                    tracing::info!("Stopping peers inactivity metrics update job ...");
                    break;
                }
                _ = interval.tick() => {
                    if let (Some(swarms), Some(stats_repository)) = (weak_swarms.upgrade(), weak_stats_repository.upgrade()) {
                        let start_time = Utc::now().time();

                        tracing::debug!("Updating peers inactivity metrics (executed every {} secs) ...", interval_in_secs);

                        let inactive_peers_total = swarms.count_inactive_peers(inactivity_cutoff).await;

                        tracing::info!(inactive_peers_total = inactive_peers_total);

                        #[allow(clippy::cast_precision_loss)]
                        let inactive_peers_total = inactive_peers_total as f64;

                        let _unused = stats_repository
                            .set_gauge(
                                &metric_name!(TORRENT_REPOSITORY_PEERS_INACTIVE_TOTAL),
                                &LabelSet::default(),
                                inactive_peers_total,
                                CurrentClock::now(),
                            )
                            .await;

                        tracing::debug!(
                            "Peers inactivity metrics updated in {} ms",
                            (Utc::now().time() - start_time).num_milliseconds()
                        );
                    } else {
                        break;
                    }
                }
            }
        }
    })
}
