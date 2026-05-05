# Refactor Plan: Split `metric_collection/mod.rs` into Submodules

## Goal

`packages/metrics/src/metric_collection/mod.rs` has grown large (~700 lines of
production code plus ~600 lines of tests). This plan splits it into focused
submodules **without changing any behaviour**. Each step is independently
verifiable by running `cargo test -p torrust-tracker-metrics` and `linter all`.

## Target Layout

```text
packages/metrics/src/metric_collection/
├── mod.rs              ← MetricCollection struct + domain methods + module
│                         declarations + re-exports
├── error.rs            ← Error enum
├── kind_collection.rs  ← MetricKindCollection<T> + Counter / Gauge
│                         specializations
├── serde.rs            ← JSON Serialize + Deserialize impls for MetricCollection
└── prometheus.rs       ← PrometheusSerializable + PrometheusDeserializable impls
                          for MetricCollection, plus all private helpers:
                            parse_prometheus_timestamp
                            collection_error
                            build_sample_collection
                            build_metric_collection
                            convert_openmetrics_label_set
                            counter_value_from_prom
                            gauge_value_from_prom
```

Tests can stay inline (`#[cfg(test)]` at the bottom of each file) or be moved
last after all production code is split. The test submodules
(`prometheus_timestamp`, `prometheus_deserialization`, etc.) should follow the
file that owns the code under test.

## Incremental Steps

### Step 1 — Extract `Error` into `error.rs`

- Create `packages/metrics/src/metric_collection/error.rs` containing the
  `Error` enum.
- In `mod.rs`: add `mod error;` + `pub use error::Error;`, remove the inline
  definition.
- **Verify**: `cargo test -p torrust-tracker-metrics` passes, `linter all`
  exits 0.

### Step 2 — Extract `MetricKindCollection` into `kind_collection.rs`

- Create `packages/metrics/src/metric_collection/kind_collection.rs` containing
  `MetricKindCollection<T>`, its generic impl blocks, and both typed
  specializations (`impl MetricKindCollection<Counter>` and
  `impl MetricKindCollection<Gauge>`).
- In `mod.rs`: add `mod kind_collection;` + `pub use kind_collection::MetricKindCollection;`,
  remove the inline code.
- Move the `metric_kind_collection` test submodule into `kind_collection.rs`.
- **Verify**: `cargo test -p torrust-tracker-metrics` passes, `linter all`
  exits 0.

### Step 3 — Extract JSON serde into `serde.rs`

- Create `packages/metrics/src/metric_collection/serde.rs` containing the
  `impl Serialize for MetricCollection` and `impl Deserialize for MetricCollection`
  blocks.
- In `mod.rs`: add `mod serde;` (no re-export needed — trait impls are
  automatically visible).
- Move the JSON-related tests (`it_should_allow_serializing_to_json`,
  `it_should_allow_deserializing_from_json`) and the `MetricCollectionFixture`
  into `serde.rs` (or keep the fixture in `mod.rs` if it is shared by Prometheus
  tests too — see note below).
- **Verify**: `cargo test -p torrust-tracker-metrics` passes, `linter all`
  exits 0.

> **Note on the shared fixture**: `MetricCollectionFixture` is used by both the
> JSON and Prometheus tests. If it remains shared, keep it in `mod.rs` inside
> `#[cfg(test)]`. If each file gets its own copy, it can be duplicated or
> extracted to a `tests/fixture.rs` helper.

### Step 4 — Extract Prometheus impls into `prometheus.rs`

- Create `packages/metrics/src/metric_collection/prometheus.rs` containing:
  - `impl PrometheusSerializable for MetricCollection`
  - All private helpers (`parse_prometheus_timestamp`, `collection_error`,
    `build_sample_collection`, `build_metric_collection`,
    `convert_openmetrics_label_set`, `counter_value_from_prom`,
    `gauge_value_from_prom`)
  - `impl PrometheusDeserializable for MetricCollection`
- In `mod.rs`: add `mod prometheus;` (no re-export needed — trait impls are
  automatically visible).
- Move the `prometheus_timestamp` and `prometheus_deserialization` test
  submodules into `prometheus.rs`.
- **Verify**: `cargo test -p torrust-tracker-metrics` passes, `linter all`
  exits 0.

### Step 5 — Clean up `mod.rs`

After all four extractions, `mod.rs` should contain only:

- Module declarations (`mod error; mod kind_collection; mod serde; mod prometheus;`)
- `pub use` re-exports (`Error`, `MetricKindCollection`, `aggregate`)
- `MetricCollection` struct definition
- All `impl MetricCollection` blocks (domain methods)
- The remaining tests (collection-level tests: name collision, merge, etc.)

- **Verify**: `cargo test -p torrust-tracker-metrics` passes, `linter all`
  exits 0.

## Verification Command Reference

```sh
# Run all tests for the metrics package
cargo test -p torrust-tracker-metrics

# Run all linters (must exit 0 before committing)
linter all
```

## Commit Strategy

One commit per step. Each commit message should follow Conventional Commits:

```text
refactor(metrics): extract Error into metric_collection/error.rs
refactor(metrics): extract MetricKindCollection into kind_collection.rs
refactor(metrics): extract JSON serde impls into metric_collection/serde.rs
refactor(metrics): extract Prometheus impls into metric_collection/prometheus.rs
refactor(metrics): clean up metric_collection/mod.rs
```
