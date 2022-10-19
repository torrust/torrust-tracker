use std::net::{IpAddr, SocketAddr};

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use serde;
use serde::Serialize;

use crate::http::AnnounceRequest;
use crate::protocol::clock::{DefaultClock, DurationSinceUnixEpoch, Time};
use crate::protocol::common::{AnnounceEventDef, NumberOfBytesDef, PeerId};
use crate::protocol::utils::ser_unix_time_value;

#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct TorrentPeer {
    pub peer_id: PeerId,
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

impl TorrentPeer {
    pub fn from_udp_announce_request(
        announce_request: &aquatic_udp_protocol::AnnounceRequest,
        remote_ip: IpAddr,
        host_opt_ip: Option<IpAddr>,
    ) -> Self {
        let peer_addr = TorrentPeer::peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip, host_opt_ip, announce_request.port.0);

        TorrentPeer {
            peer_id: PeerId(announce_request.peer_id.0),
            peer_addr,
            updated: DefaultClock::now(),
            uploaded: announce_request.bytes_uploaded,
            downloaded: announce_request.bytes_downloaded,
            left: announce_request.bytes_left,
            event: announce_request.event,
        }
    }

    pub fn from_http_announce_request(
        announce_request: &AnnounceRequest,
        remote_ip: IpAddr,
        host_opt_ip: Option<IpAddr>,
    ) -> Self {
        let peer_addr = TorrentPeer::peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip, host_opt_ip, announce_request.port);

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

        TorrentPeer {
            peer_id: announce_request.peer_id.clone(),
            peer_addr,
            updated: DefaultClock::now(),
            uploaded: NumberOfBytes(announce_request.uploaded as i64),
            downloaded: NumberOfBytes(announce_request.downloaded as i64),
            left: NumberOfBytes(announce_request.left as i64),
            event,
        }
    }

    // potentially substitute localhost ip with external ip
    pub fn peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip: IpAddr, host_opt_ip: Option<IpAddr>, port: u16) -> SocketAddr {
        if remote_ip.is_loopback() && host_opt_ip.is_some() {
            SocketAddr::new(host_opt_ip.unwrap(), port)
        } else {
            SocketAddr::new(remote_ip, port)
        }
    }

    pub fn is_seeder(&self) -> bool {
        self.left.0 <= 0 && self.event != AnnounceEvent::Stopped
    }
}

#[cfg(test)]
mod test {
    mod torrent_peer {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};

        use crate::peer::TorrentPeer;
        use crate::protocol::clock::{DefaultClock, Time};
        use crate::PeerId;

        #[test]
        fn it_should_be_serializable() {
            let torrent_peer = TorrentPeer {
                peer_id: PeerId(*b"-qB00000000000000000"),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
                updated: DefaultClock::now(),
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

        use crate::peer::TorrentPeer;
        use crate::udp::connection_cookie::{into_connection_id, ConnectionCookie, HashedConnectionCookie};
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
                    connection_id: into_connection_id(
                        &HashedConnectionCookie::make_connection_cookie(&sample_ipv4_remote_addr()),
                    ),
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

            let torrent_peer = TorrentPeer::from_udp_announce_request(&announce_request, remote_ip, None);

            assert_eq!(torrent_peer.peer_addr, SocketAddr::new(remote_ip, announce_request.port.0));
        }

        #[test]
        fn it_should_always_use_the_port_in_the_announce_request_for_the_peer_port() {
            let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));
            let announce_request = AnnounceRequestBuilder::default().into();

            let torrent_peer = TorrentPeer::from_udp_announce_request(&announce_request, remote_ip, None);

            assert_eq!(torrent_peer.peer_addr, SocketAddr::new(remote_ip, announce_request.port.0));
        }

        mod when_source_udp_ip_is_a_ipv_4_loopback_ip {

            use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
            use std::str::FromStr;

            use crate::peer::test::torrent_peer_constructor_from_udp_requests::AnnounceRequestBuilder;
            use crate::peer::TorrentPeer;

            #[test]
            fn it_should_use_the_loopback_ip_if_the_server_does_not_have_the_external_ip_configuration() {
                let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let torrent_peer = TorrentPeer::from_udp_announce_request(&announce_request, remote_ip, None);

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(remote_ip, announce_request.port.0));
            }

            #[test]
            fn it_should_use_the_external_host_ip_in_tracker_configuration_if_defined() {
                let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let host_opt_ip = IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap());
                let torrent_peer = TorrentPeer::from_udp_announce_request(&announce_request, remote_ip, Some(host_opt_ip));

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(host_opt_ip, announce_request.port.0));
            }

            #[test]
            fn it_should_use_the_external_ip_in_tracker_configuration_if_defined_even_if_the_external_ip_is_an_ipv6_ip() {
                let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let host_opt_ip = IpAddr::V6(Ipv6Addr::from_str("2345:0425:2CA1:0000:0000:0567:5673:23b5").unwrap());
                let torrent_peer = TorrentPeer::from_udp_announce_request(&announce_request, remote_ip, Some(host_opt_ip));

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(host_opt_ip, announce_request.port.0));
            }
        }

        mod when_source_udp_ip_is_a_ipv6_loopback_ip {

            use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
            use std::str::FromStr;

            use crate::peer::test::torrent_peer_constructor_from_udp_requests::AnnounceRequestBuilder;
            use crate::peer::TorrentPeer;

            #[test]
            fn it_should_use_the_loopback_ip_if_the_server_does_not_have_the_external_ip_configuration() {
                let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let torrent_peer = TorrentPeer::from_udp_announce_request(&announce_request, remote_ip, None);

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(remote_ip, announce_request.port.0));
            }

            #[test]
            fn it_should_use_the_external_host_ip_in_tracker_configuration_if_defined() {
                let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let host_opt_ip = IpAddr::V6(Ipv6Addr::from_str("2345:0425:2CA1:0000:0000:0567:5673:23b5").unwrap());
                let torrent_peer = TorrentPeer::from_udp_announce_request(&announce_request, remote_ip, Some(host_opt_ip));

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(host_opt_ip, announce_request.port.0));
            }

            #[test]
            fn it_should_use_the_external_ip_in_tracker_configuration_if_defined_even_if_the_external_ip_is_an_ipv4_ip() {
                let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
                let announce_request = AnnounceRequestBuilder::default().into();

                let host_opt_ip = IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap());
                let torrent_peer = TorrentPeer::from_udp_announce_request(&announce_request, remote_ip, Some(host_opt_ip));

                assert_eq!(torrent_peer.peer_addr, SocketAddr::new(host_opt_ip, announce_request.port.0));
            }
        }
    }

    mod torrent_peer_constructor_from_for_http_requests {
        use std::net::{IpAddr, Ipv4Addr};

        use crate::http::AnnounceRequest;
        use crate::peer::TorrentPeer;
        use crate::{InfoHash, PeerId};

        fn sample_http_announce_request(peer_addr: IpAddr, port: u16) -> AnnounceRequest {
            AnnounceRequest {
                info_hash: InfoHash([0u8; 20]),
                peer_addr,
                downloaded: 0u64,
                uploaded: 0u64,
                peer_id: PeerId(*b"-qB00000000000000000"),
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

            let torrent_peer = TorrentPeer::from_http_announce_request(&announce_request, remote_ip, None);

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

            let torrent_peer = TorrentPeer::from_http_announce_request(&announce_request, remote_ip, None);

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
