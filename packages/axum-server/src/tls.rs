use std::panic::Location;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use thiserror::Error;
use torrust_located_error::{DynError, LocatedError};
use torrust_tracker_configuration::TslConfig;
use tracing::instrument;

/// Error returned by the Bootstrap Process.
#[derive(Error, Debug)]
pub enum Error {
    /// Enabled tls but missing config.
    #[error("tls config missing")]
    MissingTlsConfig { location: &'static Location<'static> },

    /// Unable to parse tls Config.
    #[error("bad tls config: {source}")]
    BadTlsConfig {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
}

#[instrument(skip(tls_config))]
/// # Errors
///
/// Returns [`Error::MissingTlsConfig`] when the certificate or key path does
/// not exist, and [`Error::BadTlsConfig`] when loading invalid PEM files
/// fails.
pub async fn make_rust_tls(tls_config: &TslConfig) -> Result<RustlsConfig, Error> {
    let cert = tls_config.ssl_cert_path.clone();
    let key = tls_config.ssl_key_path.clone();

    if !cert.exists() || !key.exists() {
        return Err(Error::MissingTlsConfig {
            location: Location::caller(),
        });
    }

    tracing::info!("Using https: cert path: {cert}.");
    tracing::info!("Using https: key path: {key}.");

    RustlsConfig::from_pem_file(cert, key)
        .await
        .map_err(|err| Error::BadTlsConfig {
            source: (Arc::new(err) as DynError).into(),
        })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use camino::Utf8PathBuf;
    use torrust_tracker_configuration::TslConfig;

    use super::{Error, make_rust_tls};

    fn make_temp_file(prefix: &str, content: &str) -> Utf8PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be later than epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{nanos}.pem"));
        fs::write(&path, content).expect("it should write temporary test file");

        Utf8PathBuf::from_path_buf(path).expect("temporary test file path should be UTF-8")
    }

    #[tokio::test]
    async fn it_should_error_on_bad_tls_config() {
        let cert_path = make_temp_file("bad-cert", "not a valid certificate");
        let key_path = make_temp_file("bad-key", "not a valid private key");

        let err = make_rust_tls(&TslConfig {
            ssl_cert_path: cert_path.clone(),
            ssl_key_path: key_path.clone(),
        })
        .await
        .expect_err("bad_cert_and_key_files");

        fs::remove_file(cert_path).expect("it should remove temporary cert file");
        fs::remove_file(key_path).expect("it should remove temporary key file");

        assert!(matches!(err, Error::BadTlsConfig { source: _ }));
    }

    #[tokio::test]
    async fn it_should_error_on_missing_cert_or_key_paths() {
        let err = make_rust_tls(&TslConfig {
            ssl_cert_path: Utf8PathBuf::from(""),
            ssl_key_path: Utf8PathBuf::from(""),
        })
        .await
        .expect_err("missing_config");

        assert!(matches!(err, Error::MissingTlsConfig { location: _ }));
    }
}
