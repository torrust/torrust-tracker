use std::borrow::Cow;
use std::sync::Arc;

use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::counter::Counter;
use crate::gauge::Gauge;
use crate::label::LabelSet;
use crate::metric::description::MetricDescription;
use crate::metric::{Metric, MetricName};
use crate::metric_collection::{MetricCollection, MetricKindCollection};
use crate::prometheus::{PrometheusDeserializable, PrometheusDeserializationError, PrometheusSerializable};
use crate::sample::Sample;
use crate::sample_collection::SampleCollection;

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

/// Converts a Prometheus timestamp (seconds since Unix epoch as `f64`) to a
/// `DurationSinceUnixEpoch`.
///
/// If `t` is non-finite or negative, `fallback` is returned instead.
pub(super) fn parse_prometheus_timestamp(t: f64, fallback: DurationSinceUnixEpoch) -> DurationSinceUnixEpoch {
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

pub(super) fn collection_error<E: ToString>(error: &E) -> PrometheusDeserializationError {
    PrometheusDeserializationError::CollectionError {
        message: error.to_string(),
    }
}

pub(super) fn build_sample_collection<T>(samples: Vec<Sample<T>>) -> Result<SampleCollection<T>, PrometheusDeserializationError> {
    SampleCollection::new(samples).map_err(|error| collection_error(&error))
}

pub(super) fn build_metric_collection(
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

/// Returns `true` if `v` is a non-negative, whole number that fits in a `u64`.
fn is_whole_u64_representable(v: f64) -> bool {
    const FIRST_UNREPRESENTABLE: f64 = 18_446_744_073_709_551_616.0; // 2^64
    v.is_finite() && v >= 0.0 && v.fract() == 0.0 && v < FIRST_UNREPRESENTABLE
}

fn description_from_help(help: &str) -> Option<MetricDescription> {
    if help.is_empty() {
        None
    } else {
        Some(help.into())
    }
}

fn ensure_trailing_newline(input: &str) -> Cow<'_, str> {
    if input.ends_with('\n') {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(format!("{input}\n"))
    }
}

trait FromPrometheusValue: Sized {
    fn from_prometheus_value(
        family_name: &str,
        value: &openmetrics_parser::PrometheusValue,
    ) -> Result<Self, PrometheusDeserializationError>;
}

impl FromPrometheusValue for Counter {
    fn from_prometheus_value(
        family_name: &str,
        prom_value: &openmetrics_parser::PrometheusValue,
    ) -> Result<Self, PrometheusDeserializationError> {
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
                    openmetrics_parser::MetricNumber::Float(value) if is_whole_u64_representable(value) =>
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
}

impl FromPrometheusValue for Gauge {
    fn from_prometheus_value(
        family_name: &str,
        prom_value: &openmetrics_parser::PrometheusValue,
    ) -> Result<Self, PrometheusDeserializationError> {
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
}

fn parse_family_samples<T: FromPrometheusValue>(
    family_name: &str,
    family: &openmetrics_parser::PrometheusMetricFamily,
    now: DurationSinceUnixEpoch,
) -> Result<Metric<T>, PrometheusDeserializationError> {
    let label_names = Arc::new(family.get_label_names().to_vec());
    let mut samples = Vec::new();

    for parser_sample in family.iter_samples() {
        let parser_label_set = openmetrics_parser::LabelSet::new(Arc::clone(&label_names), parser_sample).map_err(|e| {
            PrometheusDeserializationError::LabelConversion {
                metric_name: family_name.to_owned(),
                message: e.to_string(),
            }
        })?;
        let label_set = convert_openmetrics_label_set(family_name, parser_label_set)?;
        let value = T::from_prometheus_value(family_name, &parser_sample.value)?;
        let time = parser_sample.timestamp.map_or(now, |t| parse_prometheus_timestamp(t, now));
        samples.push(Sample::new(value, time, label_set));
    }

    let metric_name = MetricName::new(family_name);
    let description = description_from_help(&family.help);
    Ok(Metric::new(metric_name, None, description, build_sample_collection(samples)?))
}

impl PrometheusDeserializable for MetricCollection {
    fn from_prometheus(input: &str, now: DurationSinceUnixEpoch) -> Result<Self, PrometheusDeserializationError> {
        // The Prometheus text format requires every metric line to end with a
        // newline character. Normalize the input so callers that produce output
        // without a trailing newline (e.g. our own `to_prometheus`) still work.
        let input = ensure_trailing_newline(input);

        let exposition = openmetrics_parser::prometheus::parse_prometheus(input.as_ref())
            .map_err(|e| PrometheusDeserializationError::ParseError { message: e.to_string() })?;

        let mut counter_metrics: Vec<Metric<Counter>> = Vec::new();
        let mut gauge_metrics: Vec<Metric<Gauge>> = Vec::new();

        for (family_name, family) in &exposition.families {
            match family.family_type {
                openmetrics_parser::PrometheusType::Counter => {
                    counter_metrics.push(parse_family_samples::<Counter>(family_name, family, now)?);
                }
                openmetrics_parser::PrometheusType::Gauge => {
                    gauge_metrics.push(parse_family_samples::<Gauge>(family_name, family, now)?);
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
    mod prometheus_timestamp {
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
            // 0.9999999995 * 1e9 rounds to exactly 1_000_000_000 nanos, triggering
            // a carry: secs becomes 2, nanos becomes 0. Use exact equality so that
            // the mutant `nanos / 1_000_000_000` (= 1 ns) is caught.
            let result = parse_prometheus_timestamp(1.999_999_999_5, now);
            assert_eq!(result, DurationSinceUnixEpoch::from_secs(2));
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

        use super::super::build_metric_collection;
        use crate::counter::Counter;
        use crate::gauge::Gauge;
        use crate::label::{LabelSet, LabelValue};
        use crate::metric::description::MetricDescription;
        use crate::metric::Metric;
        use crate::metric_collection::{MetricCollection, MetricKindCollection};
        use crate::prometheus::{PrometheusDeserializable, PrometheusDeserializationError, PrometheusSerializable};
        use crate::sample::Sample;
        use crate::sample_collection::SampleCollection;
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

            let label_set = LabelSet::empty();
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
            let label_set = LabelSet::empty();
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

            let result = build_metric_collection(counter_metrics, Vec::new());

            assert!(matches!(result, Err(PrometheusDeserializationError::CollectionError { .. })));
        }

        #[test]
        fn it_should_accept_a_counter_value_that_is_a_whole_number_float() {
            // A counter value written as a float with no fractional part (e.g. "42.0")
            // must be accepted and treated as the integer 42. This test catches
            // mutations that corrupt the float-counter match guard by replacing it
            // with `false` or inverting the `>= 0.0` / `< MAX` checks.
            let now = DurationSinceUnixEpoch::from_secs(1_000);
            let input = "# TYPE requests_total counter\nrequests_total 42.0\n";

            let result = MetricCollection::from_prometheus(input, now).expect("should parse successfully");

            let label_set = LabelSet::empty();
            let value = result
                .get_counter_value(&metric_name!("requests_total"), &label_set)
                .expect("counter should be present");

            assert_eq!(value, Counter::new(42));
        }

        #[test]
        fn it_should_reject_a_float_counter_value_equal_to_first_unrepresentable_u64() {
            // 18_446_744_073_709_551_616.0 == 2^64, the first f64 that cannot be
            // safely cast to u64. The guard `value < FIRST_UNREPRESENTABLE_U64_AS_F64`
            // must be strict (<), not <=. This test catches the `<` → `<=` mutation.
            let now = DurationSinceUnixEpoch::from_secs(1_000);
            let input = "# TYPE requests_total counter\nrequests_total 18446744073709551616.0\n";

            let result = MetricCollection::from_prometheus(input, now);

            assert!(
                matches!(result, Err(PrometheusDeserializationError::ValueMismatch { .. })),
                "expected ValueMismatch, got {result:?}"
            );
        }

        #[test]
        fn it_should_return_unknown_type_error_when_no_type_declaration_is_present() {
            let now = DurationSinceUnixEpoch::from_secs(0);
            // No # TYPE line → the parser assigns type Unknown, which triggers
            // the PrometheusType::Unknown arm and returns UnknownType error.
            let input = "hits_total 7\n";

            let result = MetricCollection::from_prometheus(input, now);

            assert!(matches!(result, Err(PrometheusDeserializationError::UnknownType { .. })));
        }
    }
}
