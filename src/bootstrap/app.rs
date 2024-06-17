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
use torrust_tracker_configuration::{Configuration, LogLevel};
use tracing::level_filters::LevelFilter;

use super::config::initialize_configuration;
use crate::core::services::tracker_factory;
use crate::core::Tracker;
use crate::shared::crypto::ephemeral_instance_keys;

fn map_to_tracing_level_filter(log_level: &LogLevel) -> LevelFilter {
    match log_level {
        LogLevel::Off => LevelFilter::OFF,
        LogLevel::Error => LevelFilter::ERROR,
        LogLevel::Warn => LevelFilter::WARN,
        LogLevel::Info => LevelFilter::INFO,
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Trace => LevelFilter::TRACE,
    }
}

/// It loads the configuration from the environment gets trace level
///
/// # Panics
///
/// It will panic if the tracing level is malformed in the configuration.
#[must_use]
pub fn config() -> (Configuration, LevelFilter) {
    let config = initialize_configuration();

    let level: LevelFilter = map_to_tracing_level_filter(&config.logging.log_level);

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
