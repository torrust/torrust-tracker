use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

use aquatic_udp_protocol::AnnounceEvent;
use serde::{Deserialize, Serialize};

use super::SwarmMetadata;
use crate::core::peer::{self, ReadInfo as _};
use crate::core::TrackerPolicy;
use crate::shared::clock::{Current, TimeNow};

/// A data structure containing all the information about a torrent in the tracker.
///
/// This is the tracker entry for a given torrent and contains the swarm data,
/// that's the list of all the peers trying to download the same torrent.
/// The tracker keeps one entry like this for every torrent.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Entry {
    /// The swarm: a network of peers that are all trying to download the torrent associated to this entry
    #[serde(skip)]
    pub(crate) peers: std::collections::BTreeMap<peer::Id, Arc<peer::Peer>>,
    /// The number of peers that have ever completed downloading the torrent associated to this entry
    pub(crate) completed: u32,
}
pub type Single = Entry;
pub type MutexStd = Arc<std::sync::Mutex<Entry>>;
pub type MutexTokio = Arc<tokio::sync::Mutex<Entry>>;

pub trait ReadInfo {
    /// It returns the swarm metadata (statistics) as a struct:
    ///
    /// `(seeders, completed, leechers)`
    fn get_stats(&self) -> SwarmMetadata;

    /// Returns True if Still a Valid Entry according to the Tracker Policy
    fn is_not_zombie(&self, policy: &TrackerPolicy) -> bool;

    /// Returns True if the Peers is Empty
    fn peers_is_empty(&self) -> bool;
}

/// Same as [`ReadInfo`], but async.
pub trait ReadInfoAsync {
    /// It returns the swarm metadata (statistics) as a struct:
    ///
    /// `(seeders, completed, leechers)`
    fn get_stats(&self) -> impl std::future::Future<Output = SwarmMetadata> + Send;

    /// Returns True if Still a Valid Entry according to the Tracker Policy
    fn is_not_zombie(&self, policy: &TrackerPolicy) -> impl std::future::Future<Output = bool> + Send;

    /// Returns True if the Peers is Empty
    fn peers_is_empty(&self) -> impl std::future::Future<Output = bool> + Send;
}

pub trait ReadPeers {
    /// Get all swarm peers, optionally limiting the result.
    fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>>;

    /// It returns the list of peers for a given peer client, optionally limiting the
    /// result.
    ///
    /// It filters out the input peer, typically because we want to return this
    /// list of peers to that client peer.
    fn get_peers_for_peer(&self, client: &peer::Peer, limit: Option<usize>) -> Vec<Arc<peer::Peer>>;
}

/// Same as [`ReadPeers`], but async.
pub trait ReadPeersAsync {
    fn get_peers(&self, limit: Option<usize>) -> impl std::future::Future<Output = Vec<Arc<peer::Peer>>> + Send;

    fn get_peers_for_peer(
        &self,
        client: &peer::Peer,
        limit: Option<usize>,
    ) -> impl std::future::Future<Output = Vec<Arc<peer::Peer>>> + Send;
}

pub trait Update {
    /// It updates a peer and returns true if the number of complete downloads have increased.
    ///
    /// The number of peers that have complete downloading is synchronously updated when peers are updated.
    /// That's the total torrent downloads counter.
    fn insert_or_update_peer(&mut self, peer: &peer::Peer) -> bool;

    // It preforms a combined operation of `insert_or_update_peer` and `get_stats`.
    fn insert_or_update_peer_and_get_stats(&mut self, peer: &peer::Peer) -> (bool, SwarmMetadata);

    /// It removes peer from the swarm that have not been updated for more than `max_peer_timeout` seconds
    fn remove_inactive_peers(&mut self, max_peer_timeout: u32);
}

/// Same as [`Update`], except not `mut`.
pub trait UpdateSync {
    fn insert_or_update_peer(&self, peer: &peer::Peer) -> bool;
    fn insert_or_update_peer_and_get_stats(&self, peer: &peer::Peer) -> (bool, SwarmMetadata);
    fn remove_inactive_peers(&self, max_peer_timeout: u32);
}

/// Same as [`Update`], except not `mut` and async.
pub trait UpdateAsync {
    fn insert_or_update_peer(&self, peer: &peer::Peer) -> impl std::future::Future<Output = bool> + Send;

    fn insert_or_update_peer_and_get_stats(
        &self,
        peer: &peer::Peer,
    ) -> impl std::future::Future<Output = (bool, SwarmMetadata)> + std::marker::Send;

    fn remove_inactive_peers(&self, max_peer_timeout: u32) -> impl std::future::Future<Output = ()> + Send;
}

impl ReadInfo for Single {
    #[allow(clippy::cast_possible_truncation)]
    fn get_stats(&self) -> SwarmMetadata {
        let complete: u32 = self.peers.values().filter(|peer| peer.is_seeder()).count() as u32;
        let incomplete: u32 = self.peers.len() as u32 - complete;

        SwarmMetadata {
            downloaded: self.completed,
            complete,
            incomplete,
        }
    }

    fn is_not_zombie(&self, policy: &TrackerPolicy) -> bool {
        if policy.persistent_torrent_completed_stat && self.completed > 0 {
            return true;
        }

        if policy.remove_peerless_torrents && self.peers.is_empty() {
            return false;
        }

        true
    }

    fn peers_is_empty(&self) -> bool {
        self.peers.is_empty()
    }
}

impl ReadInfo for MutexStd {
    fn get_stats(&self) -> SwarmMetadata {
        self.lock().expect("it should get a lock").get_stats()
    }

    fn is_not_zombie(&self, policy: &TrackerPolicy) -> bool {
        self.lock().expect("it should get a lock").is_not_zombie(policy)
    }

    fn peers_is_empty(&self) -> bool {
        self.lock().expect("it should get a lock").peers_is_empty()
    }
}

impl ReadInfoAsync for MutexTokio {
    async fn get_stats(&self) -> SwarmMetadata {
        self.lock().await.get_stats()
    }

    async fn is_not_zombie(&self, policy: &TrackerPolicy) -> bool {
        self.lock().await.is_not_zombie(policy)
    }

    async fn peers_is_empty(&self) -> bool {
        self.lock().await.peers_is_empty()
    }
}

impl ReadPeers for Single {
    fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match limit {
            Some(limit) => self.peers.values().take(limit).cloned().collect(),
            None => self.peers.values().cloned().collect(),
        }
    }

    fn get_peers_for_peer(&self, client: &peer::Peer, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match limit {
            Some(limit) => self
                .peers
                .values()
                // Take peers which are not the client peer
                .filter(|peer| peer.get_address() != client.get_address())
                // Limit the number of peers on the result
                .take(limit)
                .cloned()
                .collect(),
            None => self
                .peers
                .values()
                // Take peers which are not the client peer
                .filter(|peer| peer.get_address() != client.get_address())
                .cloned()
                .collect(),
        }
    }
}

impl ReadPeers for MutexStd {
    fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().expect("it should get lock").get_peers(limit)
    }

    fn get_peers_for_peer(&self, client: &peer::Peer, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().expect("it should get lock").get_peers_for_peer(client, limit)
    }
}

impl ReadPeersAsync for MutexTokio {
    async fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().await.get_peers(limit)
    }

    async fn get_peers_for_peer(&self, client: &peer::Peer, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().await.get_peers_for_peer(client, limit)
    }
}

impl Update for Single {
    fn insert_or_update_peer(&mut self, peer: &peer::Peer) -> bool {
        let mut did_torrent_stats_change: bool = false;

        match peer.get_event() {
            AnnounceEvent::Stopped => {
                drop(self.peers.remove(&peer.get_id()));
            }
            AnnounceEvent::Completed => {
                let peer_old = self.peers.insert(peer.get_id(), Arc::new(*peer));
                // Don't count if peer was not previously known and not already completed.
                if peer_old.is_some_and(|p| p.event != AnnounceEvent::Completed) {
                    self.completed += 1;
                    did_torrent_stats_change = true;
                }
            }
            _ => {
                drop(self.peers.insert(peer.get_id(), Arc::new(*peer)));
            }
        }

        did_torrent_stats_change
    }

    fn insert_or_update_peer_and_get_stats(&mut self, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let changed = self.insert_or_update_peer(peer);
        let stats = self.get_stats();
        (changed, stats)
    }

    fn remove_inactive_peers(&mut self, max_peer_timeout: u32) {
        let current_cutoff = Current::sub(&Duration::from_secs(u64::from(max_peer_timeout))).unwrap_or_default();
        self.peers.retain(|_, peer| peer.get_updated() > current_cutoff);
    }
}

impl UpdateSync for MutexStd {
    fn insert_or_update_peer(&self, peer: &peer::Peer) -> bool {
        self.lock().expect("it should lock the entry").insert_or_update_peer(peer)
    }

    fn insert_or_update_peer_and_get_stats(&self, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        self.lock()
            .expect("it should lock the entry")
            .insert_or_update_peer_and_get_stats(peer)
    }

    fn remove_inactive_peers(&self, max_peer_timeout: u32) {
        self.lock()
            .expect("it should lock the entry")
            .remove_inactive_peers(max_peer_timeout);
    }
}

impl UpdateAsync for MutexTokio {
    async fn insert_or_update_peer(&self, peer: &peer::Peer) -> bool {
        self.lock().await.insert_or_update_peer(peer)
    }

    async fn insert_or_update_peer_and_get_stats(&self, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        self.lock().await.insert_or_update_peer_and_get_stats(peer)
    }

    async fn remove_inactive_peers(&self, max_peer_timeout: u32) {
        self.lock().await.remove_inactive_peers(max_peer_timeout);
    }
}
