use crate::counter::Counter;
use crate::gauge::Gauge;
use crate::label::LabelSet;
use crate::metric::aggregate::avg::Avg as MetricAvgTrait;
use crate::metric::MetricName;
use crate::metric_collection::{MetricCollection, MetricKindCollection};

pub trait Avg {
    fn avg(&self, metric_name: &MetricName, label_set_criteria: &LabelSet) -> Option<f64>;
}

impl Avg for MetricCollection {
    fn avg(&self, metric_name: &MetricName, label_set_criteria: &LabelSet) -> Option<f64> {
        if let Some(value) = self.counters.avg(metric_name, label_set_criteria) {
            return Some(value);
        }

        if let Some(value) = self.gauges.avg(metric_name, label_set_criteria) {
            return Some(value);
        }

        None
    }
}

impl Avg for MetricKindCollection<Counter> {
    fn avg(&self, metric_name: &MetricName, label_set_criteria: &LabelSet) -> Option<f64> {
        self.metrics.get(metric_name).map(|metric| metric.avg(label_set_criteria))
    }
}

impl Avg for MetricKindCollection<Gauge> {
    fn avg(&self, metric_name: &MetricName, label_set_criteria: &LabelSet) -> Option<f64> {
        self.metrics.get(metric_name).map(|metric| metric.avg(label_set_criteria))
    }
}

#[cfg(test)]
mod tests {

    mod it_should_allow_averaging_all_metric_samples_containing_some_given_labels {

        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use crate::label::LabelValue;
        use crate::label_name;
        use crate::metric_collection::aggregate::avg::Avg;

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

            // Two samples with value 1 each, average should be 1.0
            assert_eq!(collection.avg(&metric_name, &LabelSet::empty()), Some(1.0));
            assert_eq!(
                collection.avg(&metric_name, &(label_name!("label_1"), LabelValue::new("value_1")).into()),
                Some(1.0)
            );
        }

        #[test]
        fn type_counter_with_different_values() {
            use crate::label::LabelSet;
            use crate::metric_collection::MetricCollection;
            use crate::metric_name;

            let metric_name = metric_name!("test_counter");

            let mut collection = MetricCollection::default();

            // First increment: value goes from 0 to 1
            collection
                .increment_counter(
                    &metric_name!("test_counter"),
                    &(label_name!("label_1"), LabelValue::new("value_1")).into(),
                    DurationSinceUnixEpoch::from_secs(1),
                )
                .unwrap();

            // Second increment on the same label: value goes from 1 to 2
            collection
                .increment_counter(
                    &metric_name!("test_counter"),
                    &(label_name!("label_1"), LabelValue::new("value_1")).into(),
                    DurationSinceUnixEpoch::from_secs(2),
                )
                .unwrap();

            // Create another counter with a different value
            collection
                .set_counter(
                    &metric_name!("test_counter"),
                    &(label_name!("label_2"), LabelValue::new("value_2")).into(),
                    4,
                    DurationSinceUnixEpoch::from_secs(3),
                )
                .unwrap();

            // Average of 2 and 4 should be 3.0
            assert_eq!(collection.avg(&metric_name, &LabelSet::empty()), Some(3.0));
            assert_eq!(
                collection.avg(&metric_name, &(label_name!("label_1"), LabelValue::new("value_1")).into()),
                Some(2.0)
            );
            assert_eq!(
                collection.avg(&metric_name, &(label_name!("label_2"), LabelValue::new("value_2")).into()),
                Some(4.0)
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
                .set_gauge(
                    &metric_name!("test_gauge"),
                    &(label_name!("label_1"), LabelValue::new("value_1")).into(),
                    2.0,
                    DurationSinceUnixEpoch::from_secs(1),
                )
                .unwrap();

            collection
                .set_gauge(
                    &metric_name!("test_gauge"),
                    &(label_name!("label_2"), LabelValue::new("value_2")).into(),
                    4.0,
                    DurationSinceUnixEpoch::from_secs(1),
                )
                .unwrap();

            // Average of 2.0 and 4.0 should be 3.0
            assert_eq!(collection.avg(&metric_name, &LabelSet::empty()), Some(3.0));
            assert_eq!(
                collection.avg(&metric_name, &(label_name!("label_1"), LabelValue::new("value_1")).into()),
                Some(2.0)
            );
        }

        #[test]
        fn type_gauge_with_negative_values() {
            use crate::label::LabelSet;
            use crate::metric_collection::MetricCollection;
            use crate::metric_name;

            let metric_name = metric_name!("test_gauge");

            let mut collection = MetricCollection::default();

            collection
                .set_gauge(
                    &metric_name!("test_gauge"),
                    &(label_name!("label_1"), LabelValue::new("value_1")).into(),
                    -2.0,
                    DurationSinceUnixEpoch::from_secs(1),
                )
                .unwrap();

            collection
                .set_gauge(
                    &metric_name!("test_gauge"),
                    &(label_name!("label_2"), LabelValue::new("value_2")).into(),
                    6.0,
                    DurationSinceUnixEpoch::from_secs(1),
                )
                .unwrap();

            // Average of -2.0 and 6.0 should be 2.0
            assert_eq!(collection.avg(&metric_name, &LabelSet::empty()), Some(2.0));
        }

        #[test]
        fn nonexistent_metric() {
            use crate::label::LabelSet;
            use crate::metric_collection::MetricCollection;
            use crate::metric_name;

            let collection = MetricCollection::default();

            assert_eq!(collection.avg(&metric_name!("nonexistent"), &LabelSet::empty()), None);
        }
    }
}
