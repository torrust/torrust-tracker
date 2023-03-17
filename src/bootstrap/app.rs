use std::env;
use std::sync::Arc;

use torrust_tracker_configuration::Configuration;

use crate::bootstrap;
use crate::shared::clock::static_time;
use crate::shared::crypto::ephemeral_instance_keys;
use crate::tracker::services::tracker_factory;
use crate::tracker::Tracker;

#[must_use]
pub fn setup() -> (Arc<Configuration>, Arc<Tracker>) {
    let configuration = Arc::new(initialize_configuration());
    let tracker = initialize_with_configuration(&configuration);

    (configuration, tracker)
}

#[must_use]
pub fn initialize_with_configuration(configuration: &Arc<Configuration>) -> Arc<Tracker> {
    initialize_static();
    initialize_logging(configuration);
    Arc::new(initialize_tracker(configuration))
}

pub fn initialize_static() {
    // Set the time of Torrust app starting
    lazy_static::initialize(&static_time::TIME_AT_APP_START);

    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);
}

/// # Panics
///
/// Will panic if it can't load the configuration from either
/// `./config.toml` file or the env var `TORRUST_TRACKER_CONFIG`.
#[must_use]
fn initialize_configuration() -> Configuration {
    const CONFIG_PATH: &str = "./config.toml";
    const CONFIG_ENV_VAR_NAME: &str = "TORRUST_TRACKER_CONFIG";

    if env::var(CONFIG_ENV_VAR_NAME).is_ok() {
        println!("Loading configuration from env var {CONFIG_ENV_VAR_NAME}");
        Configuration::load_from_env_var(CONFIG_ENV_VAR_NAME).unwrap()
    } else {
        println!("Loading configuration from config file {CONFIG_PATH}");
        Configuration::load_from_file(CONFIG_PATH).unwrap()
    }
}

#[must_use]
pub fn initialize_tracker(config: &Arc<Configuration>) -> Tracker {
    tracker_factory(config.clone())
}

pub fn initialize_logging(config: &Arc<Configuration>) {
    bootstrap::logging::setup(config);
}
