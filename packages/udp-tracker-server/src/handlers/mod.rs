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
use bittorrent_tracker_core::MAX_SCRAPE_TORRENTS;
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use connect::handle_connect;
use error::handle_error;
use scrape::handle_scrape;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_primitives::service_binding::ServiceBinding;
use tracing::{instrument, Level};
use uuid::Uuid;

use super::RawRequest;
use crate::container::UdpTrackerServerContainer;
use crate::error::Error;
use crate::event::UdpRequestKind;
use crate::CurrentClock;

#[derive(Debug, Clone, PartialEq)]
pub struct CookieTimeValues {
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
#[instrument(fields(request_id), skip(udp_request, udp_tracker_core_container, udp_tracker_server_container, cookie_time_values), ret(level = Level::TRACE))]
pub(crate) async fn handle_packet(
    udp_request: RawRequest,
    udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
    udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
    server_service_binding: ServiceBinding,
    cookie_time_values: CookieTimeValues,
) -> (Response, Option<UdpRequestKind>) {
    let request_id = Uuid::new_v4();

    tracing::Span::current().record("request_id", request_id.to_string());
    tracing::debug!("Handling Packets: {udp_request:?}");

    let start_time = Instant::now();

    let (response, opt_req_kind) =
        match Request::parse_bytes(&udp_request.payload[..udp_request.payload.len()], MAX_SCRAPE_TORRENTS).map_err(Error::from) {
            Ok(request) => match handle_request(
                request,
                udp_request.from,
                server_service_binding.clone(),
                udp_tracker_core_container.clone(),
                udp_tracker_server_container.clone(),
                cookie_time_values.clone(),
            )
            .await
            {
                Ok((response, req_kid)) => return (response, Some(req_kid)),
                Err((error, transaction_id, req_kind)) => {
                    let response = handle_error(
                        Some(req_kind.clone()),
                        udp_request.from,
                        server_service_binding,
                        request_id,
                        &udp_tracker_server_container.stats_event_sender,
                        cookie_time_values.valid_range.clone(),
                        &error,
                        Some(transaction_id),
                    )
                    .await;

                    (response, Some(req_kind))
                }
            },
            Err(e) => {
                // The request payload could not be parsed, so we handle it as an error.

                let opt_transaction_id = if let Error::InvalidRequest { request_parse_error } = e.clone() {
                    request_parse_error.opt_transaction_id
                } else {
                    None
                };

                let response = handle_error(
                    None,
                    udp_request.from,
                    server_service_binding,
                    request_id,
                    &udp_tracker_server_container.stats_event_sender,
                    cookie_time_values.valid_range.clone(),
                    &e,
                    opt_transaction_id,
                )
                .await;

                (response, None)
            }
        };

    let latency = start_time.elapsed();
    tracing::trace!(?latency, "responded");

    (response, opt_req_kind)
}

/// It dispatches the request to the correct handler.
///
/// # Errors
///
/// If a error happens in the `handle_request` function, it will just return the  `ServerError`.
#[instrument(skip(
    request,
    client_socket_addr,
    server_service_binding,
    udp_tracker_core_container,
    udp_tracker_server_container,
    cookie_time_values
))]
pub async fn handle_request(
    request: Request,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
    udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
    cookie_time_values: CookieTimeValues,
) -> Result<(Response, UdpRequestKind), (Error, TransactionId, UdpRequestKind)> {
    tracing::trace!("handle request");

    match request {
        Request::Connect(connect_request) => Ok((
            handle_connect(
                client_socket_addr,
                server_service_binding,
                &connect_request,
                &udp_tracker_core_container.connect_service,
                &udp_tracker_server_container.stats_event_sender,
                cookie_time_values.issue_time,
            )
            .await,
            UdpRequestKind::Connect,
        )),
        Request::Announce(announce_request) => {
            match handle_announce(
                &udp_tracker_core_container.announce_service,
                client_socket_addr,
                server_service_binding,
                &announce_request,
                &udp_tracker_core_container.tracker_core_container.core_config,
                &udp_tracker_server_container.stats_event_sender,
                cookie_time_values.valid_range,
            )
            .await
            {
                Ok(response) => Ok((response, UdpRequestKind::Announce { announce_request })),
                Err(err) => Err(err),
            }
        }
        Request::Scrape(scrape_request) => {
            match handle_scrape(
                &udp_tracker_core_container.scrape_service,
                client_socket_addr,
                server_service_binding,
                &scrape_request,
                &udp_tracker_server_container.stats_event_sender,
                cookie_time_values.valid_range,
            )
            .await
            {
                Ok(response) => Ok((response, UdpRequestKind::Scrape)),
                Err(err) => Err(err),
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    use std::ops::Range;
    use std::sync::Arc;

    use bittorrent_tracker_core::announce_handler::AnnounceHandler;
    use bittorrent_tracker_core::databases::setup::initialize_database;
    use bittorrent_tracker_core::scrape_handler::ScrapeHandler;
    use bittorrent_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use bittorrent_tracker_core::whitelist;
    use bittorrent_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use bittorrent_udp_tracker_core::connection_cookie::gen_remote_fingerprint;
    use bittorrent_udp_tracker_core::event::bus::EventBus;
    use bittorrent_udp_tracker_core::event::sender::Broadcaster;
    use bittorrent_udp_tracker_core::services::announce::AnnounceService;
    use bittorrent_udp_tracker_core::services::scrape::ScrapeService;
    use bittorrent_udp_tracker_core::{self, event as core_event};
    use futures::future::BoxFuture;
    use mockall::mock;
    use torrust_tracker_configuration::{Configuration, Core};
    use torrust_tracker_events::bus::SenderStatus;
    use torrust_tracker_events::sender::SendError;
    use torrust_tracker_test_helpers::configuration;

    use crate::event as server_event;

    pub(crate) struct CoreTrackerServices {
        pub core_config: Arc<Core>,
        pub announce_handler: Arc<AnnounceHandler>,
        pub in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
        pub in_memory_whitelist: Arc<InMemoryWhitelist>,
        pub whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    }

    pub(crate) struct CoreUdpTrackerServices {
        pub announce_service: Arc<AnnounceService>,
        pub scrape_service: Arc<ScrapeService>,
    }

    pub(crate) struct ServerUdpTrackerServices {
        pub udp_server_stats_event_sender: crate::event::sender::Sender,
    }

    fn default_testing_tracker_configuration() -> Configuration {
        configuration::ephemeral()
    }

    pub(crate) fn initialize_core_tracker_services_for_default_tracker_configuration(
    ) -> (CoreTrackerServices, CoreUdpTrackerServices, ServerUdpTrackerServices) {
        initialize_core_tracker_services(&default_testing_tracker_configuration())
    }

    pub(crate) fn initialize_core_tracker_services_for_public_tracker(
    ) -> (CoreTrackerServices, CoreUdpTrackerServices, ServerUdpTrackerServices) {
        initialize_core_tracker_services(&configuration::ephemeral_public())
    }

    pub(crate) fn initialize_core_tracker_services_for_listed_tracker(
    ) -> (CoreTrackerServices, CoreUdpTrackerServices, ServerUdpTrackerServices) {
        initialize_core_tracker_services(&configuration::ephemeral_listed())
    }

    fn initialize_core_tracker_services(
        config: &Configuration,
    ) -> (CoreTrackerServices, CoreUdpTrackerServices, ServerUdpTrackerServices) {
        let core_config = Arc::new(config.core.clone());
        let database = initialize_database(&config.core);
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let db_downloads_metric_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&database));
        let announce_handler = Arc::new(AnnounceHandler::new(
            &config.core,
            &whitelist_authorization,
            &in_memory_torrent_repository,
            &db_downloads_metric_repository,
        ));
        let scrape_handler = Arc::new(ScrapeHandler::new(&whitelist_authorization, &in_memory_torrent_repository));

        let udp_core_broadcaster = Broadcaster::default();
        let core_event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, udp_core_broadcaster.clone()));
        let udp_core_stats_event_sender = core_event_bus.sender();

        let udp_server_broadcaster = crate::event::sender::Broadcaster::default();
        let server_event_bus = Arc::new(crate::event::bus::EventBus::new(
            SenderStatus::Disabled,
            udp_server_broadcaster.clone(),
        ));

        let udp_server_stats_event_sender = server_event_bus.sender();

        let announce_service = Arc::new(AnnounceService::new(
            announce_handler.clone(),
            whitelist_authorization.clone(),
            udp_core_stats_event_sender.clone(),
        ));

        let scrape_service = Arc::new(ScrapeService::new(
            scrape_handler.clone(),
            udp_core_stats_event_sender.clone(),
        ));

        (
            CoreTrackerServices {
                core_config,
                announce_handler,
                in_memory_torrent_repository,
                in_memory_whitelist,
                whitelist_authorization,
            },
            CoreUdpTrackerServices {
                announce_service,
                scrape_service,
            },
            ServerUdpTrackerServices {
                udp_server_stats_event_sender,
            },
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
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080)
    }

    fn sample_ipv6_socket_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 8080)
    }

    pub(crate) fn sample_issue_time() -> f64 {
        1_000_000_000_f64
    }

    pub(crate) fn sample_cookie_valid_range() -> Range<f64> {
        sample_issue_time() - 10.0..sample_issue_time() + 10.0
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
        pub(crate) UdpCoreStatsEventSender {}
        impl torrust_tracker_events::sender::Sender for UdpCoreStatsEventSender {
            type Event = core_event::Event;

            fn send(&self, event: core_event::Event) -> BoxFuture<'static,Option<Result<usize,SendError<core_event::Event> > > > ;
        }
    }

    mock! {
        pub(crate) UdpServerStatsEventSender {}
        impl torrust_tracker_events::sender::Sender for UdpServerStatsEventSender {
            type Event = server_event::Event;

            fn send(&self, event: server_event::Event) -> BoxFuture<'static,Option<Result<usize,SendError<server_event::Event> > > > ;
        }
    }
}
