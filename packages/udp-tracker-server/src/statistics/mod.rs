pub mod event;
pub mod keeper;
pub mod metrics;
pub mod repository;
pub mod services;
pub mod setup;

use metrics::Metrics;
use torrust_tracker_metrics::metric::description::MetricDescription;
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_metrics::unit::Unit;

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    metrics.metric_collection.describe_counter(
        &MetricName::new("udp_tracker_server_requests_aborted_total"),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests aborted")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new("udp_tracker_server_requests_banned_total"),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests banned")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new("udp_tracker_server_requests_received_total"),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests received")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new("udp_tracker_server_requests_accepted_total"),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests accepted")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new("udp_tracker_server_responses_sent_total"),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP responses sent")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new("udp_tracker_server_errors_total"),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of errors processing UDP requests")),
    );

    metrics.metric_collection.describe_gauge(
        &MetricName::new("udp_tracker_server_performance_avg_connect_processing_time_ns"),
        Some(Unit::Nanoseconds),
        Some(MetricDescription::new(
            "Average time to process a UDP connect request in nanoseconds",
        )),
    );

    metrics.metric_collection.describe_gauge(
        &MetricName::new("udp_tracker_server_performance_avg_announce_processing_time_ns"),
        Some(Unit::Nanoseconds),
        Some(MetricDescription::new(
            "Average time to process a UDP announce request in nanoseconds",
        )),
    );

    metrics.metric_collection.describe_gauge(
        &MetricName::new("udp_tracker_server_performance_avg_scrape_processing_time_ns"),
        Some(Unit::Nanoseconds),
        Some(MetricDescription::new(
            "Average time to process a UDP scrape request in nanoseconds",
        )),
    );

    metrics
}
