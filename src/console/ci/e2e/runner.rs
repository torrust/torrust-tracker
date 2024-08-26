//! Program to run E2E tests.
//!
//! You can execute it with (passing a TOML config file path):
//!
//! ```text
//! cargo run --bin e2e_tests_runner -- --config-toml-path "./share/default/config/tracker.e2e.container.sqlite3.toml"
//! ```
//!
//! Or:
//!
//! ```text
//! TORRUST_TRACKER_CONFIG_TOML_PATH="./share/default/config/tracker.e2e.container.sqlite3.toml" cargo run --bin e2e_tests_runner"
//! ```
//!
//! You can execute it with (directly passing TOML config):
//!
//! ```text
//! TORRUST_TRACKER_CONFIG_TOML=$(cat "./share/default/config/tracker.e2e.container.sqlite3.toml") cargo run --bin e2e_tests_runner
//! ```
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use tracing::level_filters::LevelFilter;

use super::tracker_container::TrackerContainer;
use crate::console::ci::e2e::docker::RunOptions;
use crate::console::ci::e2e::logs_parser::RunningServices;
use crate::console::ci::e2e::tracker_checker::{self};

/* code-review:
     - We use always the same docker image name. Should we use a random image name (tag)?
     - We use the name image name we use in other workflows `torrust-tracker:local`.
       Should we use a different one like `torrust-tracker:e2e`?
     - We remove the container after running tests but not the container image.
       Should we remove the image too?
*/

const CONTAINER_IMAGE: &str = "torrust-tracker:local";
const CONTAINER_NAME_PREFIX: &str = "tracker_";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON configuration file.
    #[clap(short, long, env = "TORRUST_TRACKER_CONFIG_TOML_PATH")]
    config_toml_path: Option<PathBuf>,

    /// Direct configuration content in JSON.
    #[clap(env = "TORRUST_TRACKER_CONFIG_TOML", hide_env_values = true)]
    config_toml: Option<String>,
}

/// Script to run E2E tests.
///
/// # Errors
///
/// Will return an error if it can't load the tracker configuration from arguments.
///
/// # Panics
///
/// Will panic if it can't not perform any of the operations.
pub fn run() -> anyhow::Result<()> {
    tracing_stdout_init(LevelFilter::INFO);

    let args = Args::parse();

    let tracker_config = load_tracker_configuration(&args)?;

    tracing::info!("tracker config:\n{tracker_config}");

    let mut tracker_container = TrackerContainer::new(CONTAINER_IMAGE, CONTAINER_NAME_PREFIX);

    tracker_container.build_image();

    // code-review: if we want to use port 0 we don't know which ports we have to open.
    // Besides, if we don't use port 0 we should get the port numbers from the tracker configuration.
    // We could not use docker, but the intention was to create E2E tests including containerization.
    let options = RunOptions {
        env_vars: vec![("TORRUST_TRACKER_CONFIG_TOML".to_string(), tracker_config.to_string())],
        ports: vec![
            "6969:6969/udp".to_string(),
            "7070:7070/tcp".to_string(),
            "1212:1212/tcp".to_string(),
            "1313:1313/tcp".to_string(),
        ],
    };

    tracker_container.run(&options);

    let running_services = tracker_container.running_services();

    tracing::info!(
        "Running services:\n {}",
        serde_json::to_string_pretty(&running_services).expect("running services to be serializable to JSON")
    );

    assert_there_is_at_least_one_service_per_type(&running_services);

    let tracker_checker_config =
        serde_json::to_string_pretty(&running_services).expect("Running services should be serialized into JSON");

    tracker_checker::run(&tracker_checker_config).expect("All tracker services should be running correctly");

    // More E2E tests could be added here in the future.
    // For example: `cargo test ...` for only E2E tests, using this shared test env.

    tracker_container.stop();

    tracker_container.remove();

    tracing::info!("Tracker container final state:\n{:#?}", tracker_container);

    Ok(())
}

fn tracing_stdout_init(filter: LevelFilter) {
    tracing_subscriber::fmt().with_max_level(filter).init();
    tracing::info!("Logging initialized");
}

fn load_tracker_configuration(args: &Args) -> anyhow::Result<String> {
    match (args.config_toml_path.clone(), args.config_toml.clone()) {
        (Some(config_path), _) => {
            tracing::info!(
                "Reading tracker configuration from file: {} ...",
                config_path.to_string_lossy()
            );
            load_config_from_file(&config_path)
        }
        (_, Some(config_content)) => {
            tracing::info!("Reading tracker configuration from env var ...");
            Ok(config_content)
        }
        _ => Err(anyhow::anyhow!("No configuration provided")),
    }
}

fn load_config_from_file(path: &PathBuf) -> anyhow::Result<String> {
    let config = std::fs::read_to_string(path).with_context(|| format!("CSan't read config file {path:?}"))?;

    Ok(config)
}

fn assert_there_is_at_least_one_service_per_type(running_services: &RunningServices) {
    assert!(
        !running_services.udp_trackers.is_empty(),
        "At least one UDP tracker should be enabled in E2E tests configuration"
    );
    assert!(
        !running_services.http_trackers.is_empty(),
        "At least one HTTP tracker should be enabled in E2E tests configuration"
    );
    assert!(
        !running_services.health_checks.is_empty(),
        "At least one Health Check should be enabled in E2E tests configuration"
    );
}
