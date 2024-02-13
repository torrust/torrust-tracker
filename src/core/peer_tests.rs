#![cfg(test)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use torrust_tracker_primitives::announce_event::AnnounceEvent;
use torrust_tracker_primitives::{peer, NumberOfBytes};

use crate::shared::clock::{self, Time};

#[test]
fn it_should_be_serializable() {
    let torrent_peer = peer::Peer {
        peer_id: peer::Id(*b"-qB0000-000000000000"),
        peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
        updated: clock::Current::now(),
        uploaded: NumberOfBytes(0),
        downloaded: NumberOfBytes(0),
        left: NumberOfBytes(0),
        event: AnnounceEvent::Started,
    };

    let raw_json = serde_json::to_string(&torrent_peer).unwrap();

    let expected_raw_json = r#"
            {
                "peer_id": {
                    "id": "0x2d7142303030302d303030303030303030303030",
                    "client": "qBittorrent"
                },
                "peer_addr":"126.0.0.1:8080",
                "updated":0,
                "uploaded":0,
                "downloaded":0,
                "left":0,
                "event":"Started"
            }
        "#;

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&raw_json).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected_raw_json).unwrap()
    );
}
