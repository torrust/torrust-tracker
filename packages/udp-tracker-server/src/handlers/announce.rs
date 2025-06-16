//! UDP tracker announce handler.
use std::net::{IpAddr, SocketAddr};
use std::ops::Range;
use std::sync::Arc;

use aquatic_udp_protocol::{
    AnnounceInterval, AnnounceRequest, AnnounceResponse, AnnounceResponseFixedData, Ipv4AddrBytes, Ipv6AddrBytes, NumberOfPeers,
    Port, Response, ResponsePeer, TransactionId,
};
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_udp_tracker_core::services::announce::AnnounceService;
use torrust_tracker_configuration::Core;
use torrust_tracker_primitives::core::AnnounceData;
use torrust_tracker_primitives::service_binding::ServiceBinding;
use tracing::{instrument, Level};
use zerocopy::network_endian::I32;

use crate::error::Error;
use crate::event::{ConnectionContext, Event, UdpRequestKind};

/// It handles the `Announce` request.
///
/// # Errors
///
/// If a error happens in the `handle_announce` function, it will just return the  `ServerError`.
#[instrument(fields(transaction_id, connection_id, info_hash), skip(announce_service, opt_udp_server_stats_event_sender), ret(level = Level::TRACE))]
pub async fn handle_announce(
    announce_service: &Arc<AnnounceService>,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    request: &AnnounceRequest,
    core_config: &Arc<Core>,
    opt_udp_server_stats_event_sender: &crate::event::sender::Sender,
    cookie_valid_range: Range<f64>,
) -> Result<Response, (Error, TransactionId, UdpRequestKind)> {
    tracing::Span::current()
        .record("transaction_id", request.transaction_id.0.to_string())
        .record("connection_id", request.connection_id.0.to_string())
        .record("info_hash", InfoHash::from_bytes(&request.info_hash.0).to_hex_string());

    tracing::trace!("handle announce");

    if let Some(udp_server_stats_event_sender) = opt_udp_server_stats_event_sender.as_deref() {
        udp_server_stats_event_sender
            .send(Event::UdpRequestAccepted {
                context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                kind: UdpRequestKind::Announce {
                    announce_request: *request,
                },
            })
            .await;
    }

    let announce_data = announce_service
        .handle_announce(client_socket_addr, server_service_binding, request, cookie_valid_range)
        .await
        .map_err(|e| {
            (
                e.into(),
                request.transaction_id,
                UdpRequestKind::Announce {
                    announce_request: *request,
                },
            )
        })?;

    Ok(build_response(client_socket_addr, request, core_config, &announce_data))
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
pub(crate) mod tests {

    pub mod announce_request {

        use std::net::Ipv4Addr;
        use std::num::NonZeroU16;

        use aquatic_udp_protocol::{
            AnnounceActionPlaceholder, AnnounceEvent, AnnounceRequest, ConnectionId, NumberOfBytes, NumberOfPeers,
            PeerId as AquaticPeerId, PeerKey, Port, TransactionId,
        };
        use bittorrent_udp_tracker_core::connection_cookie::make;

        use crate::handlers::tests::{sample_ipv4_remote_addr_fingerprint, sample_issue_time};

        pub struct AnnounceRequestBuilder {
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
            use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
            use bittorrent_udp_tracker_core::connection_cookie::{gen_remote_fingerprint, make};
            use mockall::predicate::eq;
            use torrust_tracker_events::bus::SenderStatus;
            use torrust_tracker_primitives::peer::fixture::PeerBuilder;
            use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

            use crate::event::{ConnectionContext, Event, UdpRequestKind};
            use crate::handlers::announce::tests::announce_request::AnnounceRequestBuilder;
            use crate::handlers::handle_announce;
            use crate::handlers::tests::{
                initialize_core_tracker_services_for_default_tracker_configuration,
                initialize_core_tracker_services_for_public_tracker, sample_cookie_valid_range, sample_ipv4_socket_address,
                sample_issue_time, CoreTrackerServices, CoreUdpTrackerServices, MockUdpServerStatsEventSender,
            };

            #[tokio::test]
            async fn an_announced_peer_should_be_added_to_the_tracker() {
                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                let client_ip = Ipv4Addr::new(126, 0, 0, 1);
                let client_port = 8080;
                let info_hash = AquaticInfoHash([0u8; 20]);
                let peer_id = AquaticPeerId([255u8; 20]);

                let client_socket_addr = SocketAddr::new(IpAddr::V4(client_ip), client_port);
                let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                    .with_info_hash(info_hash)
                    .with_peer_id(peer_id)
                    .with_ip_address(client_ip)
                    .with_port(client_port)
                    .into();

                handle_announce(
                    &core_udp_tracker_services.announce_service,
                    client_socket_addr,
                    server_service_binding,
                    &request,
                    &core_tracker_services.core_config,
                    &server_udp_tracker_services.udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();

                let peers = core_tracker_services
                    .in_memory_torrent_repository
                    .get_torrent_peers(&info_hash.0.into())
                    .await;

                let expected_peer = PeerBuilder::default()
                    .with_peer_id(&peer_id)
                    .with_peer_address(SocketAddr::new(IpAddr::V4(client_ip), client_port))
                    .updated_on(peers[0].updated)
                    .into();

                assert_eq!(peers[0], Arc::new(expected_peer));
            }

            #[tokio::test]
            async fn the_announced_peer_should_not_be_included_in_the_response() {
                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                let client_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080);
                let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                    .into();

                let response = handle_announce(
                    &core_udp_tracker_services.announce_service,
                    client_socket_addr,
                    server_service_binding,
                    &request,
                    &core_tracker_services.core_config,
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

                let client_socket_addr = SocketAddr::new(IpAddr::V4(remote_client_ip), remote_client_port);
                let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                    .with_info_hash(info_hash)
                    .with_peer_id(peer_id)
                    .with_ip_address(peer_address)
                    .with_port(client_port)
                    .into();

                handle_announce(
                    &core_udp_tracker_services.announce_service,
                    client_socket_addr,
                    server_service_binding,
                    &request,
                    &core_tracker_services.core_config,
                    &server_udp_tracker_services.udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();

                let peers = core_tracker_services
                    .in_memory_torrent_repository
                    .get_torrent_peers(&info_hash.0.into())
                    .await;

                assert_eq!(peers[0].peer_addr, SocketAddr::new(IpAddr::V4(remote_client_ip), client_port));
            }

            async fn add_a_torrent_peer_using_ipv6(in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>) {
                let info_hash = AquaticInfoHash([0u8; 20]);

                let client_ip_v4 = Ipv4Addr::new(126, 0, 0, 1);
                let client_ip_v6 = client_ip_v4.to_ipv6_compatible();
                let client_port = 8080;
                let peer_id = AquaticPeerId([255u8; 20]);

                let peer_using_ipv6 = PeerBuilder::default()
                    .with_peer_id(&peer_id)
                    .with_peer_address(SocketAddr::new(IpAddr::V6(client_ip_v6), client_port))
                    .into();

                in_memory_torrent_repository
                    .handle_announcement(&info_hash.0.into(), &peer_using_ipv6, None)
                    .await;
            }

            async fn announce_a_new_peer_using_ipv4(
                core_tracker_services: Arc<CoreTrackerServices>,
                core_udp_tracker_services: Arc<CoreUdpTrackerServices>,
            ) -> Response {
                let udp_server_broadcaster = crate::event::sender::Broadcaster::default();
                let event_bus = Arc::new(crate::event::bus::EventBus::new(
                    SenderStatus::Disabled,
                    udp_server_broadcaster.clone(),
                ));

                let udp_server_stats_event_sender = event_bus.sender();

                let client_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080);
                let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                    .into();

                handle_announce(
                    &core_udp_tracker_services.announce_service,
                    client_socket_addr,
                    server_service_binding,
                    &request,
                    &core_tracker_services.core_config,
                    &udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap()
            }

            #[tokio::test]
            async fn when_the_announce_request_comes_from_a_client_using_ipv4_the_response_should_not_include_peers_using_ipv6() {
                let (core_tracker_services, core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                add_a_torrent_peer_using_ipv6(&core_tracker_services.in_memory_torrent_repository).await;

                let response =
                    announce_a_new_peer_using_ipv4(Arc::new(core_tracker_services), Arc::new(core_udp_tracker_services)).await;

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
                let client_socket_addr = sample_ipv4_socket_address();
                let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();
                let announce_request = AnnounceRequestBuilder::default().into();

                let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
                udp_server_stats_event_sender_mock
                    .expect_send()
                    .with(eq(Event::UdpRequestAccepted {
                        context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                        kind: UdpRequestKind::Announce { announce_request },
                    }))
                    .times(1)
                    .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
                let udp_server_stats_event_sender: crate::event::sender::Sender =
                    Some(Arc::new(udp_server_stats_event_sender_mock));

                let (core_tracker_services, core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_default_tracker_configuration();

                handle_announce(
                    &core_udp_tracker_services.announce_service,
                    client_socket_addr,
                    server_service_binding,
                    &announce_request,
                    &core_tracker_services.core_config,
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
                use torrust_tracker_primitives::peer::fixture::PeerBuilder;
                use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

                use crate::handlers::announce::tests::announce_request::AnnounceRequestBuilder;
                use crate::handlers::handle_announce;
                use crate::handlers::tests::{
                    initialize_core_tracker_services_for_public_tracker, sample_cookie_valid_range, sample_issue_time,
                };

                #[tokio::test]
                async fn the_peer_ip_should_be_changed_to_the_external_ip_in_the_tracker_configuration_if_defined() {
                    let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                        initialize_core_tracker_services_for_public_tracker();

                    let client_ip = Ipv4Addr::LOCALHOST;
                    let client_port = 8080;
                    let info_hash = AquaticInfoHash([0u8; 20]);
                    let peer_id = AquaticPeerId([255u8; 20]);

                    let client_socket_addr = SocketAddr::new(IpAddr::V4(client_ip), client_port);
                    let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
                    let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                    let request = AnnounceRequestBuilder::default()
                        .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                        .with_info_hash(info_hash)
                        .with_peer_id(peer_id)
                        .with_ip_address(client_ip)
                        .with_port(client_port)
                        .into();

                    handle_announce(
                        &core_udp_tracker_services.announce_service,
                        client_socket_addr,
                        server_service_binding,
                        &request,
                        &core_tracker_services.core_config,
                        &server_udp_tracker_services.udp_server_stats_event_sender,
                        sample_cookie_valid_range(),
                    )
                    .await
                    .unwrap();

                    let peers = core_tracker_services
                        .in_memory_torrent_repository
                        .get_torrent_peers(&info_hash.0.into())
                        .await;

                    let external_ip_in_tracker_configuration = core_tracker_services.core_config.net.external_ip.unwrap();

                    let expected_peer = PeerBuilder::default()
                        .with_peer_id(&peer_id)
                        .with_peer_address(SocketAddr::new(external_ip_in_tracker_configuration, client_port))
                        .updated_on(peers[0].updated)
                        .into();

                    assert_eq!(peers[0], Arc::new(expected_peer));
                }
            }
        }

        mod using_ipv6 {

            use std::future;
            use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
            use std::sync::Arc;

            use aquatic_udp_protocol::{
                AnnounceInterval, AnnounceResponse, AnnounceResponseFixedData, InfoHash as AquaticInfoHash, Ipv4AddrBytes,
                Ipv6AddrBytes, NumberOfPeers, PeerId as AquaticPeerId, Response, ResponsePeer,
            };
            use bittorrent_tracker_core::announce_handler::AnnounceHandler;
            use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
            use bittorrent_tracker_core::whitelist;
            use bittorrent_udp_tracker_core::connection_cookie::{gen_remote_fingerprint, make};
            use bittorrent_udp_tracker_core::event::bus::EventBus;
            use bittorrent_udp_tracker_core::event::sender::Broadcaster;
            use bittorrent_udp_tracker_core::services::announce::AnnounceService;
            use mockall::predicate::eq;
            use torrust_tracker_configuration::Core;
            use torrust_tracker_events::bus::SenderStatus;
            use torrust_tracker_primitives::peer::fixture::PeerBuilder;
            use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

            use crate::event::{ConnectionContext, Event, UdpRequestKind};
            use crate::handlers::announce::tests::announce_request::AnnounceRequestBuilder;
            use crate::handlers::handle_announce;
            use crate::handlers::tests::{
                initialize_core_tracker_services_for_default_tracker_configuration,
                initialize_core_tracker_services_for_public_tracker, sample_cookie_valid_range, sample_ipv6_remote_addr,
                sample_issue_time, MockUdpServerStatsEventSender,
            };

            #[tokio::test]
            async fn an_announced_peer_should_be_added_to_the_tracker() {
                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                let client_ip_v4 = Ipv4Addr::new(126, 0, 0, 1);
                let client_ip_v6 = client_ip_v4.to_ipv6_compatible();
                let client_port = 8080;
                let info_hash = AquaticInfoHash([0u8; 20]);
                let peer_id = AquaticPeerId([255u8; 20]);

                let client_socket_addr = SocketAddr::new(IpAddr::V6(client_ip_v6), client_port);
                let server_socket_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                    .with_info_hash(info_hash)
                    .with_peer_id(peer_id)
                    .with_ip_address(client_ip_v4)
                    .with_port(client_port)
                    .into();

                handle_announce(
                    &core_udp_tracker_services.announce_service,
                    client_socket_addr,
                    server_service_binding,
                    &request,
                    &core_tracker_services.core_config,
                    &server_udp_tracker_services.udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();

                let peers = core_tracker_services
                    .in_memory_torrent_repository
                    .get_torrent_peers(&info_hash.0.into())
                    .await;

                let expected_peer = PeerBuilder::default()
                    .with_peer_id(&peer_id)
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

                let client_socket_addr = SocketAddr::new(IpAddr::V6(client_ip_v6), 8080);
                let server_socket_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                    .into();

                let response = handle_announce(
                    &core_udp_tracker_services.announce_service,
                    client_socket_addr,
                    server_service_binding,
                    &request,
                    &core_tracker_services.core_config,
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

                let client_socket_addr = SocketAddr::new(IpAddr::V6(remote_client_ip), remote_client_port);
                let server_socket_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                    .with_info_hash(info_hash)
                    .with_peer_id(peer_id)
                    .with_ip_address(peer_address)
                    .with_port(client_port)
                    .into();

                handle_announce(
                    &core_udp_tracker_services.announce_service,
                    client_socket_addr,
                    server_service_binding,
                    &request,
                    &core_tracker_services.core_config,
                    &server_udp_tracker_service.udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();

                let peers = core_tracker_services
                    .in_memory_torrent_repository
                    .get_torrent_peers(&info_hash.0.into())
                    .await;

                // When using IPv6 the tracker converts the remote client ip into a IPv4 address
                assert_eq!(peers[0].peer_addr, SocketAddr::new(IpAddr::V6(remote_client_ip), client_port));
            }

            async fn add_a_torrent_peer_using_ipv4(in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>) {
                let info_hash = AquaticInfoHash([0u8; 20]);

                let client_ip_v4 = Ipv4Addr::new(126, 0, 0, 1);
                let client_port = 8080;
                let peer_id = AquaticPeerId([255u8; 20]);

                let peer_using_ipv4 = PeerBuilder::default()
                    .with_peer_id(&peer_id)
                    .with_peer_address(SocketAddr::new(IpAddr::V4(client_ip_v4), client_port))
                    .into();

                in_memory_torrent_repository
                    .handle_announcement(&info_hash.0.into(), &peer_using_ipv4, None)
                    .await;
            }

            async fn announce_a_new_peer_using_ipv6(
                core_config: Arc<Core>,
                announce_handler: Arc<AnnounceHandler>,
                whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
            ) -> Response {
                let udp_core_broadcaster = Broadcaster::default();
                let core_event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, udp_core_broadcaster.clone()));
                let udp_core_stats_event_sender = core_event_bus.sender();

                let udp_server_broadcaster = crate::event::sender::Broadcaster::default();
                let server_event_bus = Arc::new(crate::event::bus::EventBus::new(
                    SenderStatus::Disabled,
                    udp_server_broadcaster.clone(),
                ));

                let udp_server_stats_event_sender = server_event_bus.sender();

                let client_ip_v4 = Ipv4Addr::new(126, 0, 0, 1);
                let client_ip_v6 = client_ip_v4.to_ipv6_compatible();
                let client_port = 8080;

                let client_socket_addr = SocketAddr::new(IpAddr::V6(client_ip_v6), client_port);
                let server_socket_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                    .into();

                let announce_service = Arc::new(AnnounceService::new(
                    announce_handler.clone(),
                    whitelist_authorization.clone(),
                    udp_core_stats_event_sender.clone(),
                ));

                handle_announce(
                    &announce_service,
                    client_socket_addr,
                    server_service_binding,
                    &request,
                    &core_config,
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

                add_a_torrent_peer_using_ipv4(&core_tracker_services.in_memory_torrent_repository).await;

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
                let client_socket_addr = sample_ipv6_remote_addr();
                let server_socket_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let announce_request = AnnounceRequestBuilder::default()
                    .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                    .into();

                let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
                udp_server_stats_event_sender_mock
                    .expect_send()
                    .with(eq(Event::UdpRequestAccepted {
                        context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                        kind: UdpRequestKind::Announce { announce_request },
                    }))
                    .times(1)
                    .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
                let udp_server_stats_event_sender: crate::event::sender::Sender =
                    Some(Arc::new(udp_server_stats_event_sender_mock));

                let (core_tracker_services, core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_default_tracker_configuration();

                handle_announce(
                    &core_udp_tracker_services.announce_service,
                    client_socket_addr,
                    server_service_binding,
                    &announce_request,
                    &core_tracker_services.core_config,
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
                use bittorrent_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
                use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
                use bittorrent_tracker_core::whitelist::authorization::WhitelistAuthorization;
                use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
                use bittorrent_udp_tracker_core::connection_cookie::{gen_remote_fingerprint, make};
                use bittorrent_udp_tracker_core::services::announce::AnnounceService;
                use bittorrent_udp_tracker_core::{self, event as core_event};
                use mockall::predicate::{self, eq};
                use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

                use crate::event::{ConnectionContext, Event, UdpRequestKind};
                use crate::handlers::announce::tests::announce_request::AnnounceRequestBuilder;
                use crate::handlers::handle_announce;
                use crate::handlers::tests::{
                    sample_cookie_valid_range, sample_issue_time, MockUdpCoreStatsEventSender, MockUdpServerStatsEventSender,
                    TrackerConfigurationBuilder,
                };
                use crate::tests::{announce_events_match, sample_peer};

                #[tokio::test]
                async fn the_peer_ip_should_be_changed_to_the_external_ip_in_the_tracker_configuration() {
                    let config = Arc::new(TrackerConfigurationBuilder::default().with_external_ip("::126.0.0.1").into());

                    let loopback_ipv4 = Ipv4Addr::LOCALHOST;
                    let loopback_ipv6 = Ipv6Addr::LOCALHOST;

                    let client_ip_v4 = loopback_ipv4;
                    let client_ip_v6 = loopback_ipv6;
                    let client_port = 8080;

                    let info_hash = AquaticInfoHash([0u8; 20]);
                    let peer_id = AquaticPeerId([255u8; 20]);
                    let mut announcement = sample_peer();
                    announcement.peer_id = peer_id;
                    announcement.peer_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0x7e00, 1)), client_port);

                    let client_socket_addr = SocketAddr::new(IpAddr::V6(client_ip_v6), client_port);
                    let mut server_socket_addr = config.udp_trackers.clone().unwrap()[0].bind_address;
                    if server_socket_addr.port() == 0 {
                        // Port 0 cannot be use in service binding
                        server_socket_addr.set_port(6969);
                    }
                    let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();
                    let server_service_binding_clone = server_service_binding.clone();

                    let database = initialize_database(&config.core);
                    let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
                    let whitelist_authorization =
                        Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
                    let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
                    let db_downloads_metric_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&database));

                    let request = AnnounceRequestBuilder::default()
                        .with_connection_id(make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap())
                        .with_info_hash(info_hash)
                        .with_peer_id(peer_id)
                        .with_ip_address(client_ip_v4)
                        .with_port(client_port)
                        .into();

                    let mut udp_core_stats_event_sender_mock = MockUdpCoreStatsEventSender::new();
                    udp_core_stats_event_sender_mock
                        .expect_send()
                        .with(predicate::function(move |event| {
                            let expected_event = core_event::Event::UdpAnnounce {
                                connection: core_event::ConnectionContext::new(
                                    client_socket_addr,
                                    server_service_binding.clone(),
                                ),
                                info_hash: info_hash.into(),
                                announcement,
                            };

                            announce_events_match(event, &expected_event)
                        }))
                        .times(1)
                        .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
                    let udp_core_stats_event_sender: bittorrent_udp_tracker_core::event::sender::Sender =
                        Some(Arc::new(udp_core_stats_event_sender_mock));

                    let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
                    udp_server_stats_event_sender_mock
                        .expect_send()
                        .with(eq(Event::UdpRequestAccepted {
                            context: ConnectionContext::new(client_socket_addr, server_service_binding_clone.clone()),
                            kind: UdpRequestKind::Announce {
                                announce_request: request,
                            },
                        }))
                        .times(1)
                        .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
                    let udp_server_stats_event_sender: crate::event::sender::Sender =
                        Some(Arc::new(udp_server_stats_event_sender_mock));

                    let announce_handler = Arc::new(AnnounceHandler::new(
                        &config.core,
                        &whitelist_authorization,
                        &in_memory_torrent_repository,
                        &db_downloads_metric_repository,
                    ));

                    let core_config = Arc::new(config.core.clone());

                    let announce_service = Arc::new(AnnounceService::new(
                        announce_handler.clone(),
                        whitelist_authorization.clone(),
                        udp_core_stats_event_sender.clone(),
                    ));

                    handle_announce(
                        &announce_service,
                        client_socket_addr,
                        server_service_binding_clone,
                        &request,
                        &core_config,
                        &udp_server_stats_event_sender,
                        sample_cookie_valid_range(),
                    )
                    .await
                    .unwrap();

                    let peers = in_memory_torrent_repository.get_torrent_peers(&info_hash.0.into()).await;

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
