use std::sync::Arc;

use bittorrent_udp_tracker_core::services::banning::BanService;
use tokio::sync::RwLock;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::{label_name, metric_name};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::{ConnectionContext, ErrorKind, UdpRequestKind};
use crate::statistics::repository::Repository;
use crate::statistics::UDP_TRACKER_SERVER_ERRORS_TOTAL;

pub async fn handle_event(
    context: ConnectionContext,
    kind: Option<UdpRequestKind>,
    error: ErrorKind,
    stats_repository: &Repository,
    ban_service: &Arc<RwLock<BanService>>,
    now: DurationSinceUnixEpoch,
) {
    if let ErrorKind::ConnectionCookie(_msg) = error {
        let mut ban_service = ban_service.write().await;
        ban_service.increase_counter(&context.client_socket_addr().ip());
    }

    update_global_fixed_metrics(&context, stats_repository).await;

    update_extendable_metrics(&context, kind, stats_repository, now).await;
}

async fn update_global_fixed_metrics(context: &ConnectionContext, stats_repository: &Repository) {
    match context.client_socket_addr().ip() {
        std::net::IpAddr::V4(_) => {
            stats_repository.increase_udp4_errors().await;
        }
        std::net::IpAddr::V6(_) => {
            stats_repository.increase_udp6_errors().await;
        }
    }
}

async fn update_extendable_metrics(
    context: &ConnectionContext,
    kind: Option<UdpRequestKind>,
    stats_repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    let mut label_set = LabelSet::from(context.clone());
    if let Some(kind) = kind {
        label_set.upsert(label_name!("request_kind"), kind.to_string().into());
    }
    match stats_repository
        .increase_counter(&metric_name!(UDP_TRACKER_SERVER_ERRORS_TOTAL), &label_set, now)
        .await
    {
        Ok(()) => {}
        Err(err) => tracing::error!("Failed to increase the counter: {}", err),
    };
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use bittorrent_udp_tracker_core::services::banning::BanService;
    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

    use crate::event::{ConnectionContext, Event};
    use crate::statistics::event::handler::error::ErrorKind;
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::repository::Repository;
    use crate::CurrentClock;

    #[tokio::test]
    async fn should_increase_the_udp4_errors_counter_when_it_receives_a_udp4_error_event() {
        let stats_repository = Repository::new();
        let ban_service = Arc::new(tokio::sync::RwLock::new(BanService::new(1)));

        handle_event(
            Event::UdpError {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                kind: None,
                error: ErrorKind::RequestParse("Invalid request format".to_string()),
            },
            &stats_repository,
            &ban_service,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_errors_handled, 1);
    }
}
