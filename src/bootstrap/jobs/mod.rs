//! Application jobs launchers.
//!
//! The main application setup has only two main stages:
//!
//! 1. Setup the domain layer: the core tracker.
//! 2. Launch all the application services as concurrent jobs.
//!
//! This modules contains all the functions needed to start those jobs.
pub mod health_check_api;
pub mod http_tracker;
pub mod torrent_cleanup;
pub mod tracker_apis;
pub mod udp_tracker;

/// This is the message that the "launcher" spawned task sends to the main
/// application process to notify the service was successfully started.
///
#[derive(Debug)]
pub struct Started {
    pub address: std::net::SocketAddr,
}

pub async fn make_rust_tls(enabled: bool, cert: &Option<String>, key: &Option<String>) -> Option<Result<RustlsConfig, Error>> {
    if !enabled {
        info!("TLS not enabled");
        return None;
    }

    if let (Some(cert), Some(key)) = (cert, key) {
        info!("Using https: cert path: {cert}.");
        info!("Using https: key path: {cert}.");

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

use std::panic::Location;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use thiserror::Error;
use torrust_tracker_located_error::{DynError, LocatedError};
use tracing::info;

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
