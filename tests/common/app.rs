use std::sync::Arc;

use torrust_tracker::bootstrap;
use torrust_tracker::tracker::Tracker;

pub fn setup_with_configuration(configuration: &Arc<torrust_tracker_configuration::Configuration>) -> Arc<Tracker> {
    bootstrap::app::initialize_with_configuration(configuration)
}
