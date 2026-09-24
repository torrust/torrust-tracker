//! UDP tracker scrape handler.
use std::net::SocketAddr;
use std::sync::Arc;

use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_primitives::ScrapeData;
use torrust_tracker_udp_core::connection_cookie::{check, gen_remote_fingerprint};
use torrust_tracker_udp_core::event::ConnectionContext;
use torrust_tracker_udp_core::services::scrape::ScrapeService;
use torrust_tracker_udp_core::{self, ConnectionIdValidationPolicy, UDP_TRACKER_LOG_TARGET};
use torrust_tracker_udp_protocol::{
    NumberOfDownloads, NumberOfPeers, Response, ScrapeRequest, ScrapeResponse, TorrentScrapeStatistics,
};
use tracing::{Level, instrument};
use zerocopy::byteorder::network_endian::I32;

use crate::event::{ErrorKind, Event, UdpRequestKind};
use crate::handlers::{CookieValidationContext, HandlerError};

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
    cookie_validation: CookieValidationContext,
) -> Result<Response, HandlerError> {
    tracing::Span::current()
        .record("transaction_id", request.transaction_id.0.to_string())
        .record("connection_id", request.connection_id.0.to_string());

    tracing::trace!("handle scrape");

    if let Some(udp_server_stats_event_sender) = opt_udp_server_stats_event_sender.as_deref() {
        udp_server_stats_event_sender
            .send(Event::UdpRequestAccepted {
                context: ConnectionContext::new(
                    scrape_service.configuration_instance_id(),
                    client_socket_addr,
                    server_service_binding.clone(),
                )
                .with_public_url(scrape_service.public_url().map(str::to_string)),
                kind: UdpRequestKind::Scrape,
            })
            .await;
    }

    let scrape_data = {
        let validate_cookie = match cookie_validation.connection_id_validation {
            ConnectionIdValidationPolicy::Strict => true,
            ConnectionIdValidationPolicy::Disabled => {
                if let Err(cookie_error) = check(
                    &request.connection_id,
                    gen_remote_fingerprint(&client_socket_addr),
                    cookie_validation.valid_range.clone(),
                ) {
                    tracing::debug!(
                        target: UDP_TRACKER_LOG_TARGET,
                        %client_socket_addr,
                        error = %cookie_error,
                        "connection ID validation disabled: invalid connection ID observed (request allowed, ban not enforced)"
                    );
                    if let Some(sender) = opt_udp_server_stats_event_sender.as_deref() {
                        sender
                            .send(Event::UdpError {
                                context: ConnectionContext::new(
                                    scrape_service.configuration_instance_id(),
                                    client_socket_addr,
                                    server_service_binding.clone(),
                                )
                                .with_public_url(scrape_service.public_url().map(str::to_string)),
                                kind: Some(UdpRequestKind::Scrape),
                                error: ErrorKind::ConnectionCookie(cookie_error.to_string()),
                            })
                            .await;
                    }
                }
                false
            }
        };

        scrape_service
            .handle_scrape(
                client_socket_addr,
                server_service_binding,
                request,
                cookie_validation.valid_range,
                validate_cookie,
            )
            .await
            .map_err(|e| Box::new((e.into(), request.transaction_id, UdpRequestKind::Scrape)))?
    };

    Ok(build_response(request, &scrape_data))
}

fn udp_counter_from_u32(value: u32) -> i32 {
    // Temporary saturation guard for UDP i32 counters. Proper type alignment across Rust and DB layers
    // will be addressed in docs/issues/1525-07-align-rust-and-db-types.md.
    i32::try_from(value).unwrap_or(i32::MAX)
}

fn build_response(request: &ScrapeRequest, scrape_data: &ScrapeData) -> Response {
    let mut torrent_stats = Vec::with_capacity(request.info_hashes.len());

    for info_hash in &request.info_hashes {
        let info_hash = info_hash.0.into();
        let swarm_metadata = scrape_data.files.get(&info_hash).copied().unwrap_or_default();

        let scrape_entry = TorrentScrapeStatistics {
            seeders: NumberOfPeers(I32::new(udp_counter_from_u32(swarm_metadata.complete))),
            completed: NumberOfDownloads(I32::new(udp_counter_from_u32(swarm_metadata.downloaded))),
            leechers: NumberOfPeers(I32::new(udp_counter_from_u32(swarm_metadata.incomplete))),
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

        use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
        use torrust_peer_id::PeerId;
        use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
        use torrust_tracker_primitives::ScrapeData;
        use torrust_tracker_primitives::peer::fixture::PeerBuilder;
        use torrust_tracker_udp_core::connection_cookie::{gen_remote_fingerprint, make};
        use torrust_tracker_udp_core::event::ConnectionContext;
        use torrust_tracker_udp_core::services::scrape::UdpScrapeError;
        use torrust_tracker_udp_protocol::{
            ConnectionId, InfoHash, NumberOfDownloads, NumberOfPeers, Response, ScrapeRequest, ScrapeResponse,
            TorrentScrapeStatistics, TransactionId,
        };
        use zerocopy::byteorder::network_endian::I64;

        use crate::error::Error;
        use crate::event::sender::Broadcaster;
        use crate::event::{Event, UdpRequestKind};
        use crate::handlers::handle_scrape;
        use crate::handlers::tests::{
            CoreTrackerServices, CoreUdpTrackerServices, initialize_core_tracker_services_for_listed_tracker,
            initialize_core_tracker_services_for_public_tracker, sample_ipv4_remote_addr, sample_issue_time,
            sample_strict_cookie_validation,
        };

        struct Tracker {
            core_tracker_services: CoreTrackerServices,
            core_udp_tracker_services: CoreUdpTrackerServices,
            client_socket_addr: SocketAddr,
            server_service_binding: ServiceBinding,
            udp_server_broadcaster: Broadcaster,
            udp_server_stats_event_sender: crate::event::sender::Sender,
        }

        impl Tracker {
            async fn public() -> Self {
                Self::public_for_client(sample_ipv4_remote_addr()).await
            }

            async fn public_for_client(client_socket_addr: SocketAddr) -> Self {
                let (core_tracker_services, core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_public_tracker().await;
                Self::with_services(core_tracker_services, core_udp_tracker_services, client_socket_addr)
            }

            async fn listed() -> Self {
                let (core_tracker_services, core_udp_tracker_services, _server_udp_tracker_services) =
                    initialize_core_tracker_services_for_listed_tracker().await;
                Self::with_services(core_tracker_services, core_udp_tracker_services, sample_ipv4_remote_addr())
            }

            fn with_services(
                core_tracker_services: CoreTrackerServices,
                core_udp_tracker_services: CoreUdpTrackerServices,
                client_socket_addr: SocketAddr,
            ) -> Self {
                let udp_server_broadcaster = Broadcaster::default();
                let server_ip = Ipv4Addr::new(203, 0, 113, 196);
                let server_socket_addr = match client_socket_addr {
                    SocketAddr::V4(_) => SocketAddr::new(IpAddr::V4(server_ip), 6969),
                    SocketAddr::V6(_) => SocketAddr::new(IpAddr::V6(server_ip.to_ipv6_compatible()), 6969),
                };
                let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

                Self {
                    core_tracker_services,
                    core_udp_tracker_services,
                    client_socket_addr,
                    server_service_binding,
                    udp_server_stats_event_sender: Some(Arc::new(udp_server_broadcaster.clone())),
                    udp_server_broadcaster,
                }
            }

            async fn with_seeder_for(self, info_hash: &InfoHash) -> Self {
                add_a_seeder(
                    self.core_tracker_services.in_memory_torrent_repository.clone(),
                    &self.client_socket_addr,
                    info_hash,
                )
                .await;
                self
            }

            async fn whitelisting(self, info_hash: &InfoHash) -> Self {
                self.core_tracker_services.in_memory_whitelist.add(&info_hash.0.into()).await;
                self
            }
        }

        struct ScrapeRequestBuilder {
            request: ScrapeRequest,
        }

        impl ScrapeRequestBuilder {
            fn for_client_and_info_hash(client_socket_addr: SocketAddr, info_hash: InfoHash) -> Self {
                Self {
                    request: ScrapeRequest {
                        connection_id: make(gen_remote_fingerprint(&client_socket_addr), sample_issue_time()).unwrap(),
                        transaction_id: TransactionId::new(0i32),
                        info_hashes: vec![info_hash],
                    },
                }
            }

            fn with_connection_id(mut self, connection_id: ConnectionId) -> Self {
                self.request.connection_id = connection_id;
                self
            }

            fn with_transaction_id(mut self, transaction_id: TransactionId) -> Self {
                self.request.transaction_id = transaction_id;
                self
            }

            fn with_info_hashes(mut self, info_hashes: Vec<InfoHash>) -> Self {
                self.request.info_hashes = info_hashes;
                self
            }

            fn into(self) -> ScrapeRequest {
                self.request
            }
        }

        /// Calls `handle_scrape` with the tracker's ordinary strict-validation context.
        async fn scrape(tracker: &Tracker, request: &ScrapeRequest) -> Result<Response, crate::handlers::HandlerError> {
            handle_scrape(
                &tracker.core_udp_tracker_services.scrape_service,
                tracker.client_socket_addr,
                tracker.server_service_binding.clone(),
                request,
                &tracker.udp_server_stats_event_sender,
                sample_strict_cookie_validation(),
            )
            .await
        }

        fn zeroed_torrent_statistics() -> TorrentScrapeStatistics {
            TorrentScrapeStatistics {
                seeders: NumberOfPeers(0.into()),
                completed: NumberOfDownloads(0.into()),
                leechers: NumberOfPeers(0.into()),
            }
        }

        #[tokio::test]
        async fn it_should_return_zeroed_statistics_when_the_tracker_does_not_have_the_requested_torrent() {
            // Arrange
            let tracker = Tracker::public().await;
            let info_hash = InfoHash([0u8; 20]);
            let request = ScrapeRequestBuilder::for_client_and_info_hash(tracker.client_socket_addr, info_hash).into();
            let expected_response = Response::from(ScrapeResponse {
                transaction_id: request.transaction_id,
                torrent_stats: vec![zeroed_torrent_statistics()],
            });

            // Act
            let response = scrape(&tracker, &request).await.unwrap();

            // Assert
            assert_eq!(response, expected_response);
        }

        #[tokio::test]
        async fn it_should_preserve_a_scrape_service_failure_for_packet_error_routing() {
            // Arrange
            let (_core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                initialize_core_tracker_services_for_public_tracker().await;
            let client_socket_addr = sample_ipv4_remote_addr();
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();
            let request = ScrapeRequestBuilder::for_client_and_info_hash(client_socket_addr, InfoHash([0u8; 20]))
                .with_connection_id(ConnectionId(I64::new(0)))
                .with_transaction_id(TransactionId::new(42))
                .into();

            // Act
            let actual = handle_scrape(
                &core_udp_tracker_services.scrape_service,
                client_socket_addr,
                server_service_binding,
                &request,
                &server_udp_tracker_services.udp_server_stats_event_sender,
                sample_strict_cookie_validation(),
            )
            .await;

            // Assert
            assert!(matches!(
                actual,
                Err(boxed_error) if matches!(
                    *boxed_error,
                    (
                        Error::ScrapeFailed {
                            source: UdpScrapeError::ConnectionCookieError { .. },
                        },
                        transaction_id,
                        UdpRequestKind::Scrape,
                    ) if transaction_id == request.transaction_id
                )
            ));
        }

        async fn add_a_seeder(
            in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
            remote_addr: &SocketAddr,
            info_hash: &InfoHash,
        ) {
            let peer_id = PeerId([255u8; 20]);

            let peer = PeerBuilder::default()
                .with_peer_id(&torrust_tracker_primitives::PeerId(peer_id.0))
                .with_peer_address(*remote_addr)
                .with_bytes_left_to_download(0)
                .into();

            in_memory_torrent_repository
                .handle_announcement(&info_hash.0.into(), &peer, None)
                .await;
        }

        async fn add_seeders(
            in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
            info_hash: &InfoHash,
            number_of_seeders: u8,
        ) {
            for peer_index in 1..=number_of_seeders {
                let peer_id = PeerId([peer_index; 20]);
                let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, peer_index)), 6880 + u16::from(peer_index));
                let peer = PeerBuilder::default()
                    .with_peer_id(&torrust_tracker_primitives::PeerId(peer_id.0))
                    .with_peer_address(remote_addr)
                    .with_bytes_left_to_download(0)
                    .into();

                in_memory_torrent_repository
                    .handle_announcement(&info_hash.0.into(), &peer, None)
                    .await;
            }
        }

        fn scrape_response(response: Response) -> ScrapeResponse {
            match response {
                Response::Scrape(scrape_response) => scrape_response,
                _ => panic!("the scrape handler should return a scrape response"),
            }
        }

        #[test]
        fn it_should_return_zeroed_statistics_when_scrape_data_does_not_contain_a_requested_hash() {
            // Arrange
            let client_socket_addr = sample_ipv4_remote_addr();
            let info_hash = InfoHash([0u8; 20]);
            let request = ScrapeRequestBuilder::for_client_and_info_hash(client_socket_addr, info_hash).into();
            let expected_response = Response::from(ScrapeResponse {
                transaction_id: request.transaction_id,
                torrent_stats: vec![zeroed_torrent_statistics()],
            });

            // Act
            let response = super::super::build_response(&request, &ScrapeData::empty());

            // Assert
            assert_eq!(response, expected_response);
        }

        #[tokio::test]
        async fn it_should_return_an_entry_for_each_duplicate_requested_info_hash() {
            // Arrange
            let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                initialize_core_tracker_services_for_public_tracker().await;
            let client_socket_addr = sample_ipv4_remote_addr();
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();
            let info_hash = InfoHash([0u8; 20]);
            let request = ScrapeRequestBuilder::for_client_and_info_hash(client_socket_addr, info_hash)
                .with_info_hashes(vec![info_hash, info_hash])
                .into();
            let expected_torrent_stats = vec![
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(1.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                },
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(1.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                },
            ];

            add_a_seeder(
                core_tracker_services.in_memory_torrent_repository.clone(),
                &client_socket_addr,
                &info_hash,
            )
            .await;

            // Act
            let response = handle_scrape(
                &core_udp_tracker_services.scrape_service,
                client_socket_addr,
                server_service_binding,
                &request,
                &server_udp_tracker_services.udp_server_stats_event_sender,
                sample_strict_cookie_validation(),
            )
            .await
            .unwrap();

            // Assert
            assert_eq!(
                response,
                Response::from(ScrapeResponse {
                    transaction_id: request.transaction_id,
                    torrent_stats: expected_torrent_stats,
                })
            );
        }

        #[tokio::test]
        async fn it_should_preserve_the_order_of_eight_requested_info_hashes() {
            // Arrange
            let (core_tracker_services, core_udp_tracker_services, server_udp_tracker_services) =
                initialize_core_tracker_services_for_public_tracker().await;
            let client_socket_addr = sample_ipv4_remote_addr();
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();
            let requested_info_hashes = vec![
                InfoHash([8u8; 20]),
                InfoHash([3u8; 20]),
                InfoHash([6u8; 20]),
                InfoHash([1u8; 20]),
                InfoHash([7u8; 20]),
                InfoHash([2u8; 20]),
                InfoHash([5u8; 20]),
                InfoHash([4u8; 20]),
            ];
            let expected_torrent_stats = vec![
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(8.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                },
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(3.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                },
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(6.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                },
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(1.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                },
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(7.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                },
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(2.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                },
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(5.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                },
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(4.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                },
            ];
            let request = ScrapeRequestBuilder::for_client_and_info_hash(client_socket_addr, requested_info_hashes[0])
                .with_info_hashes(requested_info_hashes)
                .into();

            for (info_hash, number_of_seeders) in request.info_hashes.iter().zip([8u8, 3, 6, 1, 7, 2, 5, 4]) {
                add_seeders(
                    core_tracker_services.in_memory_torrent_repository.clone(),
                    info_hash,
                    number_of_seeders,
                )
                .await;
            }

            // Act
            let response = handle_scrape(
                &core_udp_tracker_services.scrape_service,
                client_socket_addr,
                server_service_binding,
                &request,
                &server_udp_tracker_services.udp_server_stats_event_sender,
                sample_strict_cookie_validation(),
            )
            .await
            .unwrap();

            // Assert
            assert_eq!(
                response,
                Response::from(ScrapeResponse {
                    transaction_id: request.transaction_id,
                    torrent_stats: expected_torrent_stats,
                })
            );
        }

        mod with_a_public_tracker {
            use torrust_tracker_udp_protocol::{NumberOfDownloads, NumberOfPeers, TorrentScrapeStatistics};

            use super::{ScrapeRequestBuilder, Tracker, scrape};
            use crate::handlers::scrape::tests::scrape_request::InfoHash;

            #[tokio::test]
            async fn it_should_return_statistics_when_the_public_tracker_has_the_requested_torrent() {
                // Arrange
                let info_hash = InfoHash([0u8; 20]);
                let tracker = Tracker::public().await.with_seeder_for(&info_hash).await;
                let request = ScrapeRequestBuilder::for_client_and_info_hash(tracker.client_socket_addr, info_hash).into();
                let expected_torrent_stats = vec![TorrentScrapeStatistics {
                    seeders: NumberOfPeers(1.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                }];

                // Act
                let response = scrape(&tracker, &request).await.unwrap();

                // Assert
                let actual_torrent_stats = match response {
                    torrust_tracker_udp_protocol::Response::Scrape(response) => response.torrent_stats,
                    _ => panic!("the scrape handler should return a scrape response"),
                };
                assert_eq!(actual_torrent_stats, expected_torrent_stats);
            }
        }

        mod with_a_whitelisted_tracker {
            use torrust_tracker_udp_protocol::{InfoHash, NumberOfDownloads, NumberOfPeers, TorrentScrapeStatistics};

            use super::{ScrapeRequestBuilder, Tracker, scrape, scrape_response, zeroed_torrent_statistics};

            #[tokio::test]
            async fn it_should_return_statistics_when_the_listed_tracker_has_a_whitelisted_torrent() {
                // Arrange
                let info_hash = InfoHash([0u8; 20]);
                let tracker = Tracker::listed()
                    .await
                    .with_seeder_for(&info_hash)
                    .await
                    .whitelisting(&info_hash)
                    .await;
                let request = ScrapeRequestBuilder::for_client_and_info_hash(tracker.client_socket_addr, info_hash).into();
                let expected_torrent_stats = vec![TorrentScrapeStatistics {
                    seeders: NumberOfPeers(1.into()),
                    completed: NumberOfDownloads(0.into()),
                    leechers: NumberOfPeers(0.into()),
                }];

                // Act
                let response = scrape(&tracker, &request).await.unwrap();

                // Assert
                assert_eq!(scrape_response(response).torrent_stats, expected_torrent_stats);
            }

            #[tokio::test]
            async fn it_should_return_zeroed_statistics_when_the_listed_tracker_has_not_whitelisted_the_torrent() {
                // Arrange
                let info_hash = InfoHash([0u8; 20]);
                let tracker = Tracker::listed().await.with_seeder_for(&info_hash).await;
                let request = ScrapeRequestBuilder::for_client_and_info_hash(tracker.client_socket_addr, info_hash).into();
                let expected_torrent_stats = vec![zeroed_torrent_statistics()];

                // Act
                let response = scrape(&tracker, &request).await.unwrap();

                // Assert
                assert_eq!(scrape_response(response).torrent_stats, expected_torrent_stats);
            }
        }

        #[tokio::test]
        async fn it_should_publish_an_accepted_scrape_event_for_an_ipv4_client() {
            // Arrange
            let tracker = Tracker::public().await;
            let request = ScrapeRequestBuilder::for_client_and_info_hash(tracker.client_socket_addr, InfoHash([0u8; 20])).into();
            let expected_event = Event::UdpRequestAccepted {
                context: ConnectionContext::new(
                    tracker.core_udp_tracker_services.scrape_service.configuration_instance_id(),
                    tracker.client_socket_addr,
                    tracker.server_service_binding.clone(),
                ),
                kind: UdpRequestKind::Scrape,
            };
            let mut receiver = tracker.udp_server_broadcaster.subscribe();

            // Act
            scrape(&tracker, &request).await.unwrap();

            // Assert
            let event = receiver.recv().await.expect("accepted scrape event should be published");
            assert_eq!(event, expected_event);
        }

        #[tokio::test]
        async fn it_should_publish_an_accepted_scrape_event_for_an_ipv6_client() {
            // Arrange
            let tracker = Tracker::public_for_client(crate::handlers::tests::sample_ipv6_remote_addr()).await;
            let request = ScrapeRequestBuilder::for_client_and_info_hash(tracker.client_socket_addr, InfoHash([0u8; 20])).into();
            let expected_event = Event::UdpRequestAccepted {
                context: ConnectionContext::new(
                    tracker.core_udp_tracker_services.scrape_service.configuration_instance_id(),
                    tracker.client_socket_addr,
                    tracker.server_service_binding.clone(),
                ),
                kind: UdpRequestKind::Scrape,
            };
            let mut receiver = tracker.udp_server_broadcaster.subscribe();

            // Act
            scrape(&tracker, &request).await.unwrap();

            // Assert
            let event = receiver.recv().await.expect("accepted scrape event should be published");
            assert_eq!(event, expected_event);
        }
    }

    #[test]
    fn should_saturate_large_download_counts_for_udp_protocol() {
        assert_eq!(super::udp_counter_from_u32(u32::MAX), i32::MAX);
        assert_eq!(super::udp_counter_from_u32((i32::MAX as u32) + 1), i32::MAX);
        assert_eq!(super::udp_counter_from_u32(42), 42);
    }
}
