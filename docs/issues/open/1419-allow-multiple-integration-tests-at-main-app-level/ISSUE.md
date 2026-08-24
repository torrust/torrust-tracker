---
doc-type: issue
issue-type: enhancement
status: open
priority: p3
github-issue: 1419
spec-path: docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
branch: 1419-allow-multiple-integration-tests
related-pr: null
last-updated-utc: 2026-08-24
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md
    - docs/issues/open/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
    - docs/issues/closed/2036-add-runtime-service-registry-metadata/ISSUE.md
    - tests/AGENTS.md
    - tests/common/
    - tests/metrics/
    - tests/banning/
    - tests/scaffold.rs
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
    - packages/test-helpers/
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/completion-plan.md
    - https://github.com/torrust/torrust-tracker/issues/1488
    - https://github.com/torrust/torrust-tracker/pull/1993
---

# Issue #1419 - Allow multiple integration tests at the main app level

## Goal

Enable independent main-application integration-test executables to run in parallel without port,
configuration, storage, or process-global-state collisions. Each executable owns one tracker
application instance and runs its scenarios sequentially.

## Background

The current test structure contains dedicated Cargo integration-test targets for port-zero metrics,
fixed-port metrics, UDP error-policy behavior, UDP banning behavior, and a scaffolding example.
They verify application-level behavior such as multiple HTTP/UDP listener coordination and global
metrics aggregation. The former `tests/stats.rs` and `tests/servers/api/contract/stats/mod.rs`
locations no longer exist.

Most tests are correctly located inside the `packages/` directory, testing individual components in
isolation. Integration tests at the main app level should be reserved for testing application-level
concerns:

- Multiple tracker instances running simultaneously
- Global metrics aggregation across services
- Application container initialization and lifecycle
- Job manager orchestration
- Cross-service coordination
- Bootstrap and configuration integration

Integration tests at this level offer advantages over E2E tests:

- **Faster execution**: No docker container overhead
- **Flexible context**: Easy to modify configuration per test
- **Portable**: Run anywhere (including inside docker image builds)
- **Better debugging**: Direct access to application state

### Current Problems

When attempting to add a second integration test, three problems arise:

#### ~~Problem 1: Logging initialization fails with multiple tests~~ [RESOLVED]

**Update**: This issue can no longer be reproduced. Initial investigation showed that calling
`app::run()` with `logging.threshold = "info"` in multiple tests would fail with:

```text
Unable to install global subscriber: SetGlobalDefaultError("a global default trace dispatcher has already been set")
```

However, testing with two concurrent tests using identical configuration (including
`logging.threshold = "info"`) now runs cleanly. The logger appears to handle reinitialization
gracefully, likely due to internal guards in the tracing infrastructure.

This problem is considered resolved and requires no further action.

#### Problem 2: Port conflicts when tests run in parallel

Tests run concurrently by default (`cargo test` uses multiple threads). If multiple tests use the
same hard-coded ports, they fail with:

```text
Could not bind tcp_listener to address.: Os { code: 98, kind: AddrInUse, message: "Address already in use" }
```

The current test uses fixed ports:

```toml
[[http_trackers]]
bind_address = "0.0.0.0:7272"

[[http_trackers]]
bind_address = "0.0.0.0:7373"

[http_api]
bind_address = "0.0.0.0:1414"
```

**Solution**: Use port `0` for all bind addresses. The OS assigns a free ephemeral port, eliminating
conflicts:

```toml
[[http_trackers]]
bind_address = "0.0.0.0:0"

[http_api]
bind_address = "0.0.0.0:0"
```

After binding, the actual assigned ports must be retrieved from the running services to construct
request URLs for test assertions. The application already provides access to bound addresses through
the `Registar` component in `AppContainer`.

#### Problem 3: Environment variable configuration conflicts and storage isolation

Tests run in parallel within the same process share the same environment. If tests inject
configuration via `std::env::set_var("TORRUST_TRACKER_CONFIG_TOML", ...)`, concurrent tests
overwrite each other's configuration, causing non-deterministic failures.

Additionally, trackers need isolated storage directories for their databases and runtime state.
Using a shared `storage/` directory or relying on default paths causes conflicts when multiple
trackers run concurrently.

The current test uses `unsafe { env::set_var(...) }` with a safety comment acknowledging this
limitation.

**Note**: The E2E runner ([`src/console/ci/e2e/runner.rs`](../../../src/console/ci/e2e/runner.rs))
demonstrates a pattern where CLI arguments (`--config-toml-path`, `--config-toml`) map to these
same environment variables (`TORRUST_TRACKER_CONFIG_TOML_PATH`, `TORRUST_TRACKER_CONFIG_TOML`).
However, the main tracker binary ([`src/main.rs`](../../../src/main.rs)) does not currently
accept CLI arguments - it only reads configuration from environment variables. Adding CLI argument
support to the main binary would be a future improvement, but is out of scope for this issue.

**Solution**: Use temporary directories (not just temp files) for complete test isolation:

1. Create a unique temporary directory per test using `tempfile::TempDir`
2. Within the temp directory, create subdirectories for:
   - Config file (e.g., `tracker-config.toml`)
   - Storage directory (e.g., `tracker-storage/` for database and runtime data)
3. Configure the tracker to use these isolated paths
4. Set `TORRUST_TRACKER_CONFIG_TOML_PATH` to point to the temp config file
5. The entire temp directory and its contents are automatically cleaned up when the `TempDir`
   handle is dropped

This pattern matches the approach used in qBittorrent E2E tests
([`src/console/ci/qbittorrent_e2e/filesystem_setup.rs`](../../../src/console/ci/qbittorrent_e2e/filesystem_setup.rs)),
which creates isolated workspaces with separate config and storage directories for each test run.

### Related work

- E2E tests ([`src/console/ci/e2e/runner.rs`](../../../src/console/ci/e2e/runner.rs)) parse tracker
  output to extract bound ports, but they run the tracker as an external process
- qBittorrent E2E tests
  ([`src/console/ci/qbittorrent_e2e/filesystem_setup.rs`](../../../src/console/ci/qbittorrent_e2e/filesystem_setup.rs))
  create isolated temporary workspaces with subdirectories for config, storage, and shared fixtures
  using `tempfile::TempDir`
- Package-level tests already use similar patterns (port 0, temp files) in various
  `testing/environment.rs` modules

## Scope

### In Scope

- Enable multiple integration tests to run concurrently without port conflicts
- Provide test utilities for managing temporary test workspaces (config + storage directories)
- Extract bound port information from `AppContainer` or `JobManager` for test assertions
- Update existing integration test to use port 0 and isolated temp workspace
- Expand global stats test coverage to verify multiple metrics
- Document patterns for writing integration tests at the main app level
- Create `tests/AGENTS.md` with guidelines for AI agents and TODO list of future integration tests

### Out of Scope

- Changing E2E test infrastructure
- Modifying package-level test infrastructure
- Changing logging infrastructure or tracing initialization
- Adding extensive integration test coverage (focus is on infrastructure, not coverage)
- Modifying `Registar` API (use existing capabilities only)

## Implementation Plan

**Status**: The following table is the historical implementation plan. Its original single-
`stats`-executable premise was superseded by the per-executable execution model below. The actual
completed work and remaining tasks are recorded after the decision pivot.

| ID  | Status | Task                                                          | Notes                                                                                                   |
| --- | ------ | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Create `tests/AGENTS.md` with guidelines and TODO list        | `tests/AGENTS.md` documents scope, execution model, and test layout.                                    |
| T2  | DONE   | Create independent integration-test executables               | `Cargo.toml` explicitly registers the current nested test targets.                                      |
| T3  | DONE   | Create test utilities module                                  | Reusable utilities live in `tests/common/`, not the obsolete `tests/helpers.rs` proposal.               |
| T4  | DONE   | Add utility to create isolated temp workspace                 | `EphemeralTrackerWorkspace` creates a `TempDir`, config file, and storage directory.                    |
| T5  | DONE   | Add utility to extract bound addresses from `AppContainer`    | Runtime registry helpers use service role and `ConfigurationInstanceId`.                                |
| T6  | DONE   | Migrate appropriate suites to port 0 and temporary workspaces | Port-zero metrics, UDP-error, and banning suites use isolated workspaces.                               |
| T7  | DONE   | Expand global stats test coverage                             | Current suites cover HTTP/UDP announces plus request, connection, response, error, and banning metrics. |
| T8  | DONE   | Resolve prerequisites for port-zero identity discovery        | #2035 bootstrap behavior and #2036 registry metadata are present and consumed by helpers.               |
| T9  | TODO   | Run automatic verification                                    | Use the current multi-target command in the revised verification plan.                                  |

## Progress Tracking

### Workflow Checkpoints

- [ ] Specification drafted and approved by user/maintainer
- [ ] GitHub issue #1419 already exists (created by maintainer)
- [x] Implementation completed (partial improvement; cooperative server shutdown remains deferred)
- [x] Automatic verification completed (current main-level integration-test targets)
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and specification moved to `docs/issues/closed/`

### Progress Log

- 2025-03-27 16:40 UTC - josecelano - Created GitHub issue #1419
- 2025-04-04 10:04 UTC - josecelano - Added comment noting Problem 1 can no longer be reproduced
- 2026-07-27 08:00 UTC - agent - Drafted issue specification
- 2026-07-27 08:37 UTC - agent - Created `tests/AGENTS.md` with guidelines and TODO list
- 2026-07-27 08:38 UTC - agent - Updated implementation plan to use prove-then-fix strategy
- 2026-07-27 13:00 UTC - agent - Updated Problem 3 and implementation plan to use temp directory
  pattern (not just temp files) for complete test isolation, matching qBittorrent E2E approach
- 2026-07-27 17:35 UTC - agent - Recorded the decision to use one tracker application per Cargo
  integration-test executable, with sequential scenarios per suite and a non-Docker process runner
  deferred unless in-process lifecycle control proves insufficient
- 2026-08-24 - agent - Reconciled the specification with the current `tests/` implementation and
  replaced the remaining-work list with an ordered completion plan: deterministic teardown,
  teardown coverage, documentation alignment, focused verification, quality gate, and closure review.
- 2026-08-24 - agent - Added `completion-plan.md` as the decision record for the non-trivial
  teardown work. It documents the lifecycle problem, rejected alternatives, selected test-local
  fixture direction, and mandatory manual verification evidence.
- 2026-08-24 - agent - Implemented an initial `TrackerApplicationFixture` and migrated the current
  suites. Focused tests exposed a lifecycle gap: `JobManager::cancel()` reaches event-listener jobs,
  but tracker server jobs wait on separate halt channels and run until their per-job wait timeout.
  Production shutdown coordination is deferred to #1488; #1419 will retain the fixture and use the
  best shutdown sequence currently exposed by production.
- 2026-08-24 - agent - Completed the partial-delivery manual verification: all six current
  main-level targets passed in one invocation; `metrics-port-zero` passed with `--nocapture` and
  with `--test-threads=1`; and the full pre-commit quality gate passed. Each suite took 60–81
  seconds because current server jobs can consume `wait_for_all`'s per-job timeout. Successful
  process exit is not evidence of cooperative server shutdown; that proof remains deferred to
  #1488.

## Acceptance Criteria

- [x] AC1: `tests/AGENTS.md` exists and documents guidelines for what belongs at main-level vs
      package-level, with a TODO list of future valuable integration tests.
- [x] AC2: Independent main-level integration-test executables are registered and can run
      concurrently as separate Cargo processes without shared environment state.
- [x] AC3: Current port-zero suites use an isolated temporary workspace with separate config and storage
      directories (no shared environment variables or storage paths).
- [x] AC4: Tests using port 0 can extract the actual bound ports from `AppContainer` to construct
      request URLs.
- [x] AC5: Port-zero suites use an isolated temp workspace and port 0 where the scenario does not
      require fixed ports.
- [x] AC6: Global stats coverage includes multiple metrics, not only `tcp4_announces_handled`.
- [x] AC7: Test utilities for temp workspace creation (config + storage) and port extraction are
      available and documented.
- [x] AC8: Every current main-level suite uses `TrackerApplicationFixture` to invoke the best
      currently exposed production shutdown sequence—`JobManager::cancel()` followed by
      `wait_for_all(...)`—before its temporary workspace is released.
- [ ] AC8a: After shutdown-overhaul #1488 is implemented, review the integration fixture against
      the new production lifecycle and prove server jobs finish cooperatively without consuming the
      current per-job timeout.
- [x] AC9: All current main-level integration-test targets pass with normal parallel scheduling,
      and a representative target passes in serial mode. Current production server jobs can still
      consume the per-job shutdown timeout; cooperative completion is deferred to AC8a.
- [x] AC10: `linter all` passes.

## Verification Plan

### Automatic Checks

- `cargo test --test metrics-fixed-ports --test metrics-port-zero --test metrics-udp-error-enabled-port-zero --test metrics-udp-error-disabled-port-zero --test banning-udp-metrics-disabled-port-zero --test scaffold` — Must pass with normal parallel scheduling after deterministic teardown is implemented
- `cargo test --test metrics-port-zero -- --test-threads=1` — Verify a representative suite also works in serial mode after deterministic teardown is implemented
- `linter all` — Standard quality gate

### Manual Verification Scenarios

| ID  | Scenario                                   | Expected Result                                                                           | Status | Evidence                                                                                                                                                                  |
| --- | ------------------------------------------ | ----------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M1  | Run all current targets in one invocation  | All targets pass; no port, configuration, storage, or shutdown interference               | DONE   | 2026-08-24; exit 0; all six targets passed. Each took 60–81 seconds because server jobs can consume the current per-job wait timeout.                                     |
| M2  | Run `metrics-port-zero` with `--nocapture` | Explicit teardown completes; services have distinct, non-zero final bindings              | DONE   | 2026-08-24; exit 0; 1 test passed in 80.07 seconds. The scenario validates distinct non-zero final bindings; duration reflects the current server-job timeout limitation. |
| M3  | Verify focused teardown coverage           | Awaited shutdown completes before workspace cleanup; no process-exit or sleep-based proof | DONE   | 2026-08-24; exit 0; `it_should_apply_metrics_policy_to_port_zero_tracker_instances` calls fixture shutdown then asserts the workspace has been released.                  |
| M4  | Run `metrics-port-zero` in serial mode     | Suite has no implicit dependency on parallel scheduling                                   | DONE   | 2026-08-24; exit 0; 1 test passed with `--test-threads=1` in 80.07 seconds.                                                                                               |
| M5  | Run `linter all` after implementation      | Full quality gate passes                                                                  | DONE   | 2026-08-24; exit 0; pre-commit gate passed `linter all` plus cargo machete, cargo deny, Containerfile lint, and documentation tests.                                      |

### Verification Evidence

All commands below completed with exit status `0` on 2026-08-24:

```sh
cargo test --test metrics-fixed-ports --test metrics-port-zero --test metrics-udp-error-enabled-port-zero --test metrics-udp-error-disabled-port-zero --test banning-udp-metrics-disabled-port-zero --test scaffold
cargo test --test metrics-port-zero -- --nocapture
cargo test --test metrics-port-zero -- --test-threads=1
TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json
linter all
```

The multi-target invocation passed all six suites. `metrics-port-zero` passed in both focused runs.
The focused lifecycle assertion confirms that fixture shutdown returns before the temporary workspace
is released. The test executables took 60–81 seconds because current HTTP, UDP, REST API, and health
check server jobs can consume `JobManager::wait_for_all`'s per-job timeout. This is a known current
production limitation, not proof of cooperative server completion; #1488 owns that follow-up.

## Risks and Trade-offs

- **Temp workspace cleanup**: If tests panic or are interrupted, temporary directories may not be
  cleaned up. This is standard behavior for integration tests and acceptable.
- **Port 0 complexity**: Tests must extract actual bound ports from the application, adding a layer
  of indirection. This is necessary for parallel execution and mirrors real-world deployment scenarios.
- **Scope creep risk**: It's tempting to add many integration tests at this level. Maintain
  discipline: most tests belong in `packages/`, only application-level concerns should be tested here.
- **Registar API surface**: If `Registar` doesn't expose bound addresses in a convenient form,
  alternative extraction methods (e.g., parsing job handles) may be needed. Investigate existing
  capabilities first.

## Notes

- The issue description mentions that the `Registar` type now includes "listen url" information,
  making it easier to extract bound addresses. Confirm this during implementation.
- The existing test has a safety comment about `std::env::set_var` being unsafe in Rust 2024 due to
  concurrent access. The temp file approach eliminates this concern entirely.
- Consider whether test utilities should live in `tests/helpers.rs` or be added to the existing
  `packages/test-helpers/` package. Decision: prefer `tests/helpers.rs` to keep test-specific
  utilities close to the tests and avoid polluting the shared `test-helpers` package.

## Decision Pivot: One Application per Integration-Test Binary

**Decision date:** 2026-07-27

The preceding specification describes the initial proposal: multiple independent test functions in
`tests/integration.rs`, each bootstrapping a tracker application and running concurrently. That
proposal is superseded by this section. The historical content remains above to preserve the
reasoning and investigation that led to the decision.

### Why the Initial Proposal Is Unsuitable

Calling `app::run()` from an integration test starts the application inside the integration-test
executable, not in an independent tracker process. Application bootstrap initializes process-wide
state through `initialize_global_services`, including clock state, UDP cryptographic state, and
logging. The application also starts a collection of long-lived servers and background jobs.

Consequently, a test function cannot safely own a fully isolated tracker lifecycle in a shared
test process. Temporary workspaces and port `0` solve filesystem and listener conflicts, but they
do not isolate process-global state, environment-variable configuration, or background task
lifecycle. Supporting multiple complete application instances per test executable would require a
larger application lifecycle redesign and is out of scope for this issue.

### Chosen Execution Model

Each top-level Rust source file in `tests/` is a separate Cargo integration-test executable and
therefore a separate operating-system process. The project will use one tracker configuration and
one tracker application instance per such executable.

Within an integration-test executable, a single suite test will:

1. Create an isolated temporary workspace containing the tracker configuration and storage.
2. Start one tracker application with the suite's fixed initial configuration.
3. Execute its scenario functions sequentially against that running application.
4. Shut down the application and wait for its jobs before releasing the temporary workspace.

Scenario functions are not independently scheduled `#[tokio::test]` functions. They share the
suite's runtime and data lifecycle, so they must use distinct test data or make assertions that
explicitly account for accumulated state.

The existing global-statistics scenarios belong in the same suite because they require the same
public-tracker configuration. A concern requiring a different initial configuration or process
lifecycle will be placed in another top-level file, such as `tests/bootstrap.rs`. Cargo may run
such executables concurrently; each suite must therefore still use a unique `TempDir` workspace,
its own database and storage paths, and port `0` for listeners.

`TORRUST_TRACKER_CONFIG_TOML_PATH` remains process-local under this model, so configuration
injection through the environment is safe between separate test executables. It must not be
modified concurrently by separate scenarios in one executable.

### Current Implementation Status

The revised execution model is implemented by the explicit targets in `Cargo.toml`:

- `metrics-port-zero` tests duplicate port-zero listener identity and metrics policy.
- `metrics-fixed-ports` tests fixed-port multi-listener routing and aggregate metrics.
- `metrics-udp-error-enabled-port-zero`, `metrics-udp-error-disabled-port-zero`, and
  `banning-udp-metrics-disabled-port-zero` test the related UDP policy variants.
- `scaffold` documents the pattern for a new isolated suite.

`tests/common/workspace.rs` provides the shared `EphemeralTrackerWorkspace`, startup readiness,
and side-effect-free runtime-registry discovery used by these suites. It creates a unique `TempDir`
containing a configuration file and tracker storage directory, while port-zero services publish
their final bindings under canonical service roles and `ConfigurationInstanceId` values.

`TrackerApplicationFixture` now owns the workspace and `JobManager`, and explicitly calls
`cancel()` then `wait_for_all(...)` before releasing its workspace. Focused execution showed that
the current production shutdown path does not yet propagate cancellation to server-specific halt
channels, so server jobs can reach `wait_for_all`'s per-job timeout. That production concern is
owned by [shutdown-overhaul #1488](https://github.com/torrust/torrust-tracker/issues/1488), whose
draft planning PR [#1993](https://github.com/torrust/torrust-tracker/pull/1993) selects token
watching in each server job starter rather than a separate coordinator. #1419 deliberately does
not implement a competing shutdown mechanism.

The former pause for #2035 and #2036 is no longer active: repeated `0.0.0.0:0` HTTP and UDP
configuration blocks retain distinct instance identities, and the runtime registry metadata needed
for stable endpoint discovery is available. The implementation is now ready for teardown work and
verification.

### Completion Plan

The remaining work is deliberately limited to lifecycle correctness, documentation alignment, and
verification. Do not add new application-level behavior or a child-process runner as part of this
issue. The problem statement, alternatives, selected fixture direction, and mandatory manual test
protocol are documented in [completion-plan.md](completion-plan.md). That companion document must
be reviewed before implementation begins.

| ID  | Status | Step                                          | Implementation and completion evidence                                                                                                                                                                                    |
| --- | ------ | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R1  | DONE   | Review lifecycle problem and fixture decision | The test-local fixture direction was selected. It centralizes the best shutdown sequence that current production exposes.                                                                                                 |
| R2  | DONE   | Implement partial deterministic teardown      | `TrackerApplicationFixture` owns the application workspace and invokes `cancel()` then `wait_for_all(...)` before workspace release; all current suites use it.                                                           |
| R3  | DONE   | Prove current fixture ordering                | Focused coverage proves awaited fixture shutdown precedes workspace cleanup without process-exit or sleep-based assertions.                                                                                               |
| R4  | DONE   | Align test documentation                      | Update `tests/scaffold.rs` references to the removed `stats` target and revise `tests/AGENTS.md` to document the fixture and process-global lifecycle constraints.                                                        |
| R5  | DONE   | Run mandatory manual integration verification | On 2026-08-24, exit 0: all six targets passed together; `metrics-port-zero` passed with `--nocapture` and in serial mode. Each suite took 60–81 seconds because server jobs can consume the current per-job wait timeout. |
| R6  | DONE   | Run the full quality gate                     | On 2026-08-24, exit 0: the pre-commit gate passed, including `linter all`.                                                                                                                                                |
| R7  | TODO   | Open partial-improvement PR                   | Submit the fixture, suite migration, focused ordering coverage, and documentation. State that cooperative server shutdown remains owned by #1488.                                                                         |
| R8  | TODO   | Revisit after shutdown overhaul #1488         | After #1488's production shutdown work merges, review this fixture against its finalized API, update it if needed, and complete AC8a. Keep #1419 open until that review is recorded.                                      |
| R9  | TODO   | Perform final closure review                  | After the #1488 follow-up, confirm all acceptance criteria, move the issue specification to `docs/issues/closed/`, and close GitHub issue #1419.                                                                          |

#### Implementation Sequence

1. Complete and record the partial-delivery manual verification of the fixture, noting that the
   current production server jobs may consume their wait timeout.
2. Run the full quality gate and submit the partial-improvement PR. Do not add server shutdown
   coordination in #1419.
3. After #1488 merges, review its finalized shutdown API and update the fixture to call the same
   application lifecycle boundary.
4. Add and run post-overhaul focused coverage that proves cooperative server completion before
   workspace cleanup, without fixed sleeps.
5. Repeat every mandatory manual run in [completion-plan.md](completion-plan.md), including the
   normal parallel, `--nocapture`, and representative serial commands, then run `linter all`.
6. Update the verification evidence and acceptance criteria. Only then prepare the issue for
   closure.

#### Completion Boundaries

- The shared fixture may remain inside `tests/common/`; do not move it to `packages/test-helpers/`
  unless a second, non-main-level consumer establishes a real shared need.
- A new integration-test binary is not required merely to prove teardown. Prefer focused coverage
  in the existing test structure.
- Do not implement a server-halt coordinator, pass cancellation tokens through server packages, or
  otherwise change production shutdown in this issue. Those responsibilities are explicitly owned
  by shutdown-overhaul #1488 and its implementation subissues.
- Do not claim temporary directories are cleaned after a forced process interruption; normal
  `TempDir` cleanup is sufficient once jobs stop before the workspace is dropped.
- Do not close the issue until the normal parallel invocation and `linter all` have both passed.

### Relationship to E2E Tests

This is not a replacement for container E2E tests. Main-application integration suites provide
faster application-composition coverage without Docker, while E2E tests continue to validate
container images, mounts, network setup, and external-client workflows.

### Deferred Alternative: Non-Docker Process Runner

If an integration suite needs to verify the real tracker executable's startup, signal handling,
logging, or exit behavior, or if reliable in-process shutdown cannot be implemented, introduce a
non-Docker process runner. That runner would launch the built tracker binary as a child process
with an isolated workspace, wait for readiness, capture diagnostics, and terminate it cleanly.

Do not create a new package solely to obtain separate test executables: Cargo already provides
that isolation through separate top-level files in `tests/`. A dedicated package becomes justified
only when the child-process runner is reusable enough to warrant its own lifecycle, readiness,
diagnostic, and cleanup abstractions.

### Superseded Scope, Plan, Acceptance Criteria, and Verification

The original scope, implementation plan, acceptance criteria, and verification plan above are
superseded where they require parallel tracker application instances or multiple independently
bootstrapped test functions in `tests/integration.rs`.

This issue now covers the following:

- Convert the existing global-statistics integration test into one sequential suite using one
  public-tracker application instance.
- Provide test-local helpers for an isolated temporary workspace, tracker startup, readiness, and
  deterministic shutdown.
- Use port `0` and discover resolved listener addresses for suite requests.
- Add further global-statistics scenarios only when they share the suite's initial configuration.
- Establish the convention that a different initial tracker configuration belongs to another
  top-level integration-test executable.

Verification must show that the suite starts one application, runs all its scenarios sequentially,
shuts it down cleanly, and leaves no shared database, storage, or port dependency. Cross-suite
parallel execution is supported through process isolation, but it is not a requirement for
parallel full-application instances inside a single executable.

## Implementation Pause and Prerequisites

The current integration suites verify aggregate statistics across multiple started HTTP and UDP
listeners. Endpoint discovery is no longer temporary: `tests/common/workspace.rs` queries the
runtime registry by canonical service role and exact `ConfigurationInstanceId`, rather than bind-IP
conventions or registry ordering.

During implementation, two prerequisite defects were discovered. Both prerequisites are now
implemented, so this issue resumes with deterministic teardown, documentation cleanup, and
verification.

1. Bug #2035: [fix duplicate port-zero tracker instance bootstrap](../../open/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md)
   — `AppContainer` now retains HTTP and UDP per-instance containers with their
   `ConfigurationInstanceId`, preventing repeated `0.0.0.0:0` configuration blocks from
   overwriting each other before startup.
2. Feature #2036: [add runtime service registry metadata](../../closed/2036-add-runtime-service-registry-metadata/ISSUE.md)
   — `Registar` metadata now exposes stable service role and configuration-instance identity for
   side-effect-free test endpoint discovery.

The runtime registry boundary remains recorded in
[ADR 20260728115400](../../../adrs/20260728115400_define_registar_as_runtime_service_registry.md).
The dedicated prerequisite specifications above are the implementation records for that boundary.
