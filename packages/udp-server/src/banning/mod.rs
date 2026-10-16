//! UDP server banning: records connection ID errors in the shared
//! [`BanService`] and publishes the banned-IPs total as the
//! `udp_tracker_server_ips_banned_total` gauge.
use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, RwLock};
use torrust_clock::DurationSinceUnixEpoch;
use torrust_tracker_udp_core::services::banning::BanService;

use crate::statistics::repository::Repository;

pub mod event;

/// Orders the publications of the banned-IPs total.
///
/// Both publishers, the banning event handler and [`reset_bans`], change the
/// bans and then publish the new total. Each holds this lock across both
/// steps, so publications reach the gauge in the order the bans changed. The
/// [`BanService`] lock, which the request path reads for every datagram, is
/// not needed for that and is released before the gauge write waits for the
/// statistics store.
#[derive(Debug, Default)]
pub struct PublicationOrder(Mutex<()>);

impl PublicationOrder {
    pub(crate) async fn lock(&self) -> MutexGuard<'_, ()> {
        self.0.lock().await
    }

    /// Whether a publisher currently holds the publication order.
    #[cfg(test)]
    pub(crate) fn is_held(&self) -> bool {
        self.0.try_lock().is_err()
    }
}

/// Clears every ban and publishes the new banned-IPs total.
///
/// The periodic ban cleanup job must use this instead of calling
/// [`BanService::reset_bans`] directly, so the published gauge never keeps
/// reporting bans that the reset removed.
pub async fn reset_bans(
    ban_service: &Arc<RwLock<BanService>>,
    publication_order: &PublicationOrder,
    repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    // Holding the publication order, not the ban service lock, until the gauge
    // is written keeps publications in the order the bans changed without
    // making the request path's ban check wait for the gauge store.
    let _publication = publication_order.lock().await;

    let ips_banned_total = {
        let mut ban_service = ban_service.write().await;
        ban_service.reset_bans();
        ban_service.get_banned_ips_total()
    };

    event::handler::update_metric_for_banned_ips_total(repository, ips_banned_total, now).await;
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use tokio::sync::RwLock;
    use tokio::task::JoinHandle;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_udp_core::event::ConnectionContext;
    use torrust_tracker_udp_core::services::banning::BanService;

    use super::{PublicationOrder, reset_bans};
    use crate::banning::event::handler::handle_event;
    use crate::event::{ErrorKind, Event};
    use crate::statistics::metrics::Metrics;
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
        let publication_order = PublicationOrder::default();
        let stats_repository = Repository::new();
        for _ in 0..=connection_id_error_limit {
            handle_event(
                connection_cookie_error_from(banned_client_ip),
                &ban_service,
                &publication_order,
                &stats_repository,
                EVENT_TIME,
            )
            .await;
        }

        // Act
        reset_bans(&ban_service, &publication_order, &stats_repository, RESET_TIME).await;

        // Assert
        assert_eq!(
            stats_repository.read_metrics(Metrics::udp_banned_ips_total).await,
            0,
            "published banned IPs total after a reset, when {banned_client_ip} had been banned and published as 1"
        );
    }

    fn spawn_reset(
        ban_service: &Arc<RwLock<BanService>>,
        publication_order: &Arc<PublicationOrder>,
        stats_repository: &Arc<Repository>,
    ) -> JoinHandle<()> {
        let ban_service = ban_service.clone();
        let publication_order = publication_order.clone();
        let stats_repository = stats_repository.clone();
        tokio::spawn(async move { reset_bans(&ban_service, &publication_order, &stats_repository, RESET_TIME).await })
    }

    /// Lets the spawned tasks of a current-thread runtime run until they wait.
    async fn let_spawned_tasks_run() {
        for _ in 0..10 {
            tokio::task::yield_now().await;
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn it_should_not_hold_the_ban_service_lock_while_waiting_to_publish_the_reset_total() {
        // Arrange
        let ban_service = Arc::new(RwLock::new(BanService::new(1)));
        let publication_order = Arc::new(PublicationOrder::default());
        let stats_repository = Arc::new(Repository::new());
        let gauge_store_lock = stats_repository.write_lock_for_test().await;
        let reset = spawn_reset(&ban_service, &publication_order, &stats_repository);

        // Act: let the reset run until it waits for the gauge store.
        let_spawned_tasks_run().await;

        // Assert
        assert!(
            ban_service.try_write().is_ok(),
            "the reset held the ban service lock while waiting for the gauge store, so the request path's ban \
             check would wait for the statistics store"
        );

        drop(gauge_store_lock);
        reset.await.expect("the reset should not panic");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn it_should_hold_the_publication_order_until_it_has_published_the_reset_total() {
        // Arrange
        let ban_service = Arc::new(RwLock::new(BanService::new(1)));
        let publication_order = Arc::new(PublicationOrder::default());
        let stats_repository = Arc::new(Repository::new());
        let gauge_store_lock = stats_repository.write_lock_for_test().await;
        let reset = spawn_reset(&ban_service, &publication_order, &stats_repository);

        // Act: let the reset run until it waits for the gauge store.
        let_spawned_tasks_run().await;

        // Assert
        assert!(
            publication_order.is_held(),
            "the reset released the publication order before publishing the new banned IPs total, so a \
             concurrent connection ID error could publish its total in between and leave the gauge out of order"
        );

        drop(gauge_store_lock);
        reset.await.expect("the reset should not panic");
    }
}
