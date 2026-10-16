use serde::Serialize;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::MetricName;
use torrust_metrics::metric_collection::aggregate::sum::Sum;
use torrust_metrics::metric_collection::{Error, MetricCollection};
use torrust_metrics::metric_name;

use crate::statistics::UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;

#[derive(Debug, PartialEq, Default, Serialize)]
pub struct Metrics {
    /// A collection of metrics.
    pub metric_collection: MetricCollection,
}

impl Metrics {
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
    /// Sum of the counter samples matching `labels`, as a whole number.
    ///
    /// `torrust-metrics` returns aggregate sums as `f64` even for `u64`
    /// counters (see torrust/torrust-metrics#1). Returns `0` when the metric
    /// does not exist yet.
    #[expect(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        reason = "The torrust-metrics Sum API returns f64; a counter sum is a finite, non-negative whole number, so the cast is exact up to 2^53, and beyond that the f64 may already be rounded (the cast itself saturates only above u64::MAX)"
    )]
    fn whole_total(&self, metric_name: &MetricName, labels: &LabelSet) -> u64 {
        self.metric_collection.sum(metric_name, labels).unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) connections from IPv4 peers.
    #[must_use]
    pub fn udp4_connections_handled(&self) -> u64 {
        self.whole_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet"), ("request_kind", "connect")].into(),
        )
    }

    /// Total number of UDP (UDP tracker) `announce` requests from IPv4 peers.
    #[must_use]
    pub fn udp4_announces_handled(&self) -> u64 {
        self.whole_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet"), ("request_kind", "announce")].into(),
        )
    }

    /// Total number of UDP (UDP tracker) `scrape` requests from IPv4 peers.
    #[must_use]
    pub fn udp4_scrapes_handled(&self) -> u64 {
        self.whole_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet"), ("request_kind", "scrape")].into(),
        )
    }

    /// Total number of UDP (UDP tracker) `connection` requests from IPv6 peers.
    #[must_use]
    pub fn udp6_connections_handled(&self) -> u64 {
        self.whole_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet6"), ("request_kind", "connect")].into(),
        )
    }

    /// Total number of UDP (UDP tracker) `announce` requests from IPv6 peers.
    #[must_use]
    pub fn udp6_announces_handled(&self) -> u64 {
        self.whole_total(
            &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet6"), ("request_kind", "announce")].into(),
        )
    }

    /// Total number of UDP (UDP tracker) `scrape` requests from IPv6 peers.
    #[must_use]
    pub fn udp6_scrapes_handled(&self) -> u64 {
        self.whole_total(
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
}
