---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/stats/handlers.rs
status: in-progress
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/stats/handlers.rs
    - packages/axum-rest-api-server/src/v1/context/stats/responses.rs
    - packages/axum-rest-api-server/src/test_support.rs
    - packages/rest-api-application/src/v1/ports/stats.rs
    - packages/rest-api-application/src/v1/use_cases/stats.rs
    - packages/rest-api-protocol/src/v1/context/stats/resources/stats.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-torrent-handlers-tests.md
---

# Stats Handlers File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/stats/handlers.rs`, ledger row 15 (rank 5), the fourth file of S4. It
reuses the pattern of the torrent handlers ([plan](v1-context-torrent-handlers-tests.md)): the handlers are called
directly, over a hand-written fake of the application port, and the response is read with the shared `observed` helper.

## Current State

- **Unit-only coverage:** 12 / 20 lines (60.00%), measured at the S4 baseline (stable Rust 1.99.1,
  `cargo-llvm-cov 0.6.16`, clean workspace, `--lib`). The `v1/routes.rs` tests reach the registration only; the
  uncovered lines are the `format` dispatch of both handlers.
- **Existing tests:** none in the file. `tests/server/v1/contract/context/stats.rs` has 4 integration tests: the JSON
  statistics, the `format=prometheus` metrics with the persisted metric omitted and with it enabled at zero, and the
  unauthenticated case.
- **Module-owned decisions:**
  - `get_stats_handler` asks the service for the statistics once and answers them as JSON (no `format`, or
    `format=json`) or as the plain-text metric lines (`format=prometheus`).
  - `get_metrics_handler` does the same with the labeled statistics: JSON, or Prometheus exposition text.
  - `Format` reads `json` and `prometheus` in lowercase only; any other value is rejected by the extractor before the
    handler runs.
  - Owned elsewhere: the content of the answers (row 16, the responses file), the statistics (the runtime adapter), the
    JSON shape of `Stats` and `LabeledStats` (`rest-api-protocol`) and the Prometheus text of a metric collection
    (`torrust-metrics`).
- **T1 hypothesis:** test candidate.

## Maintainer Decisions

The maintainer approved the plan, with the shared fixtures `a_stats` and `a_labeled_stats` in `src/test_support.rs`
(rather than a copy in each file), on 2026-11-04.

## Current Tests Review

No tests in the file, so R1 is skipped.

## Coverage And Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| `get_stats_handler` default and `format=json` | 3A unit | R2: `200`, `application/json`, a body that starts with the JSON object of the statistics; the fake records exactly one `get_stats` call and no `get_labeled_stats` call. |
| `get_stats_handler` with `format=prometheus` | 3A unit | R2: `200`, `text/plain; charset=utf-8`, a body that starts with the first metric line; one `get_stats` call. |
| `get_metrics_handler` default and `format=json` | 3A unit | R2: `200`, `application/json`, a body that starts with the JSON object of the labeled metrics; one `get_labeled_stats` call. |
| `get_metrics_handler` with `format=prometheus` | 3A unit | R2: `200`, `text/plain; charset=utf-8`, a body that holds the Prometheus text of the collection; one `get_labeled_stats` call. |
| `Format` and `QueryParams` deserialization | 3A unit | R2: through the extractor the router uses (`axum::extract::Query::try_from_uri`): no `format`, `json` and `prometheus` are read; `JSON` (capitals), `xml` and an empty `format=` are rejected. |
| The exact body of each answer | Row 16 | The complete JSON and Prometheus literals are asserted in the tests of `responses.rs`, so this file asserts only what identifies the chosen builder (status, content type and the start of the body). |
| The statistics values and the registry behind the port | Collaborator-owned | The runtime adapter and tracker-core; the four integration tests keep protecting the end to end behavior. |
| `Format`'s `Debug` and `Default` implementations | Not tested | Derived; `Default` is not used by the handlers (an absent value is an `Option::None`). |

### Equivalent mutant, recorded in advance

`#[serde(default)]` on `QueryParams::format` is redundant: serde already reads a missing `Option` field as `None`. The
mutation that removes it will survive by design and is recorded as such, not a gap.

### Cases for R2

| Handler | Query | Expected status | Content type | Body (checked) | Port calls |
| --- | --- | --- | --- | --- | --- |
| stats | none | `200` | `application/json` | starts with `{"torrents":1,` | `get_stats` |
| stats | `format=json` | `200` | `application/json` | starts with `{"torrents":1,` | `get_stats` |
| stats | `format=prometheus` | `200` | `text/plain; charset=utf-8` | starts with `torrents 1\nseeders 2\n` | `get_stats` |
| metrics | none | `200` | `application/json` | starts with `{"metrics":[` | `get_labeled_stats` |
| metrics | `format=json` | `200` | `application/json` | starts with `{"metrics":[` | `get_labeled_stats` |
| metrics | `format=prometheus` | `200` | `text/plain; charset=utf-8` | contains `requests_total 1` | `get_labeled_stats` |

## Test Design

- **Fake:** a `FakeStatsQueryPort` implementing `StatsQueryPort` that answers a fixed `Stats` and a fixed
  `LabeledStats` (`Stats` and `LabeledStats` are not `Clone`, so it builds them with two function pointers) and records
  a test-local `Call` enum (`GetStats`, `GetLabeledStats`) in an `Arc<Mutex<Vec<Call>>>`.
- **Fixtures (approved):** `Stats` has 33 fields and no `Default`, and rows 15 and 16 both need a
  value whose fields are all different (so a swapped label shows). I propose one crate-private fixture in
  `src/test_support.rs`: `a_stats()` (fields `1` to `33` in the declaration order, `completed_persisted_enabled` true)
  and `a_labeled_stats()` (one counter `requests_total` with the value `1`). It is test data with one responsibility,
  used by two files, and the shared helper of this package already lives there. The alternative is a copy of the
  33-field value in each file.
- **Helpers:** the shared `observed`; the query is read with `Query::<QueryParams>::try_from_uri` as in the torrent
  tests.
- **Dependencies:** none new. `torrust-metrics` and `torrust-clock` are already regular dependencies of the package, and
  a collection with one counter is built with `MetricCollection::increment_counter`.

## Property-Test Decision

Not flagged and not selected: the handlers choose between two builders.

## Dependencies

None new.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | TODO | Add the fixtures to `test_support.rs` and the in-file `#[cfg(test)]` module with the fake and the cases above. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | TODO | Integration increment: `none` expected. | The 4 stats integration tests keep protecting the wire contract. |
| R4 | TODO | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change a test must catch; revert it exactly after the run. A mutant that leaves an import
or variable unused does not compile under `unused = "deny"`, so such a mutant is redone so that it compiles.

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| S1 | `get_stats_handler` | No `format` answers Prometheus. | the default stats case |
| S2 | `get_stats_handler` | `Format::Json` answers Prometheus. | the `format=json` stats case |
| S3 | `get_stats_handler` | `Format::Prometheus` answers JSON. | the Prometheus stats case |
| S4 | `get_metrics_handler` | No `format` answers Prometheus. | the default metrics case |
| S5 | `get_metrics_handler` | `Format::Json` answers Prometheus. | the `format=json` metrics case |
| S6 | `get_metrics_handler` | `Format::Prometheus` answers JSON. | the Prometheus metrics case |
| S7 | `get_stats_handler` | Also call `get_labeled_stats` (result dropped). | the call-log assertion of the stats cases |
| S8 | `Format` | Remove `rename_all = "lowercase"`. | the deserialization cases |
| S9 | `QueryParams` | Remove `#[serde(default)]` from `format`. | none: an equivalent mutant (see above) |

## Prose-First AAA Review

Written after the tests, before comparing them with the code.

## Results

Written after the tests.
