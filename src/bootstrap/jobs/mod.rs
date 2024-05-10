//! Application jobs launchers.
//!
//! The main application setup has only two main stages:
//!
//! 1. Setup the domain layer: the core tracker.
//! 2. Launch all the application services as concurrent jobs.
//!
//! This modules contains all the functions needed to start those jobs.

use std::panic::Location;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use camino::Utf8PathBuf;
use thiserror::Error;
use torrust_tracker_located_error::{DynError, LocatedError};
use tracing::{info, instrument};
pub mod health_check_api;
pub mod http_tracker;
pub mod torrent_cleanup;
pub mod tracker_apis;
pub mod udp_tracker;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Timeout elapsed for Task")]
    TimeoutError { err: Arc<tokio::time::error::Elapsed> },
    #[error("Error From Service: {err}")]
    ServiceError { err: crate::servers::service::Error },

    #[error("tls config missing")]
    MissingTlsConfig { location: &'static Location<'static> },

    #[error("bad tls config: {source}")]
    BadTlsConfig {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
}

impl From<tokio::time::error::Elapsed> for Error {
    fn from(e: tokio::time::error::Elapsed) -> Self {
        Error::TimeoutError { err: e.into() }
    }
}

impl From<crate::servers::service::Error> for Error {
    fn from(err: crate::servers::service::Error) -> Self {
        Error::ServiceError { err }
    }
}

#[instrument(ret)]
pub async fn make_rust_tls(enabled: bool, cert: &Option<String>, key: &Option<String>) -> Option<Result<RustlsConfig, Error>> {
    if !enabled {
        info!("TLS not enabled");
        return None;
    }

    if let (Some(cert), Some(key)) = (cert, key) {
        info!("Using https: cert path: {cert}.");
        info!("Using https: key path: {key}.");

        Some(
            RustlsConfig::from_pem_file(cert, key)
                .await
                .map_err(|err| Error::BadTlsConfig {
                    source: (Arc::new(err) as DynError).into(),
                }),
        )
    } else {
        Some(Err(Error::MissingTlsConfig {
            location: Location::caller(),
        }))
    }
}

#[instrument(ret)]
pub async fn make_rust_tls_from_path_buf(
    enabled: bool,
    cert: &Option<Utf8PathBuf>,
    key: &Option<Utf8PathBuf>,
) -> Option<Result<RustlsConfig, Error>> {
    if !enabled {
        info!("TLS not enabled");
        return None;
    }

    if let (Some(cert), Some(key)) = (cert, key) {
        info!("Using https: cert path: {cert}.");
        info!("Using https: key path: {key}.");

        Some(
            RustlsConfig::from_pem_file(cert, key)
                .await
                .map_err(|err| Error::BadTlsConfig {
                    source: (Arc::new(err) as DynError).into(),
                }),
        )
    } else {
        Some(Err(Error::MissingTlsConfig {
            location: Location::caller(),
        }))
    }
}

#[cfg(test)]
mod tests {

    use super::make_rust_tls;

    #[tokio::test]
    async fn it_should_error_on_bad_tls_config() {
        let (bad_cert_path, bad_key_path) = (Some("bad cert path".to_string()), Some("bad key path".to_string()));
        let err = make_rust_tls(true, &bad_cert_path, &bad_key_path)
            .await
            .expect("tls_was_enabled")
            .expect_err("bad_cert_and_key_files");

        assert!(err
            .to_string()
            .contains("bad tls config: No such file or directory (os error 2)"));
    }

    #[tokio::test]
    async fn it_should_error_on_missing_tls_config() {
        let err = make_rust_tls(true, &None, &None)
            .await
            .expect("tls_was_enabled")
            .expect_err("missing_config");

        assert_eq!(err.to_string(), "tls config missing");
    }
}
