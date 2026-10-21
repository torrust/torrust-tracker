---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/services/mod.rs
status: draft
semantic-links:
  related-artifacts:
    - packages/http-core/src/services/mod.rs
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Services Module File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/http-core/src/services/mod.rs`.

## Current State

- **Unit-only coverage:** no executable entries (the report lists no line for the file).
- **Existing tests:** none; the three modules it declares are covered by their own plans.
- **Module-owned decisions:** which services are public (`announce`, `scrape`) and which is crate-private
  (`error_mapping`), and the module documentation.
- **T1 hypothesis:** wiring only.

## Current Tests Review

Nothing to review. A test of the declarations would only restate the compiler's checks; a test of the visibility of
`error_mapping` would need a compile-fail harness the package does not have, and the visibility is exercised by the
announce and scrape services that use the module.

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Module declarations and visibility | not tested | Wiring checked by the compiler; no executable entry to cover. |
| Module documentation | doc correction | The text says the services "does depend on the Axum web server" and then that they "could be reused for other web servers". The sentence contradicts itself and the package layering (`http-core` has no Axum dependency, see `Cargo.toml`). **D1 (maintainer decision):** correct it to "does not depend", a comment-only change that the specification allows for a file under test. |

No property candidate, no integration increment.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | N/A | No existing test. | Recorded in the plan. |
| R2 | TODO | Correct the documentation sentence (D1). No test. | The diff is the comment line only; `cargo doc` and the doc tests pass. |
| R3 | TODO | Select an integration increment, or `none`. | `none`. |
| R4 | TODO | Re-run unit-only coverage; record that the file still has no executable entry. | `coverage-evidence.md`. |

Gates: this plan needs approval first; the completed file needs the maintainer's completed-file review before its
one signed commit.

## Progress Log

- 2026-10-21 13:23 UTC - GitHub Copilot - Created the plan from `services/mod.rs`. No Rust test or production code
  changed. The plan awaits maintainer approval.
