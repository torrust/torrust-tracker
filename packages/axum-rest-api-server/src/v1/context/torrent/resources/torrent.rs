//! `Torrent` and `ListItem` API resources.
//!
//! Re-exports the protocol DTOs plus domain-conversion utilities.
//!
//! - `Torrent` is the full torrent resource.
//! - `ListItem` is a list item resource on a torrent list.
use torrust_tracker_core::torrent::services::{BasicInfo, Info};
pub use torrust_tracker_rest_api_protocol::v1::resources::torrent::{ListItem, Torrent};

use super::peer;

/// Convert a domain [`Info`] into a protocol [`Torrent`].
#[must_use]
pub fn from_domain_info(info: Info) -> Torrent {
    let peers: Option<peer::Vector> = info.peers.map(|peers| peers.into_iter().collect());

    let peers: Option<Vec<torrust_tracker_rest_api_protocol::v1::resources::peer::Peer>> = peers.map(|peers| peers.0);

    Torrent {
        info_hash: info.info_hash.to_string(),
        seeders: info.seeders,
        completed: info.completed,
        leechers: info.leechers,
        peers,
    }
}

/// Build a [`ListItem`] from a domain [`BasicInfo`].
#[must_use]
pub fn list_item_from_domain(basic_info: &BasicInfo) -> ListItem {
    ListItem {
        info_hash: basic_info.info_hash.to_string(),
        seeders: basic_info.seeders,
        completed: basic_info.completed,
        leechers: basic_info.leechers,
    }
}

/// Build a vector of [`ListItem`] from domain [`BasicInfo`] slices.
#[must_use]
pub fn list_items_from_domain(basic_info_vec: &[BasicInfo]) -> Vec<ListItem> {
    basic_info_vec.iter().map(list_item_from_domain).collect()
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::str::FromStr;

    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_info_hash::InfoHash;
    use torrust_tracker_core::torrent::services::{BasicInfo, Info};
    use torrust_tracker_primitives::{AnnounceEvent, NumberOfBytes, PeerId, peer};

    use super::peer::from_domain_peer;
    use super::{from_domain_info, list_item_from_domain};
    use crate::v1::context::torrent::resources::torrent::{ListItem, Torrent};

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
            from_domain_info(Info {
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
                peers: Some(vec![from_domain_peer(sample_peer())]),
            }
        );
    }

    #[test]
    fn torrent_resource_list_item_should_be_converted_from_the_basic_torrent_info() {
        assert_eq!(
            list_item_from_domain(&BasicInfo {
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
