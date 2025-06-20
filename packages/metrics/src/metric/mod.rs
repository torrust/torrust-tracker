pub mod aggregate;
pub mod description;
pub mod name;

use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::counter::Counter;
use super::label::LabelSet;
use super::prometheus::PrometheusSerializable;
use super::sample_collection::SampleCollection;
use crate::gauge::Gauge;
use crate::metric::description::MetricDescription;
use crate::sample::Measurement;
use crate::unit::Unit;

pub type MetricName = name::MetricName;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Metric<T> {
    name: MetricName,

    #[serde(rename = "unit")]
    opt_unit: Option<Unit>,

    #[serde(rename = "description")]
    opt_description: Option<MetricDescription>,

    #[serde(rename = "samples")]
    sample_collection: SampleCollection<T>,
}

impl<T> Metric<T> {
    #[must_use]
    pub fn new(
        name: MetricName,
        opt_unit: Option<Unit>,
        opt_description: Option<MetricDescription>,
        samples: SampleCollection<T>,
    ) -> Self {
        Self {
            name,
            opt_unit,
            opt_description,
            sample_collection: samples,
        }
    }

    /// # Panics
    ///
    /// This function will panic if the empty sample collection cannot be created.
    #[must_use]
    pub fn new_empty_with_name(name: MetricName) -> Self {
        Self {
            name,
            opt_unit: None,
            opt_description: None,
            sample_collection: SampleCollection::new(vec![]).expect("Empty sample collection creation should not fail"),
        }
    }

    #[must_use]
    pub fn name(&self) -> &MetricName {
        &self.name
    }

    #[must_use]
    pub fn get_sample_data(&self, label_set: &LabelSet) -> Option<&Measurement<T>> {
        self.sample_collection.get(label_set)
    }

    #[must_use]
    pub fn number_of_samples(&self) -> usize {
        self.sample_collection.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sample_collection.is_empty()
    }

    #[must_use]
    pub fn collect_matching_samples(
        &self,
        label_set_criteria: &LabelSet,
    ) -> Vec<(&crate::label::LabelSet, &crate::sample::Measurement<T>)> {
        self.sample_collection
            .iter()
            .filter(|(label_set, _measurement)| label_set.matches(label_set_criteria))
            .collect()
    }
}

impl Metric<Counter> {
    pub fn increment(&mut self, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        self.sample_collection.increment(label_set, time);
    }

    pub fn absolute(&mut self, label_set: &LabelSet, value: u64, time: DurationSinceUnixEpoch) {
        self.sample_collection.absolute(label_set, value, time);
    }
}

impl Metric<Gauge> {
    pub fn set(&mut self, label_set: &LabelSet, value: f64, time: DurationSinceUnixEpoch) {
        self.sample_collection.set(label_set, value, time);
    }

    pub fn increment(&mut self, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        self.sample_collection.increment(label_set, time);
    }

    pub fn decrement(&mut self, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        self.sample_collection.decrement(label_set, time);
    }
}

enum PrometheusType {
    Counter,
    Gauge,
}

impl PrometheusSerializable for PrometheusType {
    fn to_prometheus(&self) -> String {
        match self {
            PrometheusType::Counter => "counter".to_string(),
            PrometheusType::Gauge => "gauge".to_string(),
        }
    }
}

impl<T: PrometheusSerializable> Metric<T> {
    #[must_use]
    fn prometheus_help_line(&self) -> String {
        if let Some(description) = &self.opt_description {
            format!("# HELP {} {}", self.name.to_prometheus(), description.to_prometheus())
        } else {
            String::new()
        }
    }

    #[must_use]
    fn prometheus_type_line(&self, prometheus_type: &PrometheusType) -> String {
        format!("# TYPE {} {}", self.name.to_prometheus(), prometheus_type.to_prometheus())
    }

    #[must_use]
    fn prometheus_sample_line(&self, label_set: &LabelSet, measurement: &Measurement<T>) -> String {
        format!(
            "{}{} {}",
            self.name.to_prometheus(),
            label_set.to_prometheus(),
            measurement.to_prometheus()
        )
    }

    #[must_use]
    fn prometheus_samples(&self) -> String {
        self.sample_collection
            .iter()
            .map(|(label_set, measurement)| self.prometheus_sample_line(label_set, measurement))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn to_prometheus(&self, prometheus_type: &PrometheusType) -> String {
        let help_line = self.prometheus_help_line();
        let type_line = self.prometheus_type_line(prometheus_type);
        let samples = self.prometheus_samples();

        format!("{help_line}\n{type_line}\n{samples}")
    }
}

impl PrometheusSerializable for Metric<Counter> {
    fn to_prometheus(&self) -> String {
        self.to_prometheus(&PrometheusType::Counter)
    }
}

impl PrometheusSerializable for Metric<Gauge> {
    fn to_prometheus(&self) -> String {
        self.to_prometheus(&PrometheusType::Gauge)
    }
}

#[cfg(test)]
mod tests {
    mod for_generic_metrics {
        use super::super::*;
        use crate::gauge::Gauge;
        use crate::label::LabelValue;
        use crate::sample::Sample;
        use crate::{label_name, metric_name};

        #[test]
        fn it_should_be_empty_when_it_does_not_have_any_sample() {
            let name = metric_name!("test_metric");

            let samples = SampleCollection::<Gauge>::default();

            let metric = Metric::<Gauge>::new(name.clone(), None, None, samples);

            assert!(metric.is_empty());
        }

        fn counter_metric_with_one_sample() -> Metric<Counter> {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);

            let name = metric_name!("test_metric");

            let label_set: LabelSet = [(label_name!("server_binding_protocol"), LabelValue::new("http"))].into();

            let samples = SampleCollection::new(vec![Sample::new(Counter::new(1), time, label_set.clone())]).unwrap();

            Metric::<Counter>::new(name.clone(), None, None, samples)
        }

        #[test]
        fn it_should_return_the_number_of_samples() {
            assert_eq!(counter_metric_with_one_sample().number_of_samples(), 1);
        }

        #[test]
        fn it_should_return_zero_number_of_samples_for_an_empty_metric() {
            let name = metric_name!("test_metric");

            let samples = SampleCollection::<Gauge>::default();

            let metric = Metric::<Gauge>::new(name.clone(), None, None, samples);

            assert_eq!(metric.number_of_samples(), 0);
        }
    }

    mod for_counter_metrics {
        use super::super::*;
        use crate::counter::Counter;
        use crate::label::LabelValue;
        use crate::sample::Sample;
        use crate::{label_name, metric_name};

        #[test]
        fn it_should_be_created_from_its_name_and_a_collection_of_samples() {
            let name = metric_name!("test_metric");

            let samples = SampleCollection::<Counter>::default();

            let _metric = Metric::<Counter>::new(name, None, None, samples);
        }

        #[test]
        fn it_should_allow_incrementing_a_sample() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let name = metric_name!("test_metric");
            let label_set: LabelSet = [(label_name!("server_binding_protocol"), LabelValue::new("http"))].into();
            let samples = SampleCollection::new(vec![Sample::new(Counter::new(0), time, label_set.clone())]).unwrap();
            let mut metric = Metric::<Counter>::new(name.clone(), None, None, samples);

            metric.increment(&label_set, time);

            assert_eq!(metric.get_sample_data(&label_set).unwrap().value().value(), 1);
        }

        #[test]
        fn it_should_allow_setting_to_an_absolute_value() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let name = metric_name!("test_metric");
            let label_set: LabelSet = [(label_name!("server_binding_protocol"), LabelValue::new("http"))].into();
            let samples = SampleCollection::new(vec![Sample::new(Counter::new(0), time, label_set.clone())]).unwrap();
            let mut metric = Metric::<Counter>::new(name.clone(), None, None, samples);

            metric.absolute(&label_set, 1, time);

            assert_eq!(metric.get_sample_data(&label_set).unwrap().value().value(), 1);
        }
    }

    mod for_gauge_metrics {
        use approx::assert_relative_eq;

        use super::super::*;
        use crate::gauge::Gauge;
        use crate::label::LabelValue;
        use crate::sample::Sample;
        use crate::{label_name, metric_name};

        #[test]
        fn it_should_be_created_from_its_name_and_a_collection_of_samples() {
            let name = metric_name!("test_metric");

            let samples = SampleCollection::<Gauge>::default();

            let _metric = Metric::<Gauge>::new(name, None, None, samples);
        }

        #[test]
        fn it_should_allow_incrementing_a_sample() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let name = metric_name!("test_metric");
            let label_set: LabelSet = [(label_name!("server_binding_protocol"), LabelValue::new("http"))].into();
            let samples = SampleCollection::new(vec![Sample::new(Gauge::new(0.0), time, label_set.clone())]).unwrap();
            let mut metric = Metric::<Gauge>::new(name.clone(), None, None, samples);

            metric.increment(&label_set, time);

            assert_relative_eq!(metric.get_sample_data(&label_set).unwrap().value().value(), 1.0);
        }

        #[test]
        fn it_should_allow_decrement_a_sample() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let name = metric_name!("test_metric");
            let label_set: LabelSet = [(label_name!("server_binding_protocol"), LabelValue::new("http"))].into();
            let samples = SampleCollection::new(vec![Sample::new(Gauge::new(1.0), time, label_set.clone())]).unwrap();
            let mut metric = Metric::<Gauge>::new(name.clone(), None, None, samples);

            metric.decrement(&label_set, time);

            assert_relative_eq!(metric.get_sample_data(&label_set).unwrap().value().value(), 0.0);
        }

        #[test]
        fn it_should_allow_setting_a_sample() {
            let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);
            let name = metric_name!("test_metric");
            let label_set: LabelSet = [(label_name!("server_binding_protocol"), LabelValue::new("http"))].into();
            let samples = SampleCollection::new(vec![Sample::new(Gauge::new(0.0), time, label_set.clone())]).unwrap();
            let mut metric = Metric::<Gauge>::new(name.clone(), None, None, samples);

            metric.set(&label_set, 1.0, time);

            assert_relative_eq!(metric.get_sample_data(&label_set).unwrap().value().value(), 1.0);
        }
    }

    mod for_prometheus_serialization {
        use super::super::*;
        use crate::counter::Counter;
        use crate::metric_name;

        #[test]
        fn it_should_return_empty_string_for_prometheus_help_line_when_description_is_none() {
            let name = metric_name!("test_metric");
            let samples = SampleCollection::<Counter>::default();
            let metric = Metric::<Counter>::new(name, None, None, samples);

            let help_line = metric.prometheus_help_line();

            assert_eq!(help_line, String::new());
        }

        #[test]
        fn it_should_return_formatted_help_line_for_prometheus_when_description_is_some() {
            let name = metric_name!("test_metric");
            let description = MetricDescription::new("This is a test metric description");
            let samples = SampleCollection::<Counter>::default();
            let metric = Metric::<Counter>::new(name, None, Some(description), samples);

            let help_line = metric.prometheus_help_line();

            assert_eq!(help_line, "# HELP test_metric This is a test metric description");
        }
    }
}
