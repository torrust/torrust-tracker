use std::sync::Arc;

use chrono::Utc;
use log::info;
use tokio::task::JoinHandle;
use torrust_tracker_configuration::Configuration;

use crate::tracker;

#[must_use]
pub fn start_job(config: &Configuration, tracker: &Arc<tracker::Tracker>) -> JoinHandle<()> {
    let weak_tracker = std::sync::Arc::downgrade(tracker);
    let interval = config.inactive_peer_cleanup_interval;

    tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(interval);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await;

        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("Stopping torrent cleanup job..");
                    break;
                }
                _ = interval.tick() => {
                    if let Some(tracker) = weak_tracker.upgrade() {
                        let start_time = Utc::now().time();
                        info!("Cleaning up torrents..");
                        tracker.cleanup_torrents().await;
                        info!("Cleaned up torrents in: {}ms", (Utc::now().time() - start_time).num_milliseconds());
                    } else {
                        break;
                    }
                }
            }
        }
    })
}
