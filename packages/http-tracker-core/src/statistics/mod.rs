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
        &MetricName::new("http_tracker_core_announce_requests_received_total"),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of HTTP announce requests received")),
    );

    metrics.metric_collection.describe_counter(
        &MetricName::new("http_tracker_core_scrape_requests_received_total"),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of HTTP scrape requests received")),
    );

    metrics
}
