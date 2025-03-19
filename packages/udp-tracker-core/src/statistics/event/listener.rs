use tokio::sync::broadcast;

use super::handler::handle_event;
use super::Event;
use crate::statistics::repository::Repository;

pub async fn dispatch_events(mut receiver: broadcast::Receiver<Event>, stats_repository: Repository) {
    loop {
        match receiver.recv().await {
            Ok(event) => handle_event(event, &stats_repository).await,
            Err(e) => {
                tracing::error!("Error receiving udp tracker core event: {:?}", e);
                break;
            }
        }
    }
}
