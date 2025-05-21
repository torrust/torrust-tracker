use std::sync::Arc;

use torrust_tracker_primitives::DurationSinceUnixEpoch;
use torrust_tracker_torrent_repository::event::Event;

use crate::torrent::repository::persisted::DatabasePersistentTorrentRepository;

pub async fn handle_event(
    event: Event,
    db_torrent_repository: &Arc<DatabasePersistentTorrentRepository>,
    _now: DurationSinceUnixEpoch,
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

            match db_torrent_repository.increase_number_of_downloads(&info_hash) {
                Ok(()) => {
                    tracing::debug!(info_hash = ?info_hash, "Number of downloads increased");
                }
                Err(err) => {
                    tracing::error!(info_hash = ?info_hash, error = ?err, "Failed to increase number of downloads");
                }
            }
        }
    }
}
