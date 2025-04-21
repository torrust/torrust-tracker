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
        assert!(!name.is_empty(), "Metric name cannot be empty.");
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

#[macro_export]
macro_rules! metric_name {
    ("") => {
        compile_error!("Metric name cannot be empty");
    };
    ($name:literal) => {
        $crate::metric::name::MetricName::new($name)
    };
    ($name:ident) => {
        $crate::metric::name::MetricName::new($name)
    };
}

#[cfg(test)]
mod tests {

    mod serialization_of_metric_name_to_prometheus {

        use crate::metric::name::MetricName;
        use crate::prometheus::PrometheusSerializable;

        #[test]
        fn valid_names_in_prometheus() {
            assert_eq!(metric_name!("valid_name").to_prometheus(), "valid_name");
            assert_eq!(metric_name!("_leading_underscore").to_prometheus(), "_leading_underscore");
            assert_eq!(metric_name!(":leading_colon").to_prometheus(), ":leading_colon");
            assert_eq!(metric_name!("v123").to_prometheus(), "v123"); // leading lowercase
            assert_eq!(metric_name!("V123").to_prometheus(), "V123"); // leading lowercase
        }

        #[test]
        fn names_that_need_changes_in_prometheus() {
            assert_eq!(metric_name!("9invalid_start").to_prometheus(), "_invalid_start");
            assert_eq!(metric_name!("@test").to_prometheus(), "_test");
            assert_eq!(metric_name!("invalid-char").to_prometheus(), "invalid_char");
            assert_eq!(metric_name!("spaces are bad").to_prometheus(), "spaces_are_bad");
            assert_eq!(metric_name!("a!b@c#d$e%f^g&h*i(j)").to_prometheus(), "a_b_c_d_e_f_g_h_i_j_");
            assert_eq!(metric_name!("my:metric/version").to_prometheus(), "my:metric_version");
            assert_eq!(metric_name!("!@#$%^&*()").to_prometheus(), "__________");
            assert_eq!(metric_name!("ñaca©").to_prometheus(), "_aca_");
        }

        #[test]
        #[should_panic(expected = "Metric name cannot be empty.")]
        fn empty_name() {
            let _name = MetricName::new("");
        }
    }
}
