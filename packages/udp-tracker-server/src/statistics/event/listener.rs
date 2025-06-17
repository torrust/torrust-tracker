use std::sync::Arc;

use bittorrent_udp_tracker_core::UDP_TRACKER_LOG_TARGET;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;

use super::handler::handle_event;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;
use crate::CurrentClock;

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    repository: &Arc<Repository>,
) -> JoinHandle<()> {
    let repository_clone = repository.clone();

    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting UDP tracker server event listener");

    tokio::spawn(async move {
        dispatch_events(receiver, cancellation_token, repository_clone).await;

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "UDP tracker server event listener finished");
    })
}

async fn dispatch_events(mut receiver: Receiver, cancellation_token: CancellationToken, stats_repository: Arc<Repository>) {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Received cancellation request, shutting down UDP tracker server event listener.");
                break;
            }

            result = receiver.recv() => {
                match result {
                    Ok(event) => handle_event(event, &stats_repository, CurrentClock::now()).await,
                    Err(e) => {
                        match e {
                            RecvError::Closed => {
                                tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Udp tracker server statistics receiver closed.");
                                break;
                            }
                            RecvError::Lagged(n) => {
                                tracing::warn!(target: UDP_TRACKER_LOG_TARGET, "Udp tracker server statistics receiver lagged by {} events.", n);
                            }
                        }
                    }
                }
            }
        }
    }
}
