use std;
use std::collections::HashMap;
use toml;
use serde;
pub use tracker::TrackerMode;

#[derive(Deserialize)]
pub struct UDPConfig {
    bind_address: String,
    mode: TrackerMode,
}

#[derive(Deserialize)]
pub struct HTTPConfig {
    bind_address: String,
    access_tokens: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct Configuration {
    udp: UDPConfig,
    http: Option<HTTPConfig>,
}

#[derive(Debug)]
pub enum ConfigError {
    IOError(std::io::Error),
    ParseError(toml::de::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Display;
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
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration{
            udp: UDPConfig{
                bind_address: String::from("0.0.0.0:6969"),
                mode: TrackerMode::DynamicMode,
            },
            http: None,
        }
    }
}