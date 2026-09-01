use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_swarm_coordination_registry::event::receiver::Receiver;

use super::handler::{handle_in_memory_event, handle_persistent_completed_statistics_event};
use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
use crate::statistics::repository::Repository;
use crate::{CurrentClock, TRACKER_CORE_LOG_TARGET};

#[must_use]
pub fn run_in_memory_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    repository: &Arc<Repository>,
) -> JoinHandle<()> {
    let stats_repository = repository.clone();
    tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Starting tracker core in-memory statistics event listener");

    tokio::spawn(async move {
        dispatch_in_memory_events(receiver, cancellation_token, stats_repository).await;

        tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Tracker core in-memory statistics event listener finished");
    })
}

#[must_use]
pub fn run_persistent_completed_statistics_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    repository: &Arc<Repository>,
) -> JoinHandle<()> {
    let db_downloads_metric_repository = db_downloads_metric_repository.clone();
    let stats_repository = repository.clone();
    tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Starting tracker core persistent completed statistics event listener");

    tokio::spawn(async move {
        dispatch_persistent_completed_statistics_events(
            receiver,
            cancellation_token,
            db_downloads_metric_repository,
            stats_repository,
        )
        .await;

        tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Tracker core persistent completed statistics event listener finished");
    })
}

async fn dispatch_in_memory_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    stats_repository: Arc<Repository>,
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
                    Ok(event) => handle_in_memory_event(event, &stats_repository, CurrentClock::now()).await,
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

async fn dispatch_persistent_completed_statistics_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    db_downloads_metric_repository: Arc<DatabaseDownloadsMetricRepository>,
    stats_repository: Arc<Repository>,
) {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Received cancellation request, shutting down tracker core persistent completed statistics event listener.");
                break;
            }

            result = receiver.recv() => {
                match result {
                    Ok(event) => handle_persistent_completed_statistics_event(event, &db_downloads_metric_repository, &stats_repository, CurrentClock::now()).await,
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
