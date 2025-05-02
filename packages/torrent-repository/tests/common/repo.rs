use std::sync::{Arc, Mutex};

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::{AggregateSwarmMetadata, SwarmMetadata};
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrent, PersistentTorrents};
use torrust_tracker_torrent_repository::entry::torrent::TrackedTorrent;
use torrust_tracker_torrent_repository::TorrentRepository;

#[derive(Debug)]
pub(crate) enum Repo {
    SkipMapMutexStd(TorrentRepository),
}

impl Repo {
    pub(crate) fn upsert_peer(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
        opt_persistent_torrent: Option<PersistentTorrent>,
    ) -> bool {
        match self {
            Repo::SkipMapMutexStd(repo) => repo.upsert_peer(info_hash, peer, opt_persistent_torrent),
        }
    }

    pub(crate) fn get_swarm_metadata(&self, info_hash: &InfoHash) -> Option<SwarmMetadata> {
        match self {
            Repo::SkipMapMutexStd(repo) => repo.get_swarm_metadata(info_hash),
        }
    }

    pub(crate) fn get(&self, key: &InfoHash) -> Option<TrackedTorrent> {
        match self {
            Repo::SkipMapMutexStd(repo) => Some(repo.get(key)?.lock().unwrap().clone()),
        }
    }

    pub(crate) fn get_metrics(&self) -> AggregateSwarmMetadata {
        match self {
            Repo::SkipMapMutexStd(repo) => repo.get_aggregate_swarm_metadata(),
        }
    }

    pub(crate) fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, TrackedTorrent)> {
        match self {
            Repo::SkipMapMutexStd(repo) => repo
                .get_paginated(pagination)
                .iter()
                .map(|(i, t)| (*i, t.lock().expect("it should get a lock").clone()))
                .collect(),
        }
    }

    pub(crate) fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        match self {
            Repo::SkipMapMutexStd(repo) => repo.import_persistent(persistent_torrents),
        }
    }

    pub(crate) fn remove(&self, key: &InfoHash) -> Option<TrackedTorrent> {
        match self {
            Repo::SkipMapMutexStd(repo) => Some(repo.remove(key)?.lock().unwrap().clone()),
        }
    }

    pub(crate) fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        match self {
            Repo::SkipMapMutexStd(repo) => repo.remove_inactive_peers(current_cutoff),
        }
    }

    pub(crate) fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        match self {
            Repo::SkipMapMutexStd(repo) => repo.remove_peerless_torrents(policy),
        }
    }

    pub(crate) fn insert(&self, info_hash: &InfoHash, torrent: TrackedTorrent) -> Option<TrackedTorrent> {
        match self {
            Repo::SkipMapMutexStd(repo) => {
                repo.torrents.insert(*info_hash, Arc::new(Mutex::new(torrent)));
            }
        }
        self.get(info_hash)
    }
}
