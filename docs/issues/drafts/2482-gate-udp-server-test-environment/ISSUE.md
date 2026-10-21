---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2482
github-issue: null
spec-path: docs/issues/drafts/2482-gate-udp-server-test-environment/ISSUE.md
branch: "{issue-number}-2482-gate-udp-server-test-environment"
related-pr: null
last-updated-utc: "2026-10-21 19:16"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - packages/udp-server/src/testing/environment.rs
    - packages/udp-server/examples/udp_only_public_tracker.rs
    - packages/axum-health-check-api-server/Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Gate the `udp-server` test environment behind a `testing` feature

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 4 of its
Subissues table. Draft written by subissue 1 (#2618). Execution order: after rows 2, 8 and 3, and
after #1488's #2416 merges.

## Goal

Apply rule A of the
[ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md) to
`udp-server`: compile `src/testing/` only with a `testing` feature, make the dependency only that
module uses optional, and let the package's own targets and its cross-package consumer enable the
feature.

## Background

`udp-server` exposes `pub mod testing;` (`src/lib.rs`), whose `environment.rs` (581 lines) starts a
UDP tracker on an ephemeral address for tests, compiled in every build. It forces one normal
dependency that production code does not use: `torrust-tracker-swarm-coordination-registry`.
Consumers: its own `tests/server/contract.rs`; its own unit tests in `src/handlers/mod.rs` and
`src/server/processor.rs`; `examples/udp_only_public_tracker.rs`; and the
`axum-health-check-api-server` tests, which dev-depend on `udp-server`.

## Gate

Issue #2416 (stop event producers before event listeners, part of #2410) is open on 2026-10-21 and its
task T9 changes the HTTP and UDP environments. This subissue starts after it merges, or this spec
is updated with why the order does not matter. #1488 work is not delayed for this EPIC.

## Scope

### In Scope

- Gate `pub mod testing;` with `#[cfg(feature = "testing")]`; add `testing` (not in `default`)
  with `dep:torrust-tracker-swarm-coordination-registry`, and make that dependency optional.
- Add the path-only self dev-dependency (no `version`) with `features = ["testing"]`; the unit tests
  then need only the feature gate.
- Enable `features = ["testing"]` on `torrust-tracker-udp-server` in
  `axum-health-check-api-server`'s `[dev-dependencies]`, in the same pull request.
- Add the feature comment (test support, outside semver) and keep `testing` out of
  `[package.metadata.docs.rs]`.
- Decide with the maintainer what `examples/udp_only_public_tracker.rs` becomes: it starts a tracker
  through the test environment today. Either label it a test-support demonstration or give it a
  production-style startup; record the choice. The ADR leaves it open.
- Update the inventory (pattern A row for `udp-server`) and the diagram.

### Out of Scope

- Changing what the environment does or how tests use it (the behaviour changes of #2416 are its
  own).
- The other three environments (rows 5 to 7).
- Any version bump (#2583).

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rule A; ownership map: `udp-server` owns the UDP environment).
- ADRs to create: None known.

## Design and Ownership Review

`udp-server` owns the UDP environment; `axum-health-check-api-server` uses it only through a
dev-dependency feature. The environment's start, ready and stop sequence and its drop path are
unchanged by this move, and #2416 may change them before it starts. After the first passing slice
(the gate plus the package's own tests), review the cross-package consumer before touching the
example.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. The existing tests are the regression suite; their count must not drop.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | BLOCKED | Record the baseline (after #2416) | `cargo tree -p torrust-tracker-udp-server -e normal,features`; test counts of `cargo test -p torrust-tracker-udp-server` and `-p torrust-tracker-axum-health-check-api-server` with and without `--all-features` |
| T2 | TODO | Gate the module, add the feature and optional dependency, add the self dev-dependency | Plain `cargo test -p torrust-tracker-udp-server` passes |
| T3 | TODO | Enable the feature in the health check package's dev-dependency | Its tests pass |
| T4 | TODO | Decide and apply the example's status | Example builds; choice recorded |
| T5 | TODO | Update inventory, diagram and docs | Pattern A `udp-server` row resolved |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Gate, feature, optional dependency, self dev-dependency | Commit after the package's tests pass without `--all-features`. |
| T3 | Health check dev-dependency | Commit after its tests pass. |
| T4 | Example | One commit. |
| T5 | Docs | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/2482-gate-udp-server-test-environment/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created, linked as a sub-issue of #2482, and its number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 4 of
  the #2482 Subissues table and rule A of the ADR; gated by #2416.

## Acceptance Criteria

- [ ] AC1: `cargo tree -p torrust-tracker-udp-server -e normal,features` shows no
      `torrust-tracker-swarm-coordination-registry` unless production code uses it, and
      `src/testing/` is not compiled without the feature.
- [ ] AC2: `cargo test -p torrust-tracker-udp-server` and
      `-p torrust-tracker-axum-health-check-api-server` pass without `--all-features`, with the
      baseline test counts.
- [ ] AC3: The self dev-dependency has no `version`; `cargo +stable package --workspace
      --no-verify` passes.
- [ ] AC4: The example's status is decided and recorded; it builds.
- [ ] AC5: The inventory row is marked resolved.
- [ ] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies. This is an O1 move: build the package without the feature, run `cargo test -p <package>`
without `--all-features`, and check the test count.

### Automatic Checks

- `linter all`, the tests of both packages with and without `--all-features`,
  `cargo build -p torrust-tracker-udp-server --examples`, `cargo +stable package --workspace
  --no-verify`, `cargo machete --with-metadata`, `cargo deny check bans`, pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The production build has no test-only dependency | `cargo tree -p torrust-tracker-udp-server -e normal,features` before and after | The forced edge is gone | TODO | `manual-verification-evidence.md` V1 |
| M2 | Plain package tests | `cargo test -p torrust-tracker-udp-server` (no `--all-features`) | Test count equals the baseline | TODO | V2 |
| M3 | The consumer still works | `cargo test -p torrust-tracker-axum-health-check-api-server` | Passes with the baseline count | TODO | V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |

## Risks and Trade-offs

- #2416 may change the environment between this draft and the implementation. Mitigation: T1 starts
  only after it merges, and re-reads the module.
- Another package may start using the environment before this lands; T1 re-greps the consumers.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory pattern A; EPIC #1488 subissue #2416
