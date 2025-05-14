use std::sync::Arc;

use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::{label_name, metric_name};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::Event;
use crate::statistics::repository::Repository;
use crate::statistics::{TORRENT_REPOSITORY_PEERS_TOTAL, TORRENT_REPOSITORY_TORRENTS_TOTAL};

pub async fn handle_event(event: Event, stats_repository: &Arc<Repository>, now: DurationSinceUnixEpoch) {
    match event {
        Event::TorrentAdded { info_hash, .. } => {
            tracing::debug!(info_hash = ?info_hash, "Torrent added",);

            match stats_repository
                .increment_gauge(&metric_name!(TORRENT_REPOSITORY_TORRENTS_TOTAL), &LabelSet::default(), now)
                .await
            {
                Ok(()) => {}
                Err(err) => tracing::error!("Failed to increment the gauge: {}", err),
            };
        }
        Event::TorrentRemoved { info_hash } => {
            tracing::debug!(info_hash = ?info_hash, "Torrent removed",);

            match stats_repository
                .decrement_gauge(&metric_name!(TORRENT_REPOSITORY_TORRENTS_TOTAL), &LabelSet::default(), now)
                .await
            {
                Ok(()) => {}
                Err(err) => tracing::error!("Failed to decrement the gauge: {}", err),
            };
        }
        Event::PeerAdded { peer } => {
            tracing::debug!(peer = ?peer, "Peer added", );

            let label_set: LabelSet = if peer.is_seeder() {
                (label_name!("peer_role"), LabelValue::new("seeder")).into()
            } else {
                (label_name!("peer_role"), LabelValue::new("leecher")).into()
            };

            match stats_repository
                .increment_gauge(&metric_name!(TORRENT_REPOSITORY_PEERS_TOTAL), &label_set, now)
                .await
            {
                Ok(()) => {}
                Err(err) => tracing::error!("Failed to increment the gauge: {}", err),
            };
        }
        Event::PeerRemoved { peer } => {
            tracing::debug!(peer = ?peer, "Peer removed", );

            let label_set: LabelSet = if peer.is_seeder() {
                (label_name!("peer_role"), LabelValue::new("seeder")).into()
            } else {
                (label_name!("peer_role"), LabelValue::new("leecher")).into()
            };

            match stats_repository
                .decrement_gauge(&metric_name!(TORRENT_REPOSITORY_PEERS_TOTAL), &label_set, now)
                .await
            {
                Ok(()) => {}
                Err(err) => tracing::error!("Failed to decrement the gauge: {}", err),
            };
        }
    }
}
