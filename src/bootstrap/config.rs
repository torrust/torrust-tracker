//! Initialize configuration from file or env var.
//!
//! All environment variables are prefixed with `TORRUST_TRACKER_`.

use torrust_tracker_configuration::Info;
use torrust_tracker_configuration::v3_0_0::Configuration;

// skill-link: run-tracker-locally
pub const DEFAULT_PATH_CONFIG: &str = "./share/default/config/tracker.development.sqlite3.toml";

/// It loads the application configuration from the environment.
///
/// There are two methods to inject the configuration:
///
/// 1. By using a config file: `tracker.toml`.
/// 2. Environment variable: `TORRUST_TRACKER_CONFIG_TOML`. The variable contains the same contents as the `tracker.toml` file.
///
/// Environment variable has priority over the config file.
///
/// Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration) for the configuration options.
///
/// # Panics
///
/// Will panic if it can't load the configuration from either
/// `./tracker.toml` file or the env var `TORRUST_TRACKER_CONFIG_TOML`.
#[must_use]
pub fn initialize_configuration() -> Configuration {
    let info = Info::new(DEFAULT_PATH_CONFIG.to_string()).expect("info to load configuration is not valid");
    Configuration::load(&info).expect("error loading configuration from sources")
}

#[cfg(test)]
mod tests {

    use torrust_tracker_configuration::Info;
    use torrust_tracker_configuration::v3_0_0::Configuration;

    #[test]
    fn it_should_load_with_default_config() {
        use crate::bootstrap::config::initialize_configuration;

        drop(initialize_configuration());
    }

    #[test]
    fn it_should_load_every_shipped_configuration_template() {
        // Arrange
        let templates = [
            "./share/default/config/tracker.container.mysql.toml",
            "./share/default/config/tracker.container.no-persistence.toml",
            "./share/default/config/tracker.container.postgresql.toml",
            "./share/default/config/tracker.container.sqlite3.toml",
            "./share/default/config/tracker.development.sqlite3.toml",
            "./share/default/config/tracker.e2e.container.sqlite3.toml",
            "./share/default/config/tracker.udp.benchmarking.toml",
        ];

        // Act and assert
        for template in templates {
            let info = Info::new(template.to_string()).expect("configuration source should be valid");

            Configuration::load(&info).unwrap_or_else(|error| panic!("template should load: {template}: {error}"));
        }
    }
}
