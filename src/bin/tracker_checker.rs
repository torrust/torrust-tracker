//! Program to run checks against running trackers.
//!
//! Run providing a config file path:
//!
//! ```text
//! cargo run --bin tracker_checker -- --config-path "./share/default/config/tracker_checker.json"
//! TORRUST_CHECKER_CONFIG_PATH="./share/default/config/tracker_checker.json" cargo run --bin tracker_checker
//! ```
//!
//! Run providing the configuration:
//!
//! ```text
//! TORRUST_CHECKER_CONFIG=$(cat "./share/default/config/tracker_checker.json") cargo run --bin tracker_checker
//! ```
use torrust_tracker::checker::app;

#[tokio::main]
async fn main() {
    app::run().await.expect("Some checks fail");
}
