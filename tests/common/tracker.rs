use std::sync::Arc;

use torrust_tracker::tracker::statistics::Keeper;
use torrust_tracker::tracker::Tracker;
use torrust_tracker::{ephemeral_instance_keys, logging, static_time};

pub fn tracker_configuration() -> Arc<torrust_tracker_configuration::Configuration> {
    Arc::new(torrust_tracker_test_helpers::configuration::ephemeral())
}

// TODO: Move to test-helpers crate once `Tracker` is isolated.
pub fn tracker_instance(configuration: &Arc<torrust_tracker_configuration::Configuration>) -> Arc<Tracker> {
    // Set the time of Torrust app starting
    lazy_static::initialize(&static_time::TIME_AT_APP_START);

    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

    // Initialize stats tracker
    let (stats_event_sender, stats_repository) = Keeper::new_active_instance();

    // Initialize Torrust tracker
    let tracker = match Tracker::new(configuration, Some(stats_event_sender), stats_repository) {
        Ok(tracker) => Arc::new(tracker),
        Err(error) => {
            panic!("{}", error)
        }
    };

    // Initialize logging
    logging::setup(configuration);

    tracker
}
