use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, Instant};

use bittorrent_http_tracker_core::services::announce::AnnounceService;
use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

use crate::helpers::util::{initialize_core_tracker_services, sample_announce_request_for_peer, sample_peer};

#[must_use]
pub async fn return_announce_data_once(samples: u64) -> Duration {
    let (core_tracker_services, core_http_tracker_services) = initialize_core_tracker_services();

    let peer = sample_peer();

    let (announce_request, client_ip_sources) = sample_announce_request_for_peer(peer);

    let announce_service = AnnounceService::new(
        core_tracker_services.core_config.clone(),
        core_tracker_services.announce_handler.clone(),
        core_tracker_services.authentication_service.clone(),
        core_tracker_services.whitelist_authorization.clone(),
        core_http_tracker_services.http_stats_event_sender.clone(),
    );

    let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
    let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

    let start = Instant::now();

    for _ in 0..samples {
        let _announce_data = announce_service
            .handle_announce(&announce_request, &client_ip_sources, &server_service_binding, None)
            .await
            .unwrap();
    }

    start.elapsed()
}
