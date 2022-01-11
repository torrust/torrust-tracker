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

#[derive(Serialize, Deserialize)]
pub struct UdpTrackerConfig {
    bind_address: String,
    announce_interval: u32,
}

impl UdpTrackerConfig {
    pub fn get_address(&self) -> &str {
        self.bind_address.as_str()
    }

    pub fn get_announce_interval(&self) -> u32 {
        self.announce_interval
    }
}

#[derive(Serialize, Deserialize)]
pub struct HttpTrackerConfig {
    bind_address: String,
    announce_interval: u32,
    ssl_enabled: bool,
    #[serde(serialize_with = "none_as_empty_string")]
    pub ssl_cert_path: Option<String>,
    #[serde(serialize_with = "none_as_empty_string")]
    pub ssl_key_path: Option<String>
}

impl HttpTrackerConfig {
    pub fn get_address(&self) -> &str {
        self.bind_address.as_str()
    }

    pub fn get_announce_interval(&self) -> u32 {
        self.announce_interval
    }

    pub fn is_ssl_enabled(&self) -> bool {
        self.ssl_enabled && self.ssl_cert_path.is_some() && self.ssl_key_path.is_some()
    }
}

#[derive(Serialize, Deserialize)]
pub struct HttpApiConfig {
    bind_address: String,
    access_tokens: HashMap<String, String>,
}

impl HttpApiConfig {
    pub fn get_address(&self) -> &str {
        self.bind_address.as_str()
    }

    pub fn get_access_tokens(&self) -> &HashMap<String, String> {
        &self.access_tokens
    }
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    log_level: Option<String>,
    mode: TrackerMode,
    db_path: String,
    cleanup_interval: Option<u64>,
    external_ip: Option<String>,
    udp_tracker: UdpTrackerConfig,
    http_tracker: Option<HttpTrackerConfig>,
    http_api: Option<HttpApiConfig>,
}

#[derive(Debug)]
pub enum ConfigurationError {
    IOError(std::io::Error),
    ParseError(toml::de::Error),
}

impl std::fmt::Display for ConfigurationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigurationError::IOError(e) => e.fmt(formatter),
            ConfigurationError::ParseError(e) => e.fmt(formatter),
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

    pub fn get_mode(&self) -> &TrackerMode {
        &self.mode
    }

    pub fn get_log_level(&self) -> &Option<String> {
        &self.log_level
    }

    pub fn get_udp_tracker_config(&self) -> &UdpTrackerConfig {
        &self.udp_tracker
    }

    pub fn get_http_tracker_config(&self) -> Option<&HttpTrackerConfig> {
        self.http_tracker.as_ref()
    }

    pub fn get_http_api_config(&self) -> Option<&HttpApiConfig> {
        self.http_api.as_ref()
    }

    pub fn get_db_path(&self) -> &str {
        &self.db_path
    }

    pub fn get_cleanup_interval(&self) -> Option<u64> {
        self.cleanup_interval
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
            http_tracker: Option::from(HttpTrackerConfig {
                bind_address: String::from("0.0.0.0:7878"),
                announce_interval: 120,
                ssl_enabled: false,
                ssl_cert_path: None,
                ssl_key_path: None
            }),
            http_api: Option::from(HttpApiConfig {
                bind_address: String::from("127.0.0.1:1212"),
                access_tokens: [(String::from("someone"), String::from("MyAccessToken"))].iter().cloned().collect(),
            }),
        }
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

        match config.try_into() {
            Ok(data) => Ok(data),
            Err(e) => Err(ConfigError::Message(format!("Errors while processing config: {}.", e))),
        }
    }

    pub fn save_to_file(&self) -> Result<(), ()>{
        let toml_string = toml::to_string(self).expect("Could not encode TOML value");
        fs::write("config.toml", toml_string).expect("Could not write to file!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Configuration;

    #[test]
    fn save_to_file() {
        let config = Configuration::default();
        let test = config.save_to_file();
        assert!(test.is_ok());
    }
}
