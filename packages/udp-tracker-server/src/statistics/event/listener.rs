use std::sync::Arc;

use bittorrent_udp_tracker_core::UDP_TRACKER_LOG_TARGET;
use tokio::task::JoinHandle;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;

use super::handler::handle_event;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;
use crate::CurrentClock;

#[must_use]
pub fn run_event_listener(receiver: Receiver, repository: &Arc<Repository>) -> JoinHandle<()> {
    let stats_repository = repository.clone();

    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting UDP tracker server event listener");

    tokio::spawn(async move {
        dispatch_events(receiver, stats_repository).await;

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "DP tracker server event listener finished");
    })
}

async fn dispatch_events(mut receiver: Receiver, stats_repository: Arc<Repository>) {
    loop {
        match receiver.recv().await {
            Ok(event) => handle_event(event, &stats_repository, CurrentClock::now()).await,
            Err(e) => {
                match e {
                    RecvError::Closed => {
                        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Udp server statistics receiver closed.");
                        break;
                    }
                    RecvError::Lagged(n) => {
                        // From now on, metrics will be imprecise
                        tracing::warn!(target: UDP_TRACKER_LOG_TARGET, "Udp server statistics receiver lagged by {} events.", n);
                    }
                }
            }
        }
    }
}
