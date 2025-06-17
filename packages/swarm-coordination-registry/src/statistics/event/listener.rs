use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;

use super::handler::handle_event;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;
use crate::{CurrentClock, SWARM_COORDINATION_REGISTRY_LOG_TARGET};

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    repository: &Arc<Repository>,
) -> JoinHandle<()> {
    let stats_repository = repository.clone();

    tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Starting swarm coordination registry event listener");

    tokio::spawn(async move {
        dispatch_events(receiver, cancellation_token, stats_repository).await;

        tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Swarm coordination registry listener finished");
    })
}

async fn dispatch_events(mut receiver: Receiver, cancellation_token: CancellationToken, stats_repository: Arc<Repository>) {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Received cancellation request, shutting down swarm coordination registry event listener.");
                break;
            }

            result = receiver.recv() => {
                match result {
                    Ok(event) => handle_event(event, &stats_repository, CurrentClock::now()).await,
                    Err(e) => {
                        match e {
                            RecvError::Closed => {
                                tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Swarm coordination registry event receiver closed.");
                                break;
                            }
                            RecvError::Lagged(n) => {
                                tracing::warn!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Swarm coordination registry event receiver lagged by {} events.", n);
                            }
                        }
                    }
                }
            }
        }
    }
}
