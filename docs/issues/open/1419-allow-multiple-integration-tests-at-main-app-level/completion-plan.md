# Completion Plan — Issue #1419

> **Issue specification:** [ISSUE.md](ISSUE.md)
>
> **Status:** Proposed; requires maintainer review before implementation
> **Scope:** Deterministic teardown, related test coverage, documentation alignment, and manual plus automated verification

## Purpose

This document supplies the problem statement, alternatives, decision criteria, and verification
requirements for the remaining non-trivial work in issue #1419. `ISSUE.md` remains the source of
truth for scope, acceptance criteria, and progress. This companion document explains why the
remaining lifecycle work is necessary and how it must be evaluated before implementation.

## Problem: Application Jobs Outlive Test-Suite Ownership

Each current main-level integration-test suite creates an `EphemeralTrackerWorkspace`, starts the
application through `start_tracker_with_config`, and receives an `AppContainer` plus a
`JobManager`. The suite binds the manager as `_jobs` and lets it drop when the test function ends.

Dropping `JobManager` does not request cancellation or wait for its `JoinHandle`s. `JobManager`
only performs a graceful shutdown when its owner explicitly:

1. calls `JobManager::cancel()` to signal the shared cancellation token; and
2. consumes the manager with `JobManager::wait_for_all(grace_period)` to await each job.

The existing tests therefore rely on test-process termination to end servers and background jobs.
That violates the selected one-application-per-executable lifecycle: the tracker should stop before
its `TempDir` workspace is released. It also hides shutdown failures, makes resource ownership
unclear, and prevents a focused test from proving graceful teardown without process exit.

This is not a port-zero or runtime-registry defect. Port-zero bindings, isolated temporary
workspaces, and runtime endpoint discovery are already implemented. The remaining defect is that
the test fixture does not own shutdown as part of its normal lifecycle.

## Required Outcome

Every current main-level suite must have one clear owner for its application lifecycle. After the
last scenario completes, that owner must cancel the application jobs, await them with a bounded
grace period, and only then permit the `EphemeralTrackerWorkspace` to be dropped.

The resulting test structure must make the required lifetime order evident:

```text
create workspace → start application → run sequential scenarios → cancel jobs → await jobs → drop workspace
```

The solution must not depend on process exit, a fixed sleep, parsing logs, or a new application
configuration.

## Alternatives Considered

### Alternative A — Keep `_jobs` and rely on test-process exit

**Description:** Retain the current bindings and permit process termination to clean up tasks and
listeners.

**Rejected because:** It does not execute `JobManager`'s intended cancellation and waiting API,
cannot prove normal teardown, and allows the workspace to be dropped while tasks may still use
runtime state. It also makes lifecycle failures invisible unless they manifest as a later test or
process-level failure.

### Alternative B — Add explicit teardown code in every suite

**Description:** At the end of each suite, manually call `jobs.cancel()` followed by
`jobs.wait_for_all(...)`.

**Advantages:** Minimal new abstraction and direct use of the existing shutdown API.

**Rejected as the default approach because:** Six suites would repeat lifecycle-sensitive ordering.
A future suite could forget one operation, select a different grace period, or drop the workspace
first. Repeated teardown also makes failure-safe cleanup difficult when a scenario returns or
panics before the final lines of the test body execute.

### Alternative C — Introduce a test-local suite lifecycle fixture

**Description:** Add a small fixture under `tests/common/` that owns both the
`EphemeralTrackerWorkspace` and `JobManager`, exposes the `AppContainer` needed by scenarios, and
provides one explicit asynchronous shutdown operation. The fixture enforces that the workspace
outlives the awaited jobs.

**Selected, subject to implementation review.** It centralizes the required ownership order in the
same test-local module that already owns workspace creation, startup readiness, and endpoint
discovery. It avoids exporting main-application-specific lifecycle behavior through
`packages/test-helpers` before there is a second consumer.

An asynchronous operation cannot be performed from a normal Rust `Drop` implementation. The
fixture must therefore make shutdown explicit in each suite, or use a test-runner structure that
can always await shutdown before returning. The implementation must document its behavior when a
scenario fails or panics; it must not claim that synchronous `Drop` guarantees async graceful
teardown.

### Alternative D — Make `JobManager` abort jobs on drop

**Description:** Change production `JobManager` drop semantics so dropping it aborts all tasks.

**Rejected because:** This changes a general production lifecycle contract to solve a test-fixture
ownership problem. Aborting is not equivalent to cooperative cancellation plus bounded graceful
waiting, and it would affect every production caller.

### Alternative E — Run the tracker as a child process

**Description:** Replace the in-process suite with a process runner that terminates the tracker
process after testing.

**Rejected for this issue:** The current in-process architecture already supplies the required
application composition coverage. A child-process runner adds readiness, diagnostic capture,
signal, and cleanup concerns without being needed to solve the existing `JobManager` ownership
gap. It remains deferred for future tests of executable startup, signals, logging, or exit behavior.

## Chosen Direction

Implement **Alternative C**, a smallest-possible lifecycle fixture in `tests/common/`, subject to
review of the exact API. It must:

- retain the `EphemeralTrackerWorkspace` for at least as long as tracker jobs are awaited;
- expose the `Arc<AppContainer>` needed by existing scenario functions without duplicating runtime
  discovery logic;
- use the existing `JobManager::cancel()` and `JobManager::wait_for_all(...)` APIs;
- define one shared, documented grace period appropriate for the current suites;
- require explicit awaited shutdown before a successful suite returns; and
- keep ownership test-local rather than modifying production lifecycle semantics.

The exact fixture name and API are intentionally not prescribed here. The implementation should
prefer a minimal, intention-revealing interface over a general-purpose test framework.

## Test-Coverage Design

The implementation must add focused coverage for the lifecycle helper itself, in addition to
migrating all current suites.

The focused test should establish observable ordering rather than merely call the helper:

1. Start a tracker using an isolated workspace through the shared fixture.
2. Complete a small existing scenario or readiness assertion.
3. Invoke the fixture's explicit asynchronous shutdown.
4. Assert the shutdown operation completed before releasing the fixture/workspace.

The test must not use a sleep as proof of completion and must not rely on test-process exit. If the
existing `JobManager` API does not expose sufficient observability to prove more than successful
awaited completion, document that limitation and use the successful bounded wait as the lifecycle
contract. Do not introduce production-only test hooks solely for this issue without a separately
reviewed justification.

All suites using `start_tracker_with_config` must migrate in the same change, including
`tests/scaffold.rs`. No `_jobs` binding may remain as an implicit teardown mechanism.

## Documentation Changes

Update the following alongside the code:

- `tests/scaffold.rs`: replace references to the removed `stats` target with actual current targets
  and show the explicit lifecycle shutdown pattern.
- `tests/AGENTS.md`: describe tracing as one of several process-global lifecycle constraints;
  retain the one-application-per-executable rule and add the fixture's required shutdown sequence.
- `ISSUE.md`: mark implementation tasks only after code and verification evidence exist. Record
  commands, dates, results, and any deviation from this decision document.

## Manual Verification Is Mandatory

Automated tests are necessary but insufficient for this lifecycle change. The implementation changes
how real listeners and background jobs are stopped, so manual execution of the affected integration
targets is mandatory after implementation. Record the command, UTC date, exit status, and concise
observed result in `ISSUE.md`.

### Required Manual Runs

| ID  | Command / action                                            | Expected observation                                                                                       |
| --- | ----------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| MV1 | Run all current main-level targets in one Cargo invocation. | All targets exit successfully without port, configuration, storage, or shutdown interference.              |
| MV2 | Run `metrics-port-zero` with `-- --nocapture`.              | The suite completes after explicit teardown; its port-zero services have distinct non-zero final bindings. |
| MV3 | Run `metrics-port-zero` with `-- --test-threads=1`.         | The suite succeeds in serial mode and does not depend on normal parallel scheduling.                       |
| MV4 | Run `linter all`.                                           | The repository quality gate succeeds after the code and documentation changes.                             |

Use the commands listed in `ISSUE.md` as the authoritative command text. If a command must change,
update both documents in the same change and explain why.

## Implementation Checklist

- [ ] Review this decision record and confirm the selected fixture direction.
- [ ] Implement the smallest test-local lifecycle fixture.
- [ ] Migrate every current suite and remove `_jobs` drop-only cleanup.
- [ ] Add focused lifecycle coverage.
- [ ] Update `tests/scaffold.rs` and `tests/AGENTS.md`.
- [ ] Execute and record MV1–MV4.
- [ ] Update `ISSUE.md` progress, acceptance criteria, and closure decision.
