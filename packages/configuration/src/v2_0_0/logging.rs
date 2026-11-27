//! Logging configuration for `v2_0_0`.
//!
//! Contains the `Logging` configuration struct, the `Threshold` level enum,
//! and the `TraceStyle` enum. The schema `v2_0_0` only describes the logging configuration.
use serde::{Deserialize, Serialize};

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
