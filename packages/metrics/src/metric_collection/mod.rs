pub mod aggregate;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Deserializer, Serialize};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::counter::Counter;
use super::gauge::Gauge;
use super::label::LabelSet;
use super::metric::{Metric, MetricName};
use super::prometheus::{PrometheusDeserializable, PrometheusDeserializationError, PrometheusSerializable};
use crate::metric::description::MetricDescription;
use crate::sample::Sample;
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
        self.check_cross_type_collision(other)?;
        self.counters.merge(&other.counters)?;
        self.gauges.merge(&other.gauges)?;
        Ok(())
    }

    /// Returns a set of all metric names in this collection.
    fn collect_names(&self) -> HashSet<MetricName> {
        self.counters.names().chain(self.gauges.names()).cloned().collect()
    }

    /// Checks for name collisions between this collection and another one.
    fn check_cross_type_collision(&self, other: &Self) -> Result<(), Error> {
        let self_names: HashSet<_> = self.collect_names();
        let other_names: HashSet<_> = other.collect_names();

        let cross_type_collisions = self_names.intersection(&other_names).next();

        if let Some(name) = cross_type_collisions {
            return Err(Error::MetricNameCollisionInMerge {
                metric_name: (*name).clone(),
            });
        }

        Ok(())
    }

    // Counter-specific methods

    pub fn describe_counter(&mut self, name: &MetricName, opt_unit: Option<Unit>, opt_description: Option<MetricDescription>) {
        tracing::info!(target: METRICS_TARGET, type = "counter", name = name.to_string(), unit = ?opt_unit, description = ?opt_description);

        let metric = Metric::<Counter>::new(name.clone(), opt_unit, opt_description, SampleCollection::default());

        self.counters.insert(metric);
    }

    #[must_use]
    pub fn contains_counter(&self, name: &MetricName) -> bool {
        self.counters.metrics.contains_key(name)
    }

    #[must_use]
    pub fn get_counter_value(&self, name: &MetricName, label_set: &LabelSet) -> Option<Counter> {
        self.counters.get_value(name, label_set)
    }

    /// Increases the counter for the given metric name and labels.
    ///
    /// # Errors
    ///
    /// Return an error if a metrics of a different type with the same name
    /// already exists.
    pub fn increment_counter(
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

    /// Sets the counter for the given metric name and labels.
    ///
    /// # Errors
    ///
    /// Return an error if a metrics of a different type with the same name
    /// already exists.
    pub fn set_counter(
        &mut self,
        name: &MetricName,
        label_set: &LabelSet,
        value: u64,
        time: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        if self.gauges.metrics.contains_key(name) {
            return Err(Error::MetricNameCollisionAdding {
                metric_name: name.clone(),
            });
        }

        self.counters.absolute(name, label_set, value, time);

        Ok(())
    }

    // Gauge-specific methods

    pub fn describe_gauge(&mut self, name: &MetricName, opt_unit: Option<Unit>, opt_description: Option<MetricDescription>) {
        tracing::info!(target: METRICS_TARGET, type = "gauge", name = name.to_string(), unit = ?opt_unit, description = ?opt_description);

        let metric = Metric::<Gauge>::new(name.clone(), opt_unit, opt_description, SampleCollection::default());

        self.gauges.insert(metric);
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
    pub fn increment_gauge(
        &mut self,
        name: &MetricName,
        label_set: &LabelSet,
        time: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
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
    pub fn decrement_gauge(
        &mut self,
        name: &MetricName,
        label_set: &LabelSet,
        time: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        if self.counters.metrics.contains_key(name) {
            return Err(Error::MetricNameCollisionAdding {
                metric_name: name.clone(),
            });
        }

        self.gauges.decrement(name, label_set, time);

        Ok(())
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
        #[serde(tag = "type", rename_all = "lowercase")]
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
        #[serde(tag = "type", rename_all = "lowercase")]
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
            .join("\n\n")
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

/// Converts a Prometheus timestamp (seconds since Unix epoch as `f64`) to a
/// `DurationSinceUnixEpoch`.
///
/// If `t` is non-finite or negative, `fallback` is returned instead.
fn parse_prometheus_timestamp(t: f64, fallback: DurationSinceUnixEpoch) -> DurationSinceUnixEpoch {
    const FIRST_UNREPRESENTABLE_U64_AS_F64: f64 = 18_446_744_073_709_551_616.0;

    if t.is_finite() && t >= 0.0 {
        if t.trunc() >= FIRST_UNREPRESENTABLE_U64_AS_F64 {
            return fallback;
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let secs = t.trunc() as u64;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let nanos = ((t - t.trunc()) * 1_000_000_000.0).round() as u32;
        let (secs, nanos) = if nanos >= 1_000_000_000 {
            match secs.checked_add(1) {
                Some(next_secs) => (next_secs, nanos - 1_000_000_000),
                None => return fallback,
            }
        } else {
            (secs, nanos)
        };
        DurationSinceUnixEpoch::new(secs, nanos)
    } else {
        fallback
    }
}

fn collection_error<E: ToString>(error: &E) -> PrometheusDeserializationError {
    PrometheusDeserializationError::CollectionError {
        message: error.to_string(),
    }
}

fn build_sample_collection<T>(samples: Vec<Sample<T>>) -> Result<SampleCollection<T>, PrometheusDeserializationError> {
    SampleCollection::new(samples).map_err(|error| collection_error(&error))
}

fn build_metric_collection(
    counter_metrics: Vec<Metric<Counter>>,
    gauge_metrics: Vec<Metric<Gauge>>,
) -> Result<MetricCollection, PrometheusDeserializationError> {
    let counters = MetricKindCollection::new(counter_metrics).map_err(|error| collection_error(&error))?;
    let gauges = MetricKindCollection::new(gauge_metrics).map_err(|error| collection_error(&error))?;

    MetricCollection::new(counters, gauges).map_err(|error| collection_error(&error))
}

/// Converts an `openmetrics_parser::LabelSet` to our `LabelSet`, remapping
/// any `LabelConversion` error to include the owning `family_name`.
fn convert_openmetrics_label_set(
    family_name: &str,
    parser_label_set: openmetrics_parser::LabelSet<'_>,
) -> Result<LabelSet, PrometheusDeserializationError> {
    LabelSet::try_from(parser_label_set).map_err(|e| match e {
        PrometheusDeserializationError::LabelConversion { message, .. } => PrometheusDeserializationError::LabelConversion {
            metric_name: family_name.to_owned(),
            message,
        },
        other => other,
    })
}

/// Extracts a `Counter` value from a Prometheus sample value.
fn counter_value_from_prom(
    family_name: &str,
    prom_value: &openmetrics_parser::PrometheusValue,
) -> Result<Counter, PrometheusDeserializationError> {
    match prom_value {
        openmetrics_parser::PrometheusValue::Counter(c) => {
            let counter = match c.value {
                openmetrics_parser::MetricNumber::Int(value) => match u64::try_from(value) {
                    Ok(value) => Counter::new(value),
                    Err(_) => {
                        return Err(PrometheusDeserializationError::ValueMismatch {
                            metric_name: family_name.to_owned(),
                            expected_type: "counter (non-negative integer)".to_owned(),
                            actual: c.value.to_string(),
                        });
                    }
                },
                openmetrics_parser::MetricNumber::Float(value)
                    if value.is_finite() && value >= 0.0 && value.fract() == 0.0 && value < 18_446_744_073_709_551_616.0 =>
                {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    Counter::new(value as u64)
                }
                openmetrics_parser::MetricNumber::Float(_) => {
                    return Err(PrometheusDeserializationError::ValueMismatch {
                        metric_name: family_name.to_owned(),
                        expected_type: "counter (non-negative integer)".to_owned(),
                        actual: c.value.to_string(),
                    });
                }
            };

            Ok(counter)
        }
        openmetrics_parser::PrometheusValue::Unknown(_) => Err(PrometheusDeserializationError::UnknownValue {
            metric_name: family_name.to_owned(),
        }),
        other => Err(PrometheusDeserializationError::ValueMismatch {
            metric_name: family_name.to_owned(),
            expected_type: "counter".to_owned(),
            actual: format!("{other:?}"),
        }),
    }
}

/// Extracts a `Gauge` value from a Prometheus sample value.
fn gauge_value_from_prom(
    family_name: &str,
    prom_value: &openmetrics_parser::PrometheusValue,
) -> Result<Gauge, PrometheusDeserializationError> {
    match prom_value {
        openmetrics_parser::PrometheusValue::Gauge(n) => Ok(Gauge::new(n.as_f64())),
        openmetrics_parser::PrometheusValue::Unknown(_) => Err(PrometheusDeserializationError::UnknownValue {
            metric_name: family_name.to_owned(),
        }),
        other => Err(PrometheusDeserializationError::ValueMismatch {
            metric_name: family_name.to_owned(),
            expected_type: "gauge".to_owned(),
            actual: format!("{other:?}"),
        }),
    }
}

impl PrometheusDeserializable for MetricCollection {
    fn from_prometheus(input: &str, now: DurationSinceUnixEpoch) -> Result<Self, PrometheusDeserializationError> {
        // The Prometheus text format requires every metric line to end with a
        // newline character. Normalize the input so callers that produce output
        // without a trailing newline (e.g. our own `to_prometheus`) still work.
        let normalized;
        let input = if input.ends_with('\n') {
            input
        } else {
            normalized = format!("{input}\n");
            normalized.as_str()
        };

        let exposition = openmetrics_parser::prometheus::parse_prometheus(input)
            .map_err(|e| PrometheusDeserializationError::ParseError { message: e.to_string() })?;

        let mut counter_metrics: Vec<Metric<Counter>> = Vec::new();
        let mut gauge_metrics: Vec<Metric<Gauge>> = Vec::new();

        for (family_name, family) in &exposition.families {
            let metric_name = MetricName::new(family_name);
            let description = if family.help.is_empty() {
                None
            } else {
                Some(MetricDescription::new(&family.help))
            };

            match family.family_type {
                openmetrics_parser::PrometheusType::Counter => {
                    let label_names = Arc::new(family.get_label_names().to_vec());
                    let mut samples: Vec<Sample<Counter>> = Vec::new();

                    for parser_sample in family.iter_samples() {
                        let parser_label_set = openmetrics_parser::LabelSet::new(Arc::clone(&label_names), parser_sample)
                            .map_err(|e| PrometheusDeserializationError::LabelConversion {
                                metric_name: family_name.clone(),
                                message: e.to_string(),
                            })?;
                        let label_set = convert_openmetrics_label_set(family_name, parser_label_set)?;
                        let value = counter_value_from_prom(family_name, &parser_sample.value)?;
                        let time = parser_sample.timestamp.map_or(now, |t| parse_prometheus_timestamp(t, now));
                        samples.push(Sample::new(value, time, label_set));
                    }

                    let sample_collection = build_sample_collection(samples)?;
                    counter_metrics.push(Metric::new(metric_name, None, description, sample_collection));
                }
                openmetrics_parser::PrometheusType::Gauge => {
                    let label_names = Arc::new(family.get_label_names().to_vec());
                    let mut samples: Vec<Sample<Gauge>> = Vec::new();

                    for parser_sample in family.iter_samples() {
                        let parser_label_set = openmetrics_parser::LabelSet::new(Arc::clone(&label_names), parser_sample)
                            .map_err(|e| PrometheusDeserializationError::LabelConversion {
                                metric_name: family_name.clone(),
                                message: e.to_string(),
                            })?;
                        let label_set = convert_openmetrics_label_set(family_name, parser_label_set)?;
                        let value = gauge_value_from_prom(family_name, &parser_sample.value)?;
                        let time = parser_sample.timestamp.map_or(now, |t| parse_prometheus_timestamp(t, now));
                        samples.push(Sample::new(value, time, label_set));
                    }

                    let sample_collection = build_sample_collection(samples)?;
                    gauge_metrics.push(Metric::new(metric_name, None, description, sample_collection));
                }
                openmetrics_parser::PrometheusType::Histogram | openmetrics_parser::PrometheusType::Summary => {
                    return Err(PrometheusDeserializationError::UnsupportedType {
                        metric_name: family_name.clone(),
                        metric_type: family.family_type.to_string(),
                    });
                }
                openmetrics_parser::PrometheusType::Unknown => {
                    return Err(PrometheusDeserializationError::UnknownType {
                        metric_name: family_name.clone(),
                    });
                }
            }
        }

        build_metric_collection(counter_metrics, gauge_metrics)
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::label::LabelValue;
    use crate::sample::Sample;
    use crate::sample_collection::SampleCollection;
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
                    None,
                    Some(MetricDescription::new("The number of announce requests received.")),
                    SampleCollection::new(vec![Sample::new(Counter::new(1), time, label_set_1.clone())]).unwrap(),
                )])
                .unwrap(),
                MetricKindCollection::new(vec![Metric::new(
                    metric_name!("udp_tracker_server_performance_avg_announce_processing_time_ns"),
                    None,
                    Some(MetricDescription::new("The average announce processing time in nanoseconds.")),
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
                    "type":"counter",
                    "name":"http_tracker_core_announce_requests_received_total",
                    "unit": null,
                    "description": "The number of announce requests received.",
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
                    "type":"gauge",
                    "name":"udp_tracker_server_performance_avg_announce_processing_time_ns",
                    "unit": null,
                    "description": "The average announce processing time in nanoseconds.",
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
                r#"# HELP http_tracker_core_announce_requests_received_total The number of announce requests received.
# TYPE http_tracker_core_announce_requests_received_total counter
http_tracker_core_announce_requests_received_total{server_binding_ip="0.0.0.0",server_binding_port="7070",server_binding_protocol="http"} 1

# HELP udp_tracker_server_performance_avg_announce_processing_time_ns The average announce processing time in nanoseconds.
# TYPE udp_tracker_server_performance_avg_announce_processing_time_ns gauge
udp_tracker_server_performance_avg_announce_processing_time_ns{server_binding_ip="0.0.0.0",server_binding_port="7070",server_binding_protocol="http"} 1
"#,
            )
        }
    }

    #[test]
    fn it_should_not_allow_duplicate_names_across_types() {
        let counters = MetricKindCollection::new(vec![Metric::new(
            metric_name!("test_metric"),
            None,
            None,
            SampleCollection::default(),
        )])
        .unwrap();
        let gauges = MetricKindCollection::new(vec![Metric::new(
            metric_name!("test_metric"),
            None,
            None,
            SampleCollection::default(),
        )])
        .unwrap();

        assert!(MetricCollection::new(counters, gauges).is_err());
    }

    #[test]
    fn it_should_not_allow_creating_a_gauge_with_the_same_name_as_a_counter() {
        let mut collection = MetricCollection::default();
        let label_set = LabelSet::default();
        let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);

        // First create a counter
        collection
            .increment_counter(&metric_name!("test_metric"), &label_set, time)
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
        let result = collection.increment_counter(&metric_name!("test_metric"), &label_set, time);

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
                None,
                Some(MetricDescription::new("The number of announce requests received.")),
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
            r#"# HELP http_tracker_core_announce_requests_received_total The number of announce requests received.
# TYPE http_tracker_core_announce_requests_received_total counter
http_tracker_core_announce_requests_received_total{server_binding_ip="0.0.0.0",server_binding_port="7070",server_binding_protocol="http"} 1
http_tracker_core_announce_requests_received_total{server_binding_ip="0.0.0.0",server_binding_port="7171",server_binding_protocol="http"} 2
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

        let counter = Metric::<Counter>::new_empty_with_name(metric_name!("test_counter"));
        counters.insert_if_absent(counter);

        let gauge = Metric::<Gauge>::new_empty_with_name(metric_name!("test_gauge"));
        gauges.insert_if_absent(gauge);

        let metric_collection = MetricCollection::new(counters, gauges).unwrap();

        let prometheus_output = metric_collection.to_prometheus();

        assert_eq!(prometheus_output, "");
    }

    #[test]
    fn it_should_allow_merging_metric_collections() {
        let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
        let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

        let mut collection1 = MetricCollection::default();
        collection1
            .increment_counter(&metric_name!("test_counter"), &label_set, time)
            .unwrap();

        let mut collection2 = MetricCollection::default();
        collection2
            .set_gauge(&metric_name!("test_gauge"), &label_set, 1.0, time)
            .unwrap();

        collection1.merge(&collection2).unwrap();

        assert!(collection1.contains_counter(&metric_name!("test_counter")));
        assert!(collection1.contains_gauge(&metric_name!("test_gauge")));
    }

    #[test]
    fn it_should_not_allow_merging_metric_collections_with_name_collisions_for_the_same_metric_types() {
        let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
        let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

        let mut collection1 = MetricCollection::default();
        collection1
            .increment_counter(&metric_name!("test_metric"), &label_set, time)
            .unwrap();

        let mut collection2 = MetricCollection::default();
        collection2
            .increment_counter(&metric_name!("test_metric"), &label_set, time)
            .unwrap();
        let result = collection1.merge(&collection2);

        assert!(result.is_err());
    }

    #[test]
    fn it_should_not_allow_merging_metric_collections_with_name_collisions_for_different_metric_types() {
        let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
        let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

        let mut collection1 = MetricCollection::default();
        collection1
            .increment_counter(&metric_name!("test_metric"), &label_set, time)
            .unwrap();

        let mut collection2 = MetricCollection::default();
        collection2
            .set_gauge(&metric_name!("test_metric"), &label_set, 1.0, time)
            .unwrap();

        let result = collection1.merge(&collection2);

        assert!(result.is_err());
    }

    fn collection_with_one_counter(metric_name: &MetricName, label_set: &LabelSet, counter: Counter) -> MetricCollection {
        let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);

        MetricCollection::new(
            MetricKindCollection::new(vec![Metric::new(
                metric_name.clone(),
                None,
                None,
                SampleCollection::new(vec![Sample::new(counter, time, label_set.clone())]).unwrap(),
            )])
            .unwrap(),
            MetricKindCollection::default(),
        )
        .unwrap()
    }

    fn collection_with_one_gauge(metric_name: &MetricName, label_set: &LabelSet, gauge: Gauge) -> MetricCollection {
        let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);

        MetricCollection::new(
            MetricKindCollection::default(),
            MetricKindCollection::new(vec![Metric::new(
                metric_name.clone(),
                None,
                None,
                SampleCollection::new(vec![Sample::new(gauge, time, label_set.clone())]).unwrap(),
            )])
            .unwrap(),
        )
        .unwrap()
    }

    mod for_counters {

        use pretty_assertions::assert_eq;

        use super::*;
        use crate::label::LabelValue;
        use crate::sample::Sample;
        use crate::sample_collection::SampleCollection;

        #[test]
        fn it_should_allow_setting_to_an_absolute_value() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let metric_name = metric_name!("test_counter");
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut collection = collection_with_one_counter(&metric_name, &label_set, Counter::new(0));

            collection
                .set_counter(&metric_name!("test_counter"), &label_set, 1, time)
                .unwrap();

            assert_eq!(
                collection.get_counter_value(&metric_name!("test_counter"), &label_set),
                Some(Counter::new(1))
            );
        }

        #[test]
        fn it_should_fail_setting_to_an_absolute_value_if_a_gauge_with_the_same_name_exists() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let metric_name = metric_name!("test_counter");
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut collection = collection_with_one_gauge(&metric_name, &label_set, Gauge::new(0.0));

            let result = collection.set_counter(&metric_name!("test_counter"), &label_set, 1, time);

            assert!(
                result.is_err()
                    && matches!(result, Err(Error::MetricNameCollisionAdding { metric_name }) if metric_name == metric_name!("test_counter"))
            );
        }

        #[test]
        fn it_should_increase_a_preexistent_counter() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let metric_name = metric_name!("test_counter");
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut collection = collection_with_one_counter(&metric_name, &label_set, Counter::new(0));

            collection
                .increment_counter(&metric_name!("test_counter"), &label_set, time)
                .unwrap();

            assert_eq!(
                collection.get_counter_value(&metric_name!("test_counter"), &label_set),
                Some(Counter::new(1))
            );
        }

        #[test]
        fn it_should_automatically_create_a_counter_when_increasing_if_it_does_not_exist() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut metric_collection =
                MetricCollection::new(MetricKindCollection::default(), MetricKindCollection::default()).unwrap();

            metric_collection
                .increment_counter(&metric_name!("test_counter"), &label_set, time)
                .unwrap();
            metric_collection
                .increment_counter(&metric_name!("test_counter"), &label_set, time)
                .unwrap();

            assert_eq!(
                metric_collection.get_counter_value(&metric_name!("test_counter"), &label_set),
                Some(Counter::new(2))
            );
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
                    None,
                    None,
                    SampleCollection::new(vec![Sample::new(Counter::new(0), time, label_set.clone())]).unwrap(),
                ),
                Metric::new(
                    metric_name!("test_counter"),
                    None,
                    None,
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
        use crate::sample_collection::SampleCollection;

        #[test]
        fn it_should_set_a_preexistent_gauge() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let metric_name = metric_name!("test_gauge");
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut collection = collection_with_one_gauge(&metric_name, &label_set, Gauge::new(0.0));

            collection
                .set_gauge(&metric_name!("test_gauge"), &label_set, 1.0, time)
                .unwrap();

            assert_eq!(
                collection.get_gauge_value(&metric_name!("test_gauge"), &label_set),
                Some(Gauge::new(1.0))
            );
        }

        #[test]
        fn it_should_allow_incrementing_a_gauge() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let metric_name = metric_name!("test_gauge");
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut collection = collection_with_one_gauge(&metric_name, &label_set, Gauge::new(0.0));

            collection
                .increment_gauge(&metric_name!("test_gauge"), &label_set, time)
                .unwrap();

            assert_eq!(
                collection.get_gauge_value(&metric_name!("test_gauge"), &label_set),
                Some(Gauge::new(1.0))
            );
        }

        #[test]
        fn it_should_fail_incrementing_a_gauge_if_it_exists_a_counter_with_the_same_name() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let metric_name = metric_name!("test_gauge");
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut collection = collection_with_one_counter(&metric_name, &label_set, Counter::new(0));

            let result = collection.increment_gauge(&metric_name!("test_gauge"), &label_set, time);

            assert!(
                result.is_err()
                    && matches!(result, Err(Error::MetricNameCollisionAdding { metric_name }) if metric_name == metric_name!("test_gauge"))
            );
        }

        #[test]
        fn it_should_allow_decrementing_a_gauge() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let metric_name = metric_name!("test_gauge");
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut collection = collection_with_one_gauge(&metric_name, &label_set, Gauge::new(1.0));

            collection
                .decrement_gauge(&metric_name!("test_gauge"), &label_set, time)
                .unwrap();

            assert_eq!(
                collection.get_gauge_value(&metric_name!("test_gauge"), &label_set),
                Some(Gauge::new(0.0))
            );
        }

        #[test]
        fn it_should_fail_decrementing_a_gauge_if_it_exists_a_counter_with_the_same_name() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let metric_name = metric_name!("test_gauge");
            let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

            let mut collection = collection_with_one_counter(&metric_name, &label_set, Counter::new(0));

            let result = collection.decrement_gauge(&metric_name!("test_gauge"), &label_set, time);

            assert!(
                result.is_err()
                    && matches!(result, Err(Error::MetricNameCollisionAdding { metric_name }) if metric_name == metric_name!("test_gauge"))
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
                    None,
                    None,
                    SampleCollection::new(vec![Sample::new(Gauge::new(0.0), time, label_set.clone())]).unwrap(),
                ),
                Metric::new(
                    metric_name!("test_gauge"),
                    None,
                    None,
                    SampleCollection::new(vec![Sample::new(Gauge::new(0.0), time, label_set.clone())]).unwrap(),
                ),
            ]);

            assert!(result.is_err());
        }
    }

    mod metric_kind_collection {

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

    mod prometheus_timestamp {
        use approx::assert_abs_diff_eq;
        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use super::super::parse_prometheus_timestamp;

        #[test]
        fn it_should_convert_a_whole_second_timestamp() {
            let now = DurationSinceUnixEpoch::from_secs(0);
            let result = parse_prometheus_timestamp(1_000.0, now);
            assert_eq!(result, DurationSinceUnixEpoch::from_secs(1_000));
        }

        #[test]
        fn it_should_convert_a_fractional_timestamp() {
            let now = DurationSinceUnixEpoch::from_secs(0);
            let result = parse_prometheus_timestamp(1.5, now);
            approx::assert_abs_diff_eq!(result.as_secs_f64(), 1.5, epsilon = 1e-9);
        }

        #[test]
        fn it_should_use_fallback_for_negative_timestamp() {
            let fallback = DurationSinceUnixEpoch::from_secs(42);
            let result = parse_prometheus_timestamp(-1.0, fallback);
            assert_eq!(result, fallback);
        }

        #[test]
        fn it_should_use_fallback_for_nan() {
            let fallback = DurationSinceUnixEpoch::from_secs(42);
            let result = parse_prometheus_timestamp(f64::NAN, fallback);
            assert_eq!(result, fallback);
        }

        #[test]
        fn it_should_use_fallback_for_positive_infinity() {
            let fallback = DurationSinceUnixEpoch::from_secs(42);
            let result = parse_prometheus_timestamp(f64::INFINITY, fallback);
            assert_eq!(result, fallback);
        }

        #[test]
        fn it_should_use_fallback_for_negative_infinity() {
            let fallback = DurationSinceUnixEpoch::from_secs(42);
            let result = parse_prometheus_timestamp(f64::NEG_INFINITY, fallback);
            assert_eq!(result, fallback);
        }

        #[test]
        fn it_should_use_fallback_when_timestamp_would_overflow_u64_seconds() {
            const FIRST_UNREPRESENTABLE_U64_AS_F64: f64 = 18_446_744_073_709_551_616.0;
            let fallback = DurationSinceUnixEpoch::from_secs(42);
            let result = parse_prometheus_timestamp(FIRST_UNREPRESENTABLE_U64_AS_F64, fallback);
            assert_eq!(result, fallback);
        }

        #[test]
        fn it_should_handle_nanosecond_boundary_overflow() {
            let now = DurationSinceUnixEpoch::from_secs(0);
            // 1 second + fractional part that rounds to exactly 1_000_000_000 nanos
            // 0.9999999995 * 1e9 rounds to 1_000_000_000, triggering carry
            let result = parse_prometheus_timestamp(1.999_999_999_5, now);
            assert_abs_diff_eq!(result.as_secs_f64(), 2.0, epsilon = 1e-3);
        }

        #[test]
        fn it_should_convert_zero_timestamp() {
            let fallback = DurationSinceUnixEpoch::from_secs(99);
            let result = parse_prometheus_timestamp(0.0, fallback);
            assert_eq!(result, DurationSinceUnixEpoch::from_secs(0));
        }
    }

    mod prometheus_deserialization {
        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use super::super::MetricCollection;
        use super::{Counter, Gauge, LabelValue, Metric, MetricDescription, MetricKindCollection, Sample, SampleCollection};
        use crate::prometheus::{PrometheusDeserializable, PrometheusDeserializationError, PrometheusSerializable};
        use crate::{label_name, metric_name};

        #[test]
        fn it_should_deserialize_a_counter_metric_from_prometheus_text() {
            let now = DurationSinceUnixEpoch::from_secs(1_000);
            let input = "# HELP requests_total The total number of requests.\n# TYPE requests_total counter\nrequests_total{method=\"get\"} 42\n";

            let result = MetricCollection::from_prometheus(input, now).expect("should parse successfully");

            let label_set = [(label_name!("method"), LabelValue::new("get"))].into();

            let expected_value = result
                .get_counter_value(&metric_name!("requests_total"), &label_set)
                .expect("counter should be present");

            assert_eq!(expected_value, Counter::new(42));
        }

        #[test]
        fn it_should_deserialize_a_gauge_metric_from_prometheus_text() {
            let now = DurationSinceUnixEpoch::from_secs(1_000);
            let input = "# HELP temperature Current temperature.\n# TYPE temperature gauge\ntemperature{room=\"kitchen\"} 21.5\n";

            let result = MetricCollection::from_prometheus(input, now).expect("should parse successfully");

            let label_set = [(label_name!("room"), LabelValue::new("kitchen"))].into();

            let expected_value = result
                .get_gauge_value(&metric_name!("temperature"), &label_set)
                .expect("gauge should be present");

            assert_eq!(expected_value, Gauge::new(21.5));
        }

        #[test]
        fn it_should_round_trip_serialize_then_deserialize_prometheus_text() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);

            let label_set_1 = [
                (label_name!("server_binding_protocol"), LabelValue::new("http")),
                (label_name!("server_binding_ip"), LabelValue::new("0.0.0.0")),
                (label_name!("server_binding_port"), LabelValue::new("7070")),
            ]
            .into();

            let original = MetricCollection::new(
                MetricKindCollection::new(vec![Metric::new(
                    metric_name!("http_tracker_core_announce_requests_received_total"),
                    None,
                    Some(MetricDescription::new("The number of announce requests received.")),
                    SampleCollection::new(vec![Sample::new(Counter::new(1), time, label_set_1)]).unwrap(),
                )])
                .unwrap(),
                MetricKindCollection::default(),
            )
            .unwrap();

            let prometheus_text = original.to_prometheus();
            let deserialized =
                MetricCollection::from_prometheus(&prometheus_text, time).expect("round-trip deserialization should succeed");

            assert_eq!(original, deserialized);
        }

        #[test]
        fn it_should_return_unsupported_type_for_histogram() {
            let now = DurationSinceUnixEpoch::from_secs(0);
            let input = "# TYPE latency histogram\nlatency_bucket{le=\"0.1\"} 5\nlatency_bucket{le=\"+Inf\"} 10\nlatency_sum 1.5\nlatency_count 10\n";

            let result = MetricCollection::from_prometheus(input, now);

            assert!(matches!(result, Err(PrometheusDeserializationError::UnsupportedType { .. })));
        }

        #[test]
        fn it_should_return_parse_error_for_malformed_input() {
            let now = DurationSinceUnixEpoch::from_secs(0);
            // An invalid TYPE declaration (missing type name) causes a parse error
            let input = "# TYPE\n";

            let result = MetricCollection::from_prometheus(input, now);

            assert!(matches!(result, Err(PrometheusDeserializationError::ParseError { .. })));
        }

        #[test]
        fn it_should_use_fallback_timestamp_when_sample_has_no_timestamp() {
            let now = DurationSinceUnixEpoch::from_secs(9_999);
            let input = "# TYPE hits_total counter\nhits_total 7\n";

            let result = MetricCollection::from_prometheus(input, now).expect("should parse");

            let label_set = super::super::LabelSet::empty();
            let value = result
                .get_counter_value(&metric_name!("hits_total"), &label_set)
                .expect("counter should be present");

            assert_eq!(value, Counter::new(7));
        }

        #[test]
        fn it_should_reject_fractional_counter_values() {
            let now = DurationSinceUnixEpoch::from_secs(1_000);
            let input = "# TYPE requests_total counter\nrequests_total 42.5\n";

            let result = MetricCollection::from_prometheus(input, now);

            assert!(matches!(result, Err(PrometheusDeserializationError::ValueMismatch { .. })));
        }

        #[test]
        fn it_should_classify_duplicate_metric_names_as_collection_errors() {
            let label_set = super::super::LabelSet::empty();
            let time = DurationSinceUnixEpoch::from_secs(1_000);
            let counter_metrics = vec![
                Metric::new(
                    metric_name!("requests_total"),
                    None,
                    None,
                    SampleCollection::new(vec![Sample::new(Counter::new(1), time, label_set.clone())]).unwrap(),
                ),
                Metric::new(
                    metric_name!("requests_total"),
                    None,
                    None,
                    SampleCollection::new(vec![Sample::new(Counter::new(2), time, label_set)]).unwrap(),
                ),
            ];

            let result = super::super::build_metric_collection(counter_metrics, Vec::new());

            assert!(matches!(result, Err(PrometheusDeserializationError::CollectionError { .. })));
        }
    }
}
