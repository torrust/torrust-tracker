use std::sync::Arc;
use log::info;
use tokio::task::JoinHandle;
use crate::{Configuration};
use crate::tracker::tracker::TorrentTracker;

pub fn start_job(config: &Configuration, tracker: Arc<TorrentTracker>) -> JoinHandle<()> {
    let weak_tracker = std::sync::Arc::downgrade(&tracker);
    let interval = config.persistence_interval.unwrap_or(900);

    tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(interval);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await;

        // periodically save torrents to database
        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    // Save before shutting down
                    tracker.periodic_saving().await;
                    info!("Stopping periodic torrent saving job..");
                    break;
                }
                _ = interval.tick() => {
                    if let Some(tracker) = weak_tracker.upgrade() {
                        info!("Saving torrents to database...");
                        tracker.periodic_saving().await;
                        info!("Periodic saving done.");
                    } else {
                        // If tracker no longer exists, stop job
                        break;
                    }
                }
            }
        }
    })
}
