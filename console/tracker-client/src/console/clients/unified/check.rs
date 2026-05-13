use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use bittorrent_primitives::info_hash::InfoHash as TorrustInfoHash;
use clap::{Parser, Subcommand};
use futures::FutureExt as _;
use serde::Serialize;
use tokio::task::JoinSet;
use torrust_tracker_configuration::DEFAULT_TIMEOUT;
use url::Url;

use super::app::OutputFormat;
use crate::console::clients::checker::checks::{health, http, udp};
use crate::console::clients::checker::config::{parse_from_json, Configuration};
use crate::console::clients::checker::error::{AppError, ConfigSource};
use crate::console::clients::checker::monitor::udp::{run_monitor, MonitorUdpConfig, DEFAULT_INFO_HASH};

#[derive(Debug, Clone, Serialize)]
enum CheckResult {
    Udp(Result<udp::Checks, udp::Checks>),
    Http(Result<http::Checks, http::Checks>),
    Health(Result<health::Checks, health::Checks>),
}

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
/// Returns `AppError` for configuration or runtime failures.
pub async fn run(raw_args: Vec<String>, output_format: OutputFormat) -> Result<(), AppError> {
    let args = parse_args(raw_args)?;

    if let Some(command) = args.command {
        return run_command(command).await;
    }

    let config = setup_config(args)?;
    run_checks(Arc::new(config), output_format).await
}

fn parse_args(raw_args: Vec<String>) -> Result<Args, AppError> {
    let mut argv = vec!["tracker_client-check".to_string()];
    argv.extend(raw_args);

    // Let clap handle parse errors directly: it prints the message to stderr
    // and exits with code 2 for usage errors, preserving the CLI I/O contract.
    Args::try_parse_from(argv).map_err(|e| e.exit())
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

async fn run_checks(config: Arc<Configuration>, output_format: OutputFormat) -> Result<(), AppError> {
    let mut check_results = Vec::default();

    let mut checks = JoinSet::new();
    checks.spawn(
        udp::run(config.udp_trackers.clone(), DEFAULT_TIMEOUT).map(|mut f| f.drain(..).map(CheckResult::Udp).collect::<Vec<_>>()),
    );
    checks.spawn(
        http::run(config.http_trackers.clone(), DEFAULT_TIMEOUT)
            .map(|mut f| f.drain(..).map(CheckResult::Http).collect::<Vec<_>>()),
    );
    checks.spawn(
        health::run(config.health_checks.clone(), DEFAULT_TIMEOUT)
            .map(|mut f| f.drain(..).map(CheckResult::Health).collect::<Vec<_>>()),
    );

    while let Some(results) = checks.join_next().await {
        check_results.append(&mut results.map_err(|error| AppError::Runtime(error.to_string()))?);
    }

    let json_output = serde_json::json!(check_results);
    let rendered = if output_format.is_pretty() {
        serde_json::to_string_pretty(&json_output)
    } else {
        serde_json::to_string(&json_output)
    }
    .map_err(|e| AppError::Runtime(format!("failed to render check output as JSON: {e}")))?;

    println!("{rendered}");

    Ok(())
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
