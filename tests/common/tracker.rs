use std::sync::Arc;

use torrust_tracker::tracker::services::common::tracker_factory;
use torrust_tracker::tracker::Tracker;
use torrust_tracker::{ephemeral_instance_keys, logging, static_time};

// TODO: Move to test-helpers crate once `Tracker` is isolated.
pub fn new_tracker(configuration: Arc<torrust_tracker_configuration::Configuration>) -> Arc<Tracker> {
    // Set the time of Torrust app starting
    lazy_static::initialize(&static_time::TIME_AT_APP_START);

    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

    // Initialize logging
    logging::setup(&configuration);

    Arc::new(tracker_factory(configuration))
}
