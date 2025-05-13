use std::collections::{HashMap, HashSet};

use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Deserializer, Serialize};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::counter::Counter;
use super::gauge::Gauge;
use super::label::LabelSet;
use super::metric::{Metric, MetricName};
use super::prometheus::PrometheusSerializable;
use crate::metric::description::MetricDescription;
use crate::sample_collection::SampleCollection;
use crate::unit::Unit;
use crate::METRICS_TARGET;

// code-review: serialize in a deterministic order? For example:
// - First the counter metrics ordered by name.
// - Then the gauge metrics ordered by name.

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MetricCollection {
    counters: MetricKindCollection<Counter>,
    gauges: MetricKindCollection<Gauge>,
}

impl MetricCollection {
    /// # Errors
    ///
    /// Returns an error if there are duplicate metric names across counters and
    /// gauges.
    pub fn new(counters: MetricKindCollection<Counter>, gauges: MetricKindCollection<Gauge>) -> Result<Self, Error> {
        // Check for name collisions across metric types
        let counter_names: HashSet<_> = counters.names().collect();
        let gauge_names: HashSet<_> = gauges.names().collect();

        if !counter_names.is_disjoint(&gauge_names) {
            return Err(Error::MetricNameCollisionInConstructor {
                counter_names: counter_names.iter().map(std::string::ToString::to_string).collect(),
                gauge_names: gauge_names.iter().map(std::string::ToString::to_string).collect(),
            });
        }

        Ok(Self { counters, gauges })
    }

    /// Merges another `MetricCollection` into this one.
    ///
    /// # Errors
    ///
    /// Returns an error if a metric name already exists in the current collection.
    pub fn merge(&mut self, other: &Self) -> Result<(), Error> {
        self.counters.merge(&other.counters)?;
        self.gauges.merge(&other.gauges)?;
        Ok(())
    }

    // Counter-specific methods

    pub fn describe_counter(&mut self, name: &MetricName, opt_unit: Option<Unit>, opt_description: Option<&MetricDescription>) {
        tracing::info!(target: METRICS_TARGET, type = "counter", name = name.to_string(), unit = ?opt_unit, description = ?opt_description);
        self.counters.ensure_metric_exists(name);
    }

    #[must_use]
    pub fn contains_counter(&self, name: &MetricName) -> bool {
        self.counters.metrics.contains_key(name)
    }

    #[must_use]
    pub fn get_counter_value(&self, name: &MetricName, label_set: &LabelSet) -> Option<Counter> {
        self.counters.get_value(name, label_set)
    }

    /// # Errors
    ///
    /// Return an error if a metrics of a different type with the same name
    /// already exists.
    pub fn increase_counter(
        &mut self,
        name: &MetricName,
        label_set: &LabelSet,
        time: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        if self.gauges.metrics.contains_key(name) {
            return Err(Error::MetricNameCollisionAdding {
                metric_name: name.clone(),
            });
        }

        self.counters.increment(name, label_set, time);

        Ok(())
    }

    pub fn ensure_counter_exists(&mut self, name: &MetricName) {
        self.counters.ensure_metric_exists(name);
    }

    // Gauge-specific methods

    pub fn describe_gauge(&mut self, name: &MetricName, opt_unit: Option<Unit>, opt_description: Option<&MetricDescription>) {
        tracing::info!(target: METRICS_TARGET, type = "gauge", name = name.to_string(), unit = ?opt_unit, description = ?opt_description);
        self.gauges.ensure_metric_exists(name);
    }

    #[must_use]
    pub fn contains_gauge(&self, name: &MetricName) -> bool {
        self.gauges.metrics.contains_key(name)
    }

    #[must_use]
    pub fn get_gauge_value(&self, name: &MetricName, label_set: &LabelSet) -> Option<Gauge> {
        self.gauges.get_value(name, label_set)
    }

    /// # Errors
    ///
    /// Return an error if a metrics of a different type with the same name
    /// already exists.
    pub fn set_gauge(
        &mut self,
        name: &MetricName,
        label_set: &LabelSet,
        value: f64,
        time: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        if self.counters.metrics.contains_key(name) {
            return Err(Error::MetricNameCollisionAdding {
                metric_name: name.clone(),
            });
        }

        self.gauges.set(name, label_set, value, time);

        Ok(())
    }

    /// # Errors
    ///
    /// Return an error if a metrics of a different type with the same name
    /// already exists.
    pub fn increase_gauge(&mut self, name: &MetricName, label_set: &LabelSet, time: DurationSinceUnixEpoch) -> Result<(), Error> {
        if self.counters.metrics.contains_key(name) {
            return Err(Error::MetricNameCollisionAdding {
                metric_name: name.clone(),
            });
        }

        self.gauges.increment(name, label_set, time);

        Ok(())
    }

    /// # Errors
    ///
    /// Return an error if a metrics of a different type with the same name
    /// already exists.
    pub fn decrease_gauge(&mut self, name: &MetricName, label_set: &LabelSet, time: DurationSinceUnixEpoch) -> Result<(), Error> {
        if self.counters.metrics.contains_key(name) {
            return Err(Error::MetricNameCollisionAdding {
                metric_name: name.clone(),
            });
        }

        self.gauges.decrement(name, label_set, time);

        Ok(())
    }

    pub fn ensure_gauge_exists(&mut self, name: &MetricName) {
        self.gauges.ensure_metric_exists(name);
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Metric names must be unique across all metrics types.")]
    MetricNameCollisionInConstructor {
        counter_names: Vec<String>,
        gauge_names: Vec<String>,
    },

    #[error("Found duplicate metric name in list. Metric names must be unique across all metrics types.")]
    DuplicateMetricNameInList { metric_name: MetricName },

    #[error("Cannot merge metric '{metric_name}': it already exists in the current collection")]
    MetricNameCollisionInMerge { metric_name: MetricName },

    #[error("Cannot create metric with name '{metric_name}': another metric with this name already exists")]
    MetricNameCollisionAdding { metric_name: MetricName },
}

/// Implements serialization for `MetricCollection`.
impl Serialize for MetricCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(tag = "kind", rename_all = "lowercase")]
        enum SerializableMetric<'a> {
            Counter(&'a Metric<Counter>),
            Gauge(&'a Metric<Gauge>),
        }

        let mut seq = serializer.serialize_seq(Some(self.counters.metrics.len() + self.gauges.metrics.len()))?;

        for metric in self.counters.metrics.values() {
            seq.serialize_element(&SerializableMetric::Counter(metric))?;
        }

        for metric in self.gauges.metrics.values() {
            seq.serialize_element(&SerializableMetric::Gauge(metric))?;
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for MetricCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "kind", rename_all = "lowercase")]
        enum MetricPayload {
            Counter(Metric<Counter>),
            Gauge(Metric<Gauge>),
        }

        let payload = Vec::<MetricPayload>::deserialize(deserializer)?;

        let mut counters = Vec::new();
        let mut gauges = Vec::new();

        for metric in payload {
            match metric {
                MetricPayload::Counter(counter) => counters.push(counter),
                MetricPayload::Gauge(gauge) => gauges.push(gauge),
            }
        }

        let counters = MetricKindCollection::new(counters).map_err(serde::de::Error::custom)?;
        let gauges = MetricKindCollection::new(gauges).map_err(serde::de::Error::custom)?;

        let metric_collection = MetricCollection::new(counters, gauges).map_err(serde::de::Error::custom)?;

        Ok(metric_collection)
    }
}

impl PrometheusSerializable for MetricCollection {
    fn to_prometheus(&self) -> String {
        self.counters
            .metrics
            .values()
            .filter(|metric| !metric.is_empty())
            .map(Metric::<Counter>::to_prometheus)
            .chain(
                self.gauges
                    .metrics
                    .values()
                    .filter(|metric| !metric.is_empty())
                    .map(Metric::<Gauge>::to_prometheus),
            )
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MetricKindCollection<T> {
    metrics: HashMap<MetricName, Metric<T>>,
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

    /// # Panics
    ///
    /// It should not panic as long as empty sample collections are allowed.
    pub fn ensure_metric_exists(&mut self, name: &MetricName) {
        if !self.metrics.contains_key(name) {
            self.metrics.insert(
                name.clone(),
                Metric::new(
                    name.clone(),
                    SampleCollection::new(vec![]).expect("Empty sample collection creation should not fail"),
                ),
            );
        }
    }
}

impl<T: Clone> MetricKindCollection<T> {
    /// Merges another `MetricKindCollection` into this one.
    ///
    /// # Errors
    ///
    /// Returns an error if a metric name already exists in the current collection.
    pub fn merge(&mut self, other: &Self) -> Result<(), Error> {
        // Check for name collisions
        for metric_name in other.metrics.keys() {
            if self.metrics.contains_key(metric_name) {
                return Err(Error::MetricNameCollisionInMerge {
                    metric_name: metric_name.clone(),
                });
            }
        }

        for (metric_name, metric) in &other.metrics {
            if self.metrics.insert(metric_name.clone(), metric.clone()).is_some() {
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
    /// Panics if the metric does not exist and it could not be created.
    pub fn increment(&mut self, name: &MetricName, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        self.ensure_metric_exists(name);

        let metric = self.metrics.get_mut(name).expect("Counter metric should exist");

        metric.increment(label_set, time);
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
        self.ensure_metric_exists(name);

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
        self.ensure_metric_exists(name);

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
        self.ensure_metric_exists(name);

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

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::label::LabelValue;
    use crate::sample::Sample;
    use crate::tests::{format_prometheus_output, sort_lines};
    use crate::{label_name, metric_name};

    /// Fixture for testing serialization and deserialization of `MetricCollection`.
    ///
    /// It contains a default `MetricCollection` object, its JSON representation,
    /// and its Prometheus format representation.
    struct MetricCollectionFixture {
        pub object: MetricCollection,
        pub json: String,
        pub prometheus: String,
    }

    impl Default for MetricCollectionFixture {
        fn default() -> Self {
            Self {
                object: Self::object(),
                json: Self::json(),
                prometheus: Self::prometheus(),
            }
        }
    }

    impl MetricCollectionFixture {
        fn deconstruct(&self) -> (MetricCollection, String, String) {
            (self.object.clone(), self.json.clone(), self.prometheus.clone())
        }

        fn object() -> MetricCollection {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);

            let label_set_1: LabelSet = [
                (label_name!("server_binding_protocol"), LabelValue::new("http")),
                (label_name!("server_binding_ip"), LabelValue::new("0.0.0.0")),
                (label_name!("server_binding_port"), LabelValue::new("7070")),
            ]
            .into();

            MetricCollection::new(
                MetricKindCollection::new(vec![Metric::new(
                    metric_name!("http_tracker_core_announce_requests_received_total"),
                    SampleCollection::new(vec![Sample::new(Counter::new(1), time, label_set_1.clone())]).unwrap(),
                )])
                .unwrap(),
                MetricKindCollection::new(vec![Metric::new(
                    metric_name!("udp_tracker_server_performance_avg_announce_processing_time_ns"),
                    SampleCollection::new(vec![Sample::new(Gauge::new(1.0), time, label_set_1.clone())]).unwrap(),
                )])
                .unwrap(),
            )
            .unwrap()
        }

        fn json() -> String {
            r#"
            [
                {
                    "kind":"counter",
                    "name":"http_tracker_core_announce_requests_received_total",
                    "samples":[
                        {
                            "value":1,
                            "recorded_at":"2025-04-02T00:00:00+00:00",
                            "labels":[
                                {
                                    "name":"server_binding_ip",
                                    "value":"0.0.0.0"
                                },
                                {
                                    "name":"server_binding_port",
                                    "value":"7070"
                                },
                                {
                                    "name":"server_binding_protocol",
                                    "value":"http"
                                }
                            ]
                        }
                    ]
                },
                {
                    "kind":"gauge",
                    "name":"udp_tracker_server_performance_avg_announce_processing_time_ns",
                    "samples":[
                        {
                            "value":1.0,
                            "recorded_at":"2025-04-02T00:00:00+00:00",
                            "labels":[
                                {
                                    "name":"server_binding_ip",
                                    "value":"0.0.0.0"
                                },
                                {
                                    "name":"server_binding_port",
                                    "value":"7070"
                                },
                                {
                                    "name":"server_binding_protocol",
                                    "value":"http"
                                }
                            ]
                        }
                    ]
                }
            ]
            "#
            .to_owned()
        }

        fn prometheus() -> String {
            format_prometheus_output(
                r#"
                    http_tracker_core_announce_requests_received_total{server_binding_ip="0.0.0.0",server_binding_port="7070",server_binding_protocol="http"} 1
                    udp_tracker_server_performance_avg_announce_processing_time_ns{server_binding_ip="0.0.0.0",server_binding_port="7070",server_binding_protocol="http"} 1
                "#,
            )
        }
    }

    #[test]
    fn it_should_not_allow_duplicate_names_across_types() {
        let counters =
            MetricKindCollection::new(vec![Metric::new(metric_name!("test_metric"), SampleCollection::default())]).unwrap();
        let gauges =
            MetricKindCollection::new(vec![Metric::new(metric_name!("test_metric"), SampleCollection::default())]).unwrap();

        assert!(MetricCollection::new(counters, gauges).is_err());
    }

    #[test]
    fn it_should_not_allow_creating_a_gauge_with_the_same_name_as_a_counter() {
        let mut collection = MetricCollection::default();
        let label_set = LabelSet::default();
        let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);

        // First create a counter
        collection
            .increase_counter(&metric_name!("test_metric"), &label_set, time)
            .unwrap();

        // Then try to create a gauge with the same name
        let result = collection.set_gauge(&metric_name!("test_metric"), &label_set, 1.0, time);

        assert!(result.is_err());
    }

    #[test]
    fn it_should_not_allow_creating_a_counter_with_the_same_name_as_a_gauge() {
        let mut collection = MetricCollection::default();
        let label_set = LabelSet::default();
        let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);

        // First set the gauge
        collection
            .set_gauge(&metric_name!("test_metric"), &label_set, 1.0, time)
            .unwrap();

        // Then try to create a counter with the same name
        let result = collection.increase_counter(&metric_name!("test_metric"), &label_set, time);

        assert!(result.is_err());
    }

    #[test]
    fn it_should_allow_serializing_to_json() {
        // todo: this test does work with metric with multiple samples because
        // samples are not serialized in the same order as they are created.
        let (metric_collection, expected_json, _expected_prometheus) = MetricCollectionFixture::default().deconstruct();

        let json = serde_json::to_string_pretty(&metric_collection).unwrap();

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&json).unwrap(),
            serde_json::from_str::<serde_json::Value>(&expected_json).unwrap()
        );
    }

    #[test]
    fn it_should_allow_deserializing_from_json() {
        let (expected_metric_collection, metric_collection_json, _expected_prometheus) =
            MetricCollectionFixture::default().deconstruct();

        let metric_collection: MetricCollection = serde_json::from_str(&metric_collection_json).unwrap();

        assert_eq!(metric_collection, expected_metric_collection);
    }

    #[test]
    fn it_should_allow_serializing_to_prometheus_format() {
        let (metric_collection, _expected_json, expected_prometheus) = MetricCollectionFixture::default().deconstruct();

        let prometheus_output = metric_collection.to_prometheus();

        assert_eq!(prometheus_output, expected_prometheus);
    }

    #[test]
    fn it_should_allow_serializing_to_prometheus_format_with_multiple_samples_per_metric() {
        let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);

        let label_set_1: LabelSet = [
            (label_name!("server_binding_protocol"), LabelValue::new("http")),
            (label_name!("server_binding_ip"), LabelValue::new("0.0.0.0")),
            (label_name!("server_binding_port"), LabelValue::new("7070")),
        ]
        .into();

        let label_set_2: LabelSet = [
            (label_name!("server_binding_protocol"), LabelValue::new("http")),
            (label_name!("server_binding_ip"), LabelValue::new("0.0.0.0")),
            (label_name!("server_binding_port"), LabelValue::new("7171")),
        ]
        .into();

        let metric_collection = MetricCollection::new(
            MetricKindCollection::new(vec![Metric::new(
                metric_name!("http_tracker_core_announce_requests_received_total"),
                SampleCollection::new(vec![
                    Sample::new(Counter::new(1), time, label_set_1.clone()),
                    Sample::new(Counter::new(2), time, label_set_2.clone()),
                ])
                .unwrap(),
            )])
            .unwrap(),
            MetricKindCollection::default(),
        )
        .unwrap();

        let prometheus_output = metric_collection.to_prometheus();

        let expected_prometheus_output = format_prometheus_output(
            r#"
            http_tracker_core_announce_requests_received_total{server_binding_ip="0.0.0.0",server_binding_port="7171",server_binding_protocol="http"} 2
            http_tracker_core_announce_requests_received_total{server_binding_ip="0.0.0.0",server_binding_port="7070",server_binding_protocol="http"} 1
            "#,
        );

        // code-review: samples are not serialized in the same order as they are created.
        // Should we use a deterministic order?

        assert_eq!(sort_lines(&prometheus_output), sort_lines(&expected_prometheus_output));
    }

    #[test]
    fn it_should_exclude_metrics_without_samples_from_prometheus_format() {
        let mut counters = MetricKindCollection::default();
        let mut gauges = MetricKindCollection::default();

        counters.ensure_metric_exists(&metric_name!("test_counter"));
        gauges.ensure_metric_exists(&metric_name!("test_gauge"));

        let metric_collection = MetricCollection::new(counters, gauges).unwrap();

        let prometheus_output = metric_collection.to_prometheus();

        assert_eq!(prometheus_output, "");
    }

    mod for_counters {

        use pretty_assertions::assert_eq;

        use super::*;
        use crate::label::LabelValue;
        use crate::sample::Sample;

        #[test]
        fn it_should_increase_a_preexistent_counter() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut metric_collection = MetricCollection::new(
                MetricKindCollection::new(vec![Metric::new(
                    metric_name!("test_counter"),
                    SampleCollection::new(vec![Sample::new(Counter::new(0), time, label_set.clone())]).unwrap(),
                )])
                .unwrap(),
                MetricKindCollection::default(),
            )
            .unwrap();

            metric_collection
                .increase_counter(&metric_name!("test_counter"), &label_set, time)
                .unwrap();
            metric_collection
                .increase_counter(&metric_name!("test_counter"), &label_set, time)
                .unwrap();

            assert_eq!(
                metric_collection.get_counter_value(&metric_name!("test_counter"), &label_set),
                Some(Counter::new(2))
            );
        }

        #[test]
        fn it_should_automatically_create_a_counter_when_increasing_if_it_does_not_exist() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut metric_collection =
                MetricCollection::new(MetricKindCollection::default(), MetricKindCollection::default()).unwrap();

            metric_collection
                .increase_counter(&metric_name!("test_counter"), &label_set, time)
                .unwrap();
            metric_collection
                .increase_counter(&metric_name!("test_counter"), &label_set, time)
                .unwrap();

            assert_eq!(
                metric_collection.get_counter_value(&metric_name!("test_counter"), &label_set),
                Some(Counter::new(2))
            );
        }

        #[test]
        fn it_should_allow_making_sure_a_counter_exists_without_increasing_it() {
            let mut metric_collection =
                MetricCollection::new(MetricKindCollection::default(), MetricKindCollection::default()).unwrap();

            metric_collection.ensure_counter_exists(&metric_name!("test_counter"));

            assert!(metric_collection.contains_counter(&metric_name!("test_counter")));
        }

        #[test]
        fn it_should_allow_describing_a_counter_before_using_it() {
            let mut metric_collection =
                MetricCollection::new(MetricKindCollection::default(), MetricKindCollection::default()).unwrap();

            metric_collection.describe_counter(&metric_name!("test_counter"), None, None);

            assert!(metric_collection.contains_counter(&metric_name!("test_counter")));
        }

        #[test]
        fn it_should_not_allow_duplicate_metric_names_when_instantiating() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let result = MetricKindCollection::new(vec![
                Metric::new(
                    metric_name!("test_counter"),
                    SampleCollection::new(vec![Sample::new(Counter::new(0), time, label_set.clone())]).unwrap(),
                ),
                Metric::new(
                    metric_name!("test_counter"),
                    SampleCollection::new(vec![Sample::new(Counter::new(0), time, label_set.clone())]).unwrap(),
                ),
            ]);

            assert!(result.is_err());
        }
    }

    mod for_gauges {

        use pretty_assertions::assert_eq;

        use super::*;
        use crate::label::LabelValue;
        use crate::sample::Sample;

        #[test]
        fn it_should_set_a_preexistent_gauge() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut metric_collection = MetricCollection::new(
                MetricKindCollection::default(),
                MetricKindCollection::new(vec![Metric::new(
                    metric_name!("test_gauge"),
                    SampleCollection::new(vec![Sample::new(Gauge::new(0.0), time, label_set.clone())]).unwrap(),
                )])
                .unwrap(),
            )
            .unwrap();

            metric_collection
                .set_gauge(&metric_name!("test_gauge"), &label_set, 1.0, time)
                .unwrap();

            assert_eq!(
                metric_collection.get_gauge_value(&metric_name!("test_gauge"), &label_set),
                Some(Gauge::new(1.0))
            );
        }

        #[test]
        fn it_should_automatically_create_a_gauge_when_setting_if_it_does_not_exist() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut metric_collection =
                MetricCollection::new(MetricKindCollection::default(), MetricKindCollection::default()).unwrap();

            metric_collection
                .set_gauge(&metric_name!("test_gauge"), &label_set, 1.0, time)
                .unwrap();

            assert_eq!(
                metric_collection.get_gauge_value(&metric_name!("test_gauge"), &label_set),
                Some(Gauge::new(1.0))
            );
        }

        #[test]
        fn it_should_allow_making_sure_a_gauge_exists_without_setting_it() {
            let mut metric_collection =
                MetricCollection::new(MetricKindCollection::default(), MetricKindCollection::default()).unwrap();

            metric_collection.ensure_gauge_exists(&metric_name!("test_gauge"));

            assert!(metric_collection.contains_gauge(&metric_name!("test_gauge")));
        }

        #[test]
        fn it_should_allow_describing_a_gauge_before_using_it() {
            let mut metric_collection =
                MetricCollection::new(MetricKindCollection::default(), MetricKindCollection::default()).unwrap();

            metric_collection.describe_gauge(&metric_name!("test_gauge"), None, None);

            assert!(metric_collection.contains_gauge(&metric_name!("test_gauge")));
        }

        #[test]
        fn it_should_not_allow_duplicate_metric_names_when_instantiating() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let result = MetricKindCollection::new(vec![
                Metric::new(
                    metric_name!("test_gauge"),
                    SampleCollection::new(vec![Sample::new(Gauge::new(0.0), time, label_set.clone())]).unwrap(),
                ),
                Metric::new(
                    metric_name!("test_gauge"),
                    SampleCollection::new(vec![Sample::new(Gauge::new(0.0), time, label_set.clone())]).unwrap(),
                ),
            ]);

            assert!(result.is_err());
        }
    }
}
