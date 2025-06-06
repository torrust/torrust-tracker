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

impl From<String> for LabelValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    use crate::label::value::LabelValue;
    use crate::prometheus::PrometheusSerializable;

    #[test]
    fn it_serializes_to_prometheus() {
        let label_value = LabelValue::new("value");
        assert_eq!(label_value.to_prometheus(), "value");
    }

    #[test]
    fn it_could_be_initialized_from_str() {
        let lv = LabelValue::new("abc");
        assert_eq!(lv.0, "abc");
    }

    #[test]
    fn it_should_allow_to_create_an_ignored_label_value() {
        let lv = LabelValue::ignore();
        assert_eq!(lv.0, "");
    }

    #[test]
    fn it_should_be_converted_from_string() {
        let s = String::from("foo");
        let lv: LabelValue = s.clone().into();
        assert_eq!(lv.0, s);
    }

    #[test]
    fn it_should_be_comparable() {
        let a = LabelValue::new("x");
        let b = LabelValue::new("x");
        let c = LabelValue::new("y");

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn it_should_be_allow_ordering() {
        let a = LabelValue::new("x");
        let b = LabelValue::new("y");

        assert!(a < b);
    }

    #[test]
    fn it_should_be_hashable() {
        let a = LabelValue::new("x");
        let mut hasher = DefaultHasher::new();
        a.hash(&mut hasher);
    }

    #[test]
    fn it_should_implement_clone() {
        let a = LabelValue::new("x");
        let _unused = a.clone();
    }

    #[test]
    fn it_should_implement_display() {
        let a = LabelValue::new("x");
        assert_eq!(format!("{a}"), "x");
    }
}
