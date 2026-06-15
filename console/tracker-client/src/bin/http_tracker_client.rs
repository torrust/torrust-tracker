#![allow(clippy::print_stderr)]

//! Program to make request to HTTP trackers.
use torrust_tracker_console_client::console::clients::http::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!(
        "warning: `http_tracker_client` is deprecated and will be removed in a future release. Use `tracker_client http ...` instead."
    );

    app::run().await
}
