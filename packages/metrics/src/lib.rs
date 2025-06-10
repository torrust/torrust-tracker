pub mod counter;
pub mod gauge;
pub mod label;
pub mod metric;
pub mod metric_collection;
pub mod prometheus;
pub mod sample;
pub mod sample_collection;
pub mod unit;

pub const METRICS_TARGET: &str = "METRICS";

#[cfg(test)]
mod tests {
    /// It removes leading and trailing whitespace from each line.
    pub fn format_prometheus_output(output: &str) -> String {
        output
            .lines()
            .map(str::trim_start)
            .map(str::trim_end)
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn sort_lines(s: &str) -> String {
        let mut lines: Vec<&str> = s.split('\n').collect();
        lines.sort_unstable();
        lines.join("\n")
    }
}
