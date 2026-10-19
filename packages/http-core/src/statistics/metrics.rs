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
    /// metric does not exist yet.
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
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;

    use crate::statistics::{HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL, describe_metrics};

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
}
