//! Initialize configuration from file or env var.
//!
//! All environment variables are prefixed with `TORRUST_TRACKER_BACK_`.

use torrust_tracker_configuration::{Configuration, Info};

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
    let info = Info::new(DEFAULT_PATH_CONFIG.to_string()).expect("info to load configuration is not valid");
    Configuration::load(&info).expect("configuration should be loaded from provided info")
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_should_load_with_default_config() {
        use crate::bootstrap::config::initialize_configuration;

        drop(initialize_configuration());
    }
}
