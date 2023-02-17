use std::io::Write;
use std::net::IpAddr;
use std::panic::Location;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{self, Deserialize, Serialize};
use thiserror::Error;

use crate::http::axum_implementation::responses;
use crate::tracker::{self, AnnounceData};

/// Normal (non compact) "announce" response
///
/// BEP 03: The ``BitTorrent`` Protocol Specification
/// <https://www.bittorrent.org/beps/bep_0003.html>
///
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NonCompact {
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub interval_min: u32,
    pub complete: u32,
    pub incomplete: u32,
    pub peers: Vec<Peer>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Peer {
    pub peer_id: String,
    pub ip: IpAddr,
    pub port: u16,
}

impl From<tracker::peer::Peer> for Peer {
    fn from(peer: tracker::peer::Peer) -> Self {
        Peer {
            peer_id: peer.peer_id.to_string(),
            ip: peer.peer_addr.ip(),
            port: peer.peer_addr.port(),
        }
    }
}

impl NonCompact {
    /// # Panics
    ///
    /// It would panic if the `Announce` struct contained an inappropriate type.
    #[must_use]
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }
}

impl IntoResponse for NonCompact {
    fn into_response(self) -> Response {
        (StatusCode::OK, self.write()).into_response()
    }
}

impl From<AnnounceData> for NonCompact {
    fn from(domain_announce_response: AnnounceData) -> Self {
        let peers: Vec<Peer> = domain_announce_response.peers.iter().map(|peer| Peer::from(*peer)).collect();

        Self {
            interval: domain_announce_response.interval,
            interval_min: domain_announce_response.interval_min,
            complete: domain_announce_response.swam_stats.seeders,
            incomplete: domain_announce_response.swam_stats.leechers,
            peers,
        }
    }
}

/// Compact "announce" response
///
/// BEP 23: Tracker Returns Compact Peer Lists
/// <https://www.bittorrent.org/beps/bep_0023.html>
///
/// BEP 07: IPv6 Tracker Extension
/// <https://www.bittorrent.org/beps/bep_0007.html>
///
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Compact {
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub interval_min: u32,
    pub complete: u32,
    pub incomplete: u32,
    pub peers: Vec<CompactPeer>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompactPeer {
    pub ip: IpAddr,
    pub port: u16,
}

impl CompactPeer {
    /// # Errors
    ///
    /// Will return `Err` if internally interrupted.
    pub fn write(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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
    /// # Errors
    ///
    /// Will return `Err` if internally interrupted.
    pub fn write(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes: Vec<u8> = Vec::new();

        // Begin dictionary
        bytes.write_all(b"d")?;

        // Write `interval`
        // Dictionary key
        bytes.write_all(b"8:interval")?;
        // Dictionary key value
        bytes.write_all(b"i")?; // Begin integer
        bytes.write_all(self.interval.to_string().as_bytes())?;
        bytes.write_all(b"e")?; // End integer

        // Write `interval_min`
        // Dictionary key
        bytes.write_all(b"12:min interval")?;
        // Dictionary key value
        bytes.write_all(b"i")?; // Begin integer
        bytes.write_all(self.interval_min.to_string().as_bytes())?;
        bytes.write_all(b"e")?; // End integer

        // Write `complete`
        // Dictionary key
        bytes.write_all(b"8:complete")?;
        // Dictionary key value
        bytes.write_all(b"i")?; // Begin integer
        bytes.write_all(self.complete.to_string().as_bytes())?;
        bytes.write_all(b"e")?; // End integer

        // Write `incomplete`
        // Dictionary key
        bytes.write_all(b"10:incomplete")?;
        // Dictionary key value
        bytes.write_all(b"i")?; // Begin integer
        bytes.write_all(self.incomplete.to_string().as_bytes())?;
        bytes.write_all(b"e")?; // End integer

        // Write peers with IPV4 IPs (BEP 23)

        // Dictionary key
        bytes.write_all(b"5:peers")?;
        // Dictionary key value
        let mut peers_v4: Vec<u8> = Vec::new();
        for compact_peer in &self.peers {
            match compact_peer.ip {
                IpAddr::V4(_ip) => {
                    let peer_bytes = compact_peer.write()?;
                    peers_v4.write_all(&peer_bytes)?;
                }
                IpAddr::V6(_) => {}
            }
        }
        bytes.write_all(peers_v4.len().to_string().as_bytes())?; // Begin byte string
        bytes.write_all(b":")?;
        bytes.write_all(peers_v4.as_slice())?;

        // todo: why is this `e` here?
        bytes.write_all(b"e")?;

        // Write peers with IPV6 IPs (BEP 07)

        // Dictionary key
        bytes.write_all(b"6:peers6")?;
        // Dictionary key value
        let mut peers_v6: Vec<u8> = Vec::new();
        for compact_peer in &self.peers {
            match compact_peer.ip {
                IpAddr::V6(_ip) => {
                    let peer_bytes = compact_peer.write()?;
                    peers_v6.write_all(&peer_bytes)?;
                }
                IpAddr::V4(_) => {}
            }
        }
        bytes.write_all(peers_v6.len().to_string().as_bytes())?; // Begin byte string
        bytes.write_all(b":")?;
        bytes.write_all(peers_v6.as_slice())?; // End byte string

        // End dictionary
        bytes.write_all(b"e")?;

        Ok(bytes)
    }
}

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
        match self.write() {
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
            complete: domain_announce_response.swam_stats.seeders,
            incomplete: domain_announce_response.swam_stats.leechers,
            peers,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use super::{NonCompact, Peer};
    use crate::http::axum_implementation::responses::announce::{Compact, CompactPeer};

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
                    peer_id: "-qB00000000000000001".to_string(),
                    ip: IpAddr::V4(Ipv4Addr::new(0x69, 0x69, 0x69, 0x69)), // 105.105.105.105
                    port: 0x7070,                                          // 28784
                },
                // IPV6
                Peer {
                    peer_id: "-qB00000000000000002".to_string(),
                    ip: IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
                    port: 0x7070, // 28784
                },
            ],
        };

        // cspell:disable-next-line
        assert_eq!(response.write(), "d8:completei333e10:incompletei444e8:intervali111e12:min intervali222e5:peersld2:ip15:105.105.105.1057:peer_id20:-qB000000000000000014:porti28784eed2:ip39:6969:6969:6969:6969:6969:6969:6969:69697:peer_id20:-qB000000000000000024:porti28784eeee");
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

        let bytes = response.write().unwrap();

        // cspell:disable-next-line
        assert_eq!(
            bytes,
            // cspell:disable-next-line
            b"d8:intervali111e12:min intervali222e8:completei333e10:incompletei444e5:peers6:iiiippe6:peers618:iiiiiiiiiiiiiiiippe"
        );
    }
}
