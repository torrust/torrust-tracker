use std::net::{IpAddr, SocketAddr};

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use serde;
use serde::Serialize;

use crate::http::request::Announce;
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
    pub left: NumberOfBytes,
    #[serde(with = "AnnounceEventDef")]
    pub event: AnnounceEvent,
}

impl Peer {
    #[must_use]
    pub fn from_udp_announce_request(
        announce_request: &aquatic_udp_protocol::AnnounceRequest,
        remote_ip: IpAddr,
        host_opt_ip: Option<IpAddr>,
    ) -> Self {
        let peer_addr = Peer::peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip, host_opt_ip, announce_request.port.0);

        Peer {
            peer_id: Id(announce_request.peer_id.0),
            peer_addr,
            updated: Current::now(),
            uploaded: announce_request.bytes_uploaded,
            downloaded: announce_request.bytes_downloaded,
            left: announce_request.bytes_left,
            event: announce_request.event,
        }
    }

    #[must_use]
    pub fn from_http_announce_request(announce_request: &Announce, remote_ip: IpAddr, host_opt_ip: Option<IpAddr>) -> Self {
        let peer_addr = Peer::peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip, host_opt_ip, announce_request.port);

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
            peer_addr,
            updated: Current::now(),
            uploaded: NumberOfBytes(i128::from(announce_request.uploaded) as i64),
            downloaded: NumberOfBytes(i128::from(announce_request.downloaded) as i64),
            left: NumberOfBytes(i128::from(announce_request.left) as i64),
            event,
        }
    }

    // potentially substitute localhost ip with external ip
    #[must_use]
    pub fn peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip: IpAddr, host_opt_ip: Option<IpAddr>, port: u16) -> SocketAddr {
        if let Some(host_ip) = host_opt_ip.filter(|_| remote_ip.is_loopback()) {
            SocketAddr::new(host_ip, port)
        } else {
            SocketAddr::new(remote_ip, port)
        }
    }

    #[must_use]
    pub fn is_seeder(&self) -> bool {
        self.left.0 <= 0 && self.event != AnnounceEvent::Stopped
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Copy)]
pub struct Id(pub [u8; 20]);

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

            let torrent_peer = Peer::from_udp_announce_request(&announce_request, remote_ip, None);

            assert_eq!(torrent_peer.peer_addr, SocketAddr::new(remote_ip, announce_request.port.0));
        }

        #[test]
        fn it_should_always_use_the_port_in_the_announce_request_for_the_peer_port() {
            let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));
            let announce_request = AnnounceRequestBuilder::default().into();

            let torrent_peer = Peer::from_udp_announce_request(&announce_request, remote_ip, None);

            assert_eq!(torrent_peer.peer_addr, SocketAddr::new(remote_ip, announce_request.port.0));
        }

        mod when_source_udp_ip_is_a_ipv_4_loopback_ip {

            use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
            use std::str::FromStr;

            use crate::tracker::peer::test::torrent_peer_constructor_from_udp_requests::AnnounceRequestBuilder;
            use crate::tracker::peer::Peer;

            #[test]
            fn it_should_use_the_loopback_ip_if_the_server_does_not_have_the_external_ip_configuration() {
                let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let torrent_peer = Peer::from_udp_announce_request(&announce_request, remote_ip, None);

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(remote_ip, announce_request.port.0));
            }

            #[test]
            fn it_should_use_the_external_host_ip_in_tracker_configuration_if_defined() {
                let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let host_opt_ip = IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap());
                let torrent_peer = Peer::from_udp_announce_request(&announce_request, remote_ip, Some(host_opt_ip));

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(host_opt_ip, announce_request.port.0));
            }

            #[test]
            fn it_should_use_the_external_ip_in_tracker_configuration_if_defined_even_if_the_external_ip_is_an_ipv6_ip() {
                let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let host_opt_ip = IpAddr::V6(Ipv6Addr::from_str("2345:0425:2CA1:0000:0000:0567:5673:23b5").unwrap());
                let torrent_peer = Peer::from_udp_announce_request(&announce_request, remote_ip, Some(host_opt_ip));

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(host_opt_ip, announce_request.port.0));
            }
        }

        mod when_source_udp_ip_is_a_ipv6_loopback_ip {

            use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
            use std::str::FromStr;

            use crate::tracker::peer::test::torrent_peer_constructor_from_udp_requests::AnnounceRequestBuilder;
            use crate::tracker::peer::Peer;

            #[test]
            fn it_should_use_the_loopback_ip_if_the_server_does_not_have_the_external_ip_configuration() {
                let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let torrent_peer = Peer::from_udp_announce_request(&announce_request, remote_ip, None);

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(remote_ip, announce_request.port.0));
            }

            #[test]
            fn it_should_use_the_external_host_ip_in_tracker_configuration_if_defined() {
                let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let host_opt_ip = IpAddr::V6(Ipv6Addr::from_str("2345:0425:2CA1:0000:0000:0567:5673:23b5").unwrap());
                let torrent_peer = Peer::from_udp_announce_request(&announce_request, remote_ip, Some(host_opt_ip));

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(host_opt_ip, announce_request.port.0));
            }

            #[test]
            fn it_should_use_the_external_ip_in_tracker_configuration_if_defined_even_if_the_external_ip_is_an_ipv4_ip() {
                let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let host_opt_ip = IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap());
                let torrent_peer = Peer::from_udp_announce_request(&announce_request, remote_ip, Some(host_opt_ip));

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(host_opt_ip, announce_request.port.0));
            }
        }
    }

    mod torrent_peer_constructor_from_for_http_requests {
        use std::net::{IpAddr, Ipv4Addr};

        use crate::http::request::Announce;
        use crate::protocol::info_hash::InfoHash;
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
        fn it_should_use_the_source_ip_in_the_udp_heder_as_the_peer_ip_address_ignoring_the_peer_ip_in_the_announce_request() {
            let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));

            let ip_in_announce_request = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1));
            let announce_request = sample_http_announce_request(ip_in_announce_request, 8080);

            let torrent_peer = Peer::from_http_announce_request(&announce_request, remote_ip, None);

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

            let torrent_peer = Peer::from_http_announce_request(&announce_request, remote_ip, None);

            assert_eq!(torrent_peer.peer_addr.port(), announce_request.port);
            assert_ne!(torrent_peer.peer_addr.port(), remote_port);
        }

        // todo: other cases are already covered by UDP cases.
        // Code review:
        // We should extract the method "peer_addr_from_ip_and_port_and_opt_host_ip" from TorrentPeer.
        // It could be another service responsible for assigning the IP to the peer.
        // So we can test that behavior independently from where you use it.
        // We could also build the peer with the IP in the announce request and let the tracker decide
        // wether it has to change it or not depending on tracker configuration.
    }
}
