//! Logging configuration and setup for `v2_0_0`.
//!
//! Contains the `Logging` configuration struct, the `Threshold` level enum,
//! the `TraceStyle` enum, and the `setup()` / `tracing_init()` helpers.
use std::sync::Once;

use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

static INIT: Once = Once::new();

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Logging {
    /// Logging level. Possible values are: `Off`, `Error`, `Warn`, `Info`,
    /// `Debug` and `Trace`. Default is `Info`.
    #[serde(default = "Logging::default_threshold")]
    pub threshold: Threshold,
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            threshold: Self::default_threshold(),
        }
    }
}

impl Logging {
    const fn default_threshold() -> Threshold {
        Threshold::Info
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

/// Redirects log output to stdout at the threshold defined in the configuration.
pub fn setup(cfg: &Logging) {
    let tracing_level = map_to_tracing_level_filter(&cfg.threshold);

    if tracing_level == LevelFilter::OFF {
        return;
    }

    INIT.call_once(|| {
        tracing_init(tracing_level, &TraceStyle::Default);
    });
}

const fn map_to_tracing_level_filter(threshold: &Threshold) -> LevelFilter {
    match threshold {
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
        TraceStyle::Default => builder.init(),
        TraceStyle::Pretty(display_filename) => builder.pretty().with_file(*display_filename).init(),
        TraceStyle::Compact => builder.compact().init(),
        TraceStyle::Json => builder.json().init(),
    };

    tracing::info!("Logging initialized");
}

#[derive(Debug)]
pub enum TraceStyle {
    Default,
    Pretty(bool),
    Compact,
    Json,
}

impl std::fmt::Display for TraceStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style = match self {
            Self::Default => "Default Style",
            Self::Pretty(path) => match path {
                true => "Pretty Style with File Paths",
                false => "Pretty Style without File Paths",
            },
            Self::Compact => "Compact Style",
            Self::Json => "Json Format",
        };

        f.write_str(style)
    }
}
