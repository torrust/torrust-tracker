//! Setup for the tracker statistics.
//!
//! The [`factory`] function builds the structs needed for handling the tracker metrics.
use tokio::sync::broadcast;

use super::event::sender::ChannelSender;
use crate::statistics;

const CHANNEL_CAPACITY: usize = 1024;

/// It builds the structs needed for handling the tracker metrics.
///
/// It returns:
///
/// - An statistics event [`Sender`](crate::statistics::event::sender::Sender) that allows you to send events related to statistics.
/// - An statistics [`Repository`](crate::statistics::repository::Repository) which is an in-memory repository for the tracker metrics.
///
/// When the input argument `tracker_usage_statistics`is false the setup does not run the event listeners, consequently the statistics
/// events are sent are received but not dispatched to the handler.
#[must_use]
pub fn factory(
    tracker_usage_statistics: bool,
) -> (
    Option<Box<dyn statistics::event::sender::Sender>>,
    statistics::repository::Repository,
) {
    let mut stats_event_sender: Option<Box<dyn statistics::event::sender::Sender>> = None;

    let mut keeper = statistics::keeper::Keeper::new();

    if tracker_usage_statistics {
        let (sender, _) = broadcast::channel(CHANNEL_CAPACITY);

        let receiver = sender.subscribe();

        stats_event_sender = Some(Box::new(ChannelSender { sender }));

        keeper.run_event_listener(receiver);
    }

    (stats_event_sender, keeper.repository)
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
