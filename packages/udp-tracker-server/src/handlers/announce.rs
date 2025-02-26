//! UDP tracker announce handler.
use std::net::{IpAddr, SocketAddr};
use std::ops::Range;
use std::sync::Arc;

use aquatic_udp_protocol::{
    AnnounceInterval, AnnounceRequest, AnnounceResponse, AnnounceResponseFixedData, Ipv4AddrBytes, Ipv6AddrBytes, NumberOfPeers,
    Port, Response, ResponsePeer, TransactionId,
};
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::announce_handler::AnnounceHandler;
use bittorrent_tracker_core::whitelist;
use bittorrent_udp_tracker_core::{services, statistics as core_statistics};
use torrust_tracker_configuration::Core;
use torrust_tracker_primitives::core::AnnounceData;
use tracing::{instrument, Level};
use zerocopy::network_endian::I32;

use crate::error::Error;
use crate::statistics as server_statistics;
use crate::statistics::event::UdpResponseKind;

/// It handles the `Announce` request.
///
/// # Errors
///
/// If a error happens in the `handle_announce` function, it will just return the  `ServerError`.
#[allow(clippy::too_many_arguments)]
#[instrument(fields(transaction_id, connection_id, info_hash), skip(announce_handler, whitelist_authorization, opt_udp_core_stats_event_sender, opt_udp_server_stats_event_sender), ret(level = Level::TRACE))]
pub async fn handle_announce(
    remote_addr: SocketAddr,
    request: &AnnounceRequest,
    core_config: &Arc<Core>,
    announce_handler: &Arc<AnnounceHandler>,
    whitelist_authorization: &Arc<whitelist::authorization::WhitelistAuthorization>,
    opt_udp_core_stats_event_sender: &Arc<Option<Box<dyn core_statistics::event::sender::Sender>>>,
    opt_udp_server_stats_event_sender: &Arc<Option<Box<dyn server_statistics::event::sender::Sender>>>,
    cookie_valid_range: Range<f64>,
) -> Result<Response, (Error, TransactionId)> {
    tracing::Span::current()
        .record("transaction_id", request.transaction_id.0.to_string())
        .record("connection_id", request.connection_id.0.to_string())
        .record("info_hash", InfoHash::from_bytes(&request.info_hash.0).to_hex_string());

    tracing::trace!("handle announce");

    if let Some(udp_server_stats_event_sender) = opt_udp_server_stats_event_sender.as_deref() {
        match remote_addr.ip() {
            IpAddr::V4(_) => {
                udp_server_stats_event_sender
                    .send_event(server_statistics::event::Event::Udp4Request {
                        kind: UdpResponseKind::Announce,
                    })
                    .await;
            }
            IpAddr::V6(_) => {
                udp_server_stats_event_sender
                    .send_event(server_statistics::event::Event::Udp6Request {
                        kind: UdpResponseKind::Announce,
                    })
                    .await;
            }
        }
    }

    let announce_data = services::announce::handle_announce(
        remote_addr,
        request,
        announce_handler,
        whitelist_authorization,
        opt_udp_core_stats_event_sender,
        cookie_valid_range,
    )
    .await
    .map_err(|e| (e.into(), request.transaction_id))?;

    Ok(build_response(remote_addr, request, core_config, &announce_data))
}

fn build_response(
    remote_addr: SocketAddr,
    request: &AnnounceRequest,
    core_config: &Arc<Core>,
    announce_data: &AnnounceData,
) -> Response {
    #[allow(clippy::cast_possible_truncation)]
    if remote_addr.is_ipv4() {
        let announce_response = AnnounceResponse {
            fixed: AnnounceResponseFixedData {
                transaction_id: request.transaction_id,
                announce_interval: AnnounceInterval(I32::new(i64::from(core_config.announce_policy.interval) as i32)),
                leechers: NumberOfPeers(I32::new(i64::from(announce_data.stats.incomplete) as i32)),
                seeders: NumberOfPeers(I32::new(i64::from(announce_data.stats.complete) as i32)),
            },
            peers: announce_data
                .peers
                .iter()
                .filter_map(|peer| {
                    if let IpAddr::V4(ip) = peer.peer_addr.ip() {
                        Some(ResponsePeer::<Ipv4AddrBytes> {
                            ip_address: ip.into(),
                            port: Port(peer.peer_addr.port().into()),
                        })
                    } else {
                        None
                    }
                })
                .collect(),
        };

        Response::from(announce_response)
    } else {
        let announce_response = AnnounceResponse {
            fixed: AnnounceResponseFixedData {
                transaction_id: request.transaction_id,
                announce_interval: AnnounceInterval(I32::new(i64::from(core_config.announce_policy.interval) as i32)),
                leechers: NumberOfPeers(I32::new(i64::from(announce_data.stats.incomplete) as i32)),
                seeders: NumberOfPeers(I32::new(i64::from(announce_data.stats.complete) as i32)),
            },
            peers: announce_data
                .peers
                .iter()
                .filter_map(|peer| {
                    if let IpAddr::V6(ip) = peer.peer_addr.ip() {
                        Some(ResponsePeer::<Ipv6AddrBytes> {
                            ip_address: ip.into(),
                            port: Port(peer.peer_addr.port().into()),
                        })
                    } else {
                        None
                    }
                })
                .collect(),
        };

        Response::from(announce_response)
    }
}

#[cfg(test)]
mod tests {

    mod announce_request {

        use std::net::Ipv4Addr;
        use std::num::NonZeroU16;

        use aquatic_udp_protocol::{
            AnnounceActionPlaceholder, AnnounceEvent, AnnounceRequest, ConnectionId, NumberOfBytes, NumberOfPeers,
            PeerId as AquaticPeerId, PeerKey, Port, TransactionId,
        };
        use bittorrent_udp_tracker_core::connection_cookie::make;

        use crate::handlers::tests::{sample_ipv4_remote_addr_fingerprint, sample_issue_time};

        struct AnnounceRequestBuilder {
            request: AnnounceRequest,
        }

        impl AnnounceRequestBuilder {
            pub fn default() -> AnnounceRequestBuilder {
                let client_ip = Ipv4Addr::new(126, 0, 0, 1);
                let client_port = 8080;
                let info_hash_aquatic = aquatic_udp_protocol::InfoHash([0u8; 20]);

                let default_request = AnnounceRequest {
                    connection_id: make(sample_ipv4_remote_addr_fingerprint(), sample_issue_time()).unwrap(),
                    action_placeholder: AnnounceActionPlaceholder::default(),
                    transaction_id: TransactionId(0i32.into()),
                    info_hash: info_hash_aquatic,
                    peer_id: AquaticPeerId([255u8; 20]),
                    bytes_downloaded: NumberOfBytes(0i64.into()),
                    bytes_uploaded: NumberOfBytes(0i64.into()),
                    bytes_left: NumberOfBytes(0i64.into()),
                    event: AnnounceEvent::Started.into(),
                    ip_address: client_ip.into(),
                    key: PeerKey::new(0i32),
                    peers_wanted: NumberOfPeers::new(1i32),
                    port: Port::new(NonZeroU16::new(client_port).expect("a non-zero client port")),
                };
                AnnounceRequestBuilder {
                    request: default_request,
                }
            }

            pub fn with_connection_id(mut self, connection_id: ConnectionId) -> Self {
                self.request.connection_id = connection_id;
                self
            }

            pub fn with_info_hash(mut self, info_hash: aquatic_udp_protocol::InfoHash) -> Self {
                self.request.info_hash = info_hash;
                self
            }

            pub fn with_peer_id(mut self, peer_id: AquaticPeerId) -> Self {
                self.request.peer_id = peer_id;
                self
            }

            pub fn with_ip_address(mut self, ip_address: Ipv4Addr) -> Self {
                self.request.ip_address = ip_address.into();
                self
            }

            pub fn with_port(mut self, port: u16) -> Self {
                self.request.port = Port(port.into());
                self
            }

            pub fn into(self) -> AnnounceRequest {
                self.request
            }
        }

        mod using_ipv4 {

            use std::future;
            use std::net::{IpAddr, Ipv4Addr, SocketAddr};
            use std::sync::Arc;

            use aquatic_udp_protocol::{
                AnnounceInterval, AnnounceResponse, AnnounceResponseFixedData, InfoHash as AquaticInfoHash, Ipv4AddrBytes,
                Ipv6AddrBytes, NumberOfPeers, PeerId as AquaticPeerId, Response, ResponsePeer,
            };
            use bittorrent_tracker_core::announce_handler::AnnounceHandler;
            use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
            use bittorrent_tracker_core::whitelist;
            use bittorrent_udp_tracker_core::connection_cookie::{gen_remote_fingerprint, make};
            use bittorrent_udp_tracker_core::statistics as core_statistics;
            use mockall::predicate::eq;
            use torrust_tracker_configuration::Core;

            use crate::handlers::announce::tests::announce_request::AnnounceRequestBuilder;
            use crate::handlers::handle_announce;
            use crate::handlers::tests::{
                initialize_core_tracker_services_for_default_tracker_configuration,
                initialize_core_tracker_services_for_public_tracker, sample_cookie_valid_range, sample_ipv4_socket_address,
                sample_issue_time, MockUdpCoreStatsEventSender, MockUdpServerStatsEventSender, TorrentPeerBuilder,
            };
            use crate::statistics as server_statistics;
            use crate::statistics::event::UdpResponseKind;

            #[tokio::test]
            async fn an_announced_peer_should_be_added_to_the_tracker() {
                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                let client_ip = Ipv4Addr::new(126, 0, 0, 1);
                let client_port = 8080;
                let info_hash = AquaticInfoHash([0u8; 20]);
                let peer_id = AquaticPeerId([255u8; 20]);

                let remote_addr = SocketAddr::new(IpAddr::V4(client_ip), client_port);

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                    .with_info_hash(info_hash)
                    .with_peer_id(peer_id)
                    .with_ip_address(client_ip)
                    .with_port(client_port)
                    .into();

                handle_announce(
                    remote_addr,
                    &request,
                    &core_tracker_services.core_config,
                    &core_tracker_services.announce_handler,
                    &core_tracker_services.whitelist_authorization,
                    &core_udp_tracker_services.udp_core_stats_event_sender,
                    &server_udp_tracker_services.udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();

                let peers = core_tracker_services
                    .in_memory_torrent_repository
                    .get_torrent_peers(&info_hash.0.into());

                let expected_peer = TorrentPeerBuilder::new()
                    .with_peer_id(peer_id)
                    .with_peer_address(SocketAddr::new(IpAddr::V4(client_ip), client_port))
                    .updated_on(peers[0].updated)
                    .into();

                assert_eq!(peers[0], Arc::new(expected_peer));
            }

            #[tokio::test]
            async fn the_announced_peer_should_not_be_included_in_the_response() {
                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080);

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                    .into();

                let response = handle_announce(
                    remote_addr,
                    &request,
                    &core_tracker_services.core_config,
                    &core_tracker_services.announce_handler,
                    &core_tracker_services.whitelist_authorization,
                    &core_udp_tracker_services.udp_core_stats_event_sender,
                    &server_udp_tracker_services.udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();

                let empty_peer_vector: Vec<ResponsePeer<Ipv4AddrBytes>> = vec![];
                assert_eq!(
                    response,
                    Response::from(AnnounceResponse {
                        fixed: AnnounceResponseFixedData {
                            transaction_id: request.transaction_id,
                            announce_interval: AnnounceInterval(120i32.into()),
                            leechers: NumberOfPeers(0i32.into()),
                            seeders: NumberOfPeers(1i32.into()),
                        },
                        peers: empty_peer_vector
                    })
                );
            }

            #[tokio::test]
            async fn the_tracker_should_always_use_the_remote_client_ip_but_not_the_port_in_the_udp_request_header_instead_of_the_peer_address_in_the_announce_request(
            ) {
                // From the BEP 15 (https://www.bittorrent.org/beps/bep_0015.html):
                // "Do note that most trackers will only honor the IP address field under limited circumstances."

                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                let info_hash = AquaticInfoHash([0u8; 20]);
                let peer_id = AquaticPeerId([255u8; 20]);
                let client_port = 8080;

                let remote_client_ip = Ipv4Addr::new(126, 0, 0, 1);
                let remote_client_port = 8081;
                let peer_address = Ipv4Addr::new(126, 0, 0, 2);

                let remote_addr = SocketAddr::new(IpAddr::V4(remote_client_ip), remote_client_port);

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                    .with_info_hash(info_hash)
                    .with_peer_id(peer_id)
                    .with_ip_address(peer_address)
                    .with_port(client_port)
                    .into();

                handle_announce(
                    remote_addr,
                    &request,
                    &core_tracker_services.core_config,
                    &core_tracker_services.announce_handler,
                    &core_tracker_services.whitelist_authorization,
                    &core_udp_tracker_services.udp_core_stats_event_sender,
                    &server_udp_tracker_services.udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();

                let peers = core_tracker_services
                    .in_memory_torrent_repository
                    .get_torrent_peers(&info_hash.0.into());

                assert_eq!(peers[0].peer_addr, SocketAddr::new(IpAddr::V4(remote_client_ip), client_port));
            }

            fn add_a_torrent_peer_using_ipv6(in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>) {
                let info_hash = AquaticInfoHash([0u8; 20]);

                let client_ip_v4 = Ipv4Addr::new(126, 0, 0, 1);
                let client_ip_v6 = client_ip_v4.to_ipv6_compatible();
                let client_port = 8080;
                let peer_id = AquaticPeerId([255u8; 20]);

                let peer_using_ipv6 = TorrentPeerBuilder::new()
                    .with_peer_id(peer_id)
                    .with_peer_address(SocketAddr::new(IpAddr::V6(client_ip_v6), client_port))
                    .into();

                let () = in_memory_torrent_repository.upsert_peer(&info_hash.0.into(), &peer_using_ipv6);
            }

            async fn announce_a_new_peer_using_ipv4(
                core_config: Arc<Core>,
                announce_handler: Arc<AnnounceHandler>,
                whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
            ) -> Response {
                let (udp_core_stats_event_sender, _udp_core_stats_repository) =
                    bittorrent_udp_tracker_core::statistics::setup::factory(false);
                let udp_core_stats_event_sender = Arc::new(udp_core_stats_event_sender);

                let (udp_server_stats_event_sender, _udp_server_stats_repository) = crate::statistics::setup::factory(false);
                let udp_server_stats_event_sender = Arc::new(udp_server_stats_event_sender);

                let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080);
                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                    .into();

                handle_announce(
                    remote_addr,
                    &request,
                    &core_config,
                    &announce_handler,
                    &whitelist_authorization,
                    &udp_core_stats_event_sender,
                    &udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap()
            }

            #[tokio::test]
            async fn when_the_announce_request_comes_from_a_client_using_ipv4_the_response_should_not_include_peers_using_ipv6() {
                let (core_tracker_services, _core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                add_a_torrent_peer_using_ipv6(&core_tracker_services.in_memory_torrent_repository);

                let response = announce_a_new_peer_using_ipv4(
                    core_tracker_services.core_config.clone(),
                    core_tracker_services.announce_handler.clone(),
                    core_tracker_services.whitelist_authorization,
                )
                .await;

                // The response should not contain the peer using IPV6
                let peers: Option<Vec<ResponsePeer<Ipv6AddrBytes>>> = match response {
                    Response::AnnounceIpv6(announce_response) => Some(announce_response.peers),
                    _ => None,
                };
                let no_ipv6_peers = peers.is_none();
                assert!(no_ipv6_peers);
            }

            #[tokio::test]
            async fn should_send_the_upd4_announce_event() {
                let mut udp_core_stats_event_sender_mock = MockUdpCoreStatsEventSender::new();
                udp_core_stats_event_sender_mock
                    .expect_send_event()
                    .with(eq(core_statistics::event::Event::Udp4Announce))
                    .times(1)
                    .returning(|_| Box::pin(future::ready(Some(Ok(())))));
                let udp_core_stats_event_sender: Arc<Option<Box<dyn core_statistics::event::sender::Sender>>> =
                    Arc::new(Some(Box::new(udp_core_stats_event_sender_mock)));

                let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
                udp_server_stats_event_sender_mock
                    .expect_send_event()
                    .with(eq(server_statistics::event::Event::Udp4Request {
                        kind: UdpResponseKind::Announce,
                    }))
                    .times(1)
                    .returning(|_| Box::pin(future::ready(Some(Ok(())))));
                let udp_server_stats_event_sender: Arc<Option<Box<dyn server_statistics::event::sender::Sender>>> =
                    Arc::new(Some(Box::new(udp_server_stats_event_sender_mock)));

                let (core_tracker_services, _core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_default_tracker_configuration();

                handle_announce(
                    sample_ipv4_socket_address(),
                    &AnnounceRequestBuilder::default().into(),
                    &core_tracker_services.core_config,
                    &core_tracker_services.announce_handler,
                    &core_tracker_services.whitelist_authorization,
                    &udp_core_stats_event_sender,
                    &udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();
            }

            mod from_a_loopback_ip {
                use std::net::{IpAddr, Ipv4Addr, SocketAddr};
                use std::sync::Arc;

                use aquatic_udp_protocol::{InfoHash as AquaticInfoHash, PeerId as AquaticPeerId};
                use bittorrent_udp_tracker_core::connection_cookie::{gen_remote_fingerprint, make};

                use crate::handlers::announce::tests::announce_request::AnnounceRequestBuilder;
                use crate::handlers::handle_announce;
                use crate::handlers::tests::{
                    initialize_core_tracker_services_for_public_tracker, sample_cookie_valid_range, sample_issue_time,
                    TorrentPeerBuilder,
                };

                #[tokio::test]
                async fn the_peer_ip_should_be_changed_to_the_external_ip_in_the_tracker_configuration_if_defined() {
                    let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                        initialize_core_tracker_services_for_public_tracker();

                    let client_ip = Ipv4Addr::new(127, 0, 0, 1);
                    let client_port = 8080;
                    let info_hash = AquaticInfoHash([0u8; 20]);
                    let peer_id = AquaticPeerId([255u8; 20]);

                    let remote_addr = SocketAddr::new(IpAddr::V4(client_ip), client_port);

                    let request = AnnounceRequestBuilder::default()
                        .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                        .with_info_hash(info_hash)
                        .with_peer_id(peer_id)
                        .with_ip_address(client_ip)
                        .with_port(client_port)
                        .into();

                    handle_announce(
                        remote_addr,
                        &request,
                        &core_tracker_services.core_config,
                        &core_tracker_services.announce_handler,
                        &core_tracker_services.whitelist_authorization,
                        &core_udp_tracker_services.udp_core_stats_event_sender,
                        &server_udp_tracker_services.udp_server_stats_event_sender,
                        sample_cookie_valid_range(),
                    )
                    .await
                    .unwrap();

                    let peers = core_tracker_services
                        .in_memory_torrent_repository
                        .get_torrent_peers(&info_hash.0.into());

                    let external_ip_in_tracker_configuration = core_tracker_services.core_config.net.external_ip.unwrap();

                    let expected_peer = TorrentPeerBuilder::new()
                        .with_peer_id(peer_id)
                        .with_peer_address(SocketAddr::new(external_ip_in_tracker_configuration, client_port))
                        .updated_on(peers[0].updated)
                        .into();

                    assert_eq!(peers[0], Arc::new(expected_peer));
                }
            }
        }

        mod using_ipv6 {

            use std::future;
            use std::net::{IpAddr, Ipv4Addr, SocketAddr};
            use std::sync::Arc;

            use aquatic_udp_protocol::{
                AnnounceInterval, AnnounceResponse, AnnounceResponseFixedData, InfoHash as AquaticInfoHash, Ipv4AddrBytes,
                Ipv6AddrBytes, NumberOfPeers, PeerId as AquaticPeerId, Response, ResponsePeer,
            };
            use bittorrent_tracker_core::announce_handler::AnnounceHandler;
            use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
            use bittorrent_tracker_core::whitelist;
            use bittorrent_udp_tracker_core::connection_cookie::{gen_remote_fingerprint, make};
            use bittorrent_udp_tracker_core::statistics as core_statistics;
            use mockall::predicate::eq;
            use torrust_tracker_configuration::Core;

            use crate::handlers::announce::tests::announce_request::AnnounceRequestBuilder;
            use crate::handlers::handle_announce;
            use crate::handlers::tests::{
                initialize_core_tracker_services_for_default_tracker_configuration,
                initialize_core_tracker_services_for_public_tracker, sample_cookie_valid_range, sample_ipv6_remote_addr,
                sample_issue_time, MockUdpCoreStatsEventSender, MockUdpServerStatsEventSender, TorrentPeerBuilder,
            };
            use crate::statistics as server_statistics;
            use crate::statistics::event::UdpResponseKind;

            #[tokio::test]
            async fn an_announced_peer_should_be_added_to_the_tracker() {
                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                let client_ip_v4 = Ipv4Addr::new(126, 0, 0, 1);
                let client_ip_v6 = client_ip_v4.to_ipv6_compatible();
                let client_port = 8080;
                let info_hash = AquaticInfoHash([0u8; 20]);
                let peer_id = AquaticPeerId([255u8; 20]);

                let remote_addr = SocketAddr::new(IpAddr::V6(client_ip_v6), client_port);

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                    .with_info_hash(info_hash)
                    .with_peer_id(peer_id)
                    .with_ip_address(client_ip_v4)
                    .with_port(client_port)
                    .into();

                handle_announce(
                    remote_addr,
                    &request,
                    &core_tracker_services.core_config,
                    &core_tracker_services.announce_handler,
                    &core_tracker_services.whitelist_authorization,
                    &core_udp_tracker_services.udp_core_stats_event_sender,
                    &server_udp_tracker_services.udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();

                let peers = core_tracker_services
                    .in_memory_torrent_repository
                    .get_torrent_peers(&info_hash.0.into());

                let expected_peer = TorrentPeerBuilder::new()
                    .with_peer_id(peer_id)
                    .with_peer_address(SocketAddr::new(IpAddr::V6(client_ip_v6), client_port))
                    .updated_on(peers[0].updated)
                    .into();

                assert_eq!(peers[0], Arc::new(expected_peer));
            }

            #[tokio::test]
            async fn the_announced_peer_should_not_be_included_in_the_response() {
                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                let client_ip_v4 = Ipv4Addr::new(126, 0, 0, 1);
                let client_ip_v6 = client_ip_v4.to_ipv6_compatible();

                let remote_addr = SocketAddr::new(IpAddr::V6(client_ip_v6), 8080);

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                    .into();

                let response = handle_announce(
                    remote_addr,
                    &request,
                    &core_tracker_services.core_config,
                    &core_tracker_services.announce_handler,
                    &core_tracker_services.whitelist_authorization,
                    &core_udp_tracker_services.udp_core_stats_event_sender,
                    &server_udp_tracker_services.udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();

                let empty_peer_vector: Vec<ResponsePeer<Ipv6AddrBytes>> = vec![];
                assert_eq!(
                    response,
                    Response::from(AnnounceResponse {
                        fixed: AnnounceResponseFixedData {
                            transaction_id: request.transaction_id,
                            announce_interval: AnnounceInterval(120i32.into()),
                            leechers: NumberOfPeers(0i32.into()),
                            seeders: NumberOfPeers(1i32.into()),
                        },
                        peers: empty_peer_vector
                    })
                );
            }

            #[tokio::test]
            async fn the_tracker_should_always_use_the_remote_client_ip_but_not_the_port_in_the_udp_request_header_instead_of_the_peer_address_in_the_announce_request(
            ) {
                // From the BEP 15 (https://www.bittorrent.org/beps/bep_0015.html):
                // "Do note that most trackers will only honor the IP address field under limited circumstances."

                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_service) =
                    initialize_core_tracker_services_for_public_tracker();

                let info_hash = AquaticInfoHash([0u8; 20]);
                let peer_id = AquaticPeerId([255u8; 20]);
                let client_port = 8080;

                let remote_client_ip = "::100".parse().unwrap(); // IPV4 ::0.0.1.0 -> IPV6 = ::100 = ::ffff:0:100 = 0:0:0:0:0:ffff:0:0100
                let remote_client_port = 8081;
                let peer_address = "126.0.0.1".parse().unwrap();

                let remote_addr = SocketAddr::new(IpAddr::V6(remote_client_ip), remote_client_port);

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                    .with_info_hash(info_hash)
                    .with_peer_id(peer_id)
                    .with_ip_address(peer_address)
                    .with_port(client_port)
                    .into();

                handle_announce(
                    remote_addr,
                    &request,
                    &core_tracker_services.core_config,
                    &core_tracker_services.announce_handler,
                    &core_tracker_services.whitelist_authorization,
                    &core_udp_tracker_services.udp_core_stats_event_sender,
                    &server_udp_tracker_service.udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();

                let peers = core_tracker_services
                    .in_memory_torrent_repository
                    .get_torrent_peers(&info_hash.0.into());

                // When using IPv6 the tracker converts the remote client ip into a IPv4 address
                assert_eq!(peers[0].peer_addr, SocketAddr::new(IpAddr::V6(remote_client_ip), client_port));
            }

            fn add_a_torrent_peer_using_ipv4(in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>) {
                let info_hash = AquaticInfoHash([0u8; 20]);

                let client_ip_v4 = Ipv4Addr::new(126, 0, 0, 1);
                let client_port = 8080;
                let peer_id = AquaticPeerId([255u8; 20]);

                let peer_using_ipv4 = TorrentPeerBuilder::new()
                    .with_peer_id(peer_id)
                    .with_peer_address(SocketAddr::new(IpAddr::V4(client_ip_v4), client_port))
                    .into();

                let () = in_memory_torrent_repository.upsert_peer(&info_hash.0.into(), &peer_using_ipv4);
            }

            async fn announce_a_new_peer_using_ipv6(
                core_config: Arc<Core>,
                announce_handler: Arc<AnnounceHandler>,
                whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
            ) -> Response {
                let (udp_core_stats_event_sender, _udp_core_stats_repository) =
                    bittorrent_udp_tracker_core::statistics::setup::factory(false);
                let udp_core_stats_event_sender = Arc::new(udp_core_stats_event_sender);

                let (udp_server_stats_event_sender, _udp_server_stats_repository) = crate::statistics::setup::factory(false);
                let udp_server_stats_event_sender = Arc::new(udp_server_stats_event_sender);

                let client_ip_v4 = Ipv4Addr::new(126, 0, 0, 1);
                let client_ip_v6 = client_ip_v4.to_ipv6_compatible();
                let client_port = 8080;
                let remote_addr = SocketAddr::new(IpAddr::V6(client_ip_v6), client_port);
                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                    .into();

                handle_announce(
                    remote_addr,
                    &request,
                    &core_config,
                    &announce_handler,
                    &whitelist_authorization,
                    &udp_core_stats_event_sender,
                    &udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap()
            }

            #[tokio::test]
            async fn when_the_announce_request_comes_from_a_client_using_ipv6_the_response_should_not_include_peers_using_ipv4() {
                let (core_tracker_services, _core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                add_a_torrent_peer_using_ipv4(&core_tracker_services.in_memory_torrent_repository);

                let response = announce_a_new_peer_using_ipv6(
                    core_tracker_services.core_config.clone(),
                    core_tracker_services.announce_handler.clone(),
                    core_tracker_services.whitelist_authorization,
                )
                .await;

                // The response should not contain the peer using IPV4
                let peers: Option<Vec<ResponsePeer<Ipv4AddrBytes>>> = match response {
                    Response::AnnounceIpv4(announce_response) => Some(announce_response.peers),
                    _ => None,
                };
                let no_ipv4_peers = peers.is_none();
                assert!(no_ipv4_peers);
            }

            #[tokio::test]
            async fn should_send_the_upd6_announce_event() {
                let mut udp_core_stats_event_sender_mock = MockUdpCoreStatsEventSender::new();
                udp_core_stats_event_sender_mock
                    .expect_send_event()
                    .with(eq(core_statistics::event::Event::Udp6Announce))
                    .times(1)
                    .returning(|_| Box::pin(future::ready(Some(Ok(())))));
                let udp_core_stats_event_sender: Arc<Option<Box<dyn core_statistics::event::sender::Sender>>> =
                    Arc::new(Some(Box::new(udp_core_stats_event_sender_mock)));

                let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
                udp_server_stats_event_sender_mock
                    .expect_send_event()
                    .with(eq(server_statistics::event::Event::Udp6Request {
                        kind: UdpResponseKind::Announce,
                    }))
                    .times(1)
                    .returning(|_| Box::pin(future::ready(Some(Ok(())))));
                let udp_server_stats_event_sender: Arc<Option<Box<dyn server_statistics::event::sender::Sender>>> =
                    Arc::new(Some(Box::new(udp_server_stats_event_sender_mock)));

                let (core_tracker_services, _core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_default_tracker_configuration();

                let remote_addr = sample_ipv6_remote_addr();

                let announce_request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                    .into();

                handle_announce(
                    remote_addr,
                    &announce_request,
                    &core_tracker_services.core_config,
                    &core_tracker_services.announce_handler,
                    &core_tracker_services.whitelist_authorization,
                    &udp_core_stats_event_sender,
                    &udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();
            }

            mod from_a_loopback_ip {
                use std::future;
                use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
                use std::sync::Arc;

                use aquatic_udp_protocol::{InfoHash as AquaticInfoHash, PeerId as AquaticPeerId};
                use bittorrent_tracker_core::announce_handler::AnnounceHandler;
                use bittorrent_tracker_core::databases::setup::initialize_database;
                use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
                use bittorrent_tracker_core::torrent::repository::persisted::DatabasePersistentTorrentRepository;
                use bittorrent_tracker_core::whitelist::authorization::WhitelistAuthorization;
                use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
                use bittorrent_udp_tracker_core::connection_cookie::{gen_remote_fingerprint, make};
                use bittorrent_udp_tracker_core::{self, statistics as core_statistics};
                use mockall::predicate::eq;

                use crate::handlers::announce::tests::announce_request::AnnounceRequestBuilder;
                use crate::handlers::handle_announce;
                use crate::handlers::tests::{
                    sample_cookie_valid_range, sample_issue_time, MockUdpCoreStatsEventSender, MockUdpServerStatsEventSender,
                    TrackerConfigurationBuilder,
                };
                use crate::statistics as server_statistics;
                use crate::statistics::event::UdpResponseKind;

                #[tokio::test]
                async fn the_peer_ip_should_be_changed_to_the_external_ip_in_the_tracker_configuration() {
                    let config = Arc::new(TrackerConfigurationBuilder::default().with_external_ip("::126.0.0.1").into());

                    let database = initialize_database(&config.core);
                    let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
                    let whitelist_authorization =
                        Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
                    let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
                    let db_torrent_repository = Arc::new(DatabasePersistentTorrentRepository::new(&database));

                    let mut udp_core_stats_event_sender_mock = MockUdpCoreStatsEventSender::new();
                    udp_core_stats_event_sender_mock
                        .expect_send_event()
                        .with(eq(core_statistics::event::Event::Udp6Announce))
                        .times(1)
                        .returning(|_| Box::pin(future::ready(Some(Ok(())))));
                    let udp_core_stats_event_sender: Arc<Option<Box<dyn core_statistics::event::sender::Sender>>> =
                        Arc::new(Some(Box::new(udp_core_stats_event_sender_mock)));

                    let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
                    udp_server_stats_event_sender_mock
                        .expect_send_event()
                        .with(eq(server_statistics::event::Event::Udp6Request {
                            kind: UdpResponseKind::Announce,
                        }))
                        .times(1)
                        .returning(|_| Box::pin(future::ready(Some(Ok(())))));
                    let udp_server_stats_event_sender: Arc<Option<Box<dyn server_statistics::event::sender::Sender>>> =
                        Arc::new(Some(Box::new(udp_server_stats_event_sender_mock)));

                    let announce_handler = Arc::new(AnnounceHandler::new(
                        &config.core,
                        &whitelist_authorization,
                        &in_memory_torrent_repository,
                        &db_torrent_repository,
                    ));

                    let loopback_ipv4 = Ipv4Addr::new(127, 0, 0, 1);
                    let loopback_ipv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);

                    let client_ip_v4 = loopback_ipv4;
                    let client_ip_v6 = loopback_ipv6;
                    let client_port = 8080;

                    let info_hash = AquaticInfoHash([0u8; 20]);
                    let peer_id = AquaticPeerId([255u8; 20]);

                    let remote_addr = SocketAddr::new(IpAddr::V6(client_ip_v6), client_port);

                    let request = AnnounceRequestBuilder::default()
                        .with_connection_id(make(gen_remote_fingerprint(&remote_addr), sample_issue_time()).unwrap())
                        .with_info_hash(info_hash)
                        .with_peer_id(peer_id)
                        .with_ip_address(client_ip_v4)
                        .with_port(client_port)
                        .into();

                    let core_config = Arc::new(config.core.clone());

                    handle_announce(
                        remote_addr,
                        &request,
                        &core_config,
                        &announce_handler,
                        &whitelist_authorization,
                        &udp_core_stats_event_sender,
                        &udp_server_stats_event_sender,
                        sample_cookie_valid_range(),
                    )
                    .await
                    .unwrap();

                    let peers = in_memory_torrent_repository.get_torrent_peers(&info_hash.0.into());

                    let external_ip_in_tracker_configuration = core_config.net.external_ip.unwrap();

                    assert!(external_ip_in_tracker_configuration.is_ipv6());

                    // There's a special type of IPv6 addresses that provide compatibility with IPv4.
                    // The last 32 bits of these addresses represent an IPv4, and are represented like this:
                    // 1111:2222:3333:4444:5555:6666:1.2.3.4
                    //
                    // ::127.0.0.1 is the IPV6 representation for the IPV4 address 127.0.0.1.
                    assert_eq!(Ok(peers[0].peer_addr.ip()), "::126.0.0.1".parse());
                }
            }
        }
    }
}
