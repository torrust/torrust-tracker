use crate::metric::MetricName;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Metric names must be unique across all metrics types.")]
    MetricNameCollisionInConstructor {
        counter_names: Vec<String>,
        gauge_names: Vec<String>,
    },

    #[error("Found duplicate metric name in list. Metric names must be unique across all metrics types.")]
    DuplicateMetricNameInList { metric_name: MetricName },

    #[error("Cannot merge metric '{metric_name}': it already exists in the current collection")]
    MetricNameCollisionInMerge { metric_name: MetricName },

    #[error("Cannot create metric with name '{metric_name}': another metric with this name already exists")]
    MetricNameCollisionAdding { metric_name: MetricName },
}
