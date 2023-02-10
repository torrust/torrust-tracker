use std::net::SocketAddr;
use std::panic::Location;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use serde;
use serde::Serialize;
use thiserror::Error;

use crate::http::warp_implementation::request::Announce;
use crate::protocol::clock::{Current, DurationSinceUnixEpoch, Time};
use crate::protocol::common::{AnnounceEventDef, NumberOfBytesDef};
use crate::protocol::utils::ser_unix_time_value;

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Copy)]
pub struct Peer {
    pub peer_id: Id,
    pub peer_addr: SocketAddr,
    #[serde(serialize_with = "ser_unix_time_value")]
    pub updated: DurationSinceUnixEpoch,
    #[serde(with = "NumberOfBytesDef")]
    pub uploaded: NumberOfBytes,
    #[serde(with = "NumberOfBytesDef")]
    pub downloaded: NumberOfBytes,
    #[serde(with = "NumberOfBytesDef")]
    pub left: NumberOfBytes, // The number of bytes this peer still has to download
    #[serde(with = "AnnounceEventDef")]
    pub event: AnnounceEvent,
}

impl Peer {
    #[must_use]
    pub fn from_udp_announce_request(announce_request: &aquatic_udp_protocol::AnnounceRequest, peer_addr: &SocketAddr) -> Self {
        Peer {
            peer_id: Id(announce_request.peer_id.0),
            peer_addr: *peer_addr,
            updated: Current::now(),
            uploaded: announce_request.bytes_uploaded,
            downloaded: announce_request.bytes_downloaded,
            left: announce_request.bytes_left,
            event: announce_request.event,
        }
    }

    #[must_use]
    pub fn from_http_announce_request(announce_request: &Announce, peer_addr: &SocketAddr) -> Self {
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
            peer_addr: *peer_addr,
            updated: Current::now(),
            uploaded: NumberOfBytes(i128::from(announce_request.uploaded) as i64),
            downloaded: NumberOfBytes(i128::from(announce_request.downloaded) as i64),
            left: NumberOfBytes(i128::from(announce_request.left) as i64),
            event,
        }
    }

    #[must_use]
    pub fn is_seeder(&self) -> bool {
        self.left.0 <= 0 && self.event != AnnounceEvent::Stopped
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Copy)]
pub struct Id(pub [u8; 20]);

const PEER_ID_BYTES_LEN: usize = 20;

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
        std::str::from_utf8(&tmp).ok().map(std::string::ToString::to_string)
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
            assert_eq!(id.to_hex_string().unwrap(), "2d71423030303030303030303030303030303030");

            let id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);
            assert_eq!(id.to_hex_string().unwrap(), "009f9296009f9296009f9296009f9296009f9296");
        }

        #[test]
        fn should_be_converted_into_string_type_using_the_hex_string_format() {
            let id = peer::Id(*b"-qB00000000000000000");
            assert_eq!(id.to_string(), "2d71423030303030303030303030303030303030");

            let id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);
            assert_eq!(id.to_string(), "009f9296009f9296009f9296009f9296009f9296");
        }
    }

    mod torrent_peer {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};

        use crate::protocol::clock::{Current, Time};
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

            let json_serialized_value = serde_json::to_string(&torrent_peer).unwrap();

            assert_eq!(
                json_serialized_value,
                // todo: compare using pretty json format to improve readability
                r#"{"peer_id":{"id":"2d71423030303030303030303030303030303030","client":"qBittorrent"},"peer_addr":"126.0.0.1:8080","updated":0,"uploaded":0,"downloaded":0,"left":0,"event":"Started"}"#
            );
        }
    }

    mod torrent_peer_constructor_from_udp_requests {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use aquatic_udp_protocol::{
            AnnounceEvent, AnnounceRequest, NumberOfBytes, NumberOfPeers, PeerId as AquaticPeerId, PeerKey, Port, TransactionId,
        };

        use crate::tracker::assign_ip_address_to_peer;
        use crate::tracker::peer::Peer;
        use crate::udp::connection_cookie::{into_connection_id, make};

        // todo: duplicate functions is PR 82. Remove duplication once both PR are merged.

        fn sample_ipv4_remote_addr() -> SocketAddr {
            sample_ipv4_socket_address()
        }

        fn sample_ipv4_socket_address() -> SocketAddr {
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
        }

        struct AnnounceRequestBuilder {
            request: AnnounceRequest,
        }

        impl AnnounceRequestBuilder {
            pub fn default() -> AnnounceRequestBuilder {
                let client_ip = Ipv4Addr::new(126, 0, 0, 1);
                let client_port = 8080;
                let info_hash_aquatic = aquatic_udp_protocol::InfoHash([0u8; 20]);

                let default_request = AnnounceRequest {
                    connection_id: into_connection_id(&make(&sample_ipv4_remote_addr())),
                    transaction_id: TransactionId(0i32),
                    info_hash: info_hash_aquatic,
                    peer_id: AquaticPeerId(*b"-qB00000000000000000"),
                    bytes_downloaded: NumberOfBytes(0i64),
                    bytes_uploaded: NumberOfBytes(0i64),
                    bytes_left: NumberOfBytes(0i64),
                    event: AnnounceEvent::Started,
                    ip_address: Some(client_ip),
                    key: PeerKey(0u32),
                    peers_wanted: NumberOfPeers(1i32),
                    port: Port(client_port),
                };
                AnnounceRequestBuilder {
                    request: default_request,
                }
            }

            pub fn into(self) -> AnnounceRequest {
                self.request
            }
        }

        #[test]
        fn it_should_use_the_udp_source_ip_as_the_peer_ip_address_instead_of_the_ip_in_the_announce_request() {
            let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));
            let announce_request = AnnounceRequestBuilder::default().into();

            let peer_ip = assign_ip_address_to_peer(&remote_ip, None);
            let peer_socket_address = SocketAddr::new(peer_ip, announce_request.port.0);

            let torrent_peer = Peer::from_udp_announce_request(&announce_request, &peer_socket_address);

            assert_eq!(torrent_peer.peer_addr, SocketAddr::new(remote_ip, announce_request.port.0));
        }

        #[test]
        fn it_should_always_use_the_port_in_the_announce_request_for_the_peer_port() {
            let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));
            let announce_request = AnnounceRequestBuilder::default().into();

            let peer_ip = assign_ip_address_to_peer(&remote_ip, None);
            let peer_socket_address = SocketAddr::new(peer_ip, announce_request.port.0);

            let torrent_peer = Peer::from_udp_announce_request(&announce_request, &peer_socket_address);

            assert_eq!(torrent_peer.peer_addr, SocketAddr::new(remote_ip, announce_request.port.0));
        }
    }

    mod torrent_peer_constructor_from_for_http_requests {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use crate::http::warp_implementation::request::Announce;
        use crate::protocol::info_hash::InfoHash;
        use crate::tracker::assign_ip_address_to_peer;
        use crate::tracker::peer::{self, Peer};

        fn sample_http_announce_request(peer_addr: IpAddr, port: u16) -> Announce {
            Announce {
                info_hash: InfoHash([0u8; 20]),
                peer_addr,
                downloaded: 0u64,
                uploaded: 0u64,
                peer_id: peer::Id(*b"-qB00000000000000000"),
                port,
                left: 0u64,
                event: None,
                compact: None,
            }
        }

        #[test]
        fn it_should_use_the_source_ip_in_the_udp_header_as_the_peer_ip_address_ignoring_the_peer_ip_in_the_announce_request() {
            let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));

            let ip_in_announce_request = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1));
            let announce_request = sample_http_announce_request(ip_in_announce_request, 8080);

            let peer_ip = assign_ip_address_to_peer(&remote_ip, None);
            let peer_socket_address = SocketAddr::new(peer_ip, announce_request.port);

            let torrent_peer = Peer::from_http_announce_request(&announce_request, &peer_socket_address);

            assert_eq!(torrent_peer.peer_addr.ip(), remote_ip);
            assert_ne!(torrent_peer.peer_addr.ip(), ip_in_announce_request);
        }

        #[test]
        fn it_should_always_use_the_port_in_the_announce_request_for_the_peer_port_ignoring_the_port_in_the_udp_header() {
            let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));
            let remote_port = 8080;

            let port_in_announce_request = 8081;
            let announce_request =
                sample_http_announce_request(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), port_in_announce_request);

            let peer_ip = assign_ip_address_to_peer(&remote_ip, None);
            let peer_socket_address = SocketAddr::new(peer_ip, announce_request.port);

            let torrent_peer = Peer::from_http_announce_request(&announce_request, &peer_socket_address);

            assert_eq!(torrent_peer.peer_addr.port(), announce_request.port);
            assert_ne!(torrent_peer.peer_addr.port(), remote_port);
        }
    }
}
