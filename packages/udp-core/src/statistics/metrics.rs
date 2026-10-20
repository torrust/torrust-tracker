use serde::Serialize;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::MetricName;
use torrust_metrics::metric_collection::aggregate::sum::SumCounter;
use torrust_metrics::metric_collection::{Error, MetricCollection};
use torrust_metrics::metric_name;

use crate::statistics::UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;

#[derive(Debug, PartialEq, Default, Serialize)]
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
    /// This function returns an error if the metric does not exist and it
    /// cannot be created.
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
    /// This function returns an error if the metric does not exist and it
    /// cannot be created.
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

    /// Total number of UDP (UDP tracker) connections from IPv4 peers.
    #[must_use]
    pub fn udp4_connections_handled(&self) -> u64 {
        self.counter_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet"), ("request_kind", "connect")].into(),
        )
    }

    /// Total number of UDP (UDP tracker) `announce` requests from IPv4 peers.
    #[must_use]
    pub fn udp4_announces_handled(&self) -> u64 {
        self.counter_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet"), ("request_kind", "announce")].into(),
        )
    }

    /// Total number of UDP (UDP tracker) `scrape` requests from IPv4 peers.
    #[must_use]
    pub fn udp4_scrapes_handled(&self) -> u64 {
        self.counter_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet"), ("request_kind", "scrape")].into(),
        )
    }

    /// Total number of UDP (UDP tracker) `connection` requests from IPv6 peers.
    #[must_use]
    pub fn udp6_connections_handled(&self) -> u64 {
        self.counter_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet6"), ("request_kind", "connect")].into(),
        )
    }

    /// Total number of UDP (UDP tracker) `announce` requests from IPv6 peers.
    #[must_use]
    pub fn udp6_announces_handled(&self) -> u64 {
        self.counter_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet6"), ("request_kind", "announce")].into(),
        )
    }

    /// Total number of UDP (UDP tracker) `scrape` requests from IPv6 peers.
    #[must_use]
    pub fn udp6_scrapes_handled(&self) -> u64 {
        self.counter_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet6"), ("request_kind", "scrape")].into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;

    use super::Metrics;
    use crate::statistics::{UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL, describe_metrics};

    fn record_requests(metrics: &mut Metrics, ip_family: &str, request_kind: &str, server_port: &str, count: u32) {
        let labels: LabelSet = [
            ("server_binding_address_ip_family", ip_family),
            ("request_kind", request_kind),
            ("server_binding_port", server_port),
        ]
        .into();

        for _ in 0..count {
            metrics
                .increase_counter(
                    &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
                    &labels,
                    DurationSinceUnixEpoch::ZERO,
                )
                .unwrap();
        }
    }

    #[test]
    fn it_should_count_in_each_projection_only_its_own_ip_family_and_request_kind() {
        // Arrange
        let mut metrics = describe_metrics();
        record_requests(&mut metrics, "inet", "connect", "6969", 1);
        record_requests(&mut metrics, "inet", "announce", "6969", 2);
        record_requests(&mut metrics, "inet", "scrape", "6969", 3);
        record_requests(&mut metrics, "inet6", "connect", "6969", 4);
        record_requests(&mut metrics, "inet6", "announce", "6969", 5);
        record_requests(&mut metrics, "inet6", "scrape", "6969", 6);

        // Act
        let projections = [
            metrics.udp4_connections_handled(),
            metrics.udp4_announces_handled(),
            metrics.udp4_scrapes_handled(),
            metrics.udp6_connections_handled(),
            metrics.udp6_announces_handled(),
            metrics.udp6_scrapes_handled(),
        ];

        // Assert
        assert_eq!(projections, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn it_should_add_up_in_a_projection_the_requests_received_by_every_listener() {
        // Arrange
        let mut metrics = describe_metrics();
        record_requests(&mut metrics, "inet", "announce", "6969", 2);
        record_requests(&mut metrics, "inet", "announce", "7070", 3);

        // Act
        let announces = metrics.udp4_announces_handled();

        // Assert
        assert_eq!(announces, 5);
    }

    #[test]
    fn it_should_project_zero_everywhere_when_no_request_was_recorded() {
        // Arrange
        let metrics = describe_metrics();

        // Act
        let projections = [
            metrics.udp4_connections_handled(),
            metrics.udp4_announces_handled(),
            metrics.udp4_scrapes_handled(),
            metrics.udp6_connections_handled(),
            metrics.udp6_announces_handled(),
            metrics.udp6_scrapes_handled(),
        ];

        // Assert
        assert_eq!(projections, [0; 6]);
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
                &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
                &labels,
                total_beyond_f64_precision,
                DurationSinceUnixEpoch::ZERO,
            )
            .unwrap();

        // Act
        let announces = metrics.udp4_announces_handled();

        // Assert
        assert_eq!(announces, total_beyond_f64_precision);
    }
}
