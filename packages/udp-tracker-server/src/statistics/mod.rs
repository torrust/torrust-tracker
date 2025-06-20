pub mod event;
pub mod metrics;
pub mod repository;
pub mod services;

use metrics::Metrics;
use torrust_tracker_metrics::metric::description::MetricDescription;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_metrics::unit::Unit;

pub const UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL: &str = "udp_tracker_server_requests_aborted_total";
pub const UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL: &str = "udp_tracker_server_requests_banned_total";
pub const UDP_TRACKER_SERVER_IPS_BANNED_TOTAL: &str = "udp_tracker_server_ips_banned_total";
pub const UDP_TRACKER_SERVER_CONNECTION_ID_ERRORS_TOTAL: &str = "udp_tracker_server_connection_id_errors_total";
pub const UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL: &str = "udp_tracker_server_requests_received_total";
pub const UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL: &str = "udp_tracker_server_requests_accepted_total";
pub const UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL: &str = "udp_tracker_server_responses_sent_total";
pub const UDP_TRACKER_SERVER_ERRORS_TOTAL: &str = "udp_tracker_server_errors_total";
pub const UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS: &str = "udp_tracker_server_performance_avg_processing_time_ns";
pub const UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSED_REQUESTS_TOTAL: &str =
    "udp_tracker_server_performance_avg_processed_requests_total";

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    metrics.metric_collection.describe_counter(
        &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests aborted")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests banned")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of IPs banned from UDP requests")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(UDP_TRACKER_SERVER_CONNECTION_ID_ERRORS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of requests with connection ID errors")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests received")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests accepted")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP responses sent")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(UDP_TRACKER_SERVER_ERRORS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of errors processing UDP requests")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
        Some(Unit::Nanoseconds),
        Some(MetricDescription::new("Average time to process a UDP request in nanoseconds")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSED_REQUESTS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new(
            "Total number of UDP requests processed for the average performance metrics",
        )),
    );

    metrics
}
