---
doc-type: issue
issue-type: enhancement
status: open
priority: p3
github-issue: 1419
spec-path: docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
branch: 1419-allow-multiple-integration-tests
related-pr: null
last-updated-utc: 2026-07-28 11:54
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md
    - docs/issues/open/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
    - docs/issues/open/2036-add-runtime-service-registry-metadata/ISSUE.md
    - tests/stats.rs
    - tests/servers/
    - src/app.rs
    - packages/test-helpers/
---

# Issue #1419 - Allow multiple integration tests at the main app level

## Goal

Enable running multiple independent integration tests at the main application level (`tests/stats.rs`)
in parallel without port conflicts, configuration collisions, or logging initialization errors.

## Background

Currently, there is one integration test for global metrics at the main app level
(`tests/servers/api/contract/stats/mod.rs`). This test verifies behavior that can only be tested at
the main app level, specifically that multiple tracker instances (HTTP and UDP) running on different
socket addresses contribute to global metrics aggregation.

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

**Strategy**: First prove the scaffolding is broken by adding a second test case that would conflict,
then fix the scaffolding infrastructure, then expand coverage.

| ID  | Status | Task                                                        | Notes                                                                                                                                        |
| --- | ------ | ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Create `tests/AGENTS.md` with guidelines and TODO list      | Document what belongs in main-level integration tests vs package tests; include prioritized TODO list of future valuable integration tests   |
| T2  | TODO   | Add second assertion to existing stats test                 | Check another global stat field (e.g., `tcp4_scrapes_handled` or `tcp6_announces_handled`) to prove need for parallel test capability        |
| T3  | TODO   | Create test utilities module                                | `tests/helpers.rs` with utilities for temp workspace creation (config + storage dirs) and port extraction                                    |
| T4  | TODO   | Add utility to create isolated temp workspace               | Returns `TempDir` with subdirectories for config and storage; writes TOML config; sets `TORRUST_TRACKER_CONFIG_TOML_PATH` env var            |
| T5  | TODO   | Add utility to extract bound addresses from `AppContainer`  | Query `Registar` or job handles to get actual bound `SocketAddr` for HTTP API, trackers, etc.                                                |
| T6  | TODO   | Update existing stats test to use port 0 and temp workspace | Replace `env::set_var` with isolated temp workspace, use port 0, extract actual ports for requests; fixes scaffolding to support parallelism |
| T7  | TODO   | Expand global stats test coverage                           | Add tests for more global stat metrics now that scaffolding supports it (scrape counters, different IP families, etc.)                       |
| T8  | TODO   | Run automatic verification                                  | `cargo test --test stats` must pass with all tests running concurrently                                                                      |

## Progress Tracking

### Workflow Checkpoints

- [ ] Specification drafted and approved by user/maintainer
- [ ] GitHub issue #1419 already exists (created by maintainer)
- [ ] Implementation completed
- [ ] Automatic verification completed (`cargo test --test stats`)
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

## Acceptance Criteria

- [x] AC1: `tests/AGENTS.md` exists and documents guidelines for what belongs at main-level vs
      package-level, with a TODO list of future valuable integration tests.
- [ ] AC2: Multiple integration tests can run concurrently with `cargo test --test stats`
      without port conflicts.
- [ ] AC3: Each test uses an isolated temporary workspace with separate config and storage
      directories (no shared environment variables or storage paths).
- [ ] AC4: Tests using port 0 can extract the actual bound ports from `AppContainer` to construct
      request URLs.
- [ ] AC5: The existing stats integration test is updated to use an isolated temp workspace and
      port 0.
- [ ] AC6: Global stats test coverage includes multiple metrics (not just `tcp4_announces_handled`).
- [ ] AC7: Test utilities for temp workspace creation (config + storage) and port extraction are
      available and documented.
- [ ] AC8: `cargo test --test stats` passes cleanly with expanded test coverage.
- [ ] AC9: `linter all` passes.

## Verification Plan

### Automatic Checks

- `cargo test --test stats` — Must pass with all integration tests running concurrently
- `cargo test --test stats -- --test-threads=1` — Verify tests also work in serial mode
- `linter all` — Standard quality gate

### Manual Verification Scenarios

| ID  | Scenario                                                                | Expected Result                                                                   | Status | Evidence |
| --- | ----------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Run `cargo test --test stats` with default parallelism                  | All tests pass; no port conflicts or configuration collisions                     | TODO   |          |
| M2  | Run `cargo test --test stats -- --nocapture` to see logs                | Each test shows unique bound ports; no env var configuration warnings             | TODO   |          |
| M3  | Add a third integration test temporarily and run the suite              | All three tests run concurrently without interference                             | TODO   |          |
| M4  | Verify temp workspaces are cleaned up after tests complete              | No leftover temporary directories in `/tmp` or system temp directory              | TODO   |          |
| M5  | Run tests in serial mode: `cargo test --test stats -- --test-threads=1` | Tests pass in serial mode (no implicit dependency on parallelism for correctness) | TODO   |          |

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

The current integration suite correctly verifies aggregate statistics across two started HTTP
listeners. Its endpoint discovery is intentionally temporary: `tests/common/mod.rs` identifies
HTTP trackers and the REST API through distinct bind-IP conventions. If discovery is incorrect,
the test fails rather than producing a false aggregate-stats success, but the convention is not a
valid application contract and must not be extended to more integration suites.

During implementation, two prerequisite defects were discovered. Work on this issue stops after
the current, working scaffold is documented and merged. The prerequisites will be implemented and
merged independently; #1419 remains open and resumes on that clean base.

1. Bug #2035: [fix duplicate port-zero tracker instance bootstrap](../../open/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md)
   — `AppContainer` stores HTTP and UDP per-instance containers in `HashMap<SocketAddr, _>`.
   Repeated `0.0.0.0:0` configuration blocks overwrite each other before startup, so distinct
   per-instance configuration can be silently lost.
2. Feature #2036: [add runtime service registry metadata](../../open/2036-add-runtime-service-registry-metadata/ISSUE.md)
   — `Registar` cannot expose stable service role or configuration-instance identity without
   health-check side effects. This requires a coordinated `torrust-server-lib` change and release.

The runtime registry boundary remains recorded in
[ADR 20260728115400](../../../adrs/20260728115400_define_registar_as_runtime_service_registry.md).
The dedicated prerequisite specifications above are the implementation records for that boundary.
