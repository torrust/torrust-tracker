//! Program to run checks against running trackers.
//!
//! Run providing a config file path:
//!
//! ```text
//! cargo run --bin tracker_checker -- --config-path "./share/default/config/tracker_checker.json"
//! TORRUST_CHECKER_CONFIG_PATH="./share/default/config/tracker_checker.json" cargo run --bin tracker_checker
//! ```
//!
//! Run providing the configuration:
//!
//! ```text
//! TORRUST_CHECKER_CONFIG=$(cat "./share/default/config/tracker_checker.json") cargo run --bin tracker_checker
//! ```
//!
//! Another real example to test the Torrust demo tracker:
//!
//! ```text
//! TORRUST_CHECKER_CONFIG='{
//!     "udp_trackers": ["144.126.245.19:6969"],
//!     "http_trackers": ["https://tracker.torrust-demo.com"],
//!     "health_checks": ["https://tracker.torrust-demo.com/api/health_check"]
//! }' cargo run --bin tracker_checker
//! ```
//!
//! The output should be something like the following:
//!
//! ```json
//! {
//!   "udp_trackers": [
//!     {
//!       "url": "144.126.245.19:6969",
//!       "status": {
//!         "code": "ok",
//!         "message": ""
//!       }
//!     }
//!   ],
//!   "http_trackers": [
//!     {
//!       "url": "https://tracker.torrust-demo.com/",
//!       "status": {
//!         "code": "ok",
//!         "message": ""
//!       }
//!     }
//!   ],
//!   "health_checks": [
//!     {
//!       "url": "https://tracker.torrust-demo.com/api/health_check",
//!       "status": {
//!         "code": "ok",
//!         "message": ""
//!       }
//!     }
//!   ]
//! }
//! ```
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use bittorrent_primitives::info_hash::InfoHash as TorrustInfoHash;
use clap::{Parser, Subcommand};
use tracing::level_filters::LevelFilter;
use url::Url;

use super::config::Configuration;
use super::console::Console;
use super::error::{AppError, ConfigSource};
use super::monitor::udp::{run_monitor, MonitorUdpConfig, DEFAULT_INFO_HASH};
use super::service::Service;
use crate::console::clients::checker::config::parse_from_json;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    /// Path to the JSON configuration file.
    #[clap(short, long, env = "TORRUST_CHECKER_CONFIG_PATH")]
    config_path: Option<PathBuf>,

    /// Direct configuration content in JSON.
    #[clap(env = "TORRUST_CHECKER_CONFIG", hide_env_values = true)]
    config_content: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run periodic monitor checks.
    Monitor {
        #[command(subcommand)]
        protocol: MonitorProtocol,
    },
}

#[derive(Subcommand, Debug)]
enum MonitorProtocol {
    /// Monitor a UDP tracker using announce probes.
    Udp {
        /// UDP tracker URL.
        #[arg(long, value_parser = parse_udp_url)]
        url: Url,

        /// Seconds between probes.
        #[arg(long, default_value_t = 300, value_parser = clap::value_parser!(u64).range(1..))]
        interval: u64,

        /// Probe timeout in seconds.
        #[arg(long, default_value_t = 10, value_parser = clap::value_parser!(u64).range(1..))]
        timeout: u64,

        /// Total monitor runtime in seconds.
        #[arg(long, default_value_t = 86_400, value_parser = clap::value_parser!(u64).range(1..))]
        duration: u64,

        /// Info-hash used in announce requests.
        #[arg(long, default_value = DEFAULT_INFO_HASH, value_parser = parse_info_hash)]
        info_hash: TorrustInfoHash,
    },
}

/// # Errors
///
/// Will return an `AppError::InvalidConfig` if the configuration cannot be parsed,
/// or an `AppError::Runtime` if the checks fail to execute.
pub async fn run() -> Result<(), AppError> {
    tracing_stdout_init(LevelFilter::INFO);

    let args = Args::parse();

    if let Some(command) = args.command {
        return run_command(command).await;
    }

    let config = setup_config(args)?;

    let console_printer = Console {};

    let service = Service {
        config: Arc::new(config),
        console: console_printer,
    };

    service
        .run_checks()
        .await
        .map_err(|e| AppError::Runtime(e.to_string()))
        .map(|_results| ())
}

fn tracing_stdout_init(filter: LevelFilter) {
    tracing_subscriber::fmt().with_max_level(filter).init();
    tracing::debug!("Logging initialized");
}

fn setup_config(args: Args) -> Result<Configuration, AppError> {
    match (args.config_path, args.config_content) {
        (Some(config_path), _) => load_config_from_file(&config_path),
        (_, Some(config_content)) => parse_from_json(&config_content).map_err(|e| AppError::InvalidConfig {
            source: ConfigSource::EnvVar("TORRUST_CHECKER_CONFIG"),
            message: e.to_string(),
        }),
        _ => Err(AppError::InvalidConfig {
            source: ConfigSource::EnvVar("TORRUST_CHECKER_CONFIG"),
            message: "no configuration provided".to_string(),
        }),
    }
}

fn load_config_from_file(path: &PathBuf) -> Result<Configuration, AppError> {
    let file_content = std::fs::read_to_string(path).map_err(|e| AppError::InvalidConfig {
        source: ConfigSource::File(path.clone()),
        message: format!("can't read config file {}: {e}", path.display()),
    })?;

    parse_from_json(&file_content).map_err(|e| AppError::InvalidConfig {
        source: ConfigSource::File(path.clone()),
        message: e.to_string(),
    })
}

async fn run_command(command: Command) -> Result<(), AppError> {
    match command {
        Command::Monitor {
            protocol:
                MonitorProtocol::Udp {
                    url,
                    interval,
                    timeout,
                    duration,
                    info_hash,
                },
        } => {
            let config = MonitorUdpConfig {
                url,
                interval: Duration::from_secs(interval),
                timeout: Duration::from_secs(timeout),
                duration: Duration::from_secs(duration),
                info_hash,
            };

            run_monitor(config)
                .await
                .map_err(|e| AppError::Runtime(format!("udp monitor failed: {e}")))
        }
    }
}

fn parse_udp_url(url_str: &str) -> Result<Url, String> {
    let url = Url::parse(url_str).map_err(|e| format!("invalid URL: {e}"))?;

    if url.scheme() != "udp" {
        return Err("URL scheme must be udp".to_string());
    }

    if url.port().is_none() {
        return Err("URL must include an explicit port".to_string());
    }

    Ok(url)
}

fn parse_info_hash(info_hash_str: &str) -> Result<TorrustInfoHash, String> {
    TorrustInfoHash::from_str(info_hash_str).map_err(|e| format!("failed to parse info-hash `{info_hash_str}`: {e:?}"))
}
