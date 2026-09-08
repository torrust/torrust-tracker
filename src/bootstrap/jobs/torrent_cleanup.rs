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
use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_core::torrent::manager::TorrentsManager;
use torrust_tracker_events::shutdown::Completion;
use tracing::instrument;

/// Returns an unspawned runner for cleaning up torrent data in the tracker.
///
/// The cleaning task is executed on an `inactive_peer_cleanup_interval`.
///
/// Refer to [`torrust-tracker-configuration documentation`](https://docs.rs/torrust-tracker-configuration) for more info about that option.
#[must_use]
#[instrument(skip(config, torrents_manager))]
pub fn run_job(
    config: Core,
    torrents_manager: Arc<TorrentsManager>,
    cancellation_token: CancellationToken,
) -> impl Future<Output = Completion> + Send + 'static {
    let weak_torrents_manager = Arc::downgrade(&torrents_manager);
    let interval = config.inactive_peer_cleanup_interval;
    let interval_in_secs = interval;

    async move {
        let interval = std::time::Duration::from_secs(interval);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await;

        loop {
            tokio::select! {
                () = cancellation_token.cancelled() => {
                    tracing::info!("Stopping torrent cleanup job ...");
                    return Completion::Cancelled;
                }
                _ = interval.tick() => {
                    match weak_torrents_manager.upgrade() { Some(torrents_manager) => {
                        let start_time = Utc::now().time();
                        tracing::info!("Cleaning up torrents (executed every {} secs) ...", interval_in_secs);
                        torrents_manager.cleanup_torrents().await;
                        tracing::info!("Cleaned up torrents in: {} ms", (Utc::now().time() - start_time).num_milliseconds());
                    } _ => {
                        return Completion::Completed;
                    }}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::time::timeout;
    use tokio_util::sync::CancellationToken;
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_events::shutdown::Completion;

    use super::run_job;
    use crate::container::AppContainer;

    #[tokio::test]
    async fn it_should_return_cancelled_when_the_token_is_cancelled() {
        // Arrange
        let configuration = Configuration::default();
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let cancellation_token = CancellationToken::new();
        let runner = run_job(
            Core {
                inactive_peer_cleanup_interval: 24 * 60 * 60,
                ..Core::default()
            },
            app_container.tracker_core_container.torrents_manager.clone(),
            cancellation_token.clone(),
        );

        // Act
        cancellation_token.cancel();
        let completion = timeout(Duration::from_secs(1), runner)
            .await
            .expect("the cleanup runner should stop after cancellation");

        // Assert
        assert_eq!(completion, Completion::Cancelled);
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_return_completed_when_the_torrents_manager_is_dropped() {
        // Arrange
        let runner = {
            let configuration = Configuration::default();
            let app_container = Arc::new(
                AppContainer::initialize(&configuration)
                    .await
                    .expect("composition should succeed"),
            );

            run_job(
                Core {
                    inactive_peer_cleanup_interval: 1,
                    ..Core::default()
                },
                app_container.tracker_core_container.torrents_manager.clone(),
                CancellationToken::new(),
            )
        };
        let runner = tokio::spawn(runner);
        tokio::task::yield_now().await;

        // Act
        tokio::time::advance(Duration::from_secs(1)).await;
        let completion = timeout(Duration::from_secs(1), runner)
            .await
            .expect("the cleanup runner should stop after the manager is dropped")
            .expect("the cleanup runner should not panic");

        // Assert
        assert_eq!(completion, Completion::Completed);
    }
}
