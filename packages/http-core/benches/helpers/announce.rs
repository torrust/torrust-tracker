use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_tracker_http_core::services::announce::AnnounceService;
use torrust_tracker_http_protocol::v1::requests::announce::Announce;
use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::ClientIpSources;
use torrust_tracker_primitives::AnnounceData;

use crate::helpers::util::{initialize_core_tracker_services, sample_announce_request_for_peer, sample_peer};

/// An announce service and request built once, outside the measured loop.
///
/// Every iteration announces the same peer to the same torrent, so after the
/// first iteration each announce updates the existing peer in a one-peer swarm
/// and returns an empty peer list. This is deliberate: every iteration does the
/// same work, so results from two runs can be compared. Announcing a new peer
/// on every iteration would grow the swarm over millions of iterations, and
/// the cost would drift with its size. The result is therefore a lower bound
/// on announce cost: it excludes inserting a new peer and building a non-empty
/// peer list.
///
/// The configuration is the ephemeral public tracker with usage statistics
/// enabled, so each announce also sends an HTTP statistics event to a listener
/// running on the benchmark's Tokio runtime.
pub struct AnnounceBenchmarkContext {
    announce_service: AnnounceService,
    announce_request: Announce,
    client_ip_sources: ClientIpSources,
    server_service_binding: ServiceBinding,
}

impl AnnounceBenchmarkContext {
    /// Must be awaited inside the Tokio runtime that runs the benchmark,
    /// because it spawns the statistics event listener on it.
    ///
    /// # Panics
    ///
    /// Panics if the fixed server address is not a valid HTTP service binding.
    pub async fn new() -> Self {
        let (core_tracker_services, core_http_tracker_services) = initialize_core_tracker_services().await;

        let (announce_request, client_ip_sources) = sample_announce_request_for_peer(sample_peer());

        let announce_service = AnnounceService::new(
            core_tracker_services.core_config,
            core_tracker_services.announce_handler,
            core_tracker_services.authentication_service,
            core_tracker_services.whitelist_authorization,
            core_http_tracker_services.http_stats_event_sender,
            core_http_tracker_services.configuration_instance_id,
        );

        let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);

        Self {
            announce_service,
            announce_request,
            client_ip_sources,
            server_service_binding: ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap(),
        }
    }

    /// # Panics
    ///
    /// Panics if the announce is rejected, which would mean the benchmark
    /// measures an error path instead of an announce.
    pub async fn announce_once(&self) -> AnnounceData {
        self.announce_service
            .handle_announce(
                &self.announce_request,
                &self.client_ip_sources,
                &self.server_service_binding,
                None,
            )
            .await
            .expect("the benchmark announce should be accepted")
    }
}
