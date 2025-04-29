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
use torrust_tracker_configuration::{Configuration, HttpTracker, UdpTracker};
use tracing::instrument;

use crate::bootstrap::jobs::{self, health_check_api, http_tracker, torrent_cleanup, tracker_apis, udp_tracker};
use crate::bootstrap::{self};
use crate::container::AppContainer;

pub async fn run() -> (Arc<AppContainer>, Vec<JoinHandle<()>>) {
    let (config, app_container) = bootstrap::app::setup();

    let app_container = Arc::new(app_container);

    let jobs = start(&config, &app_container).await;

    (app_container, jobs)
}

/// Starts the tracker application.
///
/// # Panics
///
/// Will panic if:
///
/// - Can't retrieve tracker keys from database.
/// - Can't load whitelist from database.
#[instrument(skip(config, app_container))]
pub async fn start(config: &Configuration, app_container: &Arc<AppContainer>) -> Vec<JoinHandle<()>> {
    warn_if_no_services_enabled(config);

    load_data_from_database(config, app_container).await;

    start_jobs(config, app_container).await
}

async fn load_data_from_database(config: &Configuration, app_container: &Arc<AppContainer>) {
    load_peer_keys(config, app_container).await;
    load_whitelisted_torrents(config, app_container).await;
}

async fn start_jobs(config: &Configuration, app_container: &Arc<AppContainer>) -> Vec<JoinHandle<()>> {
    let mut jobs: Vec<JoinHandle<()>> = Vec::new();

    start_http_core_event_listener(config, app_container);
    start_udp_core_event_listener(config, app_container);
    start_udp_server_event_listener(config, app_container);
    start_the_udp_instances(config, app_container, &mut jobs).await;
    start_the_http_instances(config, app_container, &mut jobs).await;
    start_the_http_api(config, app_container, &mut jobs).await;
    start_torrent_cleanup(config, app_container, &mut jobs);
    start_health_check_api(config, app_container, &mut jobs).await;

    jobs
}

fn warn_if_no_services_enabled(config: &Configuration) {
    if config.http_api.is_none()
        && (config.udp_trackers.is_none() || config.udp_trackers.as_ref().map_or(true, std::vec::Vec::is_empty))
        && (config.http_trackers.is_none() || config.http_trackers.as_ref().map_or(true, std::vec::Vec::is_empty))
    {
        tracing::warn!("No services enabled in configuration");
    }
}

async fn load_peer_keys(config: &Configuration, app_container: &Arc<AppContainer>) {
    if config.core.private {
        app_container
            .tracker_core_container
            .keys_handler
            .load_peer_keys_from_database()
            .await
            .expect("Could not retrieve keys from database.");
    }
}

async fn load_whitelisted_torrents(config: &Configuration, app_container: &Arc<AppContainer>) {
    if config.core.listed {
        app_container
            .tracker_core_container
            .whitelist_manager
            .load_whitelist_from_database()
            .await
            .expect("Could not load whitelist from database.");
    }
}

fn start_http_core_event_listener(config: &Configuration, app_container: &Arc<AppContainer>) {
    let _job = jobs::http_tracker_core::start_event_listener(config, app_container);

    // todo: this cannot be enabled otherwise the application never ends
    // because the event listener never stops. You see this console message
    // forever:
    //
    // !! shuting down in 90 seconds !!
    // 2025-04-24T15:27:45.454101Z  INFO graceful_shutdown: torrust_axum_server::signals: remaining alive connections: 0
    //
    // Depends on: https://github.com/torrust/torrust-tracker/issues/1405
}

fn start_udp_core_event_listener(config: &Configuration, app_container: &Arc<AppContainer>) {
    let _job = jobs::udp_tracker_core::start_event_listener(config, app_container);

    // todo: the job cannot be added in the jobs vector otherwise the application never ends
    // because the event listener never stops. You see this console message
    // forever:
    //
    // !! shuting down in 90 seconds !!
    // 2025-04-24T15:27:45.454101Z  INFO graceful_shutdown: torrust_axum_server::signals: remaining alive connections: 0
    //
    // Depends on: https://github.com/torrust/torrust-tracker/issues/1405
}

fn start_udp_server_event_listener(config: &Configuration, app_container: &Arc<AppContainer>) {
    let _job = jobs::udp_tracker_server::start_event_listener(config, app_container);

    // todo: the job cannot be added in the jobs vector otherwise the application never ends
    // because the event listener never stops. You see this console message
    // forever:
    //
    // !! shuting down in 90 seconds !!
    // 2025-04-24T15:27:45.454101Z  INFO graceful_shutdown: torrust_axum_server::signals: remaining alive connections: 0
    //
    // Depends on: https://github.com/torrust/torrust-tracker/issues/1405
}

async fn start_the_udp_instances(config: &Configuration, app_container: &Arc<AppContainer>, jobs: &mut Vec<JoinHandle<()>>) {
    if let Some(udp_trackers) = &config.udp_trackers {
        for udp_tracker_config in udp_trackers {
            if config.core.private {
                tracing::warn!(
                    "Could not start UDP tracker on: {} while in private mode. UDP is not safe for private trackers!",
                    udp_tracker_config.bind_address
                );
            } else {
                start_udp_instance(udp_tracker_config, app_container, jobs).await;
            }
        }
    } else {
        tracing::info!("No UDP blocks in configuration");
    }
}

async fn start_udp_instance(udp_tracker_config: &UdpTracker, app_container: &Arc<AppContainer>, jobs: &mut Vec<JoinHandle<()>>) {
    let udp_tracker_container = app_container
        .udp_tracker_container(udp_tracker_config.bind_address)
        .expect("Could not create UDP tracker container");
    let udp_tracker_server_container = app_container.udp_tracker_server_container();

    jobs.push(
        udp_tracker::start_job(
            udp_tracker_container,
            udp_tracker_server_container,
            app_container.registar.give_form(),
        )
        .await,
    );
}

async fn start_the_http_instances(config: &Configuration, app_container: &Arc<AppContainer>, jobs: &mut Vec<JoinHandle<()>>) {
    if let Some(http_trackers) = &config.http_trackers {
        for http_tracker_config in http_trackers {
            start_http_instance(http_tracker_config, app_container, jobs).await;
        }
    } else {
        tracing::info!("No HTTP blocks in configuration");
    }
}

async fn start_http_instance(
    http_tracker_config: &HttpTracker,
    app_container: &Arc<AppContainer>,
    jobs: &mut Vec<JoinHandle<()>>,
) {
    let http_tracker_container = app_container
        .http_tracker_container(http_tracker_config.bind_address)
        .expect("Could not create HTTP tracker container");

    if let Some(job) = http_tracker::start_job(
        http_tracker_container,
        app_container.registar.give_form(),
        torrust_axum_http_tracker_server::Version::V1,
    )
    .await
    {
        jobs.push(job);
    }
}

async fn start_the_http_api(config: &Configuration, app_container: &Arc<AppContainer>, jobs: &mut Vec<JoinHandle<()>>) {
    if let Some(http_api_config) = &config.http_api {
        let http_api_config = Arc::new(http_api_config.clone());
        let http_api_container = app_container.tracker_http_api_container(&http_api_config);

        if let Some(job) = tracker_apis::start_job(
            http_api_container,
            app_container.registar.give_form(),
            torrust_axum_rest_tracker_api_server::Version::V1,
        )
        .await
        {
            jobs.push(job);
        }
    } else {
        tracing::info!("No API block in configuration");
    }
}

fn start_torrent_cleanup(config: &Configuration, app_container: &Arc<AppContainer>, jobs: &mut Vec<JoinHandle<()>>) {
    if config.core.inactive_peer_cleanup_interval > 0 {
        jobs.push(torrent_cleanup::start_job(
            &config.core,
            &app_container.tracker_core_container.torrents_manager,
        ));
    }
}

async fn start_health_check_api(config: &Configuration, app_container: &Arc<AppContainer>, jobs: &mut Vec<JoinHandle<()>>) {
    jobs.push(health_check_api::start_job(&config.health_check_api, app_container.registar.entries()).await);
}
