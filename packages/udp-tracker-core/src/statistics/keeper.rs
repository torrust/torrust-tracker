use tokio::sync::broadcast::Receiver;

use super::event::listener::dispatch_events;
use super::repository::Repository;
use crate::event::Event;

/// The service responsible for keeping tracker metrics (listening to statistics events and handle them).
///
/// It actively listen to new statistics events. When it receives a new event
/// it accordingly increases the counters.
pub struct Keeper {
    pub repository: Repository,
}

impl Default for Keeper {
    fn default() -> Self {
        Self::new()
    }
}

impl Keeper {
    #[must_use]
    pub fn new() -> Self {
        Self {
            repository: Repository::new(),
        }
    }

    pub fn run_event_listener(&mut self, receiver: Receiver<Event>) {
        let stats_repository = self.repository.clone();

        tokio::spawn(async move { dispatch_events(receiver, stats_repository).await });
    }
}

#[cfg(test)]
mod tests {
    use crate::statistics::keeper::Keeper;
    use crate::statistics::metrics::Metrics;

    #[tokio::test]
    async fn should_contain_the_tracker_statistics() {
        let stats_tracker = Keeper::new();

        let stats = stats_tracker.repository.get_stats().await;

        assert_eq!(stats.udp4_announces_handled, Metrics::default().udp4_announces_handled);
    }
}
