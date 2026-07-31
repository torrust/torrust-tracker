//! Encoding layer for the HTTP tracker announce response.
//!
//! Types for encoding announce responses into bencoded bytes.
//! Supports two encoding forms: [`Normal`] (dictionary-based) and [`Compact`] (packed binary).
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use derive_more::{AsRef, Constructor};
use torrust_bencode::{BMutAccess, BencodeMut, ben_bytes, ben_int, ben_list, ben_map};

use crate::v1::responses::announce::data::{AnnounceData, Peer, PeerAddress};

const I2P_PLACEHOLDER_PORT: u16 = 1;

/// An [`Announce`] response, that can be anything that is convertible from [`AnnounceData`].
///
/// The [`Announce`] can built from any data that implements: [`From<AnnounceData>`] and [`Into<Vec<u8>>`].
///
/// The two standard forms of an announce response are: [`Normal`] and [`Compact`].
///
///
/// _"To reduce the size of tracker responses and to reduce memory and
/// computational requirements in trackers, trackers may return peers as a
/// packed string rather than as a bencoded list."_
///
/// Refer to the official BEPs for more information:
///
/// - [BEP 03: The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
/// - [BEP 23: Tracker Returns Compact Peer Lists](https://www.bittorrent.org/beps/bep_0023.html)
/// - [BEP 07: IPv6 Tracker Extension](https://www.bittorrent.org/beps/bep_0007.html)
/// - [I2P BitTorrent client protocol](https://i2p.net/en/docs/applications/bittorrent/)
///
// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(clippy::redundant_field_names)]
#[derive(Debug, AsRef, PartialEq, Constructor)]
pub struct Announce<E>
where
    E: From<AnnounceData> + Into<Vec<u8>>,
{
    pub data: E,
}

/// Build any [`Announce`] from an [`AnnounceData`].
impl<E: From<AnnounceData> + Into<Vec<u8>>> From<AnnounceData> for Announce<E> {
    fn from(data: AnnounceData) -> Self {
        Self::new(data.into())
    }
}

/// Format of the [`Normal`] (Non-Compact) Encoding
pub struct Normal {
    complete: i64,
    incomplete: i64,
    interval: i64,
    min_interval: i64,
    peers: Vec<NormalPeer>,
}

impl From<AnnounceData> for Normal {
    fn from(data: AnnounceData) -> Self {
        Self {
            complete: data.stats.complete.into(),
            incomplete: data.stats.incomplete.into(),
            interval: data.policy.interval.into(),
            min_interval: data.policy.interval_min.into(),
            peers: data.peers.into_iter().map(NormalPeer::from).collect(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<u8>> for Normal {
    fn into(self) -> Vec<u8> {
        let mut peers_list = ben_list!();
        let peers_list_mut = peers_list.list_mut().unwrap();
        for peer in &self.peers {
            peers_list_mut.push(peer.into());
        }

        (ben_map! {
            "complete" => ben_int!(self.complete),
            "incomplete" => ben_int!(self.incomplete),
            "interval" => ben_int!(self.interval),
            "min interval" => ben_int!(self.min_interval),
            "peers" => peers_list.clone()
        })
        .encode()
    }
}

/// Format of the [`Compact`] Encoding
pub struct Compact {
    complete: i64,
    incomplete: i64,
    interval: i64,
    min_interval: i64,
    peers: Vec<u8>,
    peers6: Vec<u8>,
}

impl From<AnnounceData> for Compact {
    fn from(data: AnnounceData) -> Self {
        let mut peers = vec![];
        let mut peers6 = vec![];

        for peer in data.peers.into_iter().map(CompactPeer::from) {
            match peer {
                CompactPeer::V4(peer) => {
                    peers.extend(u32::from(peer.ip).to_be_bytes());
                    peers.extend(peer.port.to_be_bytes());
                }
                CompactPeer::V6(peer) => {
                    peers6.extend(u128::from(peer.ip).to_be_bytes());
                    peers6.extend(peer.port.to_be_bytes());
                }
                CompactPeer::I2p(hash) => peers.extend(hash),
            }
        }

        Self {
            complete: data.stats.complete.into(),
            incomplete: data.stats.incomplete.into(),
            interval: data.policy.interval.into(),
            min_interval: data.policy.interval_min.into(),
            peers,
            peers6,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<u8>> for Compact {
    fn into(self) -> Vec<u8> {
        (ben_map! {
            "complete" => ben_int!(self.complete),
            "incomplete" => ben_int!(self.incomplete),
            "interval" => ben_int!(self.interval),
            "min interval" => ben_int!(self.min_interval),
            "peers" => ben_bytes!(self.peers),
            "peers6" => ben_bytes!(self.peers6)
        })
        .encode()
    }
}

/// A [`NormalPeer`], for the [`Normal`] form.
///
/// ```rust
/// use torrust_tracker_http_protocol::v1::responses::announce::{Normal, NormalPeer};
///
/// let peer = NormalPeer {
///     peer_id: *b"-RC3000-000000000001",
///     ip: "105.105.105.105".to_owned(),
///     port: 0x7070, // 28784
/// };
///
///  ```
#[derive(Debug, PartialEq)]
pub struct NormalPeer {
    /// The peer's ID.
    pub peer_id: [u8; 20],
    /// The peer's IP address or I2P Destination.
    pub ip: String,
    /// The peer's port number.
    pub port: u16,
}

impl From<Peer> for NormalPeer {
    fn from(peer: Peer) -> Self {
        match peer.peer_addr {
            PeerAddress::Clearnet(address) => NormalPeer {
                peer_id: peer.peer_id.0,
                ip: address.ip().to_string(),
                port: address.port(),
            },
            PeerAddress::I2p { destination, .. } => NormalPeer {
                peer_id: peer.peer_id.0,
                ip: destination,
                port: I2P_PLACEHOLDER_PORT,
            },
        }
    }
}

impl From<&NormalPeer> for BencodeMut<'_> {
    fn from(value: &NormalPeer) -> Self {
        ben_map! {
            "peer id" => ben_bytes!(value.peer_id.clone().to_vec()),
            "ip" => ben_bytes!(value.ip.clone()),
            "port" => ben_int!(i64::from(value.port))
        }
    }
}

/// A [`CompactPeer`], for the [`Compact`] form.
///
///  _"To reduce the size of tracker responses and to reduce memory and
/// computational requirements in trackers, trackers may return peers as a
/// packed string rather than as a bencoded list."_
///
/// A part from reducing the size of the response, this format does not contain
/// the peer's ID.
///
/// ```rust
///  use std::net::{IpAddr, Ipv4Addr};
///  use torrust_tracker_http_protocol::v1::responses::announce::{Compact, CompactPeer, CompactPeerData};
///
///  let peer = CompactPeer::V4(CompactPeerData {
///     ip: Ipv4Addr::new(0x69, 0x69, 0x69, 0x69), // 105.105.105.105
///     port: 0x7070, // 28784
/// });
///
///  ```
///
/// Refer to [BEP 23: Tracker Returns Compact Peer Lists](https://www.bittorrent.org/beps/bep_0023.html)
/// for more information.
#[derive(Clone, Debug, PartialEq)]
pub enum CompactPeer {
    /// The peer's IP address.
    V4(CompactPeerData<Ipv4Addr>),
    /// The peer's port number.
    V6(CompactPeerData<Ipv6Addr>),
    /// The SHA-256 hash of an I2P Destination.
    I2p([u8; 32]),
}

impl CompactPeer {
    /// Creates a compact peer from a socket address.
    #[must_use]
    pub fn new(socket_addr: &SocketAddr) -> Self {
        match socket_addr.ip() {
            IpAddr::V4(ip) => Self::V4(CompactPeerData {
                ip,
                port: socket_addr.port(),
            }),
            IpAddr::V6(ip) => Self::V6(CompactPeerData {
                ip,
                port: socket_addr.port(),
            }),
        }
    }

    /// Creates a compact peer from 6 bytes (IPv4) or 18 bytes (IPv6).
    #[must_use]
    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() == 18 {
            // IPv6: 16 bytes IP + 2 bytes port
            let ip = Ipv6Addr::new(
                u16::from_be_bytes([bytes[0], bytes[1]]),
                u16::from_be_bytes([bytes[2], bytes[3]]),
                u16::from_be_bytes([bytes[4], bytes[5]]),
                u16::from_be_bytes([bytes[6], bytes[7]]),
                u16::from_be_bytes([bytes[8], bytes[9]]),
                u16::from_be_bytes([bytes[10], bytes[11]]),
                u16::from_be_bytes([bytes[12], bytes[13]]),
                u16::from_be_bytes([bytes[14], bytes[15]]),
            );
            let port = u16::from_be_bytes([bytes[16], bytes[17]]);
            Self::V6(CompactPeerData { ip, port })
        } else {
            // IPv4: 4 bytes IP + 2 bytes port (BEP 23)
            let ip = Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]);
            let port = u16::from_be_bytes([bytes[4], bytes[5]]);
            Self::V4(CompactPeerData { ip, port })
        }
    }
}

impl From<Peer> for CompactPeer {
    fn from(peer: Peer) -> Self {
        match peer.peer_addr {
            PeerAddress::Clearnet(SocketAddr::V4(address)) => Self::V4(CompactPeerData {
                ip: *address.ip(),
                port: address.port(),
            }),
            PeerAddress::Clearnet(SocketAddr::V6(address)) => Self::V6(CompactPeerData {
                ip: *address.ip(),
                port: address.port(),
            }),
            PeerAddress::I2p { destination_hash, .. } => Self::I2p(destination_hash),
        }
    }
}

/// The [`CompactPeerData`], that made with either a [`Ipv4Addr`], or [`Ipv6Addr`] along with a `port`.
///
#[derive(Clone, Debug, PartialEq)]
pub struct CompactPeerData<V> {
    /// The peer's IP address.
    pub ip: V,
    /// The peer's port number.
    pub port: u16,
}

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use torrust_peer_id::PeerId;

    use crate::v1::responses::announce::{
        Announce, AnnounceData, AnnouncePolicy, Compact, Normal, Peer, PeerAddress, SwarmMetadata,
    };

    // Some ascii values used in tests:
    //
    // +-----------------+
    // | Dec | Hex | Chr |
    // +-----------------+
    // | 105 | 69  | i   |
    // | 112 | 70  | p   |
    // +-----------------+
    //
    // IP addresses and port numbers used in tests are chosen so that their bencoded representation
    // is also a valid string which makes asserts more readable.

    fn setup_announce_data() -> AnnounceData {
        let policy = AnnouncePolicy::new(111, 222);

        let peer_ipv4 = Peer {
            peer_id: PeerId(*b"-RC3000-000000000001"),
            peer_addr: PeerAddress::Clearnet(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0x69, 0x69, 0x69, 0x69)), 0x7070)),
        };

        let peer_ipv6 = Peer {
            peer_id: PeerId(*b"-RC3000-000000000002"),
            peer_addr: PeerAddress::Clearnet(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
                0x7070,
            )),
        };

        let peers = vec![peer_ipv4, peer_ipv6];
        let stats = SwarmMetadata::new(333, 333, 444);

        AnnounceData::new(peers, stats, policy)
    }

    #[test]
    fn non_compact_announce_response_can_be_bencoded() {
        let response: Announce<Normal> = setup_announce_data().into();
        let bytes = response.data.into();

        // cspell:disable-next-line
        let expected_bytes = b"d8:completei333e10:incompletei444e8:intervali111e12:min intervali222e5:peersld2:ip15:105.105.105.1057:peer id20:-RC3000-0000000000014:porti28784eed2:ip39:6969:6969:6969:6969:6969:6969:6969:69697:peer id20:-RC3000-0000000000024:porti28784eeee";

        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            String::from_utf8(expected_bytes.to_vec()).unwrap()
        );
    }

    #[test]
    fn compact_announce_response_can_be_bencoded() {
        let response: Announce<Compact> = setup_announce_data().into();
        let bytes = response.data.into();

        let expected_bytes =
            // cspell:disable-next-line
            b"d8:completei333e10:incompletei444e8:intervali111e12:min intervali222e5:peers6:iiiipp6:peers618:iiiiiiiiiiiiiiiippe";

        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            String::from_utf8(expected_bytes.to_vec()).unwrap()
        );
    }

    #[test]
    fn it_should_encode_an_i2p_peer_as_a_destination_in_a_non_compact_response() {
        let destination = format!("{}.i2p", "A".repeat(516));
        let data = AnnounceData::new(
            vec![Peer {
                peer_id: PeerId(*b"-RC3000-000000000001"),
                peer_addr: PeerAddress::I2p {
                    destination: destination.clone(),
                    destination_hash: [7; 32],
                },
            }],
            SwarmMetadata::default(),
            AnnouncePolicy::default(),
        );

        let response: Announce<Normal> = data.into();
        let bytes: Vec<u8> = response.data.into();
        let decoded = serde_bencode::from_bytes::<crate::v1::responses::announce::DeserializedNormal>(&bytes).unwrap();

        assert_eq!(decoded.peers[0].ip, destination);
        assert_eq!(decoded.peers[0].port, 1);
    }

    #[test]
    fn it_should_encode_an_i2p_peer_hash_in_a_compact_response() {
        let destination_hash = [7; 32];
        let data = AnnounceData::new(
            vec![Peer {
                peer_id: PeerId(*b"-RC3000-000000000001"),
                peer_addr: PeerAddress::I2p {
                    destination: format!("{}.i2p", "A".repeat(516)),
                    destination_hash,
                },
            }],
            SwarmMetadata::default(),
            AnnouncePolicy::default(),
        );

        let response: Announce<Compact> = data.into();
        let bytes: Vec<u8> = response.data.into();
        let decoded = crate::v1::responses::announce::DeserializedCompact::from_bytes(&bytes).unwrap();

        assert_eq!(decoded.peers, destination_hash);
        assert!(decoded.peers6.is_empty());
    }
}
