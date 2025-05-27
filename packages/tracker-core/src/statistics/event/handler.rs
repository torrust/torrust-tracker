use std::sync::Arc;

use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_primitives::DurationSinceUnixEpoch;
use torrust_tracker_torrent_repository::event::Event;

use crate::statistics::repository::Repository;
use crate::statistics::TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL;
use crate::torrent::repository::persisted::DatabasePersistentTorrentRepository;

pub async fn handle_event(
    event: Event,
    stats_repository: &Arc<Repository>,
    db_torrent_repository: &Arc<DatabasePersistentTorrentRepository>,
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

            // Increment the number of downloads for the torrent
            match db_torrent_repository.increase_number_of_downloads(&info_hash) {
                Ok(()) => {
                    tracing::debug!(info_hash = ?info_hash, "Number of downloads increased");
                }
                Err(err) => {
                    tracing::error!(info_hash = ?info_hash, error = ?err, "Failed to increase number of downloads");
                }
            }

            // Increment the number of downloads for all the torrents
            let _unused = stats_repository
                .increment_counter(
                    &metric_name!(TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL),
                    &LabelSet::default(),
                    now,
                )
                .await;

            // todo:
            //   - Persist the metric into the database.
            //   - Load the metric from the database.
        }
    }
}
