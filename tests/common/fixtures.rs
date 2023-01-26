use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use torrust_tracker::protocol::clock::DurationSinceUnixEpoch;
use torrust_tracker::tracker::peer::{self, Id, Peer};

pub struct PeerBuilder {
    peer: Peer,
}

impl PeerBuilder {
    pub fn default() -> PeerBuilder {
        Self {
            peer: default_peer_for_testing(),
        }
    }

    #[allow(dead_code)]
    pub fn with_peer_id(mut self, peer_id: &Id) -> Self {
        self.peer.peer_id = *peer_id;
        self
    }

    pub fn into(self) -> Peer {
        self.peer
    }
}

fn default_peer_for_testing() -> Peer {
    Peer {
        peer_id: peer::Id(*b"-qB00000000000000000"),
        peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
        updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
        uploaded: NumberOfBytes(0),
        downloaded: NumberOfBytes(0),
        left: NumberOfBytes(0),
        event: AnnounceEvent::Started,
    }
}

pub fn invalid_info_hashes() -> Vec<String> {
    [
        "0".to_string(),
        "-1".to_string(),
        "1.1".to_string(),
        "INVALID INFOHASH".to_string(),
        "9c38422213e30bff212b30c360d26f9a0213642".to_string(), // 39-char length instead of 40
        "9c38422213e30bff212b30c360d26f9a0213642&".to_string(), // Invalid char
    ]
    .to_vec()
}