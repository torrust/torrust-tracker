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

pub fn start_stats_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<JoinHandle<()>> {
    if config.core.tracker_usage_statistics {
        let job = torrust_tracker_udp_server::statistics::event::listener::run_event_listener(
            app_container.udp_tracker_server_container.event_bus.receiver(),
            cancellation_token,
            &app_container.udp_tracker_server_container.stats_repository,
        );
        Some(job)
    } else {
        tracing::info!("UDP tracker server event listener job is disabled.");
        None
    }
}

#[must_use]
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
pub fn start_ban_cleanup_job(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<JoinHandle<()>> {
    if !should_start_ban_cleanup_job(config) {
        return None;
    }

    let ban_service = app_container.udp_tracker_core_services.ban_service.clone();
    let reset_interval_in_secs = UdpTrackerServer::DEFAULT_IP_BANS_RESET_INTERVAL_IN_SECS;

    Some(tokio::spawn(run_ban_cleanup_job(
        ban_service,
        reset_interval_in_secs,
        cancellation_token,
    )))
}

fn should_start_ban_cleanup_job(config: &Configuration) -> bool {
    !config.core.private
        && config
            .udp_trackers
            .as_ref()
            .is_some_and(|udp_trackers| !udp_trackers.is_empty())
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
    use torrust_tracker_configuration::{Configuration, UdpTracker};
    use torrust_tracker_udp_core::services::banning::BanService;

    use super::{run_ban_cleanup_job, should_start_ban_cleanup_job};

    #[test]
    fn it_should_not_start_the_ban_cleanup_job_without_udp_trackers() {
        assert!(!should_start_ban_cleanup_job(&Configuration::default()));
    }

    #[test]
    fn it_should_not_start_the_ban_cleanup_job_with_an_empty_udp_tracker_list() {
        let configuration = Configuration {
            udp_trackers: Some(Vec::new()),
            ..Configuration::default()
        };

        assert!(!should_start_ban_cleanup_job(&configuration));
    }

    #[test]
    fn it_should_not_start_the_ban_cleanup_job_for_a_private_tracker() {
        let configuration = Configuration {
            core: torrust_tracker_configuration::Core {
                private: true,
                ..Default::default()
            },
            udp_trackers: Some(vec![UdpTracker::default()]),
            ..Configuration::default()
        };

        assert!(!should_start_ban_cleanup_job(&configuration));
    }

    #[test]
    fn it_should_start_the_ban_cleanup_job_when_udp_trackers_are_configured() {
        let configuration = Configuration {
            udp_trackers: Some(vec![UdpTracker::default()]),
            ..Configuration::default()
        };

        assert!(should_start_ban_cleanup_job(&configuration));
    }

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
