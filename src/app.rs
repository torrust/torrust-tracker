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

use torrust_clock::clock::Time;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
use tracing::instrument;

use crate::CurrentClock;
use crate::bootstrap::jobs::manager::JobManager;
use crate::bootstrap::jobs::{
    self, activity_metrics_updater, health_check_api, http_tracker, torrent_cleanup, tracker_apis, udp_tracker,
};
use crate::bootstrap::{self};
use crate::container::AppContainer;

pub async fn run() -> (Arc<AppContainer>, JobManager) {
    let (config, app_container) = bootstrap::app::setup().await;

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
    start_tracker_core_in_memory_event_listener(config, app_container, &mut job_manager);
    start_tracker_core_persistent_completed_statistics_event_listener(config, app_container, &mut job_manager);
    start_http_core_event_listener(config, app_container, &mut job_manager);
    start_udp_core_event_listener(config, app_container, &mut job_manager);
    start_udp_tracker_services(config, app_container, &mut job_manager).await;
    start_the_http_instances(config, app_container, &mut job_manager).await;

    start_torrent_cleanup(config, app_container, &mut job_manager);
    start_peers_inactivity_update(config, app_container, &mut job_manager);

    start_the_http_api(config, app_container, &mut job_manager).await;
    start_health_check_api(config, app_container, &mut job_manager).await;

    job_manager
}

fn warn_if_no_services_enabled(config: &Configuration) {
    if config.http_api.is_none()
        && config.udp_trackers.as_ref().is_none_or(std::vec::Vec::is_empty)
        && config.http_trackers.as_ref().is_none_or(std::vec::Vec::is_empty)
    {
        tracing::warn!("No services enabled in configuration");
    }
}

async fn load_peer_keys(config: &Configuration, app_container: &Arc<AppContainer>) {
    if !config.core.private {
        return;
    }

    let Some(persistence) = app_container.tracker_core_container.persistence.as_ref() else {
        return;
    };

    persistence
        .keys_handler
        .load_peer_keys_from_database()
        .await
        .expect("Could not retrieve keys from database.");
}

async fn load_whitelisted_torrents(config: &Configuration, app_container: &Arc<AppContainer>) {
    if !config.core.listed {
        return;
    }

    let Some(persistence) = app_container.tracker_core_container.persistence.as_ref() else {
        return;
    };

    persistence
        .whitelist_manager
        .load_whitelist_from_database()
        .await
        .expect("Could not load whitelist from database.");
}

async fn load_torrent_metrics(config: &Configuration, app_container: &Arc<AppContainer>) {
    if !config.core.tracker_policy.persistent_torrent_completed_stat {
        return;
    }

    let Some(persistence) = app_container.tracker_core_container.persistence.as_ref() else {
        return;
    };

    torrust_tracker_core::statistics::persisted::load_persisted_metrics(
        &app_container.tracker_core_container.stats_repository,
        &persistence.db_downloads_metric_repository,
        CurrentClock::now(),
    )
    .await
    .expect("Could not load persisted metrics from database.");
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

fn start_tracker_core_in_memory_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) {
    job_manager.push_opt(
        "tracker_core_in_memory_event_listener",
        jobs::tracker_core::start_in_memory_event_listener(config, app_container, job_manager.new_cancellation_token()),
    );
}

fn start_tracker_core_persistent_completed_statistics_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) {
    job_manager.push_opt(
        "tracker_core_persistent_completed_statistics_event_listener",
        jobs::tracker_core::start_persistent_completed_statistics_event_listener(
            config,
            app_container,
            job_manager.new_cancellation_token(),
        ),
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

async fn start_udp_tracker_services(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    if !should_start_udp_tracker_services(config) {
        log_udp_tracker_services_not_started(config);
        return;
    }

    start_udp_server_stats_event_listener(config, app_container, job_manager);
    start_udp_server_banning_event_listener(app_container, job_manager);
    // issue: #1453
    start_udp_ban_cleanup_job(config, app_container, job_manager);
    start_the_udp_instances(config, app_container, job_manager).await;
}

fn should_start_udp_tracker_services(config: &Configuration) -> bool {
    !config.core.private
        && config
            .udp_trackers
            .as_ref()
            .is_some_and(|udp_trackers| !udp_trackers.is_empty())
}

fn log_udp_tracker_services_not_started(config: &Configuration) {
    if config.core.private
        && config
            .udp_trackers
            .as_ref()
            .is_some_and(|udp_trackers| !udp_trackers.is_empty())
    {
        tracing::warn!("Could not start UDP trackers while in private mode. UDP is not safe for private trackers!");
    } else {
        tracing::info!("No UDP trackers configured");
    }
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

fn start_udp_ban_cleanup_job(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    job_manager.push(
        "udp_ban_cleanup",
        jobs::udp_tracker_server::start_ban_cleanup_job(
            config.udp_tracker_server.ip_bans_reset_interval_in_secs.get(),
            app_container,
            job_manager.new_cancellation_token(),
        ),
    );
}

async fn start_the_udp_instances(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    let udp_trackers = config
        .udp_trackers
        .as_ref()
        .expect("UDP tracker services require at least one configured UDP tracker");

    let connection_id_validation = connection_id_validation_policy(config);

    for (idx, udp_tracker_config) in udp_trackers.iter().enumerate() {
        start_udp_instance(idx, udp_tracker_config, connection_id_validation, app_container, job_manager).await;
    }
}

async fn start_udp_instance(
    idx: usize,
    udp_tracker_config: &UdpTracker,
    connection_id_validation: ConnectionIdValidationPolicy,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) {
    let (configuration_instance_id, udp_tracker_container) = app_container
        .udp_tracker_container(idx)
        .expect("Could not create UDP tracker container");
    let udp_tracker_server_container = app_container.udp_tracker_server_container();

    let handle = udp_tracker::start_job(
        udp_tracker_container,
        udp_tracker_server_container,
        app_container.registar.give_form(),
        RuntimeServiceMetadata::new(configuration_instance_id)
            .with_public_url(udp_tracker_config.public_url.as_ref().map(ToString::to_string)),
        connection_id_validation,
    )
    .await;

    job_manager.push(format!("udp_instance_{}_{}", idx, udp_tracker_config.bind_address), handle);
}

const fn connection_id_validation_policy(config: &Configuration) -> ConnectionIdValidationPolicy {
    match config.udp_tracker_server.connection_id_validation {
        torrust_tracker_configuration::v3_0_0::udp_tracker_server::ConnectionIdValidationPolicy::Strict => {
            ConnectionIdValidationPolicy::Strict
        }
        torrust_tracker_configuration::v3_0_0::udp_tracker_server::ConnectionIdValidationPolicy::Disabled => {
            ConnectionIdValidationPolicy::Disabled
        }
    }
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
    let (configuration_instance_id, http_tracker_container) = app_container
        .http_tracker_container(idx)
        .expect("Could not create HTTP tracker container");

    if let Some(handle) = http_tracker::start_job(
        http_tracker_container,
        app_container.registar.give_form(),
        RuntimeServiceMetadata::new(configuration_instance_id)
            .with_public_url(http_tracker_config.public_url.as_ref().map(ToString::to_string)),
        torrust_tracker_axum_http_server::Version::V1,
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
            RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0))
                .with_public_url(http_api_config.public_url.as_ref().map(ToString::to_string)),
            torrust_tracker_axum_rest_api_server::Version::V1,
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
    let handle = health_check_api::start_job(&config.health_check_api, app_container.registar.as_ref().clone()).await;

    job_manager.push("health_check_api", handle);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use tokio_util::sync::CancellationToken;
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;

    use super::{load_data_from_database, should_start_udp_tracker_services};
    use crate::bootstrap::jobs::tracker_core;
    use crate::container::AppContainer;

    #[test]
    fn it_should_not_start_udp_tracker_services_without_udp_trackers() {
        assert!(!should_start_udp_tracker_services(&Configuration::default()));
    }

    #[test]
    fn it_should_not_start_udp_tracker_services_with_an_empty_udp_tracker_list() {
        let configuration = Configuration {
            udp_trackers: Some(Vec::new()),
            ..Configuration::default()
        };

        assert!(!should_start_udp_tracker_services(&configuration));
    }

    #[test]
    fn it_should_not_start_udp_tracker_services_for_a_private_tracker() {
        let configuration = Configuration {
            core: Core {
                private: true,
                ..Default::default()
            },
            udp_trackers: Some(vec![UdpTracker::default()]),
            ..Configuration::default()
        };

        assert!(!should_start_udp_tracker_services(&configuration));
    }

    #[test]
    fn it_should_start_udp_tracker_services_for_a_public_tracker_with_udp_trackers() {
        let configuration = Configuration {
            udp_trackers: Some(vec![UdpTracker::default()]),
            ..Configuration::default()
        };

        assert!(should_start_udp_tracker_services(&configuration));
    }

    #[tokio::test]
    async fn it_should_start_tracker_core_statistics_listener_without_persistence() {
        let mut configuration = Configuration::default();
        configuration.core.tracker_usage_statistics = true;
        assert!(configuration.core.database.is_none());
        let app_container = Arc::new(AppContainer::initialize(&configuration).await);
        let cancellation_token = CancellationToken::new();

        let listener = tracker_core::start_in_memory_event_listener(&configuration, &app_container, cancellation_token.clone())
            .expect("tracker usage statistics must start the in-memory listener");

        cancellation_token.cancel();
        tokio::time::timeout(Duration::from_secs(1), listener)
            .await
            .expect("in-memory listener should stop after cancellation")
            .expect("in-memory listener should not panic");
    }

    #[tokio::test]
    async fn it_should_skip_persistence_loaders_when_persistence_is_absent() {
        let mut configuration = Configuration::default();
        let app_container = Arc::new(AppContainer::initialize(&configuration).await);
        assert!(app_container.tracker_core_container.persistence.is_none());

        configuration.core.private = true;
        configuration.core.listed = true;
        configuration.core.tracker_policy.persistent_torrent_completed_stat = true;

        load_data_from_database(&configuration, &app_container).await;
    }
}
