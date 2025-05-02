use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use crossbeam_skiplist::SkipMap;
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::{AggregateSwarmMetadata, SwarmMetadata};
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrent, PersistentTorrents};

use crate::entry::peer_list::PeerList;
use crate::entry::torrent::TrackedTorrent;
use crate::TrackedTorrentHandle;

#[derive(Default, Debug)]
pub struct TorrentRepository {
    pub torrents: SkipMap<InfoHash, TrackedTorrentHandle>,
}

impl TorrentRepository {
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
            tracing::debug!("Torrent already exists: {:?}", info_hash);

            existing_entry
                .value()
                .lock()
                .expect("can't acquire lock for tracked torrent handle")
                .upsert_peer(peer)
        } else {
            tracing::debug!("Inserting new torrent: {:?}", info_hash);

            let new_entry = if let Some(number_of_downloads) = opt_persistent_torrent {
                TrackedTorrentHandle::new(
                    TrackedTorrent {
                        swarm: PeerList::default(),
                        downloaded: number_of_downloads,
                    }
                    .into(),
                )
            } else {
                TrackedTorrentHandle::default()
            };

            let inserted_entry = self.torrents.get_or_insert(*info_hash, new_entry);

            let mut torrent_guard = inserted_entry
                .value()
                .lock()
                .expect("can't acquire lock for tracked torrent handle");

            torrent_guard.upsert_peer(peer)
        }
    }

    /// Removes a torrent entry from the repository.
    ///
    /// # Returns
    ///
    /// An `Option` containing the removed torrent entry if it existed.
    #[must_use]
    pub fn remove(&self, key: &InfoHash) -> Option<TrackedTorrentHandle> {
        self.torrents.remove(key).map(|entry| entry.value().clone())
    }

    /// Removes inactive peers from all torrent entries.
    ///
    /// A peer is considered inactive if its last update timestamp is older than
    /// the provided cutoff time.
    ///
    /// # Panics
    ///
    /// This function panics if the lock for the entry cannot be obtained.
    pub fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        for entry in &self.torrents {
            entry
                .value()
                .lock()
                .expect("can't acquire lock for tracked torrent handle")
                .remove_inactive_peers(current_cutoff);
        }
    }

    /// Retrieves a tracked torrent handle by its infohash.
    ///
    /// # Returns
    ///
    /// An `Option` containing the tracked torrent handle if found.
    #[must_use]
    pub fn get(&self, key: &InfoHash) -> Option<TrackedTorrentHandle> {
        let maybe_entry = self.torrents.get(key);
        maybe_entry.map(|entry| entry.value().clone())
    }

    /// Retrieves a paginated list of tracked torrent handles.
    ///
    /// This method returns a vector of tuples, each containing an infohash and
    /// its associated tracked torrent handle. The pagination parameters
    /// (offset and limit) can be used to control the size of the result set.
    ///
    /// # Returns
    ///
    /// A vector of `(InfoHash, TorrentEntry)` tuples.
    #[must_use]
    pub fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, TrackedTorrentHandle)> {
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

    /// Retrieves swarm metadata for a given torrent.
    ///
    /// # Returns
    ///
    /// A `SwarmMetadata` struct containing the aggregated torrent data if found.
    ///
    /// # Panics
    ///
    /// This function panics if the lock for the entry cannot be obtained.
    #[must_use]
    pub fn get_swarm_metadata(&self, info_hash: &InfoHash) -> Option<SwarmMetadata> {
        self.torrents.get(info_hash).map(|entry| {
            entry
                .value()
                .lock()
                .expect("can't acquire lock for tracked torrent handle")
                .get_swarm_metadata()
        })
    }

    /// Retrieves swarm metadata for a given torrent.
    ///
    /// # Returns
    ///
    /// A `SwarmMetadata` struct containing the aggregated torrent data if it's
    /// found or a zeroed metadata struct if not.
    #[must_use]
    pub fn get_swarm_metadata_or_default(&self, info_hash: &InfoHash) -> SwarmMetadata {
        match self.get_swarm_metadata(info_hash) {
            Some(swarm_metadata) => swarm_metadata,
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
    /// # Returns
    ///
    /// A vector of peers (wrapped in `Arc`) representing the active peers for
    /// the torrent, excluding the requesting client.
    ///
    /// # Panics
    ///
    /// This function panics if the lock for the torrent entry cannot be obtained.
    #[must_use]
    pub fn get_peers_for(&self, info_hash: &InfoHash, peer: &peer::Peer, limit: usize) -> Vec<Arc<peer::Peer>> {
        match self.get(info_hash) {
            None => vec![],
            Some(entry) => entry
                .lock()
                .expect("can't acquire lock for tracked torrent handle")
                .get_peers_for_client(&peer.peer_addr, Some(limit)),
        }
    }

    /// Retrieves the list of peers for a given torrent.
    ///
    /// This method returns up to `TORRENT_PEERS_LIMIT` peers for the torrent
    /// specified by the info-hash.
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
    pub fn get_torrent_peers(&self, info_hash: &InfoHash, limit: usize) -> Vec<Arc<peer::Peer>> {
        match self.get(info_hash) {
            None => vec![],
            Some(entry) => entry
                .lock()
                .expect("can't acquire lock for tracked torrent handle")
                .get_peers(Some(limit)),
        }
    }

    /// Removes torrent entries that have no active peers.
    ///
    /// Depending on the tracker policy, torrents without any peers may be
    /// removed to conserve memory.
    ///
    /// # Panics
    ///
    /// This function panics if the lock for the entry cannot be obtained.
    pub fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        for entry in &self.torrents {
            if entry
                .value()
                .lock()
                .expect("can't acquire lock for tracked torrent handle")
                .meets_retaining_policy(policy)
            {
                continue;
            }

            entry.remove();
        }
    }

    /// Imports persistent torrent data into the in-memory repository.
    ///
    /// This method takes a set of persisted torrent entries (e.g., from a
    /// database) and imports them into the in-memory repository for immediate
    /// access.
    pub fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        for (info_hash, completed) in persistent_torrents {
            if self.torrents.contains_key(info_hash) {
                continue;
            }

            let entry = TrackedTorrentHandle::new(
                TrackedTorrent {
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

    /// Calculates and returns overall torrent metrics.
    ///
    /// The returned [`AggregateSwarmMetadata`] contains aggregate data such as
    /// the total number of torrents, total complete (seeders), incomplete
    /// (leechers), and downloaded counts.
    ///
    /// # Returns
    ///
    /// A [`AggregateSwarmMetadata`] struct with the aggregated metrics.
    ///
    /// # Panics
    ///
    /// This function panics if the lock for the entry cannot be obtained.
    #[must_use]
    pub fn get_aggregate_swarm_metadata(&self) -> AggregateSwarmMetadata {
        let mut metrics = AggregateSwarmMetadata::default();

        for entry in &self.torrents {
            let stats = entry
                .value()
                .lock()
                .expect("can't acquire lock for tracked torrent handle")
                .get_swarm_metadata();
            metrics.total_complete += u64::from(stats.complete);
            metrics.total_downloaded += u64::from(stats.downloaded);
            metrics.total_incomplete += u64::from(stats.incomplete);
            metrics.total_torrents += 1;
        }

        metrics
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

        // The `TorrentRepository` has these responsibilities:
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

            use crate::repository::TorrentRepository;
            use crate::tests::{sample_info_hash, sample_peer};

            #[tokio::test]
            async fn it_should_add_the_first_peer_to_the_torrent_peer_list() {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let info_hash = sample_info_hash();

                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &sample_peer(), None);

                assert!(in_memory_torrent_repository.get(&info_hash).is_some());
            }

            #[tokio::test]
            async fn it_should_allow_adding_the_same_peer_twice_to_the_torrent_peer_list() {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

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

            use crate::repository::tests::the_in_memory_torrent_repository::numeric_peer_id;
            use crate::repository::TorrentRepository;
            use crate::tests::{sample_info_hash, sample_peer};

            #[tokio::test]
            async fn it_should_return_the_peers_for_a_given_torrent() {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let info_hash = sample_info_hash();
                let peer = sample_peer();

                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &peer, None);

                let peers = in_memory_torrent_repository.get_torrent_peers(&info_hash, 74);

                assert_eq!(peers, vec![Arc::new(peer)]);
            }

            #[tokio::test]
            async fn it_should_return_an_empty_list_or_peers_for_a_non_existing_torrent() {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let peers = in_memory_torrent_repository.get_torrent_peers(&sample_info_hash(), 74);

                assert!(peers.is_empty());
            }

            #[tokio::test]
            async fn it_should_return_74_peers_at_the_most_for_a_given_torrent() {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

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

                let peers = in_memory_torrent_repository.get_torrent_peers(&info_hash, 74);

                assert_eq!(peers.len(), 74);
            }

            mod excluding_the_client_peer {

                use std::net::{IpAddr, Ipv4Addr, SocketAddr};
                use std::sync::Arc;

                use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
                use torrust_tracker_configuration::TORRENT_PEERS_LIMIT;
                use torrust_tracker_primitives::peer::Peer;
                use torrust_tracker_primitives::DurationSinceUnixEpoch;

                use crate::repository::tests::the_in_memory_torrent_repository::numeric_peer_id;
                use crate::repository::TorrentRepository;
                use crate::tests::{sample_info_hash, sample_peer};

                #[tokio::test]
                async fn it_should_return_an_empty_peer_list_for_a_non_existing_torrent() {
                    let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                    let peers =
                        in_memory_torrent_repository.get_peers_for(&sample_info_hash(), &sample_peer(), TORRENT_PEERS_LIMIT);

                    assert_eq!(peers, vec![]);
                }

                #[tokio::test]
                async fn it_should_return_the_peers_for_a_given_torrent_excluding_a_given_peer() {
                    let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                    let info_hash = sample_info_hash();
                    let peer = sample_peer();

                    let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &peer, None);

                    let peers = in_memory_torrent_repository.get_peers_for(&info_hash, &peer, TORRENT_PEERS_LIMIT);

                    assert_eq!(peers, vec![]);
                }

                #[tokio::test]
                async fn it_should_return_74_peers_at_the_most_for_a_given_torrent_when_it_filters_out_a_given_peer() {
                    let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

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

            use crate::repository::TorrentRepository;
            use crate::tests::{sample_info_hash, sample_peer};

            #[tokio::test]
            async fn it_should_remove_a_torrent_entry() {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let info_hash = sample_info_hash();
                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &sample_peer(), None);

                let _unused = in_memory_torrent_repository.remove(&info_hash);

                assert!(in_memory_torrent_repository.get(&info_hash).is_none());
            }

            #[tokio::test]
            async fn it_should_remove_peers_that_have_not_been_updated_after_a_cutoff_time() {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let info_hash = sample_info_hash();
                let mut peer = sample_peer();
                peer.updated = DurationSinceUnixEpoch::new(0, 0);

                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&info_hash, &peer, None);

                // Cut off time is 1 second after the peer was updated
                in_memory_torrent_repository.remove_inactive_peers(peer.updated.add(Duration::from_secs(1)));

                assert!(!in_memory_torrent_repository
                    .get_torrent_peers(&info_hash, 74)
                    .contains(&Arc::new(peer)));
            }

            fn initialize_repository_with_one_torrent_without_peers(info_hash: &InfoHash) -> Arc<TorrentRepository> {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

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

            use crate::repository::TorrentRepository;
            use crate::tests::{sample_info_hash, sample_peer};
            use crate::TrackedTorrentHandle;

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
            impl Into<TorrentEntryInfo> for TrackedTorrentHandle {
                fn into(self) -> TorrentEntryInfo {
                    let torrent_guard = self.lock().expect("can't acquire lock for tracked torrent handle");

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
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

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

                use crate::repository::tests::the_in_memory_torrent_repository::returning_torrent_entries::TorrentEntryInfo;
                use crate::repository::TorrentRepository;
                use crate::tests::{sample_info_hash, sample_peer};

                #[tokio::test]
                async fn without_pagination() {
                    let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

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

                    use crate::repository::tests::the_in_memory_torrent_repository::returning_torrent_entries::TorrentEntryInfo;
                    use crate::repository::TorrentRepository;
                    use crate::tests::{
                        sample_info_hash_alphabetically_ordered_after_sample_info_hash_one, sample_info_hash_one,
                        sample_peer_one, sample_peer_two,
                    };

                    #[tokio::test]
                    async fn it_should_return_the_first_page() {
                        let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

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
                        let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

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
                        let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

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

            use crate::repository::TorrentRepository;
            use crate::tests::{complete_peer, leecher, sample_info_hash, seeder};

            // todo: refactor to use test parametrization

            #[tokio::test]
            async fn it_should_get_empty_aggregate_swarm_metadata_when_there_are_no_torrents() {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let aggregate_swarm_metadata = in_memory_torrent_repository.get_aggregate_swarm_metadata();

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
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let _number_of_downloads_increased =
                    in_memory_torrent_repository.upsert_peer(&sample_info_hash(), &leecher(), None);

                let aggregate_swarm_metadata = in_memory_torrent_repository.get_aggregate_swarm_metadata();

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
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let _number_of_downloads_increased =
                    in_memory_torrent_repository.upsert_peer(&sample_info_hash(), &seeder(), None);

                let aggregate_swarm_metadata = in_memory_torrent_repository.get_aggregate_swarm_metadata();

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
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let _number_of_downloads_increased =
                    in_memory_torrent_repository.upsert_peer(&sample_info_hash(), &complete_peer(), None);

                let aggregate_swarm_metadata = in_memory_torrent_repository.get_aggregate_swarm_metadata();

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
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let start_time = std::time::Instant::now();
                for i in 0..1_000_000 {
                    let _number_of_downloads_increased =
                        in_memory_torrent_repository.upsert_peer(&gen_seeded_infohash(&i), &leecher(), None);
                }
                let result_a = start_time.elapsed();

                let start_time = std::time::Instant::now();
                let aggregate_swarm_metadata = in_memory_torrent_repository.get_aggregate_swarm_metadata();
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

            use crate::repository::TorrentRepository;
            use crate::tests::{leecher, sample_info_hash};

            #[tokio::test]
            async fn it_should_get_swarm_metadata_for_an_existing_torrent() {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let infohash = sample_info_hash();

                let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(&infohash, &leecher(), None);

                let swarm_metadata = in_memory_torrent_repository.get_swarm_metadata_or_default(&infohash);

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
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let swarm_metadata = in_memory_torrent_repository.get_swarm_metadata_or_default(&sample_info_hash());

                assert_eq!(swarm_metadata, SwarmMetadata::zeroed());
            }
        }

        mod handling_persistence {

            use std::sync::Arc;

            use torrust_tracker_primitives::PersistentTorrents;

            use crate::repository::TorrentRepository;
            use crate::tests::sample_info_hash;

            #[tokio::test]
            async fn it_should_allow_importing_persisted_torrent_entries() {
                let in_memory_torrent_repository = Arc::new(TorrentRepository::default());

                let infohash = sample_info_hash();

                let mut persistent_torrents = PersistentTorrents::default();

                persistent_torrents.insert(infohash, 1);

                in_memory_torrent_repository.import_persistent(&persistent_torrents);

                let swarm_metadata = in_memory_torrent_repository.get_swarm_metadata_or_default(&infohash);

                // Only the number of downloads is persisted.
                assert_eq!(swarm_metadata.downloaded, 1);
            }
        }
    }
}
