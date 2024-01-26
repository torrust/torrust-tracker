//! Program to run checks against running trackers.
//!
//! ```text
//! cargo run --bin tracker_checker "./share/default/config/tracker_checker.json"
//! ```
use torrust_tracker::checker::app;

#[tokio::main]
async fn main() {
    app::run().await.expect("Some checks fail");
}
