use std::sync::Arc;

use bittorrent_udp_tracker_core::services::banning::BanService;
use tokio::sync::RwLock;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::{ErrorKind, Event};

pub async fn handle_event(event: Event, ban_service: &Arc<RwLock<BanService>>, _now: DurationSinceUnixEpoch) {
    if let Event::UdpError {
        context,
        kind: _,
        error: ErrorKind::ConnectionCookie(_msg),
    } = event
    {
        let mut ban_service = ban_service.write().await;
        ban_service.increase_counter(&context.client_socket_addr().ip());
    }
}
