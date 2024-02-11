use super::SwarmMetadata;
use crate::core::databases::PersistentTorrents;
use crate::core::services::torrent::Pagination;
use crate::core::{peer, TorrentsMetrics, TrackerPolicy};
use crate::shared::bit_torrent::info_hash::InfoHash;

pub mod rw_lock_std;
pub mod rw_lock_std_mutex_std;
pub mod rw_lock_std_mutex_tokio;
pub mod rw_lock_tokio;
pub mod rw_lock_tokio_mutex_std;
pub mod rw_lock_tokio_mutex_tokio;

pub trait Repository<T>: Default + 'static {
    fn get(&self, key: &InfoHash) -> impl std::future::Future<Output = Option<T>> + Send;
    fn get_metrics(&self) -> impl std::future::Future<Output = TorrentsMetrics> + Send;
    fn get_paginated(&self, pagination: Option<&Pagination>) -> impl std::future::Future<Output = Vec<(InfoHash, T)>> + Send;
    fn import_persistent(&self, persistent_torrents: &PersistentTorrents) -> impl std::future::Future<Output = ()> + Send;
    fn remove(&self, key: &InfoHash) -> impl std::future::Future<Output = Option<T>> + Send;
    fn remove_inactive_peers(&self, max_peer_timeout: u32) -> impl std::future::Future<Output = ()> + Send;
    fn remove_peerless_torrents(&self, policy: &TrackerPolicy) -> impl std::future::Future<Output = ()> + Send;
}

pub trait UpdateTorrentSync {
    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata);
}

pub trait UpdateTorrentAsync {
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
