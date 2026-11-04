//! Minimal UDP-only public tracker started the way the `torrust-tracker` binary starts it.
//!
//! This example **simulates production as closely as possible**: it does not use the
//! `torrust_tracker_udp_server::testing` module. It composes the containers
//! itself, starts the server like `start_job` in `src/bootstrap/jobs/udp_tracker.rs`
//! (`Server::new(Spawner::new(bind_to)).start_with_cancellation(...)`), runs the three event
//! listeners, and stops them in the order the application does: the receive loop first, then
//! the listeners. Unlike the application, it spawns the listeners itself instead of handing them
//! to a supervisor, and it composes the containers for one instance only.
//!
//! The package's own targets, examples included, get the registry crate that composes the
//! containers through the `testing` feature, which the package enables for them through a
//! path-only self dev-dependency. The example never imports the `testing` module.
//!
//! Its sibling, `udp_only_public_tracker`, shows the same tracker through the test
//! environment and **follows the conventions used in testing**. Compare the two to see what
//! the environment hides.
//!
//! > **Note**: production startup may become more like the testing style in the future. An end
//! > user who prefers the object-oriented testing-style version can adopt it, at the price of
//! > enabling the `testing` feature, which is outside the semver guarantees of this package.
//!
//! ## How to run
//!
//! ```bash
//! cargo run -p torrust-tracker-udp-server --example udp_only_public_tracker_production_style
//! ```
//!
//! Stop it with Ctrl-C (SIGINT) or, on Unix, SIGTERM (`kill <pid>`). When sending signals by
//! PID, run the built binary (`target/debug/examples/udp_only_public_tracker_production_style`)
//! so the signal reaches this process rather than `cargo`.

#![allow(clippy::print_stdout, reason = "runnable public-tracker example prints service information")]

use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

#[cfg(unix)]
use tokio::signal::unix::{SignalKind, signal};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::Registar;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::database::Database;
use torrust_tracker_configuration::v3_0_0::network::Network;
use torrust_tracker_configuration::v3_0_0::shutdown::Shutdown;
use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;
use torrust_tracker_configuration::v3_0_0::udp_tracker_server::UdpTrackerServer;
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_server::container::UdpTrackerServerContainer;
use torrust_tracker_udp_server::server::Server;
use torrust_tracker_udp_server::server::spawner::Spawner;

#[tokio::main]
async fn main() {
    // Temporary database file, unique per process so concurrent runs and a run that crashed before its
    // cleanup do not share state; removed on exit.
    let db_path = std::env::temp_dir().join(format!("torrust-udp-example-production-style-{}.db", std::process::id()));

    // Build Core and UdpTracker directly — no full Configuration aggregate needed.
    // Public tracker: peers do not need an authentication key.
    let core = Arc::new(Core {
        private: false,
        database: Some(Database::Sqlite3 {
            path: db_path.to_string_lossy().into_owned(),
        }),
        ..Core::default()
    });

    let udp_tracker = Arc::new(UdpTracker {
        bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
        cookie_lifetime: Duration::from_secs(120),
        tracker_usage_statistics: false,
        public_url: None,
        network: Network::default(),
    });

    let udp_tracker_server = UdpTrackerServer::default();
    let instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
    let service_metadata = RuntimeServiceMetadata::new(instance_id);

    torrust_clock::initialize_static();

    let (udp_tracker_core_container, udp_tracker_server_container) = compose_containers(
        &core,
        &udp_tracker,
        udp_tracker_server.max_connection_id_errors_per_ip,
        instance_id,
    )
    .await;

    // Subscribe before the server can publish.
    let udp_core_events = udp_tracker_core_container.event_bus.receiver();
    let udp_server_statistics_events = udp_tracker_server_container.event_bus.receiver();
    let udp_server_banning_events = udp_tracker_server_container.event_bus.receiver();

    // The receive loop and the listeners have independent tokens, like components of the application.
    let server_token = CancellationToken::new();
    let listeners_token = CancellationToken::new();

    let bind_to = udp_tracker_core_container.udp_tracker_config.bind_address;
    let cookie_lifetime = udp_tracker_core_container.udp_tracker_config.cookie_lifetime;
    let registar = Registar::default();
    // Like the application, the listeners filter metrics by the configuration of each instance.
    let metrics_policy: BTreeMap<_, _> = [(instance_id, udp_tracker.tracker_usage_statistics)].into();
    let listener_drain = Shutdown::default().event_listener_drain();

    let server = Server::new(Spawner::new(bind_to))
        .start_with_cancellation(
            udp_tracker_core_container.clone(),
            udp_tracker_server_container.clone(),
            registar.give_form(),
            service_metadata,
            cookie_lifetime,
            ConnectionIdValidationPolicy::Strict,
            Shutdown::default().udp_request_drain(),
            server_token.clone(),
        )
        .await
        .expect("Failed to start the UDP tracker server");

    let core_statistics_listener = torrust_tracker_udp_core::statistics::event::listener::run_event_listener(
        udp_core_events,
        listeners_token.clone(),
        &udp_tracker_core_container.stats_repository,
        metrics_policy.clone(),
        listener_drain,
    );
    let server_statistics_listener = torrust_tracker_udp_server::statistics::event::listener::run_event_listener(
        udp_server_statistics_events,
        listeners_token.clone(),
        &udp_tracker_server_container.stats_repository,
        metrics_policy.clone(),
        listener_drain,
    );
    let server_banning_listener = torrust_tracker_udp_server::banning::event::listener::run_event_listener(
        udp_server_banning_events,
        listeners_token.clone(),
        &udp_tracker_core_container.ban_service,
        &udp_tracker_server_container.banned_ips_publication_order,
        &udp_tracker_server_container.stats_repository,
        listener_drain,
    );

    // On Unix the handlers are installed here, before readiness is printed, so a later signal is never lost.
    let shutdown_signal = install_shutdown_signal_handlers();

    println!("Listening on {}", server.local_addr);
    println!("Press Ctrl-C (or send SIGTERM on Unix) to stop.");

    let signal = shutdown_signal.await;
    println!("\nReceived {signal}. Shutting down...");

    // Stop the receive loop first, so the events of the requests it drains are still processed.
    server_token.cancel();
    match server.task.await {
        Ok(Ok(())) => {}
        Ok(Err(error)) => println!("The UDP receive loop stopped with an error: {error}"),
        Err(error) => println!("The UDP receive loop task failed: {error}"),
    }

    stop_event_listeners(
        listeners_token,
        [core_statistics_listener, server_statistics_listener, server_banning_listener],
    )
    .await;

    // Best-effort cleanup of the temporary database file.
    std::fs::remove_file(&db_path).ok();

    println!("Stopped.");
}

/// Composes the containers in the order the application composes them.
async fn compose_containers(
    core: &Arc<Core>,
    udp_tracker: &Arc<UdpTracker>,
    max_connection_id_errors_per_ip: u32,
    instance_id: ConfigurationInstanceId,
) -> (Arc<UdpTrackerCoreContainer>, Arc<UdpTrackerServerContainer>) {
    let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
        core.tracker_usage_statistics.into(),
    ));

    let tracker_core_container = Arc::new(
        TrackerCoreContainer::initialize_from(core, &swarm_coordination_registry_container, core.database.as_ref())
            .await
            .expect("the tracker core container requires persistence"),
    );

    let udp_tracker_core_container = UdpTrackerCoreContainer::initialize_from_tracker_core(
        &tracker_core_container,
        udp_tracker,
        max_connection_id_errors_per_ip,
        instance_id,
    );

    let udp_tracker_server_container = UdpTrackerServerContainer::initialize(core);

    (udp_tracker_core_container, udp_tracker_server_container)
}

/// Cancels the listeners and joins them, reporting every failure.
async fn stop_event_listeners(token: CancellationToken, listeners: impl IntoIterator<Item = JoinHandle<()>>) {
    token.cancel();
    for listener in listeners {
        if let Err(error) = listener.await {
            println!("An event listener task failed: {error}");
        }
    }
}

/// Installs the shutdown signal handlers and returns a future that resolves
/// with the name of the first signal received.
#[cfg(unix)]
fn install_shutdown_signal_handlers() -> impl Future<Output = &'static str> {
    let mut interrupt = signal(SignalKind::interrupt()).expect("failed to install SIGINT handler");
    let mut terminate = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");

    async move {
        tokio::select! {
            _ = interrupt.recv() => "SIGINT",
            _ = terminate.recv() => "SIGTERM",
        }
    }
}

/// Returns a future that resolves when Ctrl-C is received. `ctrl_c()` registers
/// its handler on first poll, so it is not installed before readiness is printed.
#[cfg(not(unix))]
fn install_shutdown_signal_handlers() -> impl Future<Output = &'static str> {
    async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl-C handler");
        "Ctrl-C"
    }
}
