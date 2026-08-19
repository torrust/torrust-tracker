use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::Configuration;
use torrust_tracker_configuration::v3_0_0::udp_tracker_server::UdpTrackerServer;
use torrust_tracker_udp_core::UDP_TRACKER_LOG_TARGET;
use torrust_tracker_udp_core::services::banning::BanService;

use crate::container::AppContainer;

#[must_use]
// issue: #2039
// The shared metrics listener filters aggregate updates by immutable listener
// policy. It must not control event publication, because banning consumes the
// same stream independently.
pub fn start_stats_event_listener(
    _config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<JoinHandle<()>> {
    let metrics_policy = app_container
        .udp_tracker_instance_containers
        .iter()
        .map(|(id, container)| (*id, container.udp_tracker_config.tracker_usage_statistics))
        .collect::<BTreeMap<_, _>>();
    let job = torrust_tracker_udp_server::statistics::event::listener::run_event_listener(
        app_container.udp_tracker_server_container.event_bus.receiver(),
        cancellation_token,
        &app_container.udp_tracker_server_container.stats_repository,
        metrics_policy,
    );
    Some(job)
}

#[must_use]
// issue: #2039
// Banning intentionally receives every UDP-server fact; it never applies the
// per-listener metrics policy used by `start_stats_event_listener`.
pub fn start_banning_event_listener(app_container: &Arc<AppContainer>, cancellation_token: CancellationToken) -> JoinHandle<()> {
    torrust_tracker_udp_server::banning::event::listener::run_event_listener(
        app_container.udp_tracker_server_container.event_bus.receiver(),
        cancellation_token,
        &app_container.udp_tracker_core_services.ban_service,
        &app_container.udp_tracker_server_container.stats_repository,
    )
}

#[must_use]
// issue: #1453
pub fn start_ban_cleanup_job(app_container: &Arc<AppContainer>, cancellation_token: CancellationToken) -> JoinHandle<()> {
    let ban_service = app_container.udp_tracker_core_services.ban_service.clone();
    let reset_interval_in_secs = UdpTrackerServer::DEFAULT_IP_BANS_RESET_INTERVAL_IN_SECS;

    tokio::spawn(run_ban_cleanup_job(ban_service, reset_interval_in_secs, cancellation_token))
}

async fn run_ban_cleanup_job(
    ban_service: Arc<tokio::sync::RwLock<BanService>>,
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
                ban_service.write().await.reset_bans();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::sync::RwLock;
    use tokio::time::timeout;
    use tokio_util::sync::CancellationToken;
    use torrust_tracker_udp_core::services::banning::BanService;

    use super::run_ban_cleanup_job;

    #[tokio::test]
    async fn it_should_stop_the_ban_cleanup_job_when_cancelled() {
        let cancellation_token = CancellationToken::new();
        let ban_service = Arc::new(RwLock::new(BanService::new(10)));
        let job = tokio::spawn(run_ban_cleanup_job(ban_service, 24 * 60 * 60, cancellation_token.clone()));

        cancellation_token.cancel();

        timeout(Duration::from_secs(1), job)
            .await
            .expect("the cleanup job should stop after cancellation")
            .expect("the cleanup job should not panic");
    }
}
