use std::sync::Arc;

use serde::Deserialize;

use crate::protocol::info_hash::InfoHash;
use crate::tracker::peer::Peer;
use crate::tracker::Tracker;

#[derive(Debug, PartialEq)]
pub struct Info {
    pub info_hash: InfoHash,
    pub seeders: u64,
    pub completed: u64,
    pub leechers: u64,
    pub peers: Option<Vec<Peer>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BasicInfo {
    pub info_hash: InfoHash,
    pub seeders: u64,
    pub completed: u64,
    pub leechers: u64,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub offset: u32,
    pub limit: u32,
}

impl Pagination {
    #[must_use]
    pub fn new(offset: u32, limit: u32) -> Self {
        Self { offset, limit }
    }

    #[must_use]
    pub fn new_with_options(offset_option: Option<u32>, limit_option: Option<u32>) -> Self {
        let offset = match offset_option {
            Some(offset) => offset,
            None => Pagination::default_offset(),
        };
        let limit = match limit_option {
            Some(offset) => offset,
            None => Pagination::default_limit(),
        };

        Self { offset, limit }
    }

    #[must_use]
    pub fn default_offset() -> u32 {
        0
    }

    #[must_use]
    pub fn default_limit() -> u32 {
        4000
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: Self::default_offset(),
            limit: Self::default_limit(),
        }
    }
}

pub async fn get_torrent_info(tracker: Arc<Tracker>, info_hash: &InfoHash) -> Option<Info> {
    let db = tracker.get_torrents().await;

    let torrent_entry_option = db.get(info_hash);

    let torrent_entry = match torrent_entry_option {
        Some(torrent_entry) => torrent_entry,
        None => {
            return None;
        }
    };

    let (seeders, completed, leechers) = torrent_entry.get_stats();

    let peers = torrent_entry.get_peers(None);

    let peers = Some(peers.iter().map(|peer| (**peer)).collect());

    Some(Info {
        info_hash: *info_hash,
        seeders: u64::from(seeders),
        completed: u64::from(completed),
        leechers: u64::from(leechers),
        peers,
    })
}

pub async fn get_torrents(tracker: Arc<Tracker>, pagination: &Pagination) -> Vec<BasicInfo> {
    let db = tracker.get_torrents().await;

    db.iter()
        .map(|(info_hash, torrent_entry)| {
            let (seeders, completed, leechers) = torrent_entry.get_stats();
            BasicInfo {
                info_hash: *info_hash,
                seeders: u64::from(seeders),
                completed: u64::from(completed),
                leechers: u64::from(leechers),
            }
        })
        .skip(pagination.offset as usize)
        .take(pagination.limit as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};

    use crate::protocol::clock::DurationSinceUnixEpoch;
    use crate::tracker::peer;

    fn sample_peer() -> peer::Peer {
        peer::Peer {
            peer_id: peer::Id(*b"-qB00000000000000000"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes(0),
            downloaded: NumberOfBytes(0),
            left: NumberOfBytes(0),
            event: AnnounceEvent::Started,
        }
    }

    mod getting_a_torrent_info {

        use std::str::FromStr;
        use std::sync::Arc;
        use torrust_tracker_configuration::{Configuration, ephemeral_configuration};

        use crate::protocol::info_hash::InfoHash;
        use crate::tracker::services::common::tracker_factory;
        use crate::tracker::services::torrent::tests::sample_peer;
        use crate::tracker::services::torrent::{get_torrent_info, Info};

        pub fn tracker_configuration() -> Arc<Configuration> {
            Arc::new(ephemeral_configuration())
        }

        #[tokio::test]
        async fn should_return_none_if_the_tracker_does_not_have_the_torrent() {
            let tracker = Arc::new(tracker_factory(&tracker_configuration()));

            let torrent_info = get_torrent_info(
                tracker.clone(),
                &InfoHash::from_str("0b3aea4adc213ce32295be85d3883a63bca25446").unwrap(),
            )
            .await;

            assert!(torrent_info.is_none());
        }

        #[tokio::test]
        async fn should_return_the_torrent_info_if_the_tracker_has_the_torrent() {
            let tracker = Arc::new(tracker_factory(&tracker_configuration()));

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash = InfoHash::from_str(&hash).unwrap();
            tracker
                .update_torrent_with_peer_and_get_stats(&info_hash, &sample_peer())
                .await;

            let torrent_info = get_torrent_info(tracker.clone(), &info_hash).await.unwrap();

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
        use torrust_tracker_configuration::{Configuration, ephemeral_configuration};

        use crate::protocol::info_hash::InfoHash;
        use crate::tracker::services::common::tracker_factory;
        use crate::tracker::services::torrent::tests::sample_peer;
        use crate::tracker::services::torrent::{get_torrents, BasicInfo, Pagination};

        pub fn tracker_configuration() -> Arc<Configuration> {
            Arc::new(ephemeral_configuration())
        }

        #[tokio::test]
        async fn should_return_an_empty_result_if_the_tracker_does_not_have_any_torrent() {
            let tracker = Arc::new(tracker_factory(&tracker_configuration()));

            let torrents = get_torrents(tracker.clone(), &Pagination::default()).await;

            assert_eq!(torrents, vec![]);
        }

        #[tokio::test]
        async fn should_return_a_summarized_info_for_all_torrents() {
            let tracker = Arc::new(tracker_factory(&tracker_configuration()));

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash = InfoHash::from_str(&hash).unwrap();

            tracker
                .update_torrent_with_peer_and_get_stats(&info_hash, &sample_peer())
                .await;

            let torrents = get_torrents(tracker.clone(), &Pagination::default()).await;

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
        async fn should_allow_limiting_the_number_of_torrents_in_the_result() {
            let tracker = Arc::new(tracker_factory(&tracker_configuration()));

            let hash1 = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash1 = InfoHash::from_str(&hash1).unwrap();
            let hash2 = "03840548643af2a7b63a9f5cbca348bc7150ca3a".to_owned();
            let info_hash2 = InfoHash::from_str(&hash2).unwrap();

            tracker
                .update_torrent_with_peer_and_get_stats(&info_hash1, &sample_peer())
                .await;
            tracker
                .update_torrent_with_peer_and_get_stats(&info_hash2, &sample_peer())
                .await;

            let offset = 0;
            let limit = 1;

            let torrents = get_torrents(tracker.clone(), &Pagination::new(offset, limit)).await;

            assert_eq!(torrents.len(), 1);
        }

        #[tokio::test]
        async fn should_allow_using_pagination_in_the_result() {
            let tracker = Arc::new(tracker_factory(&tracker_configuration()));

            let hash1 = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash1 = InfoHash::from_str(&hash1).unwrap();
            let hash2 = "03840548643af2a7b63a9f5cbca348bc7150ca3a".to_owned();
            let info_hash2 = InfoHash::from_str(&hash2).unwrap();

            tracker
                .update_torrent_with_peer_and_get_stats(&info_hash1, &sample_peer())
                .await;
            tracker
                .update_torrent_with_peer_and_get_stats(&info_hash2, &sample_peer())
                .await;

            let offset = 1;
            let limit = 4000;

            let torrents = get_torrents(tracker.clone(), &Pagination::new(offset, limit)).await;

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
        async fn should_return_torrents_ordered_by_info_hash() {
            let tracker = Arc::new(tracker_factory(&tracker_configuration()));

            let hash1 = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash1 = InfoHash::from_str(&hash1).unwrap();
            tracker
                .update_torrent_with_peer_and_get_stats(&info_hash1, &sample_peer())
                .await;

            let hash2 = "03840548643af2a7b63a9f5cbca348bc7150ca3a".to_owned();
            let info_hash2 = InfoHash::from_str(&hash2).unwrap();
            tracker
                .update_torrent_with_peer_and_get_stats(&info_hash2, &sample_peer())
                .await;

            let torrents = get_torrents(tracker.clone(), &Pagination::default()).await;

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
}
