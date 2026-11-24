//! Tracker configuration factories for testing.
use std::env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::time::Duration;

use tempfile::TempDir;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_configuration::v3_0_0::database::Database;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_configuration::v3_0_0::logging::Threshold;
use torrust_tracker_configuration::v3_0_0::network::Network;
use torrust_tracker_configuration::v3_0_0::tracker_api::HttpApi;
use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;

use crate::random;

/// This configuration is used for testing. It generates random config values
/// so they do not collide if you run more than one tracker at the same time.
///
/// > **NOTICE**: This configuration is not meant to be used in production.
///
/// > **NOTICE**: Port 0 is used for ephemeral ports, which means that the OS
/// > will assign a random free port for the tracker to use.
///
/// > **NOTICE**: You can change the log threshold to `debug` to see the logs of the
/// > tracker while running the tests. That can be particularly useful when
/// > debugging tests.
///
/// # Panics
///
/// Will panic if it can't convert the temp file path to string
#[must_use]
pub fn ephemeral() -> Configuration {
    // todo: disable services that are not needed.
    // For example: a test for the UDP tracker should disable the API and HTTP tracker.

    let mut config = Configuration::default();
    config.core.database = Some(Database::Sqlite3 {
        path: ephemeral_sqlite_database().to_string_lossy().into_owned(),
    });

    // This have to be Off otherwise the tracing global subscriber
    // initialization will panic because you can't set a global subscriber more
    // than once. You can use enable logging in tests with:
    // `crate::common::logging::setup(LevelFilter::ERROR);`
    // That will also allow you to capture logs and write assertions on them.
    config.logging.trace_filter = Threshold::Off;

    // Ephemeral socket address for API
    let api_port = 0u16;
    let mut http_api = HttpApi {
        bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), api_port),
        ..Default::default()
    };
    http_api.add_token("admin", "MyAccessToken");
    config.http_api = Some(http_api);

    // Ephemeral socket address for Health Check API
    let health_check_api_port = 0u16;
    config.health_check_api.bind_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), health_check_api_port);

    // Ephemeral socket address for UDP tracker
    let udp_port = 0u16;
    config.udp_trackers = Some(vec![UdpTracker {
        bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), udp_port),
        cookie_lifetime: Duration::from_secs(120),
        tracker_usage_statistics: true,
        network: Network::default(),
        ..UdpTracker::default()
    }]);

    // Ephemeral socket address for HTTP tracker
    let http_port = 0u16;
    config.http_trackers = Some(vec![HttpTracker {
        bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), http_port),
        tracker_usage_statistics: true,
        network: Network::default(),
        ..HttpTracker::default()
    }]);

    config
}

#[must_use]
pub fn ephemeral_sqlite_database() -> PathBuf {
    let temp_directory = env::temp_dir();
    let random_db_id = random::string(16);
    temp_directory.join(format!("data_{random_db_id}.db"))
}

/// Owns the temporary directory that holds a test's `SQLite` database file.
///
/// It is called a "guard" after the RAII guard idiom (like `MutexGuard`): the
/// value that creates a resource also releases it when it goes out of scope.
/// Dropping the guard deletes the directory and the database in it, even when
/// the test panics.
///
/// Keep the guard alive for as long as the database is used and bind it to a
/// named variable such as `_guard`: a bare `_` drops it immediately. The guard
/// does not depend on a test environment, so a restart test can share one guard
/// between two environments.
#[derive(Debug)]
pub struct SqliteDatabaseGuard {
    directory: TempDir,
}

impl SqliteDatabaseGuard {
    /// Creates an empty temporary directory for a `SQLite` database.
    ///
    /// # Panics
    ///
    /// Panics if the temporary directory cannot be created.
    #[must_use]
    pub fn new() -> Self {
        Self {
            directory: TempDir::new().expect("it should create a temporary directory for the SQLite database"),
        }
    }

    /// The directory owned by the guard.
    #[must_use]
    pub fn directory(&self) -> &Path {
        self.directory.path()
    }

    /// The path of the database file inside the directory. The file is created
    /// by `SQLite` when the tracker opens it.
    #[must_use]
    pub fn path(&self) -> PathBuf {
        self.directory.path().join("data.db")
    }
}

impl Default for SqliteDatabaseGuard {
    fn default() -> Self {
        Self::new()
    }
}

/// Like [`ephemeral`], but the `SQLite` database lives in a directory owned by
/// the returned guard, so it is removed when the guard is dropped.
///
/// [`ephemeral`] is unchanged: it still points at a file in the system temp
/// directory that nobody deletes.
#[must_use]
pub fn ephemeral_with_sqlite_database() -> (Configuration, SqliteDatabaseGuard) {
    let guard = SqliteDatabaseGuard::new();
    let mut config = ephemeral();
    config.core.database = Some(Database::Sqlite3 {
        path: guard.path().to_string_lossy().into_owned(),
    });
    (config, guard)
}

/// Ephemeral configuration with reverse proxy enabled.
///
/// # Panics
///
/// Panics if the ephemeral configuration does not enable an HTTP tracker.
#[must_use]
pub fn ephemeral_with_reverse_proxy() -> Configuration {
    let mut cfg = ephemeral();

    cfg.http_trackers
        .as_mut()
        .expect("ephemeral configuration enables an HTTP tracker")[0]
        .network
        .on_reverse_proxy = true;

    cfg
}

/// Ephemeral configuration with reverse proxy disabled.
///
/// # Panics
///
/// Panics if the ephemeral configuration does not enable an HTTP tracker.
#[must_use]
pub fn ephemeral_without_reverse_proxy() -> Configuration {
    let mut cfg = ephemeral();

    cfg.http_trackers
        .as_mut()
        .expect("ephemeral configuration enables an HTTP tracker")[0]
        .network
        .on_reverse_proxy = false;

    cfg
}

/// Ephemeral configuration with `public` mode.
#[must_use]
pub fn ephemeral_public() -> Configuration {
    let mut cfg = ephemeral();

    cfg.core.private = false;

    cfg
}

/// Ephemeral configuration with `private` mode.
#[must_use]
pub fn ephemeral_private() -> Configuration {
    let mut cfg = ephemeral();

    cfg.core.private = true;

    cfg
}

/// Ephemeral configuration with `listed` mode.
#[must_use]
pub fn ephemeral_listed() -> Configuration {
    let mut cfg = ephemeral();

    cfg.core.listed = true;

    cfg
}

/// Ephemeral configuration with `private_listed` mode.
#[must_use]
pub fn ephemeral_private_and_listed() -> Configuration {
    let mut cfg = ephemeral();

    cfg.core.private = true;
    cfg.core.listed = true;

    cfg
}

/// Ephemeral configuration with a custom external (public) IP for the tracker.
///
/// # Panics
///
/// Panics if the provided IP is a wildcard/unspecified address (`0.0.0.0` or `::`).
#[must_use]
pub fn ephemeral_with_external_ip(ip: IpAddr) -> Configuration {
    let mut cfg = ephemeral();

    let external_ip = Some(ip.try_into().expect("wildcard IP is not a valid external IP"));
    cfg.http_trackers
        .as_mut()
        .expect("ephemeral configuration enables an HTTP tracker")[0]
        .network
        .external_ip = external_ip;

    cfg
}

/// Ephemeral configuration using a wildcard IPv6 for the UDP, HTTP and API
/// services.
#[must_use]
pub fn ephemeral_ipv6() -> Configuration {
    let mut cfg = ephemeral();

    let ipv6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);

    if let Some(ref mut http_api) = cfg.http_api {
        http_api.bind_address.clone_from(&ipv6);
    }

    if let Some(ref mut http_trackers) = cfg.http_trackers {
        http_trackers[0].bind_address.clone_from(&ipv6);
    }

    if let Some(ref mut udp_trackers) = cfg.udp_trackers {
        udp_trackers[0].bind_address.clone_from(&ipv6);
    }

    cfg
}

/// Ephemeral without running any services.
#[must_use]
pub fn ephemeral_with_no_services() -> Configuration {
    let mut cfg = ephemeral();

    cfg.http_api = None;
    cfg.http_trackers = None;
    cfg.udp_trackers = None;

    cfg
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn the_guard_should_create_an_empty_directory_without_the_database_file() {
        let guard = SqliteDatabaseGuard::new();

        assert!(guard.directory().is_dir());
        assert!(!guard.path().exists());
    }

    #[test]
    fn dropping_the_guard_should_remove_the_directory_and_the_files_in_it() {
        let guard = SqliteDatabaseGuard::new();
        let directory = guard.directory().to_path_buf();
        fs::write(guard.path(), b"data").unwrap();

        drop(guard);

        assert!(!directory.exists());
    }

    #[test]
    fn dropping_the_guard_while_a_test_panics_should_remove_the_directory() {
        let guard = SqliteDatabaseGuard::new();
        let directory = guard.directory().to_path_buf();

        let result = std::panic::catch_unwind(move || {
            let _guard = guard;
            panic!("the test fails");
        });

        assert!(result.is_err());
        assert!(!directory.exists());
    }

    #[test]
    fn two_guards_should_not_share_a_directory() {
        let first = SqliteDatabaseGuard::new();
        let second = SqliteDatabaseGuard::new();

        assert_ne!(first.directory(), second.directory());
    }

    #[test]
    fn the_factory_should_point_the_configuration_at_the_database_file_of_the_guard() {
        let (config, guard) = ephemeral_with_sqlite_database();

        let Some(Database::Sqlite3 { path }) = config.core.database else {
            panic!("it should configure a SQLite database");
        };

        assert_eq!(PathBuf::from(path), guard.path());
    }

    #[test]
    fn the_factory_should_not_create_a_file_in_the_system_temp_directory() {
        let (config, _guard) = ephemeral_with_sqlite_database();

        let Some(Database::Sqlite3 { path }) = config.core.database else {
            panic!("it should configure a SQLite database");
        };

        assert_ne!(PathBuf::from(path).parent(), Some(env::temp_dir().as_path()));
    }
}
