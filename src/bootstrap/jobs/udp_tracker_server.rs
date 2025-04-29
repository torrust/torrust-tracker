use std::sync::Arc;

use tokio::task::JoinHandle;
use torrust_tracker_configuration::Configuration;

use crate::container::AppContainer;

pub fn start_event_listener(config: &Configuration, app_container: &Arc<AppContainer>) -> Option<JoinHandle<()>> {
    if config.core.tracker_usage_statistics {
        let job = torrust_udp_tracker_server::statistics::event::listener::run_event_listener(
            app_container.udp_tracker_server_container.event_bus.receiver(),
            &app_container.udp_tracker_server_container.stats_repository,
        );
        Some(job)
    } else {
        tracing::info!("UDP tracker server event listener job is disabled.");
        None
    }
}
