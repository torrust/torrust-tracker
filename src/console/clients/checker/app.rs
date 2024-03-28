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
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, LevelFilter};

use super::config::Configuration;
use super::console::Console;
use super::service::{CheckResult, Service};
use crate::console::clients::checker::config::parse_from_json;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON configuration file.
    #[clap(short, long, env = "TORRUST_CHECKER_CONFIG_PATH")]
    config_path: Option<PathBuf>,

    /// Direct configuration content in JSON.
    #[clap(env = "TORRUST_CHECKER_CONFIG", hide_env_values = true)]
    config_content: Option<String>,
}

/// # Errors
///
/// Will return an error if the configuration was not provided.
pub async fn run() -> Result<Vec<CheckResult>> {
    setup_logging(LevelFilter::Info);

    let args = Args::parse();

    let config = setup_config(args)?;

    let console_printer = Console {};

    let service = Service {
        config: Arc::new(config),
        console: console_printer,
    };

    service.run_checks().await
}

fn setup_logging(level: LevelFilter) {
    if let Err(_err) = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}][{}] {}",
                chrono::Local::now().format("%+"),
                record.target(),
                record.level(),
                message
            ));
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
    {
        panic!("Failed to initialize logging.")
    }

    debug!("logging initialized.");
}

fn setup_config(args: Args) -> Result<Configuration> {
    // If a config is directly supplied, we use it.
    if let Some(config) = args.config_content {
        parse_from_json(&config).context("invalid config format")
    }
    // or we load it from a file...
    else if let Some(path) = args.config_path {
        let file_content = std::fs::read_to_string(path.clone()).with_context(|| format!("can't read config file {path:?}"))?;
        parse_from_json(&file_content).context("invalid config format")
    }
    // but we cannot run without any config...
    else {
        Err(anyhow::anyhow!("no configuration provided"))
    }
}
