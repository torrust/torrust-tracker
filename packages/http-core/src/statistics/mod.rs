pub mod event;
pub mod metrics;
pub mod repository;

use metrics::Metrics;
use torrust_metrics::metric::description::MetricDescription;
use torrust_metrics::metric_name;
use torrust_metrics::unit::Unit;

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

#[cfg(test)]
mod tests {
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric::description::MetricDescription;
    use torrust_metrics::metric_name;
    use torrust_metrics::prometheus::PrometheusSerializable;
    use torrust_metrics::unit::Unit;

    use super::{HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL, Metrics, describe_metrics};

    #[test]
    fn it_should_name_the_request_counter_http_tracker_core_requests_received_total() {
        // Act
        let name = HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;

        // Assert: the Prometheus series name that dashboards and alerts read.
        assert_eq!(name, "http_tracker_core_requests_received_total");
    }

    #[test]
    fn it_should_describe_the_request_counter_without_any_sample() {
        // Act
        let metrics = describe_metrics();

        // Assert: the counter exists, and a collection prints only metrics that have samples.
        assert!(
            metrics
                .metric_collection()
                .contains_counter(&metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL))
        );
        assert_eq!(metrics.metric_collection().to_prometheus(), "");
    }

    #[test]
    fn it_should_describe_the_request_counter_with_its_description_and_type() {
        // Arrange
        let mut metrics = describe_metrics();
        let labels: LabelSet = [("request_kind", "announce")].into();
        metrics
            .metric_collection
            .set_counter(
                &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
                &labels,
                1,
                DurationSinceUnixEpoch::ZERO,
            )
            .unwrap();

        // Act
        let exposition = metrics.metric_collection().to_prometheus();

        // Assert: the description and the type are what a Prometheus scraper reads. The unit is not part of this output.
        assert_eq!(
            exposition,
            "# HELP http_tracker_core_requests_received_total Total number of HTTP requests received\n\
             # TYPE http_tracker_core_requests_received_total counter\n\
             http_tracker_core_requests_received_total{request_kind=\"announce\"} 1"
        );
    }

    #[test]
    fn it_should_describe_the_request_counter_with_the_count_unit_and_its_description() {
        // Arrange: the whole value, built independently, including the unit that the exposition text omits.
        let mut expected = Metrics::default();
        expected.metric_collection.describe_counter(
            &metric_name!("http_tracker_core_requests_received_total"),
            Some(Unit::Count),
            Some(MetricDescription::new("Total number of HTTP requests received")),
        );

        // Act
        let described = describe_metrics();

        // Assert
        assert_eq!(described, expected);
    }
}
