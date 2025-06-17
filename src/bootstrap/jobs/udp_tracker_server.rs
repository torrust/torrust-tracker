use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::Configuration;

use crate::container::AppContainer;

pub fn start_stats_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<JoinHandle<()>> {
    if config.core.tracker_usage_statistics {
        let job = torrust_udp_tracker_server::statistics::event::listener::run_event_listener(
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
    torrust_udp_tracker_server::banning::event::listener::run_event_listener(
        app_container.udp_tracker_server_container.event_bus.receiver(),
        cancellation_token,
        &app_container.udp_tracker_core_services.ban_service,
        &app_container.udp_tracker_server_container.stats_repository,
    )
}
