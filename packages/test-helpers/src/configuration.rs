use std::env;

use torrust_tracker_configuration::Configuration;

use crate::random;

/// This configuration is used for testing. It generates random config values so they do not collide
/// if  you run more than one tracker at the same time.
///
/// # Panics
///
/// Will panic if it can't convert the temp file path to string
#[must_use]
pub fn ephemeral() -> Configuration {
    let mut config = Configuration {
        log_level: Some("off".to_owned()),
        ..Default::default()
    };

    // Ephemeral socket addresses
    let bind_addr = "127.0.0.1:0".to_string();

    config.http_api.bind_address = bind_addr.to_string();
    config.udp_trackers[0].bind_address = bind_addr;

    // Ephemeral sqlite database
    let temp_directory = env::temp_dir();
    let random_db_id = random::string(16);
    let temp_file = temp_directory.join(format!("data_{random_db_id}.db"));

    config.db_path = temp_file.to_str().unwrap().to_owned();

    config
}
