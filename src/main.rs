use std::env;
use std::sync::Arc;

use log::info;
use torrust_tracker::stats::setup_statistics;
use torrust_tracker::{ephemeral_instance_keys, logging, setup, static_time, tracker};
use torrust_tracker_configuration::Configuration;

#[tokio::main]
async fn main() {
    const CONFIG_PATH: &str = "./config.toml";
    const CONFIG_ENV_VAR_NAME: &str = "TORRUST_TRACKER_CONFIG";

    // Set the time of Torrust app starting
    lazy_static::initialize(&static_time::TIME_AT_APP_START);

    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

    // Initialize Torrust config
    let config = if env::var(CONFIG_ENV_VAR_NAME).is_ok() {
        println!("Loading configuration from env var {CONFIG_ENV_VAR_NAME}");
        Arc::new(Configuration::load_from_env_var(CONFIG_ENV_VAR_NAME).unwrap())
    } else {
        println!("Loading configuration from config file {CONFIG_PATH}");
        Arc::new(Configuration::load_from_file(CONFIG_PATH).unwrap())
    };

    // Initialize statistics
    let (stats_event_sender, stats_repository) = setup_statistics(config.tracker_usage_statistics);

    // Initialize Torrust tracker
    let tracker = match tracker::Tracker::new(&config.clone(), stats_event_sender, stats_repository) {
        Ok(tracker) => Arc::new(tracker),
        Err(error) => {
            panic!("{}", error)
        }
    };

    // Initialize logging
    logging::setup(&config);

    // Run jobs
    let jobs = setup::setup(&config, tracker.clone()).await;

    // handle the signals here
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Torrust shutting down..");

            // Await for all jobs to shutdown
            futures::future::join_all(jobs).await;
            info!("Torrust successfully shutdown.");
        }
    }
}
