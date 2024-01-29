use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use clap::Parser;

use super::config::Configuration;
use super::console::Console;
use super::service::{CheckResult, Service};
use crate::checker::config::parse_from_json;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    config_path: PathBuf,
}

/// # Errors
///
/// Will return an error if it can't read or parse the configuration file.
pub async fn run() -> anyhow::Result<Vec<CheckResult>> {
    let args = Args::parse();

    let config = setup_config(&args)?;

    let console_printer = Console {};

    let service = Service {
        config: Arc::new(config),
        console: console_printer,
    };

    Ok(service.run_checks().await)
}

fn setup_config(args: &Args) -> anyhow::Result<Configuration> {
    let file_content =
        std::fs::read_to_string(&args.config_path).with_context(|| format!("can't read config file {:?}", args.config_path))?;

    parse_from_json(&file_content).context("invalid config format")
}
