---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/statistics/mod.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/statistics/mod.rs
    - packages/tracker-core/src/statistics/event/mod.rs
    - packages/tracker-core/src/statistics/repository.rs
    - docs/adrs/20260901113500_define_completed_download_metric_retention_names.md
---

# Statistics Module File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 7:
`packages/tracker-core/src/statistics/mod.rs` and `packages/tracker-core/src/statistics/event/mod.rs`,
the last row of S1.

## Current State

- **Unit-only coverage:** `statistics/mod.rs` 42 / 43 lines, 32 / 33 regions, 2 / 2 functions, at
  the T1 baseline and after rows 1 to 6 (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`,
  `--all-features --lib`, 2026-11-18). Uncovered: line 49, the implicit `else` of
  `if tracker_usage_statistics_enabled`. `statistics/event/mod.rs` has no measured lines: it only
  declares the `handler` and `listener` modules.
- **Existing tests:** none in the file. `describe_metrics` runs through `Repository::new`; the
  repository tests (row 1) pin that the persisted total is absent when persisted statistics are
  disabled and present at zero when they are enabled. No test builds a repository with usage
  statistics disabled.
- **Module-owned decisions:** `describe_metrics(usage, persisted)` registers the persistent total
  (deprecated name) always, the in-session total only when `usage` is on, and the persisted total
  only when `persisted` is on, each at zero.

## Maintainer Decision

Plan approved on 2026-11-18, with the four-case table.

## Test Design

One table test in a new `tests` module of `statistics/mod.rs` (now one `rstest` case per combination; see Changes After Review):

| Test | Cases | Assert |
| --- | --- | --- |
| `it_should_register_each_downloads_total_only_when_its_statistics_are_enabled` | the four combinations of `(usage, persisted)` | for each case, the persistent total is `Some(0)`; the in-session total is `Some(0)` only with `usage`; the persisted total is `Some(0)` only with `persisted`; read with `metric_collection().get_counter_value(.., &LabelSet::default())`, and the message names the case |

`statistics/event/mod.rs`: no change, declarations only.

## Property-Test Decision

No `quickcheck`: the input is two booleans, all four cases listed.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | The in-session total is registered whatever `usage` says (the `if` removed) | the `usage = false` cases |
| M2 | The in-session condition reads `persisted` instead of `usage` | the mixed cases |
| M3 | The persistent total is registered only when `usage` is on | the `usage = false` cases |
| M4 | The persisted condition is negated | the table (and the row 1 repository tests) |

M1 and M2 survive today: no test disables usage statistics.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| C1 | DONE | Add the table test. | It passes; line 49 runs. |
| C2 | DONE | Run M1-M4. | Each is killed by the table. |
| C3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

- **Mutations** (each applied alone and reverted, run against the `statistics::` tests):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the table |
  | M2 | Killed by the table and by the row 1 test `it_should_omit_persisted_metric_when_persisted_completed_statistics_are_disabled` |
  | M3 | Killed by the table |
  | M4 | Killed by the table, the two row 1 persisted-metric tests and a row 3 test |

  Correction to the plan: only M1 survived before this change; M2 was already killed by the row 1
  repository test, which builds a repository with usage statistics on and persisted statistics off.
- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage
  target directory, 2026-11-18): `statistics/mod.rs` 42 / 43 lines to 60 / 60, regions 32 / 33 to
  56 / 56, functions 2 / 2 to 5 / 5. The package's unit-only lines 4,721 / 5,597 (84.35%) to
  4,739 / 5,614 (84.41%), regions 5,051 / 6,113 (82.63%), functions 771 / 1,015 (75.96%).
- **Tests:** one added; 180 unit tests in the package. `statistics/event/mod.rs`: no change
  (declarations only).
- **Review outcome:** the maintainer approved the completed file on 2026-11-18.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning) and `cargo +nightly fmt --all -- --check` pass.

## Changes After Review (#2988)

- **`rstest` cases (finding F2 of #2988, maintainer decision of 2026-11-22):** the table loops no
  longer. Each `(usage, persisted)` combination is an `rstest` case with its own name
  (`case_1_both_disabled` to `case_4_both_enabled`), as the `write-unit-test` skill prescribes, so
  each broken combination fails under its own name.
