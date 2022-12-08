use std::sync::Arc;

use log::info;
use torrust_tracker::config::Configuration;
use torrust_tracker::stats::setup_statistics;
use torrust_tracker::{ephemeral_instance_keys, logging, setup, static_time, tracker};

#[tokio::main]
async fn main() {
    const CONFIG_PATH: &str = "./storage/config/config.toml";

    // Set the time of Torrust app starting
    lazy_static::initialize(&static_time::TIME_AT_APP_START);

    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

    // Initialize Torrust config
    let config = match Configuration::load_from_file(CONFIG_PATH) {
        Ok(config) => Arc::new(config),
        Err(error) => {
            panic!("{}", error)
        }
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
