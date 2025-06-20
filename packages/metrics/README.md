# Torrust Tracker Metrics

A comprehensive metrics library providing type-safe metric collection, aggregation, and Prometheus export functionality for the [Torrust Tracker](https://github.com/torrust/torrust-tracker) ecosystem.

## Overview

This library offers a robust metrics system designed specifically for tracking and monitoring BitTorrent tracker performance. It provides type-safe metric collection with support for labels, time-series data, and multiple export formats including Prometheus.

## Key Features

- **Type-Safe Metrics**: Strongly typed `Counter` and `Gauge` metrics with compile-time guarantees
- **Label Support**: Rich labeling system for multi-dimensional metrics
- **Time-Series Data**: Built-in support for timestamped samples
- **Prometheus Export**: Native Prometheus format serialization
- **Aggregation Functions**: Sum operations with mathematically appropriate return types
- **JSON Serialization**: Full serde support for all metric types
- **Memory Efficient**: Optimized data structures for high-performance scenarios

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
torrust-tracker-metrics = "3.0.0"
```

### Basic Usage

```rust
use torrust_tracker_metrics::{
    metric_collection::MetricCollection,
    label::{LabelSet, LabelValue},
    metric_name, label_name,
};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

// Create a metric collection
let mut metrics = MetricCollection::default();

// Define labels
let labels: LabelSet = [
    (label_name!("server"), LabelValue::new("tracker-01")),
    (label_name!("protocol"), LabelValue::new("http")),
].into();

// Record metrics
let time = DurationSinceUnixEpoch::from_secs(1234567890);
metrics.increment_counter(
    &metric_name!("requests_total"),
    &labels,
    time,
)?;

metrics.set_gauge(
    &metric_name!("active_connections"),
    &labels,
    42.0,
    time,
)?;

// Export to Prometheus format
let prometheus_output = metrics.to_prometheus();
println!("{}", prometheus_output);
```

### Metric Aggregation

```rust
use torrust_tracker_metrics::metric_collection::aggregate::{Sum, Avg};

// Sum all counter values matching specific labels
let total_requests = metrics.sum(
    &metric_name!("requests_total"),
    &[("server", "tracker-01")].into(),
);

println!("Total requests: {:?}", total_requests);

// Calculate average of gauge values matching specific labels
let avg_response_time = metrics.avg(
    &metric_name!("response_time_seconds"),
    &[("endpoint", "/announce")].into(),
);

println!("Average response time: {:?}", avg_response_time);
```

## Architecture

### Core Components

- **`Counter`**: Monotonically increasing integer values (u64)
- **`Gauge`**: Arbitrary floating-point values that can increase or decrease (f64)
- **`Metric<T>`**: Generic metric container with metadata (name, description, unit)
- **`MetricCollection`**: Type-safe collection managing both counters and gauges
- **`LabelSet`**: Key-value pairs for metric dimensionality
- **`Sample`**: Timestamped metric values with associated labels

### Type System

The library uses Rust's type system to ensure metric safety:

```rust
// Counter operations return u64
let counter_sum: Option<u64> = counter_collection.sum(&name, &labels);

// Gauge operations return f64  
let gauge_sum: Option<f64> = gauge_collection.sum(&name, &labels);

// Mixed collections convert to f64 for compatibility
let mixed_sum: Option<f64> = metric_collection.sum(&name, &labels);
```

### Module Structure

```output
src/
├── counter.rs             # Counter metric type
├── gauge.rs               # Gauge metric type  
├── metric/                # Generic metric container
│   ├── mod.rs
│   ├── name.rs            # Metric naming
│   ├── description.rs     # Metric descriptions
│   └── aggregate/         # Metric-level aggregations
├── metric_collection/     # Collection management
│   ├── mod.rs
│   └── aggregate/         # Collection-level aggregations
├── label/                 # Label system
│   ├── name.rs            # Label names
│   ├── value.rs           # Label values
│   └── set.rs             # Label collections
├── sample.rs              # Timestamped values
├── sample_collection.rs   # Sample management
├── prometheus.rs          # Prometheus export
└── unit.rs                # Measurement units
```

## Documentation

- [Crate documentation](https://docs.rs/torrust-tracker-metrics)
- [API Reference](https://docs.rs/torrust-tracker-metrics/latest/torrust_tracker_metrics/)

## Development

### Code Coverage

Run basic coverage report:

```console
cargo llvm-cov --package torrust-tracker-metrics 
```

Generate LCOV report (for IDE integration):

```console
mkdir -p ./.coverage
cargo llvm-cov --package torrust-tracker-metrics --lcov --output-path=./.coverage/lcov.info
```

Generate detailed HTML coverage report:

Generate detailed HTML coverage report:

```console
mkdir -p ./.coverage
cargo llvm-cov --package torrust-tracker-metrics --html --output-dir ./.coverage
```

Open the coverage report in your browser:

```console
open ./.coverage/index.html  # macOS
xdg-open ./.coverage/index.html  # Linux
```

## Performance Considerations

- **Memory Usage**: Metrics are stored in-memory with efficient HashMap-based collections
- **Label Cardinality**: Be mindful of label combinations as they create separate time series
- **Aggregation**: Sum operations are optimized for both single-type and mixed collections

## Compatibility

This library is designed to be compatible with the standard Rust [metrics](https://crates.io/crates/metrics) crate ecosystem where possible.

## Contributing

We welcome contributions! Please see the main [Torrust Tracker repository](https://github.com/torrust/torrust-tracker) for contribution guidelines.

### Reporting Issues

- [Bug Reports](https://github.com/torrust/torrust-tracker/issues/new?template=bug_report.md)
- [Feature Requests](https://github.com/torrust/torrust-tracker/issues/new?template=feature_request.md)

## Acknowledgements

This library draws inspiration from the Rust [metrics](https://crates.io/crates/metrics) crate, incorporating compatible APIs and naming conventions where possible. We may consider migrating to the standard metrics crate in future versions while maintaining our specialized functionality.

Special thanks to the Rust metrics ecosystem contributors for establishing excellent patterns for metrics collection and export.

## License

This project is licensed under the [GNU AFFERO GENERAL PUBLIC LICENSE v3.0](./LICENSE).

## Related Projects

- [Torrust Tracker](https://github.com/torrust/torrust-tracker) - The main BitTorrent tracker
- [metrics](https://crates.io/crates/metrics) - Standard Rust metrics facade
- [prometheus](https://crates.io/crates/prometheus) - Prometheus client library
