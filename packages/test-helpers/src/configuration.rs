use std::env;

use torrust_tracker_configuration::Configuration;

use crate::random;

/// This configuration is used for testing. It generates random config values so they do not collide
/// if you run more than one tracker at the same time.
///
/// # Panics
///
/// Will panic if it can't convert the temp file path to string
#[must_use]
pub fn ephemeral() -> Configuration {
    // todo: disable services that are not needed.
    // For example: a test for the UDP tracker should disable the API and HTTP tracker.

    let mut config = Configuration {
        log_level: Some("off".to_owned()), // Change to `debug` for tests debugging
        ..Default::default()
    };

    // Ephemeral socket address for API
    let api_port = 0u16;
    config.http_api.enabled = true;
    config.http_api.bind_address = format!("127.0.0.1:{}", &api_port);

    // Ephemeral socket address for UDP tracker
    let udp_port = 0u16;
    config.udp_trackers[0].enabled = true;
    config.udp_trackers[0].bind_address = format!("127.0.0.1:{}", &udp_port);

    // Ephemeral socket address for HTTP tracker
    let http_port = 0u16;
    config.http_trackers[0].enabled = true;
    config.http_trackers[0].bind_address = format!("127.0.0.1:{}", &http_port);

    // Ephemeral sqlite database
    let temp_directory = env::temp_dir();
    let random_db_id = random::string(16);
    let temp_file = temp_directory.join(format!("data_{random_db_id}.db"));
    config.db_path = temp_file.to_str().unwrap().to_owned();

    config
}
