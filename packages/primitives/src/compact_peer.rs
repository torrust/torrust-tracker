//! Lightweight peer representation for announce responses.
//!
//! [`CompactPeer`] carries only the fields needed by response builders
//! (`peer_id` and `peer_addr`), unlike the full [`peer::Peer`] struct which
//! also carries swarm-management metadata (`updated`, `uploaded`,
//! `downloaded`, `left`, `event`).
//!
//! [`CompactPeer`] is [`Copy`] and stack-only (52 bytes), making it
//! cheaper to pass through the call chain than `Vec<Arc<peer::Peer>>`.

use std::net::SocketAddr;

use crate::{PeerId, peer};

/// Lightweight peer for announce responses.
///
/// Contains only the fields that response builders actually consume:
/// `peer_id` and `peer_addr`. This avoids carrying the full [`peer::Peer`]
/// struct (which includes swarm-management metadata) through the announce
/// call chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactPeer {
    /// Peer ID.
    pub peer_id: PeerId,
    /// IP address and port the peer is listening on.
    pub peer_addr: SocketAddr,
}

impl From<&peer::Peer> for CompactPeer {
    fn from(peer: &peer::Peer) -> Self {
        Self {
            peer_id: peer.peer_id,
            peer_addr: peer.peer_addr,
        }
    }
}

impl From<peer::Peer> for CompactPeer {
    fn from(peer: peer::Peer) -> Self {
        Self {
            peer_id: peer.peer_id,
            peer_addr: peer.peer_addr,
        }
    }
}

impl From<&CompactPeer> for CompactPeer {
    fn from(peer: &CompactPeer) -> Self {
        *peer
    }
}

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use torrust_clock::DurationSinceUnixEpoch;

    use super::CompactPeer;
    use crate::peer::Peer;
    use crate::{AnnounceEvent, NumberOfBytes, PeerId};

    fn sample_peer() -> Peer {
        Peer {
            peer_id: PeerId(*b"-qB00000000000000001"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes::new(0),
            downloaded: NumberOfBytes::new(0),
            left: NumberOfBytes::new(0),
            event: AnnounceEvent::Started,
        }
    }

    fn expected_compact() -> CompactPeer {
        CompactPeer {
            peer_id: PeerId(*b"-qB00000000000000001"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
        }
    }

    #[test]
    fn it_should_convert_from_peer_reference() {
        // Arrange
        let peer = sample_peer();

        // Act
        let compact = CompactPeer::from(&peer);

        // Assert
        assert_eq!(compact, expected_compact());
    }

    #[test]
    fn it_should_convert_from_owned_peer() {
        // Arrange
        let peer = sample_peer();

        // Act
        let compact = CompactPeer::from(peer);

        // Assert
        assert_eq!(compact, expected_compact());
    }

    #[test]
    fn it_should_support_copy_semantics() {
        // Arrange
        let compact = expected_compact();

        // Act
        let copied = compact;

        // Assert — both should be usable (Copy semantics)
        assert_eq!(compact, copied);
    }

    #[test]
    fn it_should_be_smaller_than_full_peer() {
        // Arrange & Act
        let compact_size = std::mem::size_of::<CompactPeer>();
        let peer_size = std::mem::size_of::<Peer>();
        let peer_id_size = std::mem::size_of::<PeerId>();
        let socket_addr_size = std::mem::size_of::<SocketAddr>();

        // Assert
        // Must be at least the sum of the fields, possibly more due to padding
        assert!(
            compact_size >= peer_id_size + socket_addr_size,
            "CompactPeer should be at least the sum of its fields"
        );
        // Must be smaller than a full Peer (which is 96 bytes)
        assert!(
            compact_size < peer_size,
            "CompactPeer ({compact_size}) should be smaller than a full Peer ({peer_size})"
        );
        // PeerId must be 20 bytes
        assert_eq!(peer_id_size, 20);
        // SocketAddr must be 32 bytes
        assert_eq!(socket_addr_size, 32);
    }
}
