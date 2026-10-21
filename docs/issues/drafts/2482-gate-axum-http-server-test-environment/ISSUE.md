---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2482
github-issue: null
spec-path: docs/issues/drafts/2482-gate-axum-http-server-test-environment/ISSUE.md
branch: "{issue-number}-2482-gate-axum-http-server-test-environment"
related-pr: null
last-updated-utc: "2026-10-21 19:16"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - packages/axum-http-server/src/testing/environment.rs
    - packages/axum-http-server/examples/http_only_public_tracker.rs
    - packages/axum-health-check-api-server/Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Gate the `axum-http-server` test environment behind a `testing` feature

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 5 of its
Subissues table. Draft written by subissue 1 (#2618). Execution order: after row 4, and after #1488's #2416 merges.

## Goal

Apply rule A of the
[ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md) to
`axum-http-server`: compile `src/testing/` only with a `testing` feature, make the dependency only
that module uses optional, and let the package's own targets and its cross-package consumer enable
the feature.

## Background

`axum-http-server` exposes `pub mod testing;` (`src/lib.rs`), whose `environment.rs` (484 lines)
starts an HTTP tracker for tests, compiled in every build. It forces one normal dependency that
production code does not use: `torrust-tracker-swarm-coordination-registry`. Consumers: its own
`tests/` (nine files), `examples/http_only_public_tracker.rs`, and the
`axum-health-check-api-server` tests.

Its normal `torrust-tracker-configuration` dependency is not forced: the only use outside
`src/testing/` and `#[cfg(test)]` code is an intra-doc link at
`src/v1/extractors/client_ip_sources.rs:19`. If this subissue makes that dependency optional, the
link must be reworded first. Decide in T2; the default is to leave the dependency as it is.

## Gate

Issue #2471 (HTTP environment drop path) merged on 2026-10-16 and no longer gates this row. #2416 (task
T9 changes the HTTP and UDP environments) is open on 2026-10-21; this subissue starts after it
merges, or this spec records why the order does not matter.

## Scope

### In Scope

- Gate `pub mod testing;` with `#[cfg(feature = "testing")]`; add `testing` (not in `default`)
  with `dep:torrust-tracker-swarm-coordination-registry`, and make that dependency optional.
- Add the path-only self dev-dependency (no `version`) with `features = ["testing"]`.
- Enable `features = ["testing"]` on `torrust-tracker-axum-http-server` in
  `axum-health-check-api-server`'s `[dev-dependencies]`, in the same pull request.
- The feature comment (test support, outside semver); no `testing` in
  `[package.metadata.docs.rs]`.
- Decide with the maintainer what `examples/http_only_public_tracker.rs` becomes (test-support
  demonstration, or production-style startup); record it.
- Update the inventory and the diagram.

### Out of Scope

- Changing the environment's behaviour or its drop path (#2471 is done).
- The other environments (rows 4, 6, 7). Any version bump (#2583).

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rule A; `axum-http-server` owns the HTTP environment).
- ADRs to create: None known.

## Design and Ownership Review

`axum-http-server` owns the HTTP environment; the health check package uses it only through a
dev-dependency feature. Start, ready, stop and drop-path behaviour are unchanged by this move.
After the first passing slice (gate plus the package's own tests), review the consumer before the
example.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. The package's nine integration test files and its unit tests are the regression
suite; their count must not drop.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | BLOCKED | Record the baseline (after #2416) | `cargo tree -p torrust-tracker-axum-http-server -e normal,features`; test counts of the package and `-p torrust-tracker-axum-health-check-api-server` with and without `--all-features` |
| T2 | TODO | Gate the module, add the feature, optional dependency and self dev-dependency | Plain `cargo test -p torrust-tracker-axum-http-server` passes; decide the `configuration` edge |
| T3 | TODO | Enable the feature in the health check package's dev-dependency | Its tests pass |
| T4 | TODO | Decide and apply the example's status | Example builds; choice recorded |
| T5 | TODO | Update inventory, diagram and docs | Pattern A row resolved |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Gate, feature, optional dependency, self dev-dependency | Commit after the package's tests pass without `--all-features`. |
| T3 | Health check dev-dependency | Commit after its tests pass. |
| T4 | Example | One commit. |
| T5 | Docs | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/2482-gate-axum-http-server-test-environment/ISSUE.md`
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

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 5 of
  the #2482 Subissues table and rule A of the ADR; gated by #2416.

## Acceptance Criteria

- [ ] AC1: `cargo tree -p torrust-tracker-axum-http-server -e normal,features` shows no
      `torrust-tracker-swarm-coordination-registry`, and `src/testing/` is not compiled without
      the feature.
- [ ] AC2: `cargo test -p torrust-tracker-axum-http-server` and
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
  `cargo build -p torrust-tracker-axum-http-server --examples`, `cargo +stable package
  --workspace --no-verify`, `cargo machete --with-metadata`, `cargo deny check bans`, pre-push
  checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The production build has no test-only dependency | `cargo tree -p torrust-tracker-axum-http-server -e normal,features` before and after | The forced edge is gone | TODO | `manual-verification-evidence.md` V1 |
| M2 | Plain package tests | `cargo test -p torrust-tracker-axum-http-server` (no `--all-features`) | Test count equals the baseline | TODO | V2 |
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

- #2416 may change the environment before this starts. Mitigation: T1 starts after it merges and
  re-reads the module.
- Making the `configuration` edge optional breaks an intra-doc link; the default is to keep it.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory pattern A; EPIC #1488 subissue #2416
