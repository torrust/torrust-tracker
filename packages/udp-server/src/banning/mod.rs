//! UDP server banning: records connection ID errors in the shared
//! [`BanService`] and publishes the banned-IPs total as the
//! `udp_tracker_server_ips_banned_total` gauge.
use std::sync::Arc;

use tokio::sync::RwLock;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_tracker_udp_core::services::banning::BanService;

use crate::statistics::repository::Repository;

pub mod event;

/// Clears every ban and publishes the new banned-IPs total.
///
/// The periodic ban cleanup job must use this instead of calling
/// [`BanService::reset_bans`] directly, so the published gauge never keeps
/// reporting bans that the reset removed.
pub async fn reset_bans(ban_service: &Arc<RwLock<BanService>>, repository: &Repository, now: DurationSinceUnixEpoch) {
    let mut ban_service = ban_service.write().await;

    ban_service.reset_bans();

    // Publish while still holding the lock, so a concurrent connection ID
    // error cannot publish its total in between and leave the gauge out of
    // order.
    let ips_banned_total = ban_service.get_banned_ips_total();
    event::handler::update_metric_for_banned_ips_total(repository, ips_banned_total, now).await;
    drop(ban_service);
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

    use super::reset_bans;
    use crate::banning::event::handler::handle_event;
    use crate::event::{ErrorKind, Event};
    use crate::statistics::repository::Repository;

    const EVENT_TIME: DurationSinceUnixEpoch = DurationSinceUnixEpoch::new(1_700_000_000, 0);
    const RESET_TIME: DurationSinceUnixEpoch = DurationSinceUnixEpoch::new(1_700_086_400, 0);

    fn connection_cookie_error_from(client_ip: IpAddr) -> Event {
        Event::UdpError {
            context: ConnectionContext::new(
                ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
                SocketAddr::new(client_ip, 8080),
                ServiceBinding::new(
                    Protocol::UDP,
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                )
                .expect("sample UDP service binding should be valid"),
            ),
            kind: None,
            error: ErrorKind::ConnectionCookie("connection ID is invalid".to_string()),
        }
    }

    #[tokio::test]
    async fn it_should_publish_the_new_banned_ips_total_after_resetting_the_bans() {
        // Arrange
        let connection_id_error_limit = 1;
        let banned_client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1));
        let ban_service = Arc::new(RwLock::new(BanService::new(connection_id_error_limit)));
        let stats_repository = Repository::new();
        for _ in 0..=connection_id_error_limit {
            handle_event(
                connection_cookie_error_from(banned_client_ip),
                &ban_service,
                &stats_repository,
                EVENT_TIME,
            )
            .await;
        }

        // Act
        reset_bans(&ban_service, &stats_repository, RESET_TIME).await;

        // Assert
        assert_eq!(
            stats_repository.get_stats().await.udp_banned_ips_total(),
            0,
            "published banned IPs total after a reset, when {banned_client_ip} had been banned and published as 1"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn it_should_keep_the_ban_service_locked_until_it_has_published_the_reset_total() {
        // Arrange
        let ban_service = Arc::new(RwLock::new(BanService::new(1)));
        let stats_repository = Arc::new(Repository::new());
        let gauge_store_lock = stats_repository.stats.write().await;
        let reset = tokio::spawn({
            let ban_service = ban_service.clone();
            let stats_repository = stats_repository.clone();
            async move { reset_bans(&ban_service, &stats_repository, RESET_TIME).await }
        });

        // Act: let the reset run until it waits for the gauge store.
        for _ in 0..10 {
            tokio::task::yield_now().await;
        }

        // Assert
        assert!(
            ban_service.try_write().is_err(),
            "the reset released the ban service before publishing the new banned IPs total, so a concurrent \
             connection ID error could publish its total in between and leave the gauge out of order"
        );

        drop(gauge_store_lock);
        reset.await.expect("the reset should not panic");
    }
}
