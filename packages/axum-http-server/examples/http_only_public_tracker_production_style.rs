//! Minimal HTTP-only public tracker started the way the `torrust-tracker` binary starts it.
//!
//! This example **simulates production as closely as possible**: it does not use the
//! `torrust_tracker_axum_http_server::testing` module. It composes the containers itself, starts
//! the server like `start_job` in `src/bootstrap/jobs/http_tracker.rs`
//! (`HttpServer::new(Launcher::new(..)).start_with_cancellation(..)`), runs the statistics event
//! listener, and stops them in the order the application does: the server first, then the
//! listener. Unlike the application, it spawns the listener itself instead of handing it to a
//! supervisor, and it composes the containers for one instance only.
//!
//! The package's own targets, examples included, get the registry crate that composes the
//! containers through the `testing` feature, which the package enables for them through a
//! path-only self dev-dependency. The example never imports the `testing` module.
//!
//! Its sibling, `http_only_public_tracker`, shows the same tracker through the test
//! environment and **follows the conventions used in testing**. Compare the two to see what the
//! environment hides.
//!
//! > **Note**: production startup may become more like the testing style in the future. An end
//! > user who prefers the object-oriented testing-style version can adopt it, at the price of
//! > enabling the `testing` feature, which is outside the semver guarantees of this package.
//!
//! ## Ownership
//!
//! `main` owns everything it starts: two cancellation tokens (one for the server, one for the
//! listener), held through drop guards so an early return or a panic cancels both, the server's
//! join handles and the listener's join handle. Only `main` subscribes to OS signals. The listener
//! is spawned only after the server started, so a failed start leaves no detached task.
//!
//! Two absolute deadlines bound the example: one for startup and a fresh one computed when
//! shutdown begins. Waiting for the shutdown signal is the normal running state and is not bounded.
//!
//! ## How to run
//!
//! ```bash
//! cargo run -p torrust-tracker-axum-http-server --example http_only_public_tracker_production_style
//! ```
//!
//! Stop it with Ctrl-C (SIGINT) or, on Unix, SIGTERM (`kill <pid>`). When sending signals by
//! PID, run the built binary (`target/debug/examples/http_only_public_tracker_production_style`)
//! so the signal reaches this process rather than `cargo`.

#![allow(clippy::print_stdout, reason = "runnable public-tracker example prints service information")]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

#[cfg(unix)]
use tokio::signal::unix::{SignalKind, signal};
use tokio::task::JoinHandle;
use tokio::time::{Instant, timeout_at};
use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::Registar;
use torrust_tracker_axum_http_server::server::{HttpServer, Launcher};
use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::database::Database;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_configuration::v3_0_0::network::Network;
use torrust_tracker_configuration::v3_0_0::shutdown::Shutdown;
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

/// Longest the startup (composition and bind) may take.
const STARTUP_BUDGET: Duration = Duration::from_secs(30);

/// Longest the stop sequence (server join, then listener join) may take, after the drain timeout.
const SHUTDOWN_BUDGET: Duration = Duration::from_secs(30);

#[tokio::main]
async fn main() -> ExitCode {
    // Temporary database file, unique per process so concurrent runs and a run that crashed before its
    // cleanup do not share state; removed on exit.
    let db_path = std::env::temp_dir().join(format!("torrust-http-example-production-style-{}.db", std::process::id()));

    let exit_code = run(&db_path).await;

    // Best-effort cleanup of the temporary database file.
    std::fs::remove_file(&db_path).ok();

    exit_code
}

async fn run(db_path: &std::path::Path) -> ExitCode {
    // Build Core and HttpTracker directly — no full Configuration aggregate needed.
    // Public tracker: peers do not need an authentication key.
    let core = Arc::new(Core {
        private: false,
        database: Some(Database::Sqlite3 {
            path: db_path.to_string_lossy().into_owned(),
        }),
        ..Core::default()
    });

    // Port 0 lets the OS assign a free port. TLS is disabled for simplicity.
    let http_tracker = Arc::new(HttpTracker {
        bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
        tls_config: None,
        tracker_usage_statistics: false,
        use_ip_from_query_string: false,
        public_url: None,
        network: Network::default(),
    });

    let instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
    let service_metadata = RuntimeServiceMetadata::new(instance_id);

    // The receive side and the listener have independent tokens, like components of the application.
    // The guards cancel both if `run` returns early or panics.
    let server_token = CancellationToken::new();
    let listeners_token = CancellationToken::new();
    let cancel_server_on_drop = server_token.clone().drop_guard();
    let cancel_listeners_on_drop = listeners_token.clone().drop_guard();

    // Startup deadline, computed once when the phase begins.
    let startup_deadline = Instant::now() + STARTUP_BUDGET;

    torrust_clock::initialize_static();

    let Ok(container) = timeout_at(startup_deadline, compose_container(&core, &http_tracker, instance_id)).await else {
        println!("Timed out composing the tracker containers.");
        return ExitCode::FAILURE;
    };

    // Subscribe before the server can publish.
    let events = container.event_bus.receiver();

    let registar = Registar::default();
    let launcher = Launcher::new(
        container.http_tracker_config.bind_address,
        None,
        container.http_tracker_config.network.ipv6_v6only,
        Shutdown::default().http_drain(),
    );

    let start = HttpServer::new(launcher).start_with_cancellation(
        container.clone(),
        registar.give_form(),
        service_metadata,
        server_token.clone(),
    );

    let server = match timeout_at(startup_deadline, start).await {
        Ok(Ok(server)) => server,
        Ok(Err(error)) => {
            println!("Failed to start the HTTP tracker server: {error}");
            return ExitCode::FAILURE;
        }
        Err(_) => {
            println!("Timed out starting the HTTP tracker server.");
            return ExitCode::FAILURE;
        }
    };

    // Spawn the listener only after a successful start, so a failed start leaves no detached task.
    let statistics_listener = torrust_tracker_http_core::statistics::event::listener::run_event_listener(
        events,
        listeners_token.clone(),
        &container.stats_repository,
        [(instance_id, http_tracker.tracker_usage_statistics)].into(),
        Shutdown::default().event_listener_drain(),
    );

    // On Unix the handlers are installed here, before readiness is printed, so a later signal is never lost.
    let shutdown_signal = install_shutdown_signal_handlers();

    println!("Listening on {}", server.binding);
    println!("Press Ctrl-C (or send SIGTERM on Unix) to stop.");

    // Not bounded: waiting for the signal is the normal running state.
    let signal = shutdown_signal.await;
    println!("\nReceived {signal}. Shutting down...");

    // A fresh deadline for the stop sequence, so a long run does not shrink its budget.
    let shutdown_deadline = Instant::now() + SHUTDOWN_BUDGET;

    let mut failed = false;

    // Stop the server first, so the events of the requests it drains are still processed.
    cancel_server_on_drop.disarm().cancel();
    failed |= !join_server(shutdown_deadline, server.task, server.shutdown_controller).await;

    // Then stop the listener.
    cancel_listeners_on_drop.disarm().cancel();
    match timeout_at(shutdown_deadline, statistics_listener).await {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            println!("The statistics event listener task failed: {error}");
            failed = true;
        }
        Err(_) => {
            println!("Timed out waiting for the statistics event listener.");
            failed = true;
        }
    }

    println!("Stopped.");

    if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

/// Composes the containers in the order the application composes them.
async fn compose_container(
    core: &Arc<Core>,
    http_tracker: &Arc<HttpTracker>,
    instance_id: ConfigurationInstanceId,
) -> Arc<HttpTrackerCoreContainer> {
    let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
        core.tracker_usage_statistics.into(),
    ));

    let tracker_core_container = Arc::new(
        TrackerCoreContainer::initialize_from(core, &swarm_coordination_registry_container, core.database.as_ref())
            .await
            .expect("the tracker core container requires persistence"),
    );

    HttpTrackerCoreContainer::initialize_from_tracker_core(&tracker_core_container, http_tracker, instance_id)
}

/// Joins the server task and its drain controller, reporting every failure. Returns whether all went well.
async fn join_server(
    deadline: Instant,
    task: JoinHandle<Launcher>,
    shutdown_controller: JoinHandle<GracefulShutdownOutcome>,
) -> bool {
    let (task_result, controller_result) = tokio::join!(timeout_at(deadline, task), timeout_at(deadline, shutdown_controller));
    let mut ok = true;

    match task_result {
        Ok(Ok(_launcher)) => {}
        Ok(Err(error)) => {
            println!("The HTTP server task failed: {error}");
            ok = false;
        }
        Err(_) => {
            println!("Timed out waiting for the HTTP server task.");
            ok = false;
        }
    }

    match controller_result {
        Ok(Ok(GracefulShutdownOutcome::Drained)) => {}
        Ok(Ok(GracefulShutdownOutcome::TimedOut)) => {
            println!("The HTTP graceful drain timed out.");
            ok = false;
        }
        Ok(Err(error)) => {
            println!("The HTTP drain controller failed: {error}");
            ok = false;
        }
        Err(_) => {
            println!("Timed out waiting for the HTTP drain controller.");
            ok = false;
        }
    }

    ok
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
