pub use crate::tracker::TrackerMode;
use serde::Deserialize;
use std;
use std::collections::HashMap;
use toml;
use std::net::{IpAddr};

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct HttpTrackerConfig {
    bind_address: String,
    announce_interval: u32,
    ssl_enabled: bool,
    pub ssl_cert_path: Option<String>,
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Configuration {
    mode: TrackerMode,
    udp_tracker: UdpTrackerConfig,
    http_tracker: Option<HttpTrackerConfig>,
    http_api: Option<HttpApiConfig>,
    log_level: Option<String>,
    db_path: Option<String>,
    cleanup_interval: Option<u64>,
    external_ip: IpAddr,
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
                    Ok(cfg) => {
                        eprintln!("Manually set external IP to: {}", cfg.get_ext_ip());
                        Ok(cfg)
                    },
                    Err(e) => Err(ConfigError::ParseError(e)),
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

    pub fn get_db_path(&self) -> &Option<String> {
        &self.db_path
    }

    pub fn get_cleanup_interval(&self) -> Option<u64> {
        self.cleanup_interval
    }

    pub fn get_ext_ip(&self) -> IpAddr { self.external_ip }
}

impl Configuration {
    pub async fn default() -> Self {
        let external_ip = external_ip::get_ip().await.unwrap();

        eprintln!("external ip: {:?}", external_ip);

        Configuration {
            log_level: Option::from(String::from("trace")),
            mode: TrackerMode::PrivateMode,
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
            db_path: None,
            cleanup_interval: None,
            external_ip,
        }
    }
}
