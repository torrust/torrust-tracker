pub use crate::tracker::TrackerMode;
use serde::Deserialize;
use std;
use std::collections::HashMap;
use toml;

#[derive(Deserialize)]
pub struct UDPConfig {
    bind_address: String,
    announce_interval: u32,
}

impl UDPConfig {
    pub fn get_address(&self) -> &str {
        self.bind_address.as_str()
    }

    pub fn get_announce_interval(&self) -> u32 {
        self.announce_interval
    }
}

#[derive(Deserialize)]
pub struct HTTPConfig {
    bind_address: String,
    access_tokens: HashMap<String, String>,
}

impl HTTPConfig {
    pub fn get_address(&self) -> &str {
        self.bind_address.as_str()
    }

    pub fn get_access_tokens(&self) -> &HashMap<String, String> {
        &self.access_tokens
    }
}

#[derive(Deserialize)]
pub struct Configuration {
    mode: TrackerMode,
    udp: UDPConfig,
    http: Option<HTTPConfig>,
    log_level: Option<String>,
    db_path: Option<String>,
    cleanup_interval: Option<u64>,
}

#[derive(Debug)]
pub enum ConfigError {
    IOError(std::io::Error),
    ParseError(toml::de::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::IOError(e) => e.fmt(formatter),
            ConfigError::ParseError(e) => e.fmt(formatter),
        }
    }
}
impl std::error::Error for ConfigError {}

impl Configuration {
    pub fn load(data: &[u8]) -> Result<Configuration, toml::de::Error> {
        toml::from_slice(data)
    }

    pub fn load_file(path: &str) -> Result<Configuration, ConfigError> {
        match std::fs::read(path) {
            Err(e) => Err(ConfigError::IOError(e)),
            Ok(data) => {
                match Self::load(data.as_slice()) {
                    Ok(cfg) => Ok(cfg),
                    Err(e) => Err(ConfigError::ParseError(e)),
                }
            }
        }
    }

    pub fn get_mode(&self) -> &TrackerMode {
        &self.mode
    }

    pub fn get_udp_config(&self) -> &UDPConfig {
        &self.udp
    }

    pub fn get_log_level(&self) -> &Option<String> {
        &self.log_level
    }

    pub fn get_http_config(&self) -> Option<&HTTPConfig> {
        self.http.as_ref()
    }

    pub fn get_db_path(&self) -> &Option<String> {
        &self.db_path
    }

    pub fn get_cleanup_interval(&self) -> Option<u64> {
        self.cleanup_interval
    }
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            log_level: None,
            mode: TrackerMode::DynamicMode,
            udp: UDPConfig {
                announce_interval: 120,
                bind_address: String::from("0.0.0.0:6969"),
            },
            http: None,
            db_path: None,
            cleanup_interval: None,
        }
    }
}
