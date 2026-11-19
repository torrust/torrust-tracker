#![allow(clippy::exit, reason = "unified tracker client returns explicit process status")]

//! Unified tracker client binary.
//!
//! A `stdout-result-data` command of the global CLI output contract: it refuses to run when
//! stdout is a terminal, writes one JSON result on stdout, and writes one JSON object per line on
//! stderr.
use torrust_tracker_console_client::console::clients::unified::app;
use torrust_tracker_console_client::console::clients::unified::output::{
    install_panic_hook, tty_refusal_record, write_diagnostic,
};

#[tokio::main]
async fn main() {
    install_panic_hook();
    if let Err(error) = app::run().await {
        match error {
            app::Error::TtyRefusal => {
                write_diagnostic(&tty_refusal_record());
                std::process::exit(2);
            }
            app::Error::Check(err) => {
                let (json, exit_code) = err.to_stderr_json_and_exit_code();
                write_diagnostic(&json);
                std::process::exit(exit_code);
            }
            app::Error::Other(err) => report_runtime_failure(&err.to_string()),
            unknown => report_runtime_failure(&format!("{unknown:?}")),
        }
    }
}

fn report_runtime_failure(message: &str) -> ! {
    let json = serde_json::json!({
        "error": {
            "kind": "runtime_failure",
            "source": "runtime",
            "message": message,
        }
    })
    .to_string();
    write_diagnostic(&json);
    std::process::exit(1);
}
