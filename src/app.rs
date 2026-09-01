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
use std::time::Duration;

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

/// Errors encountered while completing initial tracker startup.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tracker setup failed. Correct the reported configuration or dependency problem and restart: {source}")]
    Setup { source: crate::bootstrap::app::Error },

    #[error(
        "Could not load initial tracker data from persistence. Verify that the configured database is available and valid: {source}"
    )]
    InitialPersistenceLoad {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Configured {service} startup failed. Correct its configuration or make its listener address available: {source}")]
    ServiceStartup {
        service: &'static str,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error(
        "Configured {service} has no matching application container. Correct the service configuration and restart: {source}"
    )]
    MissingServiceContainer {
        service: &'static str,
        source: crate::container::Error,
    },

    #[error(
        "Persistent completed statistics were enabled without a persistence container. Configure `[core.database]` or disable the feature."
    )]
    PersistentStatisticsRequirePersistence,
}

/// Runs all initial tracker startup operations.
///
/// # Errors
///
/// Returns setup, persistence-load, or initial-service startup errors.
pub async fn run() -> Result<(Arc<AppContainer>, JobManager), Error> {
    let (config, app_container) = bootstrap::app::setup().await.map_err(|source| Error::Setup { source })?;

    let app_container = Arc::new(app_container);

    run_after_setup(&config, &app_container).await
}

async fn run_after_setup(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
) -> Result<(Arc<AppContainer>, JobManager), Error> {
    let jobs = start(config, app_container).await?;

    Ok((app_container.clone(), jobs))
}

/// Starts the tracker application.
///
/// # Errors
///
/// Returns initial persistence-load or service-start errors.
#[instrument(skip(config, app_container))]
pub async fn start(config: &Configuration, app_container: &Arc<AppContainer>) -> Result<JobManager, Error> {
    warn_if_no_services_enabled(config);

    load_data_from_database(config, app_container).await?;

    start_jobs(config, app_container).await
}

async fn load_data_from_database(config: &Configuration, app_container: &Arc<AppContainer>) -> Result<(), Error> {
    load_peer_keys(config, app_container).await?;
    load_whitelisted_torrents(config, app_container).await?;
    load_torrent_metrics(config, app_container).await?;

    Ok(())
}

fn initial_persistence_load_error(source: impl std::error::Error + Send + Sync + 'static) -> Error {
    Error::InitialPersistenceLoad {
        source: Box::new(source),
    }
}

fn map_initial_persistence_load<T, E>(result: Result<T, E>) -> Result<T, Error>
where
    E: std::error::Error + Send + Sync + 'static,
{
    result.map_err(initial_persistence_load_error)
}

async fn start_jobs(config: &Configuration, app_container: &Arc<AppContainer>) -> Result<JobManager, Error> {
    let mut job_manager = JobManager::new();

    if let Err(error) = start_jobs_with_manager(config, app_container, &mut job_manager).await {
        job_manager.cancel();
        job_manager.wait_for_all(Duration::from_secs(10)).await;
        return Err(error);
    }

    Ok(job_manager)
}

async fn start_jobs_with_manager(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    start_swarm_coordination_registry_event_listener(config, app_container, job_manager);
    start_tracker_core_in_memory_event_listener(config, app_container, job_manager);
    start_tracker_core_persistent_completed_statistics_event_listener(config, app_container, job_manager)?;
    start_http_core_event_listener(config, app_container, job_manager);
    start_udp_core_event_listener(config, app_container, job_manager);
    start_udp_tracker_services(config, app_container, job_manager).await?;
    start_the_http_instances(config, app_container, job_manager).await?;

    start_torrent_cleanup(config, app_container, job_manager);
    start_peers_inactivity_update(config, app_container, job_manager);

    start_the_http_api(config, app_container, job_manager).await?;
    start_health_check_api(config, app_container, job_manager).await?;

    Ok(())
}

fn warn_if_no_services_enabled(config: &Configuration) {
    if config.http_api.is_none()
        && config.udp_trackers.as_ref().is_none_or(std::vec::Vec::is_empty)
        && config.http_trackers.as_ref().is_none_or(std::vec::Vec::is_empty)
    {
        tracing::warn!("No services enabled in configuration");
    }
}

async fn load_peer_keys(config: &Configuration, app_container: &Arc<AppContainer>) -> Result<(), Error> {
    if !config.core.private {
        return Ok(());
    }

    let Some(persistence) = app_container.tracker_core_container.persistence.as_ref() else {
        return Ok(());
    };

    map_initial_persistence_load(persistence.keys_handler.load_peer_keys_from_database().await)?;

    Ok(())
}

async fn load_whitelisted_torrents(config: &Configuration, app_container: &Arc<AppContainer>) -> Result<(), Error> {
    if !config.core.listed {
        return Ok(());
    }

    let Some(persistence) = app_container.tracker_core_container.persistence.as_ref() else {
        return Ok(());
    };

    map_initial_persistence_load(persistence.whitelist_manager.load_whitelist_from_database().await)?;

    Ok(())
}

async fn load_torrent_metrics(config: &Configuration, app_container: &Arc<AppContainer>) -> Result<(), Error> {
    if !config.core.tracker_policy.persistent_torrent_completed_stat {
        return Ok(());
    }

    let Some(persistence) = app_container.tracker_core_container.persistence.as_ref() else {
        return Ok(());
    };

    map_initial_persistence_load(
        torrust_tracker_core::statistics::persisted::load_persisted_metrics(
            &app_container.tracker_core_container.stats_repository,
            &persistence.db_downloads_metric_repository,
            CurrentClock::now(),
        )
        .await,
    )?;

    Ok(())
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
) -> Result<(), Error> {
    let listener = jobs::tracker_core::start_persistent_completed_statistics_event_listener(
        config,
        app_container,
        job_manager.new_cancellation_token(),
    )
    .map_err(|_| Error::PersistentStatisticsRequirePersistence)?;

    job_manager.push_opt("tracker_core_persistent_completed_statistics_event_listener", listener);

    Ok(())
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

async fn start_udp_tracker_services(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    if !should_start_udp_tracker_services(config) {
        log_udp_tracker_services_not_started(config);
        return Ok(());
    }

    start_udp_server_stats_event_listener(config, app_container, job_manager);
    start_udp_server_banning_event_listener(app_container, job_manager);
    // issue: #1453
    start_udp_ban_cleanup_job(config, app_container, job_manager);
    start_the_udp_instances(config, app_container, job_manager).await
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

async fn start_the_udp_instances(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    let udp_trackers = config.udp_trackers.as_ref().ok_or_else(|| Error::ServiceStartup {
        service: "UDP tracker",
        source: "UDP tracker startup requires at least one configured UDP tracker".into(),
    })?;

    let connection_id_validation = connection_id_validation_policy(config);

    for (idx, udp_tracker_config) in udp_trackers.iter().enumerate() {
        start_udp_instance(idx, udp_tracker_config, connection_id_validation, app_container, job_manager).await?;
    }

    Ok(())
}

async fn start_udp_instance(
    idx: usize,
    udp_tracker_config: &UdpTracker,
    connection_id_validation: ConnectionIdValidationPolicy,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    let (configuration_instance_id, udp_tracker_container) =
        app_container
            .udp_tracker_container(idx)
            .map_err(|source| Error::MissingServiceContainer {
                service: "UDP tracker",
                source,
            })?;
    let udp_tracker_server_container = app_container.udp_tracker_server_container();

    let handle = udp_tracker::start_job(
        udp_tracker_container,
        udp_tracker_server_container,
        app_container.registar.give_form(),
        RuntimeServiceMetadata::new(configuration_instance_id)
            .with_public_url(udp_tracker_config.public_url.as_ref().map(|url| url.as_url().clone())),
        connection_id_validation,
        job_manager.new_cancellation_token(),
    )
    .await
    .map_err(|source| Error::ServiceStartup {
        service: "UDP tracker",
        source: Box::new(source),
    })?;

    job_manager.push(format!("udp_instance_{}_{}", idx, udp_tracker_config.bind_address), handle);
    Ok(())
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

async fn start_the_http_instances(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    if let Some(http_trackers) = &config.http_trackers {
        for (idx, http_tracker_config) in http_trackers.iter().enumerate() {
            start_http_instance(idx, http_tracker_config, app_container, job_manager).await?;
        }
    } else {
        tracing::info!("No HTTP blocks in configuration");
    }
    Ok(())
}

async fn start_http_instance(
    idx: usize,
    http_tracker_config: &HttpTracker,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    let (configuration_instance_id, http_tracker_container) =
        app_container
            .http_tracker_container(idx)
            .map_err(|source| Error::MissingServiceContainer {
                service: "HTTP tracker",
                source,
            })?;

    if let Some(handle) = http_tracker::start_job(
        http_tracker_container,
        app_container.registar.give_form(),
        RuntimeServiceMetadata::new(configuration_instance_id)
            .with_public_url(http_tracker_config.public_url.as_ref().map(|url| url.as_url().clone())),
        torrust_tracker_axum_http_server::Version::V1,
        job_manager.new_cancellation_token(),
    )
    .await
    .map_err(|source| Error::ServiceStartup {
        service: "HTTP tracker",
        source: Box::new(source),
    })? {
        job_manager.push(format!("http_instance_{}_{}", idx, http_tracker_config.bind_address), handle);
    }
    Ok(())
}

async fn start_the_http_api(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    if let Some(http_api_config) = &config.http_api {
        let http_api_config = Arc::new(http_api_config.clone());
        let http_api_container = app_container.tracker_http_api_container(&http_api_config);

        if let Some(job) = tracker_apis::start_job(
            http_api_container,
            app_container.registar.give_form(),
            RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0))
                .with_public_url(http_api_config.public_url.as_ref().map(|url| url.as_url().clone())),
            torrust_tracker_axum_rest_api_server::Version::V1,
            job_manager.new_cancellation_token(),
        )
        .await
        .map_err(|source| Error::ServiceStartup {
            service: "tracker API",
            source: Box::new(source),
        })? {
            job_manager.push("http_api", job);
        }
    } else {
        tracing::info!("No API block in configuration");
    }
    Ok(())
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

async fn start_health_check_api(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    let handle = health_check_api::start_job(
        &config.health_check_api,
        app_container.registar.as_ref().clone(),
        job_manager.new_cancellation_token(),
    )
    .await
    .map_err(|source| Error::ServiceStartup {
        service: "health check API",
        source: Box::new(source),
    })?;

    job_manager.push("health_check_api", handle);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, TcpListener, UdpSocket};
    use std::sync::Arc;
    use std::time::Duration;

    use tokio_util::sync::CancellationToken;
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;

    use super::{Error, load_data_from_database, run_after_setup, should_start_udp_tracker_services};
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
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
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
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        assert!(app_container.tracker_core_container.persistence.is_none());

        configuration.core.private = true;
        configuration.core.listed = true;
        configuration.core.tracker_policy.persistent_torrent_completed_stat = true;

        load_data_from_database(&configuration, &app_container)
            .await
            .expect("persistence loaders should be skipped when persistence is absent");
    }

    #[tokio::test]
    async fn it_should_retain_the_loader_error_when_initial_peer_key_loading_fails() {
        // Arrange
        let configuration = torrust_tracker_test_helpers::configuration::ephemeral_private();
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let persistence = app_container
            .tracker_core_container
            .persistence
            .as_ref()
            .expect("private test configuration should compose persistence");
        persistence
            .database_stores
            .schema_migrator
            .drop_database_tables()
            .await
            .expect("remove the peer-key table after composition");

        // Act
        let error = load_data_from_database(&configuration, &app_container)
            .await
            .expect_err("a failed peer-key loader should return an application error");

        // Assert
        let Error::InitialPersistenceLoad { source } = error else {
            panic!("initial persistence load errors must retain their source");
        };
        let source = source
            .downcast_ref::<torrust_tracker_core::databases::error::Error>()
            .expect("initial persistence load error source should retain the database error");
        assert!(matches!(
            source,
            torrust_tracker_core::databases::error::Error::InvalidQuery { .. }
        ));
        assert!(
            std::error::Error::source(source).is_some(),
            "database error should retain its SQL source"
        );
    }

    #[tokio::test]
    async fn it_should_release_udp_listener_before_returning_from_run_after_setup_when_later_http_startup_fails() {
        // Arrange
        let mut configuration = torrust_tracker_test_helpers::configuration::ephemeral_public();
        let udp_address = reserve_udp_address();
        configuration.udp_trackers.as_mut().expect("test configuration enables UDP")[0].bind_address = udp_address;
        let http_listener = TcpListener::bind("127.0.0.1:0").expect("reserve HTTP listener address");
        configuration.http_trackers.as_mut().expect("test configuration enables HTTP")[0].bind_address =
            http_listener.local_addr().expect("read HTTP listener address");
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );

        // Act
        let result = run_after_setup(&configuration, &app_container).await;

        // Assert
        assert!(result.is_err());
        UdpSocket::bind(udp_address).expect("UDP listener should be released before startup returns");
    }

    fn reserve_udp_address() -> SocketAddr {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("reserve UDP listener address");
        socket.local_addr().expect("read UDP listener address")
    }
}
