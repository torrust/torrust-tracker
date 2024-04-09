//! Program to run E2E tests.
//!
//! ```text
//! cargo run --bin e2e_tests_runner share/default/config/tracker.e2e.container.sqlite3.toml
//! ```
use tracing::{debug, info, Level};

use super::tracker_container::TrackerContainer;
use crate::console::ci::e2e::docker::RunOptions;
use crate::console::ci::e2e::trace_parser::RunningServices;
use crate::console::ci::e2e::tracker_checker::{self};

/* code-review:
     - We use always the same docker image name. Should we use a random image name (tag)?
     - We use the name image name we use in other workflows `torrust-tracker:local`.
       Should we use a different one like `torrust-tracker:e2e`?
     - We remove the container after running tests but not the container image.
       Should we remove the image too?
*/

const NUMBER_OF_ARGUMENTS: usize = 2;
const CONTAINER_IMAGE: &str = "torrust-tracker:local";
const CONTAINER_NAME_PREFIX: &str = "tracker_";

pub struct Arguments {
    pub tracker_config_path: String,
}

/// Script to run E2E tests.
///
/// # Panics
///
/// Will panic if it can't not perform any of the operations.
pub fn run() {
    setup_runner_tracing(Level::INFO);

    let args = parse_arguments();

    let tracker_config = load_tracker_configuration(&args.tracker_config_path);

    let mut tracker_container = TrackerContainer::new(CONTAINER_IMAGE, CONTAINER_NAME_PREFIX);

    tracker_container.build_image();

    // code-review: if we want to use port 0 we don't know which ports we have to open.
    // Besides, if we don't use port 0 we should get the port numbers from the tracker configuration.
    // We could not use docker, but the intention was to create E2E tests including containerization.
    let options = RunOptions {
        env_vars: vec![("TORRUST_TRACKER_CONFIG".to_string(), tracker_config.to_string())],
        ports: vec![
            "6969:6969/udp".to_string(),
            "7070:7070/tcp".to_string(),
            "1212:1212/tcp".to_string(),
            "1313:1313/tcp".to_string(),
        ],
    };

    tracker_container.run(&options);

    let running_services = tracker_container.running_services();

    assert_there_is_at_least_one_service_per_type(&running_services);

    let tracker_checker_config =
        serde_json::to_string_pretty(&running_services).expect("Running services should be serialized into JSON");

    tracker_checker::run(&tracker_checker_config).expect("All tracker services should be running correctly");

    // More E2E tests could be added here in the future.
    // For example: `cargo test ...` for only E2E tests, using this shared test env.

    tracker_container.stop();

    tracker_container.remove();

    info!("Tracker container final state:\n{:#?}", tracker_container);
}

fn setup_runner_tracing(level: Level) {
    let () = tracing_subscriber::fmt().pretty().with_max_level(level).init();

    debug!("tracing initialized.");
}

fn parse_arguments() -> Arguments {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < NUMBER_OF_ARGUMENTS {
        eprintln!("Usage:       cargo run --bin e2e_tests_runner <PATH_TO_TRACKER_CONFIG_FILE>");
        eprintln!("For example: cargo run --bin e2e_tests_runner ./share/default/config/tracker.e2e.container.sqlite3.toml");
        std::process::exit(1);
    }

    let config_path = &args[1];

    Arguments {
        tracker_config_path: config_path.to_string(),
    }
}

fn load_tracker_configuration(tracker_config_path: &str) -> String {
    info!("Reading tracker configuration from file: {} ...", tracker_config_path);
    read_file(tracker_config_path)
}

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Can't read file {path}"))
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
