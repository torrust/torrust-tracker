use std::sync::Arc;
use log::info;
use tokio::task::JoinHandle;
use crate::{Configuration};
use crate::tracker::tracker::TorrentTracker;

pub fn start_job(config: &Configuration, tracker: Arc<TorrentTracker>) -> JoinHandle<()> {
    let weak_tracker = std::sync::Arc::downgrade(&tracker);
    let interval = config.log_interval.unwrap_or(60);

    tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(interval);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await;

        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("Stopping statistics logging job..");
                    break;
                }
                _ = interval.tick() => {
                    if let Some(tracker) = weak_tracker.upgrade() {
                        tracker.post_log().await;
                    } else {
                        break;
                    }
                }
            }
        }
    })
}
