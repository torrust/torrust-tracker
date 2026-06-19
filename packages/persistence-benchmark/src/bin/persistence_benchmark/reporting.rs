use torrust_tracker_primitives::Driver;

use super::types::DbVersion;
use super::{metrics, report};

/// Builds the final JSON-serializable report from run context and metrics.
///
/// For `sqlite3` runs, `db_version` is normalized to `-` because there is no
/// image tag associated with the local file-backed database.
#[must_use]
pub fn build_report(
    driver: &Driver,
    db_version: &DbVersion,
    ops: usize,
    timings_ms: report::ReportTimings,
    operation_stats: Vec<metrics::OperationStats>,
) -> report::BenchReport {
    let normalized_db_version = match driver {
        Driver::Sqlite3 => "-".to_string(),
        Driver::MySQL | Driver::PostgreSQL => db_version.to_string(),
    };

    let meta = report::ReportMeta::from_run_context(driver.as_str(), &normalized_db_version, ops, timings_ms);

    report::BenchReport::new(meta, operation_stats)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::time::Duration;

    use torrust_tracker_primitives::Driver;

    use super::build_report;
    use crate::persistence_benchmark::metrics::OperationStats;
    use crate::persistence_benchmark::report::ReportTimings;
    use crate::persistence_benchmark::types::DbVersion;

    #[test]
    fn it_should_normalize_db_version_to_dash_for_sqlite_reports() {
        let db_version = DbVersion::from_str("8.4").expect("db version should parse");
        let timings_ms = ReportTimings {
            benchmark: 7,
            report_build: 1,
            total: 8,
        };
        let operation_stats = vec![OperationStats {
            name: "save_torrent_downloads".to_string(),
            count: 1,
            best: Duration::from_micros(1),
            median: Duration::from_micros(1),
            worst: Duration::from_micros(1),
        }];

        let report = build_report(&Driver::Sqlite3, &db_version, 1, timings_ms, operation_stats);

        assert_eq!(report.meta.driver, "sqlite3");
        assert_eq!(report.meta.db_version, "-");
    }

    #[test]
    fn it_should_keep_mysql_db_version_in_report_metadata() {
        let db_version = DbVersion::from_str("8.4").expect("db version should parse");
        let timings_ms = ReportTimings {
            benchmark: 9,
            report_build: 1,
            total: 10,
        };
        let operation_stats = vec![OperationStats {
            name: "load_keys".to_string(),
            count: 2,
            best: Duration::from_micros(2),
            median: Duration::from_micros(3),
            worst: Duration::from_micros(4),
        }];

        let report = build_report(&Driver::MySQL, &db_version, 2, timings_ms, operation_stats);

        assert_eq!(report.meta.driver, "mysql");
        assert_eq!(report.meta.db_version, "8.4");
        assert_eq!(report.meta.ops, 2);
    }

    #[test]
    fn it_should_keep_postgresql_db_version_in_report_metadata() {
        let db_version = DbVersion::from_str("17").expect("db version should parse");
        let timings_ms = ReportTimings {
            benchmark: 5,
            report_build: 1,
            total: 6,
        };
        let operation_stats = vec![OperationStats {
            name: "load_keys".to_string(),
            count: 1,
            best: Duration::from_micros(1),
            median: Duration::from_micros(2),
            worst: Duration::from_micros(3),
        }];

        let report = build_report(&Driver::PostgreSQL, &db_version, 1, timings_ms, operation_stats);

        assert_eq!(report.meta.driver, "postgresql");
        assert_eq!(report.meta.db_version, "17");
        assert_eq!(report.meta.ops, 1);
    }
}
