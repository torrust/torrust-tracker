use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::prometheus::PrometheusSerializable;

#[derive(Debug, Display, Clone, Eq, PartialEq, Default, Deserialize, Serialize, Hash, Ord, PartialOrd)]
pub struct MetricDescription(String);

impl MetricDescription {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }
}

impl PrometheusSerializable for MetricDescription {
    fn to_prometheus(&self) -> String {
        self.0.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_created_from_a_string_reference() {
        let metric = MetricDescription::new("Metric description");
        assert_eq!(metric.0, "Metric description");
    }

    #[test]
    fn it_serializes_to_prometheus() {
        let label_value = MetricDescription::new("name");
        assert_eq!(label_value.to_prometheus(), "name");
    }

    #[test]
    fn it_should_be_displayed() {
        let metric = MetricDescription::new("Metric description");
        assert_eq!(metric.to_string(), "Metric description");
    }
}
