use std::sync::Arc;

use bittorrent_udp_tracker_core::services::banning::BanService;
use tokio::sync::RwLock;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::{ErrorKind, Event};
use crate::statistics::repository::Repository;
use crate::statistics::UDP_TRACKER_SERVER_IPS_BANNED_TOTAL;

pub async fn handle_event(
    event: Event,
    ban_service: &Arc<RwLock<BanService>>,
    repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    if let Event::UdpError {
        context,
        kind: _,
        error: ErrorKind::ConnectionCookie(_msg),
    } = event
    {
        let mut ban_service = ban_service.write().await;

        ban_service.increase_counter(&context.client_socket_addr().ip());

        update_metric_for_banned_ips_total(repository, ban_service.get_banned_ips_total(), now).await;
    }
}

#[allow(clippy::cast_precision_loss)]
async fn update_metric_for_banned_ips_total(repository: &Repository, ips_banned_total: usize, now: DurationSinceUnixEpoch) {
    match repository
        .set_gauge(
            &metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL),
            &LabelSet::default(),
            ips_banned_total as f64,
            now,
        )
        .await
    {
        Ok(()) => {}
        Err(err) => tracing::error!("Failed to increase the counter: {}", err),
    }
}
