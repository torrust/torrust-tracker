use std::sync::RwLock;

use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::counter::Counter;
use crate::gauge::Gauge;
use crate::label::LabelSet;
use crate::metric::description::MetricDescription;
use crate::metric::MetricName;
use crate::metric_collection::{MetricCollection, MetricKindCollection};
use crate::unit::Unit;

/* code-review:

   This might be not necessary, since the `MetricCollection` doesn't expose
   any method to mutate the collection items directly.

*/

/// A thread-safe wrapper around `MetricCollection` that allows concurrent
/// access to the metrics collection.
///
/// It protects the `MetricCollection` invariant:
///
/// "Metric's names must be unique in the collection for all types of metrics."
#[derive(Debug, Default)]
pub struct ThreadSafeMetricCollection {
    inner: RwLock<MetricCollection>,
}

impl ThreadSafeMetricCollection {
    #[must_use]
    pub fn new(counters: MetricKindCollection<Counter>, gauges: MetricKindCollection<Gauge>) -> Self {
        Self {
            inner: RwLock::new(MetricCollection::new(counters, gauges)),
        }
    }

    // Counter-specific methods

    /// # Panics
    ///
    /// Panics if it can't get write access to the inner collection.
    pub fn describe_counter(&mut self, name: &MetricName, _opt_unit: Option<Unit>, _opt_description: Option<MetricDescription>) {
        self.inner.write().unwrap().ensure_counter_exists(name);
    }

    /// It allows to describe a counter metric so the metrics appear in the JSON
    /// response even if there are no samples yet.
    ///
    /// # Panics
    ///
    /// Panics if it can't get read access to the inner collection.
    #[must_use]
    pub fn get_counter_value(&self, name: &MetricName, label_set: &LabelSet) -> Counter {
        self.inner.read().unwrap().get_counter_value(name, label_set)
    }

    /// # Panics
    ///
    /// Panics if it can't get write access to the inner collection.
    pub fn increase_counter(&mut self, name: &MetricName, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        self.inner.write().unwrap().increase_counter(name, label_set, time);
    }

    // Gauge-specific methods

    /// It allows to describe a gauge metric so the metrics appear in the JSON
    /// response even if there are no samples yet.
    ///
    /// # Panics
    ///
    /// Panics if it can't get write access to the inner collection.
    pub fn describe_gauge(&mut self, name: &MetricName, _opt_unit: Option<Unit>, _opt_description: Option<MetricDescription>) {
        self.inner.write().unwrap().ensure_gauge_exists(name);
    }

    /// # Panics
    ///
    /// Panics if it can't get read access to the inner collection.
    #[must_use]
    pub fn get_gauge_value(&self, name: &MetricName, label_set: &LabelSet) -> Gauge {
        self.inner.read().unwrap().get_gauge_value(name, label_set)
    }

    /// # Panics
    ///
    /// Panics if it can't get write access to the inner collection.
    pub fn set_gauge(&mut self, name: &MetricName, label_set: &LabelSet, value: f64, time: DurationSinceUnixEpoch) {
        self.inner.write().unwrap().set_gauge(name, label_set, value, time);
    }
}
