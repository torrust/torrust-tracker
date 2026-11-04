---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2482
github-issue: 2724
spec-path: docs/issues/open/2724-2482-gate-axum-http-server-test-environment/ISSUE.md
branch: "2724-2482-gate-axum-http-server-test-environment-spec"
related-pr: null
last-updated-utc: "2026-11-04 18:14"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - packages/axum-http-server/src/testing/environment.rs
    - packages/axum-http-server/examples/http_only_public_tracker.rs
    - docs/issues/closed/2699-2482-gate-udp-server-test-environment/ISSUE.md
    - packages/axum-health-check-api-server/Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #2724 - Gate the `axum-http-server` test environment behind a `testing` feature

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 5 of its
Subissues table. Draft written by subissue 1 (#2618), refreshed on 2026-11-04 against
`torrust/develop`. Execution order: after row 4 (done); rows 6 and 7 follow.

## Goal

Apply rule A of the
[ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md) to
`axum-http-server`: compile `src/testing/` only with a `testing` feature, make the dependency only
that module uses optional, and let the package's own targets and its cross-package consumer enable
the feature.

## Background

`axum-http-server` exposes `pub mod testing;` (`src/lib.rs`), whose `environment.rs` (716 lines on
`develop` on 2026-11-04; the draft said 484) starts an HTTP tracker on an ephemeral address for
tests, compiled in every build. Consumers: its own `tests/` (nine files under `tests/server/`),
its own unit tests (`src/server.rs`, which builds a `SwarmCoordinationRegistryContainer`, and
`src/testing/environment.rs`), `examples/http_only_public_tracker.rs`, and the
`axum-health-check-api-server` tests (`tests/server/contract.rs`), which dev-depend on the package.
The root crate (`src/app.rs`, `src/bootstrap/jobs/http_tracker.rs`) uses the package for the real
server and never imports `testing`.

Dependency audit (2026-11-04, `cargo tree -p torrust-tracker-axum-http-server -e normal --depth 1`
shows 26 direct dependencies, plus a source grep split at each file's `#[cfg(test)]` module):

| Direct normal dependency | Used outside `src/testing/`, `#[cfg(test)]` code and `tests/`? | Result |
| --- | --- | --- |
| `torrust-tracker-swarm-coordination-registry` | No: `src/server.rs` unit test and `src/testing/environment.rs` only | Optional behind `testing` |
| `torrust-clock` | No: `src/server.rs` unit test (`initialize_static`), `src/testing/environment.rs:261`, `tests/integration.rs` | Optional behind `testing` |
| `torrust-info-hash` | No: unit-test modules (`lib.rs`, `handlers/scrape.rs`, `extractors/*`), `src/testing/environment.rs`, `tests/` | Optional behind `testing` |
| `torrust-tracker-configuration` | Only an intra-doc link, `src/v1/extractors/client_ip_sources.rs:19` (the environment, unit tests, `tests/` and the example use it too) | Stays a normal dependency (maintainer decision, below) |
| every other direct dependency (for example `reqwest`, which `src/server.rs` uses for the health check client, and `torrust-server-lib`, `torrust-net-primitives`, `torrust-tracker-primitives`, `torrust-tracker-core`, `torrust-tracker-http-core`) | Yes | Unchanged |

Each of the three optional crates stays reachable through production dependencies (for example
`torrust-info-hash` and `torrust-clock` through `tracker-core`); the change removes the direct
edges only.

## Gate

The gate is satisfied. The dependencies on #1488 that the draft listed are all closed: #2471
(HTTP environment drop path) merged on 2026-10-16; #2416 (task T9 changed the HTTP and UDP
environments: the server stops before the listeners) closed on 2026-10-24; #2449 and #2450 are
closed too. Later commits touched the package (the configured drain timeout, 2026-10-27; the
`torrust-server-lib` 0.4.0 bump, 2026-10-26). None changes what this subissue moves. The row 4
precedent (#2699, merged 2026-11-04) applies, including its container build fix: the pin
`torrust-cargo-chef` 0.1.79 is already in the `Containerfile`.

## Scope

### In Scope

- Audit every normal dependency of the package, workspace or third-party, against the ADR's rule
  A.2 (done in Background; T1 re-checks it): optional behind `testing` if only the module,
  `#[cfg(test)]` code and `tests/` use it.
- Gate `pub mod testing;` with `#[cfg(feature = "testing")]`; add `testing` (not in `default`)
  with `dep:torrust-tracker-swarm-coordination-registry`, `dep:torrust-clock` and
  `dep:torrust-info-hash`, and make those three dependencies optional.
- Add the path-only self dev-dependency (no `version`) with `features = ["testing"]`.
- Enable `features = ["testing"]` on `torrust-tracker-axum-http-server` in
  `axum-health-check-api-server`'s `[dev-dependencies]`, in the same pull request.
- The feature comment (test support, outside semver); no `testing` in
  `[package.metadata.docs.rs]`.
- Examples (maintainer decision, see Architectural Decisions): keep
  `examples/http_only_public_tracker.rs` on the test environment and add a new production-style
  example next to it; both doc headers say clearly why the two exist and where they differ. This
  grows the scope beyond a pure gate on purpose, as in row 4.
- Make the self dev-dependency's `deny.toml` wrapper entry in the same commit as the manifest.
- Run the container recipe check (`cargo chef prepare --external-only`, then `cargo chef cook`) and
  the Shipped Features check before the first push.
- Update the inventory (pattern A row for `axum-http-server`) and the diagram if it shows the edge.

### Out of Scope

- Changing the environment's behaviour or its drop path (#2471 and #2416 are done).
- Making `torrust-tracker-configuration` optional, or rewording the doc link at
  `src/v1/extractors/client_ip_sources.rs:19`.
- The other environments (rows 6 and 7; row 4 is done). Any version bump (#2583).

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rule A; `axum-http-server` owns the HTTP environment).
- ADRs to create: None known. The examples decision of row 4 is recorded in the ADR by an
  amendment note (PR #2723).

### Decision: what the examples become

Decided by the maintainer on 2026-11-04 in this session, following the same problem in `udp-server`
(#2699): keep two examples, one in production style and one in testing style, and document clearly
why they exist and where they differ.

| Option | Strengths | Weaknesses | Outcome |
| --- | --- | --- | --- |
| (a) Relabel the existing example as a test-support demonstration only | Smallest change | The package has no example of starting the HTTP tracker the way the binary does | Rejected |
| (b) Rewrite the existing example to a production-style startup | One example | Loses the testing-style demonstration of the narrowed configuration (#1861, DEC-09) | Rejected |
| (c) Delete the example | No label to maintain | Loses that documentation | Rejected |
| (d) `required-features = ["testing"]` on the example | No code change | Banned by the ADR (a skipped target is silent); `cargo run` would fail without the flag | Rejected |
| (e) Keep it and add a production-style example, label both | Both styles documented; no behaviour lost; the same as `udp-server` | One more example to build and keep working | **Chosen** |

Requirements of the chosen option:

- The existing example keeps using `testing::environment` and its doc header says it follows the
  conventions used in testing and why that style exists (object-oriented start, ready, stop, with
  an ephemeral port, the way the package's own tests start a tracker).
- The new example, `examples/http_only_public_tracker_production_style.rs`, does NOT use
  `testing`. It starts the server the way `src/bootstrap/jobs/http_tracker.rs` does, runs the
  event listeners as `environment.rs` does, and runs with plain `cargo run -p
  torrust-tracker-axum-http-server --example http_only_public_tracker_production_style`. Its
  header says it simulates production as closely as possible and lists where it differs from the
  testing-style example (T4 compares the startup against the production code before claiming
  "as production", the lesson of #2699).
- Both headers say: production startup may become more testing-like in the future, and an end user
  may adopt the object-oriented, testing-style version if they prefer it. Each header links to the
  other example.
- Both examples build under `cargo test --examples` and `cargo +nightly check --tests --benches
  --examples --workspace --all-targets --all-features`.

### Decision: which dependencies become optional

Decided by the maintainer on 2026-11-04: `torrust-tracker-swarm-coordination-registry`,
`torrust-clock` and `torrust-info-hash` become optional behind `testing`; `torrust-tracker-configuration`
stays a normal dependency (its only production use is an intra-doc link). Rejected: making only
the registry optional (leaves two direct edges that production code never uses), and making all
four optional (would force rewording a public doc link for no coupling benefit: `configuration` is a
leaf data crate that the production dependencies need anyway).

## Design and Ownership Review

`axum-http-server` owns the HTTP environment; the health check package uses it only through a
dev-dependency feature. Start, ready, stop and drop-path behaviour of the existing environment are
unchanged by this move (its server-before-listeners stop order and drop guards stay as they are).

The one new asynchronous startup path is the production-style example (T4). Before it is written:

- **Responsibilities.** The example's `main` owns everything it starts: two cancellation tokens
  (one for the server, one for the event listeners), the server's join handle (from the same
  `start_with_cancellation` call that `src/bootstrap/jobs/http_tracker.rs` uses) and the listener
  task handles. The server and the listeners do not subscribe to OS signals; only `main` does,
  as the existing example does. It does not use the `testing` module.
- **Normal path.** On SIGINT or SIGTERM `main` cancels the server token and joins the server
  (bounded by the configured drain timeout), then cancels the listeners token and joins the
  listeners, so events of requests completed while the server drains are still processed.
- **Failure path.** Listeners are spawned only after the server start succeeded, so a failed
  start leaves no detached task; a start error is reported and the process exits non-zero.
  Every task is joined before the failures are reported.
- **Drop path.** The two tokens are held through drop guards, so an early return or a panic in
  `main` cancels both and leaves no task running; the example does not rely on `Drop` to join.
- **Absolute deadline.** One absolute deadline (computed once from `Instant::now()`) bounds the
  awaited start/readiness step and the stop sequence; no await in the example is unbounded.
- **Design-review checkpoint.** After the first passing T4 slice (the example starts, answers
  `/health_check` and an announce, and stops on SIGTERM), the author compares the startup with
  `src/bootstrap/jobs/http_tracker.rs` and `src/testing/environment.rs`, and the review is
  recorded in the evidence before the header says "as production". The consumer review of the
  health check package happens after the first passing gate slice (T2, T3), before the examples.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. The package's nine integration test files and its unit tests are the regression
suite; their count must not drop.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Record the baseline | `cargo tree -p torrust-tracker-axum-http-server -e normal --depth 1` (26 direct dependencies); measured on `torrust/develop`: axum-http-server 49 unit + 62 integration + 0 doc; axum-health-check-api-server 15 unit + 11 integration + 0 doc. A first measurement at `15da3a9c3` (2026-11-04 17:10 UTC) gave 8 integration tests for the health check package; develop then gained three readiness tests (#2697), and the remeasurement at `4e23c4356` (2026-11-04 18:04 UTC) gave 11. The same counts hold with and without `--all-features` (four runs, all at `4e23c4356`). T2 remeasures on the implementation base before comparing |
| T2 | TODO | Gate the module, add the feature, optional dependencies and self dev-dependency | Plain `cargo test -p torrust-tracker-axum-http-server` passes with the baseline counts; `cargo check -p torrust-tracker-axum-http-server --lib` passes with no features; the `deny.toml` self wrapper and `Cargo.lock` are in the same commit |
| T3 | TODO | Enable the feature in the health check package's dev-dependency | Its tests pass with the baseline counts; without the entry they fail to compile |
| T4 | TODO | Relabel the existing example and add the production-style example | Both build, run under plain `cargo run`, answer a real HTTP request (`/health_check` and an announce), stop on SIGTERM; headers carry the required wording |
| T5 | TODO | Update inventory, diagram and docs | Pattern A resolved for `axum-http-server`; EPIC row note |
| T6 | TODO | Verify the container recipe and the shipped features | `cargo chef prepare --external-only` then `cargo chef cook --tests --workspace --all-features` exit 0 in a scratch directory; `cargo run -q --package shipped-features-check -- --package torrust-tracker` exits 0 |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Gate, feature, optional dependency, self dev-dependency | Commit after the package's tests pass without `--all-features`. |
| T3 | Health check dev-dependency | Commit after its tests pass. |
| T4 | Examples | One commit (two if the relabel and the new example are independent). |
| T5 | Docs | One commit. |
| T6 | Evidence | One commit with the spec update. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2724-2482-gate-axum-http-server-test-environment/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer (2026-11-04)
- [x] GitHub issue created (#2724), linked as a sub-issue of #2482, and its number added to this spec
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
- 2026-10-22 18:10 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Changed after PR #2624's review: AC1 and M1 check direct dependencies (`--depth 1`); the scope audits third-party normal dependencies and names `torrust-clock` and `torrust-info-hash`.
- 2026-11-04 17:31 UTC - AI assistant (Claude Code, #2482 row 5) - Refreshed the draft against `torrust/develop` (environment.rs now 716 lines; the gate is satisfied, #2416, #2449 and #2450 are closed; dependency audit and baseline counts measured, T1 DONE); recorded the maintainer's decisions of 2026-11-04: keep the testing-style example and add a production-style one, documenting clearly why and where they differ (as in #2699), and make `torrust-tracker-swarm-coordination-registry`, `torrust-clock` and `torrust-info-hash` optional while `torrust-tracker-configuration` stays. The ADR amendment note on the examples decision was merged first (PR #2723). Added T6 and AC6 (container recipe, shipped features).

- 2026-11-04 17:41 UTC - AI assistant (Claude Code, #2482 row 5) - The maintainer approved the spec. Created issue #2724 as `josecelano-bot`; the bot cannot add labels or link sub-issues on `torrust/torrust-tracker`, so the maintainer does both. The spec moved to `docs/issues/open/`; the #2482 EPIC row, the inventory and the 2026-10-06 coupling report link to it.
- 2026-11-04 17:48 UTC - AI assistant (Claude Code, #2482 row 5) - At the maintainer's explicit instruction in this session, the labels (`task`, `packages`), the sub-issue link under #2482 and the review requests on PR #2725 were done with the `josecelano` account, because the bot account lacks those permissions. The link is verified (`parent_issue_url` is #2482).
- 2026-11-04 18:14 UTC - AI assistant (Claude Code, #2482 row 5) - Copilot review of PR #2725 (round 1, two low findings, both valid): the health check package has 11 integration tests on current develop, not 8 (develop gained three readiness tests from #2697 after the first measurement), so T1 is corrected and T2 remeasures on the implementation base; the Design and Ownership Review now defines the ownership, normal, failure and drop paths, the absolute deadline and the design-review checkpoint of the production-style example.

## Acceptance Criteria

- [ ] AC1: `cargo tree -p torrust-tracker-axum-http-server -e normal --depth 1` shows no direct
      dependency on `torrust-tracker-swarm-coordination-registry`, `torrust-clock` or
      `torrust-info-hash` (each stays reachable through production dependencies), and
      `src/testing/` is not compiled without the feature.
- [ ] AC2: `cargo test -p torrust-tracker-axum-http-server` and
      `-p torrust-tracker-axum-health-check-api-server` pass without `--all-features`, with the
      baseline test counts.
- [ ] AC3: The self dev-dependency has no `version`; `cargo +stable package --workspace
      --no-verify` passes; `cargo deny check bans` passes.
- [ ] AC4: The existing example keeps the test environment and is labelled; the new production-style
      example does not use `testing`, runs under plain `cargo run`, both headers explain why they
      exist and where they differ, and both build under `cargo test --examples` and the nightly
      `--all-features` check.
- [ ] AC5: The inventory row is marked resolved.
- [ ] AC6: The container recipe cooks (`prepare --external-only` then `cook`) and the Shipped
      Features check exits 0; no `testing` feature is in a `[dependencies]` entry or a `default`
      chain.
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
- `cargo check -p torrust-tracker-axum-http-server --lib` with no features;
  `cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features`;
  no `required-features` on any target; the tests of `torrust-tracker-axum-rest-api-server` and
  the root crate's `cli-configuration` tests with a shipped-profile binary.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The production build has no test-only dependency | `cargo tree -p torrust-tracker-axum-http-server -e normal --depth 1` before and after | The forced direct dependencies are gone (paths through production dependencies remain) | TODO | `manual-verification-evidence.md` V1 |
| M2 | Plain package tests | `cargo test -p torrust-tracker-axum-http-server` (no `--all-features`) | Test count equals the baseline | TODO | V2 |
| M3 | The consumer still works | `cargo test -p torrust-tracker-axum-health-check-api-server` | Passes with the baseline count | TODO | V3 |
| M4 | Shipped features | `cargo run -q --package shipped-features-check -- --package torrust-tracker`, then with `features = ["testing"]` on the root `axum-http-server` entry (reverted afterwards) | Exit 0 on the real tree, exit 1 with the feature on a `[dependencies]` edge | TODO | V4 |
| M5 | Examples | Build, `cargo run`, an HTTP request and SIGTERM for both examples | Both run and stop cleanly; only the existing one imports `testing` | TODO | V5 |
| M6 | Deny, machete, packaging, nightly check | The commands of the Verification Plan | All exit 0; `deny` fails without the self wrapper | TODO | V6 |
| M7 | The container recipe cooks | `cargo chef prepare --external-only --recipe-path <file>` then `cargo chef cook --tests --workspace --all-features --recipe-path <file>` in a scratch directory | Exit 0 | TODO | V7 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- The environment changed since the draft (484 to 716 lines, #2416). Mitigation: T1 re-read the
  module and the audit above is from 2026-11-04.
- Making workspace path dependencies optional can break the container build's
  `cargo chef prepare --external-only` stage; fixed by `torrust-cargo-chef` 0.1.79 (pinned) and
  checked by T6 before the first push.
- A path-only self dev-dependency fails `cargo deny check bans` until the crate is in its own
  `wrappers` list: the manifest and `deny.toml` change in the same commit.
- Another package may start using the environment before this lands; T1 re-greps the consumers.
- `torrust-info-hash` and `torrust-clock` are also used by `tests/`; they reach those targets
  through the self dev-dependency, like the registry. A packaged `.crate` therefore needs
  `--features testing` to build its `tests/` and `examples/` (the same known limit as row 4).
- The production-style example may be hand-built differently from the binary; T4 compares it with
  `src/bootstrap/jobs/http_tracker.rs` before the header says "as production".

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory pattern A; row 4 spec [`#2699`](../../closed/2699-2482-gate-udp-server-test-environment/ISSUE.md)
  (model, and the container build decision)
