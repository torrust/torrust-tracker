pub mod counter;
pub mod gauge;
pub mod label;
pub mod metric;
pub mod metric_collection;
pub mod prometheus;
pub mod sample;
pub mod sample_collection;
pub mod thread_safe_metric_collection;
pub mod unit;

#[cfg(test)]
mod tests {
    /// It removes leading and trailing whitespace from each line, and empty lines.
    pub fn format_prometheus_output(output: &str) -> String {
        output
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn sort_lines(s: &str) -> String {
        let mut lines: Vec<&str> = s.split('\n').collect();
        lines.sort_unstable();
        lines.join("\n")
    }
}
