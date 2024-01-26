use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};

use log::{debug, info, LevelFilter};

use super::tracker_container::TrackerContainer;
use crate::e2e::docker::RunOptions;
use crate::e2e::logs_parser::RunningServices;
use crate::e2e::temp_dir::Handler;

/* code-review:
     - We use always the same docker image name. Should we use a random image name (tag)?
     - We use the name image name we use in other workflows `torrust-tracker:local`.
       Should we use a different one like `torrust-tracker:e2e`?
     - We remove the container after running tests but not the container image.
       Should we remove the image too?
*/

pub const NUMBER_OF_ARGUMENTS: usize = 2;
const CONTAINER_IMAGE: &str = "torrust-tracker:local";
const CONTAINER_NAME_PREFIX: &str = "tracker_";
const TRACKER_CHECKER_CONFIG_FILE: &str = "tracker_checker.json";

pub struct Arguments {
    pub tracker_config_path: String,
}

/// Script to run E2E tests.
///
/// # Panics
///
/// Will panic if it can't not perform any of the operations.
pub fn run() {
    setup_runner_logging(LevelFilter::Info);

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

    let temp_dir = create_temp_dir();

    let tracker_checker_config_path =
        create_tracker_checker_config_file(&running_services, temp_dir.temp_dir.path(), TRACKER_CHECKER_CONFIG_FILE);

    // todo: inject the configuration with an env variable so that we don't have 
    // to create the temporary directory/file.
    run_tracker_checker(&tracker_checker_config_path).expect("All tracker services should be running correctly");

    // More E2E tests could be added here in the future.
    // For example: `cargo test ...` for only E2E tests, using this shared test env.

    tracker_container.stop();

    tracker_container.remove();

    info!("Tracker container final state:\n{:#?}", tracker_container);
}

fn setup_runner_logging(level: LevelFilter) {
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

fn create_temp_dir() -> Handler {
    debug!(
        "Current dir: {:?}",
        env::current_dir().expect("It should return the current dir")
    );

    let temp_dir_handler = Handler::new().expect("A temp dir should be created");

    info!("Temp dir created: {:?}", temp_dir_handler.temp_dir);

    temp_dir_handler
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

fn create_tracker_checker_config_file(running_services: &RunningServices, config_path: &Path, config_name: &str) -> PathBuf {
    let tracker_checker_config =
        serde_json::to_string_pretty(&running_services).expect("Running services should be serialized into JSON");

    let mut tracker_checker_config_path = PathBuf::from(&config_path);
    tracker_checker_config_path.push(config_name);

    write_tracker_checker_config_file(&tracker_checker_config_path, &tracker_checker_config);

    tracker_checker_config_path
}

fn write_tracker_checker_config_file(config_file_path: &Path, config: &str) {
    info!(
        "Writing Tracker Checker configuration file: {:?} \n{config}",
        config_file_path
    );

    let mut file = File::create(config_file_path).expect("Tracker checker config file to be created");

    file.write_all(config.as_bytes())
        .expect("Tracker checker config file to be written");
}

/// Runs the Tracker Checker.
///
/// For example:
///
/// ```text
/// cargo run --bin tracker_checker "./share/default/config/tracker_checker.json"
/// ```
///
/// # Errors
///
/// Will return an error if the tracker checker fails.
///
/// # Panics
///
/// Will panic if the config path is not a valid string.
pub fn run_tracker_checker(config_path: &Path) -> io::Result<()> {
    info!(
        "Running Tracker Checker: cargo --bin tracker_checker {}",
        config_path.display()
    );

    let path = config_path.to_str().expect("The path should be a valid string");

    let status = Command::new("cargo")
        .args(["run", "--bin", "tracker_checker", path])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to run Tracker Checker with config file {path}"),
        ))
    }
}
