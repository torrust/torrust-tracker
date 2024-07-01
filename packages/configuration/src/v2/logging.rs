use serde::{Deserialize, Serialize};

use crate::LogLevel;

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Logging {
    /// Logging level. Possible values are: `Off`, `Error`, `Warn`, `Info`,
    /// `Debug` and `Trace`. Default is `Info`.
    #[serde(default = "Logging::default_log_level")]
    pub log_level: LogLevel,
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            log_level: Self::default_log_level(),
        }
    }
}

impl Logging {
    fn default_log_level() -> LogLevel {
        LogLevel::Info
    }
}
