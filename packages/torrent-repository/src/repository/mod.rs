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

use std::fmt::Debug;

pub trait Repository<T>: Debug + Default + Sized + 'static {
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
pub trait RepositoryAsync<T>: Debug + Default + Sized + 'static {
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

#[derive(Default, Debug)]
pub struct RwLockStd<T> {
    torrents: std::sync::RwLock<std::collections::BTreeMap<InfoHash, T>>,
}

#[derive(Default, Debug)]
pub struct RwLockTokio<T> {
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, T>>,
}

impl<T> RwLockStd<T> {
    /// # Panics
    ///
    /// Panics if unable to get a lock.
    pub fn write(
        &self,
    ) -> std::sync::RwLockWriteGuard<'_, std::collections::BTreeMap<torrust_tracker_primitives::info_hash::InfoHash, T>> {
        self.torrents.write().expect("it should get lock")
    }
}

impl<T> RwLockTokio<T> {
    pub fn write(
        &self,
    ) -> impl std::future::Future<
        Output = tokio::sync::RwLockWriteGuard<
            '_,
            std::collections::BTreeMap<torrust_tracker_primitives::info_hash::InfoHash, T>,
        >,
    > {
        self.torrents.write()
    }
}
