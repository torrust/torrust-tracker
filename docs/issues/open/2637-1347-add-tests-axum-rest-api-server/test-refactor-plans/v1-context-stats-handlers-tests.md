---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/stats/handlers.rs
status: done
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
- **Fixtures (approved):** `Stats` has 32 fields and no `Default`, and rows 15 and 16 both need a
  value whose fields are all different (so a swapped label shows). I propose one crate-private fixture in
  `src/test_support.rs`: `a_stats()` (each field is its position from `1` to `32` in the declaration order; the boolean `completed_persisted_enabled` is true)
  and `a_labeled_stats()` (one counter `requests_total` with the value `1`). It is test data with one responsibility,
  used by two files, and the shared helper of this package already lives there. The alternative is a copy of the
  32-field value in each file.
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
| R2 | DONE | Add the fixtures to `test_support.rs` and the in-file `#[cfg(test)]` module with the fake and the cases above. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | DONE | Integration increment: `none` expected. | The 4 stats integration tests keep protecting the wire contract. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

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

S3 and S6 first failed to compile (they removed the last use of an import under `unused = "deny"`) and were redone so that the original call stays and its result is dropped.

## Prose-First AAA Review

Temporary prose, written before comparing with the code.

| Test (cases) | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_answer_the_statistics_in_the_requested_format` (3) | A fake that answers the fixed statistics and records the call; a query with no format, `json` or `prometheus`. | `get_stats_handler` with the query read by the real extractor. | Status `200`, the content type, that the body starts with the expected text, and that the one call was `get_stats`; the body is printed when the tuple differs. | The body is checked only at its start because the full literals belong to row 16. Kept. |
| `it_should_answer_the_labeled_metrics_in_the_requested_format` (3) | The same fake; a query with no format, `json` or `prometheus`. | `get_metrics_handler`. | Status `200`, the content type, that the body holds the expected part, and that the one call was `get_labeled_stats`. | As above. Kept. |
| `it_should_read_the_format_in_lowercase` (3) | A query string only. | Read the query with the extractor. | The parsed format, mapped by an exhaustive `match` to a small local `Asked` enum, equals `Nothing`, `Json` or `Prometheus`. | `Format` has no `PartialEq`, and no production derive was added for a test. The first version compared the `Debug` string; Copilot's review of #2718 called that brittle, so it now matches the variants (a new variant breaks the build). Changed after review. |
| `it_should_reject_a_format_that_is_not_json_or_prometheus` (3) | A query string with capitals, an unknown value or an empty value. | Read the query with the extractor. | The extractor rejects it; the message is not asserted (owned by `axum`). | Kept. |

Helpers: `FakeStatsQueryPort` and `fake_with_log` are the Arrange mechanics; `query`, `uri` and `recorded` read the query
and the log; the shared `observed` comes from `test_support`, which also holds the new fixtures `a_stats` and
`a_labeled_stats`.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): `stats/handlers.rs` 12 / 20
  lines at the S4 baseline; 40 / 40 lines (100.00%), 61 / 61 regions and 14 / 14 functions after (the counts include
  the test module). `test_support.rs` is 109 / 109 lines (63 / 63 before the two fixtures). Package unit-only lines after
  this file: 1,957 / 2,065 (94.77%), regions 2,932 / 3,116 (94.09%), functions 325 / 353 (92.07%), 152 unit tests.
- **Covered but not asserted:** `stats/responses.rs` (row 16) is now 45 / 45 lines, because the Prometheus case of
  `get_stats_handler` runs all 32 metric lines of `metrics_response` while asserting only the first two. The line figure
  hides that; row 16 asserts every line.
- **Tests:** 4 test functions, 12 test cases counting the tables; the package went from 140 to 152 unit tests. No
  integration test was added or changed.
- **Mutations** (each applied alone, stable toolchain, then reverted; the file was identical to the tested one
  afterwards): S1 killed by the default stats case, S2 by the `json` stats case, S3 by the Prometheus stats case, S4 by
  the default metrics case, S5 by the `json` metrics case, S6 by the Prometheus metrics case, S7 by the call-log assertion
  of the three stats cases, S8 by the deserialization and the Prometheus and `json` cases. S9 survived, as recorded in
  advance (an equivalent mutant).
- **Dependency:** none new. **Clippy allows:** none.
- **Validation:** `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy -p
  torrust-tracker-axum-rest-api-server --all-targets` (stable), `cargo machete --with-metadata`, `git diff --check` and
  `cargo test -p torrust-tracker-axum-rest-api-server` (152 unit and 58 integration tests) pass.
- **Review outcome:** the maintainer approved the plan and the shared fixtures on 2026-11-04 and the completed file on 2026-11-04.
