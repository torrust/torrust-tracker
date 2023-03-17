use std::sync::Arc;

use torrust_tracker::bootstrap;
use torrust_tracker::tracker::services::common::tracker_factory;
use torrust_tracker::tracker::Tracker;

// TODO: Move to test-helpers crate once `Tracker` is isolated.
#[allow(clippy::module_name_repetitions)]
pub fn new_tracker(configuration: Arc<torrust_tracker_configuration::Configuration>) -> Arc<Tracker> {
    bootstrap::app::initialize_static();

    // Initialize logging
    bootstrap::logging::setup(&configuration);

    Arc::new(tracker_factory(configuration))
}
