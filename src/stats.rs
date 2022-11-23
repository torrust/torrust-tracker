use crate::tracker::statistics::{StatsRepository, StatsTracker, TrackerStatisticsEventSender};

pub fn setup_statistics(tracker_usage_statistics: bool) -> (Option<Box<dyn TrackerStatisticsEventSender>>, StatsRepository) {
    let mut stats_event_sender = None;

    let mut stats_tracker = StatsTracker::new();

    if tracker_usage_statistics {
        stats_event_sender = Some(stats_tracker.run_event_listener());
    }

    (stats_event_sender, stats_tracker.stats_repository)
}

#[cfg(test)]
mod test {
    use crate::stats::setup_statistics;

    #[tokio::test]
    async fn should_not_send_any_event_when_statistics_are_disabled() {
        let tracker_usage_statistics = false;

        let (stats_event_sender, _stats_repository) = setup_statistics(tracker_usage_statistics);

        assert!(stats_event_sender.is_none());
    }

    #[tokio::test]
    async fn should_send_events_when_statistics_are_enabled() {
        let tracker_usage_statistics = true;

        let (stats_event_sender, _stats_repository) = setup_statistics(tracker_usage_statistics);

        assert!(stats_event_sender.is_some());
    }
}
