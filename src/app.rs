//! Torrust Tracker application.
//!
//! The tracker application has a global configuration for multiple jobs.
//! It's basically a container for other services.
//! It also check constraint and dependencies between services. For example:
//! It's not safe to run a UDP tracker on top of a core public tracker, as UDP trackers
//! do not allow private access to the tracker data.
//!
//! The application is responsible for:
//!
//! - Loading data from the database when it's needed.
//! - Starting some jobs depending on the configuration.
//!
//! Jobs executed always:
//!
//! - Health Check API
//!
//! Optional jobs:
//!
//! - Torrent cleaner: it removes inactive peers and (optionally) peerless torrents.
//! - UDP trackers: the user can enable multiple UDP tracker on several ports.
//! - HTTP trackers: the user can enable multiple HTTP tracker on several ports.
//! - Tracker REST API: the tracker API can be enabled/disabled.
use std::sync::Arc;

use tokio::task::JoinHandle;
use torrust_tracker_configuration::Configuration;
use tracing::warn;

use crate::bootstrap::jobs::{health_check_api, http_tracker, torrent_cleanup, tracker_apis, udp_tracker};
use crate::servers::registar::Registar;
use crate::{core, servers};

/// # Panics
///
/// Will panic if:
///
/// - Can't retrieve tracker keys from database.
/// - Can't load whitelist from database.
pub async fn start(config: &Configuration, tracker: Arc<core::Tracker>) -> Vec<JoinHandle<()>> {
    let mut jobs: Vec<JoinHandle<()>> = Vec::new();

    let registar = Registar::default();

    // Load peer keys
    if tracker.is_private() {
        tracker
            .load_keys_from_database()
            .await
            .expect("Could not retrieve keys from database.");
    }

    // Load whitelisted torrents
    if tracker.is_whitelisted() {
        tracker
            .load_whitelist_from_database()
            .await
            .expect("Could not load whitelist from database.");
    }

    // Start the UDP blocks
    for udp_tracker_config in &config.udp_trackers {
        if !udp_tracker_config.enabled {
            continue;
        }

        if tracker.is_private() {
            warn!(
                "Could not start UDP tracker on: {} while in {:?}. UDP is not safe for private trackers!",
                udp_tracker_config.bind_address, config.mode
            );
        } else {
            jobs.push(udp_tracker::start_job(udp_tracker_config, tracker.clone(), registar.give_form()).await);
        }
    }

    // Start the HTTP blocks
    for http_tracker_config in &config.http_trackers {
        if !http_tracker_config.enabled {
            continue;
        }

        if let Some(job) = http_tracker::start_job(
            http_tracker_config,
            tracker.clone(),
            registar.give_form(),
            servers::http::Version::V1,
        )
        .await
        {
            jobs.push(job);
        };
    }

    // Start HTTP API
    if config.http_api.enabled {
        if let Some(job) = tracker_apis::start_job(
            &config.http_api,
            tracker.clone(),
            registar.give_form(),
            servers::apis::Version::V1,
        )
        .await
        {
            jobs.push(job);
        };
    }

    // Start runners to remove torrents without peers, every interval
    if config.inactive_peer_cleanup_interval > 0 {
        jobs.push(torrent_cleanup::start_job(config, &tracker));
    }

    // Start Health Check API
    jobs.push(health_check_api::start_job(&config.health_check_api, registar.entries()).await);

    jobs
}
