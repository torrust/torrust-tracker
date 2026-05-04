use std::collections::HashMap;

use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::counter::Counter;
use crate::gauge::Gauge;
use crate::label::LabelSet;
use crate::metric::{Metric, MetricName};
use crate::metric_collection::error::Error;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MetricKindCollection<T> {
    pub(super) metrics: HashMap<MetricName, Metric<T>>,
}

impl<T> MetricKindCollection<T> {
    /// Creates a new `MetricKindCollection` from a vector of metrics
    ///
    /// # Errors
    ///
    /// Returns an error if duplicate metric names are passed.
    pub fn new(metrics: Vec<Metric<T>>) -> Result<Self, Error> {
        let mut map = HashMap::with_capacity(metrics.len());

        for metric in metrics {
            let metric_name = metric.name().clone();

            if let Some(_old_metric) = map.insert(metric.name().clone(), metric) {
                return Err(Error::DuplicateMetricNameInList { metric_name });
            }
        }

        Ok(Self { metrics: map })
    }

    /// Returns an iterator over all metric names in this collection.
    pub fn names(&self) -> impl Iterator<Item = &MetricName> {
        self.metrics.keys()
    }

    pub fn insert_if_absent(&mut self, metric: Metric<T>) {
        if !self.metrics.contains_key(metric.name()) {
            self.insert(metric);
        }
    }

    pub fn insert(&mut self, metric: Metric<T>) {
        self.metrics.insert(metric.name().clone(), metric);
    }
}

impl<T: Clone> MetricKindCollection<T> {
    /// Merges another `MetricKindCollection` into this one.
    ///
    /// # Errors
    ///
    /// Returns an error if a metric name already exists in the current collection.
    pub fn merge(&mut self, other: &Self) -> Result<(), Error> {
        self.check_for_name_collision(other)?;

        for (metric_name, metric) in &other.metrics {
            self.metrics.insert(metric_name.clone(), metric.clone());
        }

        Ok(())
    }

    fn check_for_name_collision(&self, other: &Self) -> Result<(), Error> {
        for metric_name in other.metrics.keys() {
            if self.metrics.contains_key(metric_name) {
                return Err(Error::MetricNameCollisionInMerge {
                    metric_name: metric_name.clone(),
                });
            }
        }

        Ok(())
    }
}

impl MetricKindCollection<Counter> {
    /// Increments the counter for the given metric name and labels.
    ///
    /// If the metric name does not exist, it will be created.
    ///
    /// # Panics
    ///
    /// Panics if the metric does not exist.
    pub fn increment(&mut self, name: &MetricName, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        let metric = Metric::<Counter>::new_empty_with_name(name.clone());

        self.insert_if_absent(metric);

        let metric = self.metrics.get_mut(name).expect("Counter metric should exist");

        metric.increment(label_set, time);
    }

    /// Sets the counter to an absolute value for the given metric name and labels.
    ///
    /// If the metric name does not exist, it will be created.
    ///
    /// # Panics
    ///
    /// Panics if the metric does not exist.
    pub fn absolute(&mut self, name: &MetricName, label_set: &LabelSet, value: u64, time: DurationSinceUnixEpoch) {
        let metric = Metric::<Counter>::new_empty_with_name(name.clone());

        self.insert_if_absent(metric);

        let metric = self.metrics.get_mut(name).expect("Counter metric should exist");

        metric.absolute(label_set, value, time);
    }

    #[must_use]
    pub fn get_value(&self, name: &MetricName, label_set: &LabelSet) -> Option<Counter> {
        self.metrics
            .get(name)
            .and_then(|metric| metric.get_sample_data(label_set))
            .map(|sample| sample.value().clone())
    }
}

impl MetricKindCollection<Gauge> {
    /// Sets the gauge for the given metric name and labels.
    ///
    /// If the metric name does not exist, it will be created.
    ///
    /// # Panics
    ///
    /// Panics if the metric does not exist and it could not be created.
    pub fn set(&mut self, name: &MetricName, label_set: &LabelSet, value: f64, time: DurationSinceUnixEpoch) {
        let metric = Metric::<Gauge>::new_empty_with_name(name.clone());

        self.insert_if_absent(metric);

        let metric = self.metrics.get_mut(name).expect("Gauge metric should exist");

        metric.set(label_set, value, time);
    }

    /// Increments the gauge for the given metric name and labels.
    ///
    /// If the metric name does not exist, it will be created.
    ///
    /// # Panics
    ///
    /// Panics if the metric does not exist and it could not be created.
    pub fn increment(&mut self, name: &MetricName, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        let metric = Metric::<Gauge>::new_empty_with_name(name.clone());

        self.insert_if_absent(metric);

        let metric = self.metrics.get_mut(name).expect("Gauge metric should exist");

        metric.increment(label_set, time);
    }

    /// Decrements the gauge for the given metric name and labels.
    ///
    /// If the metric name does not exist, it will be created.
    ///
    /// # Panics
    ///
    /// Panics if the metric does not exist and it could not be created.
    pub fn decrement(&mut self, name: &MetricName, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        let metric = Metric::<Gauge>::new_empty_with_name(name.clone());

        self.insert_if_absent(metric);

        let metric = self.metrics.get_mut(name).expect("Gauge metric should exist");

        metric.decrement(label_set, time);
    }

    #[must_use]
    pub fn get_value(&self, name: &MetricName, label_set: &LabelSet) -> Option<Gauge> {
        self.metrics
            .get(name)
            .and_then(|metric| metric.get_sample_data(label_set))
            .map(|sample| sample.value().clone())
    }
}

#[cfg(test)]
mod tests {

    use crate::counter::Counter;
    use crate::gauge::Gauge;
    use crate::metric::Metric;
    use crate::metric_collection::{Error, MetricKindCollection};
    use crate::metric_name;

    #[test]
    fn it_should_not_allow_merging_counter_metric_collections_with_name_collisions() {
        let mut collection1 = MetricKindCollection::<Counter>::default();
        collection1.insert(Metric::<Counter>::new_empty_with_name(metric_name!("test_metric")));

        let mut collection2 = MetricKindCollection::<Counter>::default();
        collection2.insert(Metric::<Counter>::new_empty_with_name(metric_name!("test_metric")));

        let result = collection1.merge(&collection2);

        assert!(
            result.is_err()
                && matches!(result, Err(Error::MetricNameCollisionInMerge { metric_name }) if metric_name == metric_name!("test_metric"))
        );
    }

    #[test]
    fn it_should_not_allow_merging_gauge_metric_collections_with_name_collisions() {
        let mut collection1 = MetricKindCollection::<Gauge>::default();
        collection1.insert(Metric::<Gauge>::new_empty_with_name(metric_name!("test_metric")));

        let mut collection2 = MetricKindCollection::<Gauge>::default();
        collection2.insert(Metric::<Gauge>::new_empty_with_name(metric_name!("test_metric")));

        let result = collection1.merge(&collection2);

        assert!(
            result.is_err()
                && matches!(result, Err(Error::MetricNameCollisionInMerge { metric_name }) if metric_name == metric_name!("test_metric"))
        );
    }
}
