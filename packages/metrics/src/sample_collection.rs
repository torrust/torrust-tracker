use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt::Write as _;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::counter::Counter;
use super::gauge::Gauge;
use super::label::LabelSet;
use super::prometheus::PrometheusSerializable;
use super::sample::Sample;
use crate::sample::Measurement;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SampleCollection<T> {
    samples: HashMap<LabelSet, Measurement<T>>,
}

impl<T> SampleCollection<T> {
    /// Creates a new `MetricKindCollection` from a vector of metrics
    ///
    /// # Errors
    ///
    /// Returns an error if there are duplicate `LabelSets` in the provided
    /// samples.
    pub fn new(samples: Vec<Sample<T>>) -> Result<Self, Error> {
        let mut map: HashMap<LabelSet, Measurement<T>> = HashMap::with_capacity(samples.len());

        for sample in samples {
            let (label_set, sample_data): (LabelSet, Measurement<T>) = sample.into();

            let label_set_clone = label_set.clone();

            if let Some(_old_measurement) = map.insert(label_set, sample_data) {
                return Err(Error::DuplicateLabelSetInList {
                    label_set: label_set_clone,
                });
            }
        }

        Ok(Self { samples: map })
    }

    #[must_use]
    pub fn get(&self, label: &LabelSet) -> Option<&Measurement<T>> {
        self.samples.get(label)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&self) -> Iter<'_, LabelSet, Measurement<T>> {
        self.samples.iter()
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Found duplicate label set in list. Label set must be unique in a SampleCollection.")]
    DuplicateLabelSetInList { label_set: LabelSet },
}

impl SampleCollection<Counter> {
    pub fn increment(&mut self, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        let sample = self
            .samples
            .entry(label_set.clone())
            .or_insert_with(|| Measurement::new(Counter::default(), time));

        sample.increment(time);
    }

    pub fn absolute(&mut self, label_set: &LabelSet, value: u64, time: DurationSinceUnixEpoch) {
        let sample = self
            .samples
            .entry(label_set.clone())
            .or_insert_with(|| Measurement::new(Counter::default(), time));

        sample.absolute(value, time);
    }
}

impl SampleCollection<Gauge> {
    pub fn set(&mut self, label_set: &LabelSet, value: f64, time: DurationSinceUnixEpoch) {
        let sample = self
            .samples
            .entry(label_set.clone())
            .or_insert_with(|| Measurement::new(Gauge::default(), time));

        sample.set(value, time);
    }

    pub fn increment(&mut self, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        let sample = self
            .samples
            .entry(label_set.clone())
            .or_insert_with(|| Measurement::new(Gauge::default(), time));

        sample.increment(time);
    }

    pub fn decrement(&mut self, label_set: &LabelSet, time: DurationSinceUnixEpoch) {
        let sample = self
            .samples
            .entry(label_set.clone())
            .or_insert_with(|| Measurement::new(Gauge::default(), time));

        sample.decrement(time);
    }
}

impl<T: Serialize> Serialize for SampleCollection<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut samples: Vec<Sample<&T>> = vec![];

        for (label_set, sample_data) in &self.samples {
            samples.push(Sample::new(sample_data.value(), sample_data.recorded_at(), label_set.clone()));
        }

        samples.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for SampleCollection<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let samples = Vec::<Sample<T>>::deserialize(deserializer)?;

        let sample_collection = SampleCollection::new(samples).map_err(serde::de::Error::custom)?;

        Ok(sample_collection)
    }
}

impl<T: PrometheusSerializable> PrometheusSerializable for SampleCollection<T> {
    fn to_prometheus(&self) -> String {
        let mut output = String::new();

        for (label_set, sample_data) in &self.samples {
            if label_set.is_empty() {
                let _ = write!(output, "{}", sample_data.to_prometheus());
            } else {
                let _ = write!(output, "{} {}", label_set.to_prometheus(), sample_data.to_prometheus());
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use torrust_tracker_primitives::DurationSinceUnixEpoch;

    use crate::counter::Counter;
    use crate::label::LabelSet;
    use crate::sample::Sample;
    use crate::sample_collection::SampleCollection;

    fn sample_update_time() -> DurationSinceUnixEpoch {
        DurationSinceUnixEpoch::from_secs(1_743_552_000)
    }

    #[test]
    fn it_should_fail_trying_to_create_a_sample_collection_with_duplicate_label_sets() {
        let samples = vec![
            Sample::new(Counter::default(), sample_update_time(), LabelSet::default()),
            Sample::new(Counter::default(), sample_update_time(), LabelSet::default()),
        ];

        let result = SampleCollection::new(samples);

        assert!(result.is_err());
    }

    #[test]
    fn it_should_return_a_sample_searching_by_label_set_with_one_empty_label_set() {
        let label_set = LabelSet::default();

        let sample = Sample::new(Counter::default(), sample_update_time(), label_set.clone());

        let collection = SampleCollection::new(vec![sample.clone()]).unwrap();

        let retrieved = collection.get(&label_set);

        assert_eq!(retrieved.unwrap(), sample.measurement());
    }

    #[test]
    fn it_should_return_a_sample_searching_by_label_set_with_two_label_sets() {
        let label_set_1 = LabelSet::from(vec![("label_name_1", "label value 1")]);
        let label_set_2 = LabelSet::from(vec![("label_name_2", "label value 2")]);

        let sample_1 = Sample::new(Counter::new(1), sample_update_time(), label_set_1.clone());
        let sample_2 = Sample::new(Counter::new(2), sample_update_time(), label_set_2.clone());

        let collection = SampleCollection::new(vec![sample_1.clone(), sample_2.clone()]).unwrap();

        let retrieved = collection.get(&label_set_1);
        assert_eq!(retrieved.unwrap(), sample_1.measurement());

        let retrieved = collection.get(&label_set_2);
        assert_eq!(retrieved.unwrap(), sample_2.measurement());
    }

    #[test]
    fn it_should_return_the_number_of_samples_in_the_collection() {
        let samples = vec![Sample::new(Counter::default(), sample_update_time(), LabelSet::default())];
        let collection = SampleCollection::new(samples).unwrap();
        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn it_should_return_zero_number_of_samples_when_empty() {
        let empty = SampleCollection::<Counter>::default();
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn it_should_indicate_is_it_is_empty() {
        let empty = SampleCollection::<Counter>::default();
        assert!(empty.is_empty());

        let samples = vec![Sample::new(Counter::default(), sample_update_time(), LabelSet::default())];
        let collection = SampleCollection::new(samples).unwrap();
        assert!(!collection.is_empty());
    }

    mod json_serialization {
        use crate::counter::Counter;
        use crate::label::LabelSet;
        use crate::sample::Sample;
        use crate::sample_collection::tests::sample_update_time;
        use crate::sample_collection::SampleCollection;

        #[test]
        fn it_should_be_serializable_and_deserializable_for_json_format() {
            let sample = Sample::new(Counter::default(), sample_update_time(), LabelSet::default());
            let collection = SampleCollection::new(vec![sample]).unwrap();

            let serialized = serde_json::to_string(&collection).unwrap();
            let deserialized: SampleCollection<Counter> = serde_json::from_str(&serialized).unwrap();

            assert_eq!(deserialized, collection);
        }

        #[test]
        fn it_should_fail_deserializing_from_json_with_duplicate_label_sets() {
            let samples = vec![
                Sample::new(Counter::default(), sample_update_time(), LabelSet::default()),
                Sample::new(Counter::default(), sample_update_time(), LabelSet::default()),
            ];

            let serialized = serde_json::to_string(&samples).unwrap();

            let result: Result<SampleCollection<Counter>, _> = serde_json::from_str(&serialized);

            assert!(result.is_err());
        }
    }

    mod prometheus_serialization {
        use crate::counter::Counter;
        use crate::label::LabelSet;
        use crate::prometheus::PrometheusSerializable;
        use crate::sample::Sample;
        use crate::sample_collection::tests::sample_update_time;
        use crate::sample_collection::SampleCollection;
        use crate::tests::format_prometheus_output;

        #[test]
        fn it_should_be_exportable_to_prometheus_format_when_empty() {
            let sample = Sample::new(Counter::default(), sample_update_time(), LabelSet::default());
            let collection = SampleCollection::new(vec![sample]).unwrap();

            let prometheus_output = collection.to_prometheus();

            assert!(!prometheus_output.is_empty());
        }

        #[test]
        fn it_should_be_exportable_to_prometheus_format() {
            let sample = Sample::new(
                Counter::new(1),
                sample_update_time(),
                LabelSet::from(vec![("labe_name_1", "label value value 1")]),
            );

            let collection = SampleCollection::new(vec![sample]).unwrap();

            let prometheus_output = collection.to_prometheus();

            let expected_prometheus_output = format_prometheus_output("{labe_name_1=\"label value value 1\"} 1");

            assert_eq!(prometheus_output, expected_prometheus_output);
        }
    }

    #[cfg(test)]
    mod for_counters {

        use std::ops::Add;

        use super::super::LabelSet;
        use super::*;

        #[test]
        fn it_should_increment_the_counter_for_a_preexisting_label_set() {
            let label_set = LabelSet::default();
            let mut collection = SampleCollection::<Counter>::default();

            // Initialize the sample
            collection.increment(&label_set, sample_update_time());

            // Verify initial state
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(sample.value(), &Counter::new(1));

            // Increment again
            collection.increment(&label_set, sample_update_time());
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(*sample.value(), Counter::new(2));
        }

        #[test]
        fn it_should_allow_increment_the_counter_for_a_non_existent_label_set() {
            let label_set = LabelSet::default();
            let mut collection = SampleCollection::<Counter>::default();

            // Increment a non-existent label
            collection.increment(&label_set, sample_update_time());

            // Verify the label exists
            assert!(collection.get(&label_set).is_some());
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(*sample.value(), Counter::new(1));
        }

        #[test]
        fn it_should_update_the_latest_update_time_when_incremented() {
            let label_set = LabelSet::default();
            let initial_time = sample_update_time();

            let mut collection = SampleCollection::<Counter>::default();
            collection.increment(&label_set, initial_time);

            // Increment with a new time
            let new_time = initial_time.add(DurationSinceUnixEpoch::from_secs(1));
            collection.increment(&label_set, new_time);

            let sample = collection.get(&label_set).unwrap();
            assert_eq!(sample.recorded_at(), new_time);
            assert_eq!(*sample.value(), Counter::new(2));
        }

        #[test]
        fn it_should_increment_the_counter_for_multiple_labels() {
            let label1 = LabelSet::from([("name", "value1")]);
            let label2 = LabelSet::from([("name", "value2")]);
            let now = sample_update_time();

            let mut collection = SampleCollection::<Counter>::default();

            collection.increment(&label1, now);
            collection.increment(&label2, now);

            assert_eq!(collection.get(&label1).unwrap().value(), &Counter::new(1));
            assert_eq!(collection.get(&label2).unwrap().value(), &Counter::new(1));
            assert_eq!(collection.len(), 2);
        }

        #[test]
        fn it_should_allow_setting_absolute_value_for_a_counter() {
            let label_set = LabelSet::default();
            let mut collection = SampleCollection::<Counter>::default();

            // Set absolute value for a non-existent label
            collection.absolute(&label_set, 42, sample_update_time());

            // Verify the label exists and has the absolute value
            assert!(collection.get(&label_set).is_some());
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(*sample.value(), Counter::new(42));
        }

        #[test]
        fn it_should_allow_setting_absolute_value_for_existing_counter() {
            let label_set = LabelSet::default();
            let mut collection = SampleCollection::<Counter>::default();

            // Initialize the sample with increment
            collection.increment(&label_set, sample_update_time());

            // Verify initial state
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(sample.value(), &Counter::new(1));

            // Set absolute value
            collection.absolute(&label_set, 100, sample_update_time());
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(*sample.value(), Counter::new(100));
        }

        #[test]
        fn it_should_update_time_when_setting_absolute_value() {
            let label_set = LabelSet::default();
            let initial_time = sample_update_time();
            let mut collection = SampleCollection::<Counter>::default();

            // Set absolute value with initial time
            collection.absolute(&label_set, 50, initial_time);

            // Set absolute value with a new time
            let new_time = initial_time.add(DurationSinceUnixEpoch::from_secs(1));
            collection.absolute(&label_set, 75, new_time);

            let sample = collection.get(&label_set).unwrap();
            assert_eq!(sample.recorded_at(), new_time);
            assert_eq!(*sample.value(), Counter::new(75));
        }
    }

    #[cfg(test)]
    mod for_gauges {

        use std::ops::Add;

        use super::super::LabelSet;
        use super::*;
        use crate::gauge::Gauge;

        #[test]
        fn it_should_allow_setting_the_gauge_for_a_preexisting_label_set() {
            let label_set = LabelSet::default();
            let mut collection = SampleCollection::<Gauge>::default();

            // Initialize the sample
            collection.set(&label_set, 1.0, sample_update_time());

            // Verify initial state
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(sample.value(), &Gauge::new(1.0));

            // Set again
            collection.set(&label_set, 2.0, sample_update_time());
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(*sample.value(), Gauge::new(2.0));
        }

        #[test]
        fn it_should_allow_setting_the_gauge_for_a_non_existent_label_set() {
            let label_set = LabelSet::default();
            let mut collection = SampleCollection::<Gauge>::default();

            // Set a non-existent label
            collection.set(&label_set, 1.0, sample_update_time());

            // Verify the label exists
            assert!(collection.get(&label_set).is_some());
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(*sample.value(), Gauge::new(1.0));
        }

        #[test]
        fn it_should_update_the_latest_update_time_when_setting() {
            let label_set = LabelSet::default();
            let initial_time = sample_update_time();

            let mut collection = SampleCollection::<Gauge>::default();
            collection.set(&label_set, 1.0, initial_time);

            // Set with a new time
            let new_time = initial_time.add(DurationSinceUnixEpoch::from_secs(1));
            collection.set(&label_set, 2.0, new_time);

            let sample = collection.get(&label_set).unwrap();
            assert_eq!(sample.recorded_at(), new_time);
            assert_eq!(*sample.value(), Gauge::new(2.0));
        }

        #[test]
        fn it_should_allow_setting_the_gauge_for_multiple_labels() {
            let label1 = LabelSet::from([("name", "value1")]);
            let label2 = LabelSet::from([("name", "value2")]);
            let now = sample_update_time();

            let mut collection = SampleCollection::<Gauge>::default();

            collection.set(&label1, 1.0, now);
            collection.set(&label2, 2.0, now);

            assert_eq!(collection.get(&label1).unwrap().value(), &Gauge::new(1.0));
            assert_eq!(collection.get(&label2).unwrap().value(), &Gauge::new(2.0));
            assert_eq!(collection.len(), 2);
        }

        #[test]
        fn it_should_allow_incrementing_the_gauge() {
            let label_set = LabelSet::default();
            let mut collection = SampleCollection::<Gauge>::default();

            // Initialize the sample
            collection.set(&label_set, 1.0, sample_update_time());

            // Increment
            collection.increment(&label_set, sample_update_time());
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(*sample.value(), Gauge::new(2.0));
        }

        #[test]
        fn it_should_allow_decrementing_the_gauge() {
            let label_set = LabelSet::default();
            let mut collection = SampleCollection::<Gauge>::default();

            // Initialize the sample
            collection.set(&label_set, 1.0, sample_update_time());

            // Increment
            collection.decrement(&label_set, sample_update_time());
            let sample = collection.get(&label_set).unwrap();
            assert_eq!(*sample.value(), Gauge::new(0.0));
        }
    }
}
