---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/lib.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/lib.rs
    - packages/configuration/src/v3_0_0/udp_tracker_server.rs
    - src/app.rs
    - docs/issues/closed/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Crate Root File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/udp-core/src/lib.rs`
(ledger order 16).

## Current State

- **Unit-only coverage:** 5 / 5 lines (100.00%), 6 / 6 regions, 1 / 1 function. T1 measured 5 / 10;
  #2458 removed the static key initialization, which was the uncovered half.
- **Contents:** module declarations; the `CurrentClock` alias (working clock in production,
  stopped clock in tests); `UDP_TRACKER_LOG_TARGET`; the `ConnectionIdValidationPolicy` enum; and
  the test-only `sample_info_hash()` helper.
- **Existing tests:** none in this file. `sample_info_hash()` is used by other modules' tests.
- **Module-owned decisions:** `ConnectionIdValidationPolicy::default()` is `Strict`, documented as
  the secure default.
- **T1 hypothesis:** probably no change.

## Current Tests Review

No tests in this file. R1 is skipped.

## Coverage Analysis

| Item | Classification | Decision |
| --- | --- | --- |
| `ConnectionIdValidationPolicy` default | Not tested | Production never calls it: `src/app.rs` maps the configuration enum variant by variant, and every test names the variant it wants. The secure default that operators rely on is the configuration enum's, which `packages/configuration` already tests (`Strict` when the field is absent). A test here would pin a value nobody reads. |
| `Strict` / `Disabled` behavior | Owned elsewhere | Exercised in `services/announce.rs`, `services/scrape.rs` (done) and the `udp-server` handlers. |
| `CurrentClock`, log target, module declarations | Not tested | Compile-time wiring; no decision. |
| `sample_info_hash()` | Test support | Exercised by its callers. |
| Property-based testing | Not selected | No input-dependent decision. |

**Doc check:** the enum doc says it lives here "so that the service layer does not need to depend
on the configuration crate". Still accurate: the crate depends on `torrust-tracker-configuration`
for `container.rs`, but the service modules use it only in their tests.

**Finding (recorded in [lessons.md](../lessons.md); no production change in this issue):** the
`Default` derive on
`udp-core`'s `ConnectionIdValidationPolicy` has no caller. Removing it would leave one source of
truth for the secure default (the configuration enum).

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests to refactor. | Recorded above. |
| R2 | SKIPPED | No tests: no module decision that production reads. | Recorded above. |
| R3 | SKIPPED | No integration increment. | Explicit rationale. |
| R4 | DONE | Record coverage. | 5 / 5 lines, unchanged since #2458 (measured after `statistics/mod.rs`, `cargo-llvm-cov 0.6.16`). |

## Results

- **Unit-only coverage:** 5 / 5 (100.00%) lines; no change.
- **Tests added:** none.
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-14.

## Approval

Approved by Jose Celano on 2026-10-14: no change, and recording the finding in `lessons.md`.

## Progress Log

- 2026-10-14 11:06 UTC - GitHub Copilot - Created the plan from `lib.rs`, the app's policy mapping,
  the configuration enum and its tests, and the crate's uses of the configuration crate. No Rust
  tests or production code changed.
- 2026-10-14 11:27 UTC - Jose Celano - Approved the no-change decision and the finding; the file is
  done.
