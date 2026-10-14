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

        // Publish while still holding the lock, so a concurrent reset cannot
        // publish its total in between and leave the gauge out of order.
        let ips_banned_total = ban_service.get_banned_ips_total();
        update_metric_for_banned_ips_total(repository, ips_banned_total, now).await;
        drop(ban_service);
    }
}

#[allow(
    clippy::cast_precision_loss,
    reason = "temporary: #2244 reviews metric aggregate conversion boundaries"
)]
pub(crate) async fn update_metric_for_banned_ips_total(
    repository: &Repository,
    ips_banned_total: usize,
    now: DurationSinceUnixEpoch,
) {
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
        Err(err) => tracing::error!("Failed to set the banned IPs total gauge: {}", err),
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
    const CONNECTION_ID_ERROR_LIMIT: u32 = 1;

    struct BanningHandlerTestContext {
        ban_service: Arc<RwLock<BanService>>,
        stats_repository: Repository,
    }

    impl BanningHandlerTestContext {
        fn with_no_tracked_clients() -> Self {
            Self {
                ban_service: Arc::new(RwLock::new(BanService::new(CONNECTION_ID_ERROR_LIMIT))),
                stats_repository: Repository::new(),
            }
        }

        async fn with_client_errors(client_errors: &[(IpAddr, u32)]) -> Self {
            let context = Self::with_no_tracked_clients();
            let mut ban_service = context.ban_service.write().await;
            for (client_ip, errors) in client_errors {
                for _ in 0..*errors {
                    ban_service.increase_counter(client_ip);
                }
            }
            drop(ban_service);
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
    async fn it_should_record_the_connection_cookie_error_for_its_client_ip() {
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
    async fn it_should_publish_the_banned_ip_total_after_a_connection_cookie_error() {
        // Arrange
        let banned_client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1));
        let cookie_error_client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 2));
        let client_at_the_limit_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 3));
        let context = BanningHandlerTestContext::with_client_errors(&[
            (banned_client_ip, CONNECTION_ID_ERROR_LIMIT + 1),
            (cookie_error_client_ip, CONNECTION_ID_ERROR_LIMIT),
            (client_at_the_limit_ip, CONNECTION_ID_ERROR_LIMIT),
        ])
        .await;
        let expected_banned_ip_total = 2;
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
            expected_banned_ip_total,
            "published banned IPs total after a cookie error takes {cookie_error_client_ip} over the limit of \
             {CONNECTION_ID_ERROR_LIMIT} while {banned_client_ip} is already banned and \
             {client_at_the_limit_ip} is at the limit"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn it_should_keep_the_ban_service_locked_until_it_has_published_the_banned_ip_total() {
        // Arrange
        let cookie_error_client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 2));
        let context = Arc::new(BanningHandlerTestContext::with_no_tracked_clients());
        let gauge_store_lock = context.stats_repository.stats.write().await;
        let handler = tokio::spawn({
            let context = context.clone();
            async move {
                let event = Event::UdpError {
                    context: sample_connection_context(cookie_error_client_ip),
                    kind: None,
                    error: ErrorKind::ConnectionCookie("connection ID is invalid".to_string()),
                };
                handle_event(event, &context.ban_service, &context.stats_repository, EVENT_TIME).await;
            }
        });

        // Act: let the handler run until it waits for the gauge store.
        for _ in 0..10 {
            tokio::task::yield_now().await;
        }

        // Assert
        assert!(
            context.ban_service.try_write().is_err(),
            "the handler released the ban service before publishing the banned IPs total, so a concurrent \
             reset could publish its total in between and leave the gauge out of order"
        );

        drop(gauge_store_lock);
        handler.await.expect("the handler should not panic");
    }
}
