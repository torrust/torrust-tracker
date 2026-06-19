use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_udp_core::UDP_TRACKER_LOG_TARGET;
use torrust_tracker_udp_core::services::banning::BanService;

use super::handler::handle_event;
use crate::CurrentClock;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    ban_service: &Arc<RwLock<BanService>>,
    repository: &Arc<Repository>,
) -> JoinHandle<()> {
    let ban_service_clone = ban_service.clone();
    let repository_clone = repository.clone();

    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting UDP tracker server event listener (banning)");

    tokio::spawn(async move {
        dispatch_events(receiver, cancellation_token, ban_service_clone, repository_clone).await;

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "UDP tracker server event listener (banning) finished");
    })
}

async fn dispatch_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    ban_service: Arc<RwLock<BanService>>,
    repository: Arc<Repository>,
) {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Received cancellation request, shutting down UDP tracker server event listener.");
                break;
            }

            result = receiver.recv() => {
                match result {
                    Ok(event) => handle_event(event, &ban_service, &repository, CurrentClock::now()).await,
                    Err(e) => {
                        match e {
                            RecvError::Closed => {
                                tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Udp tracker server receiver  (banning) closed.");
                                break;
                            }
                            RecvError::Lagged(n) => {
                                tracing::warn!(target: UDP_TRACKER_LOG_TARGET, "Udp tracker server receiver (banning) lagged by {} events.", n);
                            }
                        }
                    }
                }
            }
        }
    }
}
