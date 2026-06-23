//! `Peer` and Peer `Id` API resources.
//!
//! Re-exports the protocol DTOs plus domain-conversion utilities.
use derive_more::From;
use torrust_tracker_primitives::{PeerId, peer};
pub use torrust_tracker_rest_api_protocol::v1::resources::peer::{Id, Peer};

/// Convert a domain [`peer::Peer`] into a protocol [`Peer`].
#[must_use]
pub fn from_domain_peer(value: peer::Peer) -> Peer {
    #[allow(deprecated)]
    Peer {
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

/// Convert a domain [`PeerId`] into a protocol [`Id`].
#[must_use]
pub fn from_domain_peer_id(peer_id: PeerId) -> Id {
    let pid = peer::Id::from(peer_id);
    Id {
        id: pid.to_hex_string(),
        client: pid.get_client_name(),
    }
}

/// A newtype vector of [`Peer`] for collecting from iterators.
#[derive(From, PartialEq, Default)]
pub struct Vector(pub Vec<Peer>);

impl FromIterator<peer::Peer> for Vector {
    fn from_iter<T: IntoIterator<Item = peer::Peer>>(iter: T) -> Self {
        let mut peers = Vector::default();

        for i in iter {
            peers.0.push(from_domain_peer(i));
        }
        peers
    }
}
