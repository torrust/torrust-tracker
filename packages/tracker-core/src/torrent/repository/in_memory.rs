//! In-memory torrents repository.
use std::cmp::max;
use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_configuration::{TrackerPolicy, TORRENT_PEERS_LIMIT};
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::{AggregateSwarmMetadata, SwarmMetadata};
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrent, PersistentTorrents};
use torrust_tracker_torrent_repository::{SwarmHandle, TorrentRepository};

/// In-memory repository for torrent entries.
///
/// This repository manages the torrent entries and their associated peer lists
/// in memory. It is built on top of a high-performance data structure (the
/// production implementation) and provides methods to update, query, and remove
/// torrent entries as well as to import persisted data.
///
/// Multiple implementations were considered, and the chosen implementation is
/// used in production. Other implementations are kept for reference.
#[derive(Debug, Default)]
pub struct InMemoryTorrentRepository {
    /// The underlying in-memory data structure that stores torrent entries.
    torrents: Arc<TorrentRepository>,
}

impl InMemoryTorrentRepository {
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
    #[must_use]
    pub fn upsert_peer(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
        opt_persistent_torrent: Option<PersistentTorrent>,
    ) -> bool {
        self.torrents.upsert_peer(info_hash, peer, opt_persistent_torrent)
    }

    /// Removes a torrent entry from the repository.
    ///
    /// This method is only available in tests. It removes the torrent entry
    /// associated with the given info hash and returns the removed entry if it
    /// existed.
    ///
    /// # Arguments
    ///
    /// * `key` - The info hash of the torrent to remove.
    ///
    /// # Returns
    ///
    /// An `Option` containing the removed torrent entry if it existed.
    #[cfg(test)]
    #[must_use]
    pub(crate) fn remove(&self, key: &InfoHash) -> Option<SwarmHandle> {
        self.torrents.remove(key)
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
    pub(crate) fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        self.torrents.remove_inactive_peers(current_cutoff);
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
    pub(crate) fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        self.torrents.remove_peerless_torrents(policy);
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
    pub(crate) fn get(&self, key: &InfoHash) -> Option<SwarmHandle> {
        self.torrents.get(key)
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
    pub(crate) fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, SwarmHandle)> {
        self.torrents.get_paginated(pagination)
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
    #[must_use]
    pub(crate) fn get_swarm_metadata_or_default(&self, info_hash: &InfoHash) -> SwarmMetadata {
        self.torrents.get_swarm_metadata_or_default(info_hash)
    }

    /// Retrieves torrent peers for a given torrent and client, excluding the
    /// requesting client.
    ///
    /// This method filters out the client making the request (based on its
    /// network address) and returns up to a maximum number of peers, defined by
    /// the greater of the provided limit or the global `TORRENT_PEERS_LIMIT`.
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
    #[must_use]
    pub(crate) fn get_peers_for(&self, info_hash: &InfoHash, peer: &peer::Peer, limit: usize) -> Vec<Arc<peer::Peer>> {
        self.torrents.get_peers_for(info_hash, peer, max(limit, TORRENT_PEERS_LIMIT))
    }

    /// Retrieves the list of peers for a given torrent.
    ///
    /// This method returns up to `TORRENT_PEERS_LIMIT` peers for the torrent
    /// specified by the info-hash.
    ///
    /// # Arguments
    ///
    /// * `info_hash` - The info hash of the torrent.
    ///
    /// # Returns
    ///
    /// A vector of peers (wrapped in `Arc`) representing the active peers for
    /// the torrent.
    ///
    /// # Panics
    ///
    /// This function panics if the lock for the torrent entry cannot be obtained.
    #[must_use]
    pub fn get_torrent_peers(&self, info_hash: &InfoHash) -> Vec<Arc<peer::Peer>> {
        // todo: pass the limit as an argument like `get_peers_for`
        self.torrents.get_torrent_peers(info_hash, TORRENT_PEERS_LIMIT)
    }

    /// Calculates and returns overall torrent metrics.
    ///
    /// The returned [`AggregateSwarmMetadata`] contains aggregate data such as
    /// the total number of torrents, total complete (seeders), incomplete
    /// (leechers), and downloaded counts.
    ///
    /// # Returns
    ///
    /// A [`AggregateSwarmMetadata`] struct with the aggregated metrics.
    #[must_use]
    pub fn get_aggregate_swarm_metadata(&self) -> AggregateSwarmMetadata {
        self.torrents.get_aggregate_swarm_metadata()
    }

    /// Imports persistent torrent data into the in-memory repository.
    ///
    /// This method takes a set of persisted torrent entries (e.g., from a database)
    /// and imports them into the in-memory repository for immediate access.
    ///
    /// # Arguments
    ///
    /// * `persistent_torrents` - A reference to the persisted torrent data.
    pub fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        self.torrents.import_persistent(persistent_torrents);
    }
}
