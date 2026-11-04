---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 2482
github-issue: 2699
spec-path: docs/issues/open/2699-2482-gate-udp-server-test-environment/ISSUE.md
branch: "2699-2482-gate-udp-server-test-environment"
related-pr: null
last-updated-utc: "2026-11-04 11:35"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - packages/udp-server/src/testing/environment.rs
    - packages/udp-server/examples/udp_only_public_tracker.rs
    - packages/udp-server/examples/udp_only_public_tracker_production_style.rs
    - packages/axum-health-check-api-server/Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #2699 - Gate the `udp-server` test environment behind a `testing` feature

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

`udp-server` exposes `pub mod testing;` (`src/lib.rs`), whose `environment.rs` (878 lines on `develop` on 2026-11-03) starts a
UDP tracker on an ephemeral address for tests, compiled in every build. It forces one normal
dependency that production code does not use: `torrust-tracker-swarm-coordination-registry`.
Consumers: its own `tests/server/contract.rs`; its own unit tests in `src/handlers/mod.rs` and
`src/server/processor.rs`; `examples/udp_only_public_tracker.rs`; and the
`axum-health-check-api-server` tests, which dev-depend on `udp-server`.

## Gate

The gate is satisfied: #2416 (stop event producers before event listeners, part of #2410) closed on
2026-10-24. Its task T9 (`fix(axum-http-server,udp-server): [#2416] stop the server before the
listeners in the test environments`) changed the UDP environment: the `Running` state holds two
independent cancellation tokens with drop guards (`server_token`, `listeners_token`); `stop()`
cancels and joins the server first, then the listeners, so events of requests completed while the
server drains are still processed; failures are reported after every task is joined; dropping an
environment without `stop()` cancels both tokens. Later commits added tests for it and applied the
configured request drain deadline. None of this changes what this subissue moves: the module's
public surface and its single forced dependency are unchanged. No commit touched the four server
packages on `develop` between 2026-10-29 and 2026-11-03 (verified with `git log` on `torrust/develop`).

## Scope

### In Scope

- Audit every normal dependency of the package, workspace or third-party, against the ADR's rule
  A.2: optional behind `testing` if only the module and `#[cfg(test)]` code use it, a
  dev-dependency if only `#[cfg(test)]` code uses it.
- Gate `pub mod testing;` with `#[cfg(feature = "testing")]`; add `testing` (not in `default`)
  with `dep:torrust-tracker-swarm-coordination-registry`, and make that dependency optional.
- Add the path-only self dev-dependency (no `version`) with `features = ["testing"]`; the unit tests
  then need only the feature gate.
- Enable `features = ["testing"]` on `torrust-tracker-udp-server` in
  `axum-health-check-api-server`'s `[dev-dependencies]`, in the same pull request.
- Add the feature comment (test support, outside semver) and keep `testing` out of
  `[package.metadata.docs.rs]`.
- Examples (maintainer decision, see Architectural Decisions): keep
  `examples/udp_only_public_tracker.rs` on the test environment and add a new production-style
  example next to it; both doc headers say which one follows the testing conventions and which one
  simulates production as closely as possible (see the decision for the exact wording requirements).
  This grows the scope beyond a pure gate on purpose.
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

### Decision: what the examples become

The ADR (section "What this ADR does not decide") leaves open whether the example that starts a
tracker through an environment keeps doing so. Decided by the maintainer on 2026-11-03 through
`ask_user`: keep the existing example on the test environment, add a NEW production-style example,
and make both doc headers explain the difference.

| Option | Strengths | Weaknesses | Outcome |
| --- | --- | --- | --- |
| (a) Relabel the existing example as a test-support demonstration only | Smallest change; keeps the object-oriented demo | The package then has no example of starting a UDP tracker the way the binary does | Not chosen alone (recommended by the assistant) |
| (b) Rewrite the existing example to a production-style startup | One example; shows real startup | Loses the testing-style demonstration; large rewrite | Rejected |
| (c) Delete the example | No label to maintain | Loses documentation of narrowed-configuration initialisation (#1861, DEC-09) | Rejected |
| (d) `required-features = ["testing"]` on the example | No code change | Banned by the ADR (a skipped target is silent); `cargo run` would fail without the flag | Rejected |
| (e) Keep it and add a production-style example, label both | Both styles documented; no behaviour lost | More scope: one new example to build and keep working | **Chosen** |

Requirements of the chosen option:

- The existing example keeps using `testing::environment` and its doc header says it follows the
  conventions used in testing.
- The new example (name proposed: `udp_only_public_tracker_production_style`) does NOT use
  `testing`. It starts the server the way `src/bootstrap/jobs/udp_tracker.rs` does
  (`Server::new(Spawner::new(bind_to)).start_with_cancellation(...)`) and runs the event listeners
  as `environment.rs` does, and runs with plain `cargo run -p torrust-tracker-udp-server --example
  <name>`. Its header says it simulates production as closely as possible.
- Both headers say: production startup may become more testing-like in the future, and an end user
  may adopt the object-oriented, testing-style version if they prefer it.
- Both examples must build under `cargo test --examples` and `cargo +nightly check --tests
  --benches --examples --workspace --all-targets --all-features`.
- Open implementation detail: hand-building `SwarmCoordinationRegistryContainer` needs the crate
  that becomes optional. The self dev-dependency enables `testing` and therefore the crate for
  every own target, so the new example compiles under cargo's normal dev feature unification; the
  implementation records whether an explicit dev-dependency is needed instead (T4), verified by
  building the example with `cargo build -p torrust-tracker-udp-server --example <name>`.

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
| T1 | DONE | Record the baseline (#2416 merged) | `cargo tree -p torrust-tracker-udp-server -e normal --depth 1`; test counts of `cargo test -p torrust-tracker-udp-server` and `-p torrust-tracker-axum-health-check-api-server` with and without `--all-features`. Measured 2026-11-03 17:04 to 17:05 UTC: udp-server 233 unit + 16 integration + 1 doc passed (1 ignored) + one further target reporting 2 passed; health check 15 unit + 8 integration; identical with and without `--all-features` before the change |
| T2 | DONE | Gate the module, add the feature and optional dependency, add the self dev-dependency | Plain `cargo test -p torrust-tracker-udp-server` passes with the baseline counts (233 + 16 + 1 + 2; evidence V2). `Cargo.lock` and the `deny.toml` self wrapper are in the same commit |
| T3 | DONE | Enable the feature in the health check package's dev-dependency | Its tests pass with the baseline counts (15 + 8); without the entry they fail to compile (evidence V3) |
| T4 | DONE | Relabel the existing example and add the production-style example | Both build, run under plain `cargo run` and answer a real UDP connect request (evidence V6, V7); headers carry the required wording. Decision on the open implementation detail: no explicit dev-dependency on the registry crate was added; the new example imports it and compiles through the self dev-dependency, which enables `testing` (and so the optional crate) for every own target, like the `src/` unit tests. The example does not import the `testing` module |
| T5 | DONE | Update inventory, diagram and docs | Inventory pattern A marked resolved for `udp-server`. `docs/packages.md` (its diagram shows layers, not test-only edges) and the package README mention no test environment, so they need no change; the coupling report of 2026-10-06 is not edited |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Gate, feature, optional dependency, self dev-dependency | Commit after the package's tests pass without `--all-features`. |
| T3 | Health check dev-dependency | Commit after its tests pass. |
| T4 | Examples | One commit (two if the relabel and the new example are independent). |
| T5 | Docs | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2482-gate-udp-server-test-environment/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2482, and its number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 4 of
  the #2482 Subissues table and rule A of the ADR; gated by #2416.
- 2026-10-22 18:10 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Changed after PR #2624's review: AC1 and M1 check direct dependencies (`--depth 1`) because the forced crate stays reachable through `tracker-core`; added the dependency audit to the scope.
- 2026-11-03 17:06 UTC - AI assistant (Copilot SDK in VS Code, #2482 row 4) - Refreshed the draft against `develop` (environment.rs now 878 lines; Gate satisfied, #2416 T9 recorded; baseline counts measured, T1 DONE); recorded the maintainer's example decision (keep, add a production-style example, label both).
- 2026-11-03 17:08 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 4) - Created issue #2699 after the maintainer approved the refreshed spec, linked it as a sub-issue of #2482 and moved the spec to `docs/issues/open/`.
- 2026-11-04 11:35 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 4) - Implementation done on branch `2699-2482-gate-udp-server-test-environment` after spec PR #2700 merged (2026-11-04 10:42 UTC): the gate, the feature, the optional registry dependency, the self dev-dependency and its `deny.toml` wrapper; the health check dev-dependency; the relabelled example and the new production-style example; the inventory. The independent pre-PR review found six minor findings on the new example and the evidence, all fixed (evidence V8). Verified with and without `--all-features` (evidence V1 to V8); no test was lost.

## Acceptance Criteria

- [x] AC1: `cargo tree -p torrust-tracker-udp-server -e normal --depth 1` shows no direct dependency on
      `torrust-tracker-swarm-coordination-registry` unless production code uses it (it stays
      reachable through `tracker-core`), and
      `src/testing/` is not compiled without the feature.
- [x] AC2: `cargo test -p torrust-tracker-udp-server` and
      `-p torrust-tracker-axum-health-check-api-server` pass without `--all-features`, with the
      baseline test counts.
- [x] AC3: The self dev-dependency has no `version`; `cargo +stable package --workspace
      --no-verify` passes.
- [x] AC4: The existing example keeps the test environment and is labelled; the new production-style example does not use `testing`, runs under plain `cargo run`, and both build under `cargo test --examples` and the nightly `--all-features` check.
- [x] AC5: The inventory row is marked resolved.
- [x] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies. This is an O1 move: build the package without the feature, run `cargo test -p <package>`
without `--all-features`, and check the test count.

### Automatic Checks

Also: the Shipped Features check green on the real tree and red when `features = ["testing"]` is
placed on a `[dependencies]` edge of `torrust-tracker-udp-server` (reverted afterwards); a
`cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features`; the
bench builds (`cargo bench -p torrust-tracker-udp-server --no-run`); no `required-features` on any
target.

- `linter all`, the tests of both packages with and without `--all-features`,
  `cargo build -p torrust-tracker-udp-server --examples`, `cargo +stable package --workspace
  --no-verify`, `cargo machete --with-metadata`, `cargo deny check bans`, pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The production build has no test-only dependency | `cargo tree -p torrust-tracker-udp-server -e normal --depth 1` before and after | The forced direct dependency is gone (paths through production dependencies remain) | DONE | `manual-verification-evidence.md` V1 |
| M2 | Plain package tests | `cargo test -p torrust-tracker-udp-server` (no `--all-features`) | Test count equals the baseline | DONE | V2 |
| M3 | The consumer still works | `cargo test -p torrust-tracker-axum-health-check-api-server` | Passes with the baseline count | DONE | V2, V3 |
| M4 | Shipped features | `cargo run -p shipped-features-check -- --package torrust-tracker`, then with `features = ["testing"]` on the root `udp-server` entry | Exit 0 on the real tree, exit 1 with the feature on a `[dependencies]` edge | DONE | V4 |
| M5 | Dependents and the shipped binary | Tests of `axum-rest-api-server` and `rest-api-runtime-adapter`; `cli-configuration` with `TORRUST_TRACKER_BINARY` set to a preserved shipped-profile binary | Baseline counts; no test environment in the binary | DONE | V2, V4 |
| M6 | Examples | Build, `cargo run`, a UDP connect request and SIGTERM for both examples | Both run and stop cleanly; only the existing one imports `testing` | DONE | V6, V7 |
| M7 | Deny, machete, packaging, nightly check, bench build | The commands of the Verification Plan | All exit 0; `deny` fails without the self wrapper | DONE | V5, V6 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | V1 (direct dependencies 27 to 26; module absent from default rustdoc output; build with and without the feature) |
| AC2   | DONE                   | V2 (233 + 16 + 1 + 2 and 15 + 8, equal to the baseline, with and without `--all-features`) |
| AC3   | DONE                   | V5 (`cargo +stable package --workspace --no-verify` exit 0; the self dev-dependency has no `version`) |
| AC4   | DONE                   | V6, V7 (both examples build, run and answer a UDP connect request; the new one has no `use` of the testing module) |
| AC5   | DONE                   | Inventory, pattern A, `udp-server` (marked resolved) |

## Risks and Trade-offs

- #2416 may change the environment between this draft and the implementation. Mitigation: T1 starts
  only after it merges, and re-reads the module.
- The Containerfile builds shipped binaries in their own stages and test archives keep
  `--all-features`, so the feature is on only in the test archive; the evidence runs the shipped
  binary (`TORRUST_TRACKER_BINARY`) in a native-tracker test and records checksums.
- Dependency audit (2026-11-03, `cargo tree -p torrust-tracker-udp-server -e normal --depth 1`
  plus source grep): the only direct dependency used solely by `src/testing/` is
  `torrust-tracker-swarm-coordination-registry` (stays transitive through `tracker-core` and
  `udp-core`); every other direct normal dependency has production uses; `mockall`, `rand`,
  `criterion` and `torrust-tracker-test-helpers` are already dev-only.
- `deny.toml`: the `torrust-tracker-udp-server` wrappers entry gets the package itself (with a
  comment, as row 3 did) in the SAME commit as the manifest change.
- Another package may start using the environment before this lands; T1 re-greps the consumers.

## Implementation Completion Review

Reviewed by the AI assistant against the acceptance criteria and the evidence on 2026-11-04 11:35 UTC, after an independent Task Reviewer pre-PR review (evidence V8: PASS WITH FINDINGS, no blocker, every finding fixed or answered); the maintainer reviews it in the pull request.

- Scope: the change is in `packages/udp-server` (`Cargo.toml`, `src/lib.rs`, the `src/testing/mod.rs` note, one relabelled and one new example), the health check package's `Cargo.toml`, `deny.toml`, `Cargo.lock`, the inventory and the EPIC row note. No test body changed, no `required-features` was added, and no `testing` feature is in a `[dependencies]` entry or a `default` chain.
- Design: as decided. Deviation from the spec text: the new example keeps spawning its listeners itself instead of handing them to a supervisor (its header says so); `docs/packages.md` needed no change.
- Known limits: the test archives keep `--all-features`; `testing` is public surface outside semver (#2583 decides versions); the production-style example compiles through the self dev-dependency, so a packaged crate needs `--features testing` to build it (the same limit as the `src/` unit tests); the ADR still lists the example question as open, and this spec records the decision.
- Retrospective: created, [`implementation-retrospective.md`](implementation-retrospective.md): the `Cargo.lock` of the self dev-dependency, and checking an example's "as production" claim against the production code.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory pattern A; EPIC #1488 subissue #2416
