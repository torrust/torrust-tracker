//! Peer struct used by the core `Tracker`.
//!
//! A sample peer:
//!
//! ```rust,no_run
//! peer::Peer {
//!     peer_id: peer::Id(*b"-qB00000000000000000"),
//!     peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
//!     updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
//!     uploaded: NumberOfBytes(0),
//!     downloaded: NumberOfBytes(0),
//!     left: NumberOfBytes(0),
//!     event: AnnounceEvent::Started,
//! }
//! ```
use std::net::{IpAddr, SocketAddr};
use std::panic::Location;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use serde;
use serde::Serialize;
use thiserror::Error;

use crate::shared::bit_torrent::common::{AnnounceEventDef, NumberOfBytesDef};
use crate::shared::clock::utils::ser_unix_time_value;
use crate::shared::clock::DurationSinceUnixEpoch;

/// IP version used by the peer to connect to the tracker: IPv4 or IPv6
#[derive(PartialEq, Eq, Debug)]
pub enum IPVersion {
    /// <https://en.wikipedia.org/wiki/Internet_Protocol_version_4>
    IPv4,
    /// <https://en.wikipedia.org/wiki/IPv6>
    IPv6,
}

/// Peer struct used by the core `Tracker`.
///
/// A sample peer:
///
/// ```rust,no_run
/// peer::Peer {
///     peer_id: peer::Id(*b"-qB00000000000000000"),
///     peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
///     updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
///     uploaded: NumberOfBytes(0),
///     downloaded: NumberOfBytes(0),
///     left: NumberOfBytes(0),
///     event: AnnounceEvent::Started,
/// }
/// ```
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Copy)]
pub struct Peer {
    /// ID used by the downloader peer
    pub peer_id: Id,
    /// The IP and port this peer is listening on
    pub peer_addr: SocketAddr,
    /// The last time the the tracker receive an announce request from this peer (timestamp)
    #[serde(serialize_with = "ser_unix_time_value")]
    pub updated: DurationSinceUnixEpoch,
    /// The total amount of bytes uploaded by this peer so far
    #[serde(with = "NumberOfBytesDef")]
    pub uploaded: NumberOfBytes,
    /// The total amount of bytes downloaded by this peer so far
    #[serde(with = "NumberOfBytesDef")]
    pub downloaded: NumberOfBytes,
    /// The number of bytes this peer still has to download
    #[serde(with = "NumberOfBytesDef")]
    pub left: NumberOfBytes,
    /// This is an optional key which maps to started, completed, or stopped (or empty, which is the same as not being present).
    #[serde(with = "AnnounceEventDef")]
    pub event: AnnounceEvent,
}

impl Peer {
    #[must_use]
    pub fn is_seeder(&self) -> bool {
        self.left.0 <= 0 && self.event != AnnounceEvent::Stopped
    }

    pub fn ip(&mut self) -> IpAddr {
        self.peer_addr.ip()
    }

    pub fn change_ip(&mut self, new_ip: &IpAddr) {
        self.peer_addr = SocketAddr::new(*new_ip, self.peer_addr.port());
    }

    /// The IP version used by the peer: IPV4 or IPV6
    #[must_use]
    pub fn ip_version(&self) -> IPVersion {
        if self.peer_addr.is_ipv4() {
            return IPVersion::IPv4;
        }
        IPVersion::IPv6
    }
}

/// Peer ID. A 20-byte array.
///
/// A string of length 20 which this downloader uses as its id.
/// Each downloader generates its own id at random at the start of a new download.
///
/// A sample peer ID:
///
/// ```rust,no_run
/// let peer_id = peer::Id(*b"-qB00000000000000000");
/// ```
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Copy)]
pub struct Id(pub [u8; 20]);

const PEER_ID_BYTES_LEN: usize = 20;

/// Error returned when trying to convert an invalid peer id from another type.
///
/// Usually because the source format does not contain 20 bytes.
#[derive(Error, Debug)]
pub enum IdConversionError {
    #[error("not enough bytes for peer id: {message} {location}")]
    NotEnoughBytes {
        location: &'static Location<'static>,
        message: String,
    },
    #[error("too many bytes for peer id: {message} {location}")]
    TooManyBytes {
        location: &'static Location<'static>,
        message: String,
    },
}

impl Id {
    /// # Panics
    ///
    /// Will panic if byte slice does not contains the exact amount of bytes need for the `Id`.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), PEER_ID_BYTES_LEN);
        let mut ret = Self([0u8; PEER_ID_BYTES_LEN]);
        ret.0.clone_from_slice(bytes);
        ret
    }

    #[must_use]
    pub fn to_bytes(&self) -> [u8; 20] {
        self.0
    }
}

impl From<[u8; 20]> for Id {
    fn from(bytes: [u8; 20]) -> Self {
        Id(bytes)
    }
}

impl TryFrom<Vec<u8>> for Id {
    type Error = IdConversionError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < PEER_ID_BYTES_LEN {
            return Err(IdConversionError::NotEnoughBytes {
                location: Location::caller(),
                message: format! {"got {} bytes, expected {}", bytes.len(), PEER_ID_BYTES_LEN},
            });
        }
        if bytes.len() > PEER_ID_BYTES_LEN {
            return Err(IdConversionError::TooManyBytes {
                location: Location::caller(),
                message: format! {"got {} bytes, expected {}", bytes.len(), PEER_ID_BYTES_LEN},
            });
        }
        Ok(Self::from_bytes(&bytes))
    }
}

impl std::str::FromStr for Id {
    type Err = IdConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.as_bytes().to_vec())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_hex_string() {
            Some(hex) => write!(f, "{hex}"),
            None => write!(f, ""),
        }
    }
}

impl Id {
    #[must_use]
    /// Converts to hex string.
    ///
    /// For the Id `-qB00000000000000000` it returns `2d71423030303030303030303030303030303030`
    ///
    /// For example:
    ///
    ///```text
    /// Bytes                = Hex
    /// -qB00000000000000000 = 2d71423030303030303030303030303030303030
    /// -qB00000000000000000 = 2d 71 42 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30
    ///
    /// -------------
    /// |Char | Hex |
    /// -------------
    /// | -   | 2D  |
    /// | q   | 71  |
    /// | B   | 42  |
    /// | 0   | 30  |
    /// -------------
    /// ```
    ///
    /// Return `None` is some of the bytes are invalid UTF8 values.
    ///
    /// # Panics
    ///
    /// It will panic if the `binascii::bin2hex` from a too-small output buffer.
    pub fn to_hex_string(&self) -> Option<String> {
        let buff_size = self.0.len() * 2;
        let mut tmp: Vec<u8> = vec![0; buff_size];

        binascii::bin2hex(&self.0, &mut tmp).unwrap();

        match std::str::from_utf8(&tmp) {
            Ok(hex) => Some(format!("0x{hex}")),
            Err(_) => None,
        }
    }

    #[must_use]
    pub fn get_client_name(&self) -> Option<&'static str> {
        if self.0[0] == b'M' {
            return Some("BitTorrent");
        }
        if self.0[0] == b'-' {
            let name = match &self.0[1..3] {
                b"AG" | b"A~" => "Ares",
                b"AR" => "Arctic",
                b"AV" => "Avicora",
                b"AX" => "BitPump",
                b"AZ" => "Azureus",
                b"BB" => "BitBuddy",
                b"BC" => "BitComet",
                b"BF" => "Bitflu",
                b"BG" => "BTG (uses Rasterbar libtorrent)",
                b"BR" => "BitRocket",
                b"BS" => "BTSlave",
                b"BX" => "~Bittorrent X",
                b"CD" => "Enhanced CTorrent",
                b"CT" => "CTorrent",
                b"DE" => "DelugeTorrent",
                b"DP" => "Propagate Data Client",
                b"EB" => "EBit",
                b"ES" => "electric sheep",
                b"FT" => "FoxTorrent",
                b"FW" => "FrostWire",
                b"FX" => "Freebox BitTorrent",
                b"GS" => "GSTorrent",
                b"HL" => "Halite",
                b"HN" => "Hydranode",
                b"KG" => "KGet",
                b"KT" => "KTorrent",
                b"LH" => "LH-ABC",
                b"LP" => "Lphant",
                b"LT" => "libtorrent",
                b"lt" => "libTorrent",
                b"LW" => "LimeWire",
                b"MO" => "MonoTorrent",
                b"MP" => "MooPolice",
                b"MR" => "Miro",
                b"MT" => "MoonlightTorrent",
                b"NX" => "Net Transport",
                b"PD" => "Pando",
                b"qB" => "qBittorrent",
                b"QD" => "QQDownload",
                b"QT" => "Qt 4 Torrent example",
                b"RT" => "Retriever",
                b"S~" => "Shareaza alpha/beta",
                b"SB" => "~Swiftbit",
                b"SS" => "SwarmScope",
                b"ST" => "SymTorrent",
                b"st" => "sharktorrent",
                b"SZ" => "Shareaza",
                b"TN" => "TorrentDotNET",
                b"TR" => "Transmission",
                b"TS" => "Torrentstorm",
                b"TT" => "TuoTu",
                b"UL" => "uLeecher!",
                b"UT" => "µTorrent",
                b"UW" => "µTorrent Web",
                b"VG" => "Vagaa",
                b"WD" => "WebTorrent Desktop",
                b"WT" => "BitLet",
                b"WW" => "WebTorrent",
                b"WY" => "FireTorrent",
                b"XL" => "Xunlei",
                b"XT" => "XanTorrent",
                b"XX" => "Xtorrent",
                b"ZT" => "ZipTorrent",
                _ => return None,
            };
            Some(name)
        } else {
            None
        }
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct PeerIdInfo<'a> {
            id: Option<String>,
            client: Option<&'a str>,
        }

        let obj = PeerIdInfo {
            id: self.to_hex_string(),
            client: self.get_client_name(),
        };
        obj.serialize(serializer)
    }
}

#[cfg(test)]
mod test {

    mod torrent_peer_id {
        use crate::tracker::peer;

        #[test]
        fn should_be_instantiated_from_a_byte_slice() {
            let id = peer::Id::from_bytes(&[
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);

            let expected_id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);

            assert_eq!(id, expected_id);
        }

        #[test]
        #[should_panic]
        fn should_fail_trying_to_instantiate_from_a_byte_slice_with_less_than_20_bytes() {
            let less_than_20_bytes = [0; 19];
            let _ = peer::Id::from_bytes(&less_than_20_bytes);
        }

        #[test]
        #[should_panic]
        fn should_fail_trying_to_instantiate_from_a_byte_slice_with_more_than_20_bytes() {
            let more_than_20_bytes = [0; 21];
            let _ = peer::Id::from_bytes(&more_than_20_bytes);
        }

        #[test]
        fn should_be_instantiated_from_a_string() {
            let id = "-qB00000000000000001".parse::<peer::Id>().unwrap();

            let expected_id = peer::Id([
                45, 113, 66, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 49,
            ]);

            assert_eq!(id, expected_id);
        }

        #[test]
        fn should_be_converted_from_a_20_byte_array() {
            let id = peer::Id::from([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);

            let expected_id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);

            assert_eq!(id, expected_id);
        }

        #[test]
        fn should_be_converted_from_a_byte_vector() {
            let id = peer::Id::try_from(
                [
                    0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
                ]
                .to_vec(),
            )
            .unwrap();

            let expected_id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);

            assert_eq!(id, expected_id);
        }

        #[test]
        #[should_panic]
        fn should_fail_trying_to_convert_from_a_byte_vector_with_less_than_20_bytes() {
            let _ = peer::Id::try_from([0; 19].to_vec()).unwrap();
        }

        #[test]
        #[should_panic]
        fn should_fail_trying_to_convert_from_a_byte_vector_with_more_than_20_bytes() {
            let _ = peer::Id::try_from([0; 21].to_vec()).unwrap();
        }

        #[test]
        fn should_be_converted_to_hex_string() {
            let id = peer::Id(*b"-qB00000000000000000");
            assert_eq!(id.to_hex_string().unwrap(), "0x2d71423030303030303030303030303030303030");

            let id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);
            assert_eq!(id.to_hex_string().unwrap(), "0x009f9296009f9296009f9296009f9296009f9296");
        }

        #[test]
        fn should_be_converted_into_string_type_using_the_hex_string_format() {
            let id = peer::Id(*b"-qB00000000000000000");
            assert_eq!(id.to_string(), "0x2d71423030303030303030303030303030303030");

            let id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);
            assert_eq!(id.to_string(), "0x009f9296009f9296009f9296009f9296009f9296");
        }

        #[test]
        fn should_return_the_inner_bytes() {
            assert_eq!(peer::Id(*b"-qB00000000000000000").to_bytes(), *b"-qB00000000000000000");
        }
    }

    mod torrent_peer {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
        use serde_json::Value;

        use crate::shared::clock::{Current, Time};
        use crate::tracker::peer::{self, Peer};

        #[test]
        fn it_should_be_serializable() {
            let torrent_peer = Peer {
                peer_id: peer::Id(*b"-qB00000000000000000"),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
                updated: Current::now(),
                uploaded: NumberOfBytes(0),
                downloaded: NumberOfBytes(0),
                left: NumberOfBytes(0),
                event: AnnounceEvent::Started,
            };

            let raw_json = serde_json::to_string(&torrent_peer).unwrap();

            let expected_raw_json = r#"
                {
                    "peer_id": {
                        "id": "0x2d71423030303030303030303030303030303030",
                        "client": "qBittorrent"
                    },
                    "peer_addr":"126.0.0.1:8080",
                    "updated":0,
                    "uploaded":0,
                    "downloaded":0,
                    "left":0,
                    "event":"Started"
                }
            "#;

            assert_eq!(
                serde_json::from_str::<Value>(&raw_json).unwrap(),
                serde_json::from_str::<Value>(expected_raw_json).unwrap()
            );
        }
    }
}
