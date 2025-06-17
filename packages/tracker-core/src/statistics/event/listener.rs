use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_swarm_coordination_registry::event::receiver::Receiver;

use super::handler::handle_event;
use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
use crate::statistics::repository::Repository;
use crate::{CurrentClock, TRACKER_CORE_LOG_TARGET};

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    repository: &Arc<Repository>,
    db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    persistent_torrent_completed_stat: bool,
) -> JoinHandle<()> {
    let stats_repository = repository.clone();
    let db_downloads_metric_repository: Arc<DatabaseDownloadsMetricRepository> = db_downloads_metric_repository.clone();

    tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Starting tracker core event listener");

    tokio::spawn(async move {
        dispatch_events(
            receiver,
            cancellation_token,
            stats_repository,
            db_downloads_metric_repository,
            persistent_torrent_completed_stat,
        )
        .await;

        tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Tracker core listener finished");
    })
}

async fn dispatch_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    stats_repository: Arc<Repository>,
    db_downloads_metric_repository: Arc<DatabaseDownloadsMetricRepository>,
    persistent_torrent_completed_stat: bool,
) {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Received cancellation request, shutting down tracker core event listener.");
                break;
            }

            result = receiver.recv() => {
                match result {
                    Ok(event) => handle_event(
                        event,
                        &stats_repository,
                        &db_downloads_metric_repository,
                        persistent_torrent_completed_stat,
                        CurrentClock::now()).await,
                    Err(e) => {
                        match e {
                            RecvError::Closed => {
                                tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Tracker core event receiver closed");
                                break;
                            }
                            RecvError::Lagged(n) => {
                                tracing::warn!(target: TRACKER_CORE_LOG_TARGET, "Tracker core event receiver lagged by {} events", n);
                            }
                        }
                    }
                }
            }
        }
    }
}
