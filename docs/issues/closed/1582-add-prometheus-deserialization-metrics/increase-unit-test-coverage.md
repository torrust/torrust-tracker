# Increase Unit Test Coverage for the `metrics` Package

## Overview

After implementing `PrometheusDeserializable for MetricCollection` and the subsequent
five-step module split of `metric_collection/mod.rs`, several source files have no test
coverage at all and several others have only minimal happy-path tests. This plan tracks
the work to close those gaps.

## Baseline (as of commit `7ba33c28`)

- **Total tests**: 225
- **Overall line coverage**: 85.72% (6970 instrumented lines, 995 uncovered)

Coverage report from `cargo llvm-cov --package torrust-tracker-metrics --summary-only`:

| File                                   | Lines | Uncovered |     Line % | Functions |   Fn % | Regions | Region % |
| -------------------------------------- | ----: | --------: | ---------: | --------: | -----: | ------: | -------: |
| `counter.rs`                           |   298 |         0 |   **100%** |        36 |   100% |     165 |     100% |
| `gauge.rs`                             |   260 |         0 |   **100%** |        33 |   100% |     149 |     100% |
| `label/name.rs`                        |    35 |         0 |   **100%** |         4 |   100% |      27 |     100% |
| `label/pair.rs`                        |    22 |         0 |   **100%** |         2 |   100% |       9 |     100% |
| `label/set.rs`                         |   817 |         1 | **99.88%** |        62 |   100% |     401 |     100% |
| `label/value.rs`                       |    90 |         0 |   **100%** |        13 |   100% |      54 |     100% |
| `lib.rs`                               |    17 |         0 |   **100%** |         2 |   100% |      13 |     100% |
| `metric/aggregate/avg.rs`              |   256 |         0 |   **100%** |         9 |   100% |     198 |     100% |
| `metric/aggregate/sum.rs`              |   230 |         0 |   **100%** |        13 |   100% |     194 |     100% |
| `metric/description.rs`                |    29 |         0 |   **100%** |         5 |   100% |      18 |     100% |
| `metric/mod.rs`                        |   459 |         0 |   **100%** |        35 |   100% |     189 |     100% |
| `metric/name.rs`                       |    87 |         0 |   **100%** |         6 |   100% |      40 |     100% |
| `metric_collection/aggregate/avg.rs`   |   190 |         0 |   **100%** |        10 |   100% |     103 |     100% |
| `metric_collection/aggregate/sum.rs`   |   103 |         2 | **98.06%** |         7 |   100% |      57 |   96.49% |
| `metric_collection/error.rs`           |     — |         — |    **n/a** |         — |      — |       — |        — |
| `metric_collection/kind_collection.rs` |   245 |         0 |   **100%** |        19 |   100% |     102 |     100% |
| `metric_collection/mod.rs`             |  1007 |         6 | **99.40%** |        45 |   100% |     542 |     100% |
| `metric_collection/prometheus.rs`      |   566 |        65 | **88.52%** |        38 | 78.95% |     301 |   84.39% |
| `metric_collection/serde.rs`           |   146 |         7 | **95.21%** |         6 |   100% |     121 |     100% |
| `prometheus.rs`                        |     4 |         0 |   **100%** |         1 |   100% |       3 |     100% |
| `sample.rs`                            |   452 |         8 | **98.23%** |        48 | 93.75% |     234 |   98.72% |
| `sample_collection.rs`                 |   755 |         4 | **99.47%** |        42 | 97.62% |     290 |   99.66% |
| `unit.rs`                              |     — |         — |    **n/a** |         — |      — |       — |        — |

> `n/a` means llvm-cov reports no instrumented lines (only `derive`-based code, no executable
> statements), so line coverage is not tracked. These files still benefit from tests that
> exercise the derived traits and error messages.

- **Priority targets** (files below 100% with meaningful gaps):

| File                                 | Line % | Uncovered lines | Action                                 |
| ------------------------------------ | -----: | --------------: | -------------------------------------- |
| `metric_collection/prometheus.rs`    | 88.52% |              65 | Highest priority — 8 functions not hit |
| `metric_collection/serde.rs`         | 95.21% |               7 | Error paths untested                   |
| `metric_collection/aggregate/sum.rs` | 98.06% |               2 | Edge cases missing                     |
| `metric_collection/mod.rs`           | 99.40% |               6 | Minor gaps                             |
| `sample.rs`                          | 98.23% |               8 | 3 functions not hit                    |
| `sample_collection.rs`               | 99.47% |               4 | 1 function not hit                     |
| `label/set.rs`                       | 99.88% |               1 | 1 line — negligible                    |
| `unit.rs`                            |    n/a |               — | Serde round-trip tests missing         |
| `metric_collection/error.rs`         |    n/a |               — | `Display` message tests missing        |

## Goals

Ordered by impact (highest uncovered lines first):

- [ ] Expand `metric_collection/prometheus.rs` tests — 88.52% line coverage (65 uncovered, 8 functions never hit)
- [ ] Expand `metric_collection/serde.rs` tests — 95.21% line coverage (7 uncovered lines)
- [ ] Expand `sample.rs` tests — 98.23% line coverage (8 uncovered lines, 3 functions never hit)
- [ ] Expand `sample_collection.rs` tests — 99.47% line coverage (4 uncovered lines, 1 function never hit)
- [ ] Expand `metric_collection/aggregate/sum.rs` tests — 98.06% line coverage (2 uncovered lines)
- [ ] Add tests for `unit.rs` — no instrumented lines (serde round-trip coverage missing)
- [ ] Add tests for `metric_collection/error.rs` — no instrumented lines (`Display` messages untested)

## Implementation Plan

### Task 1: `metric_collection/prometheus.rs` — cover 8 missing functions

**File**: `packages/metrics/src/metric_collection/prometheus.rs`

Current: 88.52% lines / 78.95% functions (8 functions never executed).

Run `cargo llvm-cov --package torrust-tracker-metrics --open` and inspect the annotated
HTML to identify the exact uncovered branches before writing tests.

- [ ] `it_should_return_unknown_value_error_for_unknown_prometheus_value`
- [ ] `it_should_return_label_conversion_error_when_label_name_is_invalid`
- [ ] `it_should_return_unknown_type_error_for_unrecognised_metric_type`
- [ ] `it_should_return_collection_error_when_building_from_duplicate_names`
- [ ] Cover remaining uncovered branches identified from HTML report

### Task 2: `metric_collection/serde.rs` — cover 7 uncovered lines

**File**: `packages/metrics/src/metric_collection/serde.rs`

Current: 95.21% lines (7 uncovered).

- [ ] `it_should_fail_deserializing_json_with_unknown_metric_type` — unknown `"type"` field → error
- [ ] `it_should_fail_deserializing_json_with_duplicate_metric_names` — collision → error
- [ ] `it_should_allow_serializing_an_empty_collection_to_json` — empty → `[]`
- [ ] `it_should_allow_deserializing_an_empty_json_array` — `[]` → empty collection

### Task 3: `sample.rs` — cover 3 missing functions

**File**: `packages/metrics/src/sample.rs`

Current: 98.23% lines / 93.75% functions (3 functions never executed).

- [ ] Inspect HTML report to identify the 3 uncovered functions
- [ ] Add targeted tests for each

### Task 4: `sample_collection.rs` — cover 1 missing function

**File**: `packages/metrics/src/sample_collection.rs`

Current: 99.47% lines / 97.62% functions (1 function never executed).

- [ ] Inspect HTML report to identify the uncovered function
- [ ] Add a targeted test

### Task 5: `metric_collection/aggregate/sum.rs` — cover 2 uncovered lines

**File**: `packages/metrics/src/metric_collection/aggregate/sum.rs`

Current: 98.06% lines (2 uncovered).

- [ ] `nonexistent_metric` — `sum()` returns `None` for a metric name not in the collection
- [ ] `empty_collection` — `sum()` returns `None` on a default empty collection

### Task 6: `unit.rs` — add serde tests

**File**: `packages/metrics/src/unit.rs`

No instrumented lines (pure `derive`-based enum), but serde correctness is untested.

- [ ] `it_should_serialize_each_variant_to_snake_case_json` — verify `rename_all = "snake_case"` for all 17 variants
- [ ] `it_should_deserialize_each_variant_from_snake_case_json` — round-trip via `serde_json`
- [ ] `it_should_implement_clone_copy_eq_hash_debug` — derive trait smoke test

### Task 7: `metric_collection/error.rs` — add `Display` message tests

**File**: `packages/metrics/src/metric_collection/error.rs`

No instrumented lines (pure `derive`/`thiserror`-based enum), but error messages are untested.

- [ ] `it_should_format_metric_name_collision_in_constructor_error_message`
- [ ] `it_should_format_duplicate_metric_name_in_list_error_message`
- [ ] `it_should_format_metric_name_collision_in_merge_error_message`
- [ ] `it_should_format_metric_name_collision_adding_error_message`
- [ ] `it_should_be_cloneable`

## Acceptance Criteria

- [ ] All new tests pass (`cargo test -p torrust-tracker-metrics`)
- [ ] No existing tests regress
- [ ] `linter all` exits with code `0`
- [ ] `metric_collection/prometheus.rs` line coverage ≥ **95%** (currently 88.52%)
- [ ] `metric_collection/serde.rs` line coverage = **100%** (currently 95.21%)
- [ ] `sample.rs` line coverage = **100%** (currently 98.23%)
- [ ] `sample_collection.rs` line coverage = **100%** (currently 99.47%)
- [ ] Overall package line coverage ≥ **95%** (currently 85.72%; note: the gap is inflated by
      zero-coverage dependency crates that appear in the report)

## References

- Issue: [#1582](https://github.com/torrust/torrust-tracker/issues/1582)
- PR: [#1729](https://github.com/torrust/torrust-tracker/pull/1729)
- Branch: `1582-add-prometheus-deserialization-metrics`
- Refactor plan: [metric-collection-module-split.md](metric-collection-module-split.md)
