//! Minimal HTTP-only public tracker — configuration coupling demonstration.
//!
//! **Purpose** (issue #1856, step 3): demonstrates how many configuration types an
//! HTTP-only binary must compile today, including types that belong to services that
//! are explicitly disabled at runtime.  This example starts a real HTTP tracker and
//! runs until Ctrl-C is pressed.
//!
//! ## Why these two examples exist (and why they live here)
//!
//! Issue #1856 analyses whether the `torrust-tracker-configuration` package should
//! be split by service type.  To make that coupling **visible and verifiable**, we
//! need one operative example per protocol:
//!
//! - `udp_only_public_tracker` (in `torrust-tracker-udp-server`) — UDP path
//! - `http_only_public_tracker` (this file, in `torrust-tracker-axum-http-server`) — HTTP path
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
//! A realistic HTTP-only public tracker needs these config types at runtime:
//!
//! - `Core` — shared tracker settings (mode, announce policy, database, …)
//! - `HttpTracker` — bind address and optional TLS config for the HTTP server
//!
//! However, the initialization entry point accepts `&Configuration` — the **full
//! aggregate** struct — so the compiler must include all fields declared in
//! `Configuration`:
//!
//! | Config type       | Used by HTTP-only binary | Why compiled in               |
//! |-------------------|--------------------------|-------------------------------|
//! | `Core`            | Yes                      | Tracker domain settings       |
//! | `HttpTracker`     | Yes                      | Bind address, TLS config      |
//! | `Logging`         | Yes (log setup only)     | Global log initialization     |
//! | `UdpTracker`      | **No**                   | Field of `Configuration`      |
//! | `HttpApi`         | **No**                   | Field of `Configuration`      |
//! | `AccessTokens`    | **No**                   | Field of `Configuration`      |
//! | `HealthCheckApi`  | **No**                   | Field of `Configuration`      |
//! | `TslConfig`       | Optional (TLS path)      | Field of `HttpTracker`        |
//!
//! ## Cross-layer coupling note
//!
//! `rest-api-core` imports **both** `HttpTracker` and `UdpTracker` from the
//! configuration package so it can expose tracker status via the REST API endpoints.
//! This means that any binary including the REST API compiles UDP config types
//! regardless of whether a UDP tracker is actually running.  Splitting the config
//! package by service type would not eliminate this cross-layer coupling; the REST
//! API would still depend on all service config types to describe tracker status.
//!
//! ## How to run
//!
//! ```bash
//! cargo run -p torrust-tracker-axum-http-server --example http_only_public_tracker
//! ```
//!
//! ## How to inspect the full dependency chain
//!
//! ```bash
//! cargo tree -p torrust-tracker-axum-http-server --example http_only_public_tracker
//! ```

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use torrust_tracker_axum_http_server::environment::Started;
use torrust_tracker_configuration::{Configuration, HttpTracker, UdpTracker};

#[tokio::main]
async fn main() {
    // Temporary database file — cleaned up on exit.
    let db_path = std::env::temp_dir().join("torrust-http-example.db");

    // Build the minimal configuration for an HTTP-only public tracker.
    let mut config = Configuration::default();

    // Public tracker: peers do not need an authentication key.
    config.core.private = false;

    // Point the database at the temporary file.
    config.core.database.path = db_path.to_string_lossy().into_owned();

    // Single HTTP tracker instance; port 0 lets the OS assign a free port.
    // TLS is disabled for simplicity; a production deployment would set tsl_config.
    config.http_trackers = Some(vec![HttpTracker {
        bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
        tsl_config: None,
        tracker_usage_statistics: false,
    }]);

    // Disable all services that this HTTP-only binary does not need.
    config.udp_trackers = None;
    config.http_api = None;

    // --- Demonstrate the coupling problem ----------------------------------------

    // `UdpTracker` is compiled into this binary even though it is never used at
    // runtime, because it is a field of the `Configuration` aggregate.
    #[allow(clippy::no_effect_underscore_binding)]
    let _unused_udp_type: Option<Vec<UdpTracker>> = config.udp_trackers.clone();

    println!("Types from torrust-tracker-configuration compiled into this binary:");
    println!("  Used at runtime    : Core, HttpTracker, Logging");
    println!("  Full aggregate     : Configuration (required by the initialization entry point)");
    println!("  Compiled but idle  : UdpTracker, HttpApi, AccessTokens, HealthCheckApi");
    println!();
    println!("Cross-layer coupling: rest-api-core imports both HttpTracker and UdpTracker");
    println!("  to expose tracker status via the REST API.  A package split would not");
    println!("  eliminate this dependency — the REST API needs all service config types.");
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
