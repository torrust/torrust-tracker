use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::prometheus::PrometheusSerializable;

#[derive(Debug, Display, Clone, Eq, PartialEq, Default, Deserialize, Serialize, Hash, Ord, PartialOrd)]
pub struct LabelValue(String);

impl LabelValue {
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self(value.to_owned())
    }

    /// Empty label values are ignored in Prometheus.
    #[must_use]
    pub fn ignore() -> Self {
        Self(String::default())
    }
}

impl PrometheusSerializable for LabelValue {
    fn to_prometheus(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::label::value::LabelValue;
    use crate::prometheus::PrometheusSerializable;

    #[test]
    fn it_serializes_to_prometheus() {
        let label_value = LabelValue::new("value");
        assert_eq!(label_value.to_prometheus(), "value");
    }
}
