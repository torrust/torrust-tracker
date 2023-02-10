use std::net::{IpAddr, SocketAddr};

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};

use super::request::Announce;
use crate::protocol::clock::{Current, Time};
use crate::tracker::peer::Peer;

#[must_use]
pub fn from_request(announce_request: &Announce, peer_ip: &IpAddr) -> Peer {
    let event: AnnounceEvent = if let Some(event) = &announce_request.event {
        match event.as_ref() {
            "started" => AnnounceEvent::Started,
            "stopped" => AnnounceEvent::Stopped,
            "completed" => AnnounceEvent::Completed,
            _ => AnnounceEvent::None,
        }
    } else {
        AnnounceEvent::None
    };

    #[allow(clippy::cast_possible_truncation)]
    Peer {
        peer_id: announce_request.peer_id,
        peer_addr: SocketAddr::new(*peer_ip, announce_request.port),
        updated: Current::now(),
        uploaded: NumberOfBytes(i128::from(announce_request.uploaded) as i64),
        downloaded: NumberOfBytes(i128::from(announce_request.downloaded) as i64),
        left: NumberOfBytes(i128::from(announce_request.left) as i64),
        event,
    }
}
