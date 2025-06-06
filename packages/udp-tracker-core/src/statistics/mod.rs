pub mod event;
pub mod metrics;
pub mod repository;
pub mod services;

use metrics::Metrics;
use torrust_tracker_metrics::metric::description::MetricDescription;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_metrics::unit::Unit;

const UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL: &str = "udp_tracker_core_requests_received_total";

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    metrics.metric_collection.describe_counter(
        &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests received")),
    );

    metrics
}
