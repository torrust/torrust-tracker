---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 2482
github-issue: 2757
spec-path: docs/issues/open/2757-2482-gate-axum-health-check-api-server-test-environment/ISSUE.md
branch: "2757-2482-gate-axum-health-check-api-server-test-environment-spec"
related-pr: 2759
last-updated-utc: "2026-11-09 15:53"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md
    - docs/issues/closed/2745-2482-gate-axum-rest-api-server-test-environment/ISSUE.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - packages/axum-health-check-api-server/src/environment.rs
---

<!-- skill-link: create-issue -->

# Issue #2757 - Gate the `axum-health-check-api-server` test environment behind a `testing` feature

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 7 of its
Subissues table, the last one. Draft written by subissue 1 (#2618), refreshed on 2026-11-06
against `torrust/develop`. Execution order: after rows 4 to 6 (done).

## Goal

Apply rule A of the
[ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md) to
`axum-health-check-api-server`: compile `src/environment.rs` only with a `testing` feature and make
the one dependency only that module and the tests use optional. Then check #2482's AC8 on the whole
workspace with a regenerated coupling report.

## Background

`axum-health-check-api-server` exposes `pub mod environment;` (`src/lib.rs`), whose `environment.rs`
(497 lines on `develop` on 2026-11-06; the draft said 118, because #2450 grew it with the token
lifecycle and its own `#[cfg(test)]` unit tests) starts the health check API on a `HealthCheckApi`
configuration and a `Registar`, compiled in every build. Consumers: its own `tests/` (a single
`integration.rs` target with `server/` modules; `tests/server/contract.rs` imports
`environment::Started`) and its own `environment.rs` unit tests. Nothing else in the workspace
imports the module (the root crate uses the package for the real server only). The package has no
`examples/` and no benches. Its tests also use the HTTP, REST API and UDP environments through
`[dev-dependencies]` entries that already enable `features = ["testing"]` (rows 4 to 6); those stay.

Dependency audit (2026-11-06, `cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1`
and a per-crate source grep that cuts each file at its `#[cfg(test)]` module):

| Direct normal dependency | Used by production code? | Result |
| --- | --- | --- |
| `torrust-tracker-configuration` | No: `src/environment.rs` (public signatures: `HealthCheckApi`, `Shutdown`), the `src/server.rs` unit tests (line 278, after `#[cfg(test)]` at 269) and `tests/server/contract.rs` only | Optional behind `testing` |
| `torrust-tracker-primitives`, `torrust-tracker-axum-server`, `torrust-server-lib`, `torrust-net-primitives`, `tokio-util` | Yes: `src/server.rs` or `src/handlers.rs` | Unchanged |
| every other direct dependency (16 in total) | Yes | Unchanged |

`torrust-tracker-configuration` stays reachable through `axum-server`; the change removes the direct
edge only. The baseline direct dependency count is 17 (`cargo tree --depth 1`, measured on
`torrust/develop` at `35ec81000`); it becomes 16.

## Gate

The gate is satisfied. The draft was gated on #1488's #2450 (migrate the health check API test
environment to the token lifecycle), which is closed, and on rows 4 to 6, which are merged. The
row 4 (#2699), row 5 (#2724) and row 6 (#2745) precedents apply, including the container build fix:
the pin `torrust-cargo-chef` 0.1.79 is already in the `Containerfile`.

## Scope

### In Scope

- Audit every normal dependency of the package against the ADR's rule A.2 (done in Background; T1
  re-checks it).
- Gate `pub mod environment;` with `#[cfg(feature = "testing")]`; add `testing` (not in `default`)
  with `dep:torrust-tracker-configuration`, and make that dependency optional.
- Add the path-only self dev-dependency (no `version`) with `features = ["testing"]`. The
  `deny.toml` `wrappers` entry that lists the package itself already existed at `ca8efcd4d`; this
  change only adds the comment explaining it.
- The feature comment (test support, outside semver); no `testing` in `[package.metadata.docs.rs]`.
- Run the container recipe check (`cargo chef prepare --external-only`, then `cargo chef cook`) and
  the Shipped Features check before the first push.
- Update the inventory (pattern A row for the health check package), the diagram and docs; the
  inventory and EPIC rows refer to a 118-line environment, which is stale.
- Close-out check for #2482 (the check only): regenerate the workspace coupling report with
  `contrib/dev-tools/analysis/workspace-coupling` (a new dated file, not an edit of an existing
  one) and check AC8: no workspace library package's normal build contains test-only code or
  dependencies unless the ADR keeps them, and the report and the diagram show no test-only runtime
  edge. Record the result in the #2482 EPIC's AC8 evidence note, without ticking the criterion (the
  coordinator proposes the close-out steps separately).

### Out of Scope

- Changing the environment's behaviour (#2450 is done).
- The other environments. Any version bump (#2583).
- A runtime-edge counter in the coupling tool (maintainer decision, below).
- The #2482 close-out steps: ticking its acceptance criteria, replacing `--all-features` in the
  test archives, archiving the #2482 folder, setting the #1669 EPIC row to DONE, and removing the
  `Blocked` label from #2576. A coordinator proposes them after row 7.

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rule A; `axum-health-check-api-server` owns the health check environment).
- ADRs to create: None known.

### Decision: runtime-edge counter

Decided by the maintainer on 2026-11-06 in this session: do not add a runtime-edge counter to the
coupling tool. AC8 is checked by reading the regenerated report and diagram, in which optional
edges are labelled `optional (testing)`. Rejected: adding the counter in row 7 (widens the last row
into tool work; the tool has no CI gate that would use the number).

### Decision: `torrust-tracker-configuration`

Decided by the maintainer on 2026-11-06: make it optional behind `testing`; the self
dev-dependency enables it for every test target, so no separate dev-dependency entry is added.
Rejected: also listing it as a dev-dependency (redundant entry, departs from rows 4 to 6).

### Decision: issue labels and sub-issue link

Decided by the maintainer on 2026-11-06: the issue is created as `josecelano-bot`; the bot account
cannot add labels or link sub-issues on `torrust/torrust-tracker`, so those two commands run with the
`josecelano` account, shown first and only on his explicit OK in this session, as in rows 5 and 6.
In this row the maintainer ran both commands himself (recorded in the progress log).

## Design and Ownership Review

The package owns its environment and only its own tests use it. Start, stop and drop-path behaviour
are unchanged by this move: the change is a `cfg` gate and manifest edits, with no new asynchronous
path. The environment's own `#[cfg(test)]` unit tests keep running because the self dev-dependency
turns the feature on for the library's test build. After the first passing slice (the gate plus the
package's own tests), review the AC8 check before reporting.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. The package's tests are the regression suite; their count must not drop.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Record the baseline | `cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1` (17 direct dependencies); plain `cargo test -p torrust-tracker-axum-health-check-api-server` on `torrust/develop` at `35ec81000` (2026-11-06): 15 unit + 11 integration + 0 doc. T2 remeasures with and without `--all-features` before comparing |
| T2 | DONE | Gate the module, add the feature, make `configuration` optional, add the self dev-dependency (the `deny.toml` self wrapper already exists; add its comment) | Plain `cargo test -p torrust-tracker-axum-health-check-api-server` passes with the baseline counts; `cargo check -p torrust-tracker-axum-health-check-api-server --lib` passes with no features; `Cargo.lock` and `deny.toml` in the same commit |
| T3 | DONE | Update inventory, diagram and docs | Pattern A row resolved; stale 118-line figure corrected; EPIC row |
| T4 | DONE | Verify the container recipe and the shipped features | `cargo chef prepare --external-only` then `cargo chef cook --tests --workspace --all-features` exit 0 in a scratch directory; `cargo run -q --package shipped-features-check -- --package torrust-tracker` exits 0 |
| T5 | DONE | Regenerate the coupling report and check AC8 | New dated report under `docs/issues/open/1669-overhaul-packages/`; the result recorded in the #2482 EPIC as evidence, without ticking AC8 |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Gate, feature, optional dependency, self dev-dependency, `deny.toml` comment | Commit after the package's tests pass without `--all-features`. |
| T3 | Docs | One commit. |
| T4 | Evidence | One commit with the spec update. |
| T5 | The regenerated report and the EPIC note | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2757-2482-gate-axum-health-check-api-server-test-environment/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer (2026-11-06)
- [x] GitHub issue created (#2757), linked as a sub-issue of #2482, and its number added to this spec
- [x] Spec-only PR merged into `develop` before implementation (PR #2759)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 7 of
  the #2482 Subissues table and rule A of the ADR; gated by #2450 and rows 4 to 6.
- 2026-10-22 18:10 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Changed after PR #2624's review: AC1 and M1 check direct dependencies (`--depth 1`) because `configuration` stays reachable through `axum-server`; added the dependency audit to the scope.
- 2026-11-06 16:34 UTC - AI assistant (Claude Code, #2482 row 7) - Refreshed the draft against `torrust/develop` at `35ec81000`: #2450 and rows 4 to 6 are done, so the gate is satisfied; `environment.rs` is 497 lines (not 118); `configuration` is the only dependency to make optional; baseline 17 direct dependencies, 15 + 11 tests. Recorded the maintainer's decisions: no runtime-edge counter, `configuration` optional behind `testing`, labels and the sub-issue link with the `josecelano` account on his explicit OK. Out of scope: the #2482 close-out steps (a coordinator proposes them).
- 2026-11-06 16:39 UTC - AI assistant (Claude Code, #2482 row 7) - The maintainer approved the spec. Created issue #2757 as `josecelano-bot`; the maintainer added the labels (`task`, `packages`) himself. The sub-issue link under #2482 is pending his action. The spec moved to `docs/issues/open/`; the #2482 EPIC row, the inventory and the 2026-10-06 coupling report link to it.
- 2026-11-06 17:12 UTC - AI assistant (Claude Code, #2482 row 7) - The maintainer added the labels (`task`, `packages`) and the sub-issue link under #2482 with his own account (verified: `parent_issue_url` is #2482). Opened spec-only PR #2759 (`Related to #2757`); `related-pr` is set to 2759. The 16:40 entry above was dated 16:34, the time the draft was committed; the earlier 16:40 was a mistake found in PR #2759 review round 1.
- 2026-11-09 15:53 UTC - AI assistant (Claude Code, #2482 row 7) - Implemented T2 to T5 on branch `2757-2482-gate-axum-health-check-api-server-test-environment`: the `testing` feature, `configuration` optional, the path-only self dev-dependency with its `deny.toml` wrapper, and `url` now declared with `features = ["serde"]` (the move exposed that the package only got it through `configuration`). Plain package tests keep the baseline (15 + 11); the direct dependency count is 17 before and 16 after. The coupling report of 2026-11-09 was regenerated and AC8 was checked: no test-only runtime edge remains once five stale diagram edges are removed (four from rows 4 and 5, one from this row). The evidence is in `manual-verification-evidence.md`; the AC8 result is recorded in the EPIC without ticking it. The #2482 close-out steps are not part of this row.
- 2026-11-09 16:38 UTC - AI assistant (Claude Code, #2482 row 7) - Corrections after da2ce7's review of PR #2784: (1) the 15:53 entry says `url` got `serde` only through `configuration` and lost it with the edge; that is wrong. Measured at `ca8efcd4d` (the merge base, with the `configuration` edge still present) `cargo check -p torrust-tracker-axum-health-check-api-server --lib` already failed with E0277, because `url` lacked `serde` there; only crates reached through `[dev-dependencies]` (`udp-server`, `rest-api-client`) request it, so `cargo test -p` passed. The declaration is correct on its own; the defect pre-existed and T1's baseline ran only `cargo test`. (2) The `deny.toml` self wrapper entry already existed at `ca8efcd4d`; this PR adds only a comment. (3) Six `test-helpers` arrows that no manifest declares are removed from the diagram (a script compares every arrow with `cargo metadata`), the edge count is restated as 135, the report gets frontmatter, the module gets its rule A.6 note, and the evidence commands live in `evidence/`.

## Acceptance Criteria

- [x] AC1: `cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1` shows no
      direct dependency on `torrust-tracker-configuration` (it stays reachable through
      `axum-server`), and `src/environment.rs` is not compiled without the feature.
- [x] AC2: `cargo test -p torrust-tracker-axum-health-check-api-server` passes without
      `--all-features`, with the baseline test counts (15 unit + 11 integration).
- [x] AC3: The self dev-dependency has no `version`; `cargo +stable package --workspace
      --no-verify` passes; `cargo deny check bans` passes.
- [x] AC4: The container recipe cooks (`prepare --external-only` then `cook`) and the Shipped
      Features check exits 0; no `testing` feature is in a `[dependencies]` entry or a `default`
      chain.
- [x] AC5: The inventory has no pattern A, B, C or D item left unresolved, unless the ADR keeps it.
- [x] AC6: A coupling report regenerated on the merged state of rows 1 to 7 and the diagram show no
      test-only runtime edge, and the AC8 result is recorded in the #2482 EPIC as evidence (the
      criterion itself is ticked by the close-out, not here).
- [x] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies. This is an O1 move: build the package without the feature, run `cargo test -p <package>`
without `--all-features`, and check the test count.

### Automatic Checks

- `linter all`, the package's tests with and without `--all-features`, `cargo +stable package
  --workspace --no-verify`, `cargo machete --with-metadata`, `cargo deny check bans`, pre-push
  checks; `cargo check -p torrust-tracker-axum-health-check-api-server --lib` with no features.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The production build has no test-only dependency | `cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1` before and after | The forced direct `configuration` edge is gone (the path through `axum-server` remains) | DONE | `manual-verification-evidence.md` V1 |
| M2 | Plain package tests | `cargo test -p torrust-tracker-axum-health-check-api-server` (no `--all-features`) | Test counts equal the baseline | DONE | V2 |
| M3 | Shipped features | `cargo run -q --package shipped-features-check -- --package torrust-tracker` | Exit 0 | DONE | V3 |
| M4 | Deny, machete, packaging, nightly check | The commands of the Automatic Checks | All exit 0; `deny` fails without the self wrapper | DONE | V4 |
| M5 | Container recipe | `cargo chef prepare --external-only`, then `cargo chef cook --tests --workspace --all-features` in a scratch directory | Exit 0 | DONE | V5 |
| M6 | AC8 on the whole workspace | Run the coupling tool and read the report and diagram | No test-only runtime edge | DONE | V6 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | [`manual-verification-evidence.md`](manual-verification-evidence.md) |
| AC2   | DONE                   | [`manual-verification-evidence.md`](manual-verification-evidence.md) |
| AC3   | DONE                   | [`manual-verification-evidence.md`](manual-verification-evidence.md) |
| AC4   | DONE                   | [`manual-verification-evidence.md`](manual-verification-evidence.md) |
| AC5   | DONE                   | [`manual-verification-evidence.md`](manual-verification-evidence.md) |
| AC6   | DONE                   | [`manual-verification-evidence.md`](manual-verification-evidence.md) |

## Risks and Trade-offs

- The environment's unit tests are inside the gated module: if the self dev-dependency were missing
  they would vanish silently. Mitigation: compare test counts before and after (15 unit tests).
- AC8 may find a leftover test-only edge in a package this EPIC did not touch. Mitigation: record
  it and propose a follow-up subissue under #2482; the maintainer decides how #2482 closes.

## Implementation Completion Review

- Retrospective: [`implementation-retrospective.md`](implementation-retrospective.md): the no-features build was already broken before the move (the package's own `url` lacked `serde`), and the AC8 check found stale diagram edges from rows 4 and 5.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory pattern A; EPIC #1488 subissue
  [#2450](../../closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md)
