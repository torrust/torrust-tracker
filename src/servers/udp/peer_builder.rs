use std::net::{IpAddr, SocketAddr};

use super::request::AnnounceWrapper;
use crate::protocol::clock::{Current, Time};
use crate::tracker::peer::{Id, Peer};

#[must_use]
pub fn from_request(announce_wrapper: &AnnounceWrapper, peer_ip: &IpAddr) -> Peer {
    Peer {
        peer_id: Id(announce_wrapper.announce_request.peer_id.0),
        peer_addr: SocketAddr::new(*peer_ip, announce_wrapper.announce_request.port.0),
        updated: Current::now(),
        uploaded: announce_wrapper.announce_request.bytes_uploaded,
        downloaded: announce_wrapper.announce_request.bytes_downloaded,
        left: announce_wrapper.announce_request.bytes_left,
        event: announce_wrapper.announce_request.event,
    }
}
