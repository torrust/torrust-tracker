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
                let json = serde_json::json!({
                    "error": {
                        "kind": "runtime_failure",
                        "source": "runtime",
                        "message": err.to_string(),
                    }
                })
                .to_string();
                eprintln!("{json}");
                std::process::exit(1);
            }
        }
    }
}
