use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Clone, Eq, PartialEq, Default, Deserialize, Serialize, Hash, Ord, PartialOrd)]
pub struct MetricDescription(String);

impl MetricDescription {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
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
    fn it_should_be_displayed() {
        let metric = MetricDescription::new("Metric description");
        assert_eq!(metric.to_string(), "Metric description");
    }
}
