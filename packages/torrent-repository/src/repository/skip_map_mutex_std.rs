use bittorrent_primitives::info_hash::InfoHash;
use crossbeam_skiplist::SkipMap;
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::{AggregateSwarmMetadata, SwarmMetadata};
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrent, PersistentTorrents};

use crate::entry::peer_list::PeerList;
use crate::{EntryMutexStd, EntrySingle};

#[derive(Default, Debug)]
pub struct TorrentsSkipMapMutexStd {
    pub torrents: SkipMap<InfoHash, EntryMutexStd>,
}

impl TorrentsSkipMapMutexStd {
    /// Upsert a peer into the swarm of a torrent.
    ///
    /// Optionally, it can also preset the number of downloads of the torrent
    /// only if it's the first time the torrent is being inserted.
    ///
    /// # Arguments
    ///
    /// * `info_hash` - The info hash of the torrent.
    /// * `peer` - The peer to upsert.
    /// * `opt_persistent_torrent` - The optional persisted data about a torrent
    ///   (number of downloads for the torrent).
    ///
    /// # Returns
    ///
    /// Returns `true` if the number of downloads was increased because the peer
    /// completed the download.
    ///
    /// # Panics
    ///
    /// This function panics if the lock for the entry cannot be obtained.
    pub fn upsert_peer(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
        opt_persistent_torrent: Option<PersistentTorrent>,
    ) -> bool {
        if let Some(existing_entry) = self.torrents.get(info_hash) {
            existing_entry
                .value()
                .lock()
                .expect("can't acquire lock for torrent entry")
                .upsert_peer(peer)
        } else {
            let new_entry = if let Some(number_of_downloads) = opt_persistent_torrent {
                EntryMutexStd::new(
                    EntrySingle {
                        swarm: PeerList::default(),
                        downloaded: number_of_downloads,
                    }
                    .into(),
                )
            } else {
                EntryMutexStd::default()
            };

            let inserted_entry = self.torrents.get_or_insert(*info_hash, new_entry);

            let number_of_downloads_increased = inserted_entry
                .value()
                .lock()
                .expect("can't acquire lock for torrent entry")
                .upsert_peer(peer);

            number_of_downloads_increased
        }
    }

    /// # Panics
    ///
    /// This function panics if the lock for the entry cannot be obtained.
    pub fn get_swarm_metadata(&self, info_hash: &InfoHash) -> Option<SwarmMetadata> {
        self.torrents.get(info_hash).map(|entry| {
            entry
                .value()
                .lock()
                .expect("can't acquire lock for torrent entry")
                .get_swarm_metadata()
        })
    }

    pub fn get(&self, key: &InfoHash) -> Option<EntryMutexStd> {
        let maybe_entry = self.torrents.get(key);
        maybe_entry.map(|entry| entry.value().clone())
    }

    /// # Panics
    ///
    /// This function panics if the lock for the entry cannot be obtained.
    pub fn get_metrics(&self) -> AggregateSwarmMetadata {
        let mut metrics = AggregateSwarmMetadata::default();

        for entry in &self.torrents {
            let stats = entry.value().lock().expect("it should get a lock").get_swarm_metadata();
            metrics.total_complete += u64::from(stats.complete);
            metrics.total_downloaded += u64::from(stats.downloaded);
            metrics.total_incomplete += u64::from(stats.incomplete);
            metrics.total_torrents += 1;
        }

        metrics
    }

    pub fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, EntryMutexStd)> {
        match pagination {
            Some(pagination) => self
                .torrents
                .iter()
                .skip(pagination.offset as usize)
                .take(pagination.limit as usize)
                .map(|entry| (*entry.key(), entry.value().clone()))
                .collect(),
            None => self
                .torrents
                .iter()
                .map(|entry| (*entry.key(), entry.value().clone()))
                .collect(),
        }
    }

    pub fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        for (info_hash, completed) in persistent_torrents {
            if self.torrents.contains_key(info_hash) {
                continue;
            }

            let entry = EntryMutexStd::new(
                EntrySingle {
                    swarm: PeerList::default(),
                    downloaded: *completed,
                }
                .into(),
            );

            // Since SkipMap is lock-free the torrent could have been inserted
            // after checking if it exists.
            self.torrents.get_or_insert(*info_hash, entry);
        }
    }

    pub fn remove(&self, key: &InfoHash) -> Option<EntryMutexStd> {
        self.torrents.remove(key).map(|entry| entry.value().clone())
    }

    /// # Panics
    ///
    /// This function panics if the lock for the entry cannot be obtained.
    pub fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        for entry in &self.torrents {
            entry
                .value()
                .lock()
                .expect("can't acquire lock for torrent entry")
                .remove_inactive_peers(current_cutoff);
        }
    }

    /// # Panics
    ///
    /// This function panics if the lock for the entry cannot be obtained.
    pub fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        for entry in &self.torrents {
            if entry
                .value()
                .lock()
                .expect("can't acquire lock for torrent entry")
                .meets_retaining_policy(policy)
            {
                continue;
            }

            entry.remove();
        }
    }
}
