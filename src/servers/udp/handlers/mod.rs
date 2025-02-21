//! Handlers for the UDP server.
pub mod announce;
pub mod connect;
pub mod error;
pub mod scrape;

use std::net::SocketAddr;
use std::ops::Range;
use std::sync::Arc;
use std::time::Instant;

use announce::handle_announce;
use aquatic_udp_protocol::{Request, Response, TransactionId};
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use bittorrent_udp_tracker_core::services::announce::UdpAnnounceError;
use connect::handle_connect;
use error::handle_error;
use scrape::handle_scrape;
use torrust_tracker_clock::clock::Time as _;
use tracing::{instrument, Level};
use uuid::Uuid;

use super::RawRequest;
use crate::servers::udp::error::Error;
use crate::shared::bit_torrent::common::MAX_SCRAPE_TORRENTS;
use crate::CurrentClock;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct CookieTimeValues {
    pub(super) issue_time: f64,
    pub(super) valid_range: Range<f64>,
}

impl CookieTimeValues {
    pub(super) fn new(cookie_lifetime: f64) -> Self {
        let issue_time = CurrentClock::now().as_secs_f64();
        let expiry_time = issue_time - cookie_lifetime - 1.0;
        let tolerance_max_time = issue_time + 1.0;

        Self {
            issue_time,
            valid_range: expiry_time..tolerance_max_time,
        }
    }
}

/// It handles the incoming UDP packets.
///
/// It's responsible for:
///
/// - Parsing the incoming packet.
/// - Delegating the request to the correct handler depending on the request type.
///
/// It will return an `Error` response if the request is invalid.
#[instrument(fields(request_id), skip(udp_request, udp_tracker_container, cookie_time_values), ret(level = Level::TRACE))]
pub(crate) async fn handle_packet(
    udp_request: RawRequest,
    udp_tracker_container: Arc<UdpTrackerCoreContainer>,
    local_addr: SocketAddr,
    cookie_time_values: CookieTimeValues,
) -> Response {
    let request_id = Uuid::new_v4();

    tracing::Span::current().record("request_id", request_id.to_string());
    tracing::debug!("Handling Packets: {udp_request:?}");

    let start_time = Instant::now();

    let response =
        match Request::parse_bytes(&udp_request.payload[..udp_request.payload.len()], MAX_SCRAPE_TORRENTS).map_err(Error::from) {
            Ok(request) => match handle_request(
                request,
                udp_request.from,
                udp_tracker_container.clone(),
                cookie_time_values.clone(),
            )
            .await
            {
                Ok(response) => return response,
                Err((error, transaction_id)) => {
                    if let Error::UdpAnnounceError {
                        source: UdpAnnounceError::ConnectionCookieError { .. },
                    } = error
                    {
                        // code-review: should we include `RequestParseError` and `BadRequest`?
                        let mut ban_service = udp_tracker_container.ban_service.write().await;
                        ban_service.increase_counter(&udp_request.from.ip());
                    }

                    handle_error(
                        udp_request.from,
                        local_addr,
                        request_id,
                        &udp_tracker_container.udp_stats_event_sender,
                        cookie_time_values.valid_range.clone(),
                        &error,
                        Some(transaction_id),
                    )
                    .await
                }
            },
            Err(e) => {
                handle_error(
                    udp_request.from,
                    local_addr,
                    request_id,
                    &udp_tracker_container.udp_stats_event_sender,
                    cookie_time_values.valid_range.clone(),
                    &e,
                    None,
                )
                .await
            }
        };

    let latency = start_time.elapsed();
    tracing::trace!(?latency, "responded");

    response
}

/// It dispatches the request to the correct handler.
///
/// # Errors
///
/// If a error happens in the `handle_request` function, it will just return the  `ServerError`.
#[instrument(skip(request, remote_addr, udp_tracker_container, cookie_time_values))]
pub async fn handle_request(
    request: Request,
    remote_addr: SocketAddr,
    udp_tracker_container: Arc<UdpTrackerCoreContainer>,
    cookie_time_values: CookieTimeValues,
) -> Result<Response, (Error, TransactionId)> {
    tracing::trace!("handle request");

    match request {
        Request::Connect(connect_request) => Ok(handle_connect(
            remote_addr,
            &connect_request,
            &udp_tracker_container.udp_stats_event_sender,
            cookie_time_values.issue_time,
        )
        .await),
        Request::Announce(announce_request) => {
            handle_announce(
                remote_addr,
                &announce_request,
                &udp_tracker_container.core_config,
                &udp_tracker_container.announce_handler,
                &udp_tracker_container.whitelist_authorization,
                &udp_tracker_container.udp_stats_event_sender,
                cookie_time_values.valid_range,
            )
            .await
        }
        Request::Scrape(scrape_request) => {
            handle_scrape(
                remote_addr,
                &scrape_request,
                &udp_tracker_container.scrape_handler,
                &udp_tracker_container.udp_stats_event_sender,
                cookie_time_values.valid_range,
            )
            .await
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    use std::ops::Range;
    use std::sync::Arc;

    use aquatic_udp_protocol::{NumberOfBytes, PeerId};
    use bittorrent_tracker_core::announce_handler::AnnounceHandler;
    use bittorrent_tracker_core::databases::setup::initialize_database;
    use bittorrent_tracker_core::scrape_handler::ScrapeHandler;
    use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use bittorrent_tracker_core::torrent::repository::persisted::DatabasePersistentTorrentRepository;
    use bittorrent_tracker_core::whitelist;
    use bittorrent_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use bittorrent_udp_tracker_core::connection_cookie::gen_remote_fingerprint;
    use bittorrent_udp_tracker_core::{self, statistics};
    use futures::future::BoxFuture;
    use mockall::mock;
    use tokio::sync::mpsc::error::SendError;
    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_configuration::{Configuration, Core};
    use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};
    use torrust_tracker_test_helpers::configuration;

    use crate::CurrentClock;

    pub(crate) struct CoreTrackerServices {
        pub core_config: Arc<Core>,
        pub announce_handler: Arc<AnnounceHandler>,
        pub scrape_handler: Arc<ScrapeHandler>,
        pub in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
        pub in_memory_whitelist: Arc<InMemoryWhitelist>,
        pub whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    }

    pub(crate) struct CoreUdpTrackerServices {
        pub udp_stats_event_sender: Arc<Option<Box<dyn statistics::event::sender::Sender>>>,
    }

    fn default_testing_tracker_configuration() -> Configuration {
        configuration::ephemeral()
    }

    pub(crate) fn initialize_core_tracker_services_for_default_tracker_configuration(
    ) -> (CoreTrackerServices, CoreUdpTrackerServices) {
        initialize_core_tracker_services(&default_testing_tracker_configuration())
    }

    pub(crate) fn initialize_core_tracker_services_for_public_tracker() -> (CoreTrackerServices, CoreUdpTrackerServices) {
        initialize_core_tracker_services(&configuration::ephemeral_public())
    }

    pub(crate) fn initialize_core_tracker_services_for_listed_tracker() -> (CoreTrackerServices, CoreUdpTrackerServices) {
        initialize_core_tracker_services(&configuration::ephemeral_listed())
    }

    fn initialize_core_tracker_services(config: &Configuration) -> (CoreTrackerServices, CoreUdpTrackerServices) {
        let core_config = Arc::new(config.core.clone());
        let database = initialize_database(&config.core);
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let db_torrent_repository = Arc::new(DatabasePersistentTorrentRepository::new(&database));
        let announce_handler = Arc::new(AnnounceHandler::new(
            &config.core,
            &whitelist_authorization,
            &in_memory_torrent_repository,
            &db_torrent_repository,
        ));
        let scrape_handler = Arc::new(ScrapeHandler::new(&whitelist_authorization, &in_memory_torrent_repository));

        let (udp_stats_event_sender, _udp_stats_repository) = bittorrent_udp_tracker_core::statistics::setup::factory(false);
        let udp_stats_event_sender = Arc::new(udp_stats_event_sender);

        (
            CoreTrackerServices {
                core_config,
                announce_handler,
                scrape_handler,
                in_memory_torrent_repository,
                in_memory_whitelist,
                whitelist_authorization,
            },
            CoreUdpTrackerServices { udp_stats_event_sender },
        )
    }

    pub(crate) fn sample_ipv4_remote_addr() -> SocketAddr {
        sample_ipv4_socket_address()
    }

    pub(crate) fn sample_ipv4_remote_addr_fingerprint() -> u64 {
        gen_remote_fingerprint(&sample_ipv4_socket_address())
    }

    pub(crate) fn sample_ipv6_remote_addr() -> SocketAddr {
        sample_ipv6_socket_address()
    }

    pub(crate) fn sample_ipv6_remote_addr_fingerprint() -> u64 {
        gen_remote_fingerprint(&sample_ipv6_socket_address())
    }

    pub(crate) fn sample_ipv4_socket_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    }

    fn sample_ipv6_socket_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8080)
    }

    pub(crate) fn sample_issue_time() -> f64 {
        1_000_000_000_f64
    }

    pub(crate) fn sample_cookie_valid_range() -> Range<f64> {
        sample_issue_time() - 10.0..sample_issue_time() + 10.0
    }

    #[derive(Debug, Default)]
    pub(crate) struct TorrentPeerBuilder {
        peer: peer::Peer,
    }

    impl TorrentPeerBuilder {
        #[must_use]
        pub fn new() -> Self {
            Self {
                peer: peer::Peer {
                    updated: CurrentClock::now(),
                    ..Default::default()
                },
            }
        }

        #[must_use]
        pub fn with_peer_address(mut self, peer_addr: SocketAddr) -> Self {
            self.peer.peer_addr = peer_addr;
            self
        }

        #[must_use]
        pub fn with_peer_id(mut self, peer_id: PeerId) -> Self {
            self.peer.peer_id = peer_id;
            self
        }

        #[must_use]
        pub fn with_number_of_bytes_left(mut self, left: i64) -> Self {
            self.peer.left = NumberOfBytes::new(left);
            self
        }

        #[must_use]
        pub fn updated_on(mut self, updated: DurationSinceUnixEpoch) -> Self {
            self.peer.updated = updated;
            self
        }

        #[must_use]
        pub fn into(self) -> peer::Peer {
            self.peer
        }
    }

    pub(crate) struct TrackerConfigurationBuilder {
        configuration: Configuration,
    }

    impl TrackerConfigurationBuilder {
        pub fn default() -> TrackerConfigurationBuilder {
            let default_configuration = default_testing_tracker_configuration();
            TrackerConfigurationBuilder {
                configuration: default_configuration,
            }
        }

        pub fn with_external_ip(mut self, external_ip: &str) -> Self {
            self.configuration.core.net.external_ip = Some(external_ip.to_owned().parse().expect("valid IP address"));
            self
        }

        pub fn into(self) -> Configuration {
            self.configuration
        }
    }

    mock! {
        pub(crate) UdpStatsEventSender {}
        impl statistics::event::sender::Sender for UdpStatsEventSender {
             fn send_event(&self, event: statistics::event::Event) -> BoxFuture<'static,Option<Result<(),SendError<statistics::event::Event> > > > ;
        }
    }
}
