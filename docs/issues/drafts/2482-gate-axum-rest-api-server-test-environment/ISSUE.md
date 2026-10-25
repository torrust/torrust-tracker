---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2482
github-issue: null
spec-path: docs/issues/drafts/2482-gate-axum-rest-api-server-test-environment/ISSUE.md
branch: "{issue-number}-2482-gate-axum-rest-api-server-test-environment"
related-pr: null
last-updated-utc: "2026-10-25 08:09"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/closed/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md
    - packages/axum-rest-api-server/src/testing/environment.rs
    - packages/axum-rest-api-server/src/testing/mod.rs
    - packages/axum-health-check-api-server/Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Gate the `axum-rest-api-server` test environment behind a `testing` feature

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 6 of its
Subissues table. Draft written by subissue 1 (#2618). Execution order: after row 5, and after #1488's #2449 merges.

## Goal

Apply rule A of the
[ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md) to
`axum-rest-api-server`: compile `src/testing/` only with a `testing` feature and make the six
dependencies only that module uses optional, so the package no longer carries UDP, HTTP-core and
tracker-core edges that its production code does not use.

## Background

`axum-rest-api-server` exposes `pub mod testing;` (`src/lib.rs`), whose `environment.rs` (232
lines) starts the REST API on the full `Configuration`, wiring the HTTP and UDP cores. It forces
six normal dependencies that production code does not use: `torrust-tracker-core`,
`torrust-tracker-http-core`, `torrust-tracker-udp-core`, `torrust-tracker-udp-server`,
`torrust-tracker-swarm-coordination-registry` and `torrust-tracker-rest-api-client` (the last is
already a dev-dependency too). Consumers: its own `tests/` (six files) and the
`axum-health-check-api-server` tests. `src/testing/mod.rs` still says the UDP dependencies are
needed at runtime until "the prerequisite decoupling in `rest-api-core`", a crate that no longer
exists (#1938); this subissue removes that comment. Production code reaches UDP only through
`rest-api-runtime-adapter`.

## Gate

Issue #2449 (migrate the REST API test environment to the token lifecycle, part of #1488) is open on
2026-10-21 and changes this environment. This subissue starts after it merges, or this spec records
why the order does not matter.

## Scope

### In Scope

- Audit every normal dependency of the package, workspace or third-party, against the ADR's rule
  A.2: optional behind `testing` if only the module and `#[cfg(test)]` code use it, a
  dev-dependency if only `#[cfg(test)]` code uses it.
- Gate `pub mod testing;` with `#[cfg(feature = "testing")]`; add `testing` (not in `default`)
  enabling the six dependencies with `dep:` syntax, and make them optional. Re-verify before
  editing that production source does not use them (the inventory's check was by `rg`).
- Add the path-only self dev-dependency (no `version`) with `features = ["testing"]`.
- Enable `features = ["testing"]` on `torrust-tracker-axum-rest-api-server` in
  `axum-health-check-api-server`'s `[dev-dependencies]`, in the same pull request.
- Remove the stale `rest-api-core` comment; add the feature comment (test support, outside
  semver); no `testing` in `[package.metadata.docs.rs]`.
- Update the inventory and the diagram (including the REST API layer diagram if it shows these
  edges).

### Out of Scope

- Changing the environment's behaviour (#2449's task).
- The other environments (rows 4, 5, 7). Any version bump (#2583).

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rule A; `axum-rest-api-server` owns the REST API environment).
- ADRs to create: None known.

## Design and Ownership Review

`axum-rest-api-server` owns the REST API environment; the health check package uses it only through
a dev-dependency feature. The environment starts the REST API and, through the cores, UDP and HTTP
services; its start, stop and drop-path behaviour is unchanged by this move. After the first passing
slice (the gate plus the package's own tests), review the consumer.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. The package's six integration test files and unit tests are the regression suite;
their count must not drop.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | BLOCKED | Record the baseline (after #2449) | `cargo tree -p torrust-tracker-axum-rest-api-server -e normal --depth 1`; test counts of the package and `-p torrust-tracker-axum-health-check-api-server` with and without `--all-features` |
| T2 | TODO | Gate the module, add the feature, make the six dependencies optional, add the self dev-dependency | Plain `cargo test -p torrust-tracker-axum-rest-api-server` passes |
| T3 | TODO | Enable the feature in the health check package's dev-dependency | Its tests pass |
| T4 | TODO | Remove the stale comment; update inventory, diagram and docs | Pattern A row resolved |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Gate, feature, optional dependencies, self dev-dependency | Commit after the package's tests pass without `--all-features`. |
| T3 | Health check dev-dependency | Commit after its tests pass. |
| T4 | Comment removal and docs | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/2482-gate-axum-rest-api-server-test-environment/ISSUE.md`
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

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 6 of
  the #2482 Subissues table and rule A of the ADR; gated by #2449.
- 2026-10-22 18:10 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Changed after PR #2624's review: AC1 and M1 check direct dependencies (`--depth 1`) because five of the six crates stay reachable through `rest-api-runtime-adapter`; added the dependency audit to the scope.

## Acceptance Criteria

- [ ] AC1: `cargo tree -p torrust-tracker-axum-rest-api-server -e normal --depth 1` shows none of
      the six forced crates as a direct dependency unless production code uses them (five stay
      reachable through `rest-api-runtime-adapter`), and `src/testing/` is not
      compiled without the feature.
- [ ] AC2: `cargo test -p torrust-tracker-axum-rest-api-server` and
      `-p torrust-tracker-axum-health-check-api-server` pass without `--all-features`, with the
      baseline test counts.
- [ ] AC3: The self dev-dependency has no `version`; `cargo +stable package --workspace
      --no-verify` passes.
- [ ] AC4: The stale `rest-api-core` comment is gone.
- [ ] AC5: The inventory row is marked resolved.
- [ ] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies. This is an O1 move: build the package without the feature, run `cargo test -p <package>`
without `--all-features`, and check the test count.

### Automatic Checks

- `linter all`, the tests of both packages with and without `--all-features`, `cargo +stable
  package --workspace --no-verify`, `cargo machete --with-metadata`, `cargo deny check bans`,
  pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The production build has no test-only dependency | `cargo tree -p torrust-tracker-axum-rest-api-server -e normal --depth 1` before and after | The six forced direct dependencies are gone (paths through production dependencies remain) | TODO | `manual-verification-evidence.md` V1 |
| M2 | Plain package tests | `cargo test -p torrust-tracker-axum-rest-api-server` (no `--all-features`) | Test count equals the baseline | TODO | V2 |
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

- #2449 changes the environment before this starts. Mitigation: T1 starts after it merges and
  re-reads the module.
- A dependency may turn out to be used by production code after all; T2 re-verifies each.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory pattern A; EPIC #1488 subissue
  [#2449](../../closed/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md)
