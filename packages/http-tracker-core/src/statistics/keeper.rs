use std::sync::Arc;

use tokio::task::JoinHandle;

use super::event::listener::dispatch_events;
use super::repository::Repository;
use crate::event::sender::{self, Broadcaster};
use crate::HTTP_TRACKER_LOG_TARGET;

/// The service responsible for keeping tracker metrics (listening to statistics events and handle them).
///
/// It actively listen to new statistics events. When it receives a new event
/// it accordingly increases the counters.
pub struct Keeper {
    pub enable_sender: bool,
    pub broadcaster: Broadcaster,
    pub repository: Arc<Repository>,
}

impl Default for Keeper {
    fn default() -> Self {
        let enable_sender = true;
        let broadcaster = Broadcaster::default();
        let repository = Arc::new(Repository::new());

        Self::new(enable_sender, broadcaster, repository)
    }
}

impl Keeper {
    /// Creates a new instance of [`Keeper`].
    #[must_use]
    pub fn new(enable_sender: bool, broadcaster: Broadcaster, repository: Arc<Repository>) -> Self {
        Self {
            enable_sender,
            broadcaster,
            repository,
        }
    }

    #[must_use]
    pub fn sender(&self) -> Option<Box<dyn sender::Sender>> {
        if self.enable_sender {
            Some(Box::new(self.broadcaster.clone()))
        } else {
            None
        }
    }

    #[must_use]
    pub fn repository(&self) -> Arc<Repository> {
        self.repository.clone()
    }

    #[must_use]
    pub fn run_event_listener(&self) -> JoinHandle<()> {
        let stats_repository = self.repository.clone();
        let receiver = self.broadcaster.subscribe();

        tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "Starting HTTP tracker core event listener");

        tokio::spawn(async move {
            dispatch_events(receiver, stats_repository).await;

            tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "HTTP tracker core event listener finished");
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::statistics::keeper::Keeper;
    use crate::statistics::metrics::Metrics;

    #[tokio::test]
    async fn should_contain_the_tracker_statistics() {
        let stats_tracker = Keeper::default();

        let stats = stats_tracker.repository.get_stats().await;

        assert_eq!(stats.tcp4_announces_handled, Metrics::default().tcp4_announces_handled);
    }
}
