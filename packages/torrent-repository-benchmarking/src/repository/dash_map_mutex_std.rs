use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use dashmap::DashMap;
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::{AggregateActiveSwarmMetadata, SwarmMetadata};
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::Repository;
use crate::entry::peer_list::PeerList;
use crate::entry::{Entry, EntrySync};
use crate::{EntryMutexStd, EntrySingle};

#[derive(Default, Debug)]
pub struct XacrimonDashMap<T> {
    pub torrents: DashMap<InfoHash, T>,
}

impl Repository<EntryMutexStd> for XacrimonDashMap<EntryMutexStd>
where
    EntryMutexStd: EntrySync,
    EntrySingle: Entry,
{
    fn upsert_peer(&self, info_hash: &InfoHash, peer: &peer::Peer, _opt_persistent_torrent: Option<NumberOfDownloads>) -> bool {
        // todo: load persistent torrent data if provided

        if let Some(entry) = self.torrents.get(info_hash) {
            entry.upsert_peer(peer)
        } else {
            let _unused = self.torrents.insert(*info_hash, Arc::default());
            if let Some(entry) = self.torrents.get(info_hash) {
                entry.upsert_peer(peer)
            } else {
                false
            }
        }
    }

    fn get_swarm_metadata(&self, info_hash: &InfoHash) -> Option<SwarmMetadata> {
        self.torrents.get(info_hash).map(|entry| entry.value().get_swarm_metadata())
    }

    fn get(&self, key: &InfoHash) -> Option<EntryMutexStd> {
        let maybe_entry = self.torrents.get(key);
        maybe_entry.map(|entry| entry.clone())
    }

    fn get_metrics(&self) -> AggregateActiveSwarmMetadata {
        let mut metrics = AggregateActiveSwarmMetadata::default();

        for entry in &self.torrents {
            let stats = entry.value().lock().expect("it should get a lock").get_swarm_metadata();
            metrics.total_complete += u64::from(stats.complete);
            metrics.total_downloaded += u64::from(stats.downloaded);
            metrics.total_incomplete += u64::from(stats.incomplete);
            metrics.total_torrents += 1;
        }

        metrics
    }

    fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, EntryMutexStd)> {
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

    fn import_persistent(&self, persistent_torrents: &NumberOfDownloadsBTreeMap) {
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

            self.torrents.insert(*info_hash, entry);
        }
    }

    fn remove(&self, key: &InfoHash) -> Option<EntryMutexStd> {
        self.torrents.remove(key).map(|(_key, value)| value.clone())
    }

    fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        for entry in &self.torrents {
            entry.value().remove_inactive_peers(current_cutoff);
        }
    }

    fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        self.torrents.retain(|_, entry| entry.meets_retaining_policy(policy));
    }
}
