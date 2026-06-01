//! Minimal UDP-only public tracker — configuration coupling demonstration.
//!
//! **Purpose** (issue #1856, step 3): demonstrates how many configuration types a
//! UDP-only binary must compile today, even though most of those types are not
//! exercised at runtime.  This example starts a real UDP tracker and runs until
//! Ctrl-C is pressed.
//!
//! ## Why these two examples exist (and why they live here)
//!
//! Issue #1856 analyses whether the `torrust-tracker-configuration` package should
//! be split by service type.  To make that coupling **visible and verifiable**, we
//! need one operative example per protocol:
//!
//! - `udp_only_public_tracker` (this file, in `torrust-tracker-udp-server`) — UDP path
//! - `http_only_public_tracker` (in `torrust-tracker-axum-http-server`) — HTTP path
//!
//! Both examples are intentionally **public** (no authentication key required).
//! Private mode was considered but rejected: it would require a running REST API to
//! issue authentication keys, which would pull `torrust-tracker-axum-rest-api-server`
//! into the dependency graph and obscure the coupling signal we are trying to
//! measure.  Keeping both examples public and self-contained makes the coupling
//! table below directly comparable between the two protocols.
//!
//! The examples live inside their respective server packages (not in a shared
//! `examples/` workspace package and not in the root crate) because each example
//! deliberately uses only the server package it belongs to as its entry point.
//! That constraint is itself part of what we are measuring: how many config types
//! does a single-protocol server package drag in?
//!
//! ## What this example shows
//!
//! A realistic UDP-only public tracker needs exactly two config types at runtime:
//!
//! - `Core` — shared tracker settings (mode, announce policy, database, …)
//! - `UdpTracker` — bind address and cookie lifetime for the UDP server
//!
//! However, the initialization entry point accepts `&Configuration` — the **full
//! aggregate** struct — so the compiler must include all fields declared in
//! `Configuration`:
//!
//! | Config type       | Used by UDP-only binary | Why compiled in               |
//! |-------------------|-------------------------|-------------------------------|
//! | `Core`            | Yes                     | Tracker domain settings       |
//! | `UdpTracker`      | Yes                     | Bind address, cookie lifetime |
//! | `Logging`         | Yes (log setup only)    | Global log initialization     |
//! | `HttpTracker`     | **No**                  | Field of `Configuration`      |
//! | `HttpApi`         | **No**                  | Field of `Configuration`      |
//! | `HealthCheckApi`  | **No**                  | Field of `Configuration`      |
//! | `TslConfig`       | **No**                  | Field of `HttpTracker`        |
//! | `AccessTokens`    | **No**                  | Used by REST API only         |
//!
//! ## How to run
//!
//! ```bash
//! cargo run -p torrust-tracker-udp-server --example udp_only_public_tracker
//! ```
//!
//! ## How to inspect the full dependency chain
//!
//! ```bash
//! cargo tree -p torrust-tracker-udp-server --example udp_only_public_tracker
//! ```

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use torrust_tracker_configuration::{Configuration, HttpTracker, UdpTracker};
use torrust_tracker_udp_server::environment::Started;

#[tokio::main]
async fn main() {
    // Temporary database file — cleaned up on exit.
    let db_path = std::env::temp_dir().join("torrust-udp-example.db");

    // Build the minimal configuration for a UDP-only public tracker.
    let mut config = Configuration::default();

    // Public tracker: peers do not need an authentication key.
    config.core.private = false;

    // Point the database at the temporary file.
    config.core.database.path = db_path.to_string_lossy().into_owned();

    // Single UDP tracker instance; port 0 lets the OS assign a free port.
    config.udp_trackers = Some(vec![UdpTracker {
        bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
        cookie_lifetime: Duration::from_secs(120),
        tracker_usage_statistics: false,
    }]);

    // Disable all services that a UDP-only binary does not need.
    config.http_trackers = None;
    config.http_api = None;

    // --- Demonstrate the coupling problem ----------------------------------------

    // `HttpTracker` is compiled into this binary even though it is never used at
    // runtime, because it is a field of the `Configuration` aggregate.
    #[allow(clippy::no_effect_underscore_binding)]
    let _unused_http_type: Option<Vec<HttpTracker>> = config.http_trackers.clone();

    println!("Types from torrust-tracker-configuration compiled into this binary:");
    println!("  Used at runtime    : Core, UdpTracker, Logging");
    println!("  Full aggregate     : Configuration (required by the initialization entry point)");
    println!("  Compiled but idle  : HttpTracker, HttpApi, HealthCheckApi, TslConfig, AccessTokens");
    println!();

    // Start the tracker; `Started` is a type alias for `Environment<Running>`.
    let config = Arc::new(config);
    let env = Started::new(&config).await;

    println!("Listening on {}", env.bind_address());
    println!("Press Ctrl-C to stop.");

    tokio::signal::ctrl_c().await.expect("failed to install Ctrl-C handler");
    println!("\nShutting down...");

    env.stop().await;

    // Best-effort cleanup of the temporary database file.
    std::fs::remove_file(&db_path).ok();

    println!("Stopped.");
}
