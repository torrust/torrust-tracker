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

#[cfg(test)]
mod tests {
    use super::Error;
    use crate::metric_name;

    #[test]
    fn it_should_display_metric_name_collision_in_constructor() {
        let err = Error::MetricNameCollisionInConstructor {
            counter_names: vec!["hits_total".to_owned()],
            gauge_names: vec!["temperature".to_owned()],
        };
        let msg = err.to_string();
        assert!(msg.contains("unique"));
    }

    #[test]
    fn it_should_display_duplicate_metric_name_in_list() {
        let err = Error::DuplicateMetricNameInList {
            metric_name: metric_name!("hits_total"),
        };
        let msg = err.to_string();
        assert!(msg.contains("duplicate") || msg.contains("Duplicate"));
    }

    #[test]
    fn it_should_display_metric_name_collision_in_merge() {
        let err = Error::MetricNameCollisionInMerge {
            metric_name: metric_name!("hits_total"),
        };
        let msg = err.to_string();
        assert!(msg.contains("hits_total"));
    }

    #[test]
    fn it_should_display_metric_name_collision_adding() {
        let err = Error::MetricNameCollisionAdding {
            metric_name: metric_name!("hits_total"),
        };
        let msg = err.to_string();
        assert!(msg.contains("hits_total"));
    }

    #[test]
    fn it_should_be_cloneable() {
        let err = Error::MetricNameCollisionAdding {
            metric_name: metric_name!("hits_total"),
        };
        let cloned = err.clone();
        assert_eq!(err.to_string(), cloned.to_string());
    }
}
