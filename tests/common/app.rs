use std::sync::Arc;

use torrust_tracker::bootstrap;
use torrust_tracker::tracker::services::tracker_factory;
use torrust_tracker::tracker::Tracker;

pub fn setup_with_config(configuration: Arc<torrust_tracker_configuration::Configuration>) -> Arc<Tracker> {
    bootstrap::app::initialize_static();

    bootstrap::logging::setup(&configuration);

    Arc::new(tracker_factory(configuration))
}
