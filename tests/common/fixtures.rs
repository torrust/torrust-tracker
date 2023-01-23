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
