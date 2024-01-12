//! Configuration data structures for [Torrust Tracker](https://docs.rs/torrust-tracker).
//!
//! This module contains the configuration data structures for the
//! Torrust Tracker, which is a `BitTorrent` tracker server.
//!
//! The configuration is loaded from a [TOML](https://toml.io/en/) file
//! `tracker.toml` in the project root folder or from an environment variable
//! with the same content as the file.
//!
//! When you run the tracker without a configuration file, a new one will be
//! created with the default values, but the tracker immediately exits. You can
//! then edit the configuration file and run the tracker again.
//!
//! Configuration can not only be loaded from a file, but also from environment
//! variable `TORRUST_TRACKER_CONFIG`. This is useful when running the tracker
//! in a Docker container or environments where you do not have a persistent
//! storage or you cannot inject a configuration file. Refer to
//! [`Torrust Tracker documentation`](https://docs.rs/torrust-tracker) for more
//! information about how to pass configuration to the tracker.
//!
//! # Table of contents
//!
//! - [Sections](#sections)
//! - [Port binding](#port-binding)
//! - [TSL support](#tsl-support)
//!     - [Generating self-signed certificates](#generating-self-signed-certificates)
//! - [Default configuration](#default-configuration)
//!
//! ## Sections
//!
//! Each section in the toml structure is mapped to a data structure. For
//! example, the `[http_api]` section (configuration for the tracker HTTP API)
//! is mapped to the [`HttpApi`] structure.
//!
//! > **NOTICE**: some sections are arrays of structures. For example, the
//! > `[[udp_trackers]]` section is an array of [`UdpTracker`] since
//! > you can have multiple running UDP trackers bound to different ports.
//!
//! Please refer to the documentation of each structure for more information
//! about each section.
//!
//! - [`Core configuration`](crate::Configuration)
//! - [`HTTP API configuration`](crate::HttpApi)
//! - [`HTTP Tracker configuration`](crate::HttpTracker)
//! - [`UDP Tracker configuration`](crate::UdpTracker)
//!
//! ## Port binding
//!
//! For the API, HTTP and UDP trackers you can bind to a random port by using
//! port `0`. For example, if you want to bind to a random port on all
//! interfaces, use `0.0.0.0:0`. The OS will choose a random port but the
//! tracker will not print the port it is listening to when it starts. It just
//! says `Starting Torrust HTTP tracker server on: http://0.0.0.0:0`. It shows
//! the port used in the configuration file, and not the port the
//! tracker is actually listening to. This is a planned feature, see issue
//! [186](https://github.com/torrust/torrust-tracker/issues/186) for more
//! information.
//!
//! ## TSL support
//!
//! For the API and HTTP tracker you can enable TSL by setting `ssl_enabled` to
//! `true` and setting the paths to the certificate and key files.
//!
//! Typically, you will have a directory structure like this:
//!
//! ```text
//! storage/
//! ├── database
//! │   └── data.db
//! └── tls
//!     ├── localhost.crt
//!     └── localhost.key
//! ```
//!
//! where you can store all the persistent data.
//!
//! Alternatively, you could setup a reverse proxy like Nginx or Apache to
//! handle the SSL/TLS part and forward the requests to the tracker. If you do
//! that, you should set [`on_reverse_proxy`](crate::Configuration::on_reverse_proxy)
//! to `true` in the configuration file. It's out of scope for this
//! documentation to explain in detail how to setup a reverse proxy, but the
//! configuration file should be something like this:
//!
//! For [NGINX](https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/):
//!
//! ```text
//! # HTTPS only (with SSL - force redirect to HTTPS)
//!
//! server {
//!     listen 80;
//!     server_name tracker.torrust.com;
//!
//!     return 301 https://$host$request_uri;
//! }
//!
//! server {
//!     listen 443;
//!     server_name tracker.torrust.com;
//!
//!     ssl_certificate CERT_PATH
//!     ssl_certificate_key CERT_KEY_PATH;
//!
//!     location / {
//!         proxy_set_header X-Forwarded-For $remote_addr;
//!         proxy_pass http://127.0.0.1:6969;
//!     }
//! }
//! ```
//!
//! For [Apache](https://httpd.apache.org/docs/2.4/howto/reverse_proxy.html):
//!
//! ```text
//! # HTTPS only (with SSL - force redirect to HTTPS)
//!
//! <VirtualHost *:80>
//!     ServerAdmin webmaster@tracker.torrust.com
//!     ServerName tracker.torrust.com
//!
//!     <IfModule mod_rewrite.c>
//!         RewriteEngine on
//!         RewriteCond %{HTTPS} off
//!         RewriteRule ^ https://%{SERVER_NAME}%{REQUEST_URI} [END,NE,R=permanent]
//!     </IfModule>
//! </VirtualHost>
//!
//! <IfModule mod_ssl.c>
//!     <VirtualHost *:443>
//!         ServerAdmin webmaster@tracker.torrust.com
//!         ServerName tracker.torrust.com
//!
//!         <Proxy *>
//!             Order allow,deny
//!             Allow from all
//!         </Proxy>
//!
//!         ProxyPreserveHost On
//!         ProxyRequests Off
//!         AllowEncodedSlashes NoDecode
//!
//!         ProxyPass / http://localhost:3000/
//!         ProxyPassReverse / http://localhost:3000/
//!         ProxyPassReverse / http://tracker.torrust.com/
//!
//!         RequestHeader set X-Forwarded-Proto "https"
//!         RequestHeader set X-Forwarded-Port "443"
//!
//!         ErrorLog ${APACHE_LOG_DIR}/tracker.torrust.com-error.log
//!         CustomLog ${APACHE_LOG_DIR}/tracker.torrust.com-access.log combined
//!
//!         SSLCertificateFile CERT_PATH
//!         SSLCertificateKeyFile CERT_KEY_PATH
//!     </VirtualHost>
//! </IfModule>
//! ```
//!
//! ## Generating self-signed certificates
//!
//! For testing purposes, you can use self-signed certificates.
//!
//! Refer to [Let's Encrypt - Certificates for localhost](https://letsencrypt.org/docs/certificates-for-localhost/)
//! for more information.
//!
//! Running the following command will generate a certificate (`localhost.crt`)
//! and key (`localhost.key`) file in your current directory:
//!
//! ```s
//! openssl req -x509 -out localhost.crt -keyout localhost.key \
//!   -newkey rsa:2048 -nodes -sha256 \
//!   -subj '/CN=localhost' -extensions EXT -config <( \
//!    printf "[dn]\nCN=localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")
//! ```
//!
//! You can then use the generated files in the configuration file:
//!
//! ```s
//! [[http_trackers]]
//! enabled = true
//! ...
//! ssl_cert_path = "./storage/tracker/lib/tls/localhost.crt"
//! ssl_key_path = "./storage/tracker/lib/tls/localhost.key"
//!
//! [http_api]
//! enabled = true
//! ...
//! ssl_cert_path = "./storage/tracker/lib/tls/localhost.crt"
//! ssl_key_path = "./storage/tracker/lib/tls/localhost.key"
//! ```
//!
//! ## Default configuration
//!
//! The default configuration is:
//!
//! ```toml
//! announce_interval = 120
//! db_driver = "Sqlite3"
//! db_path = "./storage/tracker/lib/database/sqlite3.db"
//! external_ip = "0.0.0.0"
//! inactive_peer_cleanup_interval = 600
//! log_level = "info"
//! max_peer_timeout = 900
//! min_announce_interval = 120
//! mode = "public"
//! on_reverse_proxy = false
//! persistent_torrent_completed_stat = false
//! remove_peerless_torrents = true
//! tracker_usage_statistics = true
//!
//! [[udp_trackers]]
//! bind_address = "0.0.0.0:6969"
//! enabled = false
//!
//! [[http_trackers]]
//! bind_address = "0.0.0.0:7070"
//! enabled = false
//! ssl_cert_path = ""
//! ssl_enabled = false
//! ssl_key_path = ""
//!
//! [http_api]
//! bind_address = "127.0.0.1:1212"
//! enabled = true
//! ssl_cert_path = ""
//! ssl_enabled = false
//! ssl_key_path = ""
//!
//! [http_api.access_tokens]
//! admin = "MyAccessToken"
//!
//! [health_check_api]
//! bind_address = "127.0.0.1:1313"
//!```
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::{env, fs};

use config::{Config, ConfigError, File, FileFormat};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use thiserror::Error;
use torrust_tracker_located_error::{DynError, Located, LocatedError};
use torrust_tracker_primitives::{DatabaseDriver, TrackerMode};

/// Information required for loading config
#[derive(Debug, Default, Clone)]
pub struct Info {
    tracker_toml: String,
    api_admin_token: Option<String>,
}

impl Info {
    /// Build Configuration Info
    ///
    /// # Examples
    ///
    /// ```
    /// use torrust_tracker_configuration::Info;
    ///
    /// let result = Info::new(env_var_config, env_var_path_config, default_path_config, env_var_api_admin_token);
    /// assert_eq!(result, );
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to obtain a configuration.
    ///
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        env_var_config: String,
        env_var_path_config: String,
        default_path_config: String,
        env_var_api_admin_token: String,
    ) -> Result<Self, Error> {
        let tracker_toml = if let Ok(tracker_toml) = env::var(&env_var_config) {
            println!("Loading configuration from env var {env_var_config} ...");

            tracker_toml
        } else {
            let config_path = if let Ok(config_path) = env::var(env_var_path_config) {
                println!("Loading configuration file: `{config_path}` ...");

                config_path
            } else {
                println!("Loading default configuration file: `{default_path_config}` ...");

                default_path_config
            };

            fs::read_to_string(config_path)
                .map_err(|e| Error::UnableToLoadFromConfigFile {
                    source: (Arc::new(e) as DynError).into(),
                })?
                .parse()
                .map_err(|_e: std::convert::Infallible| Error::Infallible)?
        };
        let api_admin_token = env::var(env_var_api_admin_token).ok();

        Ok(Self {
            tracker_toml,
            api_admin_token,
        })
    }
}

/// Configuration for each UDP tracker.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct UdpTracker {
    /// Weather the UDP tracker is enabled or not.
    pub enabled: bool,
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    pub bind_address: String,
}

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

/// Configuration for the HTTP API.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct HttpApi {
    /// Weather the HTTP API is enabled or not.
    pub enabled: bool,
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    pub bind_address: String,
    /// Weather the HTTP API will use SSL or not.
    pub ssl_enabled: bool,
    /// Path to the SSL certificate file. Only used if `ssl_enabled` is `true`.
    #[serde_as(as = "NoneAsEmptyString")]
    pub ssl_cert_path: Option<String>,
    /// Path to the SSL key file. Only used if `ssl_enabled` is `true`.
    #[serde_as(as = "NoneAsEmptyString")]
    pub ssl_key_path: Option<String>,
    /// Access tokens for the HTTP API. The key is a label identifying the
    /// token and the value is the token itself. The token is used to
    /// authenticate the user. All tokens are valid for all endpoints and have
    /// the all permissions.
    pub access_tokens: HashMap<String, String>,
}

impl HttpApi {
    fn override_admin_token(&mut self, api_admin_token: &str) {
        self.access_tokens.insert("admin".to_string(), api_admin_token.to_string());
    }

    /// Checks if the given token is one of the token in the configuration.
    #[must_use]
    pub fn contains_token(&self, token: &str) -> bool {
        let tokens: HashMap<String, String> = self.access_tokens.clone();
        let tokens: HashSet<String> = tokens.into_values().collect();
        tokens.contains(token)
    }
}

/// Configuration for the Health Check API.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct HealthCheckApi {
    /// The address the API will bind to.
    /// The format is `ip:port`, for example `127.0.0.1:1313`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    pub bind_address: String,
}

/// Announce policy
#[derive(PartialEq, Eq, Debug, Clone, Copy, Constructor)]
pub struct AnnouncePolicy {
    /// Interval in seconds that the client should wait between sending regular
    /// announce requests to the tracker.
    ///
    /// It's a **recommended** wait time between announcements.
    ///
    /// This is the standard amount of time that clients should wait between
    /// sending consecutive announcements to the tracker. This value is set by
    /// the tracker and is typically provided in the tracker's response to a
    /// client's initial request. It serves as a guideline for clients to know
    /// how often they should contact the tracker for updates on the peer list,
    /// while ensuring that the tracker is not overwhelmed with requests.
    pub interval: u32,

    /// Minimum announce interval. Clients must not reannounce more frequently
    /// than this.
    ///
    /// It establishes the shortest allowed wait time.
    ///
    /// This is an optional parameter in the protocol that the tracker may
    /// provide in its response. It sets a lower limit on the frequency at which
    /// clients are allowed to send announcements. Clients should respect this
    /// value to prevent sending too many requests in a short period, which
    /// could lead to excessive load on the tracker or even getting banned by
    /// the tracker for not adhering to the rules.
    pub interval_min: u32,
}

impl Default for AnnouncePolicy {
    fn default() -> Self {
        Self {
            interval: 120,
            interval_min: 120,
        }
    }
}

/// Core configuration for the tracker.
#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Configuration {
    /// Logging level. Possible values are: `Off`, `Error`, `Warn`, `Info`,
    /// `Debug` and `Trace`. Default is `Info`.
    pub log_level: Option<String>,
    /// Tracker mode. See [`TrackerMode`] for more information.
    pub mode: TrackerMode,

    // Database configuration
    /// Database driver. Possible values are: `Sqlite3`, and `MySQL`.
    pub db_driver: DatabaseDriver,
    /// Database connection string. The format depends on the database driver.
    /// For `Sqlite3`, the format is `path/to/database.db`, for example:
    /// `./storage/tracker/lib/database/sqlite3.db`.
    /// For `Mysql`, the format is `mysql://db_user:db_user_password:port/db_name`, for
    /// example: `root:password@localhost:3306/torrust`.
    pub db_path: String,

    /// See [`AnnouncePolicy::interval`]
    pub announce_interval: u32,

    /// See [`AnnouncePolicy::interval_min`]
    pub min_announce_interval: u32,
    /// Weather the tracker is behind a reverse proxy or not.
    /// If the tracker is behind a reverse proxy, the `X-Forwarded-For` header
    /// sent from the proxy will be used to get the client's IP address.
    pub on_reverse_proxy: bool,
    /// The external IP address of the tracker. If the client is using a
    /// loopback IP address, this IP address will be used instead. If the peer
    /// is using a loopback IP address, the tracker assumes that the peer is
    /// in the same network as the tracker and will use the tracker's IP
    /// address instead.
    pub external_ip: Option<String>,
    /// Weather the tracker should collect statistics about tracker usage.
    /// If enabled, the tracker will collect statistics like the number of
    /// connections handled, the number of announce requests handled, etc.
    /// Refer to the [`Tracker`](https://docs.rs/torrust-tracker) for more
    /// information about the collected metrics.
    pub tracker_usage_statistics: bool,
    /// If enabled the tracker will persist the number of completed downloads.
    /// That's how many times a torrent has been downloaded completely.
    pub persistent_torrent_completed_stat: bool,

    // Cleanup job configuration
    /// Maximum time in seconds that a peer can be inactive before being
    /// considered an inactive peer. If a peer is inactive for more than this
    /// time, it will be removed from the torrent peer list.
    pub max_peer_timeout: u32,
    /// Interval in seconds that the cleanup job will run to remove inactive
    /// peers from the torrent peer list.
    pub inactive_peer_cleanup_interval: u64,
    /// If enabled, the tracker will remove torrents that have no peers.
    /// THe clean up torrent job runs every `inactive_peer_cleanup_interval`
    /// seconds and it removes inactive peers. Eventually, the peer list of a
    /// torrent could be empty and the torrent will be removed if this option is
    /// enabled.
    pub remove_peerless_torrents: bool,

    // Server jobs configuration
    /// The list of UDP trackers the tracker is running. Each UDP tracker
    /// represents a UDP server that the tracker is running and it has its own
    /// configuration.
    pub udp_trackers: Vec<UdpTracker>,
    /// The list of HTTP trackers the tracker is running. Each HTTP tracker
    /// represents a HTTP server that the tracker is running and it has its own
    /// configuration.
    pub http_trackers: Vec<HttpTracker>,
    /// The HTTP API configuration.
    pub http_api: HttpApi,
    /// The Health Check API configuration.
    pub health_check_api: HealthCheckApi,
}

/// Errors that can occur when loading the configuration.
#[derive(Error, Debug)]
pub enum Error {
    /// Unable to load the configuration from the environment variable.
    /// This error only occurs if there is no configuration file and the
    /// `TORRUST_TRACKER_CONFIG` environment variable is not set.
    #[error("Unable to load from Environmental Variable: {source}")]
    UnableToLoadFromEnvironmentVariable {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to load from Config File: {source}")]
    UnableToLoadFromConfigFile {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    /// Unable to load the configuration from the configuration file.
    #[error("Failed processing the configuration: {source}")]
    ConfigError { source: LocatedError<'static, ConfigError> },

    #[error("The error for errors that can never happen.")]
    Infallible,
}

impl From<ConfigError> for Error {
    #[track_caller]
    fn from(err: ConfigError) -> Self {
        Self::ConfigError {
            source: Located(err).into(),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let announce_policy = AnnouncePolicy::default();

        let mut configuration = Configuration {
            log_level: Option::from(String::from("info")),
            mode: TrackerMode::Public,
            db_driver: DatabaseDriver::Sqlite3,
            db_path: String::from("./storage/tracker/lib/database/sqlite3.db"),
            announce_interval: announce_policy.interval,
            min_announce_interval: announce_policy.interval_min,
            max_peer_timeout: 900,
            on_reverse_proxy: false,
            external_ip: Some(String::from("0.0.0.0")),
            tracker_usage_statistics: true,
            persistent_torrent_completed_stat: false,
            inactive_peer_cleanup_interval: 600,
            remove_peerless_torrents: true,
            udp_trackers: Vec::new(),
            http_trackers: Vec::new(),
            http_api: HttpApi {
                enabled: true,
                bind_address: String::from("127.0.0.1:1212"),
                ssl_enabled: false,
                ssl_cert_path: None,
                ssl_key_path: None,
                access_tokens: [(String::from("admin"), String::from("MyAccessToken"))]
                    .iter()
                    .cloned()
                    .collect(),
            },
            health_check_api: HealthCheckApi {
                bind_address: String::from("127.0.0.1:1313"),
            },
        };
        configuration.udp_trackers.push(UdpTracker {
            enabled: false,
            bind_address: String::from("0.0.0.0:6969"),
        });
        configuration.http_trackers.push(HttpTracker {
            enabled: false,
            bind_address: String::from("0.0.0.0:7070"),
            ssl_enabled: false,
            ssl_cert_path: None,
            ssl_key_path: None,
        });
        configuration
    }
}

impl Configuration {
    fn override_api_admin_token(&mut self, api_admin_token: &str) {
        self.http_api.override_admin_token(api_admin_token);
    }

    /// Returns the tracker public IP address id defined in the configuration,
    /// and `None` otherwise.
    #[must_use]
    pub fn get_ext_ip(&self) -> Option<IpAddr> {
        match &self.external_ip {
            None => None,
            Some(external_ip) => match IpAddr::from_str(external_ip) {
                Ok(external_ip) => Some(external_ip),
                Err(_) => None,
            },
        }
    }

    /// Loads the configuration from the configuration file.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `path` does not exist or has a bad configuration.    
    pub fn load_from_file(path: &str) -> Result<Configuration, Error> {
        let config_builder = Config::builder();

        #[allow(unused_assignments)]
        let mut config = Config::default();

        config = config_builder.add_source(File::with_name(path)).build()?;

        let torrust_config: Configuration = config.try_deserialize()?;

        Ok(torrust_config)
    }

    /// Saves the default configuration at the given path.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `path` is not a valid path or the configuration
    /// file cannot be created.
    pub fn create_default_configuration_file(path: &str) -> Result<Configuration, Error> {
        let config = Configuration::default();
        config.save_to_file(path)?;
        Ok(config)
    }

    /// Loads the configuration from the `Info` struct. The whole
    /// configuration in toml format is included in the `info.tracker_toml` string.
    ///
    /// Optionally will override the admin api token.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the environment variable does not exist or has a bad configuration.
    pub fn load(info: &Info) -> Result<Configuration, Error> {
        let config_builder = Config::builder()
            .add_source(File::from_str(&info.tracker_toml, FileFormat::Toml))
            .build()?;
        let mut config: Configuration = config_builder.try_deserialize()?;

        if let Some(ref token) = info.api_admin_token {
            config.override_api_admin_token(token);
        };

        Ok(config)
    }

    /// Saves the configuration to the configuration file.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `filename` does not exist or the user does not have
    /// permission to read it. Will also return `Err` if the configuration is
    /// not valid or cannot be encoded to TOML.
    ///
    /// # Panics
    ///
    /// Will panic if the configuration cannot be written into the file.
    pub fn save_to_file(&self, path: &str) -> Result<(), Error> {
        fs::write(path, self.to_toml()).expect("Could not write to file!");
        Ok(())
    }

    /// Encodes the configuration to TOML.
    fn to_toml(&self) -> String {
        toml::to_string(self).expect("Could not encode TOML value")
    }
}

#[cfg(test)]
mod tests {
    use crate::Configuration;

    #[cfg(test)]
    fn default_config_toml() -> String {
        let config = r#"log_level = "info"
                                mode = "public"
                                db_driver = "Sqlite3"
                                db_path = "./storage/tracker/lib/database/sqlite3.db"
                                announce_interval = 120
                                min_announce_interval = 120
                                on_reverse_proxy = false
                                external_ip = "0.0.0.0"
                                tracker_usage_statistics = true
                                persistent_torrent_completed_stat = false
                                max_peer_timeout = 900
                                inactive_peer_cleanup_interval = 600
                                remove_peerless_torrents = true

                                [[udp_trackers]]
                                enabled = false
                                bind_address = "0.0.0.0:6969"

                                [[http_trackers]]
                                enabled = false
                                bind_address = "0.0.0.0:7070"
                                ssl_enabled = false
                                ssl_cert_path = ""
                                ssl_key_path = ""

                                [http_api]
                                enabled = true
                                bind_address = "127.0.0.1:1212"
                                ssl_enabled = false
                                ssl_cert_path = ""
                                ssl_key_path = ""

                                [http_api.access_tokens]
                                admin = "MyAccessToken"

                                [health_check_api]
                                bind_address = "127.0.0.1:1313"
        "#
        .lines()
        .map(str::trim_start)
        .collect::<Vec<&str>>()
        .join("\n");
        config
    }

    #[test]
    fn configuration_should_have_default_values() {
        let configuration = Configuration::default();

        let toml = toml::to_string(&configuration).expect("Could not encode TOML value");

        assert_eq!(toml, default_config_toml());
    }

    #[test]
    fn configuration_should_contain_the_external_ip() {
        let configuration = Configuration::default();

        assert_eq!(configuration.external_ip, Some(String::from("0.0.0.0")));
    }

    #[test]
    fn configuration_should_be_saved_in_a_toml_config_file() {
        use std::{env, fs};

        use uuid::Uuid;

        // Build temp config file path
        let temp_directory = env::temp_dir();
        let temp_file = temp_directory.join(format!("test_config_{}.toml", Uuid::new_v4()));

        // Convert to argument type for Configuration::save_to_file
        let config_file_path = temp_file;
        let path = config_file_path.to_string_lossy().to_string();

        let default_configuration = Configuration::default();

        default_configuration
            .save_to_file(&path)
            .expect("Could not save configuration to file");

        let contents = fs::read_to_string(&path).expect("Something went wrong reading the file");

        assert_eq!(contents, default_config_toml());
    }

    #[cfg(test)]
    fn create_temp_config_file_with_default_config() -> String {
        use std::env;
        use std::fs::File;
        use std::io::Write;

        use uuid::Uuid;

        // Build temp config file path
        let temp_directory = env::temp_dir();
        let temp_file = temp_directory.join(format!("test_config_{}.toml", Uuid::new_v4()));

        // Convert to argument type for Configuration::load_from_file
        let config_file_path = temp_file.clone();
        let path = config_file_path.to_string_lossy().to_string();

        // Write file contents
        let mut file = File::create(temp_file).unwrap();
        writeln!(&mut file, "{}", default_config_toml()).unwrap();

        path
    }

    #[test]
    fn configuration_should_be_loaded_from_a_toml_config_file() {
        let config_file_path = create_temp_config_file_with_default_config();

        let configuration = Configuration::load_from_file(&config_file_path).expect("Could not load configuration from file");

        assert_eq!(configuration, Configuration::default());
    }

    #[test]
    fn http_api_configuration_should_check_if_it_contains_a_token() {
        let configuration = Configuration::default();

        assert!(configuration.http_api.contains_token("MyAccessToken"));
        assert!(!configuration.http_api.contains_token("NonExistingToken"));
    }
}
