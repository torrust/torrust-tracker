use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::Configuration;

use crate::container::AppContainer;

pub fn start_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    cancellation_token: CancellationToken,
) -> Option<JoinHandle<()>> {
    if config.core.tracker_usage_statistics {
        let job = bittorrent_udp_tracker_core::statistics::event::listener::run_event_listener(
            app_container.udp_tracker_core_services.event_bus.receiver(),
            cancellation_token,
            &app_container.udp_tracker_core_services.stats_repository,
        );
        Some(job)
    } else {
        tracing::info!("UDP tracker core event listener job is disabled.");
        None
    }
}
