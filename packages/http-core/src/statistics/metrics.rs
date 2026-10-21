use serde::Serialize;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::MetricName;
use torrust_metrics::metric_collection::aggregate::sum::SumCounter;
use torrust_metrics::metric_collection::{Error, MetricCollection};
use torrust_metrics::metric_name;

use crate::statistics::HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;

/// Metrics collected by the tracker.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Metrics {
    /// A collection of metrics.
    pub(crate) metric_collection: MetricCollection,
}

impl Metrics {
    /// The collection of metrics, for read-only access.
    #[must_use]
    pub const fn metric_collection(&self) -> &MetricCollection {
        &self.metric_collection
    }

    /// # Errors
    ///
    /// Returns an error if the metric does not exist and it cannot be created.
    pub fn increase_counter(
        &mut self,
        metric_name: &MetricName,
        labels: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        self.metric_collection.increment_counter(metric_name, labels, now)
    }

    /// # Errors
    ///
    /// Returns an error if the metric does not exist and it cannot be created.
    pub fn set_gauge(
        &mut self,
        metric_name: &MetricName,
        labels: &LabelSet,
        value: f64,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        self.metric_collection.set_gauge(metric_name, labels, value, now)
    }
}

impl Metrics {
    /// Sum of the counter samples matching `labels`. Returns `0` when the
    /// metric does not exist yet, and also when it is a gauge, because
    /// `sum_counter` returns `None` for a gauge. Call it only with counter
    /// names.
    fn counter_total(&self, metric_name: &MetricName, labels: &LabelSet) -> u64 {
        self.metric_collection.sum_counter(metric_name, labels).unwrap_or_default()
    }

    /// Total number of TCP (HTTP tracker) `announce` requests from IPv4 peers.
    #[must_use]
    pub fn tcp4_announces_handled(&self) -> u64 {
        self.counter_total(
            &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet"), ("request_kind", "announce")].into(),
        )
    }

    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv4 peers.
    #[must_use]
    pub fn tcp4_scrapes_handled(&self) -> u64 {
        self.counter_total(
            &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet"), ("request_kind", "scrape")].into(),
        )
    }

    /// Total number of TCP (HTTP tracker) `announce` requests from IPv6 peers.
    #[must_use]
    pub fn tcp6_announces_handled(&self) -> u64 {
        self.counter_total(
            &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet6"), ("request_kind", "announce")].into(),
        )
    }

    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv6 peers.
    #[must_use]
    pub fn tcp6_scrapes_handled(&self) -> u64 {
        self.counter_total(
            &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet6"), ("request_kind", "scrape")].into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_metrics::gauge::Gauge;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;

    use super::Metrics;
    use crate::statistics::{HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL, describe_metrics};

    const NOW: DurationSinceUnixEpoch = DurationSinceUnixEpoch::ZERO;

    /// The labels of a request sample of a server bound in `server_family`, of kind `request_kind`, on `server_port`.
    fn request_labels(server_family: &str, request_kind: &str, server_port: &str) -> LabelSet {
        [
            ("server_binding_address_ip_family", server_family),
            ("request_kind", request_kind),
            ("server_binding_port", server_port),
        ]
        .into()
    }

    /// Sets the request sample with exactly `labels` to `value`.
    fn set_requests(metrics: &mut Metrics, labels: &LabelSet, value: u64) {
        metrics
            .metric_collection
            .set_counter(&metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), labels, value, NOW)
            .unwrap();
    }

    /// The four public totals: IPv4 announces, IPv4 scrapes, IPv6 announces, IPv6 scrapes.
    fn totals(metrics: &Metrics) -> (u64, u64, u64, u64) {
        (
            metrics.tcp4_announces_handled(),
            metrics.tcp4_scrapes_handled(),
            metrics.tcp6_announces_handled(),
            metrics.tcp6_scrapes_handled(),
        )
    }

    fn metrics_with_a_gauge_named_like_the_request_counter() -> Metrics {
        let mut metrics = Metrics::default();
        metrics
            .set_gauge(
                &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
                &request_labels("inet", "announce", "7070"),
                5.0,
                NOW,
            )
            .unwrap();
        metrics
    }

    #[test]
    fn it_should_project_exactly_a_total_that_an_f64_cannot_represent() {
        // Arrange: 2^53 + 1 is the first whole number an f64 rounds.
        let total_beyond_f64_precision = (1_u64 << 53) + 1;
        let mut metrics = describe_metrics();
        let labels: LabelSet = [("server_binding_address_ip_family", "inet"), ("request_kind", "announce")].into();
        metrics
            .metric_collection
            .set_counter(
                &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
                &labels,
                total_beyond_f64_precision,
                DurationSinceUnixEpoch::ZERO,
            )
            .unwrap();

        // Act
        let announces = metrics.tcp4_announces_handled();

        // Assert
        assert_eq!(announces, total_beyond_f64_precision);
    }

    #[test]
    fn it_should_project_each_getter_from_its_own_family_and_request_kind() {
        // Arrange: four samples with distinct values, so a getter that reads another sample returns another value.
        let mut metrics = describe_metrics();
        set_requests(&mut metrics, &request_labels("inet", "announce", "7070"), 1);
        set_requests(&mut metrics, &request_labels("inet", "scrape", "7070"), 2);
        set_requests(&mut metrics, &request_labels("inet6", "announce", "7070"), 3);
        set_requests(&mut metrics, &request_labels("inet6", "scrape", "7070"), 4);

        // Act
        let projected = totals(&metrics);

        // Assert
        assert_eq!(projected, (1, 2, 3, 4));
    }

    #[test]
    fn it_should_sum_the_samples_of_a_family_and_request_kind_whatever_their_other_labels() {
        // Arrange: two IPv4 announce samples on different ports, and a scrape that must not be added.
        let mut metrics = describe_metrics();
        set_requests(&mut metrics, &request_labels("inet", "announce", "7070"), 3);
        set_requests(&mut metrics, &request_labels("inet", "announce", "7071"), 4);
        set_requests(&mut metrics, &request_labels("inet", "scrape", "7070"), 5);

        // Act
        let projected = totals(&metrics);

        // Assert
        assert_eq!(projected, (7, 5, 0, 0));
    }

    #[test]
    fn it_should_count_by_the_family_of_the_server_binding_and_not_of_the_client() {
        // Arrange: an IPv6 client of a tracker bound to an IPv4 address.
        let mut metrics = describe_metrics();
        let labels: LabelSet = [
            ("server_binding_address_ip_family", "inet"),
            ("client_address_ip_family", "inet6"),
            ("request_kind", "announce"),
        ]
        .into();
        set_requests(&mut metrics, &labels, 1);

        // Act
        let projected = totals(&metrics);

        // Assert: the request counts as IPv4 because the tracker listens on IPv4.
        assert_eq!(projected, (1, 0, 0, 0));
    }

    #[rstest]
    #[case::no_metric(Metrics::default())]
    #[case::described_without_samples(describe_metrics())]
    #[case::a_gauge_under_the_counter_name(metrics_with_a_gauge_named_like_the_request_counter())]
    fn it_should_total_zero_when_there_is_no_request_counter_sample(#[case] metrics: Metrics) {
        // Act
        let projected = totals(&metrics);

        // Assert
        assert_eq!(projected, (0, 0, 0, 0));
    }

    #[test]
    fn it_should_increase_a_counter_by_one_for_each_call() {
        // Arrange
        let mut metrics = Metrics::default();
        let name = metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL);
        let labels = request_labels("inet", "announce", "7070");

        // Act
        metrics.increase_counter(&name, &labels, NOW).unwrap();
        metrics.increase_counter(&name, &labels, NOW).unwrap();

        // Assert
        assert_eq!(totals(&metrics), (2, 0, 0, 0));
    }

    #[test]
    fn it_should_return_the_error_of_the_collection_when_it_increases_a_gauge_as_a_counter() {
        // Arrange
        let mut metrics = metrics_with_a_gauge_named_like_the_request_counter();

        // Act
        let result = metrics.increase_counter(
            &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &request_labels("inet", "announce", "7070"),
            NOW,
        );

        // Assert
        assert!(result.is_err(), "increasing a gauge as a counter should fail: {result:?}");
    }

    #[test]
    fn it_should_store_the_value_of_a_gauge_set_through_the_metrics() {
        // Arrange
        let mut metrics = Metrics::default();
        let name = metric_name!("a_test_gauge");
        let labels = request_labels("inet", "announce", "7070");

        // Act
        metrics.set_gauge(&name, &labels, 5.0, NOW).unwrap();

        // Assert
        assert_eq!(
            metrics.metric_collection().get_gauge_value(&name, &labels),
            Some(Gauge::new(5.0))
        );
    }
}
