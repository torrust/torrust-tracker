//! Initialize configuration from file or env var.
//!
//! All environment variables are prefixed with `TORRUST_TRACKER_BACK_`.
use std::env;
use std::path::Path;

use torrust_tracker_configuration::{Configuration, Error};

// Environment variables

/// The whole `config.toml` file content. It has priority over the config file.
/// Even if the file is not on the default path.
const ENV_VAR_CONFIG: &str = "TORRUST_TRACKER_CONFIG";

/// The `config.toml` file location.
pub const ENV_VAR_CONFIG_PATH: &str = "TORRUST_IDX_BACK_CONFIG_PATH";

// Default values

const ENV_VAR_DEFAULT_CONFIG_PATH: &str = "./config.toml";

/// It loads the application configuration from the environment.
///
/// There are two methods to inject the configuration:
///
/// 1. By using a config file: `config.toml`.
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
    if env::var(ENV_VAR_CONFIG).is_ok() {
        println!("Loading configuration from env var {ENV_VAR_CONFIG} ...");

        Configuration::load_from_env_var(ENV_VAR_CONFIG).unwrap()
    } else {
        let config_path = env::var(ENV_VAR_CONFIG_PATH).unwrap_or_else(|_| ENV_VAR_DEFAULT_CONFIG_PATH.to_string());

        println!("Loading configuration from configuration file: `{config_path}` ...");

        load_from_file_or_create_default(&config_path).unwrap()
    }
}

/// Loads the configuration from the configuration file. If the file does
/// not exist, it will create a default configuration file and return an
/// error.
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or has a bad configuration.
fn load_from_file_or_create_default(path: &str) -> Result<Configuration, Error> {
    if Path::new(&path).is_file() {
        Configuration::load_from_file(path)
    } else {
        println!("Missing configuration file.");
        println!("Creating a default configuration file: `{path}` ...");
        let config = Configuration::create_default_configuration_file(path)?;
        println!("Please review the config file: `{path}` and restart the tracker if needed.");
        Ok(config)
    }
}
