//! Unified tracker client binary.
use torrust_tracker_client::console::clients::unified::app;

#[tokio::main]
async fn main() {
    if let Err(error) = app::run().await {
        match error {
            app::Error::Check(err) => {
                let (json, exit_code) = err.to_stderr_json_and_exit_code();
                eprintln!("{json}");
                std::process::exit(exit_code);
            }
            app::Error::Other(err) => {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
    }
}
