//! Initialize configuration from file or env var.
//!
//! All environment variables are prefixed with `TORRUST_TRACKER_BACK_`.

use torrust_tracker_configuration::{Configuration, Info};

// Environment variables

/// The whole `tracker.toml` file content. It has priority over the config file.
/// Even if the file is not on the default path.
const ENV_VAR_CONFIG: &str = "TORRUST_TRACKER_CONFIG";
const ENV_VAR_API_ADMIN_TOKEN: &str = "TORRUST_TRACKER_API_ADMIN_TOKEN";

/// The `tracker.toml` file location.
pub const ENV_VAR_PATH_CONFIG: &str = "TORRUST_TRACKER_PATH_CONFIG";

// Default values
pub const DEFAULT_PATH_CONFIG: &str = "./share/default/config/tracker.development.sqlite3.toml";

/// It loads the application configuration from the environment.
///
/// There are two methods to inject the configuration:
///
/// 1. By using a config file: `tracker.toml`.
/// 2. Environment variable: `TORRUST_TRACKER_CONFIG`. The variable contains the same contents as the `tracker.toml` file.
///
/// Environment variable has priority over the config file.
///
/// Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration) for the configuration options.
///
/// # Panics
///
/// Will panic if it can't load the configuration from either
/// `./tracker.toml` file or the env var `TORRUST_TRACKER_CONFIG`.
#[must_use]
pub fn initialize_configuration() -> Configuration {
    let info = Info::new(
        ENV_VAR_CONFIG.to_string(),
        ENV_VAR_PATH_CONFIG.to_string(),
        DEFAULT_PATH_CONFIG.to_string(),
        ENV_VAR_API_ADMIN_TOKEN.to_string(),
    )
    .unwrap();

    Configuration::load(&info).unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_should_load_with_default_config() {
        use crate::bootstrap::config::initialize_configuration;

        drop(initialize_configuration());
    }
}
