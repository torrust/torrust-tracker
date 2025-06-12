use std::collections::btree_map::Iter;
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
    #[must_use]
    pub fn empty() -> Self {
        Self { items: BTreeMap::new() }
    }

    /// Insert a new label pair or update the value of an existing label.
    pub fn upsert(&mut self, name: LabelName, value: LabelValue) {
        self.items.insert(name, value);
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn contains_pair(&self, name: &LabelName, value: &LabelValue) -> bool {
        match self.items.get(name) {
            Some(existing_value) => existing_value == value,
            None => false,
        }
    }

    pub fn matches(&self, criteria: &LabelSet) -> bool {
        criteria.iter().all(|(name, value)| self.contains_pair(name, value))
    }

    pub fn iter(&self) -> Iter<'_, LabelName, LabelValue> {
        self.items.iter()
    }
}

impl Display for LabelSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .items
            .iter()
            .map(|(name, value)| format!("{name}=\"{value}\""))
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

        for (name, value) in vec {
            items.insert(name, value);
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
            .map(|(name, value)| SerializedLabel {
                name: name.clone(),
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
        if self.is_empty() {
            return String::new();
        }

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
    use std::hash::{DefaultHasher, Hash};

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
    fn it_should_handle_prometheus_format_with_special_characters() {
        let label_set: LabelSet = vec![("label_with_underscores", "value_with_underscores")].into();
        assert_eq!(
            label_set.to_prometheus(),
            r#"{label_with_underscores="value_with_underscores"}"#
        );
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
    fn it_should_allow_instantiation_from_vec_of_str_tuples() {
        let label_set: LabelSet = vec![("foo", "bar"), ("baz", "qux")].into();

        let mut expected = BTreeMap::new();
        expected.insert(LabelName::new("foo"), LabelValue::new("bar"));
        expected.insert(LabelName::new("baz"), LabelValue::new("qux"));

        assert_eq!(label_set, LabelSet { items: expected });
    }

    #[test]
    fn it_should_allow_instantiation_from_vec_of_string_tuples() {
        let label_set: LabelSet = vec![("foo".to_string(), "bar".to_string()), ("baz".to_string(), "qux".to_string())].into();

        let mut expected = BTreeMap::new();
        expected.insert(LabelName::new("foo"), LabelValue::new("bar"));
        expected.insert(LabelName::new("baz"), LabelValue::new("qux"));

        assert_eq!(label_set, LabelSet { items: expected });
    }

    #[test]
    fn it_should_allow_instantiation_from_vec_of_serialized_label() {
        use super::SerializedLabel;
        let label_set: LabelSet = vec![
            SerializedLabel {
                name: LabelName::new("foo"),
                value: LabelValue::new("bar"),
            },
            SerializedLabel {
                name: LabelName::new("baz"),
                value: LabelValue::new("qux"),
            },
        ]
        .into();

        let mut expected = BTreeMap::new();
        expected.insert(LabelName::new("foo"), LabelValue::new("bar"));
        expected.insert(LabelName::new("baz"), LabelValue::new("qux"));

        assert_eq!(label_set, LabelSet { items: expected });
    }

    #[test]
    fn it_should_allow_instantiation_from_array_of_string_tuples() {
        let arr: [(String, String); 2] = [("foo".to_string(), "bar".to_string()), ("baz".to_string(), "qux".to_string())];
        let label_set: LabelSet = arr.into();

        let mut expected = BTreeMap::new();

        expected.insert(LabelName::new("foo"), LabelValue::new("bar"));
        expected.insert(LabelName::new("baz"), LabelValue::new("qux"));

        assert_eq!(label_set, LabelSet { items: expected });
    }

    #[test]
    fn it_should_allow_instantiation_from_array_of_str_tuples() {
        let arr: [(&str, &str); 2] = [("foo", "bar"), ("baz", "qux")];
        let label_set: LabelSet = arr.into();

        let mut expected = BTreeMap::new();

        expected.insert(LabelName::new("foo"), LabelValue::new("bar"));
        expected.insert(LabelName::new("baz"), LabelValue::new("qux"));

        assert_eq!(label_set, LabelSet { items: expected });
    }

    #[test]
    fn it_should_be_comparable() {
        let a: LabelSet = (label_name!("x"), LabelValue::new("1")).into();
        let b: LabelSet = (label_name!("x"), LabelValue::new("1")).into();
        let c: LabelSet = (label_name!("y"), LabelValue::new("2")).into();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn it_should_be_allow_ordering() {
        let a: LabelSet = (label_name!("x"), LabelValue::new("1")).into();
        let b: LabelSet = (label_name!("y"), LabelValue::new("2")).into();

        assert!(a < b);
    }

    #[test]
    fn it_should_be_hashable() {
        let a: LabelSet = (label_name!("x"), LabelValue::new("1")).into();

        let mut hasher = DefaultHasher::new();

        a.hash(&mut hasher);
    }

    #[test]
    fn it_should_implement_clone() {
        let a: LabelSet = (label_name!("x"), LabelValue::new("1")).into();
        let _unused = a.clone();
    }

    #[test]
    fn it_should_check_if_empty() {
        let empty_set = LabelSet::empty();
        assert!(empty_set.is_empty());
    }

    #[test]
    fn it_should_check_if_non_empty() {
        let non_empty_set: LabelSet = (label_name!("label"), LabelValue::new("value")).into();
        assert!(!non_empty_set.is_empty());
    }

    #[test]
    fn it_should_create_an_empty_label_set() {
        let empty_set = LabelSet::empty();
        assert!(empty_set.is_empty());
    }

    #[test]
    fn it_should_check_if_contains_specific_label_pair() {
        let label_set: LabelSet = vec![("service", "tracker"), ("protocol", "http")].into();

        // Test existing pair
        assert!(label_set.contains_pair(&LabelName::new("service"), &LabelValue::new("tracker")));
        assert!(label_set.contains_pair(&LabelName::new("protocol"), &LabelValue::new("http")));

        // Test non-existing name
        assert!(!label_set.contains_pair(&LabelName::new("missing"), &LabelValue::new("value")));

        // Test existing name with wrong value
        assert!(!label_set.contains_pair(&LabelName::new("service"), &LabelValue::new("wrong")));
    }

    #[test]
    fn it_should_match_against_criteria() {
        let label_set: LabelSet = vec![("service", "tracker"), ("protocol", "http"), ("version", "v1")].into();

        // Empty criteria should match any label set
        assert!(label_set.matches(&LabelSet::empty()));

        // Single matching criterion
        let single_criteria: LabelSet = vec![("service", "tracker")].into();
        assert!(label_set.matches(&single_criteria));

        // Multiple matching criteria
        let multiple_criteria: LabelSet = vec![("service", "tracker"), ("protocol", "http")].into();
        assert!(label_set.matches(&multiple_criteria));

        // Non-matching criterion
        let non_matching: LabelSet = vec![("service", "wrong")].into();
        assert!(!label_set.matches(&non_matching));

        // Partially matching criteria (one matches, one doesn't)
        let partial_matching: LabelSet = vec![("service", "tracker"), ("missing", "value")].into();
        assert!(!label_set.matches(&partial_matching));

        // Criteria with label not in original set
        let missing_label: LabelSet = vec![("missing_label", "value")].into();
        assert!(!label_set.matches(&missing_label));
    }

    #[test]
    fn it_should_allow_iteration_over_label_pairs() {
        let label_set: LabelSet = vec![("service", "tracker"), ("protocol", "http")].into();

        let mut count = 0;

        for (name, value) in label_set.iter() {
            count += 1;
            // Verify we can access name and value
            assert!(!name.to_string().is_empty());
            assert!(!value.to_string().is_empty());
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn it_should_display_empty_label_set() {
        let empty_set = LabelSet::empty();
        assert_eq!(empty_set.to_string(), "{}");
    }

    #[test]
    fn it_should_serialize_empty_label_set_to_prometheus_format() {
        let empty_set = LabelSet::empty();
        assert_eq!(empty_set.to_prometheus(), "");
    }

    #[test]
    fn it_should_maintain_order_in_iteration() {
        let label_set: LabelSet = vec![("z_label", "z_value"), ("a_label", "a_value"), ("m_label", "m_value")].into();

        let mut labels: Vec<String> = vec![];
        for (name, _) in label_set.iter() {
            labels.push(name.to_string());
        }

        // Should be in alphabetical order
        assert_eq!(labels, vec!["a_label", "m_label", "z_label"]);
    }
}
