use crate::counter::Counter;
use crate::gauge::Gauge;
use crate::label::LabelSet;
use crate::metric::aggregate::sum::Sum;
use crate::metric::Metric;

pub trait Avg {
    type Output;
    fn avg(&self, label_set_criteria: &LabelSet) -> Self::Output;
}

impl Avg for Metric<Counter> {
    type Output = f64;

    fn avg(&self, label_set_criteria: &LabelSet) -> Self::Output {
        let matching_samples = self.collect_matching_samples(label_set_criteria);

        if matching_samples.is_empty() {
            return 0.0;
        }

        let sum = self.sum(label_set_criteria);

        #[allow(clippy::cast_precision_loss)]
        (sum as f64 / matching_samples.len() as f64)
    }
}

impl Avg for Metric<Gauge> {
    type Output = f64;

    fn avg(&self, label_set_criteria: &LabelSet) -> Self::Output {
        let matching_samples = self.collect_matching_samples(label_set_criteria);

        if matching_samples.is_empty() {
            return 0.0;
        }

        let sum = self.sum(label_set_criteria);

        #[allow(clippy::cast_precision_loss)]
        (sum / matching_samples.len() as f64)
    }
}

#[cfg(test)]
mod tests {

    use torrust_tracker_primitives::DurationSinceUnixEpoch;

    use crate::counter::Counter;
    use crate::gauge::Gauge;
    use crate::label::LabelSet;
    use crate::metric::aggregate::avg::Avg;
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

    fn counter_cases() -> Vec<(Metric<Counter>, LabelSet, f64)> {
        // (metric, label set criteria, expected_average_value)
        vec![
            // Metric with one sample without label set
            (
                MetricBuilder::default().with_sample(1.into(), &LabelSet::empty()).build(),
                LabelSet::empty(),
                1.0,
            ),
            // Metric with one sample with a label set
            (
                MetricBuilder::default()
                    .with_sample(1.into(), &[("l1", "l1_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                1.0,
            ),
            // Metric with two samples, different label sets, average all
            (
                MetricBuilder::default()
                    .with_sample(1.into(), &[("l1", "l1_value")].into())
                    .with_sample(3.into(), &[("l2", "l2_value")].into())
                    .build(),
                LabelSet::empty(),
                2.0, // (1 + 3) / 2 = 2.0
            ),
            // Metric with two samples, different label sets, average one
            (
                MetricBuilder::default()
                    .with_sample(1.into(), &[("l1", "l1_value")].into())
                    .with_sample(2.into(), &[("l2", "l2_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                1.0,
            ),
            // Metric with three samples, same label key, different label values, average by key
            (
                MetricBuilder::default()
                    .with_sample(2.into(), &[("l1", "l1_value"), ("la", "la_value")].into())
                    .with_sample(4.into(), &[("l1", "l1_value"), ("lb", "lb_value")].into())
                    .with_sample(6.into(), &[("l1", "l1_value"), ("lc", "lc_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                4.0, // (2 + 4 + 6) / 3 = 4.0
            ),
            // Metric with two samples, different label values, average by subkey
            (
                MetricBuilder::default()
                    .with_sample(5.into(), &[("l1", "l1_value"), ("la", "la_value")].into())
                    .with_sample(7.into(), &[("l1", "l1_value"), ("lb", "lb_value")].into())
                    .build(),
                [("la", "la_value")].into(),
                5.0,
            ),
            // Edge: Metric with no samples at all
            (MetricBuilder::default().build(), LabelSet::empty(), 0.0),
            // Edge: Metric with samples but no matching labels
            (
                MetricBuilder::default()
                    .with_sample(5.into(), &[("foo", "bar")].into())
                    .build(),
                [("not", "present")].into(),
                0.0,
            ),
            // Edge: Metric with zero value
            (
                MetricBuilder::default()
                    .with_sample(0.into(), &[("l3", "l3_value")].into())
                    .build(),
                [("l3", "l3_value")].into(),
                0.0,
            ),
            // Edge: Metric with a very large value
            (
                MetricBuilder::default()
                    .with_sample((u64::MAX / 2).into(), &[("edge", "large1")].into())
                    .with_sample((u64::MAX / 2).into(), &[("edge", "large2")].into())
                    .build(),
                LabelSet::empty(),
                #[allow(clippy::cast_precision_loss)]
                (u64::MAX as f64 / 2.0), // Average of (max/2) and (max/2)
            ),
        ]
    }

    fn gauge_cases() -> Vec<(Metric<Gauge>, LabelSet, f64)> {
        // (metric, label set criteria, expected_average_value)
        vec![
            // Metric with one sample without label set
            (
                MetricBuilder::default().with_sample(1.0.into(), &LabelSet::empty()).build(),
                LabelSet::empty(),
                1.0,
            ),
            // Metric with one sample with a label set
            (
                MetricBuilder::default()
                    .with_sample(1.0.into(), &[("l1", "l1_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                1.0,
            ),
            // Metric with two samples, different label sets, average all
            (
                MetricBuilder::default()
                    .with_sample(1.0.into(), &[("l1", "l1_value")].into())
                    .with_sample(3.0.into(), &[("l2", "l2_value")].into())
                    .build(),
                LabelSet::empty(),
                2.0, // (1.0 + 3.0) / 2 = 2.0
            ),
            // Metric with two samples, different label sets, average one
            (
                MetricBuilder::default()
                    .with_sample(1.0.into(), &[("l1", "l1_value")].into())
                    .with_sample(2.0.into(), &[("l2", "l2_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                1.0,
            ),
            // Metric with three samples, same label key, different label values, average by key
            (
                MetricBuilder::default()
                    .with_sample(2.0.into(), &[("l1", "l1_value"), ("la", "la_value")].into())
                    .with_sample(4.0.into(), &[("l1", "l1_value"), ("lb", "lb_value")].into())
                    .with_sample(6.0.into(), &[("l1", "l1_value"), ("lc", "lc_value")].into())
                    .build(),
                [("l1", "l1_value")].into(),
                4.0, // (2.0 + 4.0 + 6.0) / 3 = 4.0
            ),
            // Metric with two samples, different label values, average by subkey
            (
                MetricBuilder::default()
                    .with_sample(5.0.into(), &[("l1", "l1_value"), ("la", "la_value")].into())
                    .with_sample(7.0.into(), &[("l1", "l1_value"), ("lb", "lb_value")].into())
                    .build(),
                [("la", "la_value")].into(),
                5.0,
            ),
            // Edge: Metric with no samples at all
            (MetricBuilder::default().build(), LabelSet::empty(), 0.0),
            // Edge: Metric with samples but no matching labels
            (
                MetricBuilder::default()
                    .with_sample(5.0.into(), &[("foo", "bar")].into())
                    .build(),
                [("not", "present")].into(),
                0.0,
            ),
            // Edge: Metric with zero value
            (
                MetricBuilder::default()
                    .with_sample(0.0.into(), &[("l3", "l3_value")].into())
                    .build(),
                [("l3", "l3_value")].into(),
                0.0,
            ),
            // Edge: Metric with negative values
            (
                MetricBuilder::default()
                    .with_sample((-2.0).into(), &[("l4", "l4_value")].into())
                    .with_sample(4.0.into(), &[("l5", "l5_value")].into())
                    .build(),
                LabelSet::empty(),
                1.0, // (-2.0 + 4.0) / 2 = 1.0
            ),
            // Edge: Metric with decimal values
            (
                MetricBuilder::default()
                    .with_sample(1.5.into(), &[("l6", "l6_value")].into())
                    .with_sample(2.5.into(), &[("l7", "l7_value")].into())
                    .build(),
                LabelSet::empty(),
                2.0, // (1.5 + 2.5) / 2 = 2.0
            ),
        ]
    }

    #[test]
    fn test_counter_cases() {
        for (idx, (metric, criteria, expected_value)) in counter_cases().iter().enumerate() {
            let avg = metric.avg(criteria);

            assert!(
                (avg - expected_value).abs() <= f64::EPSILON,
                "at case {idx}, expected avg to be {expected_value}, got {avg}"
            );
        }
    }

    #[test]
    fn test_gauge_cases() {
        for (idx, (metric, criteria, expected_value)) in gauge_cases().iter().enumerate() {
            let avg = metric.avg(criteria);

            assert!(
                (avg - expected_value).abs() <= f64::EPSILON,
                "at case {idx}, expected avg to be {expected_value}, got {avg}"
            );
        }
    }
}
