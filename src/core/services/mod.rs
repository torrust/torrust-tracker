//! Tracker domain services. Core and statistics services.
//!
//! There are two types of service:
//!
//! - [Core tracker services](crate::core::services::torrent): related to the tracker main functionalities like getting info about torrents.
//! - [Services for statistics](crate::core::services::statistics): related to tracker metrics. Aggregate data about the tracker server.
pub mod statistics;
pub mod torrent;

use std::sync::Arc;

use torrust_tracker_configuration::Configuration;

use crate::core::Tracker;

/// It returns a new tracker building its dependencies.
///
/// # Panics
///
/// Will panic if tracker cannot be instantiated.
#[must_use]
pub fn tracker_factory(config: &Configuration) -> Tracker {
    // Initialize statistics
    let (stats_event_sender, stats_repository) = statistics::setup::factory(config.tracker_usage_statistics);

    // Initialize Torrust tracker
    match Tracker::new(&Arc::new(config), stats_event_sender, stats_repository) {
        Ok(tracker) => tracker,
        Err(error) => {
            panic!("{}", error)
        }
    }
}
