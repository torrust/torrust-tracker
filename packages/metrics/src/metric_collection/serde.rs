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
    use pretty_assertions::assert_eq;
    use torrust_tracker_primitives::DurationSinceUnixEpoch;

    use crate::counter::Counter;
    use crate::gauge::Gauge;
    use crate::label::LabelSet;
    use crate::metric::description::MetricDescription;
    use crate::metric::Metric;
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
    fn it_should_allow_deserializing_from_json() {
        let expected_metric_collection = fixture_object();
        let metric_collection_json = fixture_json();

        let metric_collection: MetricCollection = serde_json::from_str(&metric_collection_json).unwrap();

        assert_eq!(metric_collection, expected_metric_collection);
    }
}
