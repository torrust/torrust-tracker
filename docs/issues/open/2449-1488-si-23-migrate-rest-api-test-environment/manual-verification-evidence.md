---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md
last-updated-utc: "2026-10-24 20:24"
---

# Manual Verification Evidence

## Purpose

Real verification of the migrated REST API test environment (#2449): the T0
baseline, the mutation proofs of the T1 tests, and the manual scenarios M1 and
M2 of the spec.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-24, 20:05 to 20:23.
- Artifact under test: the REST API test environment after the commit
  `refactor(axum-rest-api-server): [#2449] migrate the REST API test environment to the token lifecycle`
  (it also contains the health-check consumer migration), and the baseline on
  `torrust/develop` at `Merge torrust/torrust-tracker#2635: docs(issues): [#2449] refresh the SI-23 and SI-24 specs after SI-22`.
- Operating system / environment: Linux, local worktree.
- Toolchain: `rustc 1.99.1 (58cf8f9c3 2026-10-13)` and `cargo 1.99.1`
  (stable) ran every `cargo test`, `cargo check`, and `cargo clippy` command;
  `cargo +nightly fmt` formatted the package.
- Prerequisites and setup performed: none beyond the build.

## V0 - T0 baseline (before any change)

- Goal: record package test counts and timing on the unmodified `develop`.
- Status: `DONE`

Command, run at 20:05:29 to 20:06:09 UTC (the first build included, 40 s wall):

```text
cargo test -p torrust-tracker-axum-rest-api-server -p torrust-tracker-axum-health-check-api-server
```

| Target                                  | Tests | Result |
| --------------------------------------- | ----- | ------ |
| health-check API, lib unit tests        | 3     | ok     |
| health-check API, `tests/integration`   | 8     | ok     |
| REST API, lib unit tests                | 6     | ok     |
| REST API, `tests/integration`           | 58    | ok     |

REST API integration suite, three further runs of
`cargo test -p torrust-tracker-axum-rest-api-server --test integration`: test
time 0.35 s each; wall time 9.67 s (includes a rebuild), 0.57 s, and 0.58 s.

Count re-derived from the tree before the change: `grep -rn "\.stop()" packages/axum-rest-api-server/tests`
gives 58 lines (the spec's 58 holds); the only consumer of the `server` field
outside the environment was `packages/axum-health-check-api-server/tests/server/contract.rs:121`
(`service.server.stop()`), found with `grep -rn "\.server\b"` over the REST API
and health-check API packages, the root `src/`, and `tests/`.

## V1 - M1: no legacy stop in the environment

- Goal: list every `ApiServer` method the environment calls.
- Status: `DONE`

Command, run on the commit named above: `head -n 320 packages/axum-rest-api-server/src/testing/environment.rs | grep -nE "ApiServer|start_with_cancellation|\.stop\(|\.start\(|Halted"`
(line 321 is the `mod tests` line; the production code is above it).

```text
22:use crate::server::{self, ApiServer, CancellationRunning, Launcher};
29:    server: ApiServer<server::Stopped>,
80:        let server = ApiServer::new(Launcher::new(bind_to, tls));
109:            .start_with_cancellation(
139:        Environment::<Stopped>::new(configuration).await.start().await
172:                server: ApiServer::new(launcher),
```

Conclusion: the environment calls `ApiServer::new` (lines 80 and 172) and
`start_with_cancellation` (line 109). Line 139 calls `Environment::start`, the
environment's own method, not an `ApiServer` method. There is no `ApiServer::start`,
no `ApiServer::stop`, and no `Halted`. Expected result met.

## V2 - M2: suite timing

- Goal: compare the REST API integration suite before and after.
- Status: `DONE`

Same command as in V0, on the commit named above, started at 20:22:51 UTC:

| Run | Test time | Wall time |
| --- | --------- | --------- |
| 1   | 2.34 s    | 3.61 s    |
| 2   | 2.32 s    | 2.53 s    |
| 3   | 2.30 s    | 2.50 s    |

Conclusion: the suite takes about 2.3 s of test time against 0.35 s before
(wall 2.5 s against 0.57 s), roughly 2 s more for 58 tests that run in
parallel. The cause is the one the spec predicted: `stop()` now joins the drain
controller, which polls the open-connection count once per second (SI-16 saw
about 1 s per stop). The cost is that bounded wait, and it is not a leak.
This is read from the spec's Risks section and consistent with the numbers;
no profile was taken. Test counts after the change: REST API lib 14 (6 before,
8 new `environment` tests), REST API integration 58, health-check API 3 and 8,
all passing (`cargo test -p torrust-tracker-axum-rest-api-server -p torrust-tracker-axum-health-check-api-server`,
20:22 UTC).

## V3 - Mutation proofs of the T1 tests

- Goal: show that each T1 test fails when the behaviour it guards is removed.
- Status: `DONE`

Method (`write-unit-test` skill, "Prove a Test Guards Each Clause of a Rule"):
a script outside the repository copied the committed `environment.rs`, applied
one mutation, ran `cargo test -p torrust-tracker-axum-rest-api-server --lib testing::environment`,
read the `test result:` line to confirm the mutation compiled, and restored the
file. After the last mutation, `git diff --quiet HEAD -- packages/axum-rest-api-server/src/testing/environment.rs`
exited 0. The mutations ran on the commit named above, between 20:21:40 and
20:22:33 UTC. An earlier pass, on the same production code before the
`stop()` panic test was added and before the commit, produced the same failures
for the mutations it ran.

The eight environment tests: `s1` = `it_should_stop_without_a_failure_when_the_server_and_its_drain_controller_finish_through_cancellation`,
`s2` = `it_should_release_the_rest_api_binding_when_stopped`,
`s3` = `it_should_release_the_rest_api_binding_when_dropped_without_being_stopped`,
`s4` = `it_should_serve_requests_after_being_stopped_and_started_again`,
`s5` = `it_should_join_the_drain_controller_before_reporting_a_server_task_failure`,
`s6` = `it_should_report_a_server_error_and_a_drain_timeout_together`,
`s7` = `it_should_report_a_drain_timeout_even_when_both_tasks_join`,
`s8` = `it_should_panic_naming_each_failing_task_when_stopped_with_failed_tasks`.

| Mutation | Behaviour removed                                                       | Compiled | Tests that failed |
| -------- | ----------------------------------------------------------------------- | -------- | ----------------- |
| A        | `stop()` does not cancel the token                                      | yes      | s1 (10 s test deadline), s2, s4 |
| B        | `stop()` drops both task handles instead of joining them                | yes      | s2 (`AddrInUse`), s8 |
| C        | the cancelled token is reused by the next start                         | yes      | s4 (connection reset) |
| D        | the `DropGuard` guards an unrelated token                               | yes      | s1, s2, s3, s4 (also breaks `stop()`) |
| H        | `Running` holds a plain `CancellationToken`, not a `DropGuard`; `stop()` still cancels | yes | s3 only (10 s deadline) |
| E        | `join_owned_tasks` joins sequentially and returns on a server failure   | yes      | s5, s6, s8 |
| F        | a drain `TimedOut` is ignored                                           | yes      | s6, s7, s8 |
| G        | `stop()` swallows the failures instead of panicking                     | yes      | s8 only |

Mutation D breaks `stop()` too, so it does not isolate the drop clause; H does
(the realistic bug of #2471: dropping a `CancellationToken` does not cancel
it), and only the drop test fails under H. Every test is failed by at least one
mutation: s1 by A and D, s2 by A, B and D, s3 by D and H, s4 by A, C and D, s5
by E, s6 by E and F, s7 by F, s8 by B, E, F and G.

The full output of the matrix run is not stored; the table is the record. The
failing-test names above are copied from the runner's output.

## Failures and Follow-up

- The first run of mutation H did not compile: the `stop()` panic test builds a
  `Running` with a `DropGuard`, so the mutation had to change that line too. A
  mutation that does not compile runs no test, so the result was discarded and
  H was rerun after fixing the mutation (V3 reports the rerun, `compiled=True`).
- A first draft of the matrix runner corrupted its multi-line mutation strings
  through an indentation hack; it aborted with an assertion before running
  mutation B, and the runner was rewritten.
- Pre-push checks: see `agent-review-reports.md` and the progress log for the
  run after the review.
