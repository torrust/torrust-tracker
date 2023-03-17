use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use torrust_tracker::shared::clock::DurationSinceUnixEpoch;
use torrust_tracker::tracker::peer::{self, Id, Peer};

pub struct PeerBuilder {
    peer: Peer,
}

impl PeerBuilder {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn with_peer_addr(mut self, peer_addr: &SocketAddr) -> Self {
        self.peer.peer_addr = *peer_addr;
        self
    }

    #[allow(dead_code)]
    pub fn with_bytes_pending_to_download(mut self, left: i64) -> Self {
        self.peer.left = NumberOfBytes(left);
        self
    }

    #[allow(dead_code)]
    pub fn with_no_bytes_pending_to_download(mut self) -> Self {
        self.peer.left = NumberOfBytes(0);
        self
    }

    #[allow(dead_code)]
    pub fn build(self) -> Peer {
        self.into()
    }

    #[allow(dead_code)]
    pub fn into(self) -> Peer {
        self.peer
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
