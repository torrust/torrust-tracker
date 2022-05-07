use std::sync::Arc;
use log::{info, warn};
use tokio::task::JoinHandle;
use crate::{Configuration};
use crate::jobs::{http_tracker, log_statistics, persistent_torrent_statistics, torrent_cleanup, tracker_api, udp_tracker};
use crate::tracker::tracker::TorrentTracker;

pub async fn setup(config: &Configuration, tracker: Arc<TorrentTracker>) -> Vec<JoinHandle<()>>{
    let mut jobs: Vec<JoinHandle<()>> = Vec::new();

    if tracker.is_private() {
        tracker.load_keys().await.expect("Could not retrieve keys.");
    }

    // todo: replace by realtime updates
    // Load persistent torrents
    if config.persistent_torrent_completed_stat && config.persistence_interval > 0 {
        info!("Loading persistent torrents into memory..");
        tracker.load_persistent_torrents().await.expect("Could not load persistent torrents.");
        info!("Persistent torrents loaded.");
        jobs.push(persistent_torrent_statistics::start_job(&config, tracker.clone()));
    }

    // Start the UDP blocks
    for udp_tracker_config in &config.udp_trackers {
        if !udp_tracker_config.enabled { continue; }

        if tracker.is_private() {
            warn!("Could not start UDP tracker on: {} while in {:?}. UDP is not safe for private trackers!", udp_tracker_config.bind_address, config.mode);
        } else {
            jobs.push(udp_tracker::start_job(&udp_tracker_config, tracker.clone()))
        }
    }

    // Start the HTTP blocks
    for http_tracker_config in &config.http_trackers {
        if !http_tracker_config.enabled { continue; }
        jobs.push(http_tracker::start_job(&http_tracker_config, tracker.clone()));
    }

    // Start HTTP API server
    if config.http_api.enabled {
        jobs.push(tracker_api::start_job(&config, tracker.clone()));
    }

    // Remove torrents without peers, every interval
    if config.inactive_peer_cleanup_interval > 0 {
        jobs.push(torrent_cleanup::start_job(&config, tracker.clone()));
    }

    // Log detailed torrent stats
    if let Some(log_interval) = config.log_interval {
        if log_interval > 0 {
            jobs.push(log_statistics::start_job(&config, tracker.clone()));
        }
    }

    jobs
}
