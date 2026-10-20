---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2415-2410-si-22-3-per-component-cancellation-tokens/ISSUE.md
last-updated-utc: "2026-10-20 20:59"
---

# Manual Verification Evidence

## Purpose

Record the verification of #2415 (SI-22.3): the tests of the reservation API that gives each
`JobManager` component its own cancellation token, the mutations that show those tests guard the
lifecycle rules, and the like-for-like shutdown comparison with `develop` (M1). The refactor is
meant to change no behavior, so there is no bug red/green pair.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-20 from 20:40.
- Artifact under test: branch `2415-per-component-cancellation-tokens`, cut from `develop` at the
  merge `Merge torrust/torrust-tracker#2587: chore(issues): archive closed issue #2414 spec to docs/issues/closed`.
  Each process below names the working-tree state it ran on.
- Operating system / environment: Linux; default toolchain `rustc 1.99.0 (b940084d7 2026-09-28)`,
  `cargo 1.99.0 (5f94df478 2026-08-27)`; nightly toolchain `rustc 1.101.0-nightly (7df81381d 2026-10-14)`
  for the doctest error-code check. `cargo test` built the `test` profile, which this workspace
  configures as optimized with debug info.
- Prerequisites and setup performed: none beyond a workspace build.

## Verification Processes

### V1 - T6a: reservation API unit tests, doctests, and mutations

- Goal: show that `JobManager::reserve` and the reservation-based `spawn` keep every stop request
  (cancellation before `reserve`, between `reserve` and `spawn`, repeated `cancel()`), that a
  component token is not derived from the legacy root token, that a dropped unspawned reservation
  is released without affecting other components, and that a reservation cannot be spawned by
  another manager or spawned twice.
- Working-tree state: the T6a change in `src/bootstrap/jobs/manager.rs`, uncommitted while these
  runs happened and then committed as `refactor(bootstrap): [#2415] add JobManager reservations that issue each component its own token`, with the
  temporary `spawn_unreserved` adapter (the previous `spawn(name, future)`) still used by
  `src/app.rs`, `src/console/profiling.rs`, and the existing `manager.rs` tests.
- Commands and outcomes:
  - `cargo test -q -p torrust-tracker --lib bootstrap::jobs::manager` (default toolchain): 17 passed,
    the 10 existing tests and 7 new ones.
  - `cargo test -p torrust-tracker --doc Reservation` (default toolchain) and
    `cargo +nightly test -p torrust-tracker --doc Reservation` (nightly toolchain): both pass the
    `compile_fail,E0382` doctest (spawning one reservation twice) and its `no_run` companion
    (reserving again for a second component). The first run failed the companion: the mechanical
    rename of `manager.spawn(` callers to the adapter had also rewritten the doc examples, and the
    `compile_fail` doctest still passed because it failed to compile for that other reason. The
    companion caught it; the doc examples were restored to `spawn`.
  - `linter clippy`: exit 0, after making the consumption of the reservation in `spawn` explicit
    (`clippy::needless_pass_by_value`) and replacing an `assert!(... .is_empty())` with `assert_eq!`
    (`clippy::assert_is_empty`).
- Mutations: each was applied to the working tree alone, the manager tests were run with the
  command above, and the file was restored from a copy (it was not committed yet). Every mutation
  compiled before its result was read.

  | Mutation | Failing tests |
  | --- | --- |
  | M1: `cancel()` no longer cancels component tokens (first form removed the `cancel_all` call and did not compile, dead code under `-D warnings`; rerun with the loop no longer calling `cancel`) | `it_should_cancel_a_spawned_components_own_token_when_the_manager_is_cancelled`, `it_should_cancel_a_reserved_component_that_is_spawned_after_the_manager_is_cancelled`, `it_should_stop_components_and_legacy_jobs_once_when_the_manager_is_cancelled_twice`, `it_should_release_a_dropped_reservation_without_affecting_other_components` |
  | M2: `reserve` ignores an earlier `cancel()` | `it_should_issue_a_cancelled_token_when_a_component_is_reserved_after_the_manager_is_cancelled` |
  | M3: the component token is a child of the legacy root token | `it_should_not_cancel_a_reserved_component_when_only_the_legacy_root_token_is_cancelled`, `it_should_reject_a_reservation_issued_by_another_manager`, `it_should_release_a_dropped_reservation_without_affecting_other_components` |
  | M4: a dropped reservation is not released | `it_should_release_a_dropped_reservation_without_affecting_other_components`, `it_should_reject_a_reservation_issued_by_another_manager` |
  | M5: a reservation from another manager is accepted | `it_should_reject_a_reservation_issued_by_another_manager` |
  | M6: `cancel()` does not record that the manager is cancelled | `it_should_issue_a_cancelled_token_when_a_component_is_reserved_after_the_manager_is_cancelled` |

- Full suite of the root package on the same tree:
  `cargo test -p torrust-tracker --tests --benches --examples --all-targets --all-features` (default
  toolchain): exit 0, 14 test targets, including `lifecycle-signals` (13 passed) and the library (123
  passed). The change is confined to the root package; no other workspace package depends on it.
- Prose-first Arrange-Act-Assert: each new test had temporary prose paragraphs above it. Comparing
  them with the code led to two changes: the release test observes release through behavior (the
  dropped reservation's token stays uncancelled after `cancel()`) instead of reading the registry,
  and every assertion message states the rule it checks. The prose was then redundant with the test
  names and assertion messages and was removed before the commit. One trade-off is kept: the
  foreign-reservation test also cancels the issuing manager in its Act, because that is the only way
  to observe that the rejected reservation was released.
- Result: pass. Every mutation is caught by at least one new test.
