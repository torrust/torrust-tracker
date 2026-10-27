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
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

use torrust_clock::clock::Time;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
use tracing::instrument;

use crate::CurrentClock;
use crate::bootstrap::event_flows::{self, Bus, EventFlows};
use crate::bootstrap::jobs::manager::{ComponentCompletion, ComponentResult, JobManager, Reservation, SpawnError, StopBefore};
use crate::bootstrap::jobs::{
    self, activity_metrics_updater, health_check_api, http_tracker, torrent_cleanup, tracker_apis, udp_tracker,
};
use crate::bootstrap::{self};
use crate::container::AppContainer;

/// Errors encountered while completing initial tracker startup.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
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

    #[error(
        "Application bootstrap could not register a component with the job manager. This is a defect in the tracker's startup code; please report it: {source}"
    )]
    ComponentRegistration { source: SpawnError },

    #[error(
        "Application bootstrap declared the event flows of its components in an order that cannot protect their events. This is a defect in the tracker's startup code; please report it: {source}"
    )]
    EventFlows { source: event_flows::Error },
}

/// Starts the tracker application.
///
/// # Errors
///
/// Returns setup, persistence-load, or initial-service startup errors.
pub async fn start() -> Result<(Arc<AppContainer>, JobManager), Error> {
    start_with_explicit_config_toml_path(None).await
}

/// Starts the tracker application with an optional explicitly selected TOML configuration file.
///
/// An explicit path takes priority over environment-provided base sources. Per-value environment
/// overrides continue to apply.
///
/// # Errors
///
/// Returns setup, persistence-load, or initial-service startup errors.
// issue: #2151
pub async fn start_with_explicit_config_toml_path(
    explicit_config_toml_path: Option<PathBuf>,
) -> Result<(Arc<AppContainer>, JobManager), Error> {
    let (config, app_container) = bootstrap::app::setup(explicit_config_toml_path)
        .await
        .map_err(|source| Error::Setup { source })?;

    let app_container = Arc::new(app_container);

    run_after_setup(&config, &app_container).await
}

async fn run_after_setup(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
) -> Result<(Arc<AppContainer>, JobManager), Error> {
    let jobs = complete_startup(config, app_container).await?;

    Ok((app_container.clone(), jobs))
}

/// Completes startup after application composition succeeds.
///
/// # Errors
///
/// Returns initial persistence-load or service-start errors.
#[instrument(skip(config, app_container))]
async fn complete_startup(config: &Configuration, app_container: &Arc<AppContainer>) -> Result<JobManager, Error> {
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
        job_manager
            .wait_for_all(app_container.application_config.shutdown.deadline())
            .await;
        return Err(error);
    }

    Ok(job_manager)
}

async fn start_jobs_with_manager(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    // Every listener is registered before any component that publishes on its bus: a listener must
    // subscribe before the first event, and a producer stops before the listeners of its buses.
    let mut flows = EventFlows::default();
    start_swarm_coordination_registry_event_listener(config, app_container, job_manager, &mut flows)?;
    start_tracker_core_in_memory_event_listener(config, app_container, job_manager, &mut flows)?;
    start_tracker_core_persistent_completed_statistics_event_listener(config, app_container, job_manager, &mut flows)?;
    start_http_core_event_listener(config, app_container, job_manager, &mut flows)?;
    start_udp_core_event_listener(config, app_container, job_manager, &mut flows)?;
    start_udp_tracker_services(config, app_container, job_manager, &mut flows).await?;
    start_the_http_instances(config, app_container, job_manager, &mut flows).await?;

    start_torrent_cleanup(config, app_container, job_manager, &mut flows)?;
    start_peers_inactivity_update(config, app_container, job_manager)?;

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
    flows: &mut EventFlows,
) -> Result<(), Error> {
    let reservation = job_manager.reserve("swarm_coordination_registry_event_listener");
    if let Some(runner) = jobs::torrent_repository::run_event_listener(config, app_container, reservation.cancellation_token()) {
        spawn_listener(job_manager, flows, Bus::Swarm, reservation, component_runner(runner))?;
    }

    Ok(())
}

fn start_tracker_core_in_memory_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
) -> Result<(), Error> {
    let reservation = job_manager.reserve("tracker_core_in_memory_event_listener");
    if let Some(runner) =
        jobs::tracker_core::run_in_memory_event_listener(config, app_container, reservation.cancellation_token())
    {
        spawn_listener(job_manager, flows, Bus::Swarm, reservation, component_runner(runner))?;
    }

    Ok(())
}

fn start_tracker_core_persistent_completed_statistics_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
) -> Result<(), Error> {
    let reservation = job_manager.reserve("tracker_core_persistent_completed_statistics_event_listener");
    if let Some(runner) = jobs::tracker_core::run_persistent_completed_statistics_event_listener(
        config,
        app_container,
        reservation.cancellation_token(),
    )
    .map_err(|_| Error::PersistentStatisticsRequirePersistence)?
    {
        spawn_listener(job_manager, flows, Bus::Swarm, reservation, component_runner(runner))?;
    }

    Ok(())
}

fn start_http_core_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
) -> Result<(), Error> {
    let reservation = job_manager.reserve("http_core_event_listener");
    let runner = jobs::http_tracker_core::run_event_listener(config, app_container, reservation.cancellation_token());
    spawn_listener(job_manager, flows, Bus::HttpCore, reservation, component_runner(runner))
}

fn start_udp_core_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
) -> Result<(), Error> {
    let reservation = job_manager.reserve("udp_core_event_listener");
    let runner = jobs::udp_tracker_core::run_event_listener(config, app_container, reservation.cancellation_token());
    spawn_listener(job_manager, flows, Bus::UdpCore, reservation, component_runner(runner))
}

async fn start_udp_tracker_services(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
) -> Result<(), Error> {
    if !should_start_udp_tracker_services(config) {
        log_udp_tracker_services_not_started(config);
        return Ok(());
    }

    start_udp_server_stats_event_listener(config, app_container, job_manager, flows)?;
    start_udp_server_banning_event_listener(config, app_container, job_manager, flows)?;
    // issue: #1453
    start_udp_ban_cleanup_job(config, app_container, job_manager);
    start_the_udp_instances(config, app_container, job_manager, flows).await
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
    flows: &mut EventFlows,
) -> Result<(), Error> {
    let reservation = job_manager.reserve("udp_server_stats_event_listener");
    let runner = jobs::udp_tracker_server::run_stats_event_listener(config, app_container, reservation.cancellation_token());
    spawn_listener(job_manager, flows, Bus::UdpServer, reservation, component_runner(runner))
}

fn start_udp_server_banning_event_listener(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
) -> Result<(), Error> {
    let reservation = job_manager.reserve("udp_server_banning_event_listener");
    let runner = jobs::udp_tracker_server::run_banning_event_listener(config, app_container, reservation.cancellation_token());
    spawn_listener(job_manager, flows, Bus::UdpServer, reservation, component_runner(runner))
}

// issue: #1488
// A legacy job: it keeps the manager's root token until UDP IP-ban cleanup
// migrates to a reserved component.
fn start_udp_ban_cleanup_job(config: &Configuration, app_container: &Arc<AppContainer>, job_manager: &mut JobManager) {
    job_manager.register_legacy(
        "udp_ban_cleanup",
        jobs::udp_tracker_server::start_ban_cleanup_job(
            config.udp_tracker_server.ip_bans_reset_interval_in_secs.get(),
            app_container,
            job_manager.legacy_cancellation_token(),
        ),
    );
}

/// Spawns a reserved component, reporting a registration defect as a startup error.
fn spawn_component<F>(job_manager: &mut JobManager, reservation: Reservation, runner: F) -> Result<(), Error>
where
    F: Future<Output = ComponentResult> + Send + 'static,
{
    job_manager
        .spawn(reservation, runner)
        .map_err(|source| Error::ComponentRegistration { source })
}

/// Spawns a listener that consumes from `bus`, and registers it as a consumer of that bus.
fn spawn_listener<F>(
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
    bus: Bus,
    reservation: Reservation,
    runner: F,
) -> Result<(), Error>
where
    F: Future<Output = ComponentResult> + Send + 'static,
{
    flows
        .add_consumer(bus, reservation.id())
        .map_err(|source| Error::EventFlows { source })?;
    spawn_component(job_manager, reservation, runner)
}

/// Spawns a component that publishes on `buses`, so that it stops before every listener of those buses.
fn spawn_producer<F>(
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
    buses: &[Bus],
    reservation: Reservation,
    runner: F,
) -> Result<(), Error>
where
    F: Future<Output = ComponentResult> + Send + 'static,
{
    let consumers = flows.consumers_of(buses);
    job_manager
        .spawn_with(reservation, StopBefore(&consumers), runner)
        .map_err(|source| Error::ComponentRegistration { source })
}

async fn component_runner(runner: impl Future<Output = torrust_tracker_events::shutdown::Completion>) -> ComponentResult {
    Ok(match runner.await {
        torrust_tracker_events::shutdown::Completion::Completed => ComponentCompletion::Completed,
        torrust_tracker_events::shutdown::Completion::Cancelled => ComponentCompletion::Cancelled,
    })
}

async fn start_the_udp_instances(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
) -> Result<(), Error> {
    let udp_trackers = config.udp_trackers.as_ref().ok_or_else(|| Error::ServiceStartup {
        service: "UDP tracker",
        source: "UDP tracker startup requires at least one configured UDP tracker".into(),
    })?;

    let connection_id_validation = connection_id_validation_policy(config);

    for (idx, udp_tracker_config) in udp_trackers.iter().enumerate() {
        start_udp_instance(
            idx,
            udp_tracker_config,
            connection_id_validation,
            app_container,
            job_manager,
            flows,
        )
        .await?;
    }

    Ok(())
}

async fn start_udp_instance(
    idx: usize,
    udp_tracker_config: &UdpTracker,
    connection_id_validation: ConnectionIdValidationPolicy,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
) -> Result<(), Error> {
    let (configuration_instance_id, udp_tracker_container) =
        app_container
            .udp_tracker_container(idx)
            .map_err(|source| Error::MissingServiceContainer {
                service: "UDP tracker",
                source,
            })?;
    let udp_tracker_server_container = app_container.udp_tracker_server_container();
    let reservation = job_manager.reserve(format!("udp_instance_{}_{}", idx, udp_tracker_config.bind_address));

    let runner = udp_tracker::start_job(
        udp_tracker_container,
        udp_tracker_server_container,
        app_container.registar.give_form(),
        RuntimeServiceMetadata::new(configuration_instance_id)
            .with_public_url(udp_tracker_config.public_url.as_ref().map(|url| url.as_url().clone())),
        connection_id_validation,
        reservation.cancellation_token(),
        app_container.application_config.shutdown.udp_request_drain(),
    )
    .await
    .map_err(|source| Error::ServiceStartup {
        service: "UDP tracker",
        source: Box::new(source),
    })?;

    spawn_producer(
        job_manager,
        flows,
        &[Bus::UdpCore, Bus::UdpServer, Bus::Swarm],
        reservation,
        runner,
    )
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
    flows: &mut EventFlows,
) -> Result<(), Error> {
    if let Some(http_trackers) = &config.http_trackers {
        for (idx, http_tracker_config) in http_trackers.iter().enumerate() {
            start_http_instance(idx, http_tracker_config, app_container, job_manager, flows).await?;
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
    flows: &mut EventFlows,
) -> Result<(), Error> {
    let (configuration_instance_id, http_tracker_container) =
        app_container
            .http_tracker_container(idx)
            .map_err(|source| Error::MissingServiceContainer {
                service: "HTTP tracker",
                source,
            })?;
    let reservation = job_manager.reserve(format!("http_instance_{}_{}", idx, http_tracker_config.bind_address));

    if let Some(runner) = http_tracker::start_job(
        http_tracker_container,
        app_container.registar.give_form(),
        RuntimeServiceMetadata::new(configuration_instance_id)
            .with_public_url(http_tracker_config.public_url.as_ref().map(|url| url.as_url().clone())),
        torrust_tracker_axum_http_server::Version::V1,
        reservation.cancellation_token(),
        app_container.application_config.shutdown.http_drain(),
    )
    .await
    .map_err(|source| Error::ServiceStartup {
        service: "HTTP tracker",
        source: Box::new(source),
    })? {
        spawn_producer(job_manager, flows, &[Bus::HttpCore, Bus::Swarm], reservation, runner)?;
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
        let reservation = job_manager.reserve("http_api");

        if let Some(runner) = tracker_apis::start_job(
            http_api_container,
            app_container.registar.give_form(),
            RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0))
                .with_public_url(http_api_config.public_url.as_ref().map(|url| url.as_url().clone())),
            torrust_tracker_axum_rest_api_server::Version::V1,
            reservation.cancellation_token(),
            app_container.application_config.shutdown.http_drain(),
        )
        .await
        .map_err(|source| Error::ServiceStartup {
            service: "tracker API",
            source: Box::new(source),
        })? {
            spawn_component(job_manager, reservation, runner)?;
        }
    } else {
        tracing::info!("No API block in configuration");
    }
    Ok(())
}

fn start_torrent_cleanup(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
    flows: &mut EventFlows,
) -> Result<(), Error> {
    if config.core.inactive_peer_cleanup_interval > 0 {
        let reservation = job_manager.reserve("torrent_cleanup");
        let runner = torrent_cleanup::run_job(
            config.core.clone(),
            app_container.tracker_core_container.torrents_manager.clone(),
            reservation.cancellation_token(),
        );
        spawn_producer(job_manager, flows, &[Bus::Swarm], reservation, component_runner(runner))?;
    }

    Ok(())
}

fn start_peers_inactivity_update(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    if config.core.tracker_usage_statistics {
        let reservation = job_manager.reserve("peers_inactivity_update");
        let runner = activity_metrics_updater::run_job(config, app_container, reservation.cancellation_token());
        spawn_component(job_manager, reservation, component_runner(runner))?;
    } else {
        tracing::info!("Peers inactivity update job is disabled.");
    }

    Ok(())
}

async fn start_health_check_api(
    config: &Configuration,
    app_container: &Arc<AppContainer>,
    job_manager: &mut JobManager,
) -> Result<(), Error> {
    let reservation = job_manager.reserve("health_check_api");
    let runner = health_check_api::start_job(
        &config.health_check_api,
        app_container.registar.as_ref().clone(),
        reservation.cancellation_token(),
        app_container.application_config.shutdown.http_drain(),
    )
    .await
    .map_err(|source| Error::ServiceStartup {
        service: "health check API",
        source: Box::new(source),
    })?;

    spawn_component(job_manager, reservation, runner)
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, TcpListener, UdpSocket};
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::sync::oneshot;
    use tokio_util::sync::CancellationToken;
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;
    use torrust_tracker_primitives::peer::Peer;
    use torrust_tracker_swarm_coordination_registry::event::Event;
    use torrust_tracker_test_helpers::configuration::ephemeral_public;

    use super::{
        Error, EventFlows, component_runner, connection_id_validation_policy, load_data_from_database, run_after_setup,
        should_start_udp_tracker_services, start_health_check_api, start_http_instance, start_jobs_with_manager,
        start_peers_inactivity_update, start_swarm_coordination_registry_event_listener, start_the_http_api,
        start_torrent_cleanup, start_tracker_core_in_memory_event_listener,
        start_tracker_core_persistent_completed_statistics_event_listener, start_udp_instance,
    };
    use crate::bootstrap::app::initialize_global_services;
    use crate::bootstrap::jobs::manager::{ComponentCompletion, JobManager, JobOutcome, JobStatus, StopBefore};
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

        let listener = torrust_tracker_core::statistics::event::listener::run_in_memory_event_listener_unspawned(
            app_container.swarm_coordination_registry_container.event_bus.receiver(),
            cancellation_token.clone(),
            app_container.tracker_core_container.stats_repository.clone(),
            torrust_tracker_events::drain::DEFAULT_DRAIN_TIMEOUT,
        );

        cancellation_token.cancel();
        let completion = tokio::time::timeout(Duration::from_secs(1), listener)
            .await
            .expect("in-memory listener should stop after cancellation");

        assert_eq!(completion, torrust_tracker_events::shutdown::Completion::Cancelled);
    }

    #[tokio::test]
    async fn it_should_register_torrent_cleanup_as_a_direct_cancelled_component() {
        // Arrange
        let configuration = Configuration {
            core: Core {
                inactive_peer_cleanup_interval: 24 * 60 * 60,
                ..Core::default()
            },
            ..Configuration::default()
        };
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let mut job_manager = JobManager::new();
        start_torrent_cleanup(&configuration, &app_container, &mut job_manager, &mut EventFlows::default())
            .expect("torrent cleanup should be registered through application bootstrap");

        // Act
        job_manager.cancel();
        let outcomes = job_manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: "torrent_cleanup".to_string(),
                status: JobStatus::Cancelled,
            }]
        );
    }

    #[tokio::test]
    async fn it_should_register_peers_inactivity_update_as_a_direct_cancelled_component() {
        // Arrange
        let configuration = Configuration {
            core: Core {
                tracker_usage_statistics: true,
                ..Core::default()
            },
            ..Configuration::default()
        };
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let mut job_manager = JobManager::new();
        start_peers_inactivity_update(&configuration, &app_container, &mut job_manager)
            .expect("peers inactivity update should be registered through application bootstrap");

        // Act
        job_manager.cancel();
        let outcomes = job_manager.wait_for_all(Duration::from_secs(5)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: "peers_inactivity_update".to_string(),
                status: JobStatus::Cancelled,
            }]
        );
    }

    #[tokio::test]
    async fn it_should_cancel_the_http_tracker_component_through_the_job_manager() {
        // Arrange
        let configuration = ephemeral_public();
        let http_tracker_config = &configuration.http_trackers.as_ref().expect("test configuration enables HTTP")[0];
        initialize_global_services(&configuration);
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let mut job_manager = JobManager::new();
        start_http_instance(
            0,
            http_tracker_config,
            &app_container,
            &mut job_manager,
            &mut EventFlows::default(),
        )
        .await
        .expect("HTTP tracker should start through application bootstrap");

        // Act
        job_manager.cancel();
        let outcomes = job_manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: format!("http_instance_0_{}", http_tracker_config.bind_address),
                status: JobStatus::Cancelled,
            }]
        );
    }

    #[tokio::test]
    async fn it_should_cancel_the_rest_api_component_through_the_job_manager() {
        // Arrange
        let configuration = ephemeral_public();
        initialize_global_services(&configuration);
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let mut job_manager = JobManager::new();
        start_the_http_api(&configuration, &app_container, &mut job_manager)
            .await
            .expect("REST API should start through application bootstrap");

        // Act
        job_manager.cancel();
        let outcomes = job_manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: "http_api".to_string(),
                status: JobStatus::Cancelled,
            }]
        );
    }

    #[tokio::test]
    async fn it_should_cancel_the_udp_tracker_component_through_the_job_manager() {
        // Arrange
        let mut configuration = torrust_tracker_test_helpers::configuration::ephemeral_public();
        configuration.udp_trackers.as_mut().expect("test configuration enables UDP")[0].bind_address = reserve_udp_address();
        let udp_tracker_config = configuration.udp_trackers.as_ref().expect("test configuration enables UDP")[0].clone();
        initialize_global_services(&configuration);
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let mut job_manager = JobManager::new();
        start_udp_instance(
            0,
            &udp_tracker_config,
            connection_id_validation_policy(&configuration),
            &app_container,
            &mut job_manager,
            &mut EventFlows::default(),
        )
        .await
        .expect("UDP tracker should start through application bootstrap");

        // Act
        job_manager.cancel();
        let outcomes = job_manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: format!("udp_instance_0_{}", udp_tracker_config.bind_address),
                status: JobStatus::Cancelled,
            }]
        );
    }

    #[tokio::test]
    async fn it_should_cancel_the_health_check_api_component_through_the_job_manager() {
        // Arrange
        let configuration = ephemeral_public();
        initialize_global_services(&configuration);
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let mut job_manager = JobManager::new();
        start_health_check_api(&configuration, &app_container, &mut job_manager)
            .await
            .expect("health check API should start through application bootstrap");

        // Act
        job_manager.cancel();
        let outcomes = job_manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: "health_check_api".to_string(),
                status: JobStatus::Cancelled,
            }]
        );
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
    async fn it_should_release_udp_listener_before_returning_from_start_after_setup_when_later_http_startup_fails() {
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

    #[tokio::test]
    async fn it_should_stop_every_started_component_cooperatively_and_each_producer_before_its_listeners_when_the_manager_is_cancelled()
     {
        // Arrange
        let mut configuration = ephemeral_public();
        configuration.core.tracker_usage_statistics = true;
        configuration.core.tracker_policy.persistent_torrent_completed_stat = true;
        configuration.core.inactive_peer_cleanup_interval = 24 * 60 * 60;
        let udp_bind_address = configuration.udp_trackers.as_ref().expect("test configuration enables UDP")[0].bind_address;
        let http_bind_address = configuration.http_trackers.as_ref().expect("test configuration enables HTTP")[0].bind_address;
        initialize_global_services(&configuration);
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let mut job_manager = JobManager::new();
        start_jobs_with_manager(&configuration, &app_container, &mut job_manager)
            .await
            .expect("every component should start through application bootstrap");

        // Act
        let declared_stop_order = sorted_stop_order(&job_manager);
        job_manager.cancel();
        let mut outcomes = job_manager.wait_for_all(Duration::from_secs(5)).await;

        // Assert
        // Each listener waits for exactly the components that publish on its bus.
        let udp_instance = format!("udp_instance_0_{udp_bind_address}");
        let http_instance = format!("http_instance_0_{http_bind_address}");
        let swarm_producers = [udp_instance.as_str(), http_instance.as_str(), "torrent_cleanup"];
        assert_eq!(
            declared_stop_order,
            expected_stop_order(&[
                ("swarm_coordination_registry_event_listener", &swarm_producers),
                ("tracker_core_in_memory_event_listener", &swarm_producers),
                (
                    "tracker_core_persistent_completed_statistics_event_listener",
                    &swarm_producers
                ),
                ("http_core_event_listener", &[http_instance.as_str()]),
                ("udp_core_event_listener", &[udp_instance.as_str()]),
                ("udp_server_stats_event_listener", &[udp_instance.as_str()]),
                ("udp_server_banning_event_listener", &[udp_instance.as_str()]),
            ]),
            "every listener must wait for each component that publishes on its bus, and nothing else"
        );

        // Every producer completes before each listener of the buses it publishes on; the outcomes
        // are in completion order until they are sorted below.
        let completion_order: Vec<&str> = outcomes.iter().map(|outcome| outcome.name.as_str()).collect();
        let swarm_listeners = [
            "swarm_coordination_registry_event_listener",
            "tracker_core_in_memory_event_listener",
            "tracker_core_persistent_completed_statistics_event_listener",
        ];
        let udp_server_listeners = ["udp_server_stats_event_listener", "udp_server_banning_event_listener"];
        assert_completes_before_every(&completion_order, &http_instance, &["http_core_event_listener"]);
        assert_completes_before_every(&completion_order, &http_instance, &swarm_listeners);
        assert_completes_before_every(&completion_order, &udp_instance, &["udp_core_event_listener"]);
        assert_completes_before_every(&completion_order, &udp_instance, &udp_server_listeners);
        assert_completes_before_every(&completion_order, &udp_instance, &swarm_listeners);
        assert_completes_before_every(&completion_order, "torrent_cleanup", &swarm_listeners);

        outcomes.sort_by(|left, right| left.name.cmp(&right.name));
        let cancelled = |name: String| JobOutcome {
            name,
            status: JobStatus::Cancelled,
        };
        let mut expected = vec![
            cancelled("swarm_coordination_registry_event_listener".to_owned()),
            cancelled("tracker_core_in_memory_event_listener".to_owned()),
            cancelled("tracker_core_persistent_completed_statistics_event_listener".to_owned()),
            cancelled("http_core_event_listener".to_owned()),
            cancelled("udp_core_event_listener".to_owned()),
            cancelled("udp_server_stats_event_listener".to_owned()),
            cancelled("udp_server_banning_event_listener".to_owned()),
            JobOutcome {
                name: "udp_ban_cleanup".to_owned(),
                status: JobStatus::Completed,
            },
            cancelled(format!("udp_instance_0_{udp_bind_address}")),
            cancelled(format!("http_instance_0_{http_bind_address}")),
            cancelled("torrent_cleanup".to_owned()),
            cancelled("peers_inactivity_update".to_owned()),
            cancelled("http_api".to_owned()),
            cancelled("health_check_api".to_owned()),
        ];
        expected.sort_by(|left, right| left.name.cmp(&right.name));
        assert_eq!(
            outcomes, expected,
            "every started component must stop through the token `cancel()` cancels; an `Aborted` status names a component that was built with another token"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_count_a_download_published_during_shutdown_by_a_producer_that_stops_before_the_listener() {
        // Arrange
        // The in-memory tracker-core listener consumes the real swarm event bus. Two producers stop
        // before it, and each publishes one completed download on that bus only when the test releases
        // it, after shutdown began, like a request that finishes while the server drains.
        let mut configuration = Configuration::default();
        configuration.core.tracker_usage_statistics = true;
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let mut job_manager = JobManager::new();
        let listener_reservation = job_manager.reserve("tracker_core_in_memory_event_listener");
        let listener_id = listener_reservation.id();
        let listener = super::jobs::tracker_core::run_in_memory_event_listener(
            &configuration,
            &app_container,
            listener_reservation.cancellation_token(),
        )
        .expect("the listener starts when tracker usage statistics are enabled");
        job_manager
            .spawn(listener_reservation, component_runner(listener))
            .expect("the reservation was issued by this manager");
        let mut release_producers = Vec::new();
        for name in ["first_producer", "second_producer"] {
            let reservation = job_manager.reserve(name);
            let sender = app_container
                .swarm_coordination_registry_container
                .event_bus
                .sender()
                .expect("the swarm event bus is enabled");
            let (release, released) = oneshot::channel::<()>();
            release_producers.push(release);
            job_manager
                .spawn_with(reservation, StopBefore(&[listener_id]), async move {
                    let _ = released.await;
                    let _ = sender.send(completed_download_event()).await;
                    Ok(ComponentCompletion::Completed)
                })
                .expect("the listener was reserved before the producers");
        }

        // Act
        // Shutdown starts while both producers run, and then each producer publishes and completes.
        job_manager.cancel();
        let shutdown = tokio::spawn(job_manager.wait_for_all(Duration::from_secs(5)));
        tokio::time::sleep(Duration::from_millis(1)).await;
        for release in release_producers {
            release.send(()).expect("the producer is waiting for its release");
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        let outcomes = shutdown.await.expect("the shutdown task must not panic");

        // Assert
        // Both downloads, published after shutdown began, are counted, and the listener stopped last.
        let counted = app_container
            .tracker_core_container
            .stats_repository
            .get_torrents_downloads_in_session_total()
            .await;
        assert_eq!(
            counted, 2,
            "a download published by a producer after shutdown began is lost when the listener stops before the producer completes"
        );
        assert_eq!(
            outcomes.last(),
            Some(&JobOutcome {
                name: "tracker_core_in_memory_event_listener".to_owned(),
                status: JobStatus::Cancelled,
            }),
            "the listener must complete after both producers"
        );
    }

    #[tokio::test]
    async fn it_should_subscribe_every_started_swarm_listener_before_its_task_runs_or_any_producer_starts() {
        // Arrange
        // On a current-thread runtime a spawned listener task cannot run until the test awaits something
        // that yields, so a receiver that exists now was created by bootstrap, not by the listener task.
        let mut configuration = ephemeral_public();
        configuration.core.tracker_usage_statistics = true;
        configuration.core.tracker_policy.persistent_torrent_completed_stat = true;
        initialize_global_services(&configuration);
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let mut job_manager = JobManager::new();
        let mut flows = EventFlows::default();

        // Act
        // Bootstrap starts the three swarm listeners, and then an event is published on the swarm bus.
        start_swarm_coordination_registry_event_listener(&configuration, &app_container, &mut job_manager, &mut flows)
            .expect("the swarm listener starts");
        start_tracker_core_in_memory_event_listener(&configuration, &app_container, &mut job_manager, &mut flows)
            .expect("the in-memory listener starts");
        start_tracker_core_persistent_completed_statistics_event_listener(
            &configuration,
            &app_container,
            &mut job_manager,
            &mut flows,
        )
        .expect("the persistent listener starts");
        let delivered_to = app_container
            .swarm_coordination_registry_container
            .event_bus
            .sender()
            .expect("the swarm event bus is enabled")
            .send(completed_download_event())
            .await;

        // Assert
        // The event reached the receivers of all three listeners: none subscribes later than the first event.
        assert!(
            matches!(delivered_to, Some(Ok(3))),
            "all three swarm listeners must be subscribed when the first event is published, got {delivered_to:?}"
        );
        job_manager.cancel();
        job_manager.wait_for_all(Duration::from_secs(5)).await;
    }

    /// The components that wait for others, with what each waits for, sorted by name.
    fn sorted_stop_order(job_manager: &JobManager) -> Vec<(String, Vec<String>)> {
        expected_stop_order_from(
            job_manager
                .waiting_components()
                .into_iter()
                .map(|component| (component.name, component.waiting_for)),
        )
    }

    fn expected_stop_order(entries: &[(&str, &[&str])]) -> Vec<(String, Vec<String>)> {
        expected_stop_order_from(entries.iter().map(|(listener, producers)| {
            (
                (*listener).to_owned(),
                producers.iter().map(|producer| (*producer).to_owned()).collect(),
            )
        }))
    }

    fn expected_stop_order_from(entries: impl Iterator<Item = (String, Vec<String>)>) -> Vec<(String, Vec<String>)> {
        let mut order: Vec<(String, Vec<String>)> = entries
            .map(|(name, mut waiting_for)| {
                waiting_for.sort();
                (name, waiting_for)
            })
            .collect();
        order.sort();
        order
    }

    /// A completed download, which the in-memory tracker-core listener counts.
    fn completed_download_event() -> Event {
        Event::PeerDownloadCompleted {
            info_hash: <_>::default(),
            peer: Peer::default(),
        }
    }

    /// Asserts that `producer` completed before each of `consumers` in `completion_order`.
    fn assert_completes_before_every(completion_order: &[&str], producer: &str, consumers: &[&str]) {
        let position = |name: &str| {
            completion_order
                .iter()
                .position(|completed| *completed == name)
                .unwrap_or_else(|| panic!("component '{name}' must be in the shutdown outcomes: {completion_order:?}"))
        };
        for consumer in consumers {
            assert!(
                position(producer) < position(consumer),
                "'{producer}' publishes on a bus that '{consumer}' consumes, so it must complete first: {completion_order:?}"
            );
        }
    }

    fn reserve_udp_address() -> SocketAddr {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("reserve UDP listener address");
        socket.local_addr().expect("read UDP listener address")
    }
}
