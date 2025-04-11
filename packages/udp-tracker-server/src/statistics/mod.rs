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

const UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL: &str = "udp_tracker_server_requests_aborted_total";
const UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL: &str = "udp_tracker_server_requests_banned_total";
const UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL: &str = "udp_tracker_server_requests_received_total";
const UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL: &str = "udp_tracker_server_requests_accepted_total";
const UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL: &str = "udp_tracker_server_responses_sent_total";
const UDP_TRACKER_SERVER_ERRORS_TOTAL: &str = "udp_tracker_server_errors_total";
const UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS: &str = "udp_tracker_server_performance_avg_processing_time_ns";

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    metrics.metric_collection.describe_counter(
        &MetricName::new(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests aborted")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new(UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests banned")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests received")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests accepted")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new(UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP responses sent")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new(UDP_TRACKER_SERVER_ERRORS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of errors processing UDP requests")),
    );

    metrics.metric_collection.describe_gauge(
        &MetricName::new(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
        Some(Unit::Nanoseconds),
        Some(MetricDescription::new(
            "Average time to process a UDP connect request in nanoseconds",
        )),
    );

    metrics
}
