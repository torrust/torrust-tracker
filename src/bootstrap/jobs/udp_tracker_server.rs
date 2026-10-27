use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time as _;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_events::shutdown::Completion;
use torrust_tracker_udp_core::UDP_TRACKER_LOG_TARGET;
use torrust_tracker_udp_core::services::banning::BanService;
use torrust_tracker_udp_server::banning::{PublicationOrder, reset_bans};
use torrust_tracker_udp_server::statistics::repository::Repository;

use crate::CurrentClock;
use crate::bootstrap::jobs::event_listener_drain_timeout;
use crate::container::AppContainer;

/// Returns an unspawned statistics listener for application-level supervision.
// issue: #2039
// The shared metrics listener filters aggregate updates by immutable listener
// policy. It must not control event publication, because banning consumes the
// same stream independently.
pub fn run_stats_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> impl Future<Output = Completion> + Send + 'static {
    let metrics_policy = app_container
        .udp_tracker_instance_containers
        .iter()
        .map(|(id, container)| (*id, container.udp_tracker_config.tracker_usage_statistics))
        .collect::<BTreeMap<_, _>>();
    torrust_tracker_udp_server::statistics::event::listener::run_event_listener_unspawned(
        app_container.udp_tracker_server_container.event_bus.receiver(),
        cancellation_token,
        app_container.udp_tracker_server_container.stats_repository.clone(),
        metrics_policy,
        event_listener_drain_timeout(config),
    )
}

/// Returns an unspawned banning listener for application-level supervision.
// issue: #2039
// Banning intentionally receives every UDP-server fact; it never applies the
// per-listener metrics policy used by `run_stats_event_listener`.
pub fn run_banning_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> impl Future<Output = Completion> + Send + 'static {
    torrust_tracker_udp_server::banning::event::listener::run_event_listener_unspawned(
        app_container.udp_tracker_server_container.event_bus.receiver(),
        cancellation_token,
        app_container.udp_tracker_core_services.ban_service.clone(),
        app_container
            .udp_tracker_server_container
            .banned_ips_publication_order
            .clone(),
        app_container.udp_tracker_server_container.stats_repository.clone(),
        event_listener_drain_timeout(config),
    )
}

#[must_use]
// issue: #1453
pub fn start_ban_cleanup_job(
    reset_interval_in_secs: u64,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> JoinHandle<()> {
    let ban_service = app_container.udp_tracker_core_services.ban_service.clone();
    let publication_order = app_container
        .udp_tracker_server_container
        .banned_ips_publication_order
        .clone();
    let stats_repository = app_container.udp_tracker_server_container.stats_repository.clone();

    tokio::spawn(run_ban_cleanup_job(
        ban_service,
        publication_order,
        stats_repository,
        reset_interval_in_secs,
        cancellation_token,
    ))
}

async fn run_ban_cleanup_job(
    ban_service: Arc<tokio::sync::RwLock<BanService>>,
    publication_order: Arc<PublicationOrder>,
    stats_repository: Arc<Repository>,
    reset_interval_in_secs: u64,
    cancellation_token: CancellationToken,
) {
    tracing::info!(
        target: UDP_TRACKER_LOG_TARGET,
        reset_interval_in_secs,
        "Starting UDP IP-ban cleanup job"
    );

    let mut cleaner_interval = interval(Duration::from_secs(reset_interval_in_secs));
    cleaner_interval.tick().await;

    loop {
        tokio::select! {
            () = cancellation_token.cancelled() => {
                tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Stopping UDP IP-ban cleanup job ...");
                break;
            }
            _ = cleaner_interval.tick() => {
                reset_bans(&ban_service, &publication_order, &stats_repository, CurrentClock::now()).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::sync::RwLock;
    use tokio::time::{sleep, timeout};
    use tokio_util::sync::CancellationToken;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_udp_core::event::ConnectionContext;
    use torrust_tracker_udp_core::services::banning::BanService;
    use torrust_tracker_udp_server::banning::PublicationOrder;
    use torrust_tracker_udp_server::banning::event::handler::handle_event;
    use torrust_tracker_udp_server::event::{ErrorKind, Event};
    use torrust_tracker_udp_server::statistics::metrics::Metrics;
    use torrust_tracker_udp_server::statistics::repository::Repository;

    use super::run_ban_cleanup_job;

    const EVENT_TIME: DurationSinceUnixEpoch = DurationSinceUnixEpoch::new(1_700_000_000, 0);

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
    async fn it_should_stop_the_ban_cleanup_job_when_cancelled() {
        let cancellation_token = CancellationToken::new();
        let ban_service = Arc::new(RwLock::new(BanService::new(10)));
        let job = tokio::spawn(run_ban_cleanup_job(
            ban_service,
            Arc::new(PublicationOrder::default()),
            Arc::new(Repository::new()),
            24 * 60 * 60,
            cancellation_token.clone(),
        ));

        cancellation_token.cancel();

        timeout(Duration::from_secs(1), job)
            .await
            .expect("the cleanup job should stop after cancellation")
            .expect("the cleanup job should not panic");
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_publish_the_new_banned_ips_total_when_it_resets_the_bans() {
        // Arrange
        let connection_id_error_limit = 1;
        let reset_interval_in_secs = 60;
        let banned_client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1));
        let ban_service = Arc::new(RwLock::new(BanService::new(connection_id_error_limit)));
        let publication_order = Arc::new(PublicationOrder::default());
        let stats_repository = Arc::new(Repository::new());
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
        let cancellation_token = CancellationToken::new();
        let job = tokio::spawn(run_ban_cleanup_job(
            ban_service,
            publication_order,
            stats_repository.clone(),
            reset_interval_in_secs,
            cancellation_token.clone(),
        ));

        // Act: the paused clock jumps to the first reset, one interval after the job starts.
        sleep(Duration::from_secs(reset_interval_in_secs) + Duration::from_millis(1)).await;

        // Assert
        assert_eq!(
            stats_repository.read_metrics(Metrics::udp_banned_ips_total).await,
            0,
            "published banned IPs total {reset_interval_in_secs} s after the cleanup job started, when \
             {banned_client_ip} had been banned and published as 1 before the first reset"
        );

        cancellation_token.cancel();
        job.await.expect("the cleanup job should not panic");
    }
}
