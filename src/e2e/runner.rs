use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use std::{env, io};

use log::{debug, info, LevelFilter};
use rand::distributions::Alphanumeric;
use rand::Rng;

use super::docker::RunningContainer;
use crate::e2e::docker::{Docker, RunOptions};
use crate::e2e::logs_parser::RunningServices;
use crate::e2e::temp_dir::Handler;

pub const NUMBER_OF_ARGUMENTS: usize = 2;
const CONTAINER_TAG: &str = "torrust-tracker:local";
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

    build_tracker_container_image(CONTAINER_TAG);

    let temp_dir = create_temp_dir();

    let container_name = generate_random_container_name("tracker_");

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

    let container = run_tracker_container(CONTAINER_TAG, &container_name, &options);

    let running_services = parse_running_services_from_logs(&container);

    let tracker_checker_config =
        serde_json::to_string_pretty(&running_services).expect("Running services should be serialized into JSON");

    let mut tracker_checker_config_path = PathBuf::from(&temp_dir.temp_dir.path());
    tracker_checker_config_path.push(TRACKER_CHECKER_CONFIG_FILE);

    write_tracker_checker_config_file(&tracker_checker_config_path, &tracker_checker_config);

    run_tracker_checker(&tracker_checker_config_path).expect("Tracker checker should check running services");

    // More E2E tests could be executed here in the future. For example: `cargo test ...`.

    info!("Running container `{}` will be automatically removed", container.name);
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

fn build_tracker_container_image(tag: &str) {
    info!("Building tracker container image with tag: {} ...", tag);
    Docker::build("./Containerfile", tag).expect("A tracker local docker image should be built");
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

fn generate_random_container_name(prefix: &str) -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();

    format!("{prefix}{rand_string}")
}

fn run_tracker_container(image: &str, container_name: &str, options: &RunOptions) -> RunningContainer {
    info!("Running docker tracker image: {container_name} ...");

    let container = Docker::run(image, container_name, options).expect("A tracker local docker image should be running");

    info!("Waiting for the container {container_name} to be healthy ...");

    let is_healthy = Docker::wait_until_is_healthy(container_name, Duration::from_secs(10));

    assert!(is_healthy, "Unhealthy tracker container: {container_name}");

    debug!("Container {container_name} is healthy ...");

    container
}

fn parse_running_services_from_logs(container: &RunningContainer) -> RunningServices {
    let logs = Docker::logs(&container.name).expect("Logs should be captured from running container");

    debug!("Logs after starting the container:\n{logs}");

    RunningServices::parse_from_logs(&logs)
}

fn write_tracker_checker_config_file(config_file_path: &Path, config: &str) {
    let mut file = File::create(config_file_path).expect("Tracker checker config file to be created");

    file.write_all(config.as_bytes())
        .expect("Tracker checker config file to be written");

    info!("Tracker checker configuration file: {:?} \n{config}", config_file_path);
}

/// Runs the tracker checker
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
        "Running tacker checker: cargo --bin tracker_checker {}",
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
            format!("Failed to run tracker checker with config file {path}"),
        ))
    }
}
