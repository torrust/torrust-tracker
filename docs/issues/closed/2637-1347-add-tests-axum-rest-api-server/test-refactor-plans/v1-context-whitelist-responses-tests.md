---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/whitelist/responses.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/whitelist/responses.rs
    - packages/axum-rest-api-server/src/v1/context/whitelist/handlers.rs
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-whitelist-handlers-tests.md
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-auth-key-responses-tests.md
---

# Whitelist Responses File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/whitelist/responses.rs`, ledger row 10 (rank 4), the fifth file of S3. It
repeats the no-change decision the maintainer approved for the auth-key responses
([plan](v1-context-auth-key-responses-tests.md)).

## Current State

- **Unit-only coverage:** 0 / 9 lines at the S1 baseline. After the previous file (row 9) the file is 9 / 9 lines,
  12 / 12 regions and 3 / 3 functions (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace, `--lib`),
  measured on this branch before any change to this file.
- **Existing tests:** none in the file. Its three functions are reached, and their output asserted, by the handler
  tests of row 9, which compare each whole response (status, content type, body) with a literal.
- **Module-owned decisions:** the three `500` messages `failed to remove torrent from whitelist: <error>`,
  `failed to whitelist torrent: <error>` and `failed to reload whitelist: <error>`, in the legacy
  `Unhandled rejection` text.
- **T1 hypothesis:** test candidate. This plan challenges it: row 9 already made the file fully covered with literal
  expectations.

## Current Tests Review

No tests in the file. The tests that assert it live in `handlers.rs` and are reviewed in the
[row 9 plan](v1-context-whitelist-handlers-tests.md).

## Coverage And Decision Analysis

| Function | Asserted today by | Classification | Decision |
| --- | --- | --- | --- |
| `failed_to_remove_torrent_from_whitelist_response` | the `remove` failure test: `500`, `failed to remove torrent from whitelist: boom` | 3A unit, already covered | No change. |
| `failed_to_whitelist_torrent_response` | the two `add` failure cases: `failed to whitelist torrent: boom` and the documented disabled-capability case | 3A unit, already covered | No change. |
| `failed_to_reload_whitelist_response` | the `reload` failure test: `failed to reload whitelist: boom` | 3A unit, already covered | No change. |

No production line, region or function of the file is uncovered, so there is no uncovered group to classify as hard
to test, owned elsewhere or merely unselected.

### Why direct tests would not add protection

Direct tests of the three functions would repeat the literals the handler tests already assert, through the same
public function, and would not show that a handler picks the right builder. The mutations below show that each
message is pinned by an existing test.

## Named Mutations

Each is a temporary hand-made change to this file that a test must catch; applied alone and reverted exactly
(`cargo test -p torrust-tracker-axum-rest-api-server --lib`, stable toolchain):

| ID | Site | Temporary change | Killed by |
| --- | --- | --- | --- |
| M1 | `failed_to_remove_torrent_from_whitelist_response` | Message `failed to remove torrent: ...`. | the `remove` failure test |
| M2 | `failed_to_whitelist_torrent_response` | Message `failed to whitelist: ...`. | both `add` failure cases |
| M3 | `failed_to_reload_whitelist_response` | Message `failed to reload: ...`. | the `reload` failure test |

## Property-Test Decision

Not flagged and not selected: the functions format a fixed shape.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | SKIPPED | No new unit tests: covered and pinned by the handler tests of row 9. | Mutations M1-M3 fail the handler tests (below). |
| R3 | DONE | Integration increment: `none`. | The 13 whitelist integration tests keep protecting the wire contract. |
| R4 | DONE | Record before and after. | Results below and `coverage-evidence.md`. |

## Prose-First AAA Review

No test is added or changed, so there is nothing to review.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): 0 / 9 lines at the S1
  baseline; 9 / 9 lines, 12 / 12 regions and 3 / 3 functions with no change to this file (covered by the row 9 tests).
- **Mutations:** M1-M3 were applied alone and reverted exactly (the working tree was clean afterwards); each made the
  named handler test or tests fail on an assertion, not on a compile error.
- **Decision:** no change, recorded as documentation. The maintainer approved the no-change decision on 2026-11-03.
