//! In-memory torrents repository.
use std::sync::Arc;

use torrust_clock::DurationSinceUnixEpoch;
use torrust_info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::{AggregateActiveSwarmMetadata, SwarmMetadata};
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsPerInfoHash, TrackerPolicy, peer};
use torrust_tracker_swarm_coordination_registry::swarm::registry::Error;
use torrust_tracker_swarm_coordination_registry::{CoordinatorHandle, Registry};

/// In-memory repository for torrent entries.
///
/// This repository manages the torrent entries and their associated peer lists
/// in memory. It is built on top of a high-performance data structure (the
/// production implementation) and provides methods to update, query, and remove
/// torrent entries as well as to import persisted data.
///
/// Multiple implementations were considered, and the chosen implementation is
/// used in production. Other implementations are kept for reference.
#[derive(Default)]
pub struct InMemoryTorrentRepository {
    /// The underlying in-memory data structure that stores swarms data.
    swarms: Arc<Registry>,
}

impl InMemoryTorrentRepository {
    #[must_use]
    pub const fn new(swarms: Arc<Registry>) -> Self {
        Self { swarms }
    }

    /// Inserts or updates a peer in the torrent entry corresponding to the
    /// given infohash.
    ///
    /// If the torrent entry already exists, the peer is added to its peer list;
    /// otherwise, a new torrent entry is created.
    ///
    /// # Arguments
    ///
    /// * `info_hash` - The unique identifier of the torrent.
    /// * `peer` - The peer to insert or update in the torrent entry.
    ///
    /// # Returns
    ///
    /// `true` if the peer stats were updated.
    ///
    /// # Errors
    ///
    /// Propagates the swarm registry [`Error`].
    pub async fn handle_announcement(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
        opt_persistent_torrent: Option<NumberOfDownloads>,
    ) -> Result<(), Error> {
        self.swarms.handle_announcement(info_hash, peer, opt_persistent_torrent).await
    }

    /// Removes inactive peers from all torrent entries.
    ///
    /// A peer is considered inactive if its last update timestamp is older than
    /// the provided cutoff time.
    ///
    /// # Arguments
    ///
    /// * `current_cutoff` - The cutoff timestamp; peers not updated since this
    ///   time will be removed.
    ///
    /// # Errors
    ///
    /// Propagates the swarm registry [`Error`].
    pub(crate) async fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) -> Result<usize, Error> {
        self.swarms.remove_inactive_peers(current_cutoff).await
    }

    /// Removes torrent entries that have no active peers.
    ///
    /// Depending on the tracker policy, torrents without any peers may be
    /// removed to conserve memory.
    ///
    /// # Arguments
    ///
    /// * `policy` - The tracker policy containing the configuration for
    ///   removing peerless torrents.
    ///
    /// # Errors
    ///
    /// Propagates the swarm registry [`Error`].
    pub(crate) async fn remove_peerless_torrents(&self, policy: &TrackerPolicy) -> Result<u64, Error> {
        self.swarms.remove_peerless_torrents(policy).await
    }

    /// Retrieves a torrent entry by its infohash.
    ///
    /// # Arguments
    ///
    /// * `key` - The info hash of the torrent.
    ///
    /// # Returns
    ///
    /// An `Option` containing the torrent entry if found.
    #[must_use]
    pub(crate) fn get(&self, key: &InfoHash) -> Option<CoordinatorHandle> {
        self.swarms.get(key)
    }

    /// Retrieves a paginated list of torrent entries.
    ///
    /// This method returns a vector of tuples, each containing an infohash and
    /// its associated torrent entry. The pagination parameters (offset and limit)
    /// can be used to control the size of the result set.
    ///
    /// # Arguments
    ///
    /// * `pagination` - An optional reference to a `Pagination` object.
    ///
    /// # Returns
    ///
    /// A vector of `(InfoHash, TorrentEntry)` tuples.
    #[must_use]
    pub(crate) fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, CoordinatorHandle)> {
        self.swarms.get_paginated(pagination)
    }

    /// Retrieves swarm metadata for a given torrent.
    ///
    /// This method returns the swarm metadata (aggregate information such as
    /// peer counts) for the torrent specified by the infohash, or `None` if the
    /// torrent is not in memory.
    ///
    /// # Errors
    ///
    /// Propagates the swarm registry [`Error`].
    pub(crate) async fn get_swarm_metadata(&self, info_hash: &InfoHash) -> Result<Option<SwarmMetadata>, Error> {
        self.swarms.get_swarm_metadata(info_hash).await
    }

    /// Retrieves swarm metadata for a given torrent.
    ///
    /// This method returns the swarm metadata (aggregate information such as
    /// peer counts) for the torrent specified by the infohash. If the torrent
    /// entry is not found, a zeroed metadata struct is returned.
    ///
    /// # Arguments
    ///
    /// * `info_hash` - The info hash of the torrent.
    ///
    /// # Returns
    ///
    /// A `SwarmMetadata` struct containing the aggregated torrent data.
    ///
    /// # Errors
    ///
    /// Propagates the swarm registry [`Error`].
    pub(crate) async fn get_swarm_metadata_or_default(&self, info_hash: &InfoHash) -> Result<SwarmMetadata, Error> {
        self.swarms.get_swarm_metadata_or_default(info_hash).await
    }

    /// Retrieves torrent peers for a given torrent and client, excluding the
    /// requesting client.
    ///
    /// This method filters out the client making the request (based on its
    /// network address) and returns up to `limit` peers.
    ///
    /// # Arguments
    ///
    /// * `info_hash` - The info hash of the torrent.
    /// * `peer` - The client peer that should be excluded from the returned list.
    /// * `limit` - The maximum number of peers to return.
    ///
    /// # Returns
    ///
    /// A vector of peers (wrapped in `Arc`) representing the active peers for
    /// the torrent, excluding the requesting client.
    ///
    /// # Errors
    ///
    /// Propagates the swarm registry [`Error`].
    pub(crate) async fn get_peers_for(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
        limit: usize,
    ) -> Result<Vec<Arc<peer::Peer>>, Error> {
        self.swarms.get_peers_peers_excluding(info_hash, peer, limit).await
    }

    /// Retrieves the list of peers for a given torrent.
    ///
    /// This method returns up to `max_peers` peers for the torrent
    /// specified by the info-hash.
    ///
    /// # Arguments
    ///
    /// * `info_hash` - The info hash of the torrent.
    /// * `max_peers` - The maximum number of peers to return.
    ///
    /// # Returns
    ///
    /// A vector of peers (wrapped in `Arc`) representing the active peers for
    /// the torrent.
    ///
    /// # Errors
    ///
    /// Propagates the swarm registry [`Error`].
    pub async fn get_torrent_peers(&self, info_hash: &InfoHash, max_peers: usize) -> Result<Vec<Arc<peer::Peer>>, Error> {
        self.swarms.get_swarm_peers(info_hash, max_peers).await
    }

    /// Calculates and returns overall torrent metrics.
    ///
    /// The returned [`AggregateActiveSwarmMetadata`] contains aggregate data such as
    /// the total number of torrents, total complete (seeders), incomplete
    /// (leechers), and downloaded counts.
    ///
    /// # Returns
    ///
    /// A [`AggregateActiveSwarmMetadata`] struct with the aggregated metrics.
    ///
    /// # Errors
    ///
    /// Propagates the swarm registry [`Error`].
    pub async fn get_aggregate_swarm_metadata(&self) -> Result<AggregateActiveSwarmMetadata, Error> {
        self.swarms.get_aggregate_swarm_metadata().await
    }

    /// Counts the number of peerless torrents in the repository.
    ///
    /// # Errors
    ///
    /// Propagates the swarm registry [`Error`].
    pub async fn count_peerless_torrents(&self) -> Result<usize, Error> {
        self.swarms.count_peerless_torrents().await
    }

    /// Counts the number of peers in the repository.
    ///
    /// # Errors
    ///
    /// Propagates the swarm registry [`Error`].
    pub async fn count_peers(&self) -> Result<usize, Error> {
        self.swarms.count_peers().await
    }

    /// Imports persistent torrent data into the in-memory repository.
    ///
    /// This method takes a set of persisted torrent entries (e.g., from a database)
    /// and imports them into the in-memory repository for immediate access.
    ///
    /// # Arguments
    ///
    /// * `persistent_torrents` - A reference to the persisted torrent data.
    pub fn import_persistent(&self, persistent_torrents: &NumberOfDownloadsPerInfoHash) {
        self.swarms.import_persistent(persistent_torrents);
    }

    /// Checks if the repository contains a torrent entry for the given infohash.
    #[must_use]
    pub fn contains(&self, info_hash: &InfoHash) -> bool {
        self.swarms.contains(info_hash)
    }
}
