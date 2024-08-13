use serde::{Deserialize, Serialize};

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
    fn default_threshold() -> Threshold {
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
