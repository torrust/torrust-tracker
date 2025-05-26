//! Core tracker domain services.
//!
//! This module defines the primary services for retrieving torrent-related data
//! from the tracker. There are two main services:
//!
//! - [`get_torrent_info`]: Returns all available data (including the list of
//!   peers) about a single torrent.
//! - [`get_torrents_page`] and [`get_torrents`]: Return summarized data about
//!   multiple torrents, excluding the peer list.
//!
//! The full torrent info is represented by the [`Info`] struct, which includes
//! swarm data (peer list) and aggregate metrics. The [`BasicInfo`] struct
//! provides similar data but without the list of peers, making it suitable for
//! bulk queries.
use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::peer;

use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

/// Full torrent information, including swarm (peer) details.
///
/// This struct contains all the information that the tracker holds about a
/// torrent, including the infohash, aggregate swarm metrics (seeders, leechers,
/// completed downloads), and the complete list of peers in the swarm.
#[derive(Debug, PartialEq)]
pub struct Info {
    /// The infohash of the torrent this data is related to
    pub info_hash: InfoHash,

    /// The total number of seeders for this torrent. Peer that actively serving
    /// a full copy of the torrent data
    pub seeders: u64,

    /// The total number of peers that have ever complete downloading this
    /// torrent
    pub completed: u64,

    /// The total number of leechers for this torrent. Peers that actively
    /// downloading this torrent
    pub leechers: u64,

    /// The swarm: the list of peers that are actively trying to download or
    /// serving this torrent
    pub peers: Option<Vec<peer::Peer>>,
}

/// Basic torrent information, excluding the list of peers.
///
/// This struct contains the same aggregate metrics as [`Info`] (infohash,
/// seeders, completed, leechers) but omits the peer list. It is used when only
/// summary information is needed.
#[derive(Debug, PartialEq, Clone)]
pub struct BasicInfo {
    /// The infohash of the torrent this data is related to
    pub info_hash: InfoHash,

    /// The total number of seeders for this torrent. Peer that actively serving
    /// a full copy of the torrent data
    pub seeders: u64,

    /// The total number of peers that have ever complete downloading this
    /// torrent
    pub completed: u64,

    /// The total number of leechers for this torrent. Peers that actively
    /// downloading this torrent
    pub leechers: u64,
}

/// Retrieves complete torrent information for a given torrent.
///
/// This function queries the in-memory torrent repository for a torrent entry
/// matching the provided infohash. If found, it extracts the swarm metadata
/// (aggregate metrics) and the current list of peers, and returns an [`Info`]
/// struct.
///
/// # Arguments
///
/// * `in_memory_torrent_repository` - A shared reference to the in-memory
///   torrent repository.
/// * `info_hash` - A reference to the torrent's infohash.
///
/// # Returns
///
/// An [`Option<Info>`] which is:
/// - `Some(Info)` if the torrent exists in the repository.
/// - `None` if the torrent is not found.
///
/// # Panics
///
/// This function panics if the lock for the torrent entry cannot be obtained.
#[must_use]
pub async fn get_torrent_info(
    in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>,
    info_hash: &InfoHash,
) -> Option<Info> {
    let torrent_entry_option = in_memory_torrent_repository.get(info_hash);

    let torrent_entry = torrent_entry_option?;

    let stats = torrent_entry.lock().await.metadata();

    let peers = torrent_entry.lock().await.peers(None);

    let peers = Some(peers.iter().map(|peer| (**peer)).collect());

    Some(Info {
        info_hash: *info_hash,
        seeders: u64::from(stats.complete),
        completed: u64::from(stats.downloaded),
        leechers: u64::from(stats.incomplete),
        peers,
    })
}

/// Retrieves summarized torrent information for a paginated set of torrents.
///
/// This function returns a vector of [`BasicInfo`] structures for torrents in
/// the repository, according to the provided pagination parameters. The
/// returned data excludes the peer list, providing only aggregate metrics.
///
/// # Arguments
///
/// * `in_memory_torrent_repository` - A shared reference to the in-memory
///   torrent repository.
/// * `pagination` - An optional reference to a [`Pagination`] object specifying
///   offset and limit.
///
/// # Returns
///
/// A vector of [`BasicInfo`] structs representing the summarized data of the
/// torrents.
///
/// # Panics
///
/// This function panics if the lock for the torrent entry cannot be obtained.
#[must_use]
pub async fn get_torrents_page(
    in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>,
    pagination: Option<&Pagination>,
) -> Vec<BasicInfo> {
    let mut basic_infos: Vec<BasicInfo> = vec![];

    for (info_hash, torrent_entry) in in_memory_torrent_repository.get_paginated(pagination) {
        let stats = torrent_entry.lock().await.metadata();

        basic_infos.push(BasicInfo {
            info_hash,
            seeders: u64::from(stats.complete),
            completed: u64::from(stats.downloaded),
            leechers: u64::from(stats.incomplete),
        });
    }

    basic_infos
}

/// Retrieves summarized torrent information for a specified list of torrents.
///
/// This function iterates over a slice of infohashes, fetches the corresponding
/// swarm metadata from the in-memory repository (if available), and returns a
/// vector of [`BasicInfo`] structs. This function is useful for bulk queries
/// where detailed peer information is not required.
///
/// # Arguments
///
/// * `in_memory_torrent_repository` - A shared reference to the in-memory
///   torrent repository.
/// * `info_hashes` - A slice of infohashes for which to retrieve the torrent
///   information.
///
/// # Returns
///
/// A vector of [`BasicInfo`] structs for the requested torrents.
///
/// # Panics
///
/// This function panics if the lock for the torrent entry cannot be obtained.
#[must_use]
pub async fn get_torrents(
    in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>,
    info_hashes: &[InfoHash],
) -> Vec<BasicInfo> {
    let mut basic_infos: Vec<BasicInfo> = vec![];

    for info_hash in info_hashes {
        if let Some(torrent_entry) = in_memory_torrent_repository.get(info_hash) {
            let metadata = torrent_entry.lock().await.metadata();

            basic_infos.push(BasicInfo {
                info_hash: *info_hash,
                seeders: u64::from(metadata.complete),
                completed: u64::from(metadata.downloaded),
                leechers: u64::from(metadata.incomplete),
            });
        }
    }

    basic_infos
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
    use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};

    fn sample_peer() -> peer::Peer {
        peer::Peer {
            peer_id: PeerId(*b"-qB00000000000000000"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes::new(0),
            downloaded: NumberOfBytes::new(0),
            left: NumberOfBytes::new(0),
            event: AnnounceEvent::Started,
        }
    }

    mod getting_a_torrent_info {

        use std::str::FromStr;
        use std::sync::Arc;

        use bittorrent_primitives::info_hash::InfoHash;

        use crate::torrent::repository::in_memory::InMemoryTorrentRepository;
        use crate::torrent::services::tests::sample_peer;
        use crate::torrent::services::{get_torrent_info, Info};

        #[tokio::test]
        async fn it_should_return_none_if_the_tracker_does_not_have_the_torrent() {
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

            let torrent_info = get_torrent_info(
                &in_memory_torrent_repository,
                &InfoHash::from_str("0b3aea4adc213ce32295be85d3883a63bca25446").unwrap(), // DevSkim: ignore DS173237
            )
            .await;

            assert!(torrent_info.is_none());
        }

        #[tokio::test]
        async fn it_should_return_the_torrent_info_if_the_tracker_has_the_torrent() {
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237
            let info_hash = InfoHash::from_str(&hash).unwrap();
            in_memory_torrent_repository
                .handle_announcement(&info_hash, &sample_peer(), None)
                .await;

            let torrent_info = get_torrent_info(&in_memory_torrent_repository, &info_hash).await.unwrap();

            assert_eq!(
                torrent_info,
                Info {
                    info_hash: InfoHash::from_str(&hash).unwrap(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: Some(vec![sample_peer()]),
                }
            );
        }
    }

    mod searching_for_torrents {

        use std::str::FromStr;
        use std::sync::Arc;

        use bittorrent_primitives::info_hash::InfoHash;

        use crate::torrent::repository::in_memory::InMemoryTorrentRepository;
        use crate::torrent::services::tests::sample_peer;
        use crate::torrent::services::{get_torrents_page, BasicInfo, Pagination};

        #[tokio::test]
        async fn it_should_return_an_empty_result_if_the_tracker_does_not_have_any_torrent() {
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

            let torrents = get_torrents_page(&in_memory_torrent_repository, Some(&Pagination::default())).await;

            assert_eq!(torrents, vec![]);
        }

        #[tokio::test]
        async fn it_should_return_a_summarized_info_for_all_torrents() {
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237
            let info_hash = InfoHash::from_str(&hash).unwrap();

            in_memory_torrent_repository
                .handle_announcement(&info_hash, &sample_peer(), None)
                .await;

            let torrents = get_torrents_page(&in_memory_torrent_repository, Some(&Pagination::default())).await;

            assert_eq!(
                torrents,
                vec![BasicInfo {
                    info_hash: InfoHash::from_str(&hash).unwrap(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                }]
            );
        }

        #[tokio::test]
        async fn it_should_allow_limiting_the_number_of_torrents_in_the_result() {
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

            let hash1 = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237
            let info_hash1 = InfoHash::from_str(&hash1).unwrap();

            let hash2 = "03840548643af2a7b63a9f5cbca348bc7150ca3a".to_owned(); // DevSkim: ignore DS173237
            let info_hash2 = InfoHash::from_str(&hash2).unwrap();

            in_memory_torrent_repository
                .handle_announcement(&info_hash1, &sample_peer(), None)
                .await;
            in_memory_torrent_repository
                .handle_announcement(&info_hash2, &sample_peer(), None)
                .await;

            let offset = 0;
            let limit = 1;

            let torrents = get_torrents_page(&in_memory_torrent_repository, Some(&Pagination::new(offset, limit))).await;

            assert_eq!(torrents.len(), 1);
        }

        #[tokio::test]
        async fn it_should_allow_using_pagination_in_the_result() {
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

            let hash1 = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237
            let info_hash1 = InfoHash::from_str(&hash1).unwrap();

            let hash2 = "03840548643af2a7b63a9f5cbca348bc7150ca3a".to_owned(); // DevSkim: ignore DS173237
            let info_hash2 = InfoHash::from_str(&hash2).unwrap();

            in_memory_torrent_repository
                .handle_announcement(&info_hash1, &sample_peer(), None)
                .await;
            in_memory_torrent_repository
                .handle_announcement(&info_hash2, &sample_peer(), None)
                .await;

            let offset = 1;
            let limit = 4000;

            let torrents = get_torrents_page(&in_memory_torrent_repository, Some(&Pagination::new(offset, limit))).await;

            assert_eq!(torrents.len(), 1);
            assert_eq!(
                torrents,
                vec![BasicInfo {
                    info_hash: InfoHash::from_str(&hash1).unwrap(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                }]
            );
        }

        #[tokio::test]
        async fn it_should_return_torrents_ordered_by_info_hash() {
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

            let hash1 = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237
            let info_hash1 = InfoHash::from_str(&hash1).unwrap();
            in_memory_torrent_repository
                .handle_announcement(&info_hash1, &sample_peer(), None)
                .await;

            let hash2 = "03840548643af2a7b63a9f5cbca348bc7150ca3a".to_owned(); // DevSkim: ignore DS173237
            let info_hash2 = InfoHash::from_str(&hash2).unwrap();
            in_memory_torrent_repository
                .handle_announcement(&info_hash2, &sample_peer(), None)
                .await;

            let torrents = get_torrents_page(&in_memory_torrent_repository, Some(&Pagination::default())).await;

            assert_eq!(
                torrents,
                vec![
                    BasicInfo {
                        info_hash: InfoHash::from_str(&hash2).unwrap(),
                        seeders: 1,
                        completed: 0,
                        leechers: 0,
                    },
                    BasicInfo {
                        info_hash: InfoHash::from_str(&hash1).unwrap(),
                        seeders: 1,
                        completed: 0,
                        leechers: 0,
                    }
                ]
            );
        }
    }

    mod getting_basic_torrent_info_for_multiple_torrents_at_once {

        use std::sync::Arc;

        use crate::test_helpers::tests::sample_info_hash;
        use crate::torrent::repository::in_memory::InMemoryTorrentRepository;
        use crate::torrent::services::tests::sample_peer;
        use crate::torrent::services::{get_torrents, BasicInfo};

        #[tokio::test]
        async fn it_should_return_an_empty_list_if_none_of_the_requested_torrents_is_found() {
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

            let torrent_info = get_torrents(&in_memory_torrent_repository, &[sample_info_hash()]).await;

            assert!(torrent_info.is_empty());
        }

        #[tokio::test]
        async fn it_should_return_a_list_with_basic_info_about_the_requested_torrents() {
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

            let info_hash = sample_info_hash();

            in_memory_torrent_repository
                .handle_announcement(&info_hash, &sample_peer(), None)
                .await;

            let torrent_info = get_torrents(&in_memory_torrent_repository, &[info_hash]).await;

            assert_eq!(
                torrent_info,
                vec!(BasicInfo {
                    info_hash,
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                })
            );
        }
    }
}
