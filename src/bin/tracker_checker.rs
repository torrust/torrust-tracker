//! Program to check running trackers.
use torrust_tracker::console::clients::checker::app;

#[tokio::main]
async fn main() {
    app::run().await.expect("Some checks fail");
}
