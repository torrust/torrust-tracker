use std::sync::Arc;

use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_primitives::DurationSinceUnixEpoch;
use torrust_tracker_swarm_coordination_registry::event::Event;

use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
use crate::statistics::repository::Repository;
use crate::statistics::TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL;

pub async fn handle_event(
    event: Event,
    stats_repository: &Arc<Repository>,
    db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    persistent_torrent_completed_stat: bool,
    now: DurationSinceUnixEpoch,
) {
    match event {
        // Torrent events
        Event::TorrentAdded { info_hash, .. } => {
            tracing::debug!(info_hash = ?info_hash, "Torrent added",);
        }
        Event::TorrentRemoved { info_hash } => {
            tracing::debug!(info_hash = ?info_hash, "Torrent removed",);
        }

        // Peer events
        Event::PeerAdded { info_hash, peer } => {
            tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer added", );
        }
        Event::PeerRemoved { info_hash, peer } => {
            tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer removed", );
        }
        Event::PeerUpdated {
            info_hash,
            old_peer,
            new_peer,
        } => {
            tracing::debug!(info_hash = ?info_hash, old_peer = ?old_peer, new_peer = ?new_peer, "Peer updated");
        }
        Event::PeerDownloadCompleted { info_hash, peer } => {
            tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer download completed", );

            // Increment the number of downloads for all the torrents in memory
            let _unused = stats_repository
                .increment_counter(
                    &metric_name!(TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL),
                    &LabelSet::default(),
                    now,
                )
                .await;

            if persistent_torrent_completed_stat {
                // Increment the number of downloads for the torrent in the database
                match db_downloads_metric_repository.increase_downloads_for_torrent(&info_hash) {
                    Ok(()) => {
                        tracing::debug!(info_hash = ?info_hash, "Number of torrent downloads increased");
                    }
                    Err(err) => {
                        tracing::error!(info_hash = ?info_hash, error = ?err, "Failed to increase number of downloads for the torrent");
                    }
                }

                // Increment the global number of downloads (for all torrents) in the database
                match db_downloads_metric_repository.increase_global_downloads() {
                    Ok(()) => {
                        tracing::debug!("Global number of downloads increased");
                    }
                    Err(err) => {
                        tracing::error!(error = ?err, "Failed to increase global number of downloads");
                    }
                }
            }
        }
    }
}
