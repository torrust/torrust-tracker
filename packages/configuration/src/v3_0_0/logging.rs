//! Logging configuration and setup for `v3_0_0`.
//!
//! Contains the `Logging` configuration struct, the `Threshold` level enum,
//! the `TraceStyle` enum, and the `setup()` / `tracing_init()` helpers.
//!
//! **Field type convention**: use typed newtypes for fields with domain constraints —
//! not `String` or other unvalidated primitives. See [`crate::v3_0_0::public_url`].
use std::sync::Once;

use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

static INIT: Once = Once::new();

#[allow(clippy::struct_excessive_bools)]
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
    fn default_trace_filter() -> Threshold {
        Threshold::Info
    }

    fn default_trace_style() -> TraceStyle {
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

/// Redirects log output to stdout using the configured filter and style.
pub fn setup(cfg: &Logging) {
    let tracing_level = map_to_tracing_level_filter(&cfg.trace_filter);

    if tracing_level == LevelFilter::OFF {
        return;
    }

    INIT.call_once(|| {
        tracing_init(tracing_level, &cfg.trace_style);
    });
}

fn map_to_tracing_level_filter(trace_filter: &Threshold) -> LevelFilter {
    match trace_filter {
        Threshold::Off => LevelFilter::OFF,
        Threshold::Error => LevelFilter::ERROR,
        Threshold::Warn => LevelFilter::WARN,
        Threshold::Info => LevelFilter::INFO,
        Threshold::Debug => LevelFilter::DEBUG,
        Threshold::Trace => LevelFilter::TRACE,
    }
}

fn tracing_init(filter: LevelFilter, style: &TraceStyle) {
    let builder = tracing_subscriber::fmt()
        .with_max_level(filter)
        .with_ansi(true)
        .with_test_writer();

    let () = match style {
        TraceStyle::Full => builder.init(),
        TraceStyle::Pretty => builder.pretty().with_file(false).init(),
        TraceStyle::Compact => builder.compact().init(),
        TraceStyle::Json => builder.json().init(),
    };

    tracing::info!("Logging initialized");
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
            TraceStyle::Full => "Full Style",
            TraceStyle::Pretty => "Pretty Style",
            TraceStyle::Compact => "Compact Style",
            TraceStyle::Json => "Json Format",
        };

        f.write_str(style)
    }
}

#[cfg(test)]
mod tests {
    use tracing::level_filters::LevelFilter;

    use super::{Logging, Threshold, TraceStyle, map_to_tracing_level_filter};

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

    #[test]
    fn it_should_map_the_trace_filter_to_the_corresponding_tracing_level() {
        // Arrange
        let trace_filter = Threshold::Warn;

        // Act
        let tracing_level = map_to_tracing_level_filter(&trace_filter);

        // Assert
        assert_eq!(tracing_level, LevelFilter::WARN);
    }
}
