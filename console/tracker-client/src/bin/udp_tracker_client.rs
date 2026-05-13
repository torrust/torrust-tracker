//! Program to make request to UDP trackers.
use torrust_tracker_client::console::clients::udp::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!(
        "warning: `udp_tracker_client` is deprecated and will be removed in a future release. Use `tracker_client udp ...` instead."
    );

    app::run().await
}
