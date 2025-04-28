use std::sync::Arc;

use tokio::sync::broadcast::{self};
use tokio::task::JoinHandle;
use torrust_tracker_clock::clock::Time;

use super::handler::handle_event;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;
use crate::{CurrentClock, UDP_TRACKER_LOG_TARGET};

#[must_use]
pub fn run_event_listener(receiver: Receiver, repository: &Arc<Repository>) -> JoinHandle<()> {
    let stats_repository = repository.clone();

    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting UDP tracker core event listener");

    tokio::spawn(async move {
        dispatch_events(receiver, stats_repository).await;

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "UDP tracker core event listener finished");
    })
}

async fn dispatch_events(mut receiver: Receiver, stats_repository: Arc<Repository>) {
    loop {
        match receiver.recv().await {
            Ok(event) => handle_event(event, &stats_repository, CurrentClock::now()).await,
            Err(e) => {
                match e {
                    broadcast::error::RecvError::Closed => {
                        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Udp core statistics receiver closed.");
                        break;
                    }
                    broadcast::error::RecvError::Lagged(n) => {
                        // From now on, metrics will be imprecise
                        tracing::warn!(target: UDP_TRACKER_LOG_TARGET, "Udp core statistics receiver lagged by {} events.", n);
                    }
                }
            }
        }
    }
}
