use super::{LabelName, LabelValue};
use crate::prometheus::PrometheusSerializable;

pub type LabelPair = (LabelName, LabelValue);

// Generic implementation for any tuple (A, B) where A and B implement PrometheusSerializable
impl<A: PrometheusSerializable, B: PrometheusSerializable> PrometheusSerializable for (A, B) {
    fn to_prometheus(&self) -> String {
        format!("{}=\"{}\"", self.0.to_prometheus(), self.1.to_prometheus())
    }
}

#[cfg(test)]
mod tests {
    mod serialization_of_label_pair_to_prometheus {
        use crate::label::LabelValue;
        use crate::label_name;
        use crate::prometheus::PrometheusSerializable;

        #[test]
        fn test_label_pair_serialization_to_prometheus() {
            let label_pair = (label_name!("label_name"), LabelValue::new("value"));
            assert_eq!(label_pair.to_prometheus(), r#"label_name="value""#);

            let label_pair = (&label_name!("label_name"), &LabelValue::new("value"));
            assert_eq!(label_pair.to_prometheus(), r#"label_name="value""#);
        }
    }
}
