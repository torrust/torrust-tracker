---
schema-version: 1
doc-type: issue
issue-type: enhancement
status: in-progress
priority: p3
epic: 2392
github-issue: 1419
spec-path: docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
branch: "1419-allow-multiple-integration-tests"
related-pr: 2189
last-updated-utc: "2026-11-19 16:42"
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md
    - docs/issues/closed/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
    - docs/issues/closed/2036-add-runtime-service-registry-metadata/ISSUE.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - tests/AGENTS.md
    - tests/common
    - tests/metrics
    - tests/banning
    - tests/scaffold.rs
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
    - packages/test-helpers
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/completion-plan.md
    - "issue #1488"
    - docs/issues/closed/1488-overhaul-tracker-shutdown/EPIC.md
    - docs/features/shutdown-process/README.md
    - tests/common/workspace.rs
---

# Issue #1419 - Allow multiple integration tests at the main app level

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

> **Current state (2026-11-19):** the issue is no longer blocked. The shutdown overhaul, EPIC #1488,
> is closed (its spec is in `docs/issues/closed/1488-overhaul-tracker-shutdown/EPIC.md`), and the
> maintainer decided on 2026-11-18 that #1419 continues under EPIC #2392. The remaining work is R8
> (with AC8a) and R9. The plan for it is in
> [R8 plan: align the fixture with the final shutdown API](#r8-plan-align-the-fixture-with-the-final-shutdown-api).
> Text below that says the work "waits on #1488" or is "deferred to #1488" is the historical record
> of the partial delivery; that plan section replaces it.

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
`app::start()` with `logging.threshold = "info"` in multiple tests would fail with:

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

The original fixture used `unsafe { env::set_var(...) }` with a safety comment acknowledging this
limitation. It is retained here as the historical problem that the explicit-path fixture migration resolves.

**Update:** Issue #2151 added `--config-toml-path` to the main tracker
startup boundary. The shared fixture now passes its workspace-local file to
`app::start_with_explicit_config_toml_path`, so it no longer needs to mutate
base-source environment variables. The E2E runner's complete-TOML input remains
a separate container-runner concern.

**Solution**: Use temporary directories (not just temp files) for complete test isolation:

1. Create a unique temporary directory per test using `tempfile::TempDir`
2. Within the temp directory, create subdirectories for:
   - Config file (e.g., `tracker-config.toml`)
   - Storage directory (e.g., `tracker-storage/` for database and runtime data)
3. Configure the tracker to use these isolated paths
4. Pass the temp config-file path directly to
   `app::start_with_explicit_config_toml_path`
5. The entire temp directory and its contents are automatically cleaned up when the `TempDir`
   handle is dropped

This pattern matches the approach used in qBittorrent E2E tests
([`src/console/ci/qbittorrent_e2e/filesystem_setup.rs`](../../../../src/console/ci/qbittorrent_e2e/filesystem_setup.rs)),
which creates isolated workspaces with separate config and storage directories for each test run.

### Related work

- E2E tests ([`src/console/ci/e2e/runner.rs`](../../../../src/console/ci/e2e/runner.rs)) parse tracker
  output to extract bound ports, but they run the tracker as an external process
- qBittorrent E2E tests
  ([`src/console/ci/qbittorrent_e2e/filesystem_setup.rs`](../../../../src/console/ci/qbittorrent_e2e/filesystem_setup.rs))
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
- [x] Partial-improvement PR [#2189](https://github.com/torrust/torrust-tracker/pull/2189) merged
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
- 2026-09-09 - GitHub Copilot - Replaced the shared fixture's test-process configuration
  environment mutation with `app::start_with_explicit_config_toml_path` using its existing
  workspace-local file. Removed the environment lock and restoration guard. All eight current
  main-level integration targets passed together; lifecycle completion remains deferred to #1488.
- 2026-09-10 15:45 UTC - GitHub - Merged partial-improvement PR
  [#2189](https://github.com/torrust/torrust-tracker/pull/2189). The PR resolved the
  fixture's base-source environment mutation but did not complete the #1488-dependent
  cooperative-shutdown follow-up.
- 2026-09-10 15:49 UTC - josecelano - Reopened GitHub issue #1419 after it was
  closed accidentally with PR #2189. The remaining R8/AC8a and R9 work still
  requires the shutdown-overhaul #1488 follow-up and final closure review.
- 2026-09-30 14:58 UTC - GitHub Copilot - Linked as a GitHub sub-issue of the new Test Isolation
  EPIC #2392. Scope is unchanged; the remaining AC8a still waits on #1488. Migrated the
  frontmatter to v1 (required by the validator for edited open specs): status `open` became
  `blocked`, directory paths lost trailing slashes, and the #1488 and PR #1993 URLs were replaced
  by an `issue #1488` reference (v1 has no pull-request reference; the body still links #1993).
- 2026-11-19 08:23 UTC - Claude Code (Claude Fable 5.2) - Unblocked the issue: EPIC #1488 is closed and the maintainer
  decided on 2026-11-18 that #1419 continues under EPIC #2392. Status `blocked` became
  `in-progress`. Reviewed `tests/common/workspace.rs` against the final shutdown API and recorded
  the gaps G1 to G3, the decisions D1 to D5 for the maintainer, the planned change and the evidence
  for AC8a in the R8 plan. Measured the ten fixture-based targets with no fixture change: each test
  finished in 0.12 to 2.05 seconds, so the 60 to 81 seconds of 2026-08-24 no longer apply.
- 2026-11-19 16:42 UTC - Claude Code (Claude Fable 5.2) - The maintainer decided D1 to D5 of the R8 plan,
  choosing option (a) for D1, D2, D3 and D5 and option (b) for D4. Recorded each option's
  advantages and disadvantages in the decision record of the R8 plan. Option (c) of D2 stays
  available if the suites become too slow.

## Acceptance Criteria

- [x] AC1: `tests/AGENTS.md` exists and documents guidelines for what belongs at main-level vs
      package-level, with a TODO list of future valuable integration tests.
- [x] AC2: Independent main-level integration-test executables are registered and can run
      concurrently as separate Cargo processes without shared environment state.
- [x] AC3: Current port-zero suites use an isolated temporary workspace with separate config and storage
      directories. The shared fixture selects its workspace file through the explicit-path startup
      API, with no base-source environment-variable mutation or shared storage paths.
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
      current per-job timeout. The evidence that meets it is defined in the
      [R8 plan](#r8-plan-align-the-fixture-with-the-final-shutdown-api) (E1 to E5).
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

Calling `app::start()` from an integration test starts the application inside the integration-test
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

Each fixture passes its workspace-local configuration file directly to
`app::start_with_explicit_config_toml_path`, so main-level test startup does
not mutate base-source environment variables. This removes one source of shared
mutable state; it does not change the one-application-per-executable rule
imposed by remaining process-global lifecycle constraints.

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
server wrappers already forward the manager token to their private `Halted::Normal` channels, but
the current split lifecycle and detached drain controllers can still make server jobs reach
`wait_for_all`'s per-job timeout. That production concern is owned by
[shutdown-overhaul #1488](https://github.com/torrust/torrust-tracker/issues/1488), whose draft
planning PR [#1993](https://github.com/torrust/torrust-tracker/pull/1993) replaces this bridge with
direct token-aware component lifecycle APIs and owner-joined children. #1419 deliberately does not
implement a competing shutdown mechanism.

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
| R6a | DONE   | Remove fixture environment injection          | `TrackerApplicationFixture` now passes its workspace TOML path to `app::start_with_explicit_config_toml_path`, removing its environment lock, restoration guard, and base-source environment mutation.                    |
| R7  | DONE   | Open partial-improvement PR                   | [PR #2189](https://github.com/torrust/torrust-tracker/pull/2189) merged on 2026-09-10. It delivered explicit workspace-file configuration for the shared fixture; cooperative server shutdown remains owned by #1488.     |
| R8  | TODO   | Revisit after shutdown overhaul #1488         | #1488 closed on 2026-11-19. The review is recorded in the [R8 plan](#r8-plan-align-the-fixture-with-the-final-shutdown-api); the maintainer decided D1 to D5 on 2026-11-19 (decision record there); the fixture update and the AC8a evidence follow it.          |
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

### R8 Plan: Align the Fixture with the Final Shutdown API

**Status:** decided on 2026-11-19. The maintainer chose the recommended option for each of D1 to
D5; see the [decision record](#decision-record) below. The code change follows this plan.

#### The final production shutdown API

EPIC #1488 delivered this lifecycle boundary (checked in the code at `torrust/develop` commit
`53916fd2b`):

- `app::begin_shutdown(&app_container, &jobs)` in `src/app.rs` marks the application not ready and
  then calls `JobManager::cancel()`. It is the production entry point of every shutdown.
- `app::run_until_shutdown_signal` waits for `SIGINT` or `SIGTERM`, calls `begin_shutdown`, awaits
  `jobs.wait_for_all(app_container.application_config.shutdown.deadline())`, and maps the outcomes
  to the process exit code. The fixture cannot call it, because it waits for a process signal.
- `JobManager::wait_for_all(deadline)` applies one shared deadline to all components
  (`[application.shutdown].deadline_in_secs`, default 25 seconds). At the deadline it aborts and
  joins what is left. It returns one `JobOutcome` per component; `JobStatus::is_graceful` is true
  only for `Completed` and `Cancelled`, and `incomplete_components(&outcomes)` names the rest
  (`Failed`, `Panicked`, `Aborted`).
- The legacy halt channels, the token-to-halt bridge and the per-job wait timeout no longer exist.

#### Review findings (R8)

Reading `tests/common/workspace.rs` against that API found three gaps:

| ID  | Gap                                                                                                                                                                              | Consequence                                                                                                         |
| --- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| G1  | `shutdown()` and `restart()` call `jobs.cancel()` directly, not `app::begin_shutdown`.                                                                                           | The fixture skips the readiness step, so the suites do not exercise the same shutdown entry point as production.    |
| G2  | Both wait with a fixture constant, `TRACKER_SHUTDOWN_GRACE_PERIOD` (10 seconds), not the configured process deadline.                                                            | The fixture has its own deadline policy that can drift from the configuration the suite started the tracker with.   |
| G3  | Both discard the `Vec<JobOutcome>` that `wait_for_all` returns.                                                                                                                  | A component that fails, panics or is aborted at the deadline does not fail the suite. Nothing proves cooperation.   |

The timing evidence of the partial delivery is also stale. On 2026-08-24 each suite took 60 to 81
seconds because server jobs consumed a per-job wait timeout. On 2026-11-19, with no fixture change,
the E2 command below (the ten fixture-based targets in one invocation) exited with status 0 and each
test finished in 0.12 to 2.05 seconds. Toolchain and host of this run and of the two probes below:
`rustc 1.100.0 (0ef1bc386 2026-11-09)`, `cargo 1.100.0`, the `dev` profile, Linux x86_64 with 32
logical CPUs.

A local probe (not committed) applied the change proposed below to the unchanged suites and ran the
E2 command with `-- --nocapture`. It exited with status 0: all ten targets passed; every component of every suite reported `Cancelled`; and the awaited shutdown took
at most 1.01 seconds against the 25-second default deadline. So the proposed assertion holds today
without any production change.

A second local probe (not committed) added the negative suite of D4 option (b) on top of the first:
a 3-second deadline with 1-second drain budgets, and one extra component spawned through
`jobs.reserve` and `jobs.spawn` whose future never completes. The awaited shutdown returned after
3.00 seconds and the fixture panicked naming `health_check_api` and `hung_component` (the health
check API stops last, so it was still waiting when the deadline aborted both). The command was
`cargo test --test <probe target> -- --nocapture`; it exited with status 0 and the `should_panic`
test passed in 3.04 seconds.

#### Decisions for the maintainer

| ID  | Decision                                              | Options                                                                                                                                                            | Recommendation                                                                                                                                                                                                                            |
| --- | ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| D1  | Which call starts the fixture's shutdown?             | (a) `app::begin_shutdown`; (b) keep `jobs.cancel()`.                                                                                                               | (a). It is the production boundary, so a change to it is exercised by every suite, and the fixture stops duplicating production lifecycle steps.                                                                                          |
| D2  | Which deadline does the fixture pass to `wait_for_all`? | (a) the configured `application_config.shutdown.deadline()`; (b) keep the fixed 10 seconds; (c) a shorter fixture deadline.                                        | (a). It is what production does, and it removes the constant. A hung component then costs up to 25 seconds once, and fails the suite (D3). A suite can still set `[application.shutdown]` in its own TOML.                               |
| D3  | What does the fixture do with the outcomes?           | (a) panic inside `shutdown()` and `restart()` when any outcome is not graceful, naming the components; (b) return the outcomes to each suite; (c) keep discarding. | (a). Every suite gets the check with no change at its call site, and no suite can forget it. A suite that tests a non-graceful shutdown of the tracker is an executable-boundary test (`tests/lifecycle/shutdown_policy.rs`, `tests/lifecycle/signals.rs`), not a user of this fixture. The one exception is the negative suite of D4 option (b), which uses the fixture to test this assertion itself.                  |
| D4  | Is an extra negative test needed for the assertion?   | (a) none; (b) add a suite that injects a component that never stops and expects the fixture's shutdown to panic.                                               | (b). No production hook is needed: `JobManager::reserve` and `JobManager::spawn` are public, and `packages/hung-component-tracker/src/main.rs` injects its hung component with them. The existing executable-boundary test (`packages/hung-component-tracker/tests/hung_component_abort.rs`) covers the supervisor's abort and exit code, not the fixture's assertion; without (b) nothing shows that the assertion can fail. The cost is one more Cargo test target, one small fixture method that only this target uses, and about 3 seconds of configured deadline per run. |
| D5  | What does `Drop` do when a suite panics before `shutdown()`? | (a) call `app::begin_shutdown`; (b) keep `jobs.cancel()`.                                                                                                   | (a), for one shutdown entry point in the fixture. `Drop` still cannot await or assert; that limit stays documented.                                                                                                                       |

#### Decision record

The maintainer decided D1 to D5 on 2026-11-19, one at a time, after reading the options with code
examples. The options he weighed are kept here with their advantages and disadvantages.

**D1: which call starts the fixture's shutdown. Decided: (a) `app::begin_shutdown`.**

- (a) `app::begin_shutdown`.
  - For: the suites stop the tracker through the production entry point, which marks the
    application not ready and then cancels the jobs, in that order.
  - For: a step added to `begin_shutdown` later is exercised by every suite with no fixture change.
  - For: no suite changes its call site; the local probe passed the ten targets.
  - Against: none found.
- (b) keep `jobs.cancel()`.
  - For: no code change.
  - Against: the fixture skips the readiness step, a path production never takes, and AC8a would
    have to record it as an accepted difference.
  - Against: no current suite needs the tracker to stay ready during shutdown, so the difference
    buys nothing.

**D2: which deadline the fixture passes to `wait_for_all`. Decided: (a) the configured deadline.**
Option (c) stays available as a later change if the suites become too slow.

- (a) the configured `application_config.shutdown.deadline()` (default 25 seconds).
  - For: the same value and code path as production; `TRACKER_SHUTDOWN_GRACE_PERIOD` is removed.
  - For: a suite controls the deadline through `[application.shutdown]` in its own TOML, as an
    operator does. The negative suite of D4 depends on this.
  - Against: with the default configuration, a component that hangs fails the suite after 25
    seconds instead of 10. The cost appears only when something is already broken.
- (b) keep the fixed 10 seconds.
  - For: no change; a hang fails 15 seconds sooner with the default configuration.
  - Against: the fixture ignores the configuration, so the deadline cannot be tested at this level.
  - Against: 10 seconds is below the default 25, so the fixture could abort a component that
    production would let finish.
- (c) the fixture forces a shorter deadline into every suite's configuration.
  - For: a hang fails soonest.
  - Against: the fixture must rewrite each suite's configuration, including the three drain
    budgets, because the configuration requires
    `max(http_drain, udp_request_drain) + event_listener_drain < deadline`. That is more fixture
    logic, and it hides what the suite's TOML says.
  - Against: the suites stop running with the defaults that operators run with.

**D3: what the fixture does with the outcomes. Decided: (a) panic and name the components.**

- (a) panic inside `shutdown()` and `restart()` when any outcome is not graceful.
  - For: every suite checks a clean shutdown with no change at its call site, and no suite can
    forget the check.
  - For: the message names the components, and the production helper `incomplete_components`
    decides what is graceful, so tests and production agree.
  - Against: a suite cannot inspect the outcomes, and a suite that expects a non-graceful shutdown
    must use `should_panic` (as the D4 suite does). A method that returns the outcomes can be
    added when a suite needs it; none does today.
- (b) return the outcomes to each suite.
  - For: the most flexible; a suite can expect a specific outcome without `should_panic`.
  - Against: every suite must remember the assertion and every existing call site changes.
  - Against: `restart()` also shuts down, so it would have to return outcomes too.
- (c) keep discarding the outcomes.
  - For: no change.
  - Against: a shutdown regression stays invisible at this level; the only sign is a slow suite.

**D4: an extra negative test for the assertion. Decided: (b) add the negative suite.**

- (a) none.
  - For: nothing to add; the root `Cargo.toml` and the fixture's API stay as they are.
  - Against: nothing shows that the D3 assertion can fail. If it were removed, or
    `incomplete_components` stopped reporting aborted components, every suite would still pass.
- (b) a suite that injects a component that never stops and expects `shutdown()` to panic.
  - For: it proves the assertion fails when it should. With the bound on the elapsed time (step 3
    of the planned change) it also proves that the configured deadline of D2 is the one used.
  - For: no production code changes; `JobManager::reserve` and `JobManager::spawn` are public.
  - Against: one more Cargo test target with a `[[test]]` entry in the root `Cargo.toml`, because
    the model is one tracker per test binary.
  - Against: one more public fixture method that other suites could misuse.
  - Against: about 3 seconds more per full run.
  - Note: in the probe the panic named `health_check_api` and `hung_component`. The health check
    API stops last, so it was also running at the deadline. The suite requires only
    `hung_component` in the message.

**D5: what `Drop` does when a suite panics before `shutdown()`. Decided: (a) `app::begin_shutdown`.**

- (a) call `app::begin_shutdown`.
  - For: consistent with D1; `jobs.cancel()` no longer appears in the fixture, so there is one way
    to start a shutdown.
  - For: `begin_shutdown` is synchronous (it sets a flag and cancels the tokens of the
    components that wait for no other), so it is safe in `Drop`.
  - Against: almost no practical effect, because the suite has already failed.
- (b) keep `jobs.cancel()`.
  - For: no change; `Drop` stays the minimal cleanup.
  - Against: the fixture keeps a second shutdown path that differs from production.

Known limit, unchanged by D5: `Drop` cannot await or assert. A suite that never calls `shutdown()`
and does not panic passes without the D3 check. A panic inside `Drop` during another panic aborts
the process, so `Drop` does not assert; the ten current targets all call `shutdown()`.

#### Planned change

The change is small. It stays in the test tree, except for one `[[test]]` entry in the root
`Cargo.toml` for the negative suite of D4:

1. `tests/common/workspace.rs`: add one private async helper that calls `app::begin_shutdown`,
   awaits `wait_for_all` with the configured deadline, and asserts that
   `incomplete_components(&outcomes)` is empty, with the component names in the panic message.
   `shutdown()` and `restart()` call it; `Drop` calls `app::begin_shutdown`. Remove
   `TRACKER_SHUTDOWN_GRACE_PERIOD`.
2. `tests/AGENTS.md` (requirement 5 and the restart paragraph), the `tests/scaffold.rs` module
   documentation and the status line of `completion-plan.md`: describe the new sequence and the
   outcome assertion.
3. For D4: add a fixture method that reserves and spawns one extra component on the fixture's
   `JobManager`, and a new target (proposed: `tests/lifecycle/fixture_shutdown_assertion.rs`,
   registered as `lifecycle-fixture-shutdown-assertion` with a `[[test]]` entry in the root
   `Cargo.toml`, like the other nested targets). Its suite sets a short
   `[application.shutdown]` deadline, spawns a component that never stops, and uses `should_panic`
   to require that `shutdown()` names it. The suite also bounds the elapsed time of `shutdown()`:
   the panic must arrive well before 10 seconds (the old constant; the default deadline is 25
   seconds), so the suite fails if the fixture ignores the configured deadline.
4. No existing suite changes its call site. No production code changes.

#### Test development

Only the negative suite of D4 adds a test. It follows the progressive test-development loop of the
`create-issue` skill, with the `write-unit-test` skill:

1. The negative suite is one test-producing increment, made after the fixture change of step 1
   passes the ten existing targets.
2. After the increment passes its focused run, do the prose-first Arrange-Act-Assert review before
   anything else: write temporary prose for each section, refactor until the code expresses it,
   and remove the redundant prose. The review confirms that the test exposes the one causal
   initial-state difference (a component that never stops), that the fixture method owns only
   incidental mechanics (reserve and spawn), and that the production Act (`shutdown()`) and the
   independently specified expected result (the panic names the component) stay visible. Record
   the result in this specification.
3. Stop for maintainer review after that increment, before the final verification (E2 to E5), the
   commit and the pull request.

#### Evidence for AC8a

| ID  | Evidence                                                                                                                                                                                                                            |
| --- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| E1  | The fixture asserts that every component outcome is graceful, so each fixture-based suite proves cooperative completion on every run, with no sleep and no reliance on process exit. |
| E2  | All ten fixture-based targets pass in one Cargo invocation with the updated fixture (M1 repeated with the current target list).                                                                                                     |
| E3  | `metrics-port-zero` passes with `--nocapture` and with `--test-threads=1` (M2 and M4 repeated). The recorded duration of each run is far below the 25-second deadline, which replaces the 60 to 81 seconds of 2026-08-24.            |
| E4  | The pre-commit gate, including `linter all`, passes (M5 repeated).                                                                                                                                                                  |
| E5  | The new target of D4 passes. It shows that a component aborted at the deadline makes the fixture's shutdown panic, that the panic names the component, and that it arrives within the suite's bound on the elapsed time, well before 10 seconds.                                                                       |

The current target list for E2 replaces the six-target command of the partial delivery:

```sh
cargo test --test metrics-fixed-ports --test metrics-port-zero --test metrics-udp-error-enabled-port-zero --test metrics-udp-error-disabled-port-zero --test banning-udp-metrics-disabled-port-zero --test banning-udp-shared-connection-id-error-limit --test banning-udp-shared-connection-id-error-limit-reverse-order --test persistence-completed-downloads-on-shutdown --test persistence-scrape-after-restart --test scaffold
```

#### Delivery

1. This specification update, in its own pull request, so the plan is reviewed before code.
2. The fixture change, the documentation updates and the recorded evidence (R8, AC8a).
3. R9: the closure review of every acceptance criterion, then the specification moves to
   `docs/issues/closed/`.

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

1. Bug #2035: [fix duplicate port-zero tracker instance bootstrap](../../closed/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md)
   — `AppContainer` now retains HTTP and UDP per-instance containers with their
   `ConfigurationInstanceId`, preventing repeated `0.0.0.0:0` configuration blocks from
   overwriting each other before startup.
2. Feature #2036: [add runtime service registry metadata](../../closed/2036-add-runtime-service-registry-metadata/ISSUE.md)
   — `Registar` metadata now exposes stable service role and configuration-instance identity for
   side-effect-free test endpoint discovery.

The runtime registry boundary remains recorded in
[ADR 20260728115400](../../../adrs/20260728115400_define_registar_as_runtime_service_registry.md).
The dedicated prerequisite specifications above are the implementation records for that boundary.
