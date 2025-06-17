use crate::counter::Counter;
use crate::gauge::Gauge;
use crate::label::LabelSet;
use crate::metric::aggregate::sum::Sum as MetricSumTrait;
use crate::metric::MetricName;
use crate::metric_collection::{MetricCollection, MetricKindCollection};

pub trait Sum {
    fn sum(&self, metric_name: &MetricName, label_set_criteria: &LabelSet) -> Option<f64>;
}

impl Sum for MetricCollection {
    fn sum(&self, metric_name: &MetricName, label_set_criteria: &LabelSet) -> Option<f64> {
        if let Some(value) = self.counters.sum(metric_name, label_set_criteria) {
            return Some(value);
        }

        if let Some(value) = self.gauges.sum(metric_name, label_set_criteria) {
            return Some(value);
        }

        None
    }
}

impl Sum for MetricKindCollection<Counter> {
    fn sum(&self, metric_name: &MetricName, label_set_criteria: &LabelSet) -> Option<f64> {
        #[allow(clippy::cast_precision_loss)]
        self.metrics
            .get(metric_name)
            .map(|metric| metric.sum(label_set_criteria) as f64)
    }
}

impl Sum for MetricKindCollection<Gauge> {
    fn sum(&self, metric_name: &MetricName, label_set_criteria: &LabelSet) -> Option<f64> {
        self.metrics.get(metric_name).map(|metric| metric.sum(label_set_criteria))
    }
}

#[cfg(test)]
mod tests {

    mod it_should_allow_summing_all_metric_samples_containing_some_given_labels {

        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use crate::label::LabelValue;
        use crate::label_name;
        use crate::metric_collection::aggregate::sum::Sum;

        #[test]
        fn type_counter_with_two_samples() {
            use crate::label::LabelSet;
            use crate::metric_collection::MetricCollection;
            use crate::metric_name;

            let metric_name = metric_name!("test_counter");

            let mut collection = MetricCollection::default();

            collection
                .increment_counter(
                    &metric_name!("test_counter"),
                    &(label_name!("label_1"), LabelValue::new("value_1")).into(),
                    DurationSinceUnixEpoch::from_secs(1),
                )
                .unwrap();

            collection
                .increment_counter(
                    &metric_name!("test_counter"),
                    &(label_name!("label_2"), LabelValue::new("value_2")).into(),
                    DurationSinceUnixEpoch::from_secs(1),
                )
                .unwrap();

            assert_eq!(collection.sum(&metric_name, &LabelSet::empty()), Some(2.0));
            assert_eq!(
                collection.sum(&metric_name, &(label_name!("label_1"), LabelValue::new("value_1")).into()),
                Some(1.0)
            );
        }

        #[test]
        fn type_gauge_with_two_samples() {
            use crate::label::LabelSet;
            use crate::metric_collection::MetricCollection;
            use crate::metric_name;

            let metric_name = metric_name!("test_gauge");

            let mut collection = MetricCollection::default();

            collection
                .increment_gauge(
                    &metric_name!("test_gauge"),
                    &(label_name!("label_1"), LabelValue::new("value_1")).into(),
                    DurationSinceUnixEpoch::from_secs(1),
                )
                .unwrap();

            collection
                .increment_gauge(
                    &metric_name!("test_gauge"),
                    &(label_name!("label_2"), LabelValue::new("value_2")).into(),
                    DurationSinceUnixEpoch::from_secs(1),
                )
                .unwrap();

            assert_eq!(collection.sum(&metric_name, &LabelSet::empty()), Some(2.0));
            assert_eq!(
                collection.sum(&metric_name, &(label_name!("label_1"), LabelValue::new("value_1")).into()),
                Some(1.0)
            );
        }
    }
}
