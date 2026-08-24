//! Version `3` for [Torrust Tracker](https://docs.rs/torrust-tracker)
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
//! environment variable `TORRUST_TRACKER_CONFIG_TOML`. This is useful when running
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
//! - [TLS support](#tls-support)
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
//! - [`Core configuration`](crate::v3_0_0::Configuration)
//! - [`HTTP API configuration`](crate::v3_0_0::tracker_api::HttpApi)
//! - [`HTTP Tracker configuration`](crate::v3_0_0::http_tracker::HttpTracker)
//! - [`UDP Tracker configuration`](crate::v3_0_0::udp_tracker::UdpTracker)
//! - [`UDP Tracker server configuration`](crate::v3_0_0::udp_tracker_server::UdpTrackerServer)
//! - [`Health Check API configuration`](crate::v3_0_0::health_check_api::HealthCheckApi)
//!
//! ## Port binding
//!
//! For the API, HTTP and UDP trackers you can bind to a random port by using
//! port `0`. For example, if you want to bind to a random port on all
//! interfaces, use `0.0.0.0:0`. The OS will choose a random free port.
//!
//! ## TLS support
//!
//! For the API and HTTP tracker you can enable TLS by providing a
//! `[http_api.tls_config]` or `[[http_trackers]].tls_config` section with
//! the paths to the certificate and key files.
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
//! Alternatively, you could set up a reverse proxy like Nginx or Apache to
//! handle the SSL/TLS part and forward the requests to the tracker. If you do
//! that, you should set
//! [`http_trackers.network.on_reverse_proxy`](crate::v3_0_0::network::Network::on_reverse_proxy)
//! to `true` for that tracker in the configuration file. It's out of scope for this
//! documentation to explain in detail how to set up a reverse proxy, but the
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
//! ...
//!
//! [http_trackers.tls_config]
//! ssl_cert_path = "./storage/tracker/lib/tls/localhost.crt"
//! ssl_key_path = "./storage/tracker/lib/tls/localhost.key"
//!
//! [http_api]
//! ...
//!
//! [http_api.tls_config]
//! ssl_cert_path = "./storage/tracker/lib/tls/localhost.crt"
//! ssl_key_path = "./storage/tracker/lib/tls/localhost.key"
//! ```
//!
//! ## Type conventions for configuration fields
//!
//! Configuration struct fields whose value space is **smaller than the raw primitive** must be
//! represented as typed newtypes, not as `String`, `u32`, or other unvalidated primitives.
//! The constraint is encoded in the type and validated once at deserialization; consuming code
//! never re-validates it.
//!
//! | Field constraint | Do this | Not this |
//! |---|---|---|
//! | URL must be `http`/`https` | `Option<HttpUrl>` | `Option<String>` |
//! | URL must be `udp` | `Option<UdpUrl>` | `Option<String>` |
//!
//! See [`public_url`] for the canonical examples and
//! [ADR 20260721100000](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md)
//! for the full rationale, the granularity decision, and the compile-time vs runtime split.
//!
//! ## Default configuration
//!
//! The default configuration is:
//!
//! ```toml
//! [logging]
//! trace_filter = "info"
//! trace_style = "full"
//!
//! [core]
//! inactive_peer_cleanup_interval = 600
//! listed = false
//! private = false
//! tracker_usage_statistics = true
//!
//! [core.announce_policy]
//! interval = 120
//! interval_min = 120
//! max_peers_per_announce = 74
//!
//! [core.database]
//! driver = "sqlite3"
//! path = "./storage/tracker/lib/database/sqlite3.db"
//!
//! [core.tracker_policy]
//! max_peer_timeout = 900
//! persistent_torrent_completed_stat = false
//! remove_peerless_torrents = true
//!
//! [udp_tracker_server]
//! ip_bans_reset_interval_in_secs = 86400
//!
//! [http_api]
//! bind_address = "127.0.0.1:1212"
//!
//! [http_api.access_tokens]
//! admin = "MyAccessToken"
//! [health_check_api]
//! bind_address = "127.0.0.1:1313"
//!```
// ── Top-level configuration section structs ───────────────────────────────────
// One module per TOML section; each maps directly to a key in `Configuration`.
pub mod core;
pub mod health_check_api;
pub mod http_tracker;
pub mod logging;
pub mod tracker_api;
pub mod types;
pub mod udp_tracker;
pub mod udp_tracker_server;

// ── Sub-configuration block structs ───────────────────────────────────────────
// Embedded inside the section structs above; each maps to a TOML sub-block
// (e.g. `[http_trackers.tls_config]`, `[http_trackers.network]`).
pub mod database;
pub mod network;
pub mod tls;

// ── Value newtypes ────────────────────────────────────────────────────────────
// Single-value types that encode a domain invariant (scheme, format, range).
// When this group grows, consider extracting these into a `types/` submodule.
pub mod public_url;

use std::fs;

use figment::Figment;
use figment::providers::{Env, Format, Serialized, Toml};
use logging::Logging;
use serde::{Deserialize, Serialize};

use self::core::Core;
use self::health_check_api::HealthCheckApi;
use self::http_tracker::HttpTracker;
use self::tracker_api::HttpApi;
use self::udp_tracker::UdpTracker;
use self::udp_tracker_server::UdpTrackerServer;
use crate::validator::{SemanticValidationError, Validator};
use crate::{Error, Info, Metadata, Version};

/// This configuration version
const VERSION_3_0_0: &str = "3.0.0";

/// Prefix for env vars that overwrite configuration options.
const CONFIG_OVERRIDE_PREFIX: &str = "TORRUST_TRACKER_CONFIG_OVERRIDE_";

/// Path separator in env var names for nested values in configuration.
const CONFIG_OVERRIDE_SEPARATOR: &str = "__";

/// Core configuration for the tracker.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    /// Configuration metadata.
    pub metadata: Metadata,

    /// Logging configuration
    pub logging: Logging,

    /// Core configuration.
    pub core: Core,

    /// The list of UDP trackers the tracker is running. Each UDP tracker
    /// represents a UDP server that the tracker is running and it has its own
    /// configuration.
    pub udp_trackers: Option<Vec<UdpTracker>>,

    /// The list of HTTP trackers the tracker is running. Each HTTP tracker
    /// represents a HTTP server that the tracker is running and it has its own
    /// configuration.
    pub http_trackers: Option<Vec<HttpTracker>>,

    /// Configuration shared by every UDP tracker listener.
    #[serde(default = "UdpTrackerServer::default")]
    pub udp_tracker_server: UdpTrackerServer,

    /// The HTTP API configuration.
    pub http_api: Option<HttpApi>,

    /// The Health Check API configuration.
    pub health_check_api: HealthCheckApi,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            metadata: Metadata::with_schema_version(Version::new(VERSION_3_0_0)),
            logging: Logging::default(),
            core: Core::default(),
            udp_trackers: None,
            http_trackers: None,
            udp_tracker_server: UdpTrackerServer::default(),
            http_api: None,
            health_check_api: HealthCheckApi::default(),
        }
    }
}

impl Configuration {
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
    /// configuration in toml format is included in the `info.tracker_toml`
    /// string.
    ///
    /// Configuration provided via env var has priority over config file path.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the environment variable does not exist or has a bad configuration.
    pub fn load(info: &Info) -> Result<Configuration, Error> {
        // Load configuration provided by the user, prioritizing env vars
        let figment = if let Some(config_toml) = &info.config_toml {
            Figment::from(Toml::string(config_toml)).merge(Env::prefixed(CONFIG_OVERRIDE_PREFIX).split(CONFIG_OVERRIDE_SEPARATOR))
        } else {
            Figment::from(Toml::file(&info.config_toml_path))
                .merge(Env::prefixed(CONFIG_OVERRIDE_PREFIX).split(CONFIG_OVERRIDE_SEPARATOR))
        };

        // Make sure user has provided the mandatory options.
        Self::check_mandatory_options(&figment)?;

        // Fill missing options with default values.
        let figment = figment.join(Serialized::defaults(Configuration::default()));

        // Build final configuration.
        let config: Configuration = figment.extract()?;

        // Make sure the provided schema version matches this version.
        if config.metadata.schema_version != Version::new(VERSION_3_0_0) {
            return Err(Error::UnsupportedVersion {
                version: config.metadata.schema_version,
            });
        }

        Ok(config)
    }

    /// Some configuration options are mandatory. The tracker will panic if
    /// the user doesn't provide an explicit value for them from one of the
    /// configuration sources: TOML or ENV VARS.
    ///
    /// # Errors
    ///
    /// Will return an error if a mandatory configuration option is only
    /// obtained by default value (code), meaning the user hasn't overridden it.
    fn check_mandatory_options(figment: &Figment) -> Result<(), Error> {
        let mandatory_options = [
            "metadata.schema_version",
            "logging.trace_filter",
            "core.private",
            "core.listed",
        ];

        for mandatory_option in mandatory_options {
            figment
                .find_value(mandatory_option)
                .map_err(|_err| Error::MissingMandatoryOption {
                    path: mandatory_option.to_owned(),
                })?;
        }

        Ok(())
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
        fs::write(path, self.serialize_toml_for_persistence()).expect("Could not write to file!");
        Ok(())
    }

    /// Encodes the configuration to TOML for an authorized persistence boundary.
    ///
    /// # Panics
    ///
    /// Will panic if it can't be converted to TOML.
    #[must_use]
    fn serialize_toml_for_persistence(&self) -> String {
        if self.http_api.is_none() {
            return toml::to_string(self).expect("Could not encode TOML value");
        }

        let mut configuration = toml::Value::try_from(self).expect("Could not encode TOML value");

        if let Some(http_api) = &self.http_api {
            configuration
                .get_mut("http_api")
                .and_then(toml::Value::as_table_mut)
                .expect("HTTP API configuration should serialize to a TOML table")
                .insert(
                    "access_tokens".to_string(),
                    toml::Value::Table(http_api.serialize_access_tokens_for_persistence()),
                );
        }

        toml::to_string(&configuration).expect("Could not encode TOML value")
    }

    /// Encodes the configuration to redacted JSON for diagnostics.
    ///
    /// # Panics
    ///
    /// Will panic if it can't be converted to JSON.
    #[must_use]
    pub fn to_redacted_json(&self) -> String {
        serde_json::to_string_pretty(&self.clone().mask_secrets()).expect("Could not encode JSON value")
    }

    /// Masks secrets in the configuration.
    #[must_use]
    pub fn mask_secrets(mut self) -> Self {
        self.core.database.mask_secrets();

        if let Some(ref mut api) = self.http_api {
            api.redact_access_tokens_for_diagnostic_output();
        }

        self
    }
}

impl Validator for Configuration {
    fn validate(&self) -> Result<(), SemanticValidationError> {
        self.core.validate()
    }
}

#[cfg(test)]
mod tests {

    use std::convert::TryFrom;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use crate::Info;
    use crate::v3_0_0::Configuration;
    use crate::v3_0_0::http_tracker::HttpTracker;
    use crate::v3_0_0::logging::TraceStyle;
    use crate::v3_0_0::network::ExternalIp;
    use crate::v3_0_0::tracker_api::HttpApi;
    use crate::v3_0_0::udp_tracker::UdpTracker;

    #[cfg(test)]
    fn default_config_toml() -> String {
        r#"[metadata]
                                app = "torrust-tracker"
                                purpose = "configuration"
                                schema_version = "3.0.0"

                                [logging]
                                trace_filter = "info"
                                trace_style = "full"

                                [core]
                                inactive_peer_cleanup_interval = 600
                                listed = false
                                private = false
                                tracker_usage_statistics = true

                                [core.announce_policy]
                                interval = 120
                                interval_min = 120
                                max_peers_per_announce = 74

                                [core.database]
                                driver = "sqlite3"
                                path = "./storage/tracker/lib/database/sqlite3.db"

                                [core.tracker_policy]
                                max_peer_timeout = 900
                                persistent_torrent_completed_stat = false
                                remove_peerless_torrents = true

                                [udp_tracker_server]
                                ip_bans_reset_interval_in_secs = 86400
                                connection_id_validation = "strict"

                                [health_check_api]
                                bind_address = "127.0.0.1:1313"
        "#
        .lines()
        .map(str::trim_start)
        .collect::<Vec<&str>>()
        .join("\n")
    }

    #[test]
    fn configuration_should_have_default_values() {
        let configuration = Configuration::default();

        let toml = toml::to_string(&configuration).expect("Could not encode TOML value");

        assert_eq!(toml, default_config_toml());
    }

    #[test]
    fn tracker_defaults_should_not_contain_an_external_ip() {
        assert_eq!(HttpTracker::default().network.external_ip, None);
        assert_eq!(UdpTracker::default().network.external_ip, None);
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn configuration_should_deserialize_a_custom_ip_bans_reset_interval() {
        figment::Jail::expect_with(|_jail| {
            let info = Info {
                config_toml: r#"
                    [metadata]
                    schema_version = "3.0.0"

                    [logging]
                    trace_filter = "info"
                    trace_style = "json"

                    [core]
                    listed = false
                    private = false

                    [udp_tracker_server]
                    ip_bans_reset_interval_in_secs = 7200
                "#
                .to_string()
                .into(),
                config_toml_path: String::new(),
            };

            let configuration = Configuration::load(&info).expect("configuration should load");

            assert_eq!(configuration.udp_tracker_server.ip_bans_reset_interval_in_secs.get(), 7200);
            assert_eq!(configuration.logging.trace_style, TraceStyle::Json);

            Ok(())
        });
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
    #[allow(clippy::result_large_err)]
    fn configuration_should_use_the_default_values_when_only_the_mandatory_options_are_provided_by_the_user_via_toml_file() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                "tracker.toml",
                r#"
                [metadata]
                schema_version = "3.0.0"

                [logging]
                trace_filter = "info"

                [core]
                listed = false
                private = false
            "#,
            )?;

            let info = Info {
                config_toml: None,
                config_toml_path: "tracker.toml".to_string(),
            };

            let configuration = Configuration::load(&info).expect("Could not load configuration from file");

            assert_eq!(
                toml::to_string(&configuration).expect("default configuration should serialize"),
                default_config_toml()
            );

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn configuration_should_use_the_default_values_when_only_the_mandatory_options_are_provided_by_the_user_via_toml_content() {
        figment::Jail::expect_with(|_jail| {
            let config_toml = r#"
                [metadata]
                schema_version = "3.0.0"

                [logging]
                trace_filter = "info"

                [core]
                listed = false
                private = false
            "#
            .to_string();

            let info = Info {
                config_toml: Some(config_toml),
                config_toml_path: String::new(),
            };

            let configuration = Configuration::load(&info).expect("Could not load configuration from file");

            assert_eq!(
                toml::to_string(&configuration).expect("default configuration should serialize"),
                default_config_toml()
            );

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn default_configuration_could_be_overwritten_from_a_single_env_var_with_toml_contents() {
        figment::Jail::expect_with(|_jail| {
            let config_toml = r#"
                [metadata]
                schema_version = "3.0.0"

                [logging]
                trace_filter = "info"

                [core]
                listed = false
                private = false

                [core.database]
                path = "OVERWRITTEN DEFAULT DB PATH"
            "#
            .to_string();

            let info = Info {
                config_toml: Some(config_toml),
                config_toml_path: String::new(),
            };

            let configuration = Configuration::load(&info).expect("Could not load configuration from file");

            assert_eq!(configuration.core.database.path, "OVERWRITTEN DEFAULT DB PATH".to_string());

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn default_configuration_could_be_overwritten_from_a_toml_config_file() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                "tracker.toml",
                r#"
                [metadata]
                schema_version = "3.0.0"

                [logging]
                trace_filter = "info"

                [core]
                listed = false
                private = false

                [core.database]
                path = "OVERWRITTEN DEFAULT DB PATH"
            "#,
            )?;

            let info = Info {
                config_toml: None,
                config_toml_path: "tracker.toml".to_string(),
            };

            let configuration = Configuration::load(&info).expect("Could not load configuration from file");

            assert_eq!(configuration.core.database.path, "OVERWRITTEN DEFAULT DB PATH".to_string());

            Ok(())
        });
    }

    #[allow(clippy::result_large_err)]
    #[test]
    fn configuration_should_allow_to_overwrite_the_default_tracker_api_token_for_admin_with_an_env_var() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("TORRUST_TRACKER_CONFIG_OVERRIDE_HTTP_API__ACCESS_TOKENS__ADMIN", "NewToken");

            let info = Info {
                config_toml: Some(default_config_toml()),
                config_toml_path: String::new(),
            };

            let configuration = Configuration::load(&info).expect("Could not load configuration from file");

            let formatted = format!("{:?}", configuration.http_api.unwrap().access_tokens);

            assert!(formatted.contains("SecretBox<str>([REDACTED])"));
            assert!(!formatted.contains("NewToken"));

            Ok(())
        });
    }

    #[test]
    fn configuration_json_output_should_redact_access_tokens() {
        let token = "v3-token-only-for-json-redaction-test";
        let mut configuration = Configuration::default();
        let mut http_api = HttpApi::default();
        http_api.add_token("admin", token);
        configuration.http_api = Some(http_api);

        let json = configuration.to_redacted_json();

        assert!(json.contains("\"***\""));
        assert!(!json.contains(token));
    }

    #[test]
    fn persisted_configuration_toml_should_include_access_tokens() {
        let token = "v3-token-only-for-toml-persistence-test";
        let mut configuration = Configuration::default();
        let mut http_api = HttpApi::default();
        http_api.add_token("admin", token);
        configuration.http_api = Some(http_api);

        let toml = configuration.serialize_toml_for_persistence();

        assert!(toml.contains("[http_api.access_tokens]"));
        assert!(toml.contains(token));
    }

    #[test]
    fn external_ip_should_reject_unspecified_ipv4_address() {
        let result = ExternalIp::try_from(IpAddr::V4(Ipv4Addr::UNSPECIFIED));
        assert!(result.is_err());
    }

    #[test]
    fn external_ip_should_reject_unspecified_ipv6_address() {
        let result = ExternalIp::try_from(IpAddr::V6(Ipv6Addr::UNSPECIFIED));
        assert!(result.is_err());
    }

    #[test]
    fn external_ip_should_accept_valid_ipv4_address() {
        let result = ExternalIp::try_from(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 5)));
        assert!(result.is_ok());
    }

    #[test]
    fn external_ip_should_parse_from_str() {
        let ip: Result<ExternalIp, _> = "203.0.113.5".parse();
        assert!(ip.is_ok());
        let ip: Result<ExternalIp, _> = "0.0.0.0".parse();
        assert!(ip.is_err());
        let ip: Result<ExternalIp, _> = "::".parse();
        assert!(ip.is_err());
    }

    #[cfg(test)]
    mod deserialization {
        use std::net::{IpAddr, Ipv4Addr};

        use figment::Jail;

        use crate::Info;
        use crate::v3_0_0::Configuration;

        #[allow(clippy::result_large_err)]
        #[test]
        fn it_should_deserialize_network_settings_from_a_http_tracker_network_block() {
            Jail::expect_with(|jail| {
                jail.create_file(
                    "tracker.toml",
                    r#"
                    [metadata]
                    schema_version = "3.0.0"

                    [logging]
                    trace_filter = "info"

                    [core]
                    listed = false
                    private = false

                    [[http_trackers]]
                    bind_address = "127.0.0.1:7070"

                    [http_trackers.network]
                    external_ip = "203.0.113.5"
                    on_reverse_proxy = true
                    ipv6_v6only = true
                "#,
                )?;

                let info = Info {
                    config_toml: None,
                    config_toml_path: "tracker.toml".to_string(),
                };

                let config = Configuration::load(&info).expect("Should load config");
                let network = &config.http_trackers.expect("HTTP tracker should be configured")[0].network;
                assert_eq!(
                    network.external_ip,
                    Some(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 5)).try_into().expect("valid IP"))
                );
                assert!(network.on_reverse_proxy, "on_reverse_proxy should be true");
                assert!(network.ipv6_v6only, "ipv6_v6only should be true");

                Ok(())
            });
        }

        #[allow(clippy::result_large_err)]
        #[test]
        fn it_should_deserialize_network_settings_from_a_udp_tracker_network_block() {
            Jail::expect_with(|jail| {
                jail.create_file(
                    "tracker.toml",
                    r#"
                    [metadata]
                    schema_version = "3.0.0"

                    [logging]
                    trace_filter = "info"

                    [core]
                    listed = false
                    private = false

                    [[udp_trackers]]
                    bind_address = "127.0.0.1:6969"

                    [udp_trackers.network]
                    external_ip = "203.0.113.5"
                    on_reverse_proxy = true
                    ipv6_v6only = true
                "#,
                )?;

                let info = Info {
                    config_toml: None,
                    config_toml_path: "tracker.toml".to_string(),
                };

                let config = Configuration::load(&info).expect("Should load config");
                let network = &config.udp_trackers.expect("UDP tracker should be configured")[0].network;
                assert_eq!(
                    network.external_ip,
                    Some(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 5)).try_into().expect("valid IP"))
                );
                assert!(network.on_reverse_proxy, "on_reverse_proxy should be true");
                assert!(network.ipv6_v6only, "ipv6_v6only should be true");

                Ok(())
            });
        }

        #[allow(clippy::result_large_err)]
        #[test]
        fn it_should_use_safe_network_defaults_when_the_network_block_is_omitted() {
            Jail::expect_with(|jail| {
                jail.create_file(
                    "tracker.toml",
                    r#"
                    [metadata]
                    schema_version = "3.0.0"

                    [logging]
                    trace_filter = "info"

                    [core]
                    listed = false
                    private = false

                    [[http_trackers]]
                    bind_address = "127.0.0.1:7070"

                    [[udp_trackers]]
                    bind_address = "127.0.0.1:6969"
                "#,
                )?;

                let info = Info {
                    config_toml: None,
                    config_toml_path: "tracker.toml".to_string(),
                };

                let configuration = Configuration::load(&info).expect("configuration should load");
                let http_network = &configuration.http_trackers.expect("HTTP tracker should be configured")[0].network;
                let udp_network = &configuration.udp_trackers.expect("UDP tracker should be configured")[0].network;

                assert_eq!(http_network.external_ip, None);
                assert!(!http_network.on_reverse_proxy);
                assert!(!http_network.ipv6_v6only);
                assert_eq!(udp_network, http_network);

                Ok(())
            });
        }

        #[allow(clippy::result_large_err)]
        #[test]
        fn it_should_reject_the_removed_core_network_layout() {
            Jail::expect_with(|jail| {
                jail.create_file(
                    "tracker.toml",
                    r#"
                    [metadata]
                    schema_version = "3.0.0"

                    [logging]
                    trace_filter = "info"

                    [core]
                    listed = false
                    private = false

                    [core.net]
                    external_ip = "203.0.113.5"
                "#,
                )?;

                let info = Info {
                    config_toml: None,
                    config_toml_path: "tracker.toml".to_string(),
                };

                let result = Configuration::load(&info);
                assert!(result.is_err(), "v3 must reject the removed core.net layout");

                Ok(())
            });
        }

        #[allow(clippy::result_large_err)]
        #[test]
        fn it_should_reject_the_removed_flat_tracker_ipv6_v6only_field() {
            Jail::expect_with(|jail| {
                jail.create_file(
                    "tracker.toml",
                    r#"
                    [metadata]
                    schema_version = "3.0.0"

                    [logging]
                    trace_filter = "info"

                    [core]
                    listed = false
                    private = false

                    [[http_trackers]]
                    bind_address = "127.0.0.1:7070"
                    ipv6_v6only = true
                "#,
                )?;

                let info = Info {
                    config_toml: None,
                    config_toml_path: "tracker.toml".to_string(),
                };

                let result = Configuration::load(&info);
                assert!(result.is_err(), "v3 must reject the removed flat ipv6_v6only field");

                Ok(())
            });
        }

        #[allow(clippy::result_large_err)]
        #[test]
        fn it_should_reject_the_removed_flat_udp_tracker_ipv6_v6only_field() {
            Jail::expect_with(|jail| {
                jail.create_file(
                    "tracker.toml",
                    r#"
                    [metadata]
                    schema_version = "3.0.0"

                    [logging]
                    trace_filter = "info"

                    [core]
                    listed = false
                    private = false

                    [[udp_trackers]]
                    bind_address = "127.0.0.1:6969"
                    ipv6_v6only = true
                "#,
                )?;

                let info = Info {
                    config_toml: None,
                    config_toml_path: "tracker.toml".to_string(),
                };

                let result = Configuration::load(&info);
                assert!(result.is_err(), "v3 must reject the removed flat ipv6_v6only field");

                Ok(())
            });
        }
    }

    mod smoke {
        use crate::Info;
        use crate::v3_0_0::Configuration;

        #[allow(clippy::result_large_err)]
        #[test]
        fn v3_configuration_should_load_when_schema_version_is_3_0_0() {
            figment::Jail::expect_with(|_jail| {
                let config_toml = r#"
                    [metadata]
                    schema_version = "3.0.0"

                    [logging]
                    trace_filter = "info"

                    [core]
                    listed = false
                    private = false
                "#
                .to_string();

                let info = Info {
                    config_toml: Some(config_toml),
                    config_toml_path: String::new(),
                };

                let result = Configuration::load(&info);
                assert!(result.is_ok(), "v3 configuration should load with schema_version 3.0.0");

                Ok(())
            });
        }

        #[allow(clippy::result_large_err)]
        #[test]
        fn v3_configuration_should_reject_schema_version_2_0_0() {
            figment::Jail::expect_with(|_jail| {
                let config_toml = r#"
                    [metadata]
                    schema_version = "2.0.0"

                    [logging]
                    trace_filter = "info"

                    [core]
                    listed = false
                    private = false
                "#
                .to_string();

                let info = Info {
                    config_toml: Some(config_toml),
                    config_toml_path: String::new(),
                };

                let result = Configuration::load(&info);
                assert!(result.is_err(), "v3 configuration should reject schema_version 2.0.0");

                Ok(())
            });
        }
    }

    /// Test-only feasibility experiments for issue #2067.
    ///
    /// These local types deliberately do not change the public v3 schema. They
    /// exercise the TOML, Serde, and Figment behavior a separately approved
    /// schema implementation would rely on.
    mod flat_service_configuration_prototype {
        use figment::Figment;
        use figment::providers::{Env, Format, Toml};
        use serde::{Deserialize, Serialize};
        use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

        use crate::v3_0_0::health_check_api::HealthCheckApi;
        use crate::v3_0_0::http_tracker::HttpTracker;
        use crate::v3_0_0::tracker_api::HttpApi;
        use crate::v3_0_0::udp_tracker::UdpTracker;

        const OVERRIDE_PREFIX: &str = "TORRUST_TRACKER_CONFIG_OVERRIDE_";
        const OVERRIDE_SEPARATOR: &str = "__";

        #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
        #[serde(tag = "kind", content = "configuration", rename_all = "snake_case", deny_unknown_fields)]
        enum AdjacentService {
            HttpTracker(HttpTracker),
            UdpTracker(UdpTracker),
            HttpApi(HttpApi),
            HealthCheckApi(HealthCheckApi),
        }

        #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
        #[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
        enum FlattenedService {
            HttpTracker {
                #[serde(flatten)]
                configuration: HttpTracker,
            },
            UdpTracker {
                #[serde(flatten)]
                configuration: UdpTracker,
            },
            HttpApi {
                #[serde(flatten)]
                configuration: HttpApi,
            },
            HealthCheckApi {
                #[serde(flatten)]
                configuration: HealthCheckApi,
            },
        }

        #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
        #[serde(rename_all = "snake_case", deny_unknown_fields)]
        enum ExternallyTaggedService {
            HttpTracker(HttpTracker),
            UdpTracker(UdpTracker),
            HttpApi(HttpApi),
            HealthCheckApi(HealthCheckApi),
        }

        #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
        #[serde(deny_unknown_fields)]
        struct AdjacentServicesDocument {
            #[serde(default)]
            services: Vec<AdjacentService>,
        }

        #[derive(Serialize, Deserialize)]
        struct FlattenedServicesDocument {
            services: Vec<FlattenedService>,
        }

        #[derive(Serialize, Deserialize)]
        struct ExternallyTaggedServicesDocument {
            services: Vec<ExternallyTaggedService>,
        }

        #[allow(dead_code)]
        #[derive(Debug, Deserialize)]
        struct SplitServicesDocument {
            #[serde(default)]
            http_trackers: Vec<HttpTracker>,
        }

        fn role_and_configuration(service: &AdjacentService) -> (ServiceRole, usize) {
            match service {
                AdjacentService::HttpTracker(_) => (ServiceRole::HttpTracker, 0),
                AdjacentService::UdpTracker(_) => (ServiceRole::UdpTracker, 0),
                AdjacentService::HttpApi(_) => (ServiceRole::RestApi, 0),
                AdjacentService::HealthCheckApi(_) => (ServiceRole::HealthCheckApi, 0),
            }
        }

        fn role_local_ids(services: &[AdjacentService]) -> Vec<ConfigurationInstanceId> {
            let mut http_trackers = 0;
            let mut udp_trackers = 0;
            let mut http_apis = 0;
            let mut health_check_apis = 0;

            services
                .iter()
                .map(|service| {
                    let (role, _) = role_and_configuration(service);
                    let index = match role {
                        ServiceRole::HttpTracker => {
                            let index = http_trackers;
                            http_trackers += 1;
                            index
                        }
                        ServiceRole::UdpTracker => {
                            let index = udp_trackers;
                            udp_trackers += 1;
                            index
                        }
                        ServiceRole::RestApi => {
                            let index = http_apis;
                            http_apis += 1;
                            index
                        }
                        ServiceRole::HealthCheckApi => {
                            let index = health_check_apis;
                            health_check_apis += 1;
                            index
                        }
                    };

                    ConfigurationInstanceId::new(role, index)
                })
                .collect()
        }

        fn validate_singletons(services: &[AdjacentService]) -> Result<(), &'static str> {
            let http_apis = services
                .iter()
                .filter(|service| matches!(service, AdjacentService::HttpApi(_)))
                .count();
            let health_check_apis = services
                .iter()
                .filter(|service| matches!(service, AdjacentService::HealthCheckApi(_)))
                .count();

            if http_apis > 1 {
                return Err("services may contain at most one http_api entry");
            }
            if health_check_apis > 1 {
                return Err("services may contain at most one health_check_api entry");
            }

            Ok(())
        }

        fn mask_service_secrets(services: &mut [AdjacentService]) {
            for service in services {
                if let AdjacentService::HttpApi(configuration) = service {
                    configuration.mask_secrets();
                }
            }
        }

        #[test]
        fn adjacent_tagged_services_round_trip_and_preserve_interleaved_order() {
            let input = r#"
                [[services]]
                kind = "http_tracker"
                [services.configuration]
                bind_address = "127.0.0.1:17070"

                [services.configuration.network]
                external_ip = "203.0.113.5"
                on_reverse_proxy = true
                ipv6_v6only = true

                [services.configuration.tls_config]
                ssl_cert_path = "tracker.crt"
                ssl_key_path = "tracker.key"

                [[services]]
                kind = "udp_tracker"
                [services.configuration]
                bind_address = "127.0.0.1:16969"

                [[services]]
                kind = "http_api"
                [services.configuration]
                bind_address = "127.0.0.1:1212"

                [services.configuration.access_tokens]
                admin = "ExampleSecretToken"

                [services.configuration.tls_config]
                ssl_cert_path = "api.crt"
                ssl_key_path = "api.key"
            "#;

            let document: AdjacentServicesDocument = toml::from_str(input).expect("adjacent-tagged TOML should deserialize");
            let serialized = toml::to_string(&document).expect("services document should serialize");
            let round_tripped: AdjacentServicesDocument =
                toml::from_str(&serialized).expect("serialized services document should deserialize");

            assert_eq!(document.services, round_tripped.services);
            assert!(matches!(document.services[0], AdjacentService::HttpTracker(_)));
            assert!(matches!(document.services[1], AdjacentService::UdpTracker(_)));
            assert!(matches!(document.services[2], AdjacentService::HttpApi(_)));

            let AdjacentService::HttpTracker(http_tracker) = &document.services[0] else {
                panic!("first service should be an HTTP tracker");
            };
            assert_eq!(http_tracker.network.external_ip, Some("203.0.113.5".parse().unwrap()));
            assert!(http_tracker.network.on_reverse_proxy);
            assert!(http_tracker.network.ipv6_v6only);
            assert!(http_tracker.tls_config.is_some());

            let AdjacentService::HttpApi(http_api) = &document.services[2] else {
                panic!("third service should be an HTTP API");
            };
            assert_eq!(http_api.access_tokens.get("admin"), Some(&"ExampleSecretToken".to_string()));
            assert!(http_api.tls_config.is_some());
        }

        #[test]
        fn enum_redaction_must_traverse_http_api_before_json_serialization() {
            let mut document = AdjacentServicesDocument {
                services: vec![AdjacentService::HttpApi(HttpApi {
                    access_tokens: [("admin".to_string(), "ExampleSecretToken".to_string())].into(),
                    ..HttpApi::default()
                })],
            };

            mask_service_secrets(&mut document.services);
            let json = serde_json::to_string(&document).expect("masked services should serialize to JSON");

            assert!(json.contains("***"));
            assert!(!json.contains("ExampleSecretToken"));
        }

        #[test]
        fn flattened_services_round_trip_but_mix_kind_and_service_specific_fields() {
            let input = r#"
                [[services]]
                kind = "http_tracker"
                bind_address = "127.0.0.1:17070"
                tracker_usage_statistics = true

                [[services]]
                kind = "udp_tracker"
                bind_address = "127.0.0.1:16969"
                cookie_lifetime = { secs = 120, nanos = 0 }
            "#;

            let document: FlattenedServicesDocument = toml::from_str(input).expect("flattened TOML should deserialize");
            let serialized = toml::to_string(&document).expect("flattened services document should serialize");
            let round_tripped: FlattenedServicesDocument =
                toml::from_str(&serialized).expect("serialized flattened services document should deserialize");

            assert_eq!(document.services, round_tripped.services);
        }

        #[test]
        fn externally_tagged_services_round_trip_but_add_a_role_named_wrapper_table() {
            let input = r#"
                [[services]]
                [services.http_tracker]
                bind_address = "127.0.0.1:17070"
            "#;

            let document: ExternallyTaggedServicesDocument =
                toml::from_str(input).expect("externally-tagged TOML should deserialize");
            let serialized = toml::to_string(&document).expect("externally-tagged services document should serialize");
            let round_tripped: ExternallyTaggedServicesDocument =
                toml::from_str(&serialized).expect("serialized externally-tagged services document should deserialize");

            assert_eq!(document.services, round_tripped.services);
        }

        #[test]
        fn adjacent_tagged_services_reject_unknown_kinds() {
            let input = r#"
                [[services]]
                kind = "smtp_tracker"
                [services.configuration]
                bind_address = "127.0.0.1:25"
            "#;

            let result = toml::from_str::<AdjacentServicesDocument>(input);

            assert!(result.is_err(), "unknown service kinds must be rejected");
        }

        #[test]
        fn omitted_or_empty_services_are_valid_and_health_defaults_are_not_materialized_by_deserialization() {
            let omitted: AdjacentServicesDocument = toml::from_str("").expect("omitted services should deserialize");
            let empty: AdjacentServicesDocument = toml::from_str("services = []").expect("empty services should deserialize");

            assert!(omitted.services.is_empty());
            assert!(empty.services.is_empty());
            assert!(validate_singletons(&omitted.services).is_ok());
            assert!(validate_singletons(&empty.services).is_ok());
        }

        #[test]
        fn singleton_validation_rejects_duplicate_http_api_and_health_check_api_entries() {
            let duplicate_http_api = vec![
                AdjacentService::HttpApi(HttpApi::default()),
                AdjacentService::HttpApi(HttpApi::default()),
            ];
            let duplicate_health_check_api = vec![
                AdjacentService::HealthCheckApi(HealthCheckApi::default()),
                AdjacentService::HealthCheckApi(HealthCheckApi::default()),
            ];

            assert_eq!(
                validate_singletons(&duplicate_http_api),
                Err("services may contain at most one http_api entry")
            );
            assert_eq!(
                validate_singletons(&duplicate_health_check_api),
                Err("services may contain at most one health_check_api entry")
            );
        }

        #[test]
        fn role_local_ids_remain_stable_when_another_role_precedes_a_service() {
            let services = vec![
                AdjacentService::UdpTracker(UdpTracker::default()),
                AdjacentService::HttpTracker(HttpTracker::default()),
                AdjacentService::UdpTracker(UdpTracker::default()),
                AdjacentService::HttpTracker(HttpTracker::default()),
            ];

            assert_eq!(
                role_local_ids(&services),
                vec![
                    ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
                    ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
                    ConfigurationInstanceId::new(ServiceRole::UdpTracker, 1),
                    ConfigurationInstanceId::new(ServiceRole::HttpTracker, 1),
                ]
            );
        }

        #[allow(clippy::result_large_err)]
        #[test]
        fn figment_rejects_numeric_adjacent_service_index_overrides() {
            figment::Jail::expect_with(|jail| {
                jail.set_env(
                    "TORRUST_TRACKER_CONFIG_OVERRIDE_SERVICES__0__CONFIGURATION__BIND_ADDRESS",
                    "127.0.0.1:18080",
                );

                let figment = Figment::from(Toml::string(
                    r#"
                        [[services]]
                        kind = "http_tracker"
                        [services.configuration]
                        bind_address = "127.0.0.1:17070"
                    "#,
                ))
                .merge(Env::prefixed(OVERRIDE_PREFIX).split(OVERRIDE_SEPARATOR));
                let errors = figment
                    .extract::<AdjacentServicesDocument>()
                    .expect_err("Figment environment maps cannot override a sequence item by numeric index");

                assert!(errors.into_iter().any(|error| {
                    matches!(
                        error.kind,
                        figment::error::Kind::InvalidType(figment::error::Actual::Map, expected)
                            if expected == "a sequence"
                    )
                }));

                Ok(())
            });
        }

        #[allow(clippy::result_large_err)]
        #[test]
        fn figment_also_rejects_numeric_split_service_index_overrides() {
            figment::Jail::expect_with(|jail| {
                jail.set_env(
                    "TORRUST_TRACKER_CONFIG_OVERRIDE_HTTP_TRACKERS__0__BIND_ADDRESS",
                    "127.0.0.1:18080",
                );

                let figment = Figment::from(Toml::string(
                    r#"
                        [[http_trackers]]
                        bind_address = "127.0.0.1:17070"
                    "#,
                ))
                .merge(Env::prefixed(OVERRIDE_PREFIX).split(OVERRIDE_SEPARATOR));
                let errors = figment
                    .extract::<SplitServicesDocument>()
                    .expect_err("Figment environment maps cannot override a split sequence item by numeric index");

                assert!(errors.into_iter().any(|error| {
                    matches!(
                        error.kind,
                        figment::error::Kind::InvalidType(figment::error::Actual::Map, expected)
                            if expected == "a sequence"
                    )
                }));

                Ok(())
            });
        }
    }
}
