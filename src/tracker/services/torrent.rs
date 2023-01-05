use std::sync::Arc;

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

#[cfg(test)]
mod tests {

    mod getting_a_torrent_info {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::str::FromStr;
        use std::sync::Arc;

        use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};

        use crate::config::{ephemeral_configuration, Configuration};
        use crate::protocol::clock::DurationSinceUnixEpoch;
        use crate::protocol::info_hash::InfoHash;
        use crate::tracker::peer;
        use crate::tracker::services::common::tracker_factory;
        use crate::tracker::services::torrent::{get_torrent_info, Info};

        pub fn tracker_configuration() -> Arc<Configuration> {
            Arc::new(ephemeral_configuration())
        }

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

            let torrent_info = get_torrent_info(tracker.clone(), &InfoHash::from_str(&hash).unwrap())
                .await
                .unwrap();

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
}
