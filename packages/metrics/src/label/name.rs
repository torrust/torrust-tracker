use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::prometheus::PrometheusSerializable;

#[derive(Debug, Display, Clone, Eq, PartialEq, Default, Deserialize, Serialize, Hash, Ord, PartialOrd)]
pub struct LabelName(String);

impl LabelName {
    /// Creates a new `LabelName` instance.
    ///
    /// # Panics
    ///
    /// Panics if the provided name is empty.
    #[must_use]
    pub fn new(name: &str) -> Self {
        assert!(!name.is_empty(), "Label name cannot be empty.");
        Self(name.to_owned())
    }
}

impl PrometheusSerializable for LabelName {
    /// In Prometheus:
    ///
    /// - Labels may contain ASCII letters, numbers, as well as underscores.
    ///   They must match the regex [a-zA-Z_][a-zA-Z0-9_]*.
    /// - Label names beginning with __ (two "_") are reserved for internal
    ///   use.
    /// - Label values may contain any Unicode characters.
    /// - Labels with an empty label value are considered equivalent to
    ///   labels that do not exist.
    ///
    /// The label name is changed:
    ///
    /// - If a label name starts with, or contains, an invalid character:
    ///   replace character with underscore.
    /// - If th label name starts with two underscores:
    ///   add additional underscore (three underscores total)
    fn to_prometheus(&self) -> String {
        // Replace invalid characters with underscore
        let processed: String = self
            .0
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    if c.is_ascii_alphabetic() || c == '_' {
                        c
                    } else {
                        '_'
                    }
                } else if c.is_ascii_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        // If the label name starts with two underscores, add an additional
        if processed.starts_with("__") && !processed.starts_with("___") {
            format!("_{processed}")
        } else {
            processed
        }
    }
}

#[macro_export]
macro_rules! label_name {
    ("") => {
        compile_error!("Label name cannot be empty");
    };
    ($name:literal) => {
        $crate::label::name::LabelName::new($name)
    };
    ($name:ident) => {
        $crate::label::name::LabelName::new($name)
    };
}
#[cfg(test)]
mod tests {
    mod serialization_of_label_name_to_prometheus {
        use rstest::rstest;

        use crate::label::LabelName;
        use crate::prometheus::PrometheusSerializable;

        #[rstest]
        #[case("1 valid name", "valid_name", "valid_name")]
        #[case("2 leading underscore", "_leading_underscore", "_leading_underscore")]
        #[case("3 leading lowercase", "v123", "v123")]
        #[case("4 leading uppercase", "V123", "V123")]
        fn valid_names_in_prometheus(#[case] case: &str, #[case] input: &str, #[case] output: &str) {
            assert_eq!(label_name!(input).to_prometheus(), output, "{case} failed: {input:?}");
        }

        #[rstest]
        #[case("1 invalid start 1", "9invalid_start", "_invalid_start")]
        #[case("2 invalid start 2", "@test", "_test")]
        #[case("3 invalid dash", "invalid-char", "invalid_char")]
        #[case("4 invalid spaces", "spaces are bad", "spaces_are_bad")]
        #[case("5 invalid special chars", "a!b@c#d$e%f^g&h*i(j)", "a_b_c_d_e_f_g_h_i_j_")]
        #[case("6 invalid colon", "my:metric/version", "my_metric_version")]
        #[case("7 all invalid characters", "!@#$%^&*()", "__________")]
        #[case("8 non_ascii_characters", "ñaca©", "_aca_")]
        fn names_that_need_changes_in_prometheus(#[case] case: &str, #[case] input: &str, #[case] output: &str) {
            assert_eq!(label_name!(input).to_prometheus(), output, "{case} failed: {input:?}");
        }

        #[rstest]
        #[case("1 double underscore start", "__private", "___private")]
        #[case("2 double underscore only", "__", "___")]
        #[case("3 processed to double underscore", "^^name", "___name")]
        #[case("4 processed to double underscore after first char", "0__name", "___name")]
        fn names_starting_with_double_underscore(#[case] case: &str, #[case] input: &str, #[case] output: &str) {
            assert_eq!(label_name!(input).to_prometheus(), output, "{case} failed: {input:?}");
        }

        #[test]
        #[should_panic(expected = "Label name cannot be empty.")]
        fn empty_name() {
            let _name = LabelName::new("");
        }
    }
}
