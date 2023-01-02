use std::net::SocketAddr;
use std::sync::Arc;

use log::warn;
use tokio::task::JoinHandle;

use crate::config::Configuration;
use crate::jobs::{http_tracker, torrent_cleanup, tracker_api, tracker_apis, udp_tracker};
use crate::tracker;

/// # Panics
///
/// Will panic if the socket address for API can't be parsed.
pub async fn setup(config: &Configuration, tracker: Arc<tracker::Tracker>) -> Vec<JoinHandle<()>> {
    let mut jobs: Vec<JoinHandle<()>> = Vec::new();

    // Load peer keys
    if tracker.is_private() {
        tracker.load_keys().await.expect("Could not retrieve keys from database.");
    }

    // Load whitelisted torrents
    if tracker.is_whitelisted() {
        tracker
            .load_whitelist()
            .await
            .expect("Could not load whitelist from database.");
    }

    // Start the UDP blocks
    for udp_tracker_config in &config.udp_trackers {
        if !udp_tracker_config.enabled {
            continue;
        }

        if tracker.is_private() {
            warn!(
                "Could not start UDP tracker on: {} while in {:?}. UDP is not safe for private trackers!",
                udp_tracker_config.bind_address, config.mode
            );
        } else {
            jobs.push(udp_tracker::start_job(udp_tracker_config, tracker.clone()));
        }
    }

    // Start the HTTP blocks
    for http_tracker_config in &config.http_trackers {
        if !http_tracker_config.enabled {
            continue;
        }
        jobs.push(http_tracker::start_job(http_tracker_config, tracker.clone()));
    }

    // Start HTTP API server
    if config.http_api.enabled {
        jobs.push(tracker_api::start_job(&config.http_api, tracker.clone()).await);
    }

    // Start HTTP APIs server (multiple API versions)
    if config.http_api.enabled {
        // Temporarily running the new API in the 1313 port
        let bind_address = config.http_api.bind_address.clone();
        let mut bind_socket: SocketAddr = bind_address.parse().unwrap();
        bind_socket.set_port(1313);

        let mut http_apis_config = config.http_api.clone();
        http_apis_config.bind_address = bind_socket.to_string();

        jobs.push(tracker_apis::start_job(&http_apis_config, tracker.clone()).await);
    }

    // Remove torrents without peers, every interval
    if config.inactive_peer_cleanup_interval > 0 {
        jobs.push(torrent_cleanup::start_job(config, &tracker));
    }

    jobs
}
