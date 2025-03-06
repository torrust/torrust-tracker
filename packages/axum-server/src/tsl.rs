use std::panic::Location;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use thiserror::Error;
use torrust_tracker_configuration::TslConfig;
use torrust_tracker_located_error::{DynError, LocatedError};
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

#[instrument(skip(opt_tsl_config))]
pub async fn make_rust_tls(opt_tsl_config: &Option<TslConfig>) -> Option<Result<RustlsConfig, Error>> {
    match opt_tsl_config {
        Some(tsl_config) => {
            let cert = tsl_config.ssl_cert_path.clone();
            let key = tsl_config.ssl_key_path.clone();

            if !cert.exists() || !key.exists() {
                return Some(Err(Error::MissingTlsConfig {
                    location: Location::caller(),
                }));
            }

            tracing::info!("Using https: cert path: {cert}.");
            tracing::info!("Using https: key path: {key}.");

            Some(
                RustlsConfig::from_pem_file(cert, key)
                    .await
                    .map_err(|err| Error::BadTlsConfig {
                        source: (Arc::new(err) as DynError).into(),
                    }),
            )
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {

    use camino::Utf8PathBuf;
    use torrust_tracker_configuration::TslConfig;

    use super::{make_rust_tls, Error};

    #[tokio::test]
    async fn it_should_error_on_bad_tls_config() {
        let err = make_rust_tls(&Some(TslConfig {
            ssl_cert_path: Utf8PathBuf::from("bad cert path"),
            ssl_key_path: Utf8PathBuf::from("bad key path"),
        }))
        .await
        .expect("tls_was_enabled")
        .expect_err("bad_cert_and_key_files");

        assert!(matches!(err, Error::MissingTlsConfig { location: _ }));
    }

    #[tokio::test]
    async fn it_should_error_on_missing_cert_or_key_paths() {
        let err = make_rust_tls(&Some(TslConfig {
            ssl_cert_path: Utf8PathBuf::from(""),
            ssl_key_path: Utf8PathBuf::from(""),
        }))
        .await
        .expect("tls_was_enabled")
        .expect_err("missing_config");

        assert!(matches!(err, Error::MissingTlsConfig { location: _ }));
    }
}
