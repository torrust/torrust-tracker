use std::sync::Arc;

use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::{label_name, metric_name};
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::Event;
use crate::statistics::repository::Repository;
use crate::statistics::{
    TORRENT_REPOSITORY_PEERS_TOTAL, TORRENT_REPOSITORY_TORRENTS_DOWNLOADS_TOTAL, TORRENT_REPOSITORY_TORRENTS_TOTAL,
};

pub async fn handle_event(event: Event, stats_repository: &Arc<Repository>, now: DurationSinceUnixEpoch) {
    match event {
        Event::TorrentAdded { info_hash, .. } => {
            tracing::debug!(info_hash = ?info_hash, "Torrent added",);

            let _unused = stats_repository
                .increment_gauge(&metric_name!(TORRENT_REPOSITORY_TORRENTS_TOTAL), &LabelSet::default(), now)
                .await;
        }
        Event::TorrentRemoved { info_hash } => {
            tracing::debug!(info_hash = ?info_hash, "Torrent removed",);

            let _unused = stats_repository
                .decrement_gauge(&metric_name!(TORRENT_REPOSITORY_TORRENTS_TOTAL), &LabelSet::default(), now)
                .await;
        }
        Event::PeerAdded { info_hash, peer } => {
            tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer added", );

            let _unused = stats_repository
                .increment_gauge(&metric_name!(TORRENT_REPOSITORY_PEERS_TOTAL), &label_set_for_peer(&peer), now)
                .await;
        }
        Event::PeerRemoved { info_hash, peer } => {
            tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer removed", );

            let _unused = stats_repository
                .decrement_gauge(&metric_name!(TORRENT_REPOSITORY_PEERS_TOTAL), &label_set_for_peer(&peer), now)
                .await;
        }
        Event::PeerUpdated {
            info_hash,
            old_peer,
            new_peer,
        } => {
            tracing::debug!(info_hash = ?info_hash, old_peer = ?old_peer, new_peer = ?new_peer, "Peer updated", );

            if old_peer.role() != new_peer.role() {
                let _unused = stats_repository
                    .increment_gauge(
                        &metric_name!(TORRENT_REPOSITORY_PEERS_TOTAL),
                        &label_set_for_peer(&new_peer),
                        now,
                    )
                    .await;

                let _unused = stats_repository
                    .decrement_gauge(
                        &metric_name!(TORRENT_REPOSITORY_PEERS_TOTAL),
                        &label_set_for_peer(&old_peer),
                        now,
                    )
                    .await;
            }
        }
        Event::PeerDownloadCompleted { info_hash, peer } => {
            tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer download completed", );

            let _unused = stats_repository
                .increment_counter(
                    &metric_name!(TORRENT_REPOSITORY_TORRENTS_DOWNLOADS_TOTAL),
                    &label_set_for_peer(&peer),
                    now,
                )
                .await;
        }
    }
}

/// Returns the label set to be included in the metrics for the given peer.
fn label_set_for_peer(peer: &Peer) -> LabelSet {
    if peer.is_seeder() {
        (label_name!("peer_role"), LabelValue::new("seeder")).into()
    } else {
        (label_name!("peer_role"), LabelValue::new("leecher")).into()
    }
}
