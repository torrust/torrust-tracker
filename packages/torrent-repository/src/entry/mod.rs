use std::fmt::Debug;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};

pub mod mutex_std;
pub mod mutex_tokio;
pub mod single;

pub trait Entry {
    /// It returns the swarm metadata (statistics) as a struct:
    ///
    /// `(seeders, completed, leechers)`
    fn get_stats(&self) -> SwarmMetadata;

    /// Returns True if Still a Valid Entry according to the Tracker Policy
    fn is_not_zombie(&self, policy: &TrackerPolicy) -> bool;

    /// Returns True if the Peers is Empty
    fn peers_is_empty(&self) -> bool;

    /// Returns the number of Peers
    fn get_peers_len(&self) -> usize;

    /// Get all swarm peers, optionally limiting the result.
    fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>>;

    /// It returns the list of peers for a given peer client, optionally limiting the
    /// result.
    ///
    /// It filters out the input peer, typically because we want to return this
    /// list of peers to that client peer.
    fn get_peers_for_peer(&self, client: &peer::Peer, limit: Option<usize>) -> Vec<Arc<peer::Peer>>;

    /// It updates a peer and returns true if the number of complete downloads have increased.
    ///
    /// The number of peers that have complete downloading is synchronously updated when peers are updated.
    /// That's the total torrent downloads counter.
    fn insert_or_update_peer(&mut self, peer: &peer::Peer) -> bool;

    // It preforms a combined operation of `insert_or_update_peer` and `get_stats`.
    fn insert_or_update_peer_and_get_stats(&mut self, peer: &peer::Peer) -> (bool, SwarmMetadata);

    /// It removes peer from the swarm that have not been updated for more than `current_cutoff` seconds
    fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch);
}

#[allow(clippy::module_name_repetitions)]
pub trait EntrySync {
    fn get_stats(&self) -> SwarmMetadata;
    fn is_not_zombie(&self, policy: &TrackerPolicy) -> bool;
    fn peers_is_empty(&self) -> bool;
    fn get_peers_len(&self) -> usize;
    fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>>;
    fn get_peers_for_peer(&self, client: &peer::Peer, limit: Option<usize>) -> Vec<Arc<peer::Peer>>;
    fn insert_or_update_peer(&self, peer: &peer::Peer) -> bool;
    fn insert_or_update_peer_and_get_stats(&self, peer: &peer::Peer) -> (bool, SwarmMetadata);
    fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch);
}

#[allow(clippy::module_name_repetitions)]
pub trait EntryAsync {
    fn get_stats(self) -> impl std::future::Future<Output = SwarmMetadata> + Send;

    #[allow(clippy::wrong_self_convention)]
    fn is_not_zombie(self, policy: &TrackerPolicy) -> impl std::future::Future<Output = bool> + Send;
    fn peers_is_empty(self) -> impl std::future::Future<Output = bool> + Send;
    fn get_peers_len(self) -> impl std::future::Future<Output = usize> + Send;
    fn get_peers(self, limit: Option<usize>) -> impl std::future::Future<Output = Vec<Arc<peer::Peer>>> + Send;
    fn get_peers_for_peer(
        self,
        client: &peer::Peer,
        limit: Option<usize>,
    ) -> impl std::future::Future<Output = Vec<Arc<peer::Peer>>> + Send;
    fn insert_or_update_peer(self, peer: &peer::Peer) -> impl std::future::Future<Output = bool> + Send;
    fn insert_or_update_peer_and_get_stats(
        self,
        peer: &peer::Peer,
    ) -> impl std::future::Future<Output = (bool, SwarmMetadata)> + std::marker::Send;
    fn remove_inactive_peers(self, current_cutoff: DurationSinceUnixEpoch) -> impl std::future::Future<Output = ()> + Send;
}

/// A data structure containing all the information about a torrent in the tracker.
///
/// This is the tracker entry for a given torrent and contains the swarm data,
/// that's the list of all the peers trying to download the same torrent.
/// The tracker keeps one entry like this for every torrent.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Torrent {
    /// The swarm: a network of peers that are all trying to download the torrent associated to this entry
    #[serde(skip)]
    pub(crate) peers: std::collections::BTreeMap<peer::Id, Arc<peer::Peer>>,
    /// The number of peers that have ever completed downloading the torrent associated to this entry
    pub(crate) completed: u32,
}
