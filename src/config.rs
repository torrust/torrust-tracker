use std;
use std::collections::HashMap;
use std::fs;
use std::net::IpAddr;
use std::path::Path;
use std::str::FromStr;

use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use toml;

use crate::databases::database::DatabaseDrivers;
use crate::mode::TrackerMode;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UdpTrackerConfig {
    pub enabled: bool,
    pub bind_address: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HttpTrackerConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub ssl_enabled: bool,
    #[serde_as(as = "NoneAsEmptyString")]
    pub ssl_cert_path: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub ssl_key_path: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HttpApiConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub access_tokens: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Configuration {
    pub log_level: Option<String>,
    pub mode: TrackerMode,
    pub db_driver: DatabaseDrivers,
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
    pub udp_trackers: Vec<UdpTrackerConfig>,
    pub http_trackers: Vec<HttpTrackerConfig>,
    pub http_api: HttpApiConfig,
}

#[derive(Debug)]
pub enum ConfigurationError {
    IOError(std::io::Error),
    ParseError(toml::de::Error),
    TrackerModeIncompatible,
}

impl std::fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigurationError::IOError(e) => e.fmt(f),
            ConfigurationError::ParseError(e) => e.fmt(f),
            _ => write!(f, "{:?}", self)
        }
    }
}

impl std::error::Error for ConfigurationError {}

impl Configuration {
    pub fn load(data: &[u8]) -> Result<Configuration, toml::de::Error> {
        toml::from_slice(data)
    }

    pub fn load_file(path: &str) -> Result<Configuration, ConfigurationError> {
        match std::fs::read(path) {
            Err(e) => Err(ConfigurationError::IOError(e)),
            Ok(data) => {
                match Self::load(data.as_slice()) {
                    Ok(cfg) => {
                        Ok(cfg)
                    }
                    Err(e) => Err(ConfigurationError::ParseError(e)),
                }
            }
        }
    }

    pub fn get_ext_ip(&self) -> Option<IpAddr> {
        match &self.external_ip {
            None => None,
            Some(external_ip) => {
                match IpAddr::from_str(external_ip) {
                    Ok(external_ip) => Some(external_ip),
                    Err(_) => None
                }
            }
        }
    }
}

impl Configuration {
    pub fn default() -> Configuration {
        let mut configuration = Configuration {
            log_level: Option::from(String::from("info")),
            mode: TrackerMode::Public,
            db_driver: DatabaseDrivers::Sqlite3,
            db_path: String::from("data.db"),
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
            http_api: HttpApiConfig {
                enabled: true,
                bind_address: String::from("127.0.0.1:1212"),
                access_tokens: [(String::from("admin"), String::from("MyAccessToken"))].iter().cloned().collect(),
            },
        };
        configuration.udp_trackers.push(
            UdpTrackerConfig {
                enabled: false,
                bind_address: String::from("0.0.0.0:6969"),
            }
        );
        configuration.http_trackers.push(
            HttpTrackerConfig {
                enabled: false,
                bind_address: String::from("0.0.0.0:6969"),
                ssl_enabled: false,
                ssl_cert_path: None,
                ssl_key_path: None,
            }
        );
        configuration
    }

    pub fn load_from_file(path: &str) -> Result<Configuration, ConfigError> {
        let mut config = Config::new();

        if Path::new(path).exists() {
            config.merge(File::with_name(path))?;
        } else {
            eprintln!("No config file found.");
            eprintln!("Creating config file..");
            let config = Configuration::default();
            let _ = config.save_to_file(path);
            return Err(ConfigError::Message(format!("Please edit the config.TOML in the root folder and restart the tracker.")));
        }

        let torrust_config: Configuration = config.try_into().map_err(|e| ConfigError::Message(format!("Errors while processing config: {}.", e)))?;

        Ok(torrust_config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), ()> {
        let toml_string = toml::to_string(self).expect("Could not encode TOML value");
        fs::write(path, toml_string).expect("Could not write to file!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    fn default_config_toml() -> String {
        let config = r#"log_level = "info"
                                mode = "public"
                                db_driver = "Sqlite3"
                                db_path = "data.db"
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
        "#.lines().map(|line| line.trim_start()).collect::<Vec<&str>>().join("\n");
        config
    }

    #[test]
    fn configuration_should_have_a_default_value_for_the_log_level() {
        use crate::Configuration;

        let configuration = Configuration::default();

        assert_eq!(configuration.log_level, Option::from(String::from("info")), "Expected default log level to be: {:?}, got {:?}", Option::from(String::from("info")), configuration.log_level);
    }

    #[test]
    fn configuration_should_be_saved_in_a_toml_config_file() {
        use std::env;
        use crate::Configuration;
        use std::fs;
        use uuid::Uuid;

        // Build temp config file path
        let temp_directory = env::temp_dir();
        let temp_file = temp_directory.join(format!("test_config_{}.toml", Uuid::new_v4()));

        // Convert to argument type for Configuration::save_to_file
        let config_file_path = temp_file.clone();
        let path = config_file_path.to_string_lossy().to_string();

        let default_configuration = Configuration::default();

        default_configuration.save_to_file(&path).expect("Could not save configuration to file");

        let contents = fs::read_to_string(&path).expect("Something went wrong reading the file");

        assert_eq!(contents, default_config_toml());
    }

    #[cfg(test)]
    fn create_temp_config_file_with_default_config()-> String {
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
        use crate::Configuration;

        let config_file_path = create_temp_config_file_with_default_config();

        let configuration = Configuration::load_from_file(&config_file_path).expect("Could not load configuration from file");

        assert_eq!(configuration, Configuration::default());
    }
}