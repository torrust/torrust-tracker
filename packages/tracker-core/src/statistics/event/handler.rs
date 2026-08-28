use std::sync::Arc;

use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric_name;
use torrust_tracker_swarm_coordination_registry::event::Event;

use crate::statistics::TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL;
use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
use crate::statistics::repository::Repository;

/// Handles a swarm coordination event and updates in-memory tracker statistics.
pub async fn handle_in_memory_event(event: Event, stats_repository: &Arc<Repository>, now: DurationSinceUnixEpoch) {
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
        }
    }
}

/// Handles a swarm coordination event and persists completed-download statistics.
pub async fn handle_persistent_completed_statistics_event(
    event: Event,
    db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
) {
    if let Event::PeerDownloadCompleted { info_hash, .. } = event {
        match db_downloads_metric_repository
            .increase_downloads_for_torrent(&info_hash)
            .await
        {
            Ok(()) => {
                tracing::debug!(info_hash = ?info_hash, "Number of torrent downloads increased");
            }
            Err(err) => {
                tracing::error!(info_hash = ?info_hash, error = ?err, "Failed to increase number of downloads for the torrent");
            }
        }

        match db_downloads_metric_repository.increase_global_downloads().await {
            Ok(()) => {
                tracing::debug!("Global number of downloads increased");
            }
            Err(err) => {
                tracing::error!(error = ?err, "Failed to increase global number of downloads");
            }
        }
    }
}
