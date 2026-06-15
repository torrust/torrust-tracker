//! Program to run persistence benchmarks directly against database drivers.
//!
//! This binary is a developer tool for measuring the persistence-layer methods
//! implemented by the [`Database`](torrust_tracker_core::databases::traits::database::Database)
//! trait. It benchmarks one driver per invocation and prints a JSON report to
//! standard output with per-operation timing statistics.
//!
//! How it works:
//!
//! - Parses CLI arguments for the target driver, database version, and sample
//!   count (`--ops`, default: `100`).
//! - Instantiates a real persistence backend:
//!   - `sqlite3` uses a temporary `SQLite` database file.
//!   - `mysql` starts a testcontainers `mysql` container with the requested
//!     image tag.
//! - Creates a clean schema and seeds the minimum data needed for each measured
//!   operation.
//! - Repeats every persistence operation `--ops` times, measuring each call
//!   with `std::time::Instant`.
//! - Sorts the collected durations and prints `count`, `best`, `median`, and
//!   `worst` values as JSON.
//! - Emits only JSON on standard output (no status line and no file output
//!   argument).
//!
//! Typical usage:
//!
//! ```text
//! cargo run -p torrust-tracker-core --bin persistence_benchmark_runner -- \
//!   --driver sqlite3
//!
//! cargo run -p torrust-tracker-core --bin persistence_benchmark_runner -- \
//!   --driver mysql \
//!   --db-version 8.4
//! ```
//!
//! Store output in a file with shell redirection:
//!
//! ```text
//! cargo run -p torrust-tracker-core --bin persistence_benchmark_runner -- \
//!   --driver sqlite3 \
//!   > .benchmarks/bench-results-sqlite3.json
//! ```
//!
//! Sample report:
//!
//! ```json
//! {
//!   "meta": {
//!     "git_revision": "16c9c8a4695d336a4531204913390a47b20d9468",
//!     "driver": "sqlite3",
//!     "db_version": "-",
//!     "ops": 100,
//!     "timestamp": "2026-04-28T16:23:24.084307218+00:00",
//!     "timings_ms": {
//!       "benchmark": 18,
//!       "report_build": 0,
//!       "total": 19
//!     }
//!   },
//!   "operations": [
//!     {
//!       "name": "save_torrent_downloads",
//!       "count": 100,
//!       "best_us": 66,
//!       "median_us": 70,
//!       "worst_us": 79
//!     }
//!   ]
//! }
//! ```
mod persistence_benchmark;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    persistence_benchmark::runner::run().await
}
