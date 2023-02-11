use std::path::Path;
use std::sync::Arc;

use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::config::Configuration;
use crate::tracker::statistics::Keeper;
use crate::tracker::Tracker;

#[derive(Builder, Getters, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
#[builder(pattern = "immutable")]
pub struct Tls {
    #[getter(rename = "get_certificate_file_path")]
    certificate_file_path: Box<Path>,
    #[getter(rename = "get_key_file_path")]
    key_file_path: Box<Path>,
}

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
