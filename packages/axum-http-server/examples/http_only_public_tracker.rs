//! Minimal HTTP-only public tracker — narrowed configuration at the initialization boundary.
//!
//! **Status** (issue #1861, implementing decision DEC-09 from EPIC #1669): the initialization
//! entry point now accepts `&Arc<Core>` and `&Arc<HttpTracker>` directly, so an HTTP-only
//! binary no longer needs to compile the full `Configuration` aggregate.
//!
//! ## What this example shows
//!
//! An HTTP-only public tracker can now be started with exactly the two config types it
//! actually uses at runtime:
//!
//! - `Core` — shared tracker settings (mode, announce policy, database, …)
//! - `HttpTracker` — bind address and optional TLS config for the HTTP server
//!
//! | Config type       | Needed? | Notes                                       |
//! |-------------------|---------|---------------------------------------------|
//! | `Core`            | Yes     | Tracker domain settings                     |
//! | `HttpTracker`     | Yes     | Bind address, TLS config                    |
//! | `Configuration`   | No      | Full aggregate — no longer required here    |
//! | `UdpTracker`      | No      | Not compiled unless explicitly imported     |
//! | `HttpApi`         | No      | Not compiled unless explicitly imported     |
//! | `HealthCheckApi`  | No      | Not compiled unless explicitly imported     |
//!
//! ## Cross-layer coupling note
//!
//! `rest-api-core` imports **both** `HttpTracker` and `UdpTracker` from the
//! configuration package so it can expose tracker status via the REST API endpoints.
//! This means that any binary including the REST API compiles UDP config types
//! regardless of whether a UDP tracker is actually running.  This is a separate
//! concern and is not addressed by this narrowing (see EPIC #1669 for context).
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

use torrust_tracker_axum_http_server::testing::environment::Started;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::database::Database;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_configuration::v3_0_0::network::Network;

#[tokio::main]
async fn main() {
    // Temporary database file — cleaned up on exit.
    let db_path = std::env::temp_dir().join("torrust-http-example.db");

    // Build Core and HttpTracker directly — no full Configuration aggregate needed.
    // Public tracker: peers do not need an authentication key.
    let core = Core {
        private: false,
        database: Some(Database::Sqlite3 {
            path: db_path.to_string_lossy().into_owned(),
        }),
        ..Core::default()
    };

    // Single HTTP tracker instance; port 0 lets the OS assign a free port.
    // TLS is disabled for simplicity; a production deployment would set tsl_config.
    let http_tracker = HttpTracker {
        bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
        tls_config: None,
        tracker_usage_statistics: false,
        use_ip_from_query_string: false,
        public_url: None,
        network: Network::default(),
    };

    println!("Types from torrust-tracker-configuration used by this binary:");
    println!("  Core        — tracker domain settings");
    println!("  HttpTracker — bind address, TLS config");
    println!("  (Configuration aggregate and idle types are NOT compiled in)");
    println!();

    // Start the tracker using the narrowed API; `Started` is a type alias for `Environment<Running>`.
    let core_config = Arc::new(core);
    let http_tracker_config = Arc::new(http_tracker);
    let env = Started::new(&core_config, &http_tracker_config).await;

    println!("Listening on {}", env.bind_address());
    println!("Press Ctrl-C to stop.");

    tokio::signal::ctrl_c().await.expect("failed to install Ctrl-C handler");
    println!("\nShutting down...");

    env.stop().await;

    // Best-effort cleanup of the temporary database file.
    std::fs::remove_file(&db_path).ok();

    println!("Stopped.");
}
