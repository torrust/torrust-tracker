//! Setup for the tracker statistics.
//!
//! The [`factory`] function builds the structs needed for handling the tracker metrics.
use std::sync::Arc;

use super::keeper::Keeper;
use super::repository::Repository;
use crate::event;
use crate::event::sender::Broadcaster;

/// It builds the structs needed for handling the tracker metrics.
///
/// It returns:
///
/// - An event [`Sender`](crate::event::sender::Sender) that allows you to send
///   events related to statistics.
/// - An statistics [`Repository`](crate::statistics::repository::Repository)
///   which is an in-memory repository for the tracker metrics.
///
/// When the input argument `tracker_usage_statistics`is false the setup does
/// not run the event listeners, consequently the statistics events are sent are
/// received but not dispatched to the handler.
#[must_use]
pub fn factory(tracker_usage_statistics: bool) -> (Option<Box<dyn event::sender::Sender>>, Arc<Repository>) {
    let keeper = keeper_factory(tracker_usage_statistics);

    if tracker_usage_statistics {
        // todo: this should be started like the other jobs during `app::start`
        // and keep the join handle in a list of jobs.
        let _unused = keeper.run_event_listener();
    }

    (keeper.sender(), keeper.repository())
}

#[must_use]
pub fn keeper_factory(tracker_usage_statistics: bool) -> Arc<Keeper> {
    let broadcaster = Broadcaster::default();
    let repository = Arc::new(Repository::new());
    Arc::new(Keeper::new(tracker_usage_statistics, broadcaster.clone(), repository.clone()))
}

#[cfg(test)]
mod test {
    use super::factory;

    #[tokio::test]
    async fn should_not_send_any_event_when_statistics_are_disabled() {
        let tracker_usage_statistics = false;

        let (stats_event_sender, _stats_repository) = factory(tracker_usage_statistics);

        assert!(stats_event_sender.is_none());
    }

    #[tokio::test]
    async fn should_send_events_when_statistics_are_enabled() {
        let tracker_usage_statistics = true;

        let (stats_event_sender, _stats_repository) = factory(tracker_usage_statistics);

        assert!(stats_event_sender.is_some());
    }
}
