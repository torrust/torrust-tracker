pub mod event;
pub mod metrics;
pub mod repository;

use metrics::Metrics;
use torrust_tracker_metrics::metric::description::MetricDescription;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_metrics::unit::Unit;

pub const HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL: &str = "http_tracker_core_requests_received_total";

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    metrics.metric_collection.describe_counter(
        &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of HTTP requests received")),
    );

    metrics
}
