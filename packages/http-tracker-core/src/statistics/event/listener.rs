use std::sync::Arc;

use tokio::task::JoinHandle;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;

use super::handler::handle_event;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;
use crate::{CurrentClock, HTTP_TRACKER_LOG_TARGET};

#[must_use]
pub fn run_event_listener(receiver: Receiver, repository: &Arc<Repository>) -> JoinHandle<()> {
    let stats_repository = repository.clone();

    tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "Starting HTTP tracker core event listener");

    tokio::spawn(async move {
        dispatch_events(receiver, stats_repository).await;

        tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "HTTP tracker core event listener finished");
    })
}

async fn dispatch_events(mut receiver: Receiver, stats_repository: Arc<Repository>) {
    let shutdown_signal = tokio::signal::ctrl_c();

    tokio::pin!(shutdown_signal);

    loop {
        tokio::select! {
            biased;

            _ = &mut shutdown_signal => {
                tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "Received Ctrl+C, shutting down HTTP tracker core event listener.");
                break;
            }

            result = receiver.recv() => {
                match result {
                    Ok(event) => handle_event(event, &stats_repository, CurrentClock::now()).await,
                    Err(e) => {
                        match e {
                            RecvError::Closed => {
                                tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "Http core statistics receiver closed.");
                                break;
                            }
                            RecvError::Lagged(n) => {
                                tracing::warn!(target: HTTP_TRACKER_LOG_TARGET, "Http core statistics receiver lagged by {} events.", n);
                            }
                        }
                    }
                }
            }
        }
    }
}
