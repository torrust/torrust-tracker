use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::torrent_metrics::TorrentsMetrics;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrents};
use torrust_tracker_torrent_repository::repository::{Repository as _, RepositoryAsync as _};
use torrust_tracker_torrent_repository::{
    EntrySingle, TorrentsRwLockStd, TorrentsRwLockStdMutexStd, TorrentsRwLockStdMutexTokio, TorrentsRwLockTokio,
    TorrentsRwLockTokioMutexStd, TorrentsRwLockTokioMutexTokio,
};

#[derive(Debug)]
pub(crate) enum Repo {
    Std(TorrentsRwLockStd),
    StdMutexStd(TorrentsRwLockStdMutexStd),
    StdMutexTokio(TorrentsRwLockStdMutexTokio),
    Tokio(TorrentsRwLockTokio),
    TokioMutexStd(TorrentsRwLockTokioMutexStd),
    TokioMutexTokio(TorrentsRwLockTokioMutexTokio),
}

impl Repo {
    pub(crate) async fn get(&self, key: &InfoHash) -> Option<EntrySingle> {
        match self {
            Repo::Std(repo) => repo.get(key),
            Repo::StdMutexStd(repo) => Some(repo.get(key)?.lock().unwrap().clone()),
            Repo::StdMutexTokio(repo) => Some(repo.get(key).await?.lock().await.clone()),
            Repo::Tokio(repo) => repo.get(key).await,
            Repo::TokioMutexStd(repo) => Some(repo.get(key).await?.lock().unwrap().clone()),
            Repo::TokioMutexTokio(repo) => Some(repo.get(key).await?.lock().await.clone()),
        }
    }
    pub(crate) async fn get_metrics(&self) -> TorrentsMetrics {
        match self {
            Repo::Std(repo) => repo.get_metrics(),
            Repo::StdMutexStd(repo) => repo.get_metrics(),
            Repo::StdMutexTokio(repo) => repo.get_metrics().await,
            Repo::Tokio(repo) => repo.get_metrics().await,
            Repo::TokioMutexStd(repo) => repo.get_metrics().await,
            Repo::TokioMutexTokio(repo) => repo.get_metrics().await,
        }
    }
    pub(crate) async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, EntrySingle)> {
        match self {
            Repo::Std(repo) => repo.get_paginated(pagination),
            Repo::StdMutexStd(repo) => repo
                .get_paginated(pagination)
                .iter()
                .map(|(i, t)| (*i, t.lock().expect("it should get a lock").clone()))
                .collect(),
            Repo::StdMutexTokio(repo) => {
                let mut v: Vec<(InfoHash, EntrySingle)> = vec![];

                for (i, t) in repo.get_paginated(pagination).await {
                    v.push((i, t.lock().await.clone()));
                }
                v
            }
            Repo::Tokio(repo) => repo.get_paginated(pagination).await,
            Repo::TokioMutexStd(repo) => repo
                .get_paginated(pagination)
                .await
                .iter()
                .map(|(i, t)| (*i, t.lock().expect("it should get a lock").clone()))
                .collect(),
            Repo::TokioMutexTokio(repo) => {
                let mut v: Vec<(InfoHash, EntrySingle)> = vec![];

                for (i, t) in repo.get_paginated(pagination).await {
                    v.push((i, t.lock().await.clone()));
                }
                v
            }
        }
    }
    pub(crate) async fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        match self {
            Repo::Std(repo) => repo.import_persistent(persistent_torrents),
            Repo::StdMutexStd(repo) => repo.import_persistent(persistent_torrents),
            Repo::StdMutexTokio(repo) => repo.import_persistent(persistent_torrents).await,
            Repo::Tokio(repo) => repo.import_persistent(persistent_torrents).await,
            Repo::TokioMutexStd(repo) => repo.import_persistent(persistent_torrents).await,
            Repo::TokioMutexTokio(repo) => repo.import_persistent(persistent_torrents).await,
        }
    }
    pub(crate) async fn remove(&self, key: &InfoHash) -> Option<EntrySingle> {
        match self {
            Repo::Std(repo) => repo.remove(key),
            Repo::StdMutexStd(repo) => Some(repo.remove(key)?.lock().unwrap().clone()),
            Repo::StdMutexTokio(repo) => Some(repo.remove(key).await?.lock().await.clone()),
            Repo::Tokio(repo) => repo.remove(key).await,
            Repo::TokioMutexStd(repo) => Some(repo.remove(key).await?.lock().unwrap().clone()),
            Repo::TokioMutexTokio(repo) => Some(repo.remove(key).await?.lock().await.clone()),
        }
    }
    pub(crate) async fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        match self {
            Repo::Std(repo) => repo.remove_inactive_peers(current_cutoff),
            Repo::StdMutexStd(repo) => repo.remove_inactive_peers(current_cutoff),
            Repo::StdMutexTokio(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Repo::Tokio(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Repo::TokioMutexStd(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Repo::TokioMutexTokio(repo) => repo.remove_inactive_peers(current_cutoff).await,
        }
    }
    pub(crate) async fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        match self {
            Repo::Std(repo) => repo.remove_peerless_torrents(policy),
            Repo::StdMutexStd(repo) => repo.remove_peerless_torrents(policy),
            Repo::StdMutexTokio(repo) => repo.remove_peerless_torrents(policy).await,
            Repo::Tokio(repo) => repo.remove_peerless_torrents(policy).await,
            Repo::TokioMutexStd(repo) => repo.remove_peerless_torrents(policy).await,
            Repo::TokioMutexTokio(repo) => repo.remove_peerless_torrents(policy).await,
        }
    }
    pub(crate) async fn update_torrent_with_peer_and_get_stats(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
    ) -> (bool, SwarmMetadata) {
        match self {
            Repo::Std(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer),
            Repo::StdMutexStd(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer),
            Repo::StdMutexTokio(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer).await,
            Repo::Tokio(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer).await,
            Repo::TokioMutexStd(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer).await,
            Repo::TokioMutexTokio(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer).await,
        }
    }
    pub(crate) async fn insert(&self, info_hash: &InfoHash, torrent: EntrySingle) -> Option<EntrySingle> {
        match self {
            Repo::Std(repo) => repo.write().insert(*info_hash, torrent),
            Repo::StdMutexStd(repo) => Some(repo.write().insert(*info_hash, torrent.into())?.lock().unwrap().clone()),
            Repo::StdMutexTokio(repo) => {
                let r = repo.write().insert(*info_hash, torrent.into());
                match r {
                    Some(t) => Some(t.lock().await.clone()),
                    None => None,
                }
            }
            Repo::Tokio(repo) => repo.write().await.insert(*info_hash, torrent),
            Repo::TokioMutexStd(repo) => Some(repo.write().await.insert(*info_hash, torrent.into())?.lock().unwrap().clone()),
            Repo::TokioMutexTokio(repo) => Some(repo.write().await.insert(*info_hash, torrent.into())?.lock().await.clone()),
        }
    }
}
