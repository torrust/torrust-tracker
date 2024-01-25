//! Program to run E2E tests.
//!
//! ```text
//! cargo run --bin e2e_tests_runner share/default/config/tracker.e2e.container.sqlite3.toml
//! ```
use torrust_tracker::e2e;

fn main() {
    e2e::runner::run();
}
