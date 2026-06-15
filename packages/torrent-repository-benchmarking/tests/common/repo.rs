use torrust_clock::DurationSinceUnixEpoch;
use torrust_info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::{AggregateActiveSwarmMetadata, SwarmMetadata};
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap, TrackerPolicy, peer};
use torrust_tracker_torrent_repository_benchmarking::repository::{Repository as _, RepositoryAsync as _};
use torrust_tracker_torrent_repository_benchmarking::{
    EntrySingle, TorrentsDashMapMutexStd, TorrentsRwLockStd, TorrentsRwLockStdMutexStd, TorrentsRwLockStdMutexTokio,
    TorrentsRwLockTokio, TorrentsRwLockTokioMutexStd, TorrentsRwLockTokioMutexTokio, TorrentsSkipMapMutexParkingLot,
    TorrentsSkipMapMutexStd, TorrentsSkipMapRwLockParkingLot,
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
    SkipMapMutexParkingLot(TorrentsSkipMapMutexParkingLot),
    SkipMapRwLockParkingLot(TorrentsSkipMapRwLockParkingLot),
    DashMapMutexStd(TorrentsDashMapMutexStd),
}

impl Repo {
    pub(crate) async fn upsert_peer(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
        opt_persistent_torrent: Option<NumberOfDownloads>,
    ) -> bool {
        match self {
            Self::RwLockStd(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent),
            Self::RwLockStdMutexStd(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent),
            Self::RwLockStdMutexTokio(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent).await,
            Self::RwLockTokio(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent).await,
            Self::RwLockTokioMutexStd(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent).await,
            Self::RwLockTokioMutexTokio(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent).await,
            Self::SkipMapMutexStd(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent),
            Self::SkipMapMutexParkingLot(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent),
            Self::SkipMapRwLockParkingLot(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent),
            Self::DashMapMutexStd(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent),
        }
    }

    pub(crate) async fn get_swarm_metadata(&self, info_hash: &InfoHash) -> Option<SwarmMetadata> {
        match self {
            Self::RwLockStd(repo) => repo.get_swarm_metadata(info_hash),
            Self::RwLockStdMutexStd(repo) => repo.get_swarm_metadata(info_hash),
            Self::RwLockStdMutexTokio(repo) => repo.get_swarm_metadata(info_hash).await,
            Self::RwLockTokio(repo) => repo.get_swarm_metadata(info_hash).await,
            Self::RwLockTokioMutexStd(repo) => repo.get_swarm_metadata(info_hash).await,
            Self::RwLockTokioMutexTokio(repo) => repo.get_swarm_metadata(info_hash).await,
            Self::SkipMapMutexStd(repo) => repo.get_swarm_metadata(info_hash),
            Self::SkipMapMutexParkingLot(repo) => repo.get_swarm_metadata(info_hash),
            Self::SkipMapRwLockParkingLot(repo) => repo.get_swarm_metadata(info_hash),
            Self::DashMapMutexStd(repo) => repo.get_swarm_metadata(info_hash),
        }
    }

    pub(crate) async fn get(&self, key: &InfoHash) -> Option<EntrySingle> {
        match self {
            Self::RwLockStd(repo) => repo.get(key),
            Self::RwLockStdMutexStd(repo) => Some(repo.get(key)?.lock().unwrap().clone()),
            Self::RwLockStdMutexTokio(repo) => Some(repo.get(key).await?.lock().await.clone()),
            Self::RwLockTokio(repo) => repo.get(key).await,
            Self::RwLockTokioMutexStd(repo) => Some(repo.get(key).await?.lock().unwrap().clone()),
            Self::RwLockTokioMutexTokio(repo) => Some(repo.get(key).await?.lock().await.clone()),
            Self::SkipMapMutexStd(repo) => Some(repo.get(key)?.lock().unwrap().clone()),
            Self::SkipMapMutexParkingLot(repo) => Some(repo.get(key)?.lock().clone()),
            Self::SkipMapRwLockParkingLot(repo) => Some(repo.get(key)?.read().clone()),
            Self::DashMapMutexStd(repo) => Some(repo.get(key)?.lock().unwrap().clone()),
        }
    }

    pub(crate) async fn get_metrics(&self) -> AggregateActiveSwarmMetadata {
        match self {
            Self::RwLockStd(repo) => repo.get_metrics(),
            Self::RwLockStdMutexStd(repo) => repo.get_metrics(),
            Self::RwLockStdMutexTokio(repo) => repo.get_metrics().await,
            Self::RwLockTokio(repo) => repo.get_metrics().await,
            Self::RwLockTokioMutexStd(repo) => repo.get_metrics().await,
            Self::RwLockTokioMutexTokio(repo) => repo.get_metrics().await,
            Self::SkipMapMutexStd(repo) => repo.get_metrics(),
            Self::SkipMapMutexParkingLot(repo) => repo.get_metrics(),
            Self::SkipMapRwLockParkingLot(repo) => repo.get_metrics(),
            Self::DashMapMutexStd(repo) => repo.get_metrics(),
        }
    }

    pub(crate) async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, EntrySingle)> {
        match self {
            Self::RwLockStd(repo) => repo.get_paginated(pagination),
            Self::RwLockStdMutexStd(repo) => repo
                .get_paginated(pagination)
                .iter()
                .map(|(i, t)| (*i, t.lock().expect("it should get a lock").clone()))
                .collect(),
            Self::RwLockStdMutexTokio(repo) => {
                let mut v: Vec<(InfoHash, EntrySingle)> = vec![];

                for (i, t) in repo.get_paginated(pagination).await {
                    v.push((i, t.lock().await.clone()));
                }
                v
            }
            Self::RwLockTokio(repo) => repo.get_paginated(pagination).await,
            Self::RwLockTokioMutexStd(repo) => repo
                .get_paginated(pagination)
                .await
                .iter()
                .map(|(i, t)| (*i, t.lock().expect("it should get a lock").clone()))
                .collect(),
            Self::RwLockTokioMutexTokio(repo) => {
                let mut v: Vec<(InfoHash, EntrySingle)> = vec![];

                for (i, t) in repo.get_paginated(pagination).await {
                    v.push((i, t.lock().await.clone()));
                }
                v
            }
            Self::SkipMapMutexStd(repo) => repo
                .get_paginated(pagination)
                .iter()
                .map(|(i, t)| (*i, t.lock().expect("it should get a lock").clone()))
                .collect(),
            Self::SkipMapMutexParkingLot(repo) => repo
                .get_paginated(pagination)
                .iter()
                .map(|(i, t)| (*i, t.lock().clone()))
                .collect(),
            Self::SkipMapRwLockParkingLot(repo) => repo
                .get_paginated(pagination)
                .iter()
                .map(|(i, t)| (*i, t.read().clone()))
                .collect(),
            Self::DashMapMutexStd(repo) => repo
                .get_paginated(pagination)
                .iter()
                .map(|(i, t)| (*i, t.lock().expect("it should get a lock").clone()))
                .collect(),
        }
    }

    pub(crate) async fn import_persistent(&self, persistent_torrents: &NumberOfDownloadsBTreeMap) {
        match self {
            Self::RwLockStd(repo) => repo.import_persistent(persistent_torrents),
            Self::RwLockStdMutexStd(repo) => repo.import_persistent(persistent_torrents),
            Self::RwLockStdMutexTokio(repo) => repo.import_persistent(persistent_torrents).await,
            Self::RwLockTokio(repo) => repo.import_persistent(persistent_torrents).await,
            Self::RwLockTokioMutexStd(repo) => repo.import_persistent(persistent_torrents).await,
            Self::RwLockTokioMutexTokio(repo) => repo.import_persistent(persistent_torrents).await,
            Self::SkipMapMutexStd(repo) => repo.import_persistent(persistent_torrents),
            Self::SkipMapMutexParkingLot(repo) => repo.import_persistent(persistent_torrents),
            Self::SkipMapRwLockParkingLot(repo) => repo.import_persistent(persistent_torrents),
            Self::DashMapMutexStd(repo) => repo.import_persistent(persistent_torrents),
        }
    }

    pub(crate) async fn remove(&self, key: &InfoHash) -> Option<EntrySingle> {
        match self {
            Self::RwLockStd(repo) => repo.remove(key),
            Self::RwLockStdMutexStd(repo) => Some(repo.remove(key)?.lock().unwrap().clone()),
            Self::RwLockStdMutexTokio(repo) => Some(repo.remove(key).await?.lock().await.clone()),
            Self::RwLockTokio(repo) => repo.remove(key).await,
            Self::RwLockTokioMutexStd(repo) => Some(repo.remove(key).await?.lock().unwrap().clone()),
            Self::RwLockTokioMutexTokio(repo) => Some(repo.remove(key).await?.lock().await.clone()),
            Self::SkipMapMutexStd(repo) => Some(repo.remove(key)?.lock().unwrap().clone()),
            Self::SkipMapMutexParkingLot(repo) => Some(repo.remove(key)?.lock().clone()),
            Self::SkipMapRwLockParkingLot(repo) => Some(repo.remove(key)?.write().clone()),
            Self::DashMapMutexStd(repo) => Some(repo.remove(key)?.lock().unwrap().clone()),
        }
    }

    pub(crate) async fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        match self {
            Self::RwLockStd(repo) => repo.remove_inactive_peers(current_cutoff),
            Self::RwLockStdMutexStd(repo) => repo.remove_inactive_peers(current_cutoff),
            Self::RwLockStdMutexTokio(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Self::RwLockTokio(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Self::RwLockTokioMutexStd(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Self::RwLockTokioMutexTokio(repo) => repo.remove_inactive_peers(current_cutoff).await,
            Self::SkipMapMutexStd(repo) => repo.remove_inactive_peers(current_cutoff),
            Self::SkipMapMutexParkingLot(repo) => repo.remove_inactive_peers(current_cutoff),
            Self::SkipMapRwLockParkingLot(repo) => repo.remove_inactive_peers(current_cutoff),
            Self::DashMapMutexStd(repo) => repo.remove_inactive_peers(current_cutoff),
        }
    }

    pub(crate) async fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        match self {
            Self::RwLockStd(repo) => repo.remove_peerless_torrents(policy),
            Self::RwLockStdMutexStd(repo) => repo.remove_peerless_torrents(policy),
            Self::RwLockStdMutexTokio(repo) => repo.remove_peerless_torrents(policy).await,
            Self::RwLockTokio(repo) => repo.remove_peerless_torrents(policy).await,
            Self::RwLockTokioMutexStd(repo) => repo.remove_peerless_torrents(policy).await,
            Self::RwLockTokioMutexTokio(repo) => repo.remove_peerless_torrents(policy).await,
            Self::SkipMapMutexStd(repo) => repo.remove_peerless_torrents(policy),
            Self::SkipMapMutexParkingLot(repo) => repo.remove_peerless_torrents(policy),
            Self::SkipMapRwLockParkingLot(repo) => repo.remove_peerless_torrents(policy),
            Self::DashMapMutexStd(repo) => repo.remove_peerless_torrents(policy),
        }
    }

    pub(crate) async fn insert(&self, info_hash: &InfoHash, torrent: EntrySingle) -> Option<EntrySingle> {
        match self {
            Self::RwLockStd(repo) => {
                repo.write().insert(*info_hash, torrent);
            }
            Self::RwLockStdMutexStd(repo) => {
                repo.write().insert(*info_hash, torrent.into());
            }
            Self::RwLockStdMutexTokio(repo) => {
                repo.write().insert(*info_hash, torrent.into());
            }
            Self::RwLockTokio(repo) => {
                repo.write().await.insert(*info_hash, torrent);
            }
            Self::RwLockTokioMutexStd(repo) => {
                repo.write().await.insert(*info_hash, torrent.into());
            }
            Self::RwLockTokioMutexTokio(repo) => {
                repo.write().await.insert(*info_hash, torrent.into());
            }
            Self::SkipMapMutexStd(repo) => {
                repo.torrents.insert(*info_hash, torrent.into());
            }
            Self::SkipMapMutexParkingLot(repo) => {
                repo.torrents.insert(*info_hash, torrent.into());
            }
            Self::SkipMapRwLockParkingLot(repo) => {
                repo.torrents.insert(*info_hash, torrent.into());
            }
            Self::DashMapMutexStd(repo) => {
                repo.torrents.insert(*info_hash, torrent.into());
            }
        }
        self.get(info_hash).await
    }
}
