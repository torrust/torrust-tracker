use std::sync::Arc;

use tokio::task::JoinHandle;
use torrust_tracker_configuration::Configuration;

use crate::container::AppContainer;

pub fn start_event_listener(config: &Configuration, app_container: &Arc<AppContainer>) -> Option<JoinHandle<()>> {
    if config.core.tracker_usage_statistics {
        let job = bittorrent_http_tracker_core::statistics::event::listener::run_event_listener(
            app_container.http_tracker_core_services.event_bus.receiver(),
            &app_container.http_tracker_core_services.stats_repository,
        );

        Some(job)
    } else {
        tracing::info!("HTTP tracker core event listener job is disabled.");
        None
    }
}
