---
doc-type: issue
issue-type: feature
status: done
priority: p2
github-issue: 1582
spec-path: docs/issues/closed/1582-add-prometheus-deserialization-metrics/ISSUE.md
branch: 1582-add-prometheus-deserialization-metrics
related-pr: null
last-updated-utc: null
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - packages/metrics/
---

# Add Deserialization from Prometheus Text Format in `metrics` Package

## Overview

`MetricCollection` can already be serialized to and from JSON, and serialized to the Prometheus
exposition text format via `PrometheusSerializable`. This issue adds the **deserialization**
direction: parsing a Prometheus exposition text string back into a `MetricCollection`.

The primary motivation is to make tests more expressive. Instead of building metrics
programmatically with a `MetricBuilder`, tests can round-trip through a Prometheus string:

```rust
// Before (verbose)
MetricBuilder::default()
    .with_sample(1.into(), &[("l1", "l1_value")].into())
    .build()

// After (expressive)
MetricCollection::from_prometheus(r#"test_metric{l1="l1_value"} 1"#, now)
```

A previous contribution (PR #1611 by `@naoNao89`) implemented a working version using the
`openmetrics-parser` crate. This spec incorporates the maintainer feedback from that PR so we
can land a clean, idiomatic implementation.

## Goals

- [ ] Add a `PrometheusDeserializable` trait in `packages/metrics/src/prometheus.rs` mirroring
      `PrometheusSerializable`
- [ ] Implement `PrometheusDeserializable` for `MetricCollection` using the `openmetrics-parser`
      crate
- [ ] Define a dedicated, fine-grained error type for Prometheus parsing in `prometheus.rs`
- [ ] Implement `TryFrom<openmetrics_parser::LabelSet>` for our `LabelSet` to avoid ad-hoc
      conversion code
- [ ] Extract the timestamp-parsing helper into a private free function
- [ ] Pass `linter all` and `cargo machete` with zero warnings

## Background and Prior Art

PR #1611 was submitted by `@naoNao89` and was well-received conceptually (`@da2ce7`: "this looks
much better and cleaner"). It stalled due to CI failures, merge conflicts, and unaddressed
maintainer feedback. The implementation approach (using `openmetrics-parser`) is sound and should
be preserved.

Key feedback that must be addressed:

1. **Trait placement** — deserialization should live as a `PrometheusDeserializable` trait in
   `packages/metrics/src/prometheus.rs`, alongside `PrometheusSerializable`.

2. **Error granularity** — a single catch-all error is insufficient. See the error design below.

3. **Code duplication** — the timestamp-parsing block was copy-pasted for `Counter` and `Gauge`.
   Extract it into a helper function.

4. **Silent unknowns** — returning `0` for `PrometheusValue::Unknown` silently discards data.
   Unknown values should be an error.

5. **Conversion via `TryFrom`** — the inline label-set conversion should be a `TryFrom` impl.

## Design

### Trait

Add to `packages/metrics/src/prometheus.rs`:

```rust
pub trait PrometheusDeserializable: Sized {
    /// Parse a Prometheus exposition text format string into `Self`.
    ///
    /// `now` is used as the sample timestamp when the exposition text does not
    /// include a timestamp for a given sample.
    ///
    /// # Errors
    ///
    /// Returns an error if the input cannot be parsed or contains unsupported
    /// or unknown metric types/values.
    fn from_prometheus(input: &str, now: DurationSinceUnixEpoch) -> Result<Self, PrometheusDeserializationError>;
}
```

### Error Type

Define a dedicated `PrometheusDeserializationError` enum in `packages/metrics/src/prometheus.rs`.
Keep it separate from `metric_collection::Error` so it can be reused if other types ever
implement the trait.

```rust
#[derive(thiserror::Error, Debug, Clone)]
pub enum PrometheusDeserializationError {
    /// The Prometheus text could not be parsed at all (syntax error).
    #[error("Failed to parse Prometheus exposition text: {message}")]
    ParseError { message: String },

    /// The parser emitted a metric type that is syntactically valid but that
    /// this implementation does not yet support (e.g. Histogram, Summary).
    #[error("Unsupported Prometheus metric type '{metric_type}' for metric '{metric_name}'")]
    UnsupportedType { metric_name: String, metric_type: String },

    /// The parser emitted a metric type that is not recognised at all.
    #[error("Unknown Prometheus metric type for metric '{metric_name}'")]
    UnknownType { metric_name: String },

    /// The value in the exposition does not match the declared metric type.
    #[error("Value mismatch for metric '{metric_name}': expected {expected_type}, got {actual}")]
    ValueMismatch { metric_name: String, expected_type: String, actual: String },

    /// The value is of an unknown/unrecognised kind.
    #[error("Unknown value for metric '{metric_name}'")]
    UnknownValue { metric_name: String },

    /// The label set could not be converted (e.g. invalid label name or value).
    #[error("Failed to convert label set for metric '{metric_name}': {message}")]
    LabelConversion { metric_name: String, message: String },

    /// A structural error when assembling the `MetricCollection` from parsed data.
    #[error("Failed to build MetricCollection: {0}")]
    CollectionError(#[from] crate::metric_collection::Error),
}
```

### `TryFrom` for `LabelSet`

Add to `packages/metrics/src/label/set.rs` (or a new
`packages/metrics/src/label/set/from_openmetrics.rs`):

```rust
// Feature-gated or in a dedicated submodule so the openmetrics-parser dep
// is clearly scoped.
impl TryFrom<openmetrics_parser::LabelSet<'_>> for LabelSet {
    type Error = PrometheusDeserializationError;

    fn try_from(parser_set: openmetrics_parser::LabelSet<'_>) -> Result<Self, Self::Error> {
        // ...
    }
}
```

### Timestamp Helper

Extract into a private function in `metric_collection/mod.rs` (or a new submodule):

```rust
fn parse_prometheus_timestamp(t: f64, fallback: DurationSinceUnixEpoch) -> DurationSinceUnixEpoch {
    if t.is_finite() && t >= 0.0 {
        let secs = t.trunc() as u64;
        let nanos = ((t - t.trunc()) * 1_000_000_000.0).round() as u32;
        let (secs, nanos) = if nanos >= 1_000_000_000 {
            (secs + 1, nanos - 1_000_000_000)
        } else {
            (secs, nanos)
        };
        DurationSinceUnixEpoch::new(secs, nanos)
    } else {
        fallback
    }
}
```

## Implementation Plan

### Task 0: Explore current state of the `metrics` package

Before writing any code, read the current codebase to confirm what has changed since PR #1611
(the package has evolved). Specifically check:

- [ ] `packages/metrics/src/prometheus.rs` — current trait surface
- [ ] `packages/metrics/src/metric_collection/mod.rs` — current `Error` enum and `MetricCollection` API
- [ ] `packages/metrics/src/label/set.rs` — existing `From` impls
- [ ] `packages/metrics/Cargo.toml` — existing dependencies

### Task 1: Add `openmetrics-parser` dependency

- [ ] Add `openmetrics-parser = "0.4.4"` to `packages/metrics/Cargo.toml` under `[dependencies]`
- [ ] Run `cargo fetch` to update `Cargo.lock`
- [ ] Verify `cargo build -p metrics` compiles cleanly

### Task 2: Add `PrometheusDeserializable` trait and `PrometheusDeserializationError`

- [ ] Open `packages/metrics/src/prometheus.rs`
- [ ] Add `use torrust_tracker_primitives::DurationSinceUnixEpoch;` import
- [ ] Add the `PrometheusDeserializable` trait (see Design section)
- [ ] Add the `PrometheusDeserializationError` enum (see Design section)
- [ ] Run `cargo build -p metrics` — expect clean compile

### Task 3: Implement `TryFrom<openmetrics_parser::LabelSet>` for our `LabelSet`

- [ ] Add the `TryFrom` impl in `packages/metrics/src/label/set.rs`
- [ ] Write a unit test confirming a round-trip: known labels survive the conversion
- [ ] Write a unit test confirming conversion errors are propagated correctly
- [ ] Run `cargo test -p metrics` — all tests pass

### Task 4: Extract the timestamp helper

- [ ] Add `parse_prometheus_timestamp(t: f64, fallback: DurationSinceUnixEpoch) -> DurationSinceUnixEpoch`
      as a private free function in `packages/metrics/src/metric_collection/mod.rs`
- [ ] Write a unit test for the helper (edge cases: negative, NaN, ±Inf, nano-second boundary)

### Task 5: Implement `PrometheusDeserializable` for `MetricCollection`

- [ ] Add `impl PrometheusDeserializable for MetricCollection` in
      `packages/metrics/src/metric_collection/mod.rs`
- [ ] Use `parse_prometheus_timestamp` for both Counter and Gauge paths
- [ ] Use `LabelSet::try_from(...)` for label conversion
- [ ] Return `PrometheusDeserializationError::UnknownValue` instead of `0` for
      `PrometheusValue::Unknown`
- [ ] Return `PrometheusDeserializationError::ValueMismatch` for type mismatches
- [ ] Return `PrometheusDeserializationError::UnsupportedType` for Histogram, Summary, etc.
- [ ] Return `PrometheusDeserializationError::UnknownType` for the catch-all `other` arm
- [ ] Run `cargo test -p metrics` — all tests pass

### Task 6: Add round-trip tests

- [ ] Add `it_should_deserialize_a_counter_metric_from_prometheus_text` test
- [ ] Add `it_should_deserialize_a_gauge_metric_from_prometheus_text` test
- [ ] Add `it_should_round_trip_serialize_then_deserialize_prometheus_text` test using the
      existing `MetricCollectionFixture`
- [ ] Add a test that verifies `UnsupportedType` is returned for an unsupported family
- [ ] Add a test that verifies `ParseError` is returned for malformed input
- [ ] Run `cargo test -p metrics` — all tests pass

### Task 7: Lint and hygiene

- [ ] Run `cargo fmt --all`
- [ ] Run `linter all` — exit code `0`
- [ ] Run `cargo machete` — no unused dependencies

## Acceptance Criteria

- [ ] `PrometheusDeserializable` trait defined in `packages/metrics/src/prometheus.rs`
- [ ] `PrometheusDeserializationError` with the six variants defined above
- [ ] No silent `0` returns for unknown/mismatched values — all become errors
- [ ] `TryFrom<openmetrics_parser::LabelSet>` for our `LabelSet` exists
- [ ] Timestamp logic is deduplicated into a single private helper
- [ ] All new code is covered by unit tests
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] `cargo test --workspace` passes

## References

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1582>
- Prior PR: <https://github.com/torrust/torrust-tracker/pull/1611> (by `@naoNao89`)
- `openmetrics-parser` crate: <https://crates.io/crates/openmetrics-parser>
- `PrometheusSerializable` trait: `packages/metrics/src/prometheus.rs`
- `MetricCollection`: `packages/metrics/src/metric_collection/mod.rs`
- `LabelSet`: `packages/metrics/src/label/set.rs`
