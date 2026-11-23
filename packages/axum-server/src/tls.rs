use std::panic::Location;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use thiserror::Error;
use torrust_located_error::{DynError, LocatedError};
use torrust_tracker_configuration::v3_0_0::tls::TlsConfig;
use tracing::instrument;

/// Error returned by the Bootstrap Process.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Enabled tls but missing config.
    #[error("TLS certificate or key file does not exist: certificate={cert}, key={key}")]
    MissingTlsConfig {
        cert: camino::Utf8PathBuf,
        key: camino::Utf8PathBuf,
        location: &'static Location<'static>,
    },

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
pub async fn make_rust_tls(tls_config: &TlsConfig) -> Result<RustlsConfig, Error> {
    let cert = tls_config.ssl_cert_path.clone();
    let key = tls_config.ssl_key_path.clone();

    if !cert.exists() || !key.exists() {
        return Err(Error::MissingTlsConfig {
            cert,
            key,
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
    use std::io::Write;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::path::PathBuf;

    use camino::Utf8PathBuf;
    use tempfile::{Builder, NamedTempFile};
    use torrust_tracker_configuration::v3_0_0::tls::TlsConfig;

    use super::{Error, make_rust_tls};

    /// Fails when `message` holds a source path: a workspace or registry path, or a `file.rs:line` suffix.
    #[track_caller]
    fn assert_names_no_source_path(message: &str) {
        for fragment in [
            ".rs:",
            "packages/",
            "packages\\",
            "src/",
            "src\\",
            ".cargo/registry",
            "rustlib/src",
        ] {
            assert!(
                !message.contains(fragment),
                "the error message must not hold a source path, but it contains {fragment:?}: {message:?}"
            );
        }
    }

    /// Writes `content` to a new `<prefix><random>.pem` file in the system temporary directory.
    ///
    /// The returned guard removes the file when it is dropped, also when the test holding it fails.
    fn make_temp_file(prefix: &str, content: &str) -> NamedTempFile {
        let mut file = Builder::new()
            .prefix(prefix)
            .suffix(".pem")
            .tempfile()
            .expect("it should create a temporary test file");
        file.write_all(content.as_bytes())
            .expect("it should write the temporary test file");
        file
    }

    fn utf8_path(file: &NamedTempFile) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(file.path().to_path_buf()).expect("temporary test file path should be UTF-8")
    }

    #[tokio::test]
    async fn it_should_error_on_bad_tls_config() {
        let cert_file = make_temp_file("bad-cert", "not a valid certificate");
        let key_file = make_temp_file("bad-key", "not a valid private key");

        let err = make_rust_tls(&TlsConfig {
            ssl_cert_path: utf8_path(&cert_file),
            ssl_key_path: utf8_path(&key_file),
        })
        .await
        .expect_err("bad_cert_and_key_files");

        assert!(matches!(err, Error::BadTlsConfig { source: _ }));
        assert_names_no_source_path(&err.to_string());
    }

    #[test]
    fn it_should_remove_the_temporary_file_when_its_guard_is_dropped() {
        // Arrange
        let file = make_temp_file("dropped", "content");
        let path = file.path().to_path_buf();

        // Act
        drop(file);

        // Assert
        assert!(!path.exists(), "{} should have been removed", path.display());
    }

    #[test]
    fn it_should_remove_the_temporary_file_when_the_test_using_it_fails() {
        // Arrange
        let mut path: Option<PathBuf> = None;

        // Act
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            let file = make_temp_file("failing-test", "content");
            path = Some(file.path().to_path_buf());
            panic!("a test that fails while it holds the file");
        }));

        // Assert
        assert!(outcome.is_err(), "the closure stands for a failing test, so it should panic");
        let path = path.expect("the file was created before the panic");
        assert!(!path.exists(), "{} should have been removed", path.display());
    }

    #[tokio::test]
    async fn it_should_error_on_missing_cert_or_key_paths() {
        let err = make_rust_tls(&TlsConfig {
            ssl_cert_path: Utf8PathBuf::from(""),
            ssl_key_path: Utf8PathBuf::from(""),
        })
        .await
        .expect_err("missing_config");

        assert!(matches!(
            err,
            Error::MissingTlsConfig {
                cert: _,
                key: _,
                location: _
            }
        ));
    }
}
