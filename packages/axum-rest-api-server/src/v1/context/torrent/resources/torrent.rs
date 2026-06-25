//! `Torrent` and `ListItem` API resources.
//!
//! Protocol DTOs are defined in `torrust-tracker-rest-api-protocol`.
//! This module only contains unit tests for domain→DTO conversions.

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::str::FromStr;

    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_info_hash::InfoHash;
    use torrust_tracker_core::torrent::services::{BasicInfo, Info};
    use torrust_tracker_primitives::{AnnounceEvent, NumberOfBytes, PeerId, peer};
    use torrust_tracker_rest_api_protocol::v1::context::torrent::resources::torrent::{ListItem, Torrent};
    use torrust_tracker_rest_api_runtime_adapter::conversion;

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

    #[test]
    fn torrent_resource_should_be_converted_from_torrent_info() {
        assert_eq!(
            conversion::from_domain_info(Info {
                info_hash: InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap(), // DevSkim: ignore DS173237
                seeders: 1,
                completed: 2,
                leechers: 3,
                peers: Some(vec![sample_peer()]),
            }),
            Torrent {
                info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(), // DevSkim: ignore DS173237
                seeders: 1,
                completed: 2,
                leechers: 3,
                peers: Some(vec![conversion::from_domain_peer(sample_peer())]),
            }
        );
    }

    #[test]
    fn torrent_resource_list_item_should_be_converted_from_the_basic_torrent_info() {
        assert_eq!(
            conversion::list_item_from_domain(&BasicInfo {
                info_hash: InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap(), // DevSkim: ignore DS173237
                seeders: 1,
                completed: 2,
                leechers: 3,
            }),
            ListItem {
                info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(), // DevSkim: ignore DS173237
                seeders: 1,
                completed: 2,
                leechers: 3,
            }
        );
    }
}
