use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Deserializer, Serialize};

use crate::counter::Counter;
use crate::gauge::Gauge;
use crate::metric::Metric;
use crate::metric_collection::{MetricCollection, MetricKindCollection};

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

#[cfg(test)]
mod tests {
    use std::fmt;

    use pretty_assertions::assert_eq;
    use serde::Serialize;
    use serde::ser::{self, Impossible, SerializeSeq};
    use torrust_clock::DurationSinceUnixEpoch;

    use crate::counter::Counter;
    use crate::gauge::Gauge;
    use crate::label::LabelSet;
    use crate::metric::Metric;
    use crate::metric::description::MetricDescription;
    use crate::metric_collection::{MetricCollection, MetricKindCollection};
    use crate::sample::Sample;
    use crate::sample_collection::SampleCollection;
    use crate::{label_name, metric_name};

    fn fixture_object() -> MetricCollection {
        let time = DurationSinceUnixEpoch::from_secs(1_743_552_000);

        let label_set: LabelSet = [
            (label_name!("server_binding_protocol"), crate::label::LabelValue::new("http")),
            (label_name!("server_binding_ip"), crate::label::LabelValue::new("0.0.0.0")),
            (label_name!("server_binding_port"), crate::label::LabelValue::new("7070")),
        ]
        .into();

        MetricCollection::new(
            MetricKindCollection::new(vec![Metric::new(
                metric_name!("http_tracker_core_announce_requests_received_total"),
                None,
                Some(MetricDescription::new("The number of announce requests received.")),
                SampleCollection::new(vec![Sample::new(Counter::new(1), time, label_set.clone())]).unwrap(),
            )])
            .unwrap(),
            MetricKindCollection::new(vec![Metric::new(
                metric_name!("udp_tracker_server_performance_avg_announce_processing_time_ns"),
                None,
                Some(MetricDescription::new("The average announce processing time in nanoseconds.")),
                SampleCollection::new(vec![Sample::new(Gauge::new(1.0), time, label_set.clone())]).unwrap(),
            )])
            .unwrap(),
        )
        .unwrap()
    }

    fn fixture_json() -> String {
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

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct StrictSeqError(String);

    impl fmt::Display for StrictSeqError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for StrictSeqError {}

    impl ser::Error for StrictSeqError {
        fn custom<T: fmt::Display>(msg: T) -> Self {
            Self(msg.to_string())
        }
    }

    struct StrictSeqLenSerializer;

    struct StrictSeq {
        expected_len: usize,
        actual_len: usize,
    }

    impl serde::Serializer for StrictSeqLenSerializer {
        type Ok = usize;
        type Error = StrictSeqError;
        type SerializeSeq = StrictSeq;
        type SerializeTuple = Impossible<usize, StrictSeqError>;
        type SerializeTupleStruct = Impossible<usize, StrictSeqError>;
        type SerializeTupleVariant = Impossible<usize, StrictSeqError>;
        type SerializeMap = Impossible<usize, StrictSeqError>;
        type SerializeStruct = Impossible<usize, StrictSeqError>;
        type SerializeStructVariant = Impossible<usize, StrictSeqError>;

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            let expected_len = len.ok_or_else(|| StrictSeqError("serialize_seq length was None".to_owned()))?;

            Ok(StrictSeq {
                expected_len,
                actual_len: 0,
            })
        }

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            Err(StrictSeqError("unsupported".to_owned()))
        }
    }

    impl SerializeSeq for StrictSeq {
        type Ok = usize;
        type Error = StrictSeqError;

        fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.actual_len += 1;

            if self.actual_len > self.expected_len {
                return Err(StrictSeqError(format!(
                    "serialized more elements ({}) than sequence hint ({})",
                    self.actual_len, self.expected_len
                )));
            }

            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            if self.actual_len == self.expected_len {
                Ok(self.actual_len)
            } else {
                Err(StrictSeqError(format!(
                    "serialized {} elements but sequence hint was {}",
                    self.actual_len, self.expected_len
                )))
            }
        }
    }

    #[test]
    fn it_should_allow_serializing_to_json() {
        // todo: this test does work with metric with multiple samples because
        // samples are not serialized in the same order as they are created.
        let metric_collection = fixture_object();
        let expected_json = fixture_json();

        let json = serde_json::to_string_pretty(&metric_collection).unwrap();

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&json).unwrap(),
            serde_json::from_str::<serde_json::Value>(&expected_json).unwrap()
        );
    }

    #[test]
    fn it_should_use_a_correct_sequence_length_hint_when_serializing() {
        let metric_collection = fixture_object();

        let serialized_len = metric_collection.serialize(StrictSeqLenSerializer).unwrap();

        assert_eq!(serialized_len, 2);
    }

    #[test]
    fn it_should_allow_deserializing_from_json() {
        let expected_metric_collection = fixture_object();
        let metric_collection_json = fixture_json();

        let metric_collection: MetricCollection = serde_json::from_str(&metric_collection_json).unwrap();

        assert_eq!(metric_collection, expected_metric_collection);
    }

    #[test]
    fn it_should_allow_serializing_an_empty_collection_to_json() {
        let collection = MetricCollection::default();
        let json = serde_json::to_string(&collection).unwrap();
        assert_eq!(json, "[]");
    }

    #[test]
    fn it_should_allow_deserializing_an_empty_json_array() {
        let collection: MetricCollection = serde_json::from_str("[]").unwrap();
        assert_eq!(collection, MetricCollection::default());
    }

    #[test]
    fn it_should_fail_deserializing_json_with_unknown_metric_type() {
        // "histogram" is not a recognised tag variant in MetricPayload
        let json = r#"[{"type":"histogram","name":"test","unit":null,"description":null,"samples":[]}]"#;

        let result = serde_json::from_str::<MetricCollection>(json);

        assert!(result.is_err());
    }

    #[test]
    fn it_should_fail_deserializing_json_with_duplicate_counter_names() {
        // Two counter entries with the same name → MetricKindCollection::new error
        let json = r#"[
            {"type":"counter","name":"hits_total","unit":null,"description":null,"samples":[]},
            {"type":"counter","name":"hits_total","unit":null,"description":null,"samples":[]}
        ]"#;

        let result = serde_json::from_str::<MetricCollection>(json);

        assert!(result.is_err());
    }

    #[test]
    fn it_should_fail_deserializing_json_with_cross_type_name_collision() {
        // A counter and a gauge sharing the same name → MetricCollection::new error
        let json = r#"[
            {"type":"counter","name":"shared_name","unit":null,"description":null,"samples":[]},
            {"type":"gauge","name":"shared_name","unit":null,"description":null,"samples":[]}
        ]"#;

        let result = serde_json::from_str::<MetricCollection>(json);

        assert!(result.is_err());
    }
}
