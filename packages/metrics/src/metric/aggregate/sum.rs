use crate::aggregate::AggregateValue;
use crate::counter::Counter;
use crate::gauge::Gauge;
use crate::label::LabelSet;
use crate::metric::Metric;

pub trait Sum {
    fn sum(&self, label_set_criteria: &LabelSet) -> AggregateValue;
}

impl Sum for Metric<Counter> {
    #[allow(clippy::cast_precision_loss)]
    fn sum(&self, label_set_criteria: &LabelSet) -> AggregateValue {
        let sum: f64 = self
            .sample_collection
            .iter()
            .filter(|(label_set, _measurement)| label_set.matches(label_set_criteria))
            .map(|(_label_set, measurement)| measurement.value().primitive() as f64)
            .sum();

        sum.into()
    }
}

impl Sum for Metric<Gauge> {
    fn sum(&self, label_set_criteria: &LabelSet) -> AggregateValue {
        let sum: f64 = self
            .sample_collection
            .iter()
            .filter(|(label_set, _measurement)| label_set.matches(label_set_criteria))
            .map(|(_label_set, measurement)| measurement.value().primitive())
            .sum();

        sum.into()
    }
}

#[cfg(test)]
mod tests {

    use torrust_tracker_primitives::DurationSinceUnixEpoch;

    use crate::aggregate::AggregateValue;
    use crate::counter::Counter;
    use crate::gauge::Gauge;
    use crate::label::LabelSet;
    use crate::metric::aggregate::sum::Sum;
    use crate::metric::{Metric, MetricName};
    use crate::metric_name;
    use crate::sample::Sample;
    use crate::sample_collection::SampleCollection;

    struct MetricBuilder<T> {
        sample_time: DurationSinceUnixEpoch,
        name: MetricName,
        samples: Vec<Sample<T>>,
    }

    impl<T> Default for MetricBuilder<T> {
        fn default() -> Self {
            Self {
                sample_time: DurationSinceUnixEpoch::from_secs(1_743_552_000),
                name: metric_name!("test_metric"),
                samples: vec![],
            }
        }
    }

    impl<T> MetricBuilder<T> {
        fn with_sample(mut self, value: T, label_set: &LabelSet) -> Self {
            let sample = Sample::new(value, self.sample_time, label_set.clone());
            self.samples.push(sample);
            self
        }

        fn build(self) -> Metric<T> {
            Metric::new(
                self.name,
                None,
                None,
                SampleCollection::new(self.samples).expect("invalid samples"),
            )
        }
    }

    fn counter_cases() -> Vec<(Metric<Counter>, LabelSet, AggregateValue)> {
        // (metric, label set criteria, expected_aggregate_value)
        vec![
            // Metric with one sample without label set
            (
                MetricBuilder::default().with_sample(1.into(), &LabelSet::empty()).build(),
                LabelSet::empty(),
                1.0.into(),
            ),
            // Metric with one sample with a label set
            (
                MetricBuilder::default()
                    .with_sample(1.into(), &[("l1", "l1_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                1.0.into(),
            ),
            // Metric with two samples, different label sets, sum all
            (
                MetricBuilder::default()
                    .with_sample(1.into(), &[("l1", "l1_value")].into())
                    .with_sample(2.into(), &[("l2", "l2_value")].into())
                    .build(),
                LabelSet::empty(),
                3.0.into(),
            ),
            // Metric with two samples, different label sets, sum one
            (
                MetricBuilder::default()
                    .with_sample(1.into(), &[("l1", "l1_value")].into())
                    .with_sample(2.into(), &[("l2", "l2_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                1.0.into(),
            ),
            // Metric with two samples, same label key, different label values, sum by key
            (
                MetricBuilder::default()
                    .with_sample(1.into(), &[("l1", "l1_value"), ("la", "la_value")].into())
                    .with_sample(2.into(), &[("l1", "l1_value"), ("lb", "lb_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                3.0.into(),
            ),
            // Metric with two samples, different label values, sum by subkey
            (
                MetricBuilder::default()
                    .with_sample(1.into(), &[("l1", "l1_value"), ("la", "la_value")].into())
                    .with_sample(2.into(), &[("l1", "l1_value"), ("lb", "lb_value")].into())
                    .build(),
                [("la", "la_value")].into(),
                1.0.into(),
            ),
            // Edge: Metric with no samples at all
            (MetricBuilder::default().build(), LabelSet::empty(), 0.0.into()),
            // Edge: Metric with samples but no matching labels
            (
                MetricBuilder::default()
                    .with_sample(5.into(), &[("foo", "bar")].into())
                    .build(),
                [("not", "present")].into(),
                0.0.into(),
            ),
            // Edge: Metric with zero value
            (
                MetricBuilder::default()
                    .with_sample(0.into(), &[("l3", "l3_value")].into())
                    .build(),
                [("l3", "l3_value")].into(),
                0.0.into(),
            ),
            // Edge: Metric with a very large value
            (
                MetricBuilder::default()
                    .with_sample(u64::MAX.into(), &LabelSet::empty())
                    .build(),
                LabelSet::empty(),
                #[allow(clippy::cast_precision_loss)]
                (u64::MAX as f64).into(),
            ),
        ]
    }

    fn gauge_cases() -> Vec<(Metric<Gauge>, LabelSet, AggregateValue)> {
        // (metric, label set criteria, expected_aggregate_value)
        vec![
            // Metric with one sample without label set
            (
                MetricBuilder::default().with_sample(1.0.into(), &LabelSet::empty()).build(),
                LabelSet::empty(),
                1.0.into(),
            ),
            // Metric with one sample with a label set
            (
                MetricBuilder::default()
                    .with_sample(1.0.into(), &[("l1", "l1_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                1.0.into(),
            ),
            // Metric with two samples, different label sets, sum all
            (
                MetricBuilder::default()
                    .with_sample(1.0.into(), &[("l1", "l1_value")].into())
                    .with_sample(2.0.into(), &[("l2", "l2_value")].into())
                    .build(),
                LabelSet::empty(),
                3.0.into(),
            ),
            // Metric with two samples, different label sets, sum one
            (
                MetricBuilder::default()
                    .with_sample(1.0.into(), &[("l1", "l1_value")].into())
                    .with_sample(2.0.into(), &[("l2", "l2_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                1.0.into(),
            ),
            // Metric with two samples, same label key, different label values, sum by key
            (
                MetricBuilder::default()
                    .with_sample(1.0.into(), &[("l1", "l1_value"), ("la", "la_value")].into())
                    .with_sample(2.0.into(), &[("l1", "l1_value"), ("lb", "lb_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                3.0.into(),
            ),
            // Metric with two samples, different label values, sum by subkey
            (
                MetricBuilder::default()
                    .with_sample(1.0.into(), &[("l1", "l1_value"), ("la", "la_value")].into())
                    .with_sample(2.0.into(), &[("l1", "l1_value"), ("lb", "lb_value")].into())
                    .build(),
                [("la", "la_value")].into(),
                1.0.into(),
            ),
            // Edge: Metric with no samples at all
            (MetricBuilder::default().build(), LabelSet::empty(), 0.0.into()),
            // Edge: Metric with samples but no matching labels
            (
                MetricBuilder::default()
                    .with_sample(5.0.into(), &[("foo", "bar")].into())
                    .build(),
                [("not", "present")].into(),
                0.0.into(),
            ),
            // Edge: Metric with zero value
            (
                MetricBuilder::default()
                    .with_sample(0.0.into(), &[("l3", "l3_value")].into())
                    .build(),
                [("l3", "l3_value")].into(),
                0.0.into(),
            ),
            // Edge: Metric with negative values
            (
                MetricBuilder::default()
                    .with_sample((-2.0).into(), &[("l4", "l4_value")].into())
                    .with_sample(3.0.into(), &[("l5", "l5_value")].into())
                    .build(),
                LabelSet::empty(),
                1.0.into(),
            ),
            // Edge: Metric with a very large value
            (
                MetricBuilder::default()
                    .with_sample(f64::MAX.into(), &LabelSet::empty())
                    .build(),
                LabelSet::empty(),
                f64::MAX.into(),
            ),
        ]
    }

    #[test]
    fn test_counter_cases() {
        for (idx, (metric, criteria, expected_value)) in counter_cases().iter().enumerate() {
            let sum = metric.sum(criteria);

            assert_eq!(
                sum, *expected_value,
                "at case {idx}, expected sum to be {expected_value}, got {sum}"
            );
        }
    }

    #[test]
    fn test_gauge_cases() {
        for (idx, (metric, criteria, expected_value)) in gauge_cases().iter().enumerate() {
            let sum = metric.sum(criteria);

            assert_eq!(
                sum, *expected_value,
                "at case {idx}, expected sum to be {expected_value}, got {sum}"
            );
        }
    }
}
