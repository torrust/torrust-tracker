use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::{AggregateSwarmMetadata, SwarmMetadata};
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrent, PersistentTorrents};

pub mod skip_map_mutex_std;

use std::fmt::Debug;

pub trait Repository<T>: Debug + Default + Sized + 'static {
    fn get(&self, key: &InfoHash) -> Option<T>;
    fn get_metrics(&self) -> AggregateSwarmMetadata;
    fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, T)>;
    fn import_persistent(&self, persistent_torrents: &PersistentTorrents);
    fn remove(&self, key: &InfoHash) -> Option<T>;
    fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch);
    fn remove_peerless_torrents(&self, policy: &TrackerPolicy);
    fn upsert_peer(&self, info_hash: &InfoHash, peer: &peer::Peer, opt_persistent_torrent: Option<PersistentTorrent>) -> bool;
    fn get_swarm_metadata(&self, info_hash: &InfoHash) -> Option<SwarmMetadata>;
}
