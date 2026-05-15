use anyhow::{Context, Result};
use chrono::Utc;
use serde::Serialize;

use super::helpers;
use super::metrics::OperationStats;

#[derive(Debug, Serialize)]
pub struct BenchReport {
    pub meta: ReportMeta,
    pub operations: Vec<OperationReport>,
}

#[derive(Debug, Serialize)]
pub struct ReportMeta {
    pub git_revision: String,
    pub driver: String,
    pub db_version: String,
    pub ops: usize,
    pub timestamp: String,
    pub timings_ms: ReportTimings,
}

#[derive(Debug, Serialize)]
pub struct ReportTimings {
    pub benchmark: u64,
    pub report_build: u64,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct OperationReport {
    pub name: String,
    pub count: usize,
    pub best_us: u64,
    pub median_us: u64,
    pub worst_us: u64,
}

impl BenchReport {
    /// Builds a serializable benchmark report from aggregated operation stats.
    ///
    /// Durations are converted to microseconds to keep report values compact,
    /// language-agnostic, and easy to compare across runs.
    #[must_use]
    pub fn new(meta: ReportMeta, operation_stats: Vec<OperationStats>) -> Self {
        let operations = operation_stats
            .into_iter()
            .map(|operation_stat| OperationReport {
                name: operation_stat.name.clone(),
                count: operation_stat.count,
                best_us: duration_to_micros(operation_stat.best),
                median_us: duration_to_micros(operation_stat.median),
                worst_us: duration_to_micros(operation_stat.worst),
            })
            .collect();

        Self { meta, operations }
    }
}

impl ReportMeta {
    /// Captures report metadata for one benchmark execution.
    ///
    /// The timestamp is recorded in RFC 3339 format and the git revision is
    /// resolved from the current repository state.
    #[must_use]
    pub fn from_run_context(driver: &str, db_version: &str, ops: usize, timings_ms: ReportTimings) -> Self {
        let git_revision = helpers::git_revision();

        Self {
            git_revision,
            driver: driver.to_string(),
            db_version: db_version.to_string(),
            ops,
            timestamp: Utc::now().to_rfc3339(),
            timings_ms,
        }
    }
}

/// Serializes the benchmark report as pretty-printed JSON.
///
/// # Errors
///
/// Returns an error if serialization fails.
pub fn to_json_pretty(report: &BenchReport) -> Result<String> {
    serde_json::to_string_pretty(report).context("failed to serialize benchmark report")
}

/// Converts a duration into microseconds for JSON serialization.
///
/// Saturates to `u64::MAX` if conversion overflows.
fn duration_to_micros(duration: std::time::Duration) -> u64 {
    u64::try_from(duration.as_micros()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{BenchReport, ReportMeta, ReportTimings, to_json_pretty};
    use crate::persistence_benchmark::metrics::OperationStats;

    #[test]
    fn it_should_convert_operation_durations_to_microseconds_in_report() {
        let meta = ReportMeta {
            git_revision: "test-revision".to_string(),
            driver: "sqlite3".to_string(),
            db_version: "-".to_string(),
            ops: 2,
            timestamp: "2026-01-01T00:00:00+00:00".to_string(),
            timings_ms: ReportTimings {
                benchmark: 10,
                report_build: 1,
                total: 11,
            },
        };
        let operation_stats = vec![OperationStats {
            name: "save_global_downloads".to_string(),
            count: 2,
            best: Duration::from_micros(7),
            median: Duration::from_micros(11),
            worst: Duration::from_micros(19),
        }];

        let report = BenchReport::new(meta, operation_stats);

        assert_eq!(report.operations.len(), 1);
        assert_eq!(report.operations[0].name, "save_global_downloads");
        assert_eq!(report.operations[0].best_us, 7);
        assert_eq!(report.operations[0].median_us, 11);
        assert_eq!(report.operations[0].worst_us, 19);
    }

    #[test]
    fn it_should_serialize_report_as_valid_pretty_json() {
        let meta = ReportMeta {
            git_revision: "test-revision".to_string(),
            driver: "sqlite3".to_string(),
            db_version: "-".to_string(),
            ops: 1,
            timestamp: "2026-01-01T00:00:00+00:00".to_string(),
            timings_ms: ReportTimings {
                benchmark: 5,
                report_build: 1,
                total: 6,
            },
        };
        let operation_stats = vec![OperationStats {
            name: "load_whitelist".to_string(),
            count: 1,
            best: Duration::from_micros(3),
            median: Duration::from_micros(3),
            worst: Duration::from_micros(3),
        }];
        let report = BenchReport::new(meta, operation_stats);

        let json = to_json_pretty(&report).expect("report should serialize");
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("json should parse");

        assert_eq!(parsed["meta"]["driver"], "sqlite3");
        assert_eq!(parsed["meta"]["timings_ms"]["total"], 6);
        assert_eq!(parsed["operations"][0]["name"], "load_whitelist");
    }
}
