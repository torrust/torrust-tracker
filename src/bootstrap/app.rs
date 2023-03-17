use std::env;
use std::sync::Arc;

use torrust_tracker_configuration::Configuration;

use crate::bootstrap::stats;
use crate::shared::clock::static_time;
use crate::shared::crypto::ephemeral_instance_keys;
use crate::tracker::Tracker;
use crate::{bootstrap, tracker};

pub fn initialize_static() {
    // Set the time of Torrust app starting
    lazy_static::initialize(&static_time::TIME_AT_APP_START);

    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);
}

/// # Panics
///
/// Will panic if it can't load the configuration from either
/// `./config.toml` file or env var `TORRUST_TRACKER_CONFIG`.
#[must_use]
pub fn setup() -> (Arc<Configuration>, Arc<Tracker>) {
    const CONFIG_PATH: &str = "./config.toml";
    const CONFIG_ENV_VAR_NAME: &str = "TORRUST_TRACKER_CONFIG";

    initialize_static();

    // Initialize Torrust config
    let config = if env::var(CONFIG_ENV_VAR_NAME).is_ok() {
        println!("Loading configuration from env var {CONFIG_ENV_VAR_NAME}");
        Arc::new(Configuration::load_from_env_var(CONFIG_ENV_VAR_NAME).unwrap())
    } else {
        println!("Loading configuration from config file {CONFIG_PATH}");
        Arc::new(Configuration::load_from_file(CONFIG_PATH).unwrap())
    };

    // Initialize statistics
    let (stats_event_sender, stats_repository) = stats::setup(config.tracker_usage_statistics);

    // Initialize Torrust tracker
    let tracker = match tracker::Tracker::new(config.clone(), stats_event_sender, stats_repository) {
        Ok(tracker) => Arc::new(tracker),
        Err(error) => {
            panic!("{}", error)
        }
    };

    // Initialize logging
    bootstrap::logging::setup(&config);

    (config, tracker)
}
