//! This binary is used for profiling with [valgrind](https://valgrind.org/)
//! and [kcachegrind](https://kcachegrind.github.io/).
use std::process::ExitCode;

use torrust_tracker_lib::console::profiling::run;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
