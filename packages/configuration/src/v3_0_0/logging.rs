//! Logging configuration for `v3_0_0`.
//!
//! Contains the `Logging` configuration struct, the `Threshold` level enum,
//! and the `TraceStyle` enum. This module only describes the logging configuration: the tracker
//! application installs the logging and applies the configuration (`torrust_tracker_lib::bootstrap::logging`).
//!
//! **Field type convention**: use typed newtypes for fields with domain constraints —
//! not `String` or other unvalidated primitives. See [`crate::v3_0_0::public_url`].
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Logging {
    /// Trace filter level. Possible values are: `Off`, `Error`, `Warn`, `Info`,
    /// `Debug` and `Trace`. Default is `Info`.
    #[serde(default = "Logging::default_trace_filter")]
    pub trace_filter: Threshold,

    /// Trace output style. Default is `Full`.
    #[serde(default = "Logging::default_trace_style")]
    pub trace_style: TraceStyle,
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            trace_filter: Self::default_trace_filter(),
            trace_style: Self::default_trace_style(),
        }
    }
}

impl Logging {
    const fn default_trace_filter() -> Threshold {
        Threshold::Info
    }

    const fn default_trace_style() -> TraceStyle {
        TraceStyle::Full
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Threshold {
    /// A threshold lower than all security levels.
    Off,
    /// Corresponds to the `Error` security level.
    Error,
    /// Corresponds to the `Warn` security level.
    Warn,
    /// Corresponds to the `Info` security level.
    Info,
    /// Corresponds to the `Debug` security level.
    Debug,
    /// Corresponds to the `Trace` security level.
    Trace,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TraceStyle {
    /// Standard human-readable output.
    Full,
    /// Pretty-printed output with colours.
    Pretty,
    /// Compact single-line output.
    Compact,
    /// Structured JSON output.
    Json,
}

impl std::fmt::Display for TraceStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style = match self {
            Self::Full => "Full Style",
            Self::Pretty => "Pretty Style",
            Self::Compact => "Compact Style",
            Self::Json => "Json Format",
        };

        f.write_str(style)
    }
}

#[cfg(test)]
mod tests {
    use super::{Logging, Threshold, TraceStyle};

    #[test]
    fn it_should_use_info_and_full_as_the_default_logging_configuration() {
        // Arrange
        let expected_trace_filter = Threshold::Info;
        let expected_trace_style = TraceStyle::Full;

        // Act
        let logging = Logging::default();

        // Assert
        assert_eq!(logging.trace_filter, expected_trace_filter);
        assert_eq!(logging.trace_style, expected_trace_style);
    }

    #[test]
    fn it_should_deserialize_all_supported_trace_styles() {
        // Arrange
        let styles = [
            ("full", TraceStyle::Full),
            ("pretty", TraceStyle::Pretty),
            ("compact", TraceStyle::Compact),
            ("json", TraceStyle::Json),
        ];

        // Act and Assert
        for (value, expected_style) in styles {
            let logging: Logging = toml::from_str(&format!("trace_filter = \"info\"\ntrace_style = \"{value}\""))
                .expect("trace style should deserialize");

            assert_eq!(logging.trace_style, expected_style, "trace style: {value}");
        }
    }

    #[test]
    fn it_should_reject_an_unsupported_trace_style() {
        // Arrange
        let logging_toml = "trace_filter = \"info\"\ntrace_style = \"default\"";

        // Act
        let result = toml::from_str::<Logging>(logging_toml);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn it_should_reject_the_removed_threshold_field() {
        // Arrange: the old v2 key `threshold` must not be accepted by v3
        let logging_toml = "threshold = \"info\"";

        // Act
        let result = toml::from_str::<Logging>(logging_toml);

        // Assert
        assert!(result.is_err());
    }
}
