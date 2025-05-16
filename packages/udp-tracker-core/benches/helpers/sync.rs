use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};

use bittorrent_udp_tracker_core::event::bus::EventBus;
use bittorrent_udp_tracker_core::event::sender::Broadcaster;
use bittorrent_udp_tracker_core::services::connect::ConnectService;
use torrust_tracker_events::bus::SenderStatus;
use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

use crate::helpers::utils::{sample_ipv4_remote_addr, sample_issue_time};

#[allow(clippy::unused_async)]
pub async fn connect_once(samples: u64) -> Duration {
    let client_socket_addr = sample_ipv4_remote_addr();
    let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
    let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

    let udp_core_broadcaster = Broadcaster::default();
    let event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, udp_core_broadcaster.clone()));

    let udp_core_stats_event_sender = event_bus.sender();
    let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));
    let start = Instant::now();

    for _ in 0..samples {
        let _response = connect_service.handle_connect(client_socket_addr, server_service_binding.clone(), sample_issue_time());
    }

    start.elapsed()
}
