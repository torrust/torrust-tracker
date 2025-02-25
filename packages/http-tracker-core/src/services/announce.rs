//! The `announce` service.
//!
//! The service is responsible for handling the `announce` requests.
//!
//! It delegates the `announce` logic to the [`AnnounceHandler`] and it returns
//! the [`AnnounceData`].
//!
//! It also sends an [`http_tracker_core::statistics::event::Event`]
//! because events are specific for the HTTP tracker.
use std::net::IpAddr;
use std::panic::Location;
use std::sync::Arc;

use bittorrent_http_tracker_protocol::v1::requests::announce::{peer_from_request, Announce};
use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::{self, ClientIpSources, PeerIpResolutionError};
use bittorrent_tracker_core::announce_handler::{AnnounceHandler, PeersWanted};
use bittorrent_tracker_core::authentication::service::AuthenticationService;
use bittorrent_tracker_core::authentication::{self, Key};
use bittorrent_tracker_core::error::{AnnounceError, TrackerCoreError, WhitelistError};
use bittorrent_tracker_core::whitelist;
use torrust_tracker_configuration::Core;
use torrust_tracker_primitives::core::AnnounceData;

use crate::statistics;

/// Errors related to announce requests.
#[derive(thiserror::Error, Debug, Clone)]
pub enum HttpAnnounceError {
    #[error("Error resolving peer IP: {source}")]
    PeerIpResolutionError { source: PeerIpResolutionError },

    #[error("Tracker core error: {source}")]
    TrackerCoreError { source: TrackerCoreError },
}

impl From<PeerIpResolutionError> for HttpAnnounceError {
    fn from(peer_ip_resolution_error: PeerIpResolutionError) -> Self {
        Self::PeerIpResolutionError {
            source: peer_ip_resolution_error,
        }
    }
}

impl From<TrackerCoreError> for HttpAnnounceError {
    fn from(tracker_core_error: TrackerCoreError) -> Self {
        Self::TrackerCoreError {
            source: tracker_core_error,
        }
    }
}

impl From<AnnounceError> for HttpAnnounceError {
    fn from(announce_error: AnnounceError) -> Self {
        Self::TrackerCoreError {
            source: announce_error.into(),
        }
    }
}

impl From<WhitelistError> for HttpAnnounceError {
    fn from(whitelist_error: WhitelistError) -> Self {
        Self::TrackerCoreError {
            source: whitelist_error.into(),
        }
    }
}

impl From<authentication::key::Error> for HttpAnnounceError {
    fn from(whitelist_error: authentication::key::Error) -> Self {
        Self::TrackerCoreError {
            source: whitelist_error.into(),
        }
    }
}

/// The HTTP tracker `announce` service.
///
/// The service sends an statistics event that increments:
///
/// - The number of TCP connections handled by the HTTP tracker.
/// - The number of TCP `announce` requests handled by the HTTP tracker.
///
/// > **NOTICE**: as the HTTP tracker does not requires a connection request
/// > like the UDP tracker, the number of TCP connections is incremented for
/// > each `announce` request.
///
/// # Errors
///
/// This function will return an error if:
///
/// - The tracker is running in `listed` mode and the torrent is not whitelisted.
/// - There is an error when resolving the client IP address.
#[allow(clippy::too_many_arguments)]
pub async fn handle_announce(
    core_config: &Arc<Core>,
    announce_handler: &Arc<AnnounceHandler>,
    authentication_service: &Arc<AuthenticationService>,
    whitelist_authorization: &Arc<whitelist::authorization::WhitelistAuthorization>,
    opt_http_stats_event_sender: &Arc<Option<Box<dyn statistics::event::sender::Sender>>>,
    announce_request: &Announce,
    client_ip_sources: &ClientIpSources,
    maybe_key: Option<Key>,
) -> Result<AnnounceData, HttpAnnounceError> {
    // Authentication
    if core_config.private {
        match maybe_key {
            Some(key) => match authentication_service.authenticate(&key).await {
                Ok(()) => (),
                Err(error) => return Err(error.into()),
            },
            None => {
                return Err(authentication::key::Error::MissingAuthKey {
                    location: Location::caller(),
                }
                .into())
            }
        }
    }

    // Authorization
    match whitelist_authorization.authorize(&announce_request.info_hash).await {
        Ok(()) => (),
        Err(error) => return Err(error.into()),
    }

    let peer_ip = match peer_ip_resolver::invoke(core_config.net.on_reverse_proxy, client_ip_sources) {
        Ok(peer_ip) => peer_ip,
        Err(error) => return Err(error.into()),
    };

    let mut peer = peer_from_request(announce_request, &peer_ip);

    let peers_wanted = match announce_request.numwant {
        Some(numwant) => PeersWanted::only(numwant),
        None => PeersWanted::AsManyAsPossible,
    };

    let original_peer_ip = peer.peer_addr.ip();

    // The tracker could change the original peer ip
    let announce_data = announce_handler
        .announce(&announce_request.info_hash, &mut peer, &original_peer_ip, &peers_wanted)
        .await?;

    if let Some(http_stats_event_sender) = opt_http_stats_event_sender.as_deref() {
        match original_peer_ip {
            IpAddr::V4(_) => {
                http_stats_event_sender
                    .send_event(statistics::event::Event::Tcp4Announce)
                    .await;
            }
            IpAddr::V6(_) => {
                http_stats_event_sender
                    .send_event(statistics::event::Event::Tcp6Announce)
                    .await;
            }
        }
    }

    Ok(announce_data)
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    use std::sync::Arc;

    use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
    use bittorrent_http_tracker_protocol::v1::requests::announce::Announce;
    use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::ClientIpSources;
    use bittorrent_tracker_core::announce_handler::AnnounceHandler;
    use bittorrent_tracker_core::authentication::key::repository::in_memory::InMemoryKeyRepository;
    use bittorrent_tracker_core::authentication::service::AuthenticationService;
    use bittorrent_tracker_core::databases::setup::initialize_database;
    use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use bittorrent_tracker_core::torrent::repository::persisted::DatabasePersistentTorrentRepository;
    use bittorrent_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use torrust_tracker_configuration::{Configuration, Core};
    use torrust_tracker_primitives::peer::Peer;
    use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};
    use torrust_tracker_test_helpers::configuration;

    struct CoreTrackerServices {
        pub core_config: Arc<Core>,
        pub announce_handler: Arc<AnnounceHandler>,
        pub authentication_service: Arc<AuthenticationService>,
        pub whitelist_authorization: Arc<WhitelistAuthorization>,
    }

    struct CoreHttpTrackerServices {
        pub http_stats_event_sender: Arc<Option<Box<dyn statistics::event::sender::Sender>>>,
    }

    fn initialize_core_tracker_services() -> (CoreTrackerServices, CoreHttpTrackerServices) {
        initialize_core_tracker_services_with_config(&configuration::ephemeral_public())
    }

    fn initialize_core_tracker_services_with_config(config: &Configuration) -> (CoreTrackerServices, CoreHttpTrackerServices) {
        let core_config = Arc::new(config.core.clone());
        let database = initialize_database(&config.core);
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let db_torrent_repository = Arc::new(DatabasePersistentTorrentRepository::new(&database));
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
        let in_memory_key_repository = Arc::new(InMemoryKeyRepository::default());
        let authentication_service = Arc::new(AuthenticationService::new(&core_config, &in_memory_key_repository));

        let announce_handler = Arc::new(AnnounceHandler::new(
            &config.core,
            &whitelist_authorization,
            &in_memory_torrent_repository,
            &db_torrent_repository,
        ));

        // HTTP stats
        let (http_stats_event_sender, http_stats_repository) = statistics::setup::factory(config.core.tracker_usage_statistics);
        let http_stats_event_sender = Arc::new(http_stats_event_sender);
        let _http_stats_repository = Arc::new(http_stats_repository);

        (
            CoreTrackerServices {
                core_config,
                announce_handler,
                authentication_service,
                whitelist_authorization,
            },
            CoreHttpTrackerServices { http_stats_event_sender },
        )
    }

    fn sample_peer_using_ipv4() -> peer::Peer {
        sample_peer()
    }

    fn sample_peer_using_ipv6() -> peer::Peer {
        let mut peer = sample_peer();
        peer.peer_addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
            8080,
        );
        peer
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

    fn sample_announce_request_for_peer(peer: Peer) -> (Announce, ClientIpSources) {
        let announce_request = Announce {
            info_hash: sample_info_hash(),
            peer_id: peer.peer_id,
            port: peer.peer_addr.port(),
            uploaded: Some(peer.uploaded),
            downloaded: Some(peer.downloaded),
            left: Some(peer.left),
            event: Some(peer.event.into()),
            compact: None,
            numwant: None,
        };

        let client_ip_sources = ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_ip: Some(peer.peer_addr.ip()),
        };

        (announce_request, client_ip_sources)
    }

    use futures::future::BoxFuture;
    use mockall::mock;
    use tokio::sync::mpsc::error::SendError;

    use crate::statistics;
    use crate::tests::sample_info_hash;

    mock! {
        HttpStatsEventSender {}
        impl statistics::event::sender::Sender for HttpStatsEventSender {
             fn send_event(&self, event: statistics::event::Event) -> BoxFuture<'static,Option<Result<(),SendError<statistics::event::Event> > > > ;
        }
    }

    mod with_tracker_in_any_mode {
        use std::future;
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
        use std::sync::Arc;

        use mockall::predicate::eq;
        use torrust_tracker_configuration::Configuration;
        use torrust_tracker_primitives::core::AnnounceData;
        use torrust_tracker_primitives::peer;
        use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
        use torrust_tracker_test_helpers::configuration;

        use super::{sample_peer_using_ipv4, sample_peer_using_ipv6};
        use crate::services::announce::handle_announce;
        use crate::services::announce::tests::{
            initialize_core_tracker_services, initialize_core_tracker_services_with_config, sample_announce_request_for_peer,
            sample_peer, MockHttpStatsEventSender,
        };
        use crate::statistics;

        #[tokio::test]
        async fn it_should_return_the_announce_data() {
            let (core_tracker_services, core_http_tracker_services) = initialize_core_tracker_services();

            let peer = sample_peer();

            let (announce_request, client_ip_sources) = sample_announce_request_for_peer(peer);

            let announce_data = handle_announce(
                &core_tracker_services.core_config,
                &core_tracker_services.announce_handler,
                &core_tracker_services.authentication_service,
                &core_tracker_services.whitelist_authorization,
                &core_http_tracker_services.http_stats_event_sender,
                &announce_request,
                &client_ip_sources,
                None,
            )
            .await
            .unwrap();

            let expected_announce_data = AnnounceData {
                peers: vec![],
                stats: SwarmMetadata {
                    downloaded: 0,
                    complete: 1,
                    incomplete: 0,
                },
                policy: core_tracker_services.core_config.announce_policy,
            };

            assert_eq!(announce_data, expected_announce_data);
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_4_announce_event_when_the_peer_uses_ipv4() {
            let mut http_stats_event_sender_mock = MockHttpStatsEventSender::new();
            http_stats_event_sender_mock
                .expect_send_event()
                .with(eq(statistics::event::Event::Tcp4Announce))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let http_stats_event_sender: Arc<Option<Box<dyn statistics::event::sender::Sender>>> =
                Arc::new(Some(Box::new(http_stats_event_sender_mock)));

            let (core_tracker_services, mut core_http_tracker_services) = initialize_core_tracker_services();
            core_http_tracker_services.http_stats_event_sender = http_stats_event_sender;

            let peer = sample_peer_using_ipv4();

            let (announce_request, client_ip_sources) = sample_announce_request_for_peer(peer);

            let _announce_data = handle_announce(
                &core_tracker_services.core_config,
                &core_tracker_services.announce_handler,
                &core_tracker_services.authentication_service,
                &core_tracker_services.whitelist_authorization,
                &core_http_tracker_services.http_stats_event_sender,
                &announce_request,
                &client_ip_sources,
                None,
            )
            .await
            .unwrap();
        }

        fn tracker_with_an_ipv6_external_ip() -> Configuration {
            let mut configuration = configuration::ephemeral();
            configuration.core.net.external_ip = Some(IpAddr::V6(Ipv6Addr::new(
                0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969,
            )));
            configuration
        }

        fn peer_with_the_ipv4_loopback_ip() -> peer::Peer {
            let loopback_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            let mut peer = sample_peer();
            peer.peer_addr = SocketAddr::new(loopback_ip, 8080);
            peer
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_4_announce_event_when_the_peer_uses_ipv4_even_if_the_tracker_changes_the_peer_ip_to_ipv6()
        {
            // Tracker changes the peer IP to the tracker external IP when the peer is using the loopback IP.

            // Assert that the event sent is a TCP4 event
            let mut http_stats_event_sender_mock = MockHttpStatsEventSender::new();
            http_stats_event_sender_mock
                .expect_send_event()
                .with(eq(statistics::event::Event::Tcp4Announce))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let http_stats_event_sender: Arc<Option<Box<dyn statistics::event::sender::Sender>>> =
                Arc::new(Some(Box::new(http_stats_event_sender_mock)));

            let (core_tracker_services, mut core_http_tracker_services) =
                initialize_core_tracker_services_with_config(&tracker_with_an_ipv6_external_ip());
            core_http_tracker_services.http_stats_event_sender = http_stats_event_sender;

            let peer = peer_with_the_ipv4_loopback_ip();

            let (announce_request, client_ip_sources) = sample_announce_request_for_peer(peer);

            let _announce_data = handle_announce(
                &core_tracker_services.core_config,
                &core_tracker_services.announce_handler,
                &core_tracker_services.authentication_service,
                &core_tracker_services.whitelist_authorization,
                &core_http_tracker_services.http_stats_event_sender,
                &announce_request,
                &client_ip_sources,
                None,
            )
            .await
            .unwrap();
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_6_announce_event_when_the_peer_uses_ipv6_even_if_the_tracker_changes_the_peer_ip_to_ipv4()
        {
            let mut http_stats_event_sender_mock = MockHttpStatsEventSender::new();
            http_stats_event_sender_mock
                .expect_send_event()
                .with(eq(statistics::event::Event::Tcp6Announce))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let http_stats_event_sender: Arc<Option<Box<dyn statistics::event::sender::Sender>>> =
                Arc::new(Some(Box::new(http_stats_event_sender_mock)));

            let (core_tracker_services, mut core_http_tracker_services) = initialize_core_tracker_services();
            core_http_tracker_services.http_stats_event_sender = http_stats_event_sender;

            let peer = sample_peer_using_ipv6();

            let (announce_request, client_ip_sources) = sample_announce_request_for_peer(peer);

            let _announce_data = handle_announce(
                &core_tracker_services.core_config,
                &core_tracker_services.announce_handler,
                &core_tracker_services.authentication_service,
                &core_tracker_services.whitelist_authorization,
                &core_http_tracker_services.http_stats_event_sender,
                &announce_request,
                &client_ip_sources,
                None,
            )
            .await
            .unwrap();
        }
    }
}
