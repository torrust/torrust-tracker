use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

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
    fn get_swarm_metadata(&self) -> SwarmMetadata;

    /// Returns True if Still a Valid Entry according to the Tracker Policy
    fn is_good(&self, policy: &TrackerPolicy) -> bool;

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
    fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>>;

    /// It updates a peer and returns true if the number of complete downloads have increased.
    ///
    /// The number of peers that have complete downloading is synchronously updated when peers are updated.
    /// That's the total torrent downloads counter.
    fn upsert_peer(&mut self, peer: &peer::Peer) -> bool;

    /// It removes peer from the swarm that have not been updated for more than `current_cutoff` seconds
    fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch);
}

#[allow(clippy::module_name_repetitions)]
pub trait EntrySync {
    fn get_swarm_metadata(&self) -> SwarmMetadata;
    fn is_good(&self, policy: &TrackerPolicy) -> bool;
    fn peers_is_empty(&self) -> bool;
    fn get_peers_len(&self) -> usize;
    fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>>;
    fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>>;
    fn upsert_peer(&self, peer: &peer::Peer) -> bool;
    fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch);
}

#[allow(clippy::module_name_repetitions)]
pub trait EntryAsync {
    fn get_swarm_metadata(&self) -> impl std::future::Future<Output = SwarmMetadata> + Send;
    fn check_good(self, policy: &TrackerPolicy) -> impl std::future::Future<Output = bool> + Send;
    fn peers_is_empty(&self) -> impl std::future::Future<Output = bool> + Send;
    fn get_peers_len(&self) -> impl std::future::Future<Output = usize> + Send;
    fn get_peers(&self, limit: Option<usize>) -> impl std::future::Future<Output = Vec<Arc<peer::Peer>>> + Send;
    fn get_peers_for_client(
        &self,
        client: &SocketAddr,
        limit: Option<usize>,
    ) -> impl std::future::Future<Output = Vec<Arc<peer::Peer>>> + Send;
    fn upsert_peer(self, peer: &peer::Peer) -> impl std::future::Future<Output = bool> + Send;
    fn remove_inactive_peers(self, current_cutoff: DurationSinceUnixEpoch) -> impl std::future::Future<Output = ()> + Send;
}

/// A data structure containing all the information about a torrent in the tracker.
///
/// This is the tracker entry for a given torrent and contains the swarm data,
/// that's the list of all the peers trying to download the same torrent.
/// The tracker keeps one entry like this for every torrent.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Torrent<T> {
    /// The swarm: a network of peers that are all trying to download the torrent associated to this entry
    // #[serde(skip)]
    pub(crate) peers: T,
    /// The number of peers that have ever completed downloading the torrent associated to this entry
    pub(crate) downloaded: u32,
}
