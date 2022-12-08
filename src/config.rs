use std::collections::HashMap;
use std::fs;
use std::net::IpAddr;
use std::path::Path;
use std::str::FromStr;

use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use {std, toml};

use crate::databases::driver::Driver;
use crate::tracker::mode;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UdpTracker {
    pub enabled: bool,
    pub bind_address: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct HttpTracker {
    pub enabled: bool,
    pub bind_address: String,
    pub ssl_enabled: bool,
    #[serde_as(as = "NoneAsEmptyString")]
    pub ssl_cert_path: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub ssl_key_path: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct HttpApi {
    pub enabled: bool,
    pub bind_address: String,
    pub access_tokens: HashMap<String, String>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Configuration {
    pub log_level: Option<String>,
    pub mode: mode::Mode,
    pub db_driver: Driver,
    pub db_path: String,
    pub announce_interval: u32,
    pub min_announce_interval: u32,
    pub max_peer_timeout: u32,
    pub on_reverse_proxy: bool,
    pub external_ip: Option<String>,
    pub tracker_usage_statistics: bool,
    pub persistent_torrent_completed_stat: bool,
    pub inactive_peer_cleanup_interval: u64,
    pub remove_peerless_torrents: bool,
    pub udp_trackers: Vec<UdpTracker>,
    pub http_trackers: Vec<HttpTracker>,
    pub http_api: HttpApi,
}

#[derive(Debug)]
pub enum Error {
    Message(String),
    ConfigError(ConfigError),
    IOError(std::io::Error),
    ParseError(toml::de::Error),
    TrackerModeIncompatible,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Message(e) => e.fmt(f),
            Error::ConfigError(e) => e.fmt(f),
            Error::IOError(e) => e.fmt(f),
            Error::ParseError(e) => e.fmt(f),
            Error::TrackerModeIncompatible => write!(f, "{:?}", self),
        }
    }
}

impl std::error::Error for Error {}

impl Configuration {
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

    #[must_use]
    pub fn default() -> Configuration {
        let mut configuration = Configuration {
            log_level: Option::from(String::from("info")),
            mode: mode::Mode::Public,
            db_driver: Driver::Sqlite3,
            db_path: String::from("./storage/database/data.db"),
            announce_interval: 120,
            min_announce_interval: 120,
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
                access_tokens: [(String::from("admin"), String::from("MyAccessToken"))]
                    .iter()
                    .cloned()
                    .collect(),
            },
        };
        configuration.udp_trackers.push(UdpTracker {
            enabled: false,
            bind_address: String::from("0.0.0.0:6969"),
        });
        configuration.http_trackers.push(HttpTracker {
            enabled: false,
            bind_address: String::from("0.0.0.0:6969"),
            ssl_enabled: false,
            ssl_cert_path: None,
            ssl_key_path: None,
        });
        configuration
    }

    /// # Errors
    ///
    /// Will return `Err` if `path` does not exist or has a bad configuration.
    pub fn load_from_file(path: &str) -> Result<Configuration, Error> {
        let config_builder = Config::builder();

        #[allow(unused_assignments)]
        let mut config = Config::default();

        if Path::new(path).exists() {
            config = config_builder
                .add_source(File::with_name(path))
                .build()
                .map_err(Error::ConfigError)?;
        } else {
            eprintln!("No config file found.");
            eprintln!("Creating config file..");
            let config = Configuration::default();
            config.save_to_file(path)?;
            return Err(Error::Message(
                "Please edit the config.TOML in ./storage/config folder and restart the tracker.".to_string(),
            ));
        }

        let torrust_config: Configuration = config.try_deserialize().map_err(Error::ConfigError)?;

        Ok(torrust_config)
    }

    /// # Errors
    ///
    /// Will return `Err` if `filename` does not exist or the user does not have
    /// permission to read it.
    pub fn save_to_file(&self, path: &str) -> Result<(), Error> {
        let toml_string = toml::to_string(self).expect("Could not encode TOML value");
        fs::write(path, toml_string).expect("Could not write to file!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Configuration, Error};

    #[cfg(test)]
    fn default_config_toml() -> String {
        let config = r#"log_level = "info"
                                mode = "public"
                                db_driver = "Sqlite3"
                                db_path = "./storage/database/data.db"
                                announce_interval = 120
                                min_announce_interval = 120
                                max_peer_timeout = 900
                                on_reverse_proxy = false
                                external_ip = "0.0.0.0"
                                tracker_usage_statistics = true
                                persistent_torrent_completed_stat = false
                                inactive_peer_cleanup_interval = 600
                                remove_peerless_torrents = true

                                [[udp_trackers]]
                                enabled = false
                                bind_address = "0.0.0.0:6969"

                                [[http_trackers]]
                                enabled = false
                                bind_address = "0.0.0.0:6969"
                                ssl_enabled = false
                                ssl_cert_path = ""
                                ssl_key_path = ""

                                [http_api]
                                enabled = true
                                bind_address = "127.0.0.1:1212"

                                [http_api.access_tokens]
                                admin = "MyAccessToken"
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

        assert_eq!(configuration.external_ip, Option::Some(String::from("0.0.0.0")));
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
    fn configuration_error_could_be_displayed() {
        let error = Error::TrackerModeIncompatible;

        assert_eq!(format!("{}", error), "TrackerModeIncompatible");
    }
}
