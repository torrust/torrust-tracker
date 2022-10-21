use crate::statistics::{StatsTracker, TrackerStatisticsEventSender};

pub fn setup_statistics(tracker_usage_statistics: bool) -> (StatsTracker, Option<Box<dyn TrackerStatisticsEventSender>>) {
    let mut stats_tracker = StatsTracker::new_inactive_instance();

    let mut stats_event_sender = None;

    if tracker_usage_statistics {
        stats_event_sender = Some(stats_tracker.run_worker());
    }

    (stats_tracker, stats_event_sender)
}

#[cfg(test)]
mod test {
    use crate::stats::setup_statistics;

    #[tokio::test]
    async fn should_not_send_any_event_when_statistics_are_disabled() {
        let tracker_usage_statistics = false;

        let (_stats_tracker, stats_event_sender) = setup_statistics(tracker_usage_statistics);

        assert!(stats_event_sender.is_none());
    }

    #[tokio::test]
    async fn should_send_events_when_statistics_are_enabled() {
        let tracker_usage_statistics = true;

        let (_stats_tracker, stats_event_sender) = setup_statistics(tracker_usage_statistics);

        assert!(stats_event_sender.is_some());
    }
}
