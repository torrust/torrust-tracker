use serde::Serialize;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::MetricName;
use torrust_metrics::metric_collection::aggregate::sum::Sum;
use torrust_metrics::metric_collection::{Error, MetricCollection};
use torrust_metrics::metric_name;

use crate::statistics::HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;

/// Metrics collected by the tracker.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Metrics {
    /// A collection of metrics.
    pub metric_collection: MetricCollection,
}

impl Metrics {
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
    /// Sum of the counter samples matching `labels`, as a whole number.
    ///
    /// `torrust-metrics` returns aggregate sums as `f64` even for `u64`
    /// counters (see torrust/torrust-metrics#1). Returns `0` when the metric
    /// does not exist yet.
    #[expect(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        reason = "The torrust-metrics Sum API returns f64; a counter sum is a finite, non-negative whole number, so the cast is exact below 2^53 and saturates above it"
    )]
    fn whole_total(&self, metric_name: &MetricName, labels: &LabelSet) -> u64 {
        self.metric_collection.sum(metric_name, labels).unwrap_or_default() as u64
    }

    /// Total number of TCP (HTTP tracker) `announce` requests from IPv4 peers.
    #[must_use]
    pub fn tcp4_announces_handled(&self) -> u64 {
        self.whole_total(
            &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet"), ("request_kind", "announce")].into(),
        )
    }

    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv4 peers.
    #[must_use]
    pub fn tcp4_scrapes_handled(&self) -> u64 {
        self.whole_total(
            &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet"), ("request_kind", "scrape")].into(),
        )
    }

    /// Total number of TCP (HTTP tracker) `announce` requests from IPv6 peers.
    #[must_use]
    pub fn tcp6_announces_handled(&self) -> u64 {
        self.whole_total(
            &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet6"), ("request_kind", "announce")].into(),
        )
    }

    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv6 peers.
    #[must_use]
    pub fn tcp6_scrapes_handled(&self) -> u64 {
        self.whole_total(
            &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
            &[("server_binding_address_ip_family", "inet6"), ("request_kind", "scrape")].into(),
        )
    }
}
