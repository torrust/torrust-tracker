use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use clap::Parser;

use super::config::Configuration;
use super::console::Console;
use super::service::{CheckResult, Service};
use crate::checker::config::parse_from_json;

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
    let args = Args::parse();

    let config = setup_config(args)?;

    let console_printer = Console {};

    let service = Service {
        config: Arc::new(config),
        console: console_printer,
    };

    Ok(service.run_checks().await)
}

fn setup_config(args: Args) -> Result<Configuration> {
    match (args.config_path, args.config_content) {
        (Some(config_path), _) => load_config_from_file(&config_path),
        (_, Some(config_content)) => parse_from_json(&config_content).context("invalid config format"),
        _ => Err(anyhow::anyhow!("no configuration provided")),
    }
}

fn load_config_from_file(path: &PathBuf) -> Result<Configuration> {
    let file_content = std::fs::read_to_string(path).with_context(|| format!("can't read config file {path:?}"))?;

    parse_from_json(&file_content).context("invalid config format")
}
