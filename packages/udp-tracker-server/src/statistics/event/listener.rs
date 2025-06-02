use std::sync::Arc;

use bittorrent_udp_tracker_core::services::banning::BanService;
use bittorrent_udp_tracker_core::UDP_TRACKER_LOG_TARGET;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;

use super::handler::handle_event;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;
use crate::CurrentClock;

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    repository: &Arc<Repository>,
    ban_service: &Arc<RwLock<BanService>>,
) -> JoinHandle<()> {
    let repository_clone = repository.clone();
    let ban_service_clone = ban_service.clone();

    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting UDP tracker server event listener");

    tokio::spawn(async move {
        dispatch_events(receiver, repository_clone, ban_service_clone).await;

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "UDP tracker server event listener finished");
    })
}

async fn dispatch_events(mut receiver: Receiver, stats_repository: Arc<Repository>, ban_service: Arc<RwLock<BanService>>) {
    let shutdown_signal = tokio::signal::ctrl_c();
    tokio::pin!(shutdown_signal);

    loop {
        tokio::select! {
            biased;

            _ = &mut shutdown_signal => {
                tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Received Ctrl+C, shutting down UDP tracker server event listener.");
                break;
            }

            result = receiver.recv() => {
                match result {
                    Ok(event) => handle_event(event, &stats_repository, &ban_service, CurrentClock::now()).await,
                    Err(e) => {
                        match e {
                            RecvError::Closed => {
                                tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Udp server statistics receiver closed.");
                                break;
                            }
                            RecvError::Lagged(n) => {
                                tracing::warn!(target: UDP_TRACKER_LOG_TARGET, "Udp server statistics receiver lagged by {} events.", n);
                            }
                        }
                    }
                }
            }
        }
    }
}
