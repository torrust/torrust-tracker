---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 2482
github-issue: 2745
spec-path: docs/issues/closed/2745-2482-gate-axum-rest-api-server-test-environment/ISSUE.md
branch: "2745-2482-gate-axum-rest-api-server-test-environment-spec"
related-pr: 2747
last-updated-utc: "2026-11-10 12:33"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/closed/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/closed/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/closed/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md
    - docs/issues/closed/2724-2482-gate-axum-http-server-test-environment/ISSUE.md
    - docs/issues/closed/2699-2482-gate-udp-server-test-environment/ISSUE.md
    - packages/axum-rest-api-server/src/lib.rs
    - packages/axum-rest-api-server/src/testing/environment.rs
    - packages/axum-rest-api-server/src/testing/mod.rs
    - packages/axum-health-check-api-server/Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #2745 - Gate the `axum-rest-api-server` test environment behind a `testing` feature

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 6 of its
Subissues table. Draft written by subissue 1 (#2618), refreshed on 2026-11-06 against
`torrust/develop`. Execution order: after row 5 (done); row 7 follows.

## Goal

Apply rule A of the
[ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md) to
`axum-rest-api-server`: compile `src/testing/` only with a `testing` feature, make the seven
dependencies only that module and the tests use optional, and let the package's own targets and its
cross-package consumer enable the feature, so the package no longer carries UDP, HTTP-core and
tracker-core edges that its production code does not use.

## Background

`axum-rest-api-server` exposes `pub mod testing;` (`src/lib.rs`), whose `environment.rs` (536 lines
on `develop` on 2026-11-06; the draft said 232) starts the REST API on the full `Configuration`,
wiring the HTTP and UDP cores, compiled in every build. Consumers: its own `tests/` (a single
`integration.rs` target with the `server/` and `common/` modules), its own unit tests
(`src/server.rs`, `src/test_support.rs`) and the `axum-health-check-api-server` tests
(`tests/server/contract.rs`), which dev-depend on the package. The root crate
(`src/bootstrap/jobs/tracker_apis.rs`) uses the package for the real server and never imports
`testing`. The package has no `examples/` and no benches.

`src/testing/mod.rs` still says the UDP dependencies are needed at runtime until "the prerequisite
decoupling in `rest-api-core`", a crate that no longer exists (#1938). The claim is false today:
production code reaches the UDP and HTTP cores only through `rest-api-runtime-adapter` (its
`TrackerHttpApiCoreContainer`). This subissue removes that comment.

Dependency audit (2026-11-06, `cargo tree -p torrust-tracker-axum-rest-api-server -e normal --depth 1`
and a per-crate source grep that cuts each file at its `#[cfg(test)]` module and ignores
`src/testing/`, `src/test_support.rs` and `tests/`):

| Direct normal dependency | Used by production code? | Result |
| --- | --- | --- |
| `torrust-tracker-core` | No: `src/testing/environment.rs` and `tests/` only | Optional behind `testing` |
| `torrust-tracker-http-core` | No: `src/testing/environment.rs` only | Optional behind `testing` |
| `torrust-tracker-udp-core` | No: `src/testing/environment.rs` only | Optional behind `testing` |
| `torrust-tracker-udp-server` | No: `src/testing/environment.rs` only | Optional behind `testing` |
| `torrust-tracker-swarm-coordination-registry` | No: `src/testing/environment.rs` only | Optional behind `testing` |
| `torrust-tracker-rest-api-client` | No: `src/testing/environment.rs` and `tests/` (it is also already a dev-dependency) | Optional behind `testing`; the duplicate normal entry goes |
| `torrust-clock` | One unused item: `src/lib.rs:164` imports `torrust_clock::clock` for `pub(crate) type CurrentClock` (`clock::Working` in production, `clock::Stopped` in tests, `#[allow(dead_code)]`). Nothing in `src` uses the alias; `tests/integration.rs` carries its own copy. The other uses are test code (`src/server.rs` unit test, `src/testing/environment.rs`, `src/test_support.rs`, `tests/integration.rs`) | Optional behind `testing` after the dead alias and its import are deleted (maintainer decision, below) |
| every other direct dependency (28 in total) | Yes | Unchanged |

Each of the seven optional crates stays reachable through production dependencies (five through
`rest-api-runtime-adapter`); the change removes the direct edges only.

## Gate

The gate is satisfied. The draft was gated on #1488's #2449 (migrate the REST API test
environment to the token lifecycle). #2449 is closed, as are #2416 and #2450. Later commits touched
the environment (it grew from 232 to 536 lines); none changes what this subissue moves. The row 4
(#2699) and row 5 (#2724) precedents apply, including the container build fix: the pin
`torrust-cargo-chef` 0.1.79 is already in the `Containerfile`.

## Scope

### In Scope

- Audit every normal dependency of the package, workspace or third-party, against the ADR's rule
  A.2 (done in Background; T1 re-checks it): optional behind `testing` if only the module,
  `#[cfg(test)]` code and `tests/` use it.
- Delete the dead `CurrentClock` alias and its `use torrust_clock::clock;` import from
  `src/lib.rs` (the alias has no user; `tests/integration.rs` keeps its own copy).
- Gate `pub mod testing;` with `#[cfg(feature = "testing")]`; add `testing` (not in `default`)
  with `dep:` entries for the seven dependencies, and make them optional. Remove the duplicate
  `torrust-tracker-rest-api-client` normal entry's non-optional form (it stays a dev-dependency).
- Add the path-only self dev-dependency (no `version`) with `features = ["testing"]`.
- Make the self dev-dependency's `deny.toml` wrapper entry in the same commit as the manifest.
- Enable `features = ["testing"]` on `torrust-tracker-axum-rest-api-server` in
  `axum-health-check-api-server`'s `[dev-dependencies]`, in the same pull request.
- Remove the stale `rest-api-core` comment; add the feature comment (test support, outside
  semver); no `testing` in `[package.metadata.docs.rs]`.
- Run the container recipe check (`cargo chef prepare --external-only`, then `cargo chef cook`) and
  the Shipped Features check before the first push.
- Update the inventory (pattern A row for `axum-rest-api-server`) and the diagram, including the
  REST API layer diagram if it shows these edges.

### Out of Scope

- Changing the environment's behaviour or its drop path (#2449 and #2416 are done).
- Examples: the package has none (maintainer decision, below).
- The other environments (rows 4, 5 and 7). Any version bump (#2583).

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rule A; `axum-rest-api-server` owns the REST API environment).
- ADRs to create: None known.

### Decision: `torrust-clock`

Decided by the maintainer on 2026-11-06 in this session, after the dead use was explained: delete
the unused `CurrentClock` alias and its import from `src/lib.rs`, and make `torrust-clock` the
seventh optional dependency. Rejected: keeping it a normal dependency and recording the dead alias
as a follow-up (leaves a direct edge that no live production code uses, which is exactly what rule
A.2 removes). The deletion cannot change behaviour: a dead `pub(crate)` type alias has no user.

### Decision: examples

Decided by the maintainer on 2026-11-06: the package has no `examples/` directory, so the
testing-style plus production-style pair of rows 4 and 5 does not apply and nothing is added.
Rejected: adding a REST API example pair (widens a pure gate into new example work).

### Decision: issue labels and sub-issue link

Decided by the maintainer on 2026-11-06: the bot account cannot add labels or link sub-issues on
`torrust/torrust-tracker`; the agent runs those commands with the `josecelano` account only after
showing them and receiving his explicit OK in this session, as in row 5.

## Design and Ownership Review

`axum-rest-api-server` owns the REST API environment; the health check package uses it only through
a dev-dependency feature. The environment starts the REST API and, through the cores, UDP and HTTP
services; its start, stop and drop-path behaviour is unchanged by this move. No new asynchronous
path is added (no example, no new task): the change is a `cfg` gate, manifest edits and one dead
alias deletion. After the first passing slice (the gate plus the package's own tests), review the
consumer.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. The package's integration test target and unit tests are the regression suite;
their count must not drop.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Record the baseline | `cargo tree -p torrust-tracker-axum-rest-api-server -e normal --depth 1` (35 direct dependencies; the first version of this row said 34, a miscount corrected on 2026-11-06); measured on `torrust/develop` at `da8bd15d1` (2026-11-06 09:13 UTC), plain `cargo test` without `--all-features`: axum-rest-api-server 162 unit + 58 integration + 0 doc; axum-health-check-api-server 15 unit + 11 integration + 0 doc. T2 remeasures on the implementation base, with and without `--all-features`, before comparing |
| T2 | DONE | Delete the dead alias, gate the module, add the feature, make the seven dependencies optional, add the self dev-dependency and its `deny.toml` wrapper | Plain `cargo test -p torrust-tracker-axum-rest-api-server` passes with the baseline counts; `cargo check -p torrust-tracker-axum-rest-api-server --lib` passes with no features; `Cargo.lock` and `deny.toml` in the same commit |
| T3 | DONE | Enable the feature in the health check package's dev-dependency | Its tests pass with the baseline counts; without the entry they fail to compile |
| T4 | DONE | Remove the stale comment; update inventory, diagram and docs | Pattern A row resolved for `axum-rest-api-server`; EPIC row note |
| T5 | DONE | Verify the container recipe and the shipped features | `cargo chef prepare --external-only` then `cargo chef cook --tests --workspace --all-features` exit 0 in a scratch directory; `cargo run -q --package shipped-features-check -- --package torrust-tracker` exits 0 |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Dead alias, gate, feature, optional dependencies, self dev-dependency, `deny.toml` wrapper | Commit after the package's tests pass without `--all-features`. |
| T3 | Health check dev-dependency | Commit after its tests pass. |
| T4 | Comment removal and docs | One commit. |
| T5 | Evidence | One commit with the spec update. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/closed/2745-2482-gate-axum-rest-api-server-test-environment/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer (2026-11-06)
- [x] GitHub issue created (#2745), linked as a sub-issue of #2482, and its number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 6 of
  the #2482 Subissues table and rule A of the ADR; gated by #2449.
- 2026-10-22 18:10 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Changed after PR #2624's review: AC1 and M1 check direct dependencies (`--depth 1`) because five of the six crates stay reachable through `rest-api-runtime-adapter`; added the dependency audit to the scope.
- 2026-11-06 09:14 UTC - AI assistant (Claude Code, #2482 row 6) - Refreshed the draft against `torrust/develop` (environment.rs now 536 lines; the gate is satisfied, #2449, #2416 and #2450 are closed; dependency audit re-run). Recorded the maintainer's decisions of 2026-11-06: `torrust-clock` becomes the seventh optional dependency after the dead `CurrentClock` alias is deleted; no examples (the package has none); labels and the sub-issue link are done with the `josecelano` account on his explicit OK.

- 2026-11-06 09:16 UTC - AI assistant (Claude Code, #2482 row 6) - The maintainer approved the spec. Created issue #2745 as `josecelano-bot`; the bot cannot add labels or link sub-issues on `torrust/torrust-tracker`, so those two steps are done with the `josecelano` account on the maintainer's explicit OK (recorded in a later entry). The spec moved to `docs/issues/open/`; the #2482 EPIC row, the inventory, the coupling report and the #2637 spec link to it.
- 2026-11-06 09:19 UTC - AI assistant (Claude Code, #2482 row 6) - At the maintainer's explicit OK in this session, the labels (`task`, `packages`) and the sub-issue link under #2482 were done with the `josecelano` account, because the bot account lacks those permissions. The link is verified (`parent_issue_url` is #2482).
- 2026-11-06 09:24 UTC - AI assistant (Claude Code, #2482 row 6) - Opened spec-only PR #2746 (`Related to #2745`); `related-pr` is set to 2746.
- 2026-11-06 10:20 UTC - AI assistant (Claude Code, #2482 row 6) - Implemented T2 to T5 on branch `2745-2482-gate-axum-rest-api-server-test-environment`: the `testing` feature, seven optional dependencies, the deleted `CurrentClock` alias, the path-only self dev-dependency with its `deny.toml` wrapper, the health check dev-dependency feature and the replaced module note. Plain package tests keep the baseline counts (162 + 58, 15 + 11); the evidence is in `manual-verification-evidence.md`. The baseline direct dependency count is corrected to 35 (the spec said 34). No retrospective is needed (see the completion review).
- 2026-11-06 14:21 UTC - AI assistant (Claude Code, #2482 row 6) - PR #2747 was merged and issue #2745 is closed. The spec is archived to `docs/issues/closed/` with `status: done` and `related-pr: 2747` (the implementation PR; the spec-only PR was #2746).

## Acceptance Criteria

- [x] AC1: `cargo tree -p torrust-tracker-axum-rest-api-server -e normal --depth 1` shows none of
      the seven optional crates as a direct dependency (five stay reachable through
      `rest-api-runtime-adapter`), and `src/testing/` is not compiled without the feature.
- [x] AC2: `cargo test -p torrust-tracker-axum-rest-api-server` and
      `-p torrust-tracker-axum-health-check-api-server` pass without `--all-features`, with the
      baseline test counts.
- [x] AC3: The self dev-dependency has no `version`; `cargo +stable package --workspace
      --no-verify` passes; `cargo deny check bans` passes.
- [x] AC4: The stale `rest-api-core` comment and the dead `CurrentClock` alias are gone.
- [x] AC5: The inventory row is marked resolved.
- [x] AC6: The container recipe cooks (`prepare --external-only` then `cook`) and the Shipped
      Features check exits 0; no `testing` feature is in a `[dependencies]` entry or a `default`
      chain.
- [x] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies. This is an O1 move: build the package without the feature, run `cargo test -p <package>`
without `--all-features`, and check the test count.

### Automatic Checks

- `linter all`, the tests of both packages with and without `--all-features`, `cargo +stable
  package --workspace --no-verify`, `cargo machete --with-metadata`, `cargo deny check bans`,
  pre-push checks.
- `cargo check -p torrust-tracker-axum-rest-api-server --lib` with no features;
  `cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features`;
  no `required-features` on any target; the root crate's tests.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The production build has no test-only dependency | `cargo tree -p torrust-tracker-axum-rest-api-server -e normal --depth 1` before and after | The seven forced direct dependencies are gone (paths through production dependencies remain) | DONE | `manual-verification-evidence.md` V1 |
| M2 | Plain package tests | `cargo test -p torrust-tracker-axum-rest-api-server` (no `--all-features`) | Test count equals the baseline | DONE | V2 |
| M3 | The consumer still works | `cargo test -p torrust-tracker-axum-health-check-api-server` | Passes with the baseline count | DONE | V3 |
| M4 | Shipped features | `cargo run -q --package shipped-features-check -- --package torrust-tracker`, then with `features = ["testing"]` on the root `axum-rest-api-server` entry (reverted afterwards) | Exit 0 on the real tree, exit 1 with the feature on a `[dependencies]` edge | DONE | V4 |
| M5 | Deny, machete, packaging, nightly check | The commands of the Verification Plan | All exit 0; `deny` fails without the self wrapper | DONE | V5 |
| M6 | The container recipe cooks | `cargo chef prepare --external-only --recipe-path <file>` then `cargo chef cook --tests --workspace --all-features --recipe-path <file>` in a scratch directory | Exit 0 | DONE | V6 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `manual-verification-evidence.md` |
| AC2   | DONE                   | `manual-verification-evidence.md` |
| AC3   | DONE                   | `manual-verification-evidence.md` |
| AC4   | DONE                   | `manual-verification-evidence.md` |
| AC5   | DONE                   | `manual-verification-evidence.md` |
| AC6   | DONE                   | `manual-verification-evidence.md` |

## Risks and Trade-offs

- The environment changed since the draft (232 to 536 lines, #2449, #2416). Mitigation: the audit
  above is from 2026-11-06 and T1 re-reads the module and re-greps the consumers.
- Making workspace path dependencies optional can break the container build's
  `cargo chef prepare --external-only` stage; fixed by `torrust-cargo-chef` 0.1.79 (pinned) and
  checked by T5 before the first push.
- A path-only self dev-dependency fails `cargo deny check bans` until the crate is in its own
  `wrappers` list: the manifest and `deny.toml` change in the same commit.
- The tests use `torrust-tracker-core`, `torrust-tracker-rest-api-client` and `torrust-clock`; they
  reach those targets through the self dev-dependency. A packaged `.crate` therefore needs
  `--features testing` to build its `tests/` (the same known limit as rows 4 and 5).
- Deleting `CurrentClock` could break a consumer that uses it: it is `pub(crate)`, so none can;
  T2 confirms with a workspace build.

## Implementation Completion Review

- Retrospective: not needed. The implementation matched the spec; the only deviations were a miscounted baseline (34 instead of 35 direct dependencies, corrected) and the need for a scratch `cargo-chef` 0.1.79 for the container recipe check, as in rows 4 and 5. Neither changes the design or the process.
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory pattern A; row 4 spec [`#2699`](../../closed/2699-2482-gate-udp-server-test-environment/ISSUE.md)
  and row 5 spec [`#2724`](../../closed/2724-2482-gate-axum-http-server-test-environment/ISSUE.md)
  (models); EPIC #1488 subissue [#2449](../../closed/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md)
