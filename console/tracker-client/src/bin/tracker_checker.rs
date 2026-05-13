//! Program to check running trackers.
use torrust_tracker_client::console::clients::checker::app;

#[tokio::main]
async fn main() {
    eprintln!(
        "warning: `tracker_checker` is deprecated and will be removed in a future release. Use `tracker_client check ...` instead."
    );

    if let Err(e) = app::run().await {
        let (json, exit_code) = e.to_stderr_json_and_exit_code();
        eprintln!("{json}");
        std::process::exit(exit_code);
    }
}
