use std::sync::Arc;
use torrust_tracker_configuration::Configuration;

use crate::tracker::statistics::Keeper;
use crate::tracker::Tracker;

/// # Panics
///
/// Will panic if tracker cannot be instantiated.
#[must_use]
pub fn tracker_factory(configuration: &Arc<Configuration>) -> Tracker {
    // todo: the tracker initialization is duplicated in many places.

    // Initialize stats tracker
    let (stats_event_sender, stats_repository) = Keeper::new_active_instance();

    // Initialize Torrust tracker
    match Tracker::new(configuration, Some(stats_event_sender), stats_repository) {
        Ok(tracker) => tracker,
        Err(error) => {
            panic!("{}", error)
        }
    }
}
