#![allow(clippy::print_stdout)]

use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use torrust_tracker_core::databases::driver::Driver;

use super::types::{DbVersion, OpsCount};
use super::{operations, report, reporting};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Database driver benchmarked in this invocation.
    #[arg(long)]
    driver: Driver,

    /// Database image tag. Used only for `MySQL`.
    #[arg(long, default_value = "8.4")]
    db_version: DbVersion,

    /// Number of samples per operation.
    #[arg(long, default_value = "100")]
    ops: OpsCount,
}

/// Executes the persistence benchmark runner CLI.
///
/// # Errors
///
/// Returns an error if argument validation fails, the benchmark execution
/// fails, or report serialization fails.
pub async fn run() -> Result<()> {
    let Args { driver, db_version, ops } = Args::parse();

    let total_started_at = Instant::now();

    let benchmark_started_at = Instant::now();
    let operation_stats = operations::collect_operation_stats(&driver, &db_version, ops).await?;
    let benchmark_duration = benchmark_started_at.elapsed();

    let report_build_started_at = Instant::now();
    let mut benchmark_report = reporting::build_report(
        &driver,
        &db_version,
        ops.get(),
        report::ReportTimings {
            benchmark: 0,
            report_build: 0,
            total: 0,
        },
        operation_stats,
    );
    let report_build_duration = report_build_started_at.elapsed();

    let total_duration = total_started_at.elapsed();
    benchmark_report.meta.timings_ms = report::ReportTimings {
        benchmark: duration_to_millis_u64(benchmark_duration),
        report_build: duration_to_millis_u64(report_build_duration),
        total: duration_to_millis_u64(total_duration),
    };

    let json = report::to_json_pretty(&benchmark_report)?;

    println!("{json}");

    Ok(())
}

fn duration_to_millis_u64(duration: std::time::Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}
