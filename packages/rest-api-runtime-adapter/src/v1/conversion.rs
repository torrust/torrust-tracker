//! Conversions from domain types to protocol DTOs.
//!
//! These functions bridge tracker-internal domain types (e.g., `Info`,
//! `BasicInfo`, `peer::Peer`) with transport-agnostic protocol DTOs.
use torrust_tracker_core::torrent::services::{BasicInfo, Info};
use torrust_tracker_primitives::{PeerId, peer as domain_peer};
use torrust_tracker_rest_api_protocol::v1::context::torrent::resources::peer as protocol_peer;
use torrust_tracker_rest_api_protocol::v1::context::torrent::resources::torrent::{ListItem, Torrent};

/// Convert a domain [`domain_peer::Peer`] into a protocol [`protocol_peer::Peer`].
#[must_use]
pub fn from_domain_peer(value: domain_peer::Peer) -> protocol_peer::Peer {
    #[allow(deprecated)]
    protocol_peer::Peer {
        peer_id: from_domain_peer_id(value.peer_id),
        peer_addr: value.peer_addr.to_string(),
        updated: value.updated.as_millis(),
        updated_milliseconds_ago: value.updated.as_millis(),
        uploaded: value.uploaded.0,
        downloaded: value.downloaded.0,
        left: value.left.0,
        event: format!("{:?}", value.event),
    }
}

/// Convert a domain [`PeerId`] into a protocol [`protocol_peer::Id`].
#[must_use]
pub fn from_domain_peer_id(peer_id: PeerId) -> protocol_peer::Id {
    let pid = domain_peer::Id::from(peer_id);
    protocol_peer::Id {
        id: pid.to_hex_string(),
        client: pid.get_client_name(),
    }
}

/// Convert a domain [`Info`] into a protocol [`Torrent`].
#[must_use]
pub fn from_domain_info(info: Info) -> Torrent {
    let peers: Option<Vec<protocol_peer::Peer>> = info.peers.map(|peers| peers.into_iter().map(from_domain_peer).collect());

    Torrent {
        info_hash: info.info_hash.to_string(),
        seeders: info.seeders,
        completed: info.completed,
        leechers: info.leechers,
        peers,
    }
}

/// Build a vector of [`ListItem`] from domain [`BasicInfo`] slices.
#[must_use]
pub fn list_items_from_domain(basic_info_vec: &[BasicInfo]) -> Vec<ListItem> {
    basic_info_vec.iter().map(list_item_from_domain).collect()
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

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::str::FromStr;

    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_info_hash::InfoHash;
    use torrust_tracker_core::torrent::services::{BasicInfo, Info};
    use torrust_tracker_primitives::{AnnounceEvent, NumberOfBytes, PeerId, peer};
    use torrust_tracker_rest_api_protocol::v1::context::torrent::resources::torrent::{ListItem, Torrent};

    use super::*;

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
