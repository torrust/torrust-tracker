use std::sync::Arc;

use tokio::task::JoinHandle;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_torrent_repository::event::receiver::Receiver;

use super::handler::handle_event;
use crate::statistics::repository::Repository;
use crate::torrent::repository::persisted::DatabaseDownloadsMetricRepository;
use crate::{CurrentClock, TRACKER_CORE_LOG_TARGET};

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    repository: &Arc<Repository>,
    db_torrent_repository: &Arc<DatabaseDownloadsMetricRepository>,
    persistent_torrent_completed_stat: bool,
) -> JoinHandle<()> {
    let stats_repository = repository.clone();
    let db_torrent_repository: Arc<DatabaseDownloadsMetricRepository> = db_torrent_repository.clone();

    tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Starting torrent repository event listener");

    tokio::spawn(async move {
        dispatch_events(
            receiver,
            stats_repository,
            db_torrent_repository,
            persistent_torrent_completed_stat,
        )
        .await;

        tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Torrent repository listener finished");
    })
}

async fn dispatch_events(
    mut receiver: Receiver,
    stats_repository: Arc<Repository>,
    db_torrent_repository: Arc<DatabaseDownloadsMetricRepository>,
    persistent_torrent_completed_stat: bool,
) {
    let shutdown_signal = tokio::signal::ctrl_c();

    tokio::pin!(shutdown_signal);

    loop {
        tokio::select! {
            biased;

            _ = &mut shutdown_signal => {
                tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Received Ctrl+C, shutting down torrent repository event listener");
                break;
            }

            result = receiver.recv() => {
                match result {
                    Ok(event) => handle_event(
                        event,
                        &stats_repository,
                        &db_torrent_repository,
                        persistent_torrent_completed_stat,
                        CurrentClock::now()).await,
                    Err(e) => {
                        match e {
                            RecvError::Closed => {
                                tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Torrent repository event receiver closed");
                                break;
                            }
                            RecvError::Lagged(n) => {
                                tracing::warn!(target: TRACKER_CORE_LOG_TARGET, "Torrent repository event receiver lagged by {} events", n);
                            }
                        }
                    }
                }
            }
        }
    }
}
