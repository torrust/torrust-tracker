use tokio::sync::broadcast;
use torrust_tracker_clock::clock::Time;

use super::handler::handle_event;
use crate::event::Event;
use crate::statistics::repository::Repository;
use crate::{CurrentClock, UDP_TRACKER_LOG_TARGET};

pub async fn dispatch_events(mut receiver: broadcast::Receiver<Event>, stats_repository: Repository) {
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
