use std::collections::BTreeMap;
use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{LabelName, LabelPair, LabelValue};
use crate::prometheus::PrometheusSerializable;

#[derive(Debug, Clone, Eq, PartialEq, Default, Ord, PartialOrd, Hash)]
pub struct LabelSet {
    items: BTreeMap<LabelName, LabelValue>,
}

impl LabelSet {
    /// Insert a new label pair or update the value of an existing label.
    pub fn upsert(&mut self, key: LabelName, value: LabelValue) {
        self.items.insert(key, value);
    }
}

impl Display for LabelSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .items
            .iter()
            .map(|(key, value)| format!("{key}=\"{value}\""))
            .collect::<Vec<_>>()
            .join(",");

        write!(f, "{{{items}}}")
    }
}

impl From<BTreeMap<LabelName, LabelValue>> for LabelSet {
    fn from(values: BTreeMap<LabelName, LabelValue>) -> Self {
        Self { items: values }
    }
}

impl From<Vec<(&str, &str)>> for LabelSet {
    fn from(vec: Vec<(&str, &str)>) -> Self {
        let mut items = BTreeMap::new();

        for (name, value) in vec {
            items.insert(LabelName::new(name), LabelValue::new(value));
        }

        Self { items }
    }
}

impl From<Vec<(String, String)>> for LabelSet {
    fn from(vec: Vec<(String, String)>) -> Self {
        let mut items = BTreeMap::new();

        for (name, value) in vec {
            items.insert(LabelName::new(&name), LabelValue::new(&value));
        }

        Self { items }
    }
}

impl From<Vec<LabelPair>> for LabelSet {
    fn from(vec: Vec<LabelPair>) -> Self {
        let mut items = BTreeMap::new();

        for (key, value) in vec {
            items.insert(key, value);
        }

        Self { items }
    }
}

impl From<Vec<SerializedLabel>> for LabelSet {
    fn from(vec: Vec<SerializedLabel>) -> Self {
        let mut items = BTreeMap::new();

        for serialized_label in vec {
            items.insert(serialized_label.name, serialized_label.value);
        }

        Self { items }
    }
}

impl<const N: usize> From<[LabelPair; N]> for LabelSet {
    fn from(arr: [LabelPair; N]) -> Self {
        let values = BTreeMap::from(arr);
        Self { items: values }
    }
}

impl<const N: usize> From<[(String, String); N]> for LabelSet {
    fn from(arr: [(String, String); N]) -> Self {
        let values = arr
            .iter()
            .map(|(name, value)| (LabelName::new(name), LabelValue::new(value)))
            .collect::<BTreeMap<_, _>>();
        Self { items: values }
    }
}

impl<const N: usize> From<[(&str, &str); N]> for LabelSet {
    fn from(arr: [(&str, &str); N]) -> Self {
        let values = arr
            .iter()
            .map(|(name, value)| (LabelName::new(name), LabelValue::new(value)))
            .collect::<BTreeMap<_, _>>();
        Self { items: values }
    }
}

impl From<LabelPair> for LabelSet {
    fn from(label_pair: LabelPair) -> Self {
        let mut set = BTreeMap::new();

        set.insert(label_pair.0, label_pair.1);

        Self { items: set }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Deserialize, Serialize)]
struct SerializedLabel {
    name: LabelName,
    value: LabelValue,
}

impl Serialize for LabelSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.items
            .iter()
            .map(|(key, value)| SerializedLabel {
                name: key.clone(),
                value: value.clone(),
            })
            .collect::<Vec<_>>()
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LabelSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serialized_labels = Vec::<SerializedLabel>::deserialize(deserializer)?;

        Ok(LabelSet::from(serialized_labels))
    }
}

impl PrometheusSerializable for LabelSet {
    fn to_prometheus(&self) -> String {
        let items = self.items.iter().fold(String::new(), |mut output, label_pair| {
            if !output.is_empty() {
                output.push(',');
            }

            output.push_str(&label_pair.to_prometheus());

            output
        });

        format!("{{{items}}}")
    }
}

#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;

    use pretty_assertions::assert_eq;

    use super::{LabelName, LabelValue};
    use crate::label::LabelSet;
    use crate::label_name;
    use crate::prometheus::PrometheusSerializable;

    fn sample_vec_of_label_pairs() -> Vec<(LabelName, LabelValue)> {
        sample_array_of_label_pairs().into()
    }

    fn sample_array_of_label_pairs() -> [(LabelName, LabelValue); 3] {
        [
            (label_name!("server_service_binding_protocol"), LabelValue::new("http")),
            (label_name!("server_service_binding_ip"), LabelValue::new("0.0.0.0")),
            (label_name!("server_service_binding_port"), LabelValue::new("7070")),
        ]
    }

    #[test]
    fn it_should_allow_instantiation_from_an_array_of_label_pairs() {
        let label_set: LabelSet = sample_array_of_label_pairs().into();

        assert_eq!(
            label_set,
            LabelSet {
                items: BTreeMap::from(sample_array_of_label_pairs())
            }
        );
    }

    #[test]
    fn it_should_allow_instantiation_from_a_vec_of_label_pairs() {
        let label_set: LabelSet = sample_vec_of_label_pairs().into();

        assert_eq!(
            label_set,
            LabelSet {
                items: BTreeMap::from(sample_array_of_label_pairs())
            }
        );
    }

    #[test]
    fn it_should_allow_instantiation_from_a_b_tree_map() {
        let label_set: LabelSet = BTreeMap::from(sample_array_of_label_pairs()).into();

        assert_eq!(
            label_set,
            LabelSet {
                items: BTreeMap::from(sample_array_of_label_pairs())
            }
        );
    }

    #[test]
    fn it_should_allow_instantiation_from_a_label_pair() {
        let label_set: LabelSet = (label_name!("label_name"), LabelValue::new("value")).into();

        assert_eq!(
            label_set,
            LabelSet {
                items: BTreeMap::from([(label_name!("label_name"), LabelValue::new("value"))])
            }
        );
    }

    #[test]
    fn it_should_allow_inserting_a_new_label_pair() {
        let mut label_set = LabelSet::default();

        label_set.upsert(label_name!("label_name"), LabelValue::new("value"));

        assert_eq!(
            label_set.items.get(&label_name!("label_name")).unwrap(),
            &LabelValue::new("value")
        );
    }

    #[test]
    fn it_should_allow_updating_a_label_value() {
        let mut label_set = LabelSet::default();

        label_set.upsert(label_name!("label_name"), LabelValue::new("old value"));
        label_set.upsert(label_name!("label_name"), LabelValue::new("new value"));

        assert_eq!(
            label_set.items.get(&label_name!("label_name")).unwrap(),
            &LabelValue::new("new value")
        );
    }

    #[test]
    fn it_should_allow_serializing_to_json_as_an_array_of_label_objects() {
        let label_set = LabelSet::from((label_name!("label_name"), LabelValue::new("label value")));

        let json = serde_json::to_string(&label_set).unwrap();

        assert_eq!(
            formatjson::format_json(&json).unwrap(),
            formatjson::format_json(
                r#"
                [
                    {
                        "name": "label_name",
                        "value": "label value"
                    }
                ]
                "#
            )
            .unwrap()
        );
    }

    #[test]
    fn it_should_allow_deserializing_from_json_as_an_array_of_label_objects() {
        let json = formatjson::format_json(
            r#"
                [
                    {
                        "name": "label_name",
                        "value": "label value"
                    }
                ]
                "#,
        )
        .unwrap();

        let label_set: LabelSet = serde_json::from_str(&json).unwrap();

        assert_eq!(
            label_set,
            LabelSet::from((label_name!("label_name"), LabelValue::new("label value")))
        );
    }

    #[test]
    fn it_should_allow_serializing_to_prometheus_format() {
        let label_set = LabelSet::from((label_name!("label_name"), LabelValue::new("label value")));

        assert_eq!(label_set.to_prometheus(), r#"{label_name="label value"}"#);
    }

    #[test]
    fn it_should_alphabetically_order_labels_in_prometheus_format() {
        let label_set = LabelSet::from([
            (label_name!("b_label_name"), LabelValue::new("b label value")),
            (label_name!("a_label_name"), LabelValue::new("a label value")),
        ]);

        assert_eq!(
            label_set.to_prometheus(),
            r#"{a_label_name="a label value",b_label_name="b label value"}"#
        );
    }

    #[test]
    fn it_should_allow_displaying() {
        let label_set = LabelSet::from((label_name!("label_name"), LabelValue::new("label value")));

        assert_eq!(label_set.to_string(), r#"{label_name="label value"}"#);
    }
}
