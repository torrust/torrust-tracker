pub use crate::tracker::TrackerMode;
use serde::{Serialize, Deserialize, Serializer};
use std;
use std::collections::HashMap;
use std::fs;
use toml;
use std::net::{IpAddr};
use std::path::Path;
use std::str::FromStr;
use config::{ConfigError, Config, File};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum TrackerServer {
    UDP,
    HTTP
}

#[derive(Serialize, Deserialize)]
pub struct UdpTrackerConfig {
    pub bind_address: String,
    pub announce_interval: u32,
}

#[derive(Serialize, Deserialize)]
pub struct HttpTrackerConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub on_reverse_proxy: bool,
    pub announce_interval: u32,
    pub ssl_enabled: bool,
    #[serde(serialize_with = "none_as_empty_string")]
    pub ssl_cert_path: Option<String>,
    #[serde(serialize_with = "none_as_empty_string")]
    pub ssl_key_path: Option<String>
}

impl HttpTrackerConfig {
    pub fn is_ssl_enabled(&self) -> bool {
        self.ssl_enabled && self.ssl_cert_path.is_some() && self.ssl_key_path.is_some()
    }
}

#[derive(Serialize, Deserialize)]
pub struct HttpApiConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub access_tokens: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub log_level: Option<String>,
    pub mode: TrackerMode,
    pub db_path: String,
    pub cleanup_interval: Option<u64>,
    pub external_ip: Option<String>,
    pub udp_tracker: UdpTrackerConfig,
    pub http_tracker: HttpTrackerConfig,
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

pub fn none_as_empty_string<T, S>(option: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
{
    if let Some(value) = option {
        value.serialize(serializer)
    } else {
        "".serialize(serializer)
    }
}

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
                    },
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
        Configuration {
            log_level: Option::from(String::from("info")),
            mode: TrackerMode::PublicMode,
            db_path: String::from("data.db"),
            cleanup_interval: Some(600),
            external_ip: Some(String::from("0.0.0.0")),
            udp_tracker: UdpTrackerConfig {
                bind_address: String::from("0.0.0.0:6969"),
                announce_interval: 120,
            },
            http_tracker: HttpTrackerConfig {
                enabled: false,
                bind_address: String::from("0.0.0.0:6969"),
                on_reverse_proxy: false,
                announce_interval: 120,
                ssl_enabled: false,
                ssl_cert_path: None,
                ssl_key_path: None
            },
            http_api: HttpApiConfig {
                enabled: true,
                bind_address: String::from("127.0.0.1:1212"),
                access_tokens: [(String::from("admin"), String::from("MyAccessToken"))].iter().cloned().collect(),
            },
        }
    }

    pub fn verify(&self) -> Result<(), ConfigurationError> {
        // UDP is not secure for sending private keys
        if (self.mode == TrackerMode::PrivateMode || self.mode == TrackerMode::PrivateListedMode) && self.get_tracker_server() == TrackerServer::UDP {
            return Err(ConfigurationError::TrackerModeIncompatible)
        }

        Ok(())
    }

    pub fn load_from_file() -> Result<Configuration, ConfigError> {
        let mut config = Config::new();

        const CONFIG_PATH: &str = "config.toml";

        if Path::new(CONFIG_PATH).exists() {
            config.merge(File::with_name(CONFIG_PATH))?;
        } else {
            eprintln!("No config file found.");
            eprintln!("Creating config file..");
            let config = Configuration::default();
            let _ = config.save_to_file();
            return Err(ConfigError::Message(format!("Please edit the config.TOML in the root folder and restart the tracker.")))
        }

        let torrust_config: Configuration = config.try_into().map_err(|e| ConfigError::Message(format!("Errors while processing config: {}.", e)))?;

        match torrust_config.verify() {
            Ok(_) => Ok(torrust_config),
            Err(e) => Err(ConfigError::Message(format!("Errors while processing config: {}.", e)))
        }
    }

    pub fn save_to_file(&self) -> Result<(), ()>{
        let toml_string = toml::to_string(self).expect("Could not encode TOML value");
        fs::write("config.toml", toml_string).expect("Could not write to file!");
        Ok(())
    }

    pub fn get_tracker_server(&self) -> TrackerServer {
        if self.http_tracker.enabled {
            TrackerServer::HTTP
        } else {
            TrackerServer::UDP
        }
    }
}
