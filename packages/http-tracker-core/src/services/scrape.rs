//! The `scrape` service.
//!
//! The service is responsible for handling the `scrape` requests.
//!
//! It delegates the `scrape` logic to the [`ScrapeHandler`] and it returns the
//! [`ScrapeData`].
//!
//! It also sends an [`http_tracker_core::statistics::event::Event`]
//! because events are specific for the HTTP tracker.
use std::sync::Arc;

use bittorrent_http_tracker_protocol::v1::requests::scrape::Scrape;
use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::{
    resolve_remote_client_addr, ClientIpSources, PeerIpResolutionError, RemoteClientAddr,
};
use bittorrent_tracker_core::authentication::service::AuthenticationService;
use bittorrent_tracker_core::authentication::{self, Key};
use bittorrent_tracker_core::error::{ScrapeError, TrackerCoreError, WhitelistError};
use bittorrent_tracker_core::scrape_handler::ScrapeHandler;
use torrust_tracker_configuration::Core;
use torrust_tracker_primitives::core::ScrapeData;
use torrust_tracker_primitives::service_binding::ServiceBinding;

use crate::event::{ConnectionContext, Event};

/// The HTTP tracker `scrape` service.
///
/// The service sends an statistics event that increments:
///
/// - The number of TCP `announce` requests handled by the HTTP tracker.
/// - The number of TCP `scrape` requests handled by the HTTP tracker.
///
/// # Errors
///
/// This function will return an error if:
///
/// - There is an error when resolving the client IP address.
pub struct ScrapeService {
    core_config: Arc<Core>,
    scrape_handler: Arc<ScrapeHandler>,
    authentication_service: Arc<AuthenticationService>,
    opt_http_stats_event_sender: crate::event::sender::Sender,
}

impl ScrapeService {
    #[must_use]
    pub fn new(
        core_config: Arc<Core>,
        scrape_handler: Arc<ScrapeHandler>,
        authentication_service: Arc<AuthenticationService>,
        opt_http_stats_event_sender: crate::event::sender::Sender,
    ) -> Self {
        Self {
            core_config,
            scrape_handler,
            authentication_service,
            opt_http_stats_event_sender,
        }
    }

    /// Handles a scrape request.
    ///
    /// When the peer is not authenticated and the tracker is running in `private`
    /// mode, the tracker returns empty stats for all the torrents.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// - There is an error when resolving the client IP address.
    pub async fn handle_scrape(
        &self,
        scrape_request: &Scrape,
        client_ip_sources: &ClientIpSources,
        server_service_binding: &ServiceBinding,
        maybe_key: Option<Key>,
    ) -> Result<ScrapeData, HttpScrapeError> {
        let scrape_data = if self.authentication_is_required() && !self.is_authenticated(maybe_key).await {
            ScrapeData::zeroed(&scrape_request.info_hashes)
        } else {
            self.scrape_handler.handle_scrape(&scrape_request.info_hashes).await?
        };

        let remote_client_addr = resolve_remote_client_addr(&self.core_config.net.on_reverse_proxy.into(), client_ip_sources)?;

        self.send_event(remote_client_addr, server_service_binding.clone()).await;

        Ok(scrape_data)
    }

    fn authentication_is_required(&self) -> bool {
        self.core_config.private
    }

    async fn is_authenticated(&self, maybe_key: Option<Key>) -> bool {
        if let Some(key) = maybe_key {
            return self.authentication_service.authenticate(&key).await.is_ok();
        }

        false
    }

    async fn send_event(&self, remote_client_addr: RemoteClientAddr, server_service_binding: ServiceBinding) {
        if let Some(http_stats_event_sender) = self.opt_http_stats_event_sender.as_deref() {
            let event = Event::TcpScrape {
                connection: ConnectionContext::new(remote_client_addr, server_service_binding),
            };

            tracing::debug!("Sending TcpScrape event: {:?}", event);

            http_stats_event_sender.send(event).await;
        }
    }
}

/// Errors related to announce requests.
#[derive(thiserror::Error, Debug, Clone)]
pub enum HttpScrapeError {
    #[error("Error resolving peer IP: {source}")]
    PeerIpResolutionError { source: PeerIpResolutionError },

    #[error("Tracker core error: {source}")]
    TrackerCoreError { source: TrackerCoreError },
}

impl From<PeerIpResolutionError> for HttpScrapeError {
    fn from(peer_ip_resolution_error: PeerIpResolutionError) -> Self {
        Self::PeerIpResolutionError {
            source: peer_ip_resolution_error,
        }
    }
}

impl From<TrackerCoreError> for HttpScrapeError {
    fn from(tracker_core_error: TrackerCoreError) -> Self {
        Self::TrackerCoreError {
            source: tracker_core_error,
        }
    }
}

impl From<ScrapeError> for HttpScrapeError {
    fn from(announce_error: ScrapeError) -> Self {
        Self::TrackerCoreError {
            source: announce_error.into(),
        }
    }
}

impl From<WhitelistError> for HttpScrapeError {
    fn from(whitelist_error: WhitelistError) -> Self {
        Self::TrackerCoreError {
            source: whitelist_error.into(),
        }
    }
}

impl From<authentication::key::Error> for HttpScrapeError {
    fn from(whitelist_error: authentication::key::Error) -> Self {
        Self::TrackerCoreError {
            source: whitelist_error.into(),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
    use bittorrent_primitives::info_hash::InfoHash;
    use bittorrent_tracker_core::announce_handler::AnnounceHandler;
    use bittorrent_tracker_core::authentication::key::repository::in_memory::InMemoryKeyRepository;
    use bittorrent_tracker_core::authentication::service::AuthenticationService;
    use bittorrent_tracker_core::databases::setup::initialize_database;
    use bittorrent_tracker_core::scrape_handler::ScrapeHandler;
    use bittorrent_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use bittorrent_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use futures::future::BoxFuture;
    use mockall::mock;
    use torrust_tracker_configuration::Configuration;
    use torrust_tracker_events::sender::SendError;
    use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};

    use crate::event::Event;
    use crate::tests::sample_info_hash;

    struct Container {
        announce_handler: Arc<AnnounceHandler>,
        scrape_handler: Arc<ScrapeHandler>,
        authentication_service: Arc<AuthenticationService>,
    }

    fn initialize_services_with_configuration(config: &Configuration) -> Container {
        let database = initialize_database(&config.core);
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let db_downloads_metric_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&database));
        let in_memory_key_repository = Arc::new(InMemoryKeyRepository::default());
        let authentication_service = Arc::new(AuthenticationService::new(&config.core, &in_memory_key_repository));

        let announce_handler = Arc::new(AnnounceHandler::new(
            &config.core,
            &whitelist_authorization,
            &in_memory_torrent_repository,
            &db_downloads_metric_repository,
        ));

        let scrape_handler = Arc::new(ScrapeHandler::new(&whitelist_authorization, &in_memory_torrent_repository));

        Container {
            announce_handler,
            scrape_handler,
            authentication_service,
        }
    }

    fn sample_info_hashes() -> Vec<InfoHash> {
        vec![sample_info_hash()]
    }

    fn sample_peer() -> peer::Peer {
        peer::Peer {
            peer_id: PeerId(*b"-qB00000000000000000"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes::new(0),
            downloaded: NumberOfBytes::new(0),
            left: NumberOfBytes::new(0),
            event: AnnounceEvent::Started,
        }
    }

    mock! {
        HttpStatsEventSender {}
        impl torrust_tracker_events::sender::Sender for HttpStatsEventSender {
             type Event = Event;

             fn send(&self, event: Event) -> BoxFuture<'static,Option<Result<usize,SendError<Event> > > > ;
        }
    }

    mod with_real_data {

        use std::future;
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
        use std::sync::Arc;

        use bittorrent_http_tracker_protocol::v1::requests::scrape::Scrape;
        use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::{ClientIpSources, RemoteClientAddr, ResolvedIp};
        use bittorrent_tracker_core::announce_handler::PeersWanted;
        use mockall::predicate::eq;
        use torrust_tracker_events::bus::SenderStatus;
        use torrust_tracker_primitives::core::ScrapeData;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};
        use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
        use torrust_tracker_test_helpers::configuration;

        use crate::event::bus::EventBus;
        use crate::event::sender::Broadcaster;
        use crate::event::{ConnectionContext, Event};
        use crate::services::scrape::tests::{
            initialize_services_with_configuration, sample_info_hashes, sample_peer, MockHttpStatsEventSender,
        };
        use crate::services::scrape::ScrapeService;
        use crate::tests::sample_info_hash;

        #[tokio::test]
        async fn it_should_return_the_scrape_data_for_a_torrent() {
            let configuration = configuration::ephemeral_public();
            let core_config = Arc::new(configuration.core.clone());

            // HTTP core stats
            let http_core_broadcaster = Broadcaster::default();
            let http_stats_event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, http_core_broadcaster.clone()));

            let http_stats_event_sender = http_stats_event_bus.sender();

            let container = initialize_services_with_configuration(&configuration);

            let info_hash = sample_info_hash();
            let info_hashes = vec![info_hash];

            // Announce a new peer to force scrape data to contain non zeroed data
            let mut peer = sample_peer();
            let original_peer_ip = peer.ip();
            container
                .announce_handler
                .handle_announcement(&info_hash, &mut peer, &original_peer_ip, &PeersWanted::AsManyAsPossible)
                .await
                .unwrap();

            let scrape_request = Scrape {
                info_hashes: info_hashes.clone(),
            };

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: Some(SocketAddr::new(original_peer_ip, 8080)),
            };

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let scrape_service = Arc::new(ScrapeService::new(
                core_config.clone(),
                container.scrape_handler.clone(),
                container.authentication_service.clone(),
                http_stats_event_sender.clone(),
            ));

            let scrape_data = scrape_service
                .handle_scrape(&scrape_request, &client_ip_sources, &server_service_binding, None)
                .await
                .unwrap();

            let mut expected_scrape_data = ScrapeData::empty();
            expected_scrape_data.add_file(
                &info_hash,
                SwarmMetadata {
                    complete: 1,
                    downloaded: 0,
                    incomplete: 0,
                },
            );

            assert_eq!(scrape_data, expected_scrape_data);
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_4_scrape_event_when_the_peer_uses_ipv4() {
            let config = configuration::ephemeral();

            let mut http_stats_event_sender_mock = MockHttpStatsEventSender::new();
            http_stats_event_sender_mock
                .expect_send()
                .with(eq(Event::TcpScrape {
                    connection: ConnectionContext::new(
                        RemoteClientAddr::new(
                            ResolvedIp::FromSocketAddr(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1))),
                            Some(8080),
                        ),
                        ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
                    ),
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let http_stats_event_sender: crate::event::sender::Sender = Some(Arc::new(http_stats_event_sender_mock));

            let container = initialize_services_with_configuration(&config);

            let peer_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1));

            let scrape_request = Scrape {
                info_hashes: sample_info_hashes(),
            };

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: Some(SocketAddr::new(peer_ip, 8080)),
            };

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let scrape_service = Arc::new(ScrapeService::new(
                Arc::new(config.core),
                container.scrape_handler.clone(),
                container.authentication_service.clone(),
                http_stats_event_sender.clone(),
            ));

            scrape_service
                .handle_scrape(&scrape_request, &client_ip_sources, &server_service_binding, None)
                .await
                .unwrap();
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_6_scrape_event_when_the_peer_uses_ipv6() {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let config = configuration::ephemeral();

            let mut http_stats_event_sender_mock = MockHttpStatsEventSender::new();
            http_stats_event_sender_mock
                .expect_send()
                .with(eq(Event::TcpScrape {
                    connection: ConnectionContext::new(
                        RemoteClientAddr::new(
                            ResolvedIp::FromSocketAddr(IpAddr::V6(Ipv6Addr::new(
                                0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969,
                            ))),
                            Some(8080),
                        ),
                        server_service_binding,
                    ),
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let http_stats_event_sender: crate::event::sender::Sender = Some(Arc::new(http_stats_event_sender_mock));

            let container = initialize_services_with_configuration(&config);

            let peer_ip = IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969));

            let scrape_request = Scrape {
                info_hashes: sample_info_hashes(),
            };

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: Some(SocketAddr::new(peer_ip, 8080)),
            };

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let scrape_service = Arc::new(ScrapeService::new(
                Arc::new(config.core),
                container.scrape_handler.clone(),
                container.authentication_service.clone(),
                http_stats_event_sender.clone(),
            ));

            scrape_service
                .handle_scrape(&scrape_request, &client_ip_sources, &server_service_binding, None)
                .await
                .unwrap();
        }
    }

    mod with_zeroed_data {

        use std::future;
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
        use std::sync::Arc;

        use bittorrent_http_tracker_protocol::v1::requests::scrape::Scrape;
        use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::{ClientIpSources, RemoteClientAddr, ResolvedIp};
        use bittorrent_tracker_core::announce_handler::PeersWanted;
        use mockall::predicate::eq;
        use torrust_tracker_events::bus::SenderStatus;
        use torrust_tracker_primitives::core::ScrapeData;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};
        use torrust_tracker_test_helpers::configuration;

        use crate::event::bus::EventBus;
        use crate::event::sender::Broadcaster;
        use crate::event::{ConnectionContext, Event};
        use crate::services::scrape::tests::{
            initialize_services_with_configuration, sample_info_hashes, sample_peer, MockHttpStatsEventSender,
        };
        use crate::services::scrape::ScrapeService;
        use crate::tests::sample_info_hash;

        #[tokio::test]
        async fn it_should_return_the_zeroed_scrape_data_when_the_tracker_is_running_in_private_mode_and_the_peer_is_not_authenticated(
        ) {
            let config = configuration::ephemeral_private();

            let container = initialize_services_with_configuration(&config);

            // HTTP core stats
            let http_core_broadcaster = Broadcaster::default();
            let http_stats_event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, http_core_broadcaster.clone()));

            let http_stats_event_sender = http_stats_event_bus.sender();

            let info_hash = sample_info_hash();
            let info_hashes = vec![info_hash];

            // Announce a new peer to force scrape data to contain non zeroed data
            let mut peer = sample_peer();
            let original_peer_ip = peer.ip();
            container
                .announce_handler
                .handle_announcement(&info_hash, &mut peer, &original_peer_ip, &PeersWanted::AsManyAsPossible)
                .await
                .unwrap();

            let scrape_request = Scrape {
                info_hashes: sample_info_hashes(),
            };

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: Some(SocketAddr::new(original_peer_ip, 8080)),
            };

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let scrape_service = Arc::new(ScrapeService::new(
                Arc::new(config.core),
                container.scrape_handler.clone(),
                container.authentication_service.clone(),
                http_stats_event_sender.clone(),
            ));

            let scrape_data = scrape_service
                .handle_scrape(&scrape_request, &client_ip_sources, &server_service_binding, None)
                .await
                .unwrap();

            let expected_scrape_data = ScrapeData::zeroed(&info_hashes);

            assert_eq!(scrape_data, expected_scrape_data);
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_4_scrape_event_when_the_peer_uses_ipv4() {
            let config = configuration::ephemeral();

            let container = initialize_services_with_configuration(&config);

            let mut http_stats_event_sender_mock = MockHttpStatsEventSender::new();
            http_stats_event_sender_mock
                .expect_send()
                .with(eq(Event::TcpScrape {
                    connection: ConnectionContext::new(
                        RemoteClientAddr::new(
                            ResolvedIp::FromSocketAddr(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1))),
                            Some(8080),
                        ),
                        ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
                    ),
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let http_stats_event_sender: crate::event::sender::Sender = Some(Arc::new(http_stats_event_sender_mock));

            let peer_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1));

            let scrape_request = Scrape {
                info_hashes: sample_info_hashes(),
            };

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: Some(SocketAddr::new(peer_ip, 8080)),
            };

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let scrape_service = Arc::new(ScrapeService::new(
                Arc::new(config.core),
                container.scrape_handler.clone(),
                container.authentication_service.clone(),
                http_stats_event_sender.clone(),
            ));

            scrape_service
                .handle_scrape(&scrape_request, &client_ip_sources, &server_service_binding, None)
                .await
                .unwrap();
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_6_scrape_event_when_the_peer_uses_ipv6() {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let config = configuration::ephemeral();

            let container = initialize_services_with_configuration(&config);

            let mut http_stats_event_sender_mock = MockHttpStatsEventSender::new();
            http_stats_event_sender_mock
                .expect_send()
                .with(eq(Event::TcpScrape {
                    connection: ConnectionContext::new(
                        RemoteClientAddr::new(
                            ResolvedIp::FromSocketAddr(IpAddr::V6(Ipv6Addr::new(
                                0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969,
                            ))),
                            Some(8080),
                        ),
                        server_service_binding,
                    ),
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let http_stats_event_sender: crate::event::sender::Sender = Some(Arc::new(http_stats_event_sender_mock));

            let peer_ip = IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969));

            let scrape_request = Scrape {
                info_hashes: sample_info_hashes(),
            };

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: Some(SocketAddr::new(peer_ip, 8080)),
            };

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let scrape_service = Arc::new(ScrapeService::new(
                Arc::new(config.core),
                container.scrape_handler.clone(),
                container.authentication_service.clone(),
                http_stats_event_sender.clone(),
            ));

            scrape_service
                .handle_scrape(&scrape_request, &client_ip_sources, &server_service_binding, None)
                .await
                .unwrap();
        }
    }
}
