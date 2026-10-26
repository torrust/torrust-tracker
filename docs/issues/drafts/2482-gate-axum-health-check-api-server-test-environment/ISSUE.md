---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2482
github-issue: null
spec-path: docs/issues/drafts/2482-gate-axum-health-check-api-server-test-environment/ISSUE.md
branch: "{issue-number}-2482-gate-axum-health-check-api-server-test-environment"
related-pr: null
last-updated-utc: "2026-10-26 19:50"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - packages/axum-health-check-api-server/src/environment.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Gate the `axum-health-check-api-server` test environment behind a `testing` feature

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 7 of its
Subissues table, the last one. Draft written by subissue 1 (#2618). Execution order: after rows 4
to 6 and after #1488's #2450 merges.

## Goal

Apply rule A of the
[ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md) to
`axum-health-check-api-server`, then check #2482's AC8 on the whole workspace.

## Background

`axum-health-check-api-server` exposes `pub mod environment;` (`src/lib.rs`), whose `environment.rs`
(118 lines) starts the health check API on a `HealthCheckApi` configuration and a `Registar`. It
forces one normal dependency that production code does not use:
`torrust-tracker-configuration`. Its only consumer is its own `tests/server/contract.rs`. Its tests
also use the HTTP, REST API and UDP environments, through `[dev-dependencies]` on
`torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server` and
`torrust-tracker-udp-server`; rows 4 to 6 enable `testing` on those entries.

## Gate

Issue #2450 (migrate the health check API test environment to the token lifecycle, part of #1488) is
open on 2026-10-21 and changes this environment. This subissue also starts after rows 4 to 6
merge, because its tests use their environments.

## Scope

### In Scope

- Audit every normal dependency of the package, workspace or third-party, against the ADR's rule
  A.2: optional behind `testing` if only the module and `#[cfg(test)]` code use it, a
  dev-dependency if only `#[cfg(test)]` code uses it.
- Gate the environment module with `#[cfg(feature = "testing")]`; add `testing` (not in
  `default`) with `dep:torrust-tracker-configuration`, and make that dependency optional (check
  first that no production source or intra-doc link needs it).
- Add the path-only self dev-dependency (no `version`) with `features = ["testing"]`.
- The feature comment (test support, outside semver); no `testing` in
  `[package.metadata.docs.rs]`.
- Update the inventory, the diagram and docs.
- Close-out for #2482: regenerate the workspace coupling report with the tool (a new dated file,
  not an edit of an existing one) and check AC8: no workspace library package's normal build
  contains test-only code or dependencies unless the ADR keeps them, and the report and the
  diagram show no test-only runtime edge. Record the result in the EPIC.

### Out of Scope

- Changing the environment's behaviour (#2450's task).
- The other environments. Any version bump (#2583).

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rule A; `axum-health-check-api-server` owns the health check environment).
- ADRs to create: None known.

## Design and Ownership Review

The package owns its environment; no other package uses it. Start, stop and drop-path behaviour are
unchanged by this move. After the first passing slice (the gate plus the package's own tests),
review the AC8 check before closing the EPIC.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. The package's tests are the regression suite; their count must not drop.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | BLOCKED | Record the baseline (after #2450 and rows 4 to 6) | `cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1`; test count with and without `--all-features` |
| T2 | TODO | Gate the module, add the feature, optional dependency and self dev-dependency | Plain `cargo test -p torrust-tracker-axum-health-check-api-server` passes |
| T3 | TODO | Update inventory, diagram and docs | Pattern A row resolved |
| T4 | TODO | Regenerate the coupling report and check AC8 | New dated report; the result recorded in the EPIC |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Gate, feature, optional dependency, self dev-dependency | Commit after the package's tests pass without `--all-features`. |
| T3 | Docs | One commit. |
| T4 | The regenerated report and the EPIC update | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/2482-gate-axum-health-check-api-server-test-environment/ISSUE.md`
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

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 7 of
  the #2482 Subissues table and rule A of the ADR; gated by #2450 and rows 4 to 6.
- 2026-10-22 18:10 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Changed after PR #2624's review: AC1 and M1 check direct dependencies (`--depth 1`) because `configuration` stays reachable through `axum-server`; added the dependency audit to the scope.

## Acceptance Criteria

- [ ] AC1: `cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1` shows no
      direct dependency on `torrust-tracker-configuration` unless production code uses it (it
      stays reachable through `axum-server`), and the environment is not
      compiled without the feature.
- [ ] AC2: `cargo test -p torrust-tracker-axum-health-check-api-server` passes without
      `--all-features`, with the baseline test count.
- [ ] AC3: The self dev-dependency has no `version`; `cargo +stable package --workspace
      --no-verify` passes.
- [ ] AC4: The regenerated coupling report and the diagram show no test-only runtime edge, and the
      result is recorded for #2482's AC8.
- [ ] AC5: The inventory has no pattern A, B, C or D item left unresolved, unless the ADR keeps it.
- [ ] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies. This is an O1 move: build the package without the feature, run `cargo test -p <package>`
without `--all-features`, and check the test count.

### Automatic Checks

- `linter all`, the package's tests with and without `--all-features`, `cargo +stable package
  --workspace --no-verify`, `cargo machete --with-metadata`, `cargo deny check bans`, pre-push
  checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The production build has no test-only dependency | `cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1` before and after | The forced direct dependency is gone (paths through production dependencies remain) | TODO | `manual-verification-evidence.md` V1 |
| M2 | Plain package tests | `cargo test -p torrust-tracker-axum-health-check-api-server` (no `--all-features`) | Test count equals the baseline | TODO | V2 |
| M3 | AC8 on the whole workspace | Run the coupling tool and read the report and diagrams | No test-only runtime edge | TODO | V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |

## Risks and Trade-offs

- #2450 changes the environment before this starts. Mitigation: T1 starts after it merges and
  re-reads the module.
- AC8 may find a leftover test-only edge in a package this EPIC did not touch. Mitigation: record
  it and open a follow-up subissue under #2482 before closing it.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory pattern A; EPIC #1488 subissue
  [#2450](../../closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md)
