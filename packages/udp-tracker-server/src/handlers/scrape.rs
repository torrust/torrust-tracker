//! UDP tracker scrape handler.
use std::net::SocketAddr;
use std::ops::Range;
use std::sync::Arc;

use aquatic_udp_protocol::{
    NumberOfDownloads, NumberOfPeers, Response, ScrapeRequest, ScrapeResponse, TorrentScrapeStatistics, TransactionId,
};
use bittorrent_udp_tracker_core::services::scrape::ScrapeService;
use bittorrent_udp_tracker_core::{self};
use torrust_tracker_primitives::core::ScrapeData;
use torrust_tracker_primitives::service_binding::ServiceBinding;
use tracing::{instrument, Level};
use zerocopy::network_endian::I32;

use crate::error::Error;
use crate::event::{ConnectionContext, Event, UdpRequestKind};

/// It handles the `Scrape` request.
///
/// # Errors
///
/// This function does not ever return an error.
#[instrument(fields(transaction_id, connection_id), skip(scrape_service, opt_udp_server_stats_event_sender),  ret(level = Level::TRACE))]
pub async fn handle_scrape(
    scrape_service: &Arc<ScrapeService>,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    request: &ScrapeRequest,
    opt_udp_server_stats_event_sender: &crate::event::sender::Sender,
    cookie_valid_range: Range<f64>,
) -> Result<Response, (Error, TransactionId, UdpRequestKind)> {
    tracing::Span::current()
        .record("transaction_id", request.transaction_id.0.to_string())
        .record("connection_id", request.connection_id.0.to_string());

    tracing::trace!("handle scrape");

    if let Some(udp_server_stats_event_sender) = opt_udp_server_stats_event_sender.as_deref() {
        udp_server_stats_event_sender
            .send(Event::UdpRequestAccepted {
                context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                kind: UdpRequestKind::Scrape,
            })
            .await;
    }

    let scrape_data = scrape_service
        .handle_scrape(client_socket_addr, server_service_binding, request, cookie_valid_range)
        .await
        .map_err(|e| (e.into(), request.transaction_id, UdpRequestKind::Scrape))?;

    Ok(build_response(request, &scrape_data))
}

fn build_response(request: &ScrapeRequest, scrape_data: &ScrapeData) -> Response {
    let mut torrent_stats: Vec<TorrentScrapeStatistics> = Vec::new();

    for file in &scrape_data.files {
        let swarm_metadata = file.1;

        #[allow(clippy::cast_possible_truncation)]
        let scrape_entry = {
            TorrentScrapeStatistics {
                seeders: NumberOfPeers(I32::new(i64::from(swarm_metadata.complete) as i32)),
                completed: NumberOfDownloads(I32::new(i64::from(swarm_metadata.downloaded) as i32)),
                leechers: NumberOfPeers(I32::new(i64::from(swarm_metadata.incomplete) as i32)),
            }
        };

        torrent_stats.push(scrape_entry);
    }

    let response = ScrapeResponse {
        transaction_id: request.transaction_id,
        torrent_stats,
    };

    Response::from(response)
}

#[cfg(test)]
mod tests {

    mod scrape_request {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::sync::Arc;

        use aquatic_udp_protocol::{
            InfoHash, NumberOfDownloads, NumberOfPeers, PeerId, Response, ScrapeRequest, ScrapeResponse, TorrentScrapeStatistics,
            TransactionId,
        };
        use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
        use bittorrent_udp_tracker_core::connection_cookie::{gen_remote_fingerprint, make};
        use torrust_tracker_events::bus::SenderStatus;
        use torrust_tracker_primitives::peer::fixture::PeerBuilder;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use crate::event::bus::EventBus;
        use crate::event::sender::Broadcaster;
        use crate::handlers::handle_scrape;
        use crate::handlers::tests::{
            initialize_core_tracker_services_for_public_tracker, sample_cookie_valid_range, sample_ipv4_remote_addr,
            sample_issue_time, CoreTrackerServices, CoreUdpTrackerServices,
        };

        fn zeroed_torrent_statistics() -> TorrentScrapeStatistics {
            TorrentScrapeStatistics {
                seeders: NumberOfPeers(0.into()),
                completed: NumberOfDownloads(0.into()),
                leechers: NumberOfPeers(0.into()),
            }
        }

        #[tokio::test]
        async fn should_return_no_stats_when_the_tracker_does_not_have_any_torrent() {
            let (_core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                initialize_core_tracker_services_for_public_tracker();

            let client_socket_addr = sample_ipv4_remote_addr();
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let info_hash = InfoHash([0u8; 20]);
            let info_hashes = vec![info_hash];

            let request = ScrapeRequest {
                connection_id: make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap(),
                transaction_id: TransactionId(0i32.into()),
                info_hashes,
            };

            let response = handle_scrape(
                &core_udp_tracker_services.scrape_service,
                client_socket_addr,
                server_service_binding,
                &request,
                &server_udp_tracker_services.udp_server_stats_event_sender,
                sample_cookie_valid_range(),
            )
            .await
            .unwrap();

            let expected_torrent_stats = vec![zeroed_torrent_statistics()];

            assert_eq!(
                response,
                Response::from(ScrapeResponse {
                    transaction_id: request.transaction_id,
                    torrent_stats: expected_torrent_stats
                })
            );
        }

        async fn add_a_seeder(
            in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
            remote_addr: &SocketAddr,
            info_hash: &InfoHash,
        ) {
            let peer_id = PeerId([255u8; 20]);

            let peer = PeerBuilder::default()
                .with_peer_id(&peer_id)
                .with_peer_address(*remote_addr)
                .with_bytes_left_to_download(0)
                .into();

            in_memory_torrent_repository
                .handle_announcement(&info_hash.0.into(), &peer, None)
                .await;
        }

        fn build_scrape_request(remote_addr: &SocketAddr, info_hash: &InfoHash) -> ScrapeRequest {
            let info_hashes = vec![*info_hash];

            ScrapeRequest {
                connection_id: make(gen_remote_fingerprint(remote_addr), sample_issue_time()).unwrap(),
                transaction_id: TransactionId::new(0i32),
                info_hashes,
            }
        }

        async fn add_a_sample_seeder_and_scrape(
            core_tracker_services: Arc<CoreTrackerServices>,
            core_udp_tracker_services: Arc<CoreUdpTrackerServices>,
        ) -> Response {
            let udp_server_broadcaster = Broadcaster::default();
            let event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, udp_server_broadcaster.clone()));

            let udp_server_stats_event_sender = event_bus.sender();

            let client_socket_addr = sample_ipv4_remote_addr();
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let info_hash = InfoHash([0u8; 20]);

            add_a_seeder(
                core_tracker_services.in_memory_torrent_repository.clone(),
                &client_socket_addr,
                &info_hash,
            )
            .await;

            let request = build_scrape_request(&client_socket_addr, &info_hash);

            handle_scrape(
                &core_udp_tracker_services.scrape_service,
                client_socket_addr,
                server_service_binding,
                &request,
                &udp_server_stats_event_sender,
                sample_cookie_valid_range(),
            )
            .await
            .unwrap()
        }

        fn match_scrape_response(response: Response) -> Option<ScrapeResponse> {
            match response {
                Response::Scrape(scrape_response) => Some(scrape_response),
                _ => None,
            }
        }

        mod with_a_public_tracker {
            use aquatic_udp_protocol::{NumberOfDownloads, NumberOfPeers, TorrentScrapeStatistics};

            use crate::handlers::scrape::tests::scrape_request::{add_a_sample_seeder_and_scrape, match_scrape_response};
            use crate::handlers::tests::initialize_core_tracker_services_for_public_tracker;

            #[tokio::test]
            async fn should_return_torrent_statistics_when_the_tracker_has_the_requested_torrent() {
                let (core_tracker_services, core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker();

                let torrent_stats = match_scrape_response(
                    add_a_sample_seeder_and_scrape(core_tracker_services.into(), core_udp_tracker_services.into()).await,
                );

                let expected_torrent_stats = vec![TorrentScrapeStatistics {
                    seeders: NumberOfPeers(1.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                }];

                assert_eq!(torrent_stats.unwrap().torrent_stats, expected_torrent_stats);
            }
        }

        mod with_a_whitelisted_tracker {
            use std::net::{IpAddr, Ipv4Addr, SocketAddr};

            use aquatic_udp_protocol::{InfoHash, NumberOfDownloads, NumberOfPeers, TorrentScrapeStatistics};
            use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

            use crate::handlers::handle_scrape;
            use crate::handlers::scrape::tests::scrape_request::{
                add_a_seeder, build_scrape_request, match_scrape_response, zeroed_torrent_statistics,
            };
            use crate::handlers::tests::{
                initialize_core_tracker_services_for_listed_tracker, sample_cookie_valid_range, sample_ipv4_remote_addr,
            };

            #[tokio::test]
            async fn should_return_the_torrent_statistics_when_the_requested_torrent_is_whitelisted() {
                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                    initialize_core_tracker_services_for_listed_tracker();

                let client_socket_addr = sample_ipv4_remote_addr();
                let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let info_hash = InfoHash([0u8; 20]);

                add_a_seeder(
                    core_tracker_services.in_memory_torrent_repository.clone(),
                    &client_socket_addr,
                    &info_hash,
                )
                .await;

                core_tracker_services.in_memory_whitelist.add(&info_hash.0.into()).await;

                let request = build_scrape_request(&client_socket_addr, &info_hash);

                let torrent_stats = match_scrape_response(
                    handle_scrape(
                        &core_udp_tracker_services.scrape_service,
                        client_socket_addr,
                        server_service_binding,
                        &request,
                        &server_udp_tracker_services.udp_server_stats_event_sender,
                        sample_cookie_valid_range(),
                    )
                    .await
                    .unwrap(),
                )
                .unwrap();

                let expected_torrent_stats = vec![TorrentScrapeStatistics {
                    seeders: NumberOfPeers(1.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                }];

                assert_eq!(torrent_stats.torrent_stats, expected_torrent_stats);
            }

            #[tokio::test]
            async fn should_return_zeroed_statistics_when_the_requested_torrent_is_not_whitelisted() {
                let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                    initialize_core_tracker_services_for_listed_tracker();

                let client_socket_addr = sample_ipv4_remote_addr();
                let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let info_hash = InfoHash([0u8; 20]);

                add_a_seeder(
                    core_tracker_services.in_memory_torrent_repository.clone(),
                    &client_socket_addr,
                    &info_hash,
                )
                .await;

                let request = build_scrape_request(&client_socket_addr, &info_hash);

                let torrent_stats = match_scrape_response(
                    handle_scrape(
                        &core_udp_tracker_services.scrape_service,
                        client_socket_addr,
                        server_service_binding,
                        &request,
                        &server_udp_tracker_services.udp_server_stats_event_sender,
                        sample_cookie_valid_range(),
                    )
                    .await
                    .unwrap(),
                )
                .unwrap();

                let expected_torrent_stats = vec![zeroed_torrent_statistics()];

                assert_eq!(torrent_stats.torrent_stats, expected_torrent_stats);
            }
        }

        fn sample_scrape_request(remote_addr: &SocketAddr) -> ScrapeRequest {
            let info_hash = InfoHash([0u8; 20]);
            let info_hashes = vec![info_hash];

            ScrapeRequest {
                connection_id: make(gen_remote_fingerprint(remote_addr), sample_issue_time()).unwrap(),
                transaction_id: TransactionId(0i32.into()),
                info_hashes,
            }
        }

        mod using_ipv4 {
            use std::future;
            use std::net::{IpAddr, Ipv6Addr, SocketAddr};
            use std::sync::Arc;

            use mockall::predicate::eq;
            use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

            use super::sample_scrape_request;
            use crate::event::{ConnectionContext, Event, UdpRequestKind};
            use crate::handlers::handle_scrape;
            use crate::handlers::tests::{
                initialize_core_tracker_services_for_default_tracker_configuration, sample_cookie_valid_range,
                sample_ipv4_remote_addr, MockUdpServerStatsEventSender,
            };

            #[tokio::test]
            async fn should_send_the_upd4_scrape_event() {
                let client_socket_addr = sample_ipv4_remote_addr();
                let server_socket_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
                udp_server_stats_event_sender_mock
                    .expect_send()
                    .with(eq(Event::UdpRequestAccepted {
                        context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                        kind: UdpRequestKind::Scrape,
                    }))
                    .times(1)
                    .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
                let udp_server_stats_event_sender: crate::event::sender::Sender =
                    Some(Arc::new(udp_server_stats_event_sender_mock));

                let (_core_tracker_services, core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_default_tracker_configuration();

                handle_scrape(
                    &core_udp_tracker_services.scrape_service,
                    client_socket_addr,
                    server_service_binding,
                    &sample_scrape_request(&client_socket_addr),
                    &udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();
            }
        }

        mod using_ipv6 {
            use std::future;
            use std::net::{IpAddr, Ipv6Addr, SocketAddr};
            use std::sync::Arc;

            use mockall::predicate::eq;
            use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

            use super::sample_scrape_request;
            use crate::event::{ConnectionContext, Event, UdpRequestKind};
            use crate::handlers::handle_scrape;
            use crate::handlers::tests::{
                initialize_core_tracker_services_for_default_tracker_configuration, sample_cookie_valid_range,
                sample_ipv6_remote_addr, MockUdpServerStatsEventSender,
            };

            #[tokio::test]
            async fn should_send_the_upd6_scrape_event() {
                let client_socket_addr = sample_ipv6_remote_addr();
                let server_socket_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969);
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
                udp_server_stats_event_sender_mock
                    .expect_send()
                    .with(eq(Event::UdpRequestAccepted {
                        context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                        kind: UdpRequestKind::Scrape,
                    }))
                    .times(1)
                    .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
                let udp_server_stats_event_sender: crate::event::sender::Sender =
                    Some(Arc::new(udp_server_stats_event_sender_mock));

                let (_core_tracker_services, core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_default_tracker_configuration();

                handle_scrape(
                    &core_udp_tracker_services.scrape_service,
                    client_socket_addr,
                    server_service_binding,
                    &sample_scrape_request(&client_socket_addr),
                    &udp_server_stats_event_sender,
                    sample_cookie_valid_range(),
                )
                .await
                .unwrap();
            }
        }
    }
}
