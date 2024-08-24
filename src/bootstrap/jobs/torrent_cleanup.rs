//! Job that runs a task on intervals to clean up torrents.
//!
//! It removes inactive peers and (optionally) peerless torrents.
//!
//! **Inactive peers** are peers that have not been updated for more than `max_peer_timeout` seconds.
//! `max_peer_timeout` is a customizable core tracker option.
//!
//! If the core tracker configuration option `remove_peerless_torrents` is true, the cleanup job will also
//! remove **peerless torrents** which are torrents with an empty peer list.
//!
//! Refer to [`torrust-tracker-configuration documentation`](https://docs.rs/torrust-tracker-configuration) for more info about those options.

use std::sync::Arc;

use chrono::Utc;
use tokio::task::JoinHandle;
use torrust_tracker_configuration::Core;

use crate::core;

/// It starts a jobs for cleaning up the torrent data in the tracker.
///
/// The cleaning task is executed on an `inactive_peer_cleanup_interval`.
///
/// Refer to [`torrust-tracker-configuration documentation`](https://docs.rs/torrust-tracker-configuration) for more info about that option.
#[must_use]
pub fn start_job(config: &Core, tracker: &Arc<core::Tracker>) -> JoinHandle<()> {
    let weak_tracker = std::sync::Arc::downgrade(tracker);
    let interval = config.inactive_peer_cleanup_interval;

    tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(interval);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await;

        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Stopping torrent cleanup job..");
                    break;
                }
                _ = interval.tick() => {
                    if let Some(tracker) = weak_tracker.upgrade() {
                        let start_time = Utc::now().time();
                        tracing::info!("Cleaning up torrents..");
                        tracker.cleanup_torrents();
                        tracing::info!("Cleaned up torrents in: {}ms", (Utc::now().time() - start_time).num_milliseconds());
                    } else {
                        break;
                    }
                }
            }
        }
    })
}
