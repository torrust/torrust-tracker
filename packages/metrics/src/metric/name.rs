use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::prometheus::PrometheusSerializable;

#[derive(Debug, Display, Clone, Eq, PartialEq, Default, Deserialize, Serialize, Hash, Ord, PartialOrd)]
pub struct MetricName(String);

impl MetricName {
    /// Creates a new `MetricName` instance.
    ///
    /// # Panics
    ///
    /// Panics if the provided name is empty.
    #[must_use]
    pub fn new(name: &str) -> Self {
        assert!(
            !name.is_empty(),
            "Metric name cannot be empty. It must have at least one character."
        );

        Self(name.to_owned())
    }
}

impl PrometheusSerializable for MetricName {
    fn to_prometheus(&self) -> String {
        // Metric names may contain ASCII letters, digits, underscores, and
        // colons. It must match the regex [a-zA-Z_:][a-zA-Z0-9_:]*.
        // If the metric name starts with, or contains, an invalid character:
        // replace character with underscore.

        self.0
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    if c.is_ascii_alphabetic() || c == '_' || c == ':' {
                        c
                    } else {
                        '_'
                    }
                } else if c.is_ascii_alphanumeric() || c == '_' || c == ':' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    mod serialization_of_metric_name_to_prometheus {

        use rstest::rstest;

        use crate::metric::MetricName;
        use crate::prometheus::PrometheusSerializable;

        #[rstest]
        #[case("valid name", "valid_name", "valid_name")]
        #[case("leading underscore", "_leading_underscore", "_leading_underscore")]
        #[case("leading colon", ":leading_colon", ":leading_colon")]
        #[case("leading lowercase", "v123", "v123")]
        #[case("leading uppercase", "V123", "V123")]
        fn valid_names_in_prometheus(#[case] case: &str, #[case] input: &str, #[case] output: &str) {
            assert_eq!(MetricName::new(input).to_prometheus(), output, "{case} failed: {input:?}");
        }

        #[rstest]
        #[case("invalid start 1", "9invalid_start", "_invalid_start")]
        #[case("invalid start 2", "@test", "_test")]
        #[case("invalid dash", "invalid-char", "invalid_char")]
        #[case("invalid spaces", "spaces are bad", "spaces_are_bad")]
        #[case("invalid special chars", "a!b@c#d$e%f^g&h*i(j)", "a_b_c_d_e_f_g_h_i_j_")]
        #[case("invalid slash", "my:metric/version", "my:metric_version")]
        #[case("all invalid characters", "!@#$%^&*()", "__________")]
        #[case("non_ascii_characters", "ñaca©", "_aca_")]
        fn names_that_need_changes_in_prometheus(#[case] case: &str, #[case] input: &str, #[case] output: &str) {
            assert_eq!(MetricName::new(input).to_prometheus(), output, "{case} failed: {input:?}");
        }

        #[test]
        #[should_panic(expected = "Metric name cannot be empty. It must have at least one character.")]
        fn empty_name() {
            let _name = MetricName::new("");
        }
    }
}
