//! TLS certificate configuration for schema v3.
//!
//! **Field type convention**: use typed newtypes for fields with domain constraints —
//! not `String` or other unvalidated primitives. See [`crate::v3_0_0::public_url`].
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// TLS certificate and private key paths.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct TlsConfig {
    /// Path to the TLS certificate file.
    #[serde(default = "TlsConfig::default_ssl_cert_path")]
    pub ssl_cert_path: Utf8PathBuf,

    /// Path to the TLS private key file.
    #[serde(default = "TlsConfig::default_ssl_key_path")]
    pub ssl_key_path: Utf8PathBuf,
}

impl TlsConfig {
    fn default_ssl_cert_path() -> Utf8PathBuf {
        Utf8PathBuf::new()
    }

    fn default_ssl_key_path() -> Utf8PathBuf {
        Utf8PathBuf::new()
    }
}
