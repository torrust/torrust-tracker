use std::sync::Arc;

use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::Event;
use crate::statistics::repository::Repository;

/// # Panics
///
/// This function panics if the client IP address is not the same as the IP
/// version of the event.
pub async fn handle_event(event: Event, stats_repository: &Arc<Repository>, _now: DurationSinceUnixEpoch) {
    match event {
        Event::TorrentAdded { info_hash, .. } => {
            // todo: update metrics
            tracing::debug!("Torrent added {info_hash}");
        }
        Event::TorrentRemoved { info_hash } => {
            // todo: update metrics
            tracing::debug!("Torrent removed {info_hash}");
        }
        Event::PeerAdded { announcement } => {
            // todo: update metrics
            tracing::debug!("Peer added {announcement:?}");
        }
        Event::PeerRemoved { socket_addr, peer_id } => {
            // todo: update metrics
            tracing::debug!("Peer removed: socket address {socket_addr:?}, peer ID: {peer_id:?}");
        }
    }

    tracing::debug!("metrics: {:?}", stats_repository.get_metrics().await);
}
