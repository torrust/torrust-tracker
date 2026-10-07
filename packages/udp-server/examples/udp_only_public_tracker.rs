//! Minimal UDP-only public tracker — narrowed configuration at the initialization boundary.
//!
//! **Status** (issue #1861, implementing decision DEC-09 from EPIC #1669): the initialization
//! entry point now accepts `&Arc<Core>` and `&Arc<UdpTracker>` directly, so a UDP-only
//! binary no longer needs to compile the full `Configuration` aggregate.
//!
//! ## What this example shows
//!
//! A UDP-only public tracker can now be started with exactly the two config types it
//! actually uses at runtime:
//!
//! - `Core` — shared tracker settings (mode, announce policy, database, …)
//! - `UdpTracker` — bind address and cookie lifetime for the UDP server
//!
//! | Config type       | Needed? | Notes                                       |
//! |-------------------|---------|---------------------------------------------|
//! | `Core`            | Yes     | Tracker domain settings                     |
//! | `UdpTracker`      | Yes     | Bind address, cookie lifetime               |
//! | `Configuration`   | No      | Full aggregate — no longer required here    |
//! | `HttpTracker`     | No      | Not compiled unless explicitly imported     |
//! | `HttpApi`         | No      | Not compiled unless explicitly imported     |
//! | `HealthCheckApi`  | No      | Not compiled unless explicitly imported     |
//!
//! ## How to run
//!
//! ```bash
//! cargo run -p torrust-tracker-udp-server --example udp_only_public_tracker
//! ```
//!
//! Stop it with Ctrl-C (SIGINT) or, on Unix, SIGTERM (`kill <pid>`). The example's `main` is
//! the only place that subscribes to OS signals; it then calls `Environment::stop()`, which
//! cancels and joins the tracker's tasks. The token-aware path it uses never listens for signals.
//! When sending signals by PID, run the built binary
//! (`target/debug/examples/udp_only_public_tracker`) so the signal reaches this process
//! rather than `cargo`.
//!
//! ## How to inspect the full dependency chain
//!
//! ```bash
//! cargo tree -p torrust-tracker-udp-server --example udp_only_public_tracker
//! ```

#![allow(clippy::print_stdout, reason = "runnable public-tracker example prints service information")]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

#[cfg(unix)]
use tokio::signal::unix::{SignalKind, signal};
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::database::Database;
use torrust_tracker_configuration::v3_0_0::network::Network;
use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;
use torrust_tracker_udp_server::testing::environment::Started;

#[tokio::main]
async fn main() {
    // Temporary database file — cleaned up on exit.
    let db_path = std::env::temp_dir().join("torrust-udp-example.db");

    // Build Core and UdpTracker directly — no full Configuration aggregate needed.
    // Public tracker: peers do not need an authentication key.
    let core = Core {
        private: false,
        database: Some(Database::Sqlite3 {
            path: db_path.to_string_lossy().into_owned(),
        }),
        ..Core::default()
    };

    let udp_tracker = UdpTracker {
        bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
        cookie_lifetime: Duration::from_secs(120),
        tracker_usage_statistics: false,
        public_url: None,
        network: Network::default(),
    };

    println!("Types from torrust-tracker-configuration used by this binary:");
    println!("  Core       — tracker domain settings");
    println!("  UdpTracker — bind address, cookie lifetime");
    println!("  (Configuration aggregate and idle types are NOT compiled in)");
    println!();

    // Start the tracker using the narrowed API; `Started` is a type alias for `Environment<Running>`.
    let core_config = Arc::new(core);
    let udp_tracker_config = Arc::new(udp_tracker);
    let env = Started::new(&core_config, &udp_tracker_config).await;

    // On Unix the handlers are installed here, before readiness is printed, so a later signal is never lost.
    let shutdown_signal = install_shutdown_signal_handlers();

    println!("Listening on {}", env.bind_address());
    println!("Press Ctrl-C (or send SIGTERM on Unix) to stop.");

    let signal = shutdown_signal.await;
    println!("\nReceived {signal}. Shutting down...");

    env.stop().await;

    // Best-effort cleanup of the temporary database file.
    std::fs::remove_file(&db_path).ok();

    println!("Stopped.");
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
