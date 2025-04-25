//! Setup for the tracker statistics.
//!
//! The [`factory`] function builds the structs needed for handling the tracker metrics.
use std::sync::Arc;

use super::keeper::Keeper;
use super::repository::Repository;
use crate::event::sender::Broadcaster;

#[must_use]
pub fn factory(tracker_usage_statistics: bool) -> (Arc<Keeper>, Arc<Repository>) {
    keeper_factory(tracker_usage_statistics)
}

#[must_use]
pub fn keeper_factory(tracker_usage_statistics: bool) -> (Arc<Keeper>, Arc<Repository>) {
    let broadcaster = Broadcaster::default();
    let repository = Arc::new(Repository::new());
    let keeper = Arc::new(Keeper::new(tracker_usage_statistics, broadcaster.clone()));

    (keeper, repository)
}

#[cfg(test)]
mod test {
    use super::factory;
    use crate::statistics::event::listener::run_event_listener;

    #[tokio::test]
    async fn should_not_send_any_event_when_statistics_are_disabled() {
        let tracker_usage_statistics = false;

        // HTTP core stats
        let (http_stats_keeper, http_stats_repository) = factory(tracker_usage_statistics);
        let http_stats_event_sender = http_stats_keeper.sender();

        if tracker_usage_statistics {
            let _unused = run_event_listener(http_stats_keeper.receiver(), &http_stats_repository);
        }

        assert!(http_stats_event_sender.is_none());
    }

    #[tokio::test]
    async fn should_send_events_when_statistics_are_enabled() {
        let tracker_usage_statistics = true;

        // HTTP core stats
        let (http_stats_keeper, _http_stats_repository) = factory(tracker_usage_statistics);
        let http_stats_event_sender = http_stats_keeper.sender();

        assert!(http_stats_event_sender.is_some());
    }
}
