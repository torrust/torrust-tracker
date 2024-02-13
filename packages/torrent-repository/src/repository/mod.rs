use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::torrent_metrics::TorrentsMetrics;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrents};

pub mod rw_lock_std;
pub mod rw_lock_std_mutex_std;
pub mod rw_lock_std_mutex_tokio;
pub mod rw_lock_tokio;
pub mod rw_lock_tokio_mutex_std;
pub mod rw_lock_tokio_mutex_tokio;

pub trait Repository<T>: Default + 'static {
    fn get(&self, key: &InfoHash) -> Option<T>;
    fn get_metrics(&self) -> TorrentsMetrics;
    fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, T)>;
    fn import_persistent(&self, persistent_torrents: &PersistentTorrents);
    fn remove(&self, key: &InfoHash) -> Option<T>;
    fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch);
    fn remove_peerless_torrents(&self, policy: &TrackerPolicy);
    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata);
}

#[allow(clippy::module_name_repetitions)]
pub trait RepositoryAsync<T>: Default + 'static {
    fn get(&self, key: &InfoHash) -> impl std::future::Future<Output = Option<T>> + Send;
    fn get_metrics(&self) -> impl std::future::Future<Output = TorrentsMetrics> + Send;
    fn get_paginated(&self, pagination: Option<&Pagination>) -> impl std::future::Future<Output = Vec<(InfoHash, T)>> + Send;
    fn import_persistent(&self, persistent_torrents: &PersistentTorrents) -> impl std::future::Future<Output = ()> + Send;
    fn remove(&self, key: &InfoHash) -> impl std::future::Future<Output = Option<T>> + Send;
    fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) -> impl std::future::Future<Output = ()> + Send;
    fn remove_peerless_torrents(&self, policy: &TrackerPolicy) -> impl std::future::Future<Output = ()> + Send;
    fn update_torrent_with_peer_and_get_stats(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
    ) -> impl std::future::Future<Output = (bool, SwarmMetadata)> + Send;
}

#[derive(Default)]
pub struct RwLockTokio<T> {
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, T>>,
}

#[derive(Default)]
pub struct RwLockStd<T> {
    torrents: std::sync::RwLock<std::collections::BTreeMap<InfoHash, T>>,
}
