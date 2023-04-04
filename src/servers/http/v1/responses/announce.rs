//! `Announce` response for the HTTP tracker [`announce`](crate::servers::http::v1::requests::announce::Announce) request.
//!
//! Data structures and logic to build the `announce` response.
use std::io::Write;
use std::net::IpAddr;
use std::panic::Location;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bip_bencode::{ben_bytes, ben_int, ben_list, ben_map, BMutAccess, BencodeMut};
use serde::{self, Deserialize, Serialize};
use thiserror::Error;

use crate::servers::http::v1::responses;
use crate::tracker::{self, AnnounceData};

/// Normal (non compact) `announce` response.
///
/// It's a bencoded dictionary.
///
/// ```rust
/// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
/// use torrust_tracker::servers::http::v1::responses::announce::{NonCompact, Peer};
///
/// let response = NonCompact {
///     interval: 111,
///     interval_min: 222,
///     complete: 333,
///     incomplete: 444,
///     peers: vec![
///         // IPV4
///         Peer {
///             peer_id: *b"-qB00000000000000001",
///             ip: IpAddr::V4(Ipv4Addr::new(0x69, 0x69, 0x69, 0x69)), // 105.105.105.105
///             port: 0x7070,                                          // 28784
///         },
///         // IPV6
///         Peer {
///             peer_id: *b"-qB00000000000000002",
///             ip: IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
///             port: 0x7070, // 28784
///         },
///     ],
/// };
///
/// let bytes = response.body();
///
/// // The expected bencoded response.
/// let expected_bytes = b"d8:completei333e10:incompletei444e8:intervali111e12:min intervali222e5:peersld2:ip15:105.105.105.1057:peer id20:-qB000000000000000014:porti28784eed2:ip39:6969:6969:6969:6969:6969:6969:6969:69697:peer id20:-qB000000000000000024:porti28784eeee";
///
/// assert_eq!(
///     String::from_utf8(bytes).unwrap(),
///     String::from_utf8(expected_bytes.to_vec()).unwrap()
/// );
/// ```
///
/// Refer to [BEP 03: The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
/// for more information.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NonCompact {
    /// Interval in seconds that the client should wait between sending regular
    /// announce requests to the tracker.
    ///
    /// It's a **recommended** wait time between announcements.
    ///
    /// This is the standard amount of time that clients should wait between
    /// sending consecutive announcements to the tracker. This value is set by
    /// the tracker and is typically provided in the tracker's response to a
    /// client's initial request. It serves as a guideline for clients to know
    /// how often they should contact the tracker for updates on the peer list,
    /// while ensuring that the tracker is not overwhelmed with requests.
    pub interval: u32,
    /// Minimum announce interval. Clients must not reannounce more frequently
    /// than this.
    ///
    /// It establishes the shortest allowed wait time.
    ///
    /// This is an optional parameter in the protocol that the tracker may
    /// provide in its response. It sets a lower limit on the frequency at which
    /// clients are allowed to send announcements. Clients should respect this
    /// value to prevent sending too many requests in a short period, which
    /// could lead to excessive load on the tracker or even getting banned by
    /// the tracker for not adhering to the rules.
    #[serde(rename = "min interval")]
    pub interval_min: u32,
    /// Number of peers with the entire file, i.e. seeders.
    pub complete: u32,
    /// Number of non-seeder peers, aka "leechers".
    pub incomplete: u32,
    /// A list of peers. The value is a list of dictionaries.
    pub peers: Vec<Peer>,
}

/// Peer information in the [`NonCompact`](crate::servers::http::v1::responses::announce::NonCompact)
/// response.
///
/// ```rust
/// use std::net::{IpAddr, Ipv4Addr};
/// use torrust_tracker::servers::http::v1::responses::announce::{NonCompact, Peer};
///
/// let peer = Peer {
///     peer_id: *b"-qB00000000000000001",
///     ip: IpAddr::V4(Ipv4Addr::new(0x69, 0x69, 0x69, 0x69)), // 105.105.105.105
///     port: 0x7070,                                          // 28784
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Peer {
    /// The peer's ID.
    pub peer_id: [u8; 20],
    /// The peer's IP address.
    pub ip: IpAddr,
    /// The peer's port number.
    pub port: u16,
}

impl Peer {
    #[must_use]
    pub fn ben_map(&self) -> BencodeMut {
        ben_map! {
            "peer id" => ben_bytes!(self.peer_id.clone().to_vec()),
            "ip" => ben_bytes!(self.ip.to_string()),
            "port" => ben_int!(i64::from(self.port))
        }
    }
}

impl From<tracker::peer::Peer> for Peer {
    fn from(peer: tracker::peer::Peer) -> Self {
        Peer {
            peer_id: peer.peer_id.to_bytes(),
            ip: peer.peer_addr.ip(),
            port: peer.peer_addr.port(),
        }
    }
}

impl NonCompact {
    /// Returns the bencoded body of the non-compact response.
    ///
    /// # Panics
    ///
    /// Will return an error if it can't access the bencode as a mutable `BListAccess`.
    #[must_use]
    pub fn body(&self) -> Vec<u8> {
        let mut peers_list = ben_list!();
        let peers_list_mut = peers_list.list_mut().unwrap();
        for peer in &self.peers {
            peers_list_mut.push(peer.ben_map());
        }

        (ben_map! {
            "complete" => ben_int!(i64::from(self.complete)),
            "incomplete" => ben_int!(i64::from(self.incomplete)),
            "interval" => ben_int!(i64::from(self.interval)),
            "min interval" => ben_int!(i64::from(self.interval_min)),
            "peers" => peers_list.clone()
        })
        .encode()
    }
}

impl IntoResponse for NonCompact {
    fn into_response(self) -> Response {
        (StatusCode::OK, self.body()).into_response()
    }
}

impl From<AnnounceData> for NonCompact {
    fn from(domain_announce_response: AnnounceData) -> Self {
        let peers: Vec<Peer> = domain_announce_response.peers.iter().map(|peer| Peer::from(*peer)).collect();

        Self {
            interval: domain_announce_response.interval,
            interval_min: domain_announce_response.interval_min,
            complete: domain_announce_response.swarm_stats.seeders,
            incomplete: domain_announce_response.swarm_stats.leechers,
            peers,
        }
    }
}

/// Compact `announce` response.
///
/// _"To reduce the size of tracker responses and to reduce memory and
/// computational requirements in trackers, trackers may return peers as a
/// packed string rather than as a bencoded list."_
///
/// ```rust
/// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
/// use torrust_tracker::servers::http::v1::responses::announce::{Compact, CompactPeer};
///
/// let response = Compact {
///     interval: 111,
///     interval_min: 222,
///     complete: 333,
///     incomplete: 444,
///     peers: vec![
///         // IPV4
///         CompactPeer {
///             ip: IpAddr::V4(Ipv4Addr::new(0x69, 0x69, 0x69, 0x69)), // 105.105.105.105
///             port: 0x7070,                                          // 28784
///         },
///         // IPV6
///         CompactPeer {
///             ip: IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
///             port: 0x7070, // 28784
///         },
///     ],
/// };
///
/// let bytes = response.body().unwrap();
///
/// // The expected bencoded response.
/// let expected_bytes =
///     // cspell:disable-next-line
///     b"d8:completei333e10:incompletei444e8:intervali111e12:min intervali222e5:peers6:iiiipp6:peers618:iiiiiiiiiiiiiiiippe";
///
/// assert_eq!(
///     String::from_utf8(bytes).unwrap(),
///     String::from_utf8(expected_bytes.to_vec()).unwrap()
/// );
/// ```
///
/// Refer to the official BEPs for more information:
///
/// - [BEP 23: Tracker Returns Compact Peer Lists](https://www.bittorrent.org/beps/bep_0023.html)
/// - [BEP 07: IPv6 Tracker Extension](https://www.bittorrent.org/beps/bep_0007.html)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Compact {
    /// Interval in seconds that the client should wait between sending regular
    /// announce requests to the tracker.
    ///
    /// It's a **recommended** wait time between announcements.
    ///
    /// This is the standard amount of time that clients should wait between
    /// sending consecutive announcements to the tracker. This value is set by
    /// the tracker and is typically provided in the tracker's response to a
    /// client's initial request. It serves as a guideline for clients to know
    /// how often they should contact the tracker for updates on the peer list,
    /// while ensuring that the tracker is not overwhelmed with requests.    
    pub interval: u32,
    /// Minimum announce interval. Clients must not reannounce more frequently
    /// than this.
    ///
    /// It establishes the shortest allowed wait time.
    ///
    /// This is an optional parameter in the protocol that the tracker may
    /// provide in its response. It sets a lower limit on the frequency at which
    /// clients are allowed to send announcements. Clients should respect this
    /// value to prevent sending too many requests in a short period, which
    /// could lead to excessive load on the tracker or even getting banned by
    /// the tracker for not adhering to the rules.    
    #[serde(rename = "min interval")]
    pub interval_min: u32,
    /// Number of seeders, aka "completed".
    pub complete: u32,
    /// Number of non-seeder peers, aka "incomplete".
    pub incomplete: u32,
    /// Compact peer list.
    pub peers: Vec<CompactPeer>,
}

/// Compact peer. It's used in the [`Compact`](crate::servers::http::v1::responses::announce::Compact)
/// response.
///
/// _"To reduce the size of tracker responses and to reduce memory and
/// computational requirements in trackers, trackers may return peers as a
/// packed string rather than as a bencoded list."_
///
/// A part from reducing the size of the response, this format does not contain
/// the peer's ID.
///
/// ```rust
/// use std::net::{IpAddr, Ipv4Addr};
/// use torrust_tracker::servers::http::v1::responses::announce::CompactPeer;
///
/// let compact_peer = CompactPeer {
///     ip: IpAddr::V4(Ipv4Addr::new(0x69, 0x69, 0x69, 0x69)), // 105.105.105.105
///     port: 0x7070                                           // 28784
/// };
/// ```
///
/// Refer to [BEP 23: Tracker Returns Compact Peer Lists](https://www.bittorrent.org/beps/bep_0023.html)
/// for more information.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompactPeer {
    /// The peer's IP address.
    pub ip: IpAddr,
    /// The peer's port number.
    pub port: u16,
}

impl CompactPeer {
    /// Returns the compact peer as a byte vector.
    ///
    /// # Errors
    ///
    /// Will return `Err` if internally interrupted.
    pub fn bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes: Vec<u8> = Vec::new();
        match self.ip {
            IpAddr::V4(ip) => {
                bytes.write_all(&u32::from(ip).to_be_bytes())?;
            }
            IpAddr::V6(ip) => {
                bytes.write_all(&u128::from(ip).to_be_bytes())?;
            }
        }
        bytes.write_all(&self.port.to_be_bytes())?;
        Ok(bytes)
    }
}

impl From<tracker::peer::Peer> for CompactPeer {
    fn from(peer: tracker::peer::Peer) -> Self {
        CompactPeer {
            ip: peer.peer_addr.ip(),
            port: peer.peer_addr.port(),
        }
    }
}

impl Compact {
    /// Returns the bencoded compact response as a byte vector.
    ///
    /// # Errors
    ///
    /// Will return `Err` if internally interrupted.
    pub fn body(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let bytes = (ben_map! {
            "complete" => ben_int!(i64::from(self.complete)),
            "incomplete" => ben_int!(i64::from(self.incomplete)),
            "interval" => ben_int!(i64::from(self.interval)),
            "min interval" => ben_int!(i64::from(self.interval_min)),
            "peers" => ben_bytes!(self.peers_v4_bytes()?),
            "peers6" => ben_bytes!(self.peers_v6_bytes()?)
        })
        .encode();

        Ok(bytes)
    }

    fn peers_v4_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes: Vec<u8> = Vec::new();
        for compact_peer in &self.peers {
            match compact_peer.ip {
                IpAddr::V4(_ip) => {
                    let peer_bytes = compact_peer.bytes()?;
                    bytes.write_all(&peer_bytes)?;
                }
                IpAddr::V6(_) => {}
            }
        }
        Ok(bytes)
    }

    fn peers_v6_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes: Vec<u8> = Vec::new();
        for compact_peer in &self.peers {
            match compact_peer.ip {
                IpAddr::V6(_ip) => {
                    let peer_bytes = compact_peer.bytes()?;
                    bytes.write_all(&peer_bytes)?;
                }
                IpAddr::V4(_) => {}
            }
        }
        Ok(bytes)
    }
}

/// `Compact` response serialization error.
#[derive(Error, Debug)]
pub enum CompactSerializationError {
    #[error("cannot write bytes: {inner_error} in {location}")]
    CannotWriteBytes {
        location: &'static Location<'static>,
        inner_error: String,
    },
}

impl From<CompactSerializationError> for responses::error::Error {
    fn from(err: CompactSerializationError) -> Self {
        responses::error::Error {
            failure_reason: format!("{err}"),
        }
    }
}

impl IntoResponse for Compact {
    fn into_response(self) -> Response {
        match self.body() {
            Ok(bytes) => (StatusCode::OK, bytes).into_response(),
            Err(err) => responses::error::Error::from(CompactSerializationError::CannotWriteBytes {
                location: Location::caller(),
                inner_error: format!("{err}"),
            })
            .into_response(),
        }
    }
}

impl From<AnnounceData> for Compact {
    fn from(domain_announce_response: AnnounceData) -> Self {
        let peers: Vec<CompactPeer> = domain_announce_response
            .peers
            .iter()
            .map(|peer| CompactPeer::from(*peer))
            .collect();

        Self {
            interval: domain_announce_response.interval,
            interval_min: domain_announce_response.interval_min,
            complete: domain_announce_response.swarm_stats.seeders,
            incomplete: domain_announce_response.swarm_stats.leechers,
            peers,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use super::{NonCompact, Peer};
    use crate::servers::http::v1::responses::announce::{Compact, CompactPeer};

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

    #[test]
    fn non_compact_announce_response_can_be_bencoded() {
        let response = NonCompact {
            interval: 111,
            interval_min: 222,
            complete: 333,
            incomplete: 444,
            peers: vec![
                // IPV4
                Peer {
                    peer_id: *b"-qB00000000000000001",
                    ip: IpAddr::V4(Ipv4Addr::new(0x69, 0x69, 0x69, 0x69)), // 105.105.105.105
                    port: 0x7070,                                          // 28784
                },
                // IPV6
                Peer {
                    peer_id: *b"-qB00000000000000002",
                    ip: IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
                    port: 0x7070, // 28784
                },
            ],
        };

        let bytes = response.body();

        // cspell:disable-next-line
        let expected_bytes = b"d8:completei333e10:incompletei444e8:intervali111e12:min intervali222e5:peersld2:ip15:105.105.105.1057:peer id20:-qB000000000000000014:porti28784eed2:ip39:6969:6969:6969:6969:6969:6969:6969:69697:peer id20:-qB000000000000000024:porti28784eeee";

        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            String::from_utf8(expected_bytes.to_vec()).unwrap()
        );
    }

    #[test]
    fn compact_announce_response_can_be_bencoded() {
        let response = Compact {
            interval: 111,
            interval_min: 222,
            complete: 333,
            incomplete: 444,
            peers: vec![
                // IPV4
                CompactPeer {
                    ip: IpAddr::V4(Ipv4Addr::new(0x69, 0x69, 0x69, 0x69)), // 105.105.105.105
                    port: 0x7070,                                          // 28784
                },
                // IPV6
                CompactPeer {
                    ip: IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
                    port: 0x7070, // 28784
                },
            ],
        };

        let bytes = response.body().unwrap();

        let expected_bytes =
            // cspell:disable-next-line
            b"d8:completei333e10:incompletei444e8:intervali111e12:min intervali222e5:peers6:iiiipp6:peers618:iiiiiiiiiiiiiiiippe";

        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            String::from_utf8(expected_bytes.to_vec()).unwrap()
        );
    }
}
