//! In-memory torrents repository.
use std::cmp::max;
use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_configuration::{TrackerPolicy, TORRENT_PEERS_LIMIT};
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::{AggregateSwarmMetadata, SwarmMetadata};
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrent, PersistentTorrents};
use torrust_tracker_torrent_repository::{TorrentEntry, Torrents};

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
    torrents: Arc<Torrents>,
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
    pub(crate) fn remove(&self, key: &InfoHash) -> Option<TorrentEntry> {
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
    pub(crate) fn get(&self, key: &InfoHash) -> Option<TorrentEntry> {
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
    pub(crate) fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, TorrentEntry)> {
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
    pub(crate) fn get_swarm_metadata(&self, info_hash: &InfoHash) -> SwarmMetadata {
        match self.torrents.get(info_hash) {
            Some(torrent_entry) => torrent_entry
                .lock()
                .expect("can't acquire lock for torrent entry")
                .get_swarm_metadata(),
            None => SwarmMetadata::zeroed(),
        }
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
        match self.torrents.get(info_hash) {
            None => vec![],
            Some(entry) => entry
                .lock()
                .expect("can't acquire lock for torrent entry")
                .get_peers_for_client(&peer.peer_addr, Some(max(limit, TORRENT_PEERS_LIMIT))),
        }
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
        match self.torrents.get(info_hash) {
            None => vec![],
            Some(entry) => entry
                .lock()
                .expect("can't acquire lock for torrent entry")
                .get_peers(Some(TORRENT_PEERS_LIMIT)),
        }
    }

    /// Calculates and returns overall torrent metrics.
    ///
    /// The returned [`TorrentsMetrics`] contains aggregate data such as the
    /// total number of torrents, total complete (seeders), incomplete (leechers),
    /// and downloaded counts.
    ///
    /// # Returns
    ///
    /// A [`TorrentsMetrics`] struct with the aggregated metrics.
    #[must_use]
    pub fn get_torrents_metrics(&self) -> AggregateSwarmMetadata {
        self.torrents.get_metrics()
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

#[cfg(test)]
mod tests {

    mod the_in_memory_torrent_repository {

        use aquatic_udp_protocol::PeerId;

        /// It generates a peer id from a number where the number is the last
        /// part of the peer ID. For example, for `12` it returns
        /// `-qB00000000000000012`.
        fn numeric_peer_id(two_digits_value: i32) -> PeerId {
            // Format idx as a string with leading zeros, ensuring it has exactly 2 digits
            let idx_str = format!("{two_digits_value:02}");

            // Create the base part of the peer ID.
            let base = b"-qB00000000000000000";

            // Concatenate the base with idx bytes, ensuring the total length is 20 bytes.
            let mut peer_id_bytes = [0u8; 20];
            peer_id_bytes[..base.len()].copy_from_slice(base);
            peer_id_bytes[base.len() - idx_str.len()..].copy_from_slice(idx_str.as_bytes());

            PeerId(peer_id_bytes)
        }

        // The `InMemoryTorrentRepository` has these responsibilities:
        // - To maintain the peer lists for each torrent.
        // - To maintain the the torrent entries, which contains all the info about the
        //   torrents, including the peer lists.
        // - To return the torrent entries.
        // - To return the peer lists for a given torrent.
        // - To return the torrent metrics.
        // - To return the swarm metadata for a given torrent.
        // - To handle the persistence of the torrent entries.

        mod maintaining_the_peer_lists {

            use std::sync::Arc;

            use crate::test_helpers::tests::{sample_info_hash, sample_peer};
            use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

            #[tokio::test]
            async fn it_should_add_the_first_peer_to_the_torrent_peer_list() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let info_hash = sample_info_hash();

                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &sample_peer(), None);

                assert!(in_memory_torrent_repository.get(&info_hash).is_some());
            }

            #[tokio::test]
            async fn it_should_allow_adding_the_same_peer_twice_to_the_torrent_peer_list() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let info_hash = sample_info_hash();

                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &sample_peer(), None);
                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &sample_peer(), None);

                assert!(in_memory_torrent_repository.get(&info_hash).is_some());
            }
        }

        mod returning_peer_lists_for_a_torrent {

            use std::net::{IpAddr, Ipv4Addr, SocketAddr};
            use std::sync::Arc;

            use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
            use torrust_tracker_primitives::peer::Peer;
            use torrust_tracker_primitives::DurationSinceUnixEpoch;

            use crate::test_helpers::tests::{sample_info_hash, sample_peer};
            use crate::torrent::repository::in_memory::tests::the_in_memory_torrent_repository::numeric_peer_id;
            use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

            #[tokio::test]
            async fn it_should_return_the_peers_for_a_given_torrent() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let info_hash = sample_info_hash();
                let peer = sample_peer();

                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &peer, None);

                let peers = in_memory_torrent_repository.get_torrent_peers(&info_hash);

                assert_eq!(peers, vec![Arc::new(peer)]);
            }

            #[tokio::test]
            async fn it_should_return_an_empty_list_or_peers_for_a_non_existing_torrent() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let peers = in_memory_torrent_repository.get_torrent_peers(&sample_info_hash());

                assert!(peers.is_empty());
            }

            #[tokio::test]
            async fn it_should_return_74_peers_at_the_most_for_a_given_torrent() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let info_hash = sample_info_hash();

                for idx in 1..=75 {
                    let peer = Peer {
                        peer_id: numeric_peer_id(idx),
                        peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, idx.try_into().unwrap())), 8080),
                        updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                        uploaded: NumberOfBytes::new(0),
                        downloaded: NumberOfBytes::new(0),
                        left: NumberOfBytes::new(0), // No bytes left to download
                        event: AnnounceEvent::Completed,
                    };

                    let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &peer, None);
                }

                let peers = in_memory_torrent_repository.get_torrent_peers(&info_hash);

                assert_eq!(peers.len(), 74);
            }

            mod excluding_the_client_peer {

                use std::net::{IpAddr, Ipv4Addr, SocketAddr};
                use std::sync::Arc;

                use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
                use torrust_tracker_configuration::TORRENT_PEERS_LIMIT;
                use torrust_tracker_primitives::peer::Peer;
                use torrust_tracker_primitives::DurationSinceUnixEpoch;

                use crate::test_helpers::tests::{sample_info_hash, sample_peer};
                use crate::torrent::repository::in_memory::tests::the_in_memory_torrent_repository::numeric_peer_id;
                use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

                #[tokio::test]
                async fn it_should_return_an_empty_peer_list_for_a_non_existing_torrent() {
                    let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                    let peers =
                        in_memory_torrent_repository.get_peers_for(&sample_info_hash(), &sample_peer(), TORRENT_PEERS_LIMIT);

                    assert_eq!(peers, vec![]);
                }

                #[tokio::test]
                async fn it_should_return_the_peers_for_a_given_torrent_excluding_a_given_peer() {
                    let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                    let info_hash = sample_info_hash();
                    let peer = sample_peer();

                    let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &peer, None);

                    let peers = in_memory_torrent_repository.get_peers_for(&info_hash, &peer, TORRENT_PEERS_LIMIT);

                    assert_eq!(peers, vec![]);
                }

                #[tokio::test]
                async fn it_should_return_74_peers_at_the_most_for_a_given_torrent_when_it_filters_out_a_given_peer() {
                    let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                    let info_hash = sample_info_hash();

                    let excluded_peer = sample_peer();

                    let _number_of_downloads_increased =
                        in_memory_torrent_repository.upsert_peer(&info_hash, &excluded_peer, None);

                    // Add 74 peers
                    for idx in 2..=75 {
                        let peer = Peer {
                            peer_id: numeric_peer_id(idx),
                            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, idx.try_into().unwrap())), 8080),
                            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                            uploaded: NumberOfBytes::new(0),
                            downloaded: NumberOfBytes::new(0),
                            left: NumberOfBytes::new(0), // No bytes left to download
                            event: AnnounceEvent::Completed,
                        };

                        let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &peer, None);
                    }

                    let peers = in_memory_torrent_repository.get_peers_for(&info_hash, &excluded_peer, TORRENT_PEERS_LIMIT);

                    assert_eq!(peers.len(), 74);
                }
            }
        }

        mod maintaining_the_torrent_entries {

            use std::ops::Add;
            use std::sync::Arc;
            use std::time::Duration;

            use bittorrent_primitives::info_hash::InfoHash;
            use torrust_tracker_configuration::TrackerPolicy;
            use torrust_tracker_primitives::DurationSinceUnixEpoch;

            use crate::test_helpers::tests::{sample_info_hash, sample_peer};
            use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

            #[tokio::test]
            async fn it_should_remove_a_torrent_entry() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let info_hash = sample_info_hash();
                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &sample_peer(), None);

                let _unused = in_memory_torrent_repository.remove(&info_hash);

                assert!(in_memory_torrent_repository.get(&info_hash).is_none());
            }

            #[tokio::test]
            async fn it_should_remove_peers_that_have_not_been_updated_after_a_cutoff_time() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let info_hash = sample_info_hash();
                let mut peer = sample_peer();
                peer.updated = DurationSinceUnixEpoch::new(0, 0);

                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &peer, None);

                // Cut off time is 1 second after the peer was updated
                in_memory_torrent_repository.remove_inactive_peers(peer.updated.add(Duration::from_secs(1)));

                assert!(!in_memory_torrent_repository
                    .get_torrent_peers(&info_hash)
                    .contains(&Arc::new(peer)));
            }

            fn initialize_repository_with_one_torrent_without_peers(info_hash: &InfoHash) -> Arc<InMemoryTorrentRepository> {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                // Insert a sample peer for the torrent to force adding the torrent entry
                let mut peer = sample_peer();
                peer.updated = DurationSinceUnixEpoch::new(0, 0);
                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(info_hash, &peer, None);

                // Remove the peer
                in_memory_torrent_repository.remove_inactive_peers(peer.updated.add(Duration::from_secs(1)));

                in_memory_torrent_repository
            }

            #[tokio::test]
            async fn it_should_remove_torrents_without_peers() {
                let info_hash = sample_info_hash();

                let in_memory_torrent_repository = initialize_repository_with_one_torrent_without_peers(&info_hash);

                let tracker_policy = TrackerPolicy {
                    remove_peerless_torrents: true,
                    ..Default::default()
                };

                in_memory_torrent_repository.remove_peerless_torrents(&tracker_policy);

                assert!(in_memory_torrent_repository.get(&info_hash).is_none());
            }
        }
        mod returning_torrent_entries {

            use std::sync::Arc;

            use torrust_tracker_primitives::peer::Peer;
            use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
            use torrust_tracker_torrent_repository::TorrentEntry;

            use crate::test_helpers::tests::{sample_info_hash, sample_peer};
            use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

            /// `TorrentEntry` data is not directly accessible. It's only
            /// accessible through the trait methods. We need this temporary
            /// DTO to write simple and more readable assertions.
            #[derive(Debug, Clone, PartialEq)]
            struct TorrentEntryInfo {
                swarm_metadata: SwarmMetadata,
                peers: Vec<Peer>,
                number_of_peers: usize,
            }

            #[allow(clippy::from_over_into)]
            impl Into<TorrentEntryInfo> for TorrentEntry {
                fn into(self) -> TorrentEntryInfo {
                    let torrent_guard = self.lock().expect("can't acquire lock for torrent entry");

                    let torrent_entry_info = TorrentEntryInfo {
                        swarm_metadata: torrent_guard.get_swarm_metadata(),
                        peers: torrent_guard.get_peers(None).iter().map(|peer| *peer.clone()).collect(),
                        number_of_peers: torrent_guard.get_peers_len(),
                    };

                    drop(torrent_guard);

                    torrent_entry_info
                }
            }

            #[tokio::test]
            async fn it_should_return_one_torrent_entry_by_infohash() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let info_hash = sample_info_hash();
                let peer = sample_peer();

                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &peer, None);

                let torrent_entry = in_memory_torrent_repository.get(&info_hash).unwrap();

                assert_eq!(
                    TorrentEntryInfo {
                        swarm_metadata: SwarmMetadata {
                            downloaded: 0,
                            complete: 1,
                            incomplete: 0
                        },
                        peers: vec!(peer),
                        number_of_peers: 1
                    },
                    torrent_entry.into()
                );
            }

            mod it_should_return_many_torrent_entries {
                use std::sync::Arc;

                use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

                use crate::test_helpers::tests::{sample_info_hash, sample_peer};
                use crate::torrent::repository::in_memory::tests::the_in_memory_torrent_repository::returning_torrent_entries::TorrentEntryInfo;
                use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

                #[tokio::test]
                async fn without_pagination() {
                    let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                    let info_hash = sample_info_hash();
                    let peer = sample_peer();
                    let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &peer, None);

                    let torrent_entries = in_memory_torrent_repository.get_paginated(None);

                    assert_eq!(torrent_entries.len(), 1);

                    let torrent_entry = torrent_entries.first().unwrap().1.clone();

                    assert_eq!(
                        TorrentEntryInfo {
                            swarm_metadata: SwarmMetadata {
                                downloaded: 0,
                                complete: 1,
                                incomplete: 0
                            },
                            peers: vec!(peer),
                            number_of_peers: 1
                        },
                        torrent_entry.into()
                    );
                }

                mod with_pagination {
                    use std::sync::Arc;

                    use torrust_tracker_primitives::pagination::Pagination;
                    use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

                    use crate::test_helpers::tests::{
                        sample_info_hash_alphabetically_ordered_after_sample_info_hash_one, sample_info_hash_one,
                        sample_peer_one, sample_peer_two,
                    };
                    use crate::torrent::repository::in_memory::tests::the_in_memory_torrent_repository::returning_torrent_entries::TorrentEntryInfo;
                    use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

                    #[tokio::test]
                    async fn it_should_return_the_first_page() {
                        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                        // Insert one torrent entry
                        let info_hash_one = sample_info_hash_one();
                        let peer_one = sample_peer_one();
                        let _number_of_downloads_increased =
                            in_memory_torrent_repository.upsert_peer(&info_hash_one, &peer_one, None);

                        // Insert another torrent entry
                        let info_hash_one = sample_info_hash_alphabetically_ordered_after_sample_info_hash_one();
                        let peer_two = sample_peer_two();
                        let _number_of_downloads_increased =
                            in_memory_torrent_repository.upsert_peer(&info_hash_one, &peer_two, None);

                        // Get only the first page where page size is 1
                        let torrent_entries =
                            in_memory_torrent_repository.get_paginated(Some(&Pagination { offset: 0, limit: 1 }));

                        assert_eq!(torrent_entries.len(), 1);

                        let torrent_entry = torrent_entries.first().unwrap().1.clone();

                        assert_eq!(
                            TorrentEntryInfo {
                                swarm_metadata: SwarmMetadata {
                                    downloaded: 0,
                                    complete: 1,
                                    incomplete: 0
                                },
                                peers: vec!(peer_one),
                                number_of_peers: 1
                            },
                            torrent_entry.into()
                        );
                    }

                    #[tokio::test]
                    async fn it_should_return_the_second_page() {
                        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                        // Insert one torrent entry
                        let info_hash_one = sample_info_hash_one();
                        let peer_one = sample_peer_one();
                        let _number_of_downloads_increased =
                            in_memory_torrent_repository.upsert_peer(&info_hash_one, &peer_one, None);

                        // Insert another torrent entry
                        let info_hash_one = sample_info_hash_alphabetically_ordered_after_sample_info_hash_one();
                        let peer_two = sample_peer_two();
                        let _number_of_downloads_increased =
                            in_memory_torrent_repository.upsert_peer(&info_hash_one, &peer_two, None);

                        // Get only the first page where page size is 1
                        let torrent_entries =
                            in_memory_torrent_repository.get_paginated(Some(&Pagination { offset: 1, limit: 1 }));

                        assert_eq!(torrent_entries.len(), 1);

                        let torrent_entry = torrent_entries.first().unwrap().1.clone();

                        assert_eq!(
                            TorrentEntryInfo {
                                swarm_metadata: SwarmMetadata {
                                    downloaded: 0,
                                    complete: 1,
                                    incomplete: 0
                                },
                                peers: vec!(peer_two),
                                number_of_peers: 1
                            },
                            torrent_entry.into()
                        );
                    }

                    #[tokio::test]
                    async fn it_should_allow_changing_the_page_size() {
                        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                        // Insert one torrent entry
                        let info_hash_one = sample_info_hash_one();
                        let peer_one = sample_peer_one();
                        let _number_of_downloads_increased =
                            in_memory_torrent_repository.upsert_peer(&info_hash_one, &peer_one, None);

                        // Insert another torrent entry
                        let info_hash_one = sample_info_hash_alphabetically_ordered_after_sample_info_hash_one();
                        let peer_two = sample_peer_two();
                        let _number_of_downloads_increased =
                            in_memory_torrent_repository.upsert_peer(&info_hash_one, &peer_two, None);

                        // Get only the first page where page size is 1
                        let torrent_entries =
                            in_memory_torrent_repository.get_paginated(Some(&Pagination { offset: 1, limit: 1 }));

                        assert_eq!(torrent_entries.len(), 1);
                    }
                }
            }
        }

        mod returning_aggregate_swarm_metadata {

            use std::sync::Arc;

            use bittorrent_primitives::info_hash::fixture::gen_seeded_infohash;
            use torrust_tracker_primitives::swarm_metadata::AggregateSwarmMetadata;

            use crate::test_helpers::tests::{complete_peer, leecher, sample_info_hash, seeder};
            use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

            // todo: refactor to use test parametrization

            #[tokio::test]
            async fn it_should_get_empty_aggregate_swarm_metadata_when_there_are_no_torrents() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let aggregate_swarm_metadata = in_memory_torrent_repository.get_torrents_metrics();

                assert_eq!(
                    aggregate_swarm_metadata,
                    AggregateSwarmMetadata {
                        total_complete: 0,
                        total_downloaded: 0,
                        total_incomplete: 0,
                        total_torrents: 0
                    }
                );
            }

            #[tokio::test]
            async fn it_should_return_the_aggregate_swarm_metadata_when_there_is_a_leecher() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let _number_of_downloads_increased =
                    in_memory_torrent_repository.upsert_peer(&sample_info_hash(), &leecher(), None);

                let aggregate_swarm_metadata = in_memory_torrent_repository.get_torrents_metrics();

                assert_eq!(
                    aggregate_swarm_metadata,
                    AggregateSwarmMetadata {
                        total_complete: 0,
                        total_downloaded: 0,
                        total_incomplete: 1,
                        total_torrents: 1,
                    }
                );
            }

            #[tokio::test]
            async fn it_should_return_the_aggregate_swarm_metadata_when_there_is_a_seeder() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let _number_of_downloads_increased =
                    in_memory_torrent_repository.upsert_peer(&sample_info_hash(), &seeder(), None);

                let aggregate_swarm_metadata = in_memory_torrent_repository.get_torrents_metrics();

                assert_eq!(
                    aggregate_swarm_metadata,
                    AggregateSwarmMetadata {
                        total_complete: 1,
                        total_downloaded: 0,
                        total_incomplete: 0,
                        total_torrents: 1,
                    }
                );
            }

            #[tokio::test]
            async fn it_should_return_the_aggregate_swarm_metadata_when_there_is_a_completed_peer() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let _number_of_downloads_increased =
                    in_memory_torrent_repository.upsert_peer(&sample_info_hash(), &complete_peer(), None);

                let aggregate_swarm_metadata = in_memory_torrent_repository.get_torrents_metrics();

                assert_eq!(
                    aggregate_swarm_metadata,
                    AggregateSwarmMetadata {
                        total_complete: 1,
                        total_downloaded: 0,
                        total_incomplete: 0,
                        total_torrents: 1,
                    }
                );
            }

            #[tokio::test]
            async fn it_should_return_the_aggregate_swarm_metadata_when_there_are_multiple_torrents() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let start_time = std::time::Instant::now();
                for i in 0..1_000_000 {
                    let _number_of_downloads_increased =
                        in_memory_torrent_repository.upsert_peer(&gen_seeded_infohash(&i), &leecher(), None);
                }
                let result_a = start_time.elapsed();

                let start_time = std::time::Instant::now();
                let aggregate_swarm_metadata = in_memory_torrent_repository.get_torrents_metrics();
                let result_b = start_time.elapsed();

                assert_eq!(
                    (aggregate_swarm_metadata),
                    (AggregateSwarmMetadata {
                        total_complete: 0,
                        total_downloaded: 0,
                        total_incomplete: 1_000_000,
                        total_torrents: 1_000_000,
                    }),
                    "{result_a:?} {result_b:?}"
                );
            }
        }

        mod returning_swarm_metadata {

            use std::sync::Arc;

            use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

            use crate::test_helpers::tests::{leecher, sample_info_hash};
            use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

            #[tokio::test]
            async fn it_should_get_swarm_metadata_for_an_existing_torrent() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let infohash = sample_info_hash();

                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&infohash, &leecher(), None);

                let swarm_metadata = in_memory_torrent_repository.get_swarm_metadata(&infohash);

                assert_eq!(
                    swarm_metadata,
                    SwarmMetadata {
                        complete: 0,
                        downloaded: 0,
                        incomplete: 1,
                    }
                );
            }

            #[tokio::test]
            async fn it_should_return_zeroed_swarm_metadata_for_a_non_existing_torrent() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let swarm_metadata = in_memory_torrent_repository.get_swarm_metadata(&sample_info_hash());

                assert_eq!(swarm_metadata, SwarmMetadata::zeroed());
            }
        }

        mod handling_persistence {

            use std::sync::Arc;

            use torrust_tracker_primitives::PersistentTorrents;

            use crate::test_helpers::tests::sample_info_hash;
            use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

            #[tokio::test]
            async fn it_should_allow_importing_persisted_torrent_entries() {
                let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

                let infohash = sample_info_hash();

                let mut persistent_torrents = PersistentTorrents::default();

                persistent_torrents.insert(infohash, 1);

                in_memory_torrent_repository.import_persistent(&persistent_torrents);

                let swarm_metadata = in_memory_torrent_repository.get_swarm_metadata(&infohash);

                // Only the number of downloads is persisted.
                assert_eq!(swarm_metadata.downloaded, 1);
            }
        }
    }
}
