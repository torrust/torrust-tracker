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

use torrust_tracker_clock::clock::Time;
use torrust_tracker_configuration::{Configuration, HttpTracker, UdpTracker};
use tracing::instrument;

use crate::bootstrap::jobs::manager::JobManager;
use crate::bootstrap::jobs::{
    self, activity_metrics_updater, health_check_api, http_tracker, torrent_cleanup, tracker_apis, udp_tracker,
};
use crate::bootstrap::{self};
use crate::container::AppContainer;
use crate::CurrentClock;

pub async fn run() -> (Arc<AppContainer>, JobManager) {
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
pub async fn start(config: &Configuration, app_container: &Arc<AppContainer>) -> JobManager {
    warn_if_no_services_enabled(config);

    load_data_from_database(config, app_container).await;

    start_jobs(config, app_container).await
}

async fn load_data_from_database(config: &Configuration, app_container: &Arc<AppContainer>) {
    load_peer_keys(config, app_container).await;
    load_whitelisted_torrents(config, app_container).await;
    load_torrent_metrics(config, app_container).await;
}

async fn start_jobs(config: &Configuration, app_container: &Arc<AppContainer>) -> JobManager {
    let mut job_manager = JobManager::new();

    start_swarm_coordination_registry_event_listener(config, app_container, &mut job_manager);
    start_tracker_core_event_listener(config, app_container, &mut job_manager);
    start_http_core_event_listener(config, app_container, &mut job_manager);
    start_udp_core_event_listener(config, app_container, &mut job_manager);
    start_udp_server_stats_event_listener(config, app_container, &mut job_manager);
    start_udp_server_banning_event_listener(app_container, &mut job_manager);

    start_the_udp_instances(config, app_container, &mut job_manager).await;
    start_the_http_instances(config, app_container, &mut job_manager).await;

    start_torrent_cleanup(config, app_container, &mut job_manager);
    start_peers_inactivity_update(config, app_container, &mut job_manager);

    start_the_http_api(config, app_container, &mut job_manager).await;
    start_health_check_api(config, app_container, &mut job_manager).await;

    job_manager
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

async fn load_torrent_metrics(config: &Configuration, app_container: &Arc<AppContainer>) {
    if config.core.tracker_policy.persistent_torrent_completed_stat {
        bittorrent_tracker_core::statistics::persisted::load_persisted_metrics(
            &app_container.tracker_core_container.stats_repository,
            &app_container.tracker_core_container.db_downloads_metric_repository,
            CurrentClock::now(),
        )
        .await
        .expect("Could not load persisted metrics from database.");
    }
}

fn start_swarm_coordination_registry_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) {
    job_manager.push_opt(
        "swarm_coordination_registry_event_listener",
        jobs::torrent_repository::start_event_listener(config, app_container, job_manager.new_cancellation_token()),
    );
}

fn start_tracker_core_event_listener(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    job_manager.push_opt(
        "tracker_core_event_listener",
        jobs::tracker_core::start_event_listener(config, app_container, job_manager.new_cancellation_token()),
    );
}

fn start_http_core_event_listener(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    job_manager.push_opt(
        "http_core_event_listener",
        jobs::http_tracker_core::start_event_listener(config, app_container, job_manager.new_cancellation_token()),
    );
}

fn start_udp_core_event_listener(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    job_manager.push_opt(
        "udp_core_event_listener",
        jobs::udp_tracker_core::start_event_listener(config, app_container, job_manager.new_cancellation_token()),
    );
}

fn start_udp_server_stats_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) {
    job_manager.push_opt(
        "udp_server_stats_event_listener",
        jobs::udp_tracker_server::start_stats_event_listener(config, app_container, job_manager.new_cancellation_token()),
    );
}

fn start_udp_server_banning_event_listener(app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    job_manager.push(
        "udp_server_banning_event_listener",
        jobs::udp_tracker_server::start_banning_event_listener(app_container, job_manager.new_cancellation_token()),
    );
}

async fn start_the_udp_instances(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    if let Some(udp_trackers) = &config.udp_trackers {
        for (idx, udp_tracker_config) in udp_trackers.iter().enumerate() {
            if config.core.private {
                tracing::warn!(
                    "Could not start UDP tracker on: {} while in private mode. UDP is not safe for private trackers!",
                    udp_tracker_config.bind_address
                );
            } else {
                start_udp_instance(idx, udp_tracker_config, app_container, job_manager).await;
            }
        }
    } else {
        tracing::info!("No UDP blocks in configuration");
    }
}

async fn start_udp_instance(
    idx: usize,
    udp_tracker_config: &UdpTracker,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) {
    let udp_tracker_container = app_container
        .udp_tracker_container(udp_tracker_config.bind_address)
        .expect("Could not create UDP tracker container");
    let udp_tracker_server_container = app_container.udp_tracker_server_container();

    let handle = udp_tracker::start_job(
        udp_tracker_container,
        udp_tracker_server_container,
        app_container.registar.give_form(),
    )
    .await;

    job_manager.push(format!("udp_instance_{}_{}", idx, udp_tracker_config.bind_address), handle);
}

async fn start_the_http_instances(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    if let Some(http_trackers) = &config.http_trackers {
        for (idx, http_tracker_config) in http_trackers.iter().enumerate() {
            start_http_instance(idx, http_tracker_config, app_container, job_manager).await;
        }
    } else {
        tracing::info!("No HTTP blocks in configuration");
    }
}

async fn start_http_instance(
    idx: usize,
    http_tracker_config: &HttpTracker,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) {
    let http_tracker_container = app_container
        .http_tracker_container(http_tracker_config.bind_address)
        .expect("Could not create HTTP tracker container");

    if let Some(handle) = http_tracker::start_job(
        http_tracker_container,
        app_container.registar.give_form(),
        torrust_axum_http_tracker_server::Version::V1,
    )
    .await
    {
        job_manager.push(format!("http_instance_{}_{}", idx, http_tracker_config.bind_address), handle);
    }
}

async fn start_the_http_api(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
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
            job_manager.push("http_api", job);
        }
    } else {
        tracing::info!("No API block in configuration");
    }
}

fn start_torrent_cleanup(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    if config.core.inactive_peer_cleanup_interval > 0 {
        let handle = torrent_cleanup::start_job(&config.core, &app_container.tracker_core_container.torrents_manager);

        job_manager.push("torrent_cleanup", handle);
    }
}

fn start_peers_inactivity_update(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    if config.core.tracker_usage_statistics {
        let handle = activity_metrics_updater::start_job(config, app_container);

        job_manager.push("peers_inactivity_update", handle);
    } else {
        tracing::info!("Peers inactivity update job is disabled.");
    }
}

async fn start_health_check_api(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    let handle = health_check_api::start_job(&config.health_check_api, app_container.registar.entries()).await;

    job_manager.push("health_check_api", handle);
}
