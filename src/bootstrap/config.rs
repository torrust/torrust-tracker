//! Initialize configuration from file or env var.
//!
//! All environment variables are prefixed with `TORRUST_TRACKER_BACK_`.
use std::env;

use torrust_tracker_configuration::Configuration;

// Environment variables

const CONFIG_PATH: &str = "./config.toml";
const CONFIG_ENV_VAR_NAME: &str = "TORRUST_TRACKER_CONFIG";

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
pub fn initialize_configuration() -> Configuration {
    if env::var(CONFIG_ENV_VAR_NAME).is_ok() {
        println!("Loading configuration from env var {CONFIG_ENV_VAR_NAME}");
        Configuration::load_from_env_var(CONFIG_ENV_VAR_NAME).unwrap()
    } else {
        println!("Loading configuration from config file {CONFIG_PATH}");
        Configuration::load_from_file(CONFIG_PATH).unwrap()
    }
}
