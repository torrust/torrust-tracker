use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::torrent_metrics::TorrentsMetrics;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrents};
use torrust_tracker_torrent_repository::repository::{Repository as _, RepositoryAsync as _};
use torrust_tracker_torrent_repository::{
    EntrySingle, TorrentsRwLockStd, TorrentsRwLockStdMutexStd, TorrentsRwLockStdMutexTokio, TorrentsRwLockTokio,
    TorrentsRwLockTokioMutexStd, TorrentsRwLockTokioMutexTokio, TorrentsSkipMapMutexStd,
};

#[derive(Debug)]
pub(crate) enum Repo {
    RwLockStd(TorrentsRwLockStd),
    RwLockStdMutexStd(TorrentsRwLockStdMutexStd),
    RwLockStdMutexTokio(TorrentsRwLockStdMutexTokio),
    RwLockTokio(TorrentsRwLockTokio),
    RwLockTokioMutexStd(TorrentsRwLockTokioMutexStd),
    RwLockTokioMutexTokio(TorrentsRwLockTokioMutexTokio),
    SkipMapMutexStd(TorrentsSkipMapMutexStd),
}

impl Repo {
    pub(crate) async fn get(&self, key: &InfoHash) -> Option<EntrySingle> {
        match self {
            Repo::RwLockStd(repo) => repo.get(key),
            Repo::RwLockStdMutexStd(repo) => Some(repo.get(key)?.lock().unwrap().clone()),
            Repo::RwLockStdMutexTokio(repo) => Some(repo.get(key).await?.lock().await.clone()),
            Repo::RwLockTokio(repo) => repo.get(key).await,
            Repo::RwLockTokioMutexStd(repo) => Some(repo.get(key).await?.lock().unwrap().clone()),
            Repo::RwLockTokioMutexTokio(repo) => Some(repo.get(key).await?.lock().await.clone()),
            Repo::SkipMapMutexStd(repo) => Some(repo.get(key)?.lock().unwrap().clone()),
        }
    }

    pub(crate) async fn get_metrics(&self) -> TorrentsMetrics {
        match self {
            Repo::RwLockStd(repo) => repo.get_metrics(),
            Repo::RwLockStdMutexStd(repo) => repo.get_metrics(),
            Repo::RwLockStdMutexTokio(repo) => repo.get_metrics().await,
            Repo::RwLockTokio(repo) => repo.get_metrics().await,
            Repo::RwLockTokioMutexStd(repo) => repo.get_metrics().await,
            Repo::RwLockTokioMutexTokio(repo) => repo.get_metrics().await,
            Repo::SkipMapMutexStd(repo) => repo.get_metrics(),
        }
    }

    pub(crate) async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, EntrySingle)> {
        match self {
            Repo::RwLockStd(repo) => repo.get_paginated(pagination),
            Repo::RwLockStdMutexStd(repo) => repo
                .get_paginated(pagination)
                .iter()
                .map(|(i, t)| (*i, t.lock().expect("it should get a lock").clone()))
                .collect(),
            Repo::RwLockStdMutexTokio(repo) => {
                let mut v: Vec<(InfoHash, EntrySingle)> = vec![];

                for (i, t) in repo.get_paginated(pagination).await {
                    v.push((i, t.lock().await.clone()));
                }
                v
            }
            Repo::RwLockTokio(repo) => repo.get_paginated(pagination).await,
            Repo::RwLockTokioMutexStd(repo) => repo
                .get_paginated(pagination)
                .await
                .iter()
                .map(|(i, t)| (*i, t.lock().expect("it should get a lock").clone()))
                .collect(),
            Repo::RwLockTokioMutexTokio(repo) => {
                let mut v: Vec<(InfoHash, EntrySingle)> = vec![];

                for (i, t) in repo.get_paginated(pagination).await {
                    v.push((i, t.lock().await.clone()));
                }
                v
            }
            Repo::SkipMapMutexStd(repo) => repo
                .get_paginated(pagination)
                .iter()
                .map(|(i, t)| (*i, t.lock().expect("it should get a lock").clone()))
                .collect(),
        }
    }

    pub(crate) async fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        match self {
            Repo::RwLockStd(repo) => repo.import_persistent(persistent_torrents),
            Repo::RwLockStdMutexStd(repo) => repo.import_persistent(persistent_torrents),
            Repo::RwLockStdMutexTokio(repo) => repo.import_persistent(persistent_torrents).await,
            Repo::RwLockTokio(repo) => repo.import_persistent(persistent_torrents).await,
            Repo::RwLockTokioMutexStd(repo) => repo.import_persistent(persistent_torrents).await,
            Repo::RwLockTokioMutexTokio(repo) => repo.import_persistent(persistent_torrents).await,
            Repo::SkipMapMutexStd(repo) => repo.import_persistent(persistent_torrents),
        }
    }

    pub(crate) async fn remove(&self, key: &InfoHash) -> Option<EntrySingle> {
        match self {
            Repo::RwLockStd(repo) => repo.remove(key),
            Repo::RwLockStdMutexStd(repo) => Some(repo.remove(key)?.lock().unwrap().clone()),
            Repo::RwLockStdMutexTokio(repo) => Some(repo.remove(key).await?.lock().await.clone()),
            Repo::RwLockTokio(repo) => repo.remove(key).await,
            Repo::RwLockTokioMutexStd(repo) => Some(repo.remove(key).await?.lock().unwrap().clone()),
            Repo::RwLockTokioMutexTokio(repo) => Some(repo.remove(key).await?.lock().await.clone()),
            Repo::SkipMapMutexStd(repo) => Some(repo.remove(key)?.lock().unwrap().clone()),
        }
    }

    pub(crate) async fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        match self {
            Repo::RwLockStd(repo) => repo.remove_inactive_peers(current_cutoff),
            Repo::RwLockStdMutexStd(repo) => repo.remove_inactive_peers(current_cutoff),
            Repo::RwLockStdMutexTokio(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Repo::RwLockTokio(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Repo::RwLockTokioMutexStd(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Repo::RwLockTokioMutexTokio(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Repo::SkipMapMutexStd(repo) => repo.remove_inactive_peers(current_cutoff),
        }
    }

    pub(crate) async fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        match self {
            Repo::RwLockStd(repo) => repo.remove_peerless_torrents(policy),
            Repo::RwLockStdMutexStd(repo) => repo.remove_peerless_torrents(policy),
            Repo::RwLockStdMutexTokio(repo) => repo.remove_peerless_torrents(policy).await,
            Repo::RwLockTokio(repo) => repo.remove_peerless_torrents(policy).await,
            Repo::RwLockTokioMutexStd(repo) => repo.remove_peerless_torrents(policy).await,
            Repo::RwLockTokioMutexTokio(repo) => repo.remove_peerless_torrents(policy).await,
            Repo::SkipMapMutexStd(repo) => repo.remove_peerless_torrents(policy),
        }
    }

    pub(crate) async fn update_torrent_with_peer_and_get_stats(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
    ) -> (bool, SwarmMetadata) {
        match self {
            Repo::RwLockStd(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer),
            Repo::RwLockStdMutexStd(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer),
            Repo::RwLockStdMutexTokio(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer).await,
            Repo::RwLockTokio(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer).await,
            Repo::RwLockTokioMutexStd(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer).await,
            Repo::RwLockTokioMutexTokio(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer).await,
            Repo::SkipMapMutexStd(repo) => repo.update_torrent_with_peer_and_get_stats(info_hash, peer),
        }
    }

    pub(crate) async fn insert(&self, info_hash: &InfoHash, torrent: EntrySingle) -> Option<EntrySingle> {
        match self {
            Repo::RwLockStd(repo) => {
                repo.write().insert(*info_hash, torrent);
            }
            Repo::RwLockStdMutexStd(repo) => {
                repo.write().insert(*info_hash, torrent.into());
            }
            Repo::RwLockStdMutexTokio(repo) => {
                repo.write().insert(*info_hash, torrent.into());
            }
            Repo::RwLockTokio(repo) => {
                repo.write().await.insert(*info_hash, torrent);
            }
            Repo::RwLockTokioMutexStd(repo) => {
                repo.write().await.insert(*info_hash, torrent.into());
            }
            Repo::RwLockTokioMutexTokio(repo) => {
                repo.write().await.insert(*info_hash, torrent.into());
            }
            Repo::SkipMapMutexStd(repo) => {
                repo.torrents.insert(*info_hash, torrent.into());
            }
        };
        self.get(info_hash).await
    }
}
