use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use crossbeam_skiplist::SkipMap;
use tokio::sync::Mutex;
use torrust_tracker_clock::conv::convert_from_timestamp_to_datetime_utc;
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::{AggregateActiveSwarmMetadata, SwarmMetadata};
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, NumberOfDownloads, NumberOfDownloadsBTreeMap};

use crate::event::sender::Sender;
use crate::event::Event;
use crate::swarm::coordinator::Coordinator;
use crate::CoordinatorHandle;

#[derive(Default)]
pub struct Registry {
    swarms: SkipMap<InfoHash, CoordinatorHandle>,
    event_sender: Sender,
}

impl Registry {
    #[must_use]
    pub fn new(event_sender: Sender) -> Self {
        Self {
            swarms: SkipMap::new(),
            event_sender,
        }
    }

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
    /// # Errors
    ///
    /// This function panics if the lock for the swarm handle cannot be acquired.
    pub async fn handle_announcement(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
        opt_persistent_torrent: Option<NumberOfDownloads>,
    ) -> Result<(), Error> {
        let swarm_handle = match self.swarms.get(info_hash) {
            None => {
                let number_of_downloads = opt_persistent_torrent.unwrap_or_default();

                let new_swarm_handle =
                    CoordinatorHandle::new(Coordinator::new(info_hash, number_of_downloads, self.event_sender.clone()).into());

                let new_swarm_handle = self.swarms.get_or_insert(*info_hash, new_swarm_handle);

                if let Some(event_sender) = self.event_sender.as_deref() {
                    event_sender
                        .send(Event::TorrentAdded {
                            info_hash: *info_hash,
                            announcement: *peer,
                        })
                        .await;
                }

                new_swarm_handle
            }
            Some(existing_swarm_handle) => existing_swarm_handle,
        };

        let mut swarm = swarm_handle.value().lock().await;

        swarm.handle_announcement(peer).await;

        Ok(())
    }

    /// Inserts a new swarm. Only used for testing purposes.
    pub fn insert(&self, info_hash: &InfoHash, swarm: Coordinator) {
        // code-review: swarms builder? or constructor from vec?
        // It's only used for testing purposes. It allows to pre-define the
        // initial state of the swarm without having to go through the upsert
        // process.

        let swarm_handle = Arc::new(Mutex::new(swarm));

        self.swarms.insert(*info_hash, swarm_handle);

        // IMPORTANT: Notice this does not send an event because is used only
        // for testing purposes. The event is sent only when the torrent is
        // announced for the first time.
    }

    /// Removes a torrent entry from the repository.
    ///
    /// # Returns
    ///
    /// An `Option` containing the removed torrent entry if it existed.
    #[must_use]
    pub async fn remove(&self, key: &InfoHash) -> Option<CoordinatorHandle> {
        let swarm_handle = self.swarms.remove(key).map(|entry| entry.value().clone());

        if let Some(event_sender) = self.event_sender.as_deref() {
            event_sender.send(Event::TorrentRemoved { info_hash: *key }).await;
        }

        swarm_handle
    }

    /// Retrieves a tracked torrent handle by its infohash.
    ///
    /// # Returns
    ///
    /// An `Option` containing the tracked torrent handle if found.
    #[must_use]
    pub fn get(&self, key: &InfoHash) -> Option<CoordinatorHandle> {
        let maybe_entry = self.swarms.get(key);
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
    pub fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, CoordinatorHandle)> {
        match pagination {
            Some(pagination) => self
                .swarms
                .iter()
                .skip(pagination.offset as usize)
                .take(pagination.limit as usize)
                .map(|entry| (*entry.key(), entry.value().clone()))
                .collect(),
            None => self
                .swarms
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
    /// # Errors
    ///
    /// This function panics if the lock for the swarm handle cannot be acquired.
    pub async fn get_swarm_metadata(&self, info_hash: &InfoHash) -> Result<Option<SwarmMetadata>, Error> {
        match self.swarms.get(info_hash) {
            None => Ok(None),
            Some(swarm_handle) => {
                let swarm = swarm_handle.value().lock().await;
                Ok(Some(swarm.metadata()))
            }
        }
    }

    /// Retrieves swarm metadata for a given torrent.
    ///
    /// # Returns
    ///
    /// A `SwarmMetadata` struct containing the aggregated torrent data if it's
    /// found or a zeroed metadata struct if not.
    ///
    /// # Errors
    ///
    /// This function returns an error if it fails to acquire the lock for the
    /// swarm handle.
    pub async fn get_swarm_metadata_or_default(&self, info_hash: &InfoHash) -> Result<SwarmMetadata, Error> {
        match self.get_swarm_metadata(info_hash).await {
            Ok(Some(swarm_metadata)) => Ok(swarm_metadata),
            Ok(None) => Ok(SwarmMetadata::zeroed()),
            Err(err) => Err(err),
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
    /// # Errors
    ///
    /// This function returns an error if it fails to acquire the lock for the
    /// swarm handle.
    pub async fn get_peers_peers_excluding(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
        limit: usize,
    ) -> Result<Vec<Arc<peer::Peer>>, Error> {
        match self.get(info_hash) {
            None => Ok(vec![]),
            Some(swarm_handle) => {
                let swarm = swarm_handle.lock().await;
                Ok(swarm.peers_excluding(&peer.peer_addr, Some(limit)))
            }
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
    /// # Errors
    ///
    /// This function returns an error if it fails to acquire the lock for the
    /// swarm handle.
    pub async fn get_swarm_peers(&self, info_hash: &InfoHash, limit: usize) -> Result<Vec<Arc<peer::Peer>>, Error> {
        match self.get(info_hash) {
            None => Ok(vec![]),
            Some(swarm_handle) => {
                let swarm = swarm_handle.lock().await;
                Ok(swarm.peers(Some(limit)))
            }
        }
    }

    pub async fn get_activity_metadata(&self, current_cutoff: DurationSinceUnixEpoch) -> AggregateActivityMetadata {
        let mut active_peers_total = 0;
        let mut inactive_peers_total = 0;
        let mut active_torrents_total = 0;

        for swarm_handle in &self.swarms {
            let swarm = swarm_handle.value().lock().await;

            let activity_metadata = swarm.get_activity_metadata(current_cutoff);

            if activity_metadata.is_active {
                active_torrents_total += 1;
            }

            active_peers_total += activity_metadata.active_peers_total;
            inactive_peers_total += activity_metadata.inactive_peers_total;
        }

        AggregateActivityMetadata {
            active_peers_total,
            inactive_peers_total,
            active_torrents_total,
            inactive_torrents_total: self.len() - active_torrents_total,
        }
    }

    /// Counts the number of inactive peers across all torrents.
    pub async fn count_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) -> usize {
        let mut inactive_peers_total = 0;

        for swarm_handle in &self.swarms {
            let swarm = swarm_handle.value().lock().await;
            inactive_peers_total += swarm.count_inactive_peers(current_cutoff);
        }

        inactive_peers_total
    }

    /// Removes inactive peers from all torrent entries.
    ///
    /// A peer is considered inactive if its last update timestamp is older than
    /// the provided cutoff time.
    ///
    /// # Errors
    ///
    /// This function returns an error if it fails to acquire the lock for any
    /// swarm handle.
    pub async fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) -> Result<usize, Error> {
        tracing::info!(
            "Removing inactive peers since: {:?} ...",
            convert_from_timestamp_to_datetime_utc(current_cutoff)
        );

        let mut inactive_peers_removed = 0;

        for swarm_handle in &self.swarms {
            let mut swarm = swarm_handle.value().lock().await;
            let removed = swarm.remove_inactive(current_cutoff).await;
            inactive_peers_removed += removed;
        }

        tracing::info!(inactive_peers_removed = inactive_peers_removed);

        Ok(inactive_peers_removed)
    }

    /// Removes torrent entries that have no active peers.
    ///
    /// Depending on the tracker policy, torrents without any peers may be
    /// removed to conserve memory.
    ///
    /// # Errors
    ///
    /// This function returns an error if it fails to acquire the lock for any
    /// swarm handle.
    pub async fn remove_peerless_torrents(&self, policy: &TrackerPolicy) -> Result<u64, Error> {
        tracing::info!("Removing peerless torrents ...");

        let mut peerless_torrents_removed = 0;

        for swarm_handle in &self.swarms {
            let swarm = swarm_handle.value().lock().await;

            if swarm.meets_retaining_policy(policy) {
                continue;
            }

            let info_hash = *swarm_handle.key();

            swarm_handle.remove();

            peerless_torrents_removed += 1;

            if let Some(event_sender) = self.event_sender.as_deref() {
                event_sender.send(Event::TorrentRemoved { info_hash }).await;
            }
        }

        tracing::info!(peerless_torrents_removed = peerless_torrents_removed);

        Ok(peerless_torrents_removed)
    }

    /// Imports persistent torrent data into the in-memory repository.
    ///
    /// This method takes a set of persisted torrent entries (e.g., from a
    /// database) and imports them into the in-memory repository for immediate
    /// access.
    pub fn import_persistent(&self, persistent_torrents: &NumberOfDownloadsBTreeMap) -> u64 {
        tracing::info!("Importing persisted info about torrents ...");

        let mut torrents_imported = 0;

        for (info_hash, completed) in persistent_torrents {
            if self.swarms.contains_key(info_hash) {
                continue;
            }

            let entry = CoordinatorHandle::new(Coordinator::new(info_hash, *completed, self.event_sender.clone()).into());

            // Since SkipMap is lock-free the torrent could have been inserted
            // after checking if it exists.
            self.swarms.get_or_insert(*info_hash, entry);

            torrents_imported += 1;
        }

        tracing::info!(imported_torrents = torrents_imported);

        torrents_imported
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
    /// # Errors
    ///
    /// This function returns an error if it fails to acquire the lock for any
    /// swarm handle.
    pub async fn get_aggregate_swarm_metadata(&self) -> Result<AggregateActiveSwarmMetadata, Error> {
        let mut metrics = AggregateActiveSwarmMetadata::default();

        for swarm_handle in &self.swarms {
            let swarm = swarm_handle.value().lock().await;

            let stats = swarm.metadata();

            metrics.total_complete += u64::from(stats.complete);
            metrics.total_downloaded += u64::from(stats.downloaded);
            metrics.total_incomplete += u64::from(stats.incomplete);
            metrics.total_torrents += 1;
        }

        Ok(metrics)
    }

    /// Counts the number of torrents that are peerless (i.e., have no active
    /// peers).
    ///
    /// # Returns
    ///
    /// A `usize` representing the number of peerless torrents.
    ///
    /// # Errors
    ///
    /// This function returns an error if it fails to acquire the lock for any
    /// swarm handle.
    pub async fn count_peerless_torrents(&self) -> Result<usize, Error> {
        let mut peerless_torrents = 0;

        for swarm_handle in &self.swarms {
            let swarm = swarm_handle.value().lock().await;

            if swarm.is_peerless() {
                peerless_torrents += 1;
            }
        }

        Ok(peerless_torrents)
    }

    /// Counts the total number of peers across all torrents.
    ///
    /// # Returns
    ///
    /// A `usize` representing the total number of peers.
    ///
    /// # Errors
    ///
    /// This function returns an error if it fails to acquire the lock for any
    /// swarm handle.
    pub async fn count_peers(&self) -> Result<usize, Error> {
        let mut peers = 0;

        for swarm_handle in &self.swarms {
            let swarm = swarm_handle.value().lock().await;

            peers += swarm.len();
        }

        Ok(peers)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.swarms.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.swarms.is_empty()
    }

    pub fn contains(&self, key: &InfoHash) -> bool {
        self.swarms.contains_key(key)
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {}

#[derive(Clone, Debug, Default)]
pub struct AggregateActivityMetadata {
    /// The number of active peers in all swarms.
    pub active_peers_total: usize,

    /// The number of inactive peers in all swarms.
    pub inactive_peers_total: usize,

    /// The number of active torrents.
    pub active_torrents_total: usize,

    /// The number of inactive torrents.
    pub inactive_torrents_total: usize,
}

impl AggregateActivityMetadata {
    pub fn log(&self) {
        tracing::info!(
            active_peers_total = self.active_peers_total,
            inactive_peers_total = self.inactive_peers_total,
            active_torrents_total = self.active_torrents_total,
            inactive_torrents_total = self.inactive_torrents_total
        );
    }
}
#[cfg(test)]
mod tests {

    mod the_swarm_repository {

        use std::sync::Arc;

        use aquatic_udp_protocol::PeerId;

        use crate::swarm::registry::Registry;
        use crate::tests::{sample_info_hash, sample_peer};

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
        // - To maintain the the torrent entries, which contains all the info
        //   about the torrents, including the peer lists.
        // - To return the torrent entries (swarm handles).
        // - To return the peer lists for a given torrent.
        // - To return the torrent metrics.
        // - To return the swarm metadata for a given torrent.
        // - To handle the persistence of the torrent entries.

        #[tokio::test]
        async fn it_should_return_zero_length_when_it_has_no_swarms() {
            let swarms = Arc::new(Registry::default());
            assert_eq!(swarms.len(), 0);
        }

        #[tokio::test]
        async fn it_should_return_the_length_when_it_has_swarms() {
            let swarms = Arc::new(Registry::default());
            let info_hash = sample_info_hash();
            let peer = sample_peer();
            swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();
            assert_eq!(swarms.len(), 1);
        }

        #[tokio::test]
        async fn it_should_be_empty_when_it_has_no_swarms() {
            let swarms = Arc::new(Registry::default());
            assert!(swarms.is_empty());

            let info_hash = sample_info_hash();
            let peer = sample_peer();
            swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();
            assert!(!swarms.is_empty());
        }

        #[tokio::test]
        async fn it_should_not_be_empty_when_it_has_at_least_one_swarm() {
            let swarms = Arc::new(Registry::default());
            let info_hash = sample_info_hash();
            let peer = sample_peer();
            swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

            assert!(!swarms.is_empty());
        }

        mod maintaining_the_peer_lists {

            use std::sync::Arc;

            use crate::swarm::registry::Registry;
            use crate::tests::{sample_info_hash, sample_peer};

            #[tokio::test]
            async fn it_should_add_the_first_peer_to_the_torrent_peer_list() {
                let swarms = Arc::new(Registry::default());

                let info_hash = sample_info_hash();

                swarms.handle_announcement(&info_hash, &sample_peer(), None).await.unwrap();

                assert!(swarms.get(&info_hash).is_some());
            }

            #[tokio::test]
            async fn it_should_allow_adding_the_same_peer_twice_to_the_torrent_peer_list() {
                let swarms = Arc::new(Registry::default());

                let info_hash = sample_info_hash();

                swarms.handle_announcement(&info_hash, &sample_peer(), None).await.unwrap();
                swarms.handle_announcement(&info_hash, &sample_peer(), None).await.unwrap();

                assert!(swarms.get(&info_hash).is_some());
            }
        }

        mod returning_peer_lists_for_a_torrent {

            use std::net::{IpAddr, Ipv4Addr, SocketAddr};
            use std::sync::Arc;

            use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
            use torrust_tracker_primitives::peer::Peer;
            use torrust_tracker_primitives::DurationSinceUnixEpoch;

            use crate::swarm::registry::tests::the_swarm_repository::numeric_peer_id;
            use crate::swarm::registry::Registry;
            use crate::tests::{sample_info_hash, sample_peer};

            #[tokio::test]
            async fn it_should_return_the_peers_for_a_given_torrent() {
                let swarms = Arc::new(Registry::default());

                let info_hash = sample_info_hash();
                let peer = sample_peer();

                swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

                let peers = swarms.get_swarm_peers(&info_hash, 74).await.unwrap();

                assert_eq!(peers, vec![Arc::new(peer)]);
            }

            #[tokio::test]
            async fn it_should_return_an_empty_list_or_peers_for_a_non_existing_torrent() {
                let swarms = Arc::new(Registry::default());

                let peers = swarms.get_swarm_peers(&sample_info_hash(), 74).await.unwrap();

                assert!(peers.is_empty());
            }

            #[tokio::test]
            async fn it_should_return_74_peers_at_the_most_for_a_given_torrent() {
                let swarms = Arc::new(Registry::default());

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

                    swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();
                }

                let peers = swarms.get_swarm_peers(&info_hash, 74).await.unwrap();

                assert_eq!(peers.len(), 74);
            }

            mod excluding_the_client_peer {

                use std::net::{IpAddr, Ipv4Addr, SocketAddr};
                use std::sync::Arc;

                use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
                use torrust_tracker_configuration::TORRENT_PEERS_LIMIT;
                use torrust_tracker_primitives::peer::Peer;
                use torrust_tracker_primitives::DurationSinceUnixEpoch;

                use crate::swarm::registry::tests::the_swarm_repository::numeric_peer_id;
                use crate::swarm::registry::Registry;
                use crate::tests::{sample_info_hash, sample_peer};

                #[tokio::test]
                async fn it_should_return_an_empty_peer_list_for_a_non_existing_torrent() {
                    let swarms = Arc::new(Registry::default());

                    let peers = swarms
                        .get_peers_peers_excluding(&sample_info_hash(), &sample_peer(), TORRENT_PEERS_LIMIT)
                        .await
                        .unwrap();

                    assert_eq!(peers, vec![]);
                }

                #[tokio::test]
                async fn it_should_return_the_peers_for_a_given_torrent_excluding_a_given_peer() {
                    let swarms = Arc::new(Registry::default());

                    let info_hash = sample_info_hash();
                    let peer = sample_peer();

                    swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

                    let peers = swarms
                        .get_peers_peers_excluding(&info_hash, &peer, TORRENT_PEERS_LIMIT)
                        .await
                        .unwrap();

                    assert_eq!(peers, vec![]);
                }

                #[tokio::test]
                async fn it_should_return_74_peers_at_the_most_for_a_given_torrent_when_it_filters_out_a_given_peer() {
                    let swarms = Arc::new(Registry::default());

                    let info_hash = sample_info_hash();

                    let excluded_peer = sample_peer();

                    swarms.handle_announcement(&info_hash, &excluded_peer, None).await.unwrap();

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

                        swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();
                    }

                    let peers = swarms
                        .get_peers_peers_excluding(&info_hash, &excluded_peer, TORRENT_PEERS_LIMIT)
                        .await
                        .unwrap();

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

            use crate::swarm::registry::Registry;
            use crate::tests::{sample_info_hash, sample_peer};

            #[tokio::test]
            async fn it_should_remove_a_torrent_entry() {
                let swarms = Arc::new(Registry::default());

                let info_hash = sample_info_hash();
                swarms.handle_announcement(&info_hash, &sample_peer(), None).await.unwrap();

                let _unused = swarms.remove(&info_hash).await;

                assert!(swarms.get(&info_hash).is_none());
            }

            #[tokio::test]
            async fn it_should_count_inactive_peers() {
                let swarms = Arc::new(Registry::default());

                let info_hash = sample_info_hash();
                let mut peer = sample_peer();
                peer.updated = DurationSinceUnixEpoch::new(0, 0);

                swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

                // Cut off time is 1 second after the peer was updated
                let inactive_peers_total = swarms.count_inactive_peers(peer.updated.add(Duration::from_secs(1))).await;

                assert_eq!(inactive_peers_total, 1);
            }

            #[tokio::test]
            async fn it_should_remove_peers_that_have_not_been_updated_after_a_cutoff_time() {
                let swarms = Arc::new(Registry::default());

                let info_hash = sample_info_hash();
                let mut peer = sample_peer();
                peer.updated = DurationSinceUnixEpoch::new(0, 0);

                swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

                // Cut off time is 1 second after the peer was updated
                swarms
                    .remove_inactive_peers(peer.updated.add(Duration::from_secs(1)))
                    .await
                    .unwrap();

                assert!(!swarms
                    .get_swarm_peers(&info_hash, 74)
                    .await
                    .unwrap()
                    .contains(&Arc::new(peer)));
            }

            async fn initialize_repository_with_one_torrent_without_peers(info_hash: &InfoHash) -> Arc<Registry> {
                let swarms = Arc::new(Registry::default());

                // Insert a sample peer for the torrent to force adding the torrent entry
                let mut peer = sample_peer();
                peer.updated = DurationSinceUnixEpoch::new(0, 0);
                swarms.handle_announcement(info_hash, &peer, None).await.unwrap();

                // Remove the peer
                swarms
                    .remove_inactive_peers(peer.updated.add(Duration::from_secs(1)))
                    .await
                    .unwrap();

                swarms
            }

            #[tokio::test]
            async fn it_should_remove_torrents_without_peers() {
                let info_hash = sample_info_hash();

                let swarms = initialize_repository_with_one_torrent_without_peers(&info_hash).await;

                let tracker_policy = TrackerPolicy {
                    remove_peerless_torrents: true,
                    ..Default::default()
                };

                swarms.remove_peerless_torrents(&tracker_policy).await.unwrap();

                assert!(swarms.get(&info_hash).is_none());
            }
        }
        mod returning_torrent_entries {

            use std::sync::Arc;

            use torrust_tracker_primitives::peer::Peer;
            use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

            use crate::swarm::registry::Registry;
            use crate::tests::{sample_info_hash, sample_peer};
            use crate::{Coordinator, CoordinatorHandle};

            /// `TorrentEntry` data is not directly accessible. It's only
            /// accessible through the trait methods. We need this temporary
            /// DTO to write simple and more readable assertions.
            #[derive(Debug, Clone, PartialEq)]
            struct TorrentEntryInfo {
                swarm_metadata: SwarmMetadata,
                peers: Vec<Peer>,
                number_of_peers: usize,
            }

            async fn torrent_entry_info(swarm_handle: CoordinatorHandle) -> TorrentEntryInfo {
                let torrent_guard = swarm_handle.lock().await;
                torrent_guard.clone().into()
            }

            #[allow(clippy::from_over_into)]
            impl Into<TorrentEntryInfo> for Coordinator {
                fn into(self) -> TorrentEntryInfo {
                    let torrent_entry_info = TorrentEntryInfo {
                        swarm_metadata: self.metadata(),
                        peers: self.peers(None).iter().map(|peer| *peer.clone()).collect(),
                        number_of_peers: self.len(),
                    };
                    torrent_entry_info
                }
            }

            #[tokio::test]
            async fn it_should_return_one_torrent_entry_by_infohash() {
                let swarms = Arc::new(Registry::default());

                let info_hash = sample_info_hash();
                let peer = sample_peer();

                swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

                let torrent_entry_info = torrent_entry_info(swarms.get(&info_hash).unwrap()).await;

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
                    torrent_entry_info
                );
            }

            mod it_should_return_many_torrent_entries {
                use std::sync::Arc;

                use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

                use crate::swarm::registry::tests::the_swarm_repository::returning_torrent_entries::{
                    torrent_entry_info, TorrentEntryInfo,
                };
                use crate::swarm::registry::Registry;
                use crate::tests::{sample_info_hash, sample_peer};

                #[tokio::test]
                async fn without_pagination() {
                    let swarms = Arc::new(Registry::default());

                    let info_hash = sample_info_hash();
                    let peer = sample_peer();
                    swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

                    let torrent_entries = swarms.get_paginated(None);

                    assert_eq!(torrent_entries.len(), 1);

                    let torrent_entry = torrent_entry_info(torrent_entries.first().unwrap().1.clone()).await;

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
                        torrent_entry
                    );
                }

                mod with_pagination {
                    use std::sync::Arc;

                    use torrust_tracker_primitives::pagination::Pagination;
                    use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

                    use crate::swarm::registry::tests::the_swarm_repository::returning_torrent_entries::{
                        torrent_entry_info, TorrentEntryInfo,
                    };
                    use crate::swarm::registry::Registry;
                    use crate::tests::{
                        sample_info_hash_alphabetically_ordered_after_sample_info_hash_one, sample_info_hash_one,
                        sample_peer_one, sample_peer_two,
                    };

                    #[tokio::test]
                    async fn it_should_return_the_first_page() {
                        let swarms = Arc::new(Registry::default());

                        // Insert one torrent entry
                        let info_hash_one = sample_info_hash_one();
                        let peer_one = sample_peer_one();
                        swarms.handle_announcement(&info_hash_one, &peer_one, None).await.unwrap();

                        // Insert another torrent entry
                        let info_hash_one = sample_info_hash_alphabetically_ordered_after_sample_info_hash_one();
                        let peer_two = sample_peer_two();
                        swarms.handle_announcement(&info_hash_one, &peer_two, None).await.unwrap();

                        // Get only the first page where page size is 1
                        let torrent_entries = swarms.get_paginated(Some(&Pagination { offset: 0, limit: 1 }));

                        assert_eq!(torrent_entries.len(), 1);

                        let torrent_entry_info = torrent_entry_info(torrent_entries.first().unwrap().1.clone()).await;

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
                            torrent_entry_info
                        );
                    }

                    #[tokio::test]
                    async fn it_should_return_the_second_page() {
                        let swarms = Arc::new(Registry::default());

                        // Insert one torrent entry
                        let info_hash_one = sample_info_hash_one();
                        let peer_one = sample_peer_one();
                        swarms.handle_announcement(&info_hash_one, &peer_one, None).await.unwrap();

                        // Insert another torrent entry
                        let info_hash_one = sample_info_hash_alphabetically_ordered_after_sample_info_hash_one();
                        let peer_two = sample_peer_two();
                        swarms.handle_announcement(&info_hash_one, &peer_two, None).await.unwrap();

                        // Get only the first page where page size is 1
                        let torrent_entries = swarms.get_paginated(Some(&Pagination { offset: 1, limit: 1 }));

                        assert_eq!(torrent_entries.len(), 1);

                        let torrent_entry_info = torrent_entry_info(torrent_entries.first().unwrap().1.clone()).await;

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
                            torrent_entry_info
                        );
                    }

                    #[tokio::test]
                    async fn it_should_allow_changing_the_page_size() {
                        let swarms = Arc::new(Registry::default());

                        // Insert one torrent entry
                        let info_hash_one = sample_info_hash_one();
                        let peer_one = sample_peer_one();
                        swarms.handle_announcement(&info_hash_one, &peer_one, None).await.unwrap();

                        // Insert another torrent entry
                        let info_hash_one = sample_info_hash_alphabetically_ordered_after_sample_info_hash_one();
                        let peer_two = sample_peer_two();
                        swarms.handle_announcement(&info_hash_one, &peer_two, None).await.unwrap();

                        // Get only the first page where page size is 1
                        let torrent_entries = swarms.get_paginated(Some(&Pagination { offset: 1, limit: 1 }));

                        assert_eq!(torrent_entries.len(), 1);
                    }
                }
            }
        }

        mod returning_aggregate_swarm_metadata {

            use std::sync::Arc;

            use bittorrent_primitives::info_hash::fixture::gen_seeded_infohash;
            use torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata;

            use crate::swarm::registry::Registry;
            use crate::tests::{complete_peer, leecher, sample_info_hash, seeder};

            // todo: refactor to use test parametrization

            #[tokio::test]
            async fn it_should_get_empty_aggregate_swarm_metadata_when_there_are_no_torrents() {
                let swarms = Arc::new(Registry::default());

                let aggregate_swarm_metadata = swarms.get_aggregate_swarm_metadata().await.unwrap();

                assert_eq!(
                    aggregate_swarm_metadata,
                    AggregateActiveSwarmMetadata {
                        total_complete: 0,
                        total_downloaded: 0,
                        total_incomplete: 0,
                        total_torrents: 0
                    }
                );
            }

            #[tokio::test]
            async fn it_should_return_the_aggregate_swarm_metadata_when_there_is_a_leecher() {
                let swarms = Arc::new(Registry::default());

                swarms
                    .handle_announcement(&sample_info_hash(), &leecher(), None)
                    .await
                    .unwrap();

                let aggregate_swarm_metadata = swarms.get_aggregate_swarm_metadata().await.unwrap();

                assert_eq!(
                    aggregate_swarm_metadata,
                    AggregateActiveSwarmMetadata {
                        total_complete: 0,
                        total_downloaded: 0,
                        total_incomplete: 1,
                        total_torrents: 1,
                    }
                );
            }

            #[tokio::test]
            async fn it_should_return_the_aggregate_swarm_metadata_when_there_is_a_seeder() {
                let swarms = Arc::new(Registry::default());

                swarms
                    .handle_announcement(&sample_info_hash(), &seeder(), None)
                    .await
                    .unwrap();

                let aggregate_swarm_metadata = swarms.get_aggregate_swarm_metadata().await.unwrap();

                assert_eq!(
                    aggregate_swarm_metadata,
                    AggregateActiveSwarmMetadata {
                        total_complete: 1,
                        total_downloaded: 0,
                        total_incomplete: 0,
                        total_torrents: 1,
                    }
                );
            }

            #[tokio::test]
            async fn it_should_return_the_aggregate_swarm_metadata_when_there_is_a_completed_peer() {
                let swarms = Arc::new(Registry::default());

                swarms
                    .handle_announcement(&sample_info_hash(), &complete_peer(), None)
                    .await
                    .unwrap();

                let aggregate_swarm_metadata = swarms.get_aggregate_swarm_metadata().await.unwrap();

                assert_eq!(
                    aggregate_swarm_metadata,
                    AggregateActiveSwarmMetadata {
                        total_complete: 1,
                        total_downloaded: 0,
                        total_incomplete: 0,
                        total_torrents: 1,
                    }
                );
            }

            #[tokio::test]
            async fn it_should_return_the_aggregate_swarm_metadata_when_there_are_multiple_torrents() {
                let swarms = Arc::new(Registry::default());

                let start_time = std::time::Instant::now();
                for i in 0..1_000_000 {
                    swarms
                        .handle_announcement(&gen_seeded_infohash(&i), &leecher(), None)
                        .await
                        .unwrap();
                }
                let result_a = start_time.elapsed();

                let start_time = std::time::Instant::now();
                let aggregate_swarm_metadata = swarms.get_aggregate_swarm_metadata().await.unwrap();
                let result_b = start_time.elapsed();

                assert_eq!(
                    (aggregate_swarm_metadata),
                    (AggregateActiveSwarmMetadata {
                        total_complete: 0,
                        total_downloaded: 0,
                        total_incomplete: 1_000_000,
                        total_torrents: 1_000_000,
                    }),
                    "{result_a:?} {result_b:?}"
                );
            }

            mod it_should_count_peerless_torrents {
                use std::sync::Arc;

                use torrust_tracker_primitives::DurationSinceUnixEpoch;

                use crate::swarm::registry::Registry;
                use crate::tests::{sample_info_hash, sample_peer};

                #[tokio::test]
                async fn no_peerless_torrents() {
                    let swarms = Arc::new(Registry::default());
                    assert_eq!(swarms.count_peerless_torrents().await.unwrap(), 0);
                }

                #[tokio::test]
                async fn one_peerless_torrents() {
                    let info_hash = sample_info_hash();
                    let peer = sample_peer();

                    let swarms = Arc::new(Registry::default());
                    swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

                    let current_cutoff = peer.updated + DurationSinceUnixEpoch::from_secs(1);
                    swarms.remove_inactive_peers(current_cutoff).await.unwrap();

                    assert_eq!(swarms.count_peerless_torrents().await.unwrap(), 1);
                }
            }

            mod it_should_count_peers {
                use std::sync::Arc;

                use crate::swarm::registry::Registry;
                use crate::tests::{sample_info_hash, sample_peer};

                #[tokio::test]
                async fn no_peers() {
                    let swarms = Arc::new(Registry::default());
                    assert_eq!(swarms.count_peers().await.unwrap(), 0);
                }

                #[tokio::test]
                async fn one_peer() {
                    let info_hash = sample_info_hash();
                    let peer = sample_peer();

                    let swarms = Arc::new(Registry::default());
                    swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

                    assert_eq!(swarms.count_peers().await.unwrap(), 1);
                }
            }
        }

        mod returning_swarm_metadata {

            use std::sync::Arc;

            use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

            use crate::swarm::registry::Registry;
            use crate::tests::{leecher, sample_info_hash};

            #[tokio::test]
            async fn it_should_get_swarm_metadata_for_an_existing_torrent() {
                let swarms = Arc::new(Registry::default());

                let infohash = sample_info_hash();

                swarms.handle_announcement(&infohash, &leecher(), None).await.unwrap();

                let swarm_metadata = swarms.get_swarm_metadata_or_default(&infohash).await.unwrap();

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
                let swarms = Arc::new(Registry::default());

                let swarm_metadata = swarms.get_swarm_metadata_or_default(&sample_info_hash()).await.unwrap();

                assert_eq!(swarm_metadata, SwarmMetadata::zeroed());
            }
        }

        mod handling_persistence {

            use std::sync::Arc;

            use torrust_tracker_primitives::NumberOfDownloadsBTreeMap;

            use crate::swarm::registry::Registry;
            use crate::tests::{leecher, sample_info_hash};

            #[tokio::test]
            async fn it_should_allow_importing_persisted_torrent_entries() {
                let swarms = Arc::new(Registry::default());

                let infohash = sample_info_hash();

                let mut persistent_torrents = NumberOfDownloadsBTreeMap::default();

                persistent_torrents.insert(infohash, 1);

                swarms.import_persistent(&persistent_torrents);

                let swarm_metadata = swarms.get_swarm_metadata_or_default(&infohash).await.unwrap();

                // Only the number of downloads is persisted.
                assert_eq!(swarm_metadata.downloaded, 1);
            }

            #[tokio::test]
            async fn it_should_allow_overwriting_a_previously_imported_persisted_torrent() {
                // code-review: do we want to allow this?

                let swarms = Arc::new(Registry::default());

                let infohash = sample_info_hash();

                let mut persistent_torrents = NumberOfDownloadsBTreeMap::default();

                persistent_torrents.insert(infohash, 1);
                persistent_torrents.insert(infohash, 2);

                swarms.import_persistent(&persistent_torrents);

                let swarm_metadata = swarms.get_swarm_metadata_or_default(&infohash).await.unwrap();

                // It takes the last value
                assert_eq!(swarm_metadata.downloaded, 2);
            }

            #[tokio::test]
            async fn it_should_now_allow_importing_a_persisted_torrent_if_it_already_exists() {
                let swarms = Arc::new(Registry::default());

                let infohash = sample_info_hash();

                // Insert a new the torrent entry
                swarms.handle_announcement(&infohash, &leecher(), None).await.unwrap();
                let initial_number_of_downloads = swarms.get_swarm_metadata_or_default(&infohash).await.unwrap().downloaded;

                // Try to import the torrent entry
                let new_number_of_downloads = initial_number_of_downloads + 1;
                let mut persistent_torrents = NumberOfDownloadsBTreeMap::default();
                persistent_torrents.insert(infohash, new_number_of_downloads);
                swarms.import_persistent(&persistent_torrents);

                // The number of downloads should not be changed
                assert_eq!(
                    swarms.get_swarm_metadata_or_default(&infohash).await.unwrap().downloaded,
                    initial_number_of_downloads
                );
            }
        }
    }

    mod triggering_events {

        use std::sync::Arc;

        use torrust_tracker_primitives::peer::fixture::PeerBuilder;
        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use crate::event::sender::tests::{expect_event_sequence, MockEventSender};
        use crate::event::Event;
        use crate::swarm::registry::Registry;
        use crate::tests::sample_info_hash;

        #[tokio::test]
        async fn it_should_trigger_an_event_when_a_new_torrent_is_added() {
            let info_hash = sample_info_hash();
            let peer = PeerBuilder::leecher().build();

            let mut event_sender_mock = MockEventSender::new();

            expect_event_sequence(
                &mut event_sender_mock,
                vec![
                    Event::TorrentAdded {
                        info_hash,
                        announcement: peer,
                    },
                    Event::PeerAdded { info_hash, peer },
                ],
            );

            let swarms = Registry::new(Some(Arc::new(event_sender_mock)));

            swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();
        }

        #[tokio::test]
        async fn it_should_trigger_an_event_when_a_torrent_is_directly_removed() {
            let info_hash = sample_info_hash();
            let peer = PeerBuilder::leecher().build();

            let mut event_sender_mock = MockEventSender::new();

            expect_event_sequence(
                &mut event_sender_mock,
                vec![
                    Event::TorrentAdded {
                        info_hash,
                        announcement: peer,
                    },
                    Event::PeerAdded { info_hash, peer },
                    Event::TorrentRemoved { info_hash },
                ],
            );

            let swarms = Registry::new(Some(Arc::new(event_sender_mock)));

            swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

            swarms.remove(&info_hash).await.unwrap();
        }

        #[tokio::test]
        async fn it_should_trigger_an_event_when_a_peerless_torrent_is_removed() {
            let info_hash = sample_info_hash();
            let peer = PeerBuilder::leecher().build();

            let mut event_sender_mock = MockEventSender::new();

            expect_event_sequence(
                &mut event_sender_mock,
                vec![
                    Event::TorrentAdded {
                        info_hash,
                        announcement: peer,
                    },
                    Event::PeerAdded { info_hash, peer },
                    Event::PeerRemoved { info_hash, peer },
                    Event::TorrentRemoved { info_hash },
                ],
            );

            let swarms = Registry::new(Some(Arc::new(event_sender_mock)));

            // Add the new torrent
            swarms.handle_announcement(&info_hash, &peer, None).await.unwrap();

            // Remove the peer
            let current_cutoff = peer.updated + DurationSinceUnixEpoch::from_secs(1);
            swarms.remove_inactive_peers(current_cutoff).await.unwrap();

            // Remove peerless torrents

            let tracker_policy = torrust_tracker_configuration::TrackerPolicy {
                remove_peerless_torrents: true,
                ..Default::default()
            };

            swarms.remove_peerless_torrents(&tracker_policy).await.unwrap();
        }
    }
}
