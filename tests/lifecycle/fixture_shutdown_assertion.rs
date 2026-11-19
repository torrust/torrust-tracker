//! Fixture shutdown assertion integration test (#1419).
//!
//! [`TrackerApplicationFixture::shutdown`](common::TrackerApplicationFixture::shutdown)
//! panics when a component of the tracker did not stop gracefully. In every
//! other suite that assertion passes, so this binary is the one that shows it
//! can fail: it adds a component that never stops and requires the panic.
//!
//! It also shows that the fixture waits with the configured
//! `[application.shutdown]` deadline, because the panic must arrive within a
//! bound that only the configured deadline can meet.
//!
//! ```text
//! cargo test --test lifecycle-fixture-shutdown-assertion
//! ```
#[path = "../common/mod.rs"]
mod common;
// issue: #1419

use std::future::pending;
use std::time::Duration;

use tokio::time::timeout;

/// A tracker whose whole shutdown may take 3 seconds.
///
/// The three drain budgets are the minimum, so the configuration keeps the
/// rule `max(http_drain, udp_request_drain) + event_listener_drain < deadline`.
const THREE_SECOND_SHUTDOWN_DEADLINE_CONFIG: &str = r#"
    [metadata]
    app = "torrust-tracker"
    purpose = "configuration"
    schema_version = "3.0.0"

    [logging]
    trace_filter = "off"

    [application.shutdown]
    deadline_in_secs = 3
    http_drain_in_secs = 1
    udp_request_drain_in_secs = 1
    event_listener_drain_in_secs = 1

    [core]
    listed = false
    private = false

    [core.database]
    driver = "sqlite3"
    path = "{STORAGE_PATH}/sqlite3.db"

    [health_check_api]
    bind_address = "127.0.0.1:0"
"#;

/// Twice the configured deadline, and below the 10 seconds the fixture waited
/// before it used the configured deadline (the default deadline is 25 seconds).
const SHUTDOWN_TIME_BOUND: Duration = Duration::from_secs(6);

#[tokio::test]
#[should_panic(expected = "hung_component")]
async fn it_should_panic_naming_a_component_that_does_not_stop_before_the_configured_shutdown_deadline() {
    let mut fixture = common::TrackerApplicationFixture::start(THREE_SECOND_SHUTDOWN_DEADLINE_CONFIG).await;
    fixture.spawn_component("hung_component", pending());

    timeout(SHUTDOWN_TIME_BOUND, fixture.shutdown())
        .await
        .expect("the shutdown should end at the configured deadline, not later");
}
