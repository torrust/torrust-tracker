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
use bittorrent_udp_tracker_core::crypto::ephemeral_instance_keys;
use bittorrent_udp_tracker_core::crypto::keys::{self, Keeper as _};
use torrust_tracker_clock::static_time;
use torrust_tracker_configuration::validator::Validator;
use torrust_tracker_configuration::{logging, Configuration};
use tracing::instrument;

use super::config::initialize_configuration;
use crate::container::AppContainer;

/// It loads the configuration from the environment and builds app container.
///
/// # Panics
///
/// Setup can file if the configuration is invalid.
#[must_use]
#[instrument(skip())]
pub fn setup() -> (Configuration, AppContainer) {
    #[cfg(not(test))]
    check_seed();

    let configuration = initialize_configuration();

    if let Err(e) = configuration.validate() {
        panic!("Configuration error: {e}");
    }

    initialize_global_services(&configuration);

    tracing::info!("Configuration:\n{}", configuration.clone().mask_secrets().to_json());

    let app_container = AppContainer::initialize(&configuration);

    (configuration, app_container)
}

/// checks if the seed is the instance seed in production.
///
/// # Panics
///
/// It would panic if the seed is not the instance seed.
pub fn check_seed() {
    let seed = keys::Current::get_seed();
    let instance = keys::Instance::get_seed();

    assert_eq!(seed, instance, "maybe using zeroed seed in production!?");
}

/// It initializes the global services.
#[instrument(skip())]
pub fn initialize_global_services(configuration: &Configuration) {
    initialize_static();
    logging::setup(&configuration.logging);
}

/// It initializes the application static values.
///
/// These values are accessible throughout the entire application:
///
/// - The time when the application started.
/// - An ephemeral instance random seed. This seed is used for encryption and it's changed when the main application process is restarted.
#[instrument(skip())]
pub fn initialize_static() {
    // Set the time of Torrust app starting
    lazy_static::initialize(&static_time::TIME_AT_APP_START);

    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

    // Initialize the Ephemeral Instance Random Cipher
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_CIPHER_BLOWFISH);

    // Initialize the Zeroed Cipher
    lazy_static::initialize(&ephemeral_instance_keys::ZEROED_TEST_CIPHER_BLOWFISH);
}
