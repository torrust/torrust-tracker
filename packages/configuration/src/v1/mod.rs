//! Version `1` for [Torrust Tracker](https://docs.rs/torrust-tracker)
//! configuration data structures.
//!
//! This module contains the configuration data structures for the
//! Torrust Tracker, which is a `BitTorrent` tracker server.
//!
//! The configuration is loaded from a [TOML](https://toml.io/en/) file
//! `tracker.toml` in the project root folder or from an environment variable
//! with the same content as the file.
//!
//! Configuration can not only be loaded from a file, but also from an
//! environment variable `TORRUST_TRACKER_CONFIG`. This is useful when running
//! the tracker in a Docker container or environments where you do not have a
//! persistent storage or you cannot inject a configuration file. Refer to
//! [`Torrust Tracker documentation`](https://docs.rs/torrust-tracker) for more
//! information about how to pass configuration to the tracker.
//!
//! When you run the tracker without providing the configuration via a file or
//! env var, the default configuration is used.
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
//! - [`Core configuration`](crate::v1::Configuration)
//! - [`HTTP API configuration`](crate::v1::tracker_api::HttpApi)
//! - [`HTTP Tracker configuration`](crate::v1::http_tracker::HttpTracker)
//! - [`UDP Tracker configuration`](crate::v1::udp_tracker::UdpTracker)
//! - [`Health Check API configuration`](crate::v1::health_check_api::HealthCheckApi)
//!
//! ## Port binding
//!
//! For the API, HTTP and UDP trackers you can bind to a random port by using
//! port `0`. For example, if you want to bind to a random port on all
//! interfaces, use `0.0.0.0:0`. The OS will choose a random free port.
//!
//! ## TSL support
//!
//! For the API and HTTP tracker you can enable TSL by setting `ssl_enabled` to
//! `true` and setting the paths to the certificate and key files.
//!
//! Typically, you will have a `storage` directory like the following:
//!
//! ```text
//! storage/
//! ├── config.toml
//! └── tracker
//!     ├── etc
//!     │   └── tracker.toml
//!     ├── lib
//!     │   ├── database
//!     │   │   ├── sqlite3.db
//!     │   │   └── sqlite.db
//!     │   └── tls
//!     │       ├── localhost.crt
//!     │       └── localhost.key
//!     └── log
//! ```
//!
//! where the application stores all the persistent data.
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
//! log_level = "info"
//! mode = "public"
//! db_driver = "Sqlite3"
//! db_path = "./storage/tracker/lib/database/sqlite3.db"
//! announce_interval = 120
//! min_announce_interval = 120
//! on_reverse_proxy = false
//! external_ip = "0.0.0.0"
//! tracker_usage_statistics = true
//! persistent_torrent_completed_stat = false
//! max_peer_timeout = 900
//! inactive_peer_cleanup_interval = 600
//! remove_peerless_torrents = true
//!
//! [[udp_trackers]]
//! enabled = false
//! bind_address = "0.0.0.0:6969"
//!
//! [[http_trackers]]
//! enabled = false
//! bind_address = "0.0.0.0:7070"
//! ssl_enabled = false
//! ssl_cert_path = ""
//! ssl_key_path = ""
//!
//! [http_api]
//! enabled = true
//! bind_address = "127.0.0.1:1212"
//! ssl_enabled = false
//! ssl_cert_path = ""
//! ssl_key_path = ""
//!
//! [http_api.access_tokens]
//! admin = "MyAccessToken"
//! [health_check_api]
//! bind_address = "127.0.0.1:1313"
//!```
pub mod health_check_api;
pub mod http_tracker;
pub mod tracker_api;
pub mod udp_tracker;

use std::fs;
use std::net::IpAddr;
use std::str::FromStr;

use figment::providers::{Env, Format, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::{DatabaseDriver, TrackerMode};

use self::health_check_api::HealthCheckApi;
use self::http_tracker::HttpTracker;
use self::tracker_api::HttpApi;
use self::udp_tracker::UdpTracker;
use crate::{AnnouncePolicy, Error, Info};

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
    /// The clean up torrent job runs every `inactive_peer_cleanup_interval`
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
            http_api: HttpApi::default(),
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
        let figment = Figment::new()
            .merge(Toml::file(path))
            .merge(Env::prefixed("TORRUST_TRACKER_"));

        let config: Configuration = figment.extract()?;

        Ok(config)
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
        let figment = Figment::new()
            .merge(Toml::string(&info.tracker_toml))
            .merge(Env::prefixed("TORRUST_TRACKER_"));

        let mut config: Configuration = figment.extract()?;

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
        // code-review: do we need to use Figment also to serialize into toml?
        toml::to_string(self).expect("Could not encode TOML value")
    }
}

#[cfg(test)]
mod tests {

    use crate::v1::Configuration;

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

    #[test]
    fn configuration_should_be_loaded_from_a_toml_config_file() {
        figment::Jail::expect_with(|jail| {
            jail.create_file("tracker.toml", &default_config_toml())?;

            let configuration = Configuration::load_from_file("tracker.toml").expect("Could not load configuration from file");

            assert_eq!(configuration, Configuration::default());

            Ok(())
        });
    }

    #[test]
    fn configuration_should_allow_to_overwrite_the_default_tracker_api_token_for_admin() {
        figment::Jail::expect_with(|jail| {
            jail.create_file("tracker.toml", &default_config_toml())?;

            jail.set_env("TORRUST_TRACKER_HTTP_API.ACCESS_TOKENS.ADMIN", "NewToken");

            let configuration = Configuration::load_from_file("tracker.toml").expect("Could not load configuration from file");

            assert_eq!(
                configuration.http_api.access_tokens.get("admin"),
                Some("NewToken".to_owned()).as_ref()
            );

            Ok(())
        });
    }
}
