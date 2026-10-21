---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/statistics/event/mod.rs
status: draft
semantic-links:
  related-artifacts:
    - packages/http-core/src/statistics/event/mod.rs
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Statistics Event Module File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/http-core/src/statistics/event/mod.rs`.

## Current State

- **Unit-only coverage:** no executable entries; the file is two `pub mod` lines (`handler`, `listener`).
- **Existing tests:** none; the declared modules are covered by their own plans (rows 6 and 7).
- **Module-owned decisions:** none beyond making the two modules public.
- **T1 hypothesis:** wiring only.

## Current Tests Review

Nothing to review.

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Module declarations | not tested | Wiring checked by the compiler; the entry points `handler::handle_event` and `listener::run_event_listener` are called by the application's bootstrap job (`src/bootstrap/jobs/http_tracker_core.rs`) and by the tests of rows 6 and 7 through these paths. |

No test, no documentation change, no integration increment. The conclusion is committed as documentation.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | N/A | No existing test. | Recorded in the plan. |
| R2 | N/A | No behavior to test. | Recorded in the plan. |
| R3 | TODO | Select an integration increment, or `none`. | `none`. |
| R4 | TODO | Re-run unit-only coverage; record that the file still has no executable entry. | `coverage-evidence.md`. |

Gates: this plan needs approval first; the completed file needs the maintainer's completed-file review before its
one signed commit.

## Progress Log

- 2026-10-21 13:23 UTC - GitHub Copilot - Created the plan from `statistics/event/mod.rs`. No Rust test or
  production code changed. The plan awaits maintainer approval.
