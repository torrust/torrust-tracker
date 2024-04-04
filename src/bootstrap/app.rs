//! Setup for the main tracker application.
//!
//! The [`setup`] only builds the application and its dependencies but it does not start the application.
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
use std::sync::Arc;

use torrust_tracker_clock::static_time;
use torrust_tracker_configuration::Configuration;
use tracing::metadata::ParseLevelError;
use tracing::Level;

use super::config::initialize_configuration;
use crate::core::services::tracker_factory;
use crate::core::Tracker;
use crate::shared::crypto::ephemeral_instance_keys;

/// It loads the configuration from the environment and setups up tracing (logging).
///
/// # Panics
///
/// It will panic if the tracing level is malformed in the configuration.
#[must_use]
pub fn config() -> (Configuration, Level) {
    let config = initialize_configuration();
    let level = parse_level_or_default(&config.log_level).expect("its should provide a valid value for the log level");

    (config, level)
}

/// It initializes the application with the given configuration.
///
/// The configuration may be obtained from the environment (via config file or env vars).
#[must_use]
pub fn tracker(configuration: &Configuration) -> Arc<Tracker> {
    initialize_static();

    Arc::new(tracker_factory(configuration))
}

/// It initializes the application static values.
///
/// These values are accessible throughout the entire application:
///
/// - The time when the application started.
/// - An ephemeral instance random seed. This seed is used for encryption and it's changed when the main application process is restarted.
fn initialize_static() {
    // Set the time of Torrust app starting
    lazy_static::initialize(&static_time::TIME_AT_APP_START);

    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);
}

fn parse_level_or_default(level: &Option<String>) -> Result<Level, ParseLevelError> {
    level.as_deref().unwrap_or("info").parse()
}
