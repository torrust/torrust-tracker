---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/torrent/responses.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/torrent/responses.rs
    - packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-torrent-handlers-tests.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-whitelist-responses-tests.md
---

# Torrent Responses File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/torrent/responses.rs`, ledger row 13 (rank 5), the second file of S4. It
repeats the no-change decision approved for the whitelist responses ([plan](v1-context-whitelist-responses-tests.md)).

## Current State

- **Unit-only coverage:** 0 / 9 lines at the S1 baseline and 6 / 9 at the S4 baseline (the two `Json` wrappers ran
  through the router tests). After the previous file (row 12) the file is 9 / 9 lines, 10 / 10 regions and 3 / 3
  functions (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace, `--lib`), measured on this branch before
  any change to this file.
- **Existing tests:** none in the file. Its three functions are reached, and their output asserted, by the handler
  tests of row 12, which compare each whole response (status, content type, body) with a literal.
- **Module-owned decisions:** the list and the single torrent are the JSON of the `ListItem` and `Torrent` resources
  (their shape is owned by `rest-api-protocol`), and an unknown torrent is the documented legacy `200` with the JSON
  string `"torrent not known"`.
- **T1 hypothesis:** test candidate. This plan challenges it: row 12 already made the file fully covered with literal
  expectations.

## Current Tests Review

No tests in the file. The tests that assert it live in `handlers.rs` and are reviewed in the
[row 12 plan](v1-context-torrent-handlers-tests.md).

## Coverage And Decision Analysis

| Function | Asserted today by | Classification | Decision |
| --- | --- | --- | --- |
| `torrent_list_response` | the page and the given-hashes tests: `200`, `application/json`, the literal list of two items | 3A unit, already covered | No change. |
| `torrent_info_response` | the known-torrent test: `200`, `application/json`, the literal torrent (no `peers` key when there are none) | 3A unit, already covered | No change. |
| `torrent_not_known_response` | the unknown-torrent test: `200`, `application/json`, `"torrent not known"` | 3A unit, already covered | No change. |

No production line, region or function of the file is uncovered, so there is no uncovered group to classify as hard
to test, owned elsewhere or merely unselected.

### Why direct tests would not add protection

Direct tests of the three functions would repeat the literals the handler tests already assert, through the same public
function, and would not show that a handler picks the right builder. The mutations below show that each answer is pinned
by an existing test.

## Named Mutations

Each is a temporary hand-made change to this file that a test must catch; applied alone and reverted exactly
(`cargo test -p torrust-tracker-axum-rest-api-server --all-features`, stable toolchain):

| ID | Site | Temporary change | Killed by |
| --- | --- | --- | --- |
| M1 | `torrent_not_known_response` | Text `torrent unknown`. | the unknown-torrent test |
| M2 | `torrent_list_response` | Keep only the first item. | the page cases that build a list |
| M3 | `torrent_info_response` | Add one to the seeders. | the known-torrent test |

## Property-Test Decision

Not flagged and not selected: the functions wrap a value in JSON.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | SKIPPED | No new unit tests: covered and pinned by the handler tests of row 12. | Mutations M1-M3 fail the handler tests (below). |
| R3 | DONE | Integration increment: `none`. | The 14 torrent integration tests keep protecting the wire contract. |
| R4 | DONE | Record before and after. | Results below and `coverage-evidence.md`. |

## Prose-First AAA Review

No test is added or changed, so there is nothing to review.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): 0 / 9 lines at the S1
  baseline and 6 / 9 at the S4 baseline; 9 / 9 lines, 10 / 10 regions and 3 / 3 functions with no change to this file
  (covered by the row 12 tests).
- **Mutations:** M1-M3 were applied alone and reverted exactly (the working tree was clean afterwards); each made the
  named handler test or tests fail on an assertion, not on a compile error.
- **Decision:** no change, recorded as documentation. The maintainer approved the no-change decision on 2026-11-04.
