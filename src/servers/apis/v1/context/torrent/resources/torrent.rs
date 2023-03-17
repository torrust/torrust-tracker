use serde::{Deserialize, Serialize};

use super::peer;
use crate::tracker::services::torrent::{BasicInfo, Info};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Torrent {
    pub info_hash: String,
    pub seeders: u64,
    pub completed: u64,
    pub leechers: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<super::peer::Peer>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ListItem {
    pub info_hash: String,
    pub seeders: u64,
    pub completed: u64,
    pub leechers: u64,
    // todo: this is always None. Remove field from endpoint?
    pub peers: Option<Vec<super::peer::Peer>>,
}

impl ListItem {
    #[must_use]
    pub fn new_vec(basic_info_vec: &[BasicInfo]) -> Vec<Self> {
        basic_info_vec
            .iter()
            .map(|basic_info| ListItem::from((*basic_info).clone()))
            .collect()
    }
}

#[must_use]
pub fn to_resource(basic_info_vec: &[BasicInfo]) -> Vec<ListItem> {
    basic_info_vec
        .iter()
        .map(|basic_info| ListItem::from((*basic_info).clone()))
        .collect()
}

impl From<Info> for Torrent {
    fn from(info: Info) -> Self {
        Self {
            info_hash: info.info_hash.to_string(),
            seeders: info.seeders,
            completed: info.completed,
            leechers: info.leechers,
            peers: info
                .peers
                .map(|peers| peers.iter().map(|peer| peer::Peer::from(*peer)).collect()),
        }
    }
}

impl From<BasicInfo> for ListItem {
    fn from(basic_info: BasicInfo) -> Self {
        Self {
            info_hash: basic_info.info_hash.to_string(),
            seeders: basic_info.seeders,
            completed: basic_info.completed,
            leechers: basic_info.leechers,
            peers: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::str::FromStr;

    use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};

    use super::Torrent;
    use crate::servers::apis::v1::context::torrent::resources::peer::Peer;
    use crate::servers::apis::v1::context::torrent::resources::torrent::ListItem;
    use crate::shared::bit_torrent::info_hash::InfoHash;
    use crate::shared::clock::DurationSinceUnixEpoch;
    use crate::tracker::peer;
    use crate::tracker::services::torrent::{BasicInfo, Info};

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

    #[test]
    fn torrent_resource_should_be_converted_from_torrent_info() {
        assert_eq!(
            Torrent::from(Info {
                info_hash: InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap(),
                seeders: 1,
                completed: 2,
                leechers: 3,
                peers: Some(vec![sample_peer()]),
            }),
            Torrent {
                info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                seeders: 1,
                completed: 2,
                leechers: 3,
                peers: Some(vec![Peer::from(sample_peer())]),
            }
        );
    }

    #[test]
    fn torrent_resource_list_item_should_be_converted_from_the_basic_torrent_info() {
        assert_eq!(
            ListItem::from(BasicInfo {
                info_hash: InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap(),
                seeders: 1,
                completed: 2,
                leechers: 3,
            }),
            ListItem {
                info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                seeders: 1,
                completed: 2,
                leechers: 3,
                peers: None,
            }
        );
    }
}
