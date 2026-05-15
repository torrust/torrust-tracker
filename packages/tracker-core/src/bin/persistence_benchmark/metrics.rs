use std::time::Duration;

use anyhow::{Result, anyhow};

use super::driver_bench::RawOperationSamples;

#[derive(Debug, Clone)]
pub struct OperationStats {
    pub name: String,
    pub count: usize,
    pub best: Duration,
    pub median: Duration,
    pub worst: Duration,
}

/// Computes benchmark statistics for each operation.
///
/// # Errors
///
/// Returns an error if an operation has no samples.
pub fn compute(raw_operations: Vec<RawOperationSamples>) -> Result<Vec<OperationStats>> {
    let mut operation_stats = Vec::with_capacity(raw_operations.len());

    for raw_operation in raw_operations {
        operation_stats.push(compute_operation(raw_operation)?);
    }

    Ok(operation_stats)
}

/// Computes summary statistics for one benchmark operation.
///
/// Samples are sorted so `best`/`median`/`worst` are deterministic and
/// independent from insertion order.
///
/// # Errors
///
/// Returns an error when no samples were collected for the operation.
fn compute_operation(raw_operation: RawOperationSamples) -> Result<OperationStats> {
    if raw_operation.samples.is_empty() {
        return Err(anyhow!("operation '{}' has no samples", raw_operation.name));
    }

    let mut sorted_samples = raw_operation.samples;
    sorted_samples.sort_unstable();

    let count = sorted_samples.len();
    let best = sorted_samples[0];
    let median = sorted_samples[count / 2];
    let worst = sorted_samples[count - 1];

    Ok(OperationStats {
        name: raw_operation.name,
        count,
        best,
        median,
        worst,
    })
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::compute;
    use crate::persistence_benchmark::driver_bench::RawOperationSamples;

    #[test]
    fn it_should_compute_sorted_best_median_and_worst_for_each_operation() {
        let raw_operations = vec![RawOperationSamples {
            name: "save_torrent_downloads".to_string(),
            samples: vec![
                Duration::from_micros(50),
                Duration::from_micros(20),
                Duration::from_micros(30),
                Duration::from_micros(10),
            ],
        }];

        let stats = compute(raw_operations).expect("metrics should compute");

        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].name, "save_torrent_downloads");
        assert_eq!(stats[0].count, 4);
        assert_eq!(stats[0].best, Duration::from_micros(10));
        assert_eq!(stats[0].median, Duration::from_micros(30));
        assert_eq!(stats[0].worst, Duration::from_micros(50));
    }

    #[test]
    fn it_should_fail_when_operation_has_no_samples() {
        let raw_operations = vec![RawOperationSamples {
            name: "load_keys".to_string(),
            samples: Vec::new(),
        }];

        let error = compute(raw_operations).expect_err("empty samples should fail");

        assert_eq!(error.to_string(), "operation 'load_keys' has no samples");
    }
}
