use std::sync::Arc;

use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::Event;
use crate::statistics::repository::Repository;
use crate::statistics::TORRENT_REPOSITORY_TORRENTS_TOTAL;

pub async fn handle_event(event: Event, stats_repository: &Arc<Repository>, now: DurationSinceUnixEpoch) {
    match event {
        Event::TorrentAdded { info_hash, .. } => {
            tracing::debug!("Torrent added {info_hash}");

            match stats_repository
                .increment_gauge(&metric_name!(TORRENT_REPOSITORY_TORRENTS_TOTAL), &LabelSet::default(), now)
                .await
            {
                Ok(()) => {}
                Err(err) => tracing::error!("Failed to increment the gauge: {}", err),
            };
        }
        Event::TorrentRemoved { info_hash } => {
            tracing::debug!("Torrent removed {info_hash}");

            match stats_repository
                .decrement_gauge(&metric_name!(TORRENT_REPOSITORY_TORRENTS_TOTAL), &LabelSet::default(), now)
                .await
            {
                Ok(()) => {}
                Err(err) => tracing::error!("Failed to decrement the gauge: {}", err),
            };
        }
        Event::PeerAdded { announcement } => {
            // todo: update metrics
            tracing::debug!("Peer added {announcement:?}");
        }
        Event::PeerRemoved {
            peer_addr: socket_addr,
            peer_id,
        } => {
            // todo: update metrics
            tracing::debug!("Peer removed: socket address {socket_addr:?}, peer ID: {peer_id:?}");
        }
    }
}
