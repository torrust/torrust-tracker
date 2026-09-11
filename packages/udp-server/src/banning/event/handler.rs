use std::sync::Arc;

use tokio::sync::RwLock;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric_name;
use torrust_tracker_udp_core::services::banning::BanService;

use crate::event::{ErrorKind, Event};
use crate::statistics::UDP_TRACKER_SERVER_IPS_BANNED_TOTAL;
use crate::statistics::repository::Repository;

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

        let ips_banned_total = ban_service.get_banned_ips_total();
        drop(ban_service);
        update_metric_for_banned_ips_total(repository, ips_banned_total, now).await;
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

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use tokio::sync::RwLock;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_udp_core::event::ConnectionContext;
    use torrust_tracker_udp_core::services::banning::BanService;

    use super::handle_event;
    use crate::event::{ErrorKind, Event};
    use crate::statistics::repository::Repository;

    const EVENT_TIME: DurationSinceUnixEpoch = DurationSinceUnixEpoch::new(1_700_000_000, 0);

    struct BanningHandlerTestContext {
        ban_service: Arc<RwLock<BanService>>,
        stats_repository: Repository,
    }

    impl BanningHandlerTestContext {
        fn with_no_tracked_clients() -> Self {
            Self {
                ban_service: Arc::new(RwLock::new(BanService::new(1))),
                stats_repository: Repository::new(),
            }
        }

        async fn with_one_tracked_client(client_ip: IpAddr) -> Self {
            let context = Self::with_no_tracked_clients();
            context.ban_service.write().await.increase_counter(&client_ip);
            context
        }
    }

    fn sample_connection_context(client_ip: IpAddr) -> ConnectionContext {
        ConnectionContext::new(
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
            SocketAddr::new(client_ip, 8080),
            ServiceBinding::new(
                Protocol::UDP,
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
            )
            .expect("sample UDP service binding should be valid"),
        )
    }

    #[tokio::test]
    async fn should_record_the_connection_cookie_error_for_its_client_ip() {
        // Arrange
        let cookie_error_client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 2));
        let context = BanningHandlerTestContext::with_no_tracked_clients();
        let event = Event::UdpError {
            context: sample_connection_context(cookie_error_client_ip),
            kind: None,
            error: ErrorKind::ConnectionCookie("connection ID is invalid".to_string()),
        };

        // Act
        handle_event(event, &context.ban_service, &context.stats_repository, EVENT_TIME).await;

        // Assert
        assert_eq!(context.ban_service.read().await.get_count(&cookie_error_client_ip), Some(1));
    }

    #[tokio::test]
    async fn should_publish_the_distinct_client_ip_total_after_a_connection_cookie_error() {
        // Arrange
        let unrelated_client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1));
        let cookie_error_client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 2));
        let context = BanningHandlerTestContext::with_one_tracked_client(unrelated_client_ip).await;
        let expected_distinct_client_ip_total = 2;
        let event = Event::UdpError {
            context: sample_connection_context(cookie_error_client_ip),
            kind: None,
            error: ErrorKind::ConnectionCookie("connection ID is invalid".to_string()),
        };

        // Act
        handle_event(event, &context.ban_service, &context.stats_repository, EVENT_TIME).await;

        // Assert
        assert_eq!(
            context.stats_repository.get_stats().await.udp_banned_ips_total(),
            expected_distinct_client_ip_total
        );
    }
}
