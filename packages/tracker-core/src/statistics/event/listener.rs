use tokio::task::JoinHandle;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_torrent_repository::event::receiver::Receiver;

use super::handler::handle_event;
use crate::{CurrentClock, TRACKER_CORE_LOG_TARGET};

#[must_use]
pub fn run_event_listener(receiver: Receiver) -> JoinHandle<()> {
    tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Starting torrent repository event listener");

    tokio::spawn(async move {
        dispatch_events(receiver).await;

        tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Torrent repository listener finished");
    })
}

async fn dispatch_events(mut receiver: Receiver) {
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
                    Ok(event) => handle_event(event, CurrentClock::now()).await,
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
