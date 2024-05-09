use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};

/// Configuration for each HTTP tracker.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct HttpTracker {
    /// Weather the HTTP tracker is enabled or not.
    pub enabled: bool,
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    pub bind_address: String,
    /// Weather the HTTP tracker will use SSL or not.
    pub ssl_enabled: bool,
    /// Path to the SSL certificate file. Only used if `ssl_enabled` is `true`.
    #[serde_as(as = "NoneAsEmptyString")]
    pub ssl_cert_path: Option<String>,
    /// Path to the SSL key file. Only used if `ssl_enabled` is `true`.
    #[serde_as(as = "NoneAsEmptyString")]
    pub ssl_key_path: Option<String>,
}

impl Default for HttpTracker {
    fn default() -> Self {
        Self {
            enabled: false,
            bind_address: String::from("0.0.0.0:7070"),
            ssl_enabled: false,
            ssl_cert_path: None,
            ssl_key_path: None,
        }
    }
}
