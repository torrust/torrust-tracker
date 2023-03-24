//! Setup for the main tracker application.
//!
//! The [`setup`](bootstrap::app::setup) only builds the application and its dependencies but it does not start the application.
//! In fact, there is no such thing as the main application process. When the application starts, the only thing it does is
//! starting a bunch of independent jobs. If you are looking for how things are started you should read [`app::start`](crate::app::start)
//! function documentation.
//!
//! Setup steps:
//!
//! 1. Load the global application configuration.
//! 2. Initialize static variables.
//! 3. Initialize logging.
//! 4. Initialize the domain tracker.
use std::env;
use std::sync::Arc;

use torrust_tracker_configuration::Configuration;

use crate::bootstrap;
use crate::shared::clock::static_time;
use crate::shared::crypto::ephemeral_instance_keys;
use crate::tracker::services::tracker_factory;
use crate::tracker::Tracker;

/// It loads the configuration from the environment and builds the main domain [`tracker`](crate::tracker::Tracker) struct.
#[must_use]
pub fn setup() -> (Arc<Configuration>, Arc<Tracker>) {
    let configuration = Arc::new(initialize_configuration());
    let tracker = initialize_with_configuration(&configuration);

    (configuration, tracker)
}

/// It initializes the application with the given configuration.
///
/// The configuration may be obtained from the environment (via config file or env vars).
#[must_use]
pub fn initialize_with_configuration(configuration: &Arc<Configuration>) -> Arc<Tracker> {
    initialize_static();
    initialize_logging(configuration);
    Arc::new(initialize_tracker(configuration))
}

/// It initializes the application static values.
///
/// These values are accessible throughout the entire application:
///
/// - The time when the application started.
/// - An ephemeral instance random seed. This seed is used for encryption and it's changed when the main application process is restarted.
pub fn initialize_static() {
    // Set the time of Torrust app starting
    lazy_static::initialize(&static_time::TIME_AT_APP_START);

    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);
}

/// It loads the application configuration from the environment.
///
/// There are two methods to inject the configuration:
///
/// 1. By using a config file: `config.toml`. The file must be in the same folder where you are running the tracker.
/// 2. Environment variable: `TORRUST_TRACKER_CONFIG`. The variable contains the same contents as the `config.toml` file.
///
/// Environment variable has priority over the config file.
///
/// Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration) for the configuration options.
///
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

/// It builds the domain tracker
///
/// The tracker is the domain layer service. It's the entrypoint to make requests to the domain layer.
/// It's used by other higher-level components like the UDP and HTTP trackers or the tracker API.
#[must_use]
pub fn initialize_tracker(config: &Arc<Configuration>) -> Tracker {
    tracker_factory(config.clone())
}

/// It initializes the log level, format and channel.
///
/// See [the logging setup](crate::bootstrap::logging::setup) for more info about logging.
pub fn initialize_logging(config: &Arc<Configuration>) {
    bootstrap::logging::setup(config);
}
