use tokio::sync::broadcast;
use torrust_tracker_clock::clock::Time;

use super::handler::handle_event;
use crate::event::Event;
use crate::statistics::repository::Repository;
use crate::CurrentClock;

pub async fn dispatch_events(mut receiver: broadcast::Receiver<Event>, stats_repository: Repository) {
    loop {
        match receiver.recv().await {
            Ok(event) => handle_event(event, &stats_repository, CurrentClock::now()).await,
            Err(e) => {
                tracing::error!("Error receiving http tracker core event: {:?}", e);
                break;
            }
        }
    }
}
