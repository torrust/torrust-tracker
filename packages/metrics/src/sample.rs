use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::counter::Counter;
use super::gauge::Gauge;
use super::label::LabelSet;
use super::prometheus::PrometheusSerializable;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sample<T> {
    #[serde(flatten)]
    measurement: Measurement<T>,

    #[serde(rename = "labels")]
    label_set: LabelSet,
}

impl<T> Sample<T> {
    #[must_use]
    pub fn new(value: T, recorded_at: DurationSinceUnixEpoch, label_set: LabelSet) -> Self {
        let data = Measurement { value, recorded_at };

        Self {
            measurement: data,
            label_set,
        }
    }

    #[must_use]
    pub fn measurement(&self) -> &Measurement<T> {
        &self.measurement
    }

    #[must_use]
    pub fn value(&self) -> &T {
        &self.measurement.value
    }

    #[must_use]
    pub fn recorded_at(&self) -> DurationSinceUnixEpoch {
        self.measurement.recorded_at
    }

    #[must_use]
    pub fn labels(&self) -> &LabelSet {
        &self.label_set
    }
}

impl<T: PrometheusSerializable> PrometheusSerializable for Sample<T> {
    fn to_prometheus(&self) -> String {
        if self.label_set.is_empty() {
            format!(" {}", self.measurement.to_prometheus())
        } else {
            format!("{} {}", self.label_set.to_prometheus(), self.measurement.to_prometheus())
        }
    }
}

impl Sample<Counter> {
    pub fn increment(&mut self, time: DurationSinceUnixEpoch) {
        self.measurement.increment(time);
    }
}

impl Sample<Gauge> {
    pub fn set(&mut self, value: f64, time: DurationSinceUnixEpoch) {
        self.measurement.set(value, time);
    }

    pub fn increment(&mut self, time: DurationSinceUnixEpoch) {
        self.measurement.increment(time);
    }

    pub fn decrement(&mut self, time: DurationSinceUnixEpoch) {
        self.measurement.decrement(time);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measurement<T> {
    /// The value of the sample.
    value: T,

    /// The time when the sample was last updated.
    #[serde(serialize_with = "serialize_duration", deserialize_with = "deserialize_duration")]
    recorded_at: DurationSinceUnixEpoch,
}

impl<T> Measurement<T> {
    #[must_use]
    pub fn new(value: T, recorded_at: DurationSinceUnixEpoch) -> Self {
        Self { value, recorded_at }
    }

    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    #[must_use]
    pub fn recorded_at(&self) -> DurationSinceUnixEpoch {
        self.recorded_at
    }

    fn set_recorded_at(&mut self, time: DurationSinceUnixEpoch) {
        self.recorded_at = time;
    }
}

impl<T> From<Sample<T>> for (LabelSet, Measurement<T>) {
    fn from(sample: Sample<T>) -> Self {
        (sample.label_set, sample.measurement)
    }
}

impl<T: PrometheusSerializable> PrometheusSerializable for Measurement<T> {
    fn to_prometheus(&self) -> String {
        self.value.to_prometheus()
    }
}

impl Measurement<Counter> {
    pub fn increment(&mut self, time: DurationSinceUnixEpoch) {
        self.value.increment(1);
        self.set_recorded_at(time);
    }

    pub fn absolute(&mut self, value: u64, time: DurationSinceUnixEpoch) {
        self.value.absolute(value);
        self.set_recorded_at(time);
    }
}

impl Measurement<Gauge> {
    pub fn set(&mut self, value: f64, time: DurationSinceUnixEpoch) {
        self.value.set(value);
        self.set_recorded_at(time);
    }

    pub fn increment(&mut self, time: DurationSinceUnixEpoch) {
        self.value.increment(1.0);
        self.set_recorded_at(time);
    }

    pub fn decrement(&mut self, time: DurationSinceUnixEpoch) {
        self.value.decrement(1.0);
        self.set_recorded_at(time);
    }
}

/// Serializes the `recorded_at` field as a string in ISO 8601 format (RFC 3339).
///
/// # Errors
///
/// Returns an error if:
/// - The conversion from `u64` to `i64` fails.
/// - The timestamp is invalid.
fn serialize_duration<S>(duration: &DurationSinceUnixEpoch, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let secs = i64::try_from(duration.as_secs()).map_err(|_| serde::ser::Error::custom("Timestamp too large"))?;
    let nanos = duration.subsec_nanos();

    let datetime = DateTime::from_timestamp(secs, nanos).ok_or_else(|| serde::ser::Error::custom("Invalid timestamp"))?;

    serializer.serialize_str(&datetime.to_rfc3339()) // Serializes as ISO 8601 (RFC 3339)
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<DurationSinceUnixEpoch, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize theISO 8601 (RFC 3339) formatted string
    let datetime_str = String::deserialize(deserializer)?;

    let datetime =
        DateTime::parse_from_rfc3339(&datetime_str).map_err(|e| de::Error::custom(format!("Invalid datetime format: {e}")))?;

    let datetime_utc = datetime.with_timezone(&Utc);

    let secs = u64::try_from(datetime_utc.timestamp()).map_err(|_| de::Error::custom("Timestamp out of range"))?;

    Ok(DurationSinceUnixEpoch::new(secs, datetime_utc.timestamp_subsec_nanos()))
}

#[cfg(test)]
mod tests {
    use torrust_tracker_primitives::DurationSinceUnixEpoch;

    use super::*;

    // Helper function to create a sample update time.
    fn updated_at_time() -> DurationSinceUnixEpoch {
        DurationSinceUnixEpoch::from_secs(1_743_552_000)
    }

    #[test]
    fn it_should_have_a_value() {
        let sample = Sample::new(
            42,
            DurationSinceUnixEpoch::from_secs(1_743_552_000),
            LabelSet::from(vec![("test", "label")]),
        );

        assert_eq!(sample.value(), &42);
    }

    #[test]
    fn it_should_record_the_latest_update_time() {
        let sample = Sample::new(
            42,
            DurationSinceUnixEpoch::from_secs(1_743_552_000),
            LabelSet::from(vec![("test", "label")]),
        );

        assert_eq!(sample.recorded_at(), updated_at_time());
    }

    #[test]
    fn it_should_include_a_label_set() {
        let sample = Sample::new(
            42,
            DurationSinceUnixEpoch::from_secs(1_743_552_000),
            LabelSet::from(vec![("test", "label")]),
        );

        assert_eq!(sample.labels(), &LabelSet::from(vec![("test", "label")]));
    }

    mod for_counter_type_sample {
        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use crate::label::LabelSet;
        use crate::prometheus::PrometheusSerializable;
        use crate::sample::tests::updated_at_time;
        use crate::sample::{Counter, Sample};

        #[test]
        fn it_should_allow_a_counter_type_value() {
            let sample = Sample::new(
                Counter::new(42),
                DurationSinceUnixEpoch::from_secs(1_743_552_000),
                LabelSet::from(vec![("label_name", "label vale")]),
            );

            assert_eq!(sample.value(), &Counter::new(42));
        }

        #[test]
        fn it_should_allow_incrementing_the_counter() {
            let mut sample = Sample::new(Counter::default(), DurationSinceUnixEpoch::default(), LabelSet::default());

            sample.increment(updated_at_time());

            assert_eq!(sample.value(), &Counter::new(1));
        }

        #[test]
        fn it_should_record_the_latest_update_time_when_the_counter_is_incremented() {
            let mut sample = Sample::new(Counter::default(), DurationSinceUnixEpoch::default(), LabelSet::default());

            let time = updated_at_time();

            sample.increment(time);

            assert_eq!(sample.recorded_at(), time);
        }

        #[test]
        fn it_should_allow_exporting_to_prometheus_format() {
            let counter = Counter::new(42);

            let labels = LabelSet::from(vec![("label_name", "label_value"), ("method", "GET")]);

            let sample = Sample::new(counter, DurationSinceUnixEpoch::default(), labels);

            assert_eq!(sample.to_prometheus(), r#"{label_name="label_value",method="GET"} 42"#);
        }

        #[test]
        fn it_should_allow_exporting_to_prometheus_format_with_empty_label_set() {
            let counter = Counter::new(42);

            let sample = Sample::new(counter, DurationSinceUnixEpoch::default(), LabelSet::default());

            assert_eq!(sample.to_prometheus(), " 42");
        }
    }
    mod for_gauge_type_sample {
        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use crate::label::LabelSet;
        use crate::prometheus::PrometheusSerializable;
        use crate::sample::tests::updated_at_time;
        use crate::sample::{Gauge, Sample};

        #[test]
        fn it_should_allow_a_counter_type_value() {
            let sample = Sample::new(
                Gauge::new(42.0),
                DurationSinceUnixEpoch::from_secs(1_743_552_000),
                LabelSet::from(vec![("label_name", "label vale")]),
            );

            assert_eq!(sample.value(), &Gauge::new(42.0));
        }

        #[test]
        fn it_should_allow_setting_a_value() {
            let mut sample = Sample::new(Gauge::default(), DurationSinceUnixEpoch::default(), LabelSet::default());

            sample.set(1.0, updated_at_time());

            assert_eq!(sample.value(), &Gauge::new(1.0));
        }

        #[test]
        fn it_should_allow_incrementing_the_value() {
            let mut sample = Sample::new(Gauge::new(0.0), DurationSinceUnixEpoch::default(), LabelSet::default());

            sample.increment(updated_at_time());

            assert_eq!(sample.value(), &Gauge::new(1.0));
        }

        #[test]
        fn it_should_allow_decrementing_the_value() {
            let mut sample = Sample::new(Gauge::new(1.0), DurationSinceUnixEpoch::default(), LabelSet::default());

            sample.decrement(updated_at_time());

            assert_eq!(sample.value(), &Gauge::new(0.0));
        }

        #[test]
        fn it_should_record_the_latest_update_time_when_the_counter_is_incremented() {
            let mut sample = Sample::new(Gauge::default(), DurationSinceUnixEpoch::default(), LabelSet::default());

            let time = updated_at_time();

            sample.set(1.0, time);

            assert_eq!(sample.recorded_at(), time);
        }

        #[test]
        fn it_should_allow_exporting_to_prometheus_format() {
            let counter = Gauge::new(42.0);

            let labels = LabelSet::from(vec![("label_name", "label_value"), ("method", "GET")]);

            let sample = Sample::new(counter, DurationSinceUnixEpoch::default(), labels);

            assert_eq!(sample.to_prometheus(), r#"{label_name="label_value",method="GET"} 42"#);
        }

        #[test]
        fn it_should_allow_exporting_to_prometheus_format_with_empty_label_set() {
            let gauge = Gauge::new(42.0);

            let sample = Sample::new(gauge, DurationSinceUnixEpoch::default(), LabelSet::default());

            assert_eq!(sample.to_prometheus(), " 42");
        }
    }

    mod serialization_to_json {
        use pretty_assertions::assert_eq;
        use serde_json::json;
        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use crate::label::LabelSet;
        use crate::sample::tests::updated_at_time;
        use crate::sample::Sample;

        #[test]
        fn test_serialization_round_trip() {
            let original = Sample::new(42, updated_at_time(), LabelSet::from(vec![("test", "serialization")]));

            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Sample<i32> = serde_json::from_str(&json).unwrap();

            assert_eq!(original.measurement.value, deserialized.measurement.value);
            assert_eq!(original.measurement.recorded_at, deserialized.measurement.recorded_at);
            assert_eq!(original.label_set, deserialized.label_set);
        }

        #[test]
        fn test_rfc3339_serialization_format_for_update_time() {
            let sample = Sample::new(
                42,
                DurationSinceUnixEpoch::new(1_743_552_000, 100),
                LabelSet::from(vec![("label_name", "label value")]),
            );

            let json = serde_json::to_string(&sample).unwrap();

            let expected_json = r#"
                {
                    "value": 42,
                    "recorded_at": "2025-04-02T00:00:00.000000100+00:00",
                    "labels": [
                        {
                        "name": "label_name",
                        "value": "label value"
                        }
                    ]
                }
            "#;

            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&json).unwrap(),
                serde_json::from_str::<serde_json::Value>(expected_json).unwrap()
            );
        }

        #[test]
        fn test_invalid_update_timestamp_serialization() {
            let timestamp_too_large = DurationSinceUnixEpoch::new(i64::MAX as u64 + 1, 0);

            let sample = Sample::new(42, timestamp_too_large, LabelSet::from(vec![("label_name", "label value")]));

            let result = serde_json::to_string(&sample);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Timestamp too large"));
        }

        #[test]
        fn test_invalid_update_datetime_deserialization() {
            let invalid_json = json!(
                r#"
                {
                    "value": 42,
                    "recorded_at": "1-1-2023T25:00:00Z",
                    "labels": [
                        {
                        "name": "label_name",
                        "value": "label value"
                        }
                    ]
                }
                "#
            );

            let result: Result<DurationSinceUnixEpoch, _> = serde_json::from_value(invalid_json);

            assert!(result.unwrap_err().to_string().contains("invalid type"));
        }

        #[test]
        fn test_update_datetime_high_precision_nanoseconds() {
            let sample = Sample::new(
                42,
                DurationSinceUnixEpoch::new(1_743_552_000, 100),
                LabelSet::from(vec![("label_name", "label value")]),
            );

            let json = serde_json::to_string(&sample).unwrap();

            let deserialized: Sample<i32> = serde_json::from_str(&json).unwrap();

            assert_eq!(deserialized, sample);
        }
    }
}
