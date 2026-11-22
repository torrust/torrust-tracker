---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/statistics/metrics.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/statistics/metrics.rs
    - packages/tracker-core/src/statistics/repository.rs
    - docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans/statistics-repository-tests.md
---

# Statistics Metrics File Test Plan

Follow the shared [guidance](README.md). This record covers only
`packages/tracker-core/src/statistics/metrics.rs`, ledger row 2. Decision: **no change**.

## Current State

- **Unit-only coverage:** 11 / 45 lines at the S1 baseline; 45 / 45 after the
  [repository tests](statistics-repository-tests.md) (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`,
  `--all-features --lib`; re-measured on 2026-11-18, the same figures). The baseline left 36-44, 49-57, 62-69 and 74-81 uncovered:
  `set_counter`, `set_gauge`, `increment_gauge` and `decrement_gauge`.
- **What the file decides:** nothing of its own. `Metrics` holds a `MetricCollection`; its accessor
  returns it and its five write methods pass their arguments to the method of the same name and
  return its result unchanged.
- **Callers in the package:** only `statistics/repository.rs`, which keeps `Metrics` behind a lock
  and calls each write method. `statistics/mod.rs` (`describe_metrics`) builds a `Metrics` but
  writes to its `metric_collection` field directly.
- **Existing tests:** none in the file.

## Decision

No change: a test here would repeat the repository tests one layer down, without the lock, and
kill no mutant they leave alive. Every line is covered by the repository tests, which assert the
values the collection holds after each write.

## Mutation Proof

Hand-made mutants, each applied alone and reverted; `cargo test -p torrust-tracker-core
--all-features --lib` on 2026-11-17.

| ID | Mutation | Result |
| --- | --- | --- |
| M1 | `increment_counter` returns `Ok(())` without writing | Killed by the repository and event tests |
| M2 | `set_counter` returns `Ok(())` without writing | Killed: `it_should_set_a_counter_to_the_given_value` and the name-collision test |
| M3 | `set_gauge` returns `Ok(())` without writing | Killed by the six gauge and name-collision tests |
| M4 | `increment_gauge` calls `decrement_gauge` | Killed: `it_should_increment_a_gauge_by_one` |
| M5 | `decrement_gauge` calls `increment_gauge` | Killed: `it_should_decrement_a_gauge_by_one` |
| M6 | `set_counter` writes `value + 1` | Killed: `it_should_set_a_counter_to_the_given_value` |

The accessor `metric_collection()` has no viable mutant: it returns a reference to the only field.

## Risk Accepted

The file relies on tests in `repository.rs`. If `Repository` stopped calling one of the methods,
that method would lose its coverage without a failing test; the methods are `pub`, so the
`unused` lint would not report them either. The next coverage run would show it.

## Results

- **Decision:** approved by the maintainer on 2026-11-18 (no change, with the mutation proof as
  evidence).
- **Tests:** none added or changed.
