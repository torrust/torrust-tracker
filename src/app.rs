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

use log::warn;
use tokio::task::JoinHandle;
use torrust_tracker_configuration::Configuration;

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
    for plain_udp_tracker_config in &config.udp_trackers {
        match torrust_tracker_configuration::sections::udp_tracker::Config::try_from(plain_udp_tracker_config.clone()) {
            Ok(udp_tracker_config) => {
                if !udp_tracker_config.is_enabled() {
                    continue;
                }

                if tracker.is_private() {
                    warn!(
                        "Could not start UDP tracker on: {} while in {:?}. UDP is not safe for private trackers!",
                        udp_tracker_config.bind_address(),
                        config.mode
                    );
                } else {
                    jobs.push(udp_tracker::start_job(&udp_tracker_config.into(), tracker.clone(), registar.give_form()).await);
                }
            }
            Err(err) => panic!("Invalid UDP Tracker configuration: {err}"),
        }
    }

    // Start the HTTP blocks
    for plain_http_tracker_config in &config.http_trackers {
        match torrust_tracker_configuration::sections::http_tracker::Config::try_from(plain_http_tracker_config.clone()) {
            Ok(http_tracker_config) => {
                if !http_tracker_config.is_enabled() {
                    continue;
                }

                if let Some(job) = http_tracker::start_job(
                    &http_tracker_config.into(),
                    tracker.clone(),
                    registar.give_form(),
                    servers::http::Version::V1,
                )
                .await
                {
                    jobs.push(job);
                };
            }
            Err(err) => panic!("Invalid HTTP Tracker configuration: {err}"),
        }
    }

    // Start HTTP API
    match torrust_tracker_configuration::sections::tracker_api::Config::try_from(config.http_api.clone()) {
        Ok(tracker_api_config) => {
            if tracker_api_config.is_enabled() {
                if let Some(job) = tracker_apis::start_job(
                    &tracker_api_config.into(),
                    tracker.clone(),
                    registar.give_form(),
                    servers::apis::Version::V1,
                )
                .await
                {
                    jobs.push(job);
                };
            }
        }
        Err(err) => panic!("Invalid Tracker API configuration: {err}"),
    }

    // Start runners to remove torrents without peers, every interval
    if config.inactive_peer_cleanup_interval > 0 {
        jobs.push(torrent_cleanup::start_job(config, &tracker));
    }

    // Start Health Check API
    match torrust_tracker_configuration::sections::health_check_api::Config::try_from(config.health_check_api.clone()) {
        Ok(health_check_api_config) => {
            jobs.push(health_check_api::start_job(&health_check_api_config.into(), registar.entries()).await);
        }
        Err(err) => panic!("Invalid Health Check API configuration: {err}"),
    }

    jobs
}
