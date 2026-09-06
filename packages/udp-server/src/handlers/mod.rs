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
use connect::handle_connect;
use error::handle_error;
use scrape::handle_scrape;
use torrust_clock::clock::Time;
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_core::MAX_SCRAPE_TORRENTS;
use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_protocol::{Request, Response, TransactionId};
use tracing::{Level, instrument};
use uuid::Uuid;

use super::RawRequest;
use crate::CurrentClock;
use crate::container::UdpTrackerServerContainer;
use crate::error::Error;
use crate::event::UdpRequestKind;

/// Type alias for the common handler error returned by UDP request handlers.
pub(crate) type HandlerError = Box<(Error, TransactionId, UdpRequestKind)>;

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

/// Cookie validation parameters passed to announce and scrape handlers.
///
/// Groups the time-based validity range with the policy that controls whether
/// the cookie is enforced. Both parameters travel together through the handler
/// call chain because they both answer "how should the cookie be validated?".
#[derive(Debug, Clone, PartialEq)]
pub struct CookieValidationContext {
    pub valid_range: Range<f64>,
    pub connection_id_validation: ConnectionIdValidationPolicy,
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
    connection_id_validation: ConnectionIdValidationPolicy,
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
                connection_id_validation,
            )
            .await
            {
                Ok((response, req_kid)) => return (response, Some(req_kid)),
                Err(boxed_err) => {
                    let (error, transaction_id, req_kind) = *boxed_err;
                    let response = handle_error(
                        Some(req_kind.clone()),
                        udp_request.from,
                        server_service_binding,
                        udp_tracker_core_container.configuration_instance_id,
                        udp_tracker_core_container
                            .udp_tracker_config
                            .public_url
                            .as_ref()
                            .map(ToString::to_string),
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

                let opt_transaction_id = match e.clone() {
                    Error::InvalidRequest { request_parse_error } => request_parse_error.opt_transaction_id,
                    _ => None,
                };

                let response = handle_error(
                    None,
                    udp_request.from,
                    server_service_binding,
                    udp_tracker_core_container.configuration_instance_id,
                    udp_tracker_core_container
                        .udp_tracker_config
                        .public_url
                        .as_ref()
                        .map(ToString::to_string),
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
    connection_id_validation: ConnectionIdValidationPolicy,
) -> Result<(Response, UdpRequestKind), HandlerError> {
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
                CookieValidationContext {
                    valid_range: cookie_time_values.valid_range,
                    connection_id_validation,
                },
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
                CookieValidationContext {
                    valid_range: cookie_time_values.valid_range,
                    connection_id_validation,
                },
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

    use futures::future::BoxFuture;
    use mockall::mock;
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_core::announce_handler::AnnounceHandler;
    use torrust_tracker_core::databases::setup::initialize_database;
    use torrust_tracker_core::scrape_handler::ScrapeHandler;
    use torrust_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use torrust_tracker_core::whitelist;
    use torrust_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use torrust_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use torrust_tracker_events::bus::SenderStatus;
    use torrust_tracker_events::sender::SendError;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_test_helpers::configuration;
    use torrust_tracker_udp_core::connection_cookie::gen_remote_fingerprint;
    use torrust_tracker_udp_core::event::bus::EventBus;
    use torrust_tracker_udp_core::event::sender::Broadcaster;
    use torrust_tracker_udp_core::services::announce::AnnounceService;
    use torrust_tracker_udp_core::services::scrape::ScrapeService;
    use torrust_tracker_udp_core::{self, event as core_event};

    use crate::event as server_event;

    pub struct CoreTrackerServices {
        pub core_config: Arc<Core>,
        pub announce_handler: Arc<AnnounceHandler>,
        pub in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
        pub in_memory_whitelist: Arc<InMemoryWhitelist>,
        pub whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    }

    pub struct CoreUdpTrackerServices {
        pub announce_service: Arc<AnnounceService>,
        pub scrape_service: Arc<ScrapeService>,
    }

    pub struct ServerUdpTrackerServices {
        pub udp_server_stats_event_sender: crate::event::sender::Sender,
    }

    fn default_testing_tracker_configuration() -> Configuration {
        configuration::ephemeral()
    }

    pub async fn initialize_core_tracker_services_for_default_tracker_configuration()
    -> (CoreTrackerServices, CoreUdpTrackerServices, ServerUdpTrackerServices) {
        initialize_core_tracker_services(&default_testing_tracker_configuration()).await
    }

    pub async fn initialize_core_tracker_services_for_public_tracker()
    -> (CoreTrackerServices, CoreUdpTrackerServices, ServerUdpTrackerServices) {
        initialize_core_tracker_services(&configuration::ephemeral_public()).await
    }

    pub async fn initialize_core_tracker_services_for_listed_tracker()
    -> (CoreTrackerServices, CoreUdpTrackerServices, ServerUdpTrackerServices) {
        initialize_core_tracker_services(&configuration::ephemeral_listed()).await
    }

    pub async fn initialize_core_tracker_services_with_config(
        config: &Configuration,
    ) -> (CoreTrackerServices, CoreUdpTrackerServices, ServerUdpTrackerServices) {
        initialize_core_tracker_services(config).await
    }

    async fn initialize_core_tracker_services(
        config: &Configuration,
    ) -> (CoreTrackerServices, CoreUdpTrackerServices, ServerUdpTrackerServices) {
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let core_config = Arc::new(config.core.clone());
        let database = initialize_database(&config.core).await;
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let db_downloads_metric_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&database.torrent_metrics_store));
        let announce_handler = if config.core.tracker_policy.persistent_torrent_completed_stat {
            Arc::new(AnnounceHandler::new_with_persistent_completed_statistics(
                &config.core,
                &whitelist_authorization,
                &in_memory_torrent_repository,
                &db_downloads_metric_repository,
            ))
        } else {
            Arc::new(AnnounceHandler::new_public(
                &config.core,
                &whitelist_authorization,
                &in_memory_torrent_repository,
            ))
        };
        let scrape_handler = Arc::new(ScrapeHandler::new(&whitelist_authorization, &in_memory_torrent_repository));

        let udp_core_broadcaster = Broadcaster::default();
        let core_event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, udp_core_broadcaster));
        let udp_core_stats_event_sender = core_event_bus.sender();

        let udp_server_broadcaster = crate::event::sender::Broadcaster::default();
        let server_event_bus = Arc::new(crate::event::bus::EventBus::new(
            SenderStatus::Disabled,
            udp_server_broadcaster,
        ));

        let udp_server_stats_event_sender = server_event_bus.sender();

        let announce_service = Arc::new(AnnounceService::new(
            announce_handler.clone(),
            whitelist_authorization.clone(),
            udp_core_stats_event_sender.clone(),
            configuration_instance_id,
            config.udp_trackers.as_ref().expect("UDP tracker configuration")[0]
                .network
                .external_ip
                .map(Into::into),
        ));

        let scrape_service = Arc::new(ScrapeService::new(
            scrape_handler,
            udp_core_stats_event_sender.clone(),
            configuration_instance_id,
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

    pub fn sample_ipv4_remote_addr() -> SocketAddr {
        sample_ipv4_socket_address()
    }

    pub fn sample_ipv4_remote_addr_fingerprint() -> u64 {
        gen_remote_fingerprint(&sample_ipv4_socket_address())
    }

    pub fn sample_ipv6_remote_addr() -> SocketAddr {
        sample_ipv6_socket_address()
    }

    pub fn sample_ipv6_remote_addr_fingerprint() -> u64 {
        gen_remote_fingerprint(&sample_ipv6_socket_address())
    }

    pub fn sample_ipv4_socket_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080)
    }

    fn sample_ipv6_socket_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 8080)
    }

    pub fn sample_issue_time() -> f64 {
        1_000_000_000_f64
    }

    pub fn sample_cookie_valid_range() -> Range<f64> {
        sample_issue_time() - 10.0..sample_issue_time() + 10.0
    }

    pub fn sample_strict_cookie_validation() -> super::CookieValidationContext {
        super::CookieValidationContext {
            valid_range: sample_cookie_valid_range(),
            connection_id_validation: torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
        }
    }

    pub struct TrackerConfigurationBuilder {
        configuration: Configuration,
    }

    impl TrackerConfigurationBuilder {
        pub fn default() -> Self {
            let default_configuration = default_testing_tracker_configuration();
            Self {
                configuration: default_configuration,
            }
        }

        pub fn with_external_ip(mut self, external_ip: &str) -> Self {
            self.configuration.udp_trackers.as_mut().expect("UDP tracker configuration")[0]
                .network
                .external_ip = Some(external_ip.parse().expect("valid external IP address"));
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
