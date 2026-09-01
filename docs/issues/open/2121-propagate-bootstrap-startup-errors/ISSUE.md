---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: null
github-issue: 2121
spec-path: docs/issues/open/2121-propagate-bootstrap-startup-errors/ISSUE.md
branch: "2121-propagate-bootstrap-errors"
related-pr: 2123
depends-on:
  - 2107
last-updated-utc: 2026-08-31 17:13
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2107-1978-activate-persistence-free-v3-runtime-composition/ISSUE.md
    - src/AGENTS.md
    - src/bootstrap/app.rs
    - src/bootstrap/config.rs
    - src/bootstrap/persistence.rs
    - src/container.rs
    - src/app.rs
    - src/main.rs
---

<!-- skill-link: create-issue -->

# Issue #2121 - Propagate bootstrap startup errors

## Goal

Make every expected failure during initial tracker startup an explicit typed
error from `app::run()` through the tracker executable boundary. The executable
must report the failure with context and exit unsuccessfully, cancelling any
jobs already started during the failed startup attempt.

## Background

The configuration source APIs already expose fallible operations:
`Info::new`, `Configuration::load`, semantic validation, and bootstrap
persistence-requirement validation each return errors. After #2107, tracker
core composition also has a persistence-enabled branch that can fail while
constructing the configured database driver and applying migrations.

Current startup code converts these categories to `expect` or `panic` in
`initialize_configuration`, `setup`, and `AppContainer::initialize`.
`app::start()` and its job-starter call stack also panic for expected
database-load, TLS-material, registration, and listener-start failures. These
are expected, operator-facing startup failures, but panic messages discard
their typed source and make failure paths difficult to test directly.

Fail-fast startup remains the intended operational behavior, but panicking
hides the typed cause at intermediate boundaries and makes individual failure
paths harder to test. A `Result`-based bootstrap boundary will show what can
fail, preserve error context, clean up partially started jobs, and give
executable entrypoints one consistent way to report startup failure.

## Scope

### In Scope

- Return typed errors from configuration source creation and loading instead of
  panicking in `initialize_configuration`.
- Define typed startup errors that retain source errors from configuration
  loading, semantic validation, persistence-requirement validation,
  application-container composition, initial persistence data loading, and
  configured service startup.
- Change `setup()`, `start()`, each fallible startup helper, and `app::run()`
  to return and propagate typed `Result` values. Update executable callers,
  including profiling and integration test helpers that start the complete
  application.
- Refactor application-container and tracker-core initialization so expected
  configured-driver and migration failures return a contextual typed error
  rather than being converted to `expect` or an ambiguous `Option`.
- Report startup failures at the executable boundary with useful context and a
  nonzero exit status.
- Cancel and join jobs that were started before a subsequent initial startup
  failure, without treating post-start task failures as startup results.
- Add focused tests for representative configuration, composition,
  persistence-load, and listener-start failures without starting unrelated
  runtime services.

### Out of Scope

- Changing `check_seed()` from its assertion-based internal cryptographic
  invariant. It is not operator configuration input.
- Treating asynchronous task failures that occur after that task successfully
  starts as initial startup results.
- Reclassifying operational database failures as configuration validation
  errors.
- Changing graceful shutdown behavior after successful startup.

## Architectural Decisions

- Related ADR: `docs/adrs/20260825193119_make_persistence_an_optional_application_composition_capability.md`.
- Related completed work: #2107 established the persistence-free and
  persistence-enabled composition branches this task must preserve.
- `app::run()` owns the typed startup boundary: it returns `Ok` only after
  `setup()`, initial persistence loading, and configured job startup succeed.
  Error variants retain their source categories instead of being flattened to
  strings.
- If a configured service fails after another job has started, `run()` cancels
  and joins the already-started jobs before returning that startup error.
- `check_seed()` remains an assertion because it guards an internal
  cryptographic invariant, not an operator-controlled configuration failure.
- ADRs to create: None known. Create one during implementation if the error
  boundary changes a repository-wide error-handling policy or package contract.

## Known Refactoring Targets

These targets reflect the current startup path and are subject to T1
reconciliation; they are not an exhaustive implementation inventory.

- `src/bootstrap/config.rs`: make `initialize_configuration()` return its
  configuration-source or load error.
- `src/bootstrap/app.rs`: replace expected validation panics and return a
  source-preserving bootstrap `Result` from `setup()`.
- `packages/tracker-core/src/container.rs` and `src/container.rs`: return
  typed configured-driver, migration, and application-composition errors rather
  than `Option` or `expect`.
- `src/app.rs`: make `start()`, initial persistence loaders, service starters,
  and `run()` propagate expected startup errors. Cancel already-started jobs
  when a later startup operation fails.
- `src/bootstrap/jobs/health_check_api.rs`,
  `src/bootstrap/jobs/http_tracker.rs`, `src/bootstrap/jobs/tracker_apis.rs`,
  and `src/bootstrap/jobs/udp_tracker.rs`: return typed TLS, registration, and
  listener-start errors rather than panicking.
- `src/bootstrap/jobs/tracker_core.rs`: replace the persistence assumption in
  the persistent-statistics listener startup path with a typed startup error.
- `src/main.rs`, `src/console/profiling.rs`, and
  `tests/common/workspace.rs`: handle or surface `run()` failures according to
  their executable and test contracts.
- `src/AGENTS.md`: replace the stated startup-panic policy with the final
  documented startup-error contract.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                  | Notes / Expected Output                                                                                                                                                                    |
| --- | ------ | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Map expected startup failures         | Recursively inspect every call from `app::run()` through `setup()`, `start()`, data loading, and initial job startup; classify expected sources, invariants, and post-start task failures. |
| T2  | TODO   | Make composition fallible             | Return typed errors from tracker-core and application-container initialization; replace expected failure `expect`/`Option` paths without changing valid persistence-free composition.      |
| T3  | TODO   | Establish bootstrap boundary          | Make `initialize_configuration()` and `setup()` return typed `Result` values with source-preserving bootstrap context.                                                                     |
| T4  | TODO   | Propagate the complete startup result | Make `start()`, its loaders, and its configured service starters return typed errors; have `run()` cancel and join partial startup jobs before it returns an error.                        |
| T5  | TODO   | Report at executable callers          | Report `app::run()` errors consistently from the tracker and profiling entrypoints and adapt full-application test helpers.                                                                |
| T6  | TODO   | Prove failure behavior                | Add focused failure-path tests and document the final startup-error contract in `src/AGENTS.md` and operator-facing documentation when it changes.                                         |

## Progress Tracking

### Workflow Checkpoints

- [x] Deferred draft recorded while implementing #2107.
- [x] #2107 completed and final composition error boundaries reviewed.
- [x] Spec drafted in `docs/issues/drafts/`.
- [x] Spec reviewed and approved by user/maintainer.
- [x] GitHub issue #2121 created and issue number added to this spec.
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation.
- [ ] Implementation completed.
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks).
- [ ] Manual verification scenarios executed and recorded (status + evidence).
- [ ] Acceptance criteria reviewed after implementation and updated with evidence.
- [ ] Reviewer validated acceptance criteria and updated checkboxes.
- [ ] Committer verified spec progress is up to date before commit.
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-08-28 11:58 UTC - GitHub Copilot/User - Recorded a deferred draft after observing expected startup failures converted to `expect` or `panic` during #2107.
- 2026-08-31 15:46 UTC - GitHub Copilot - Reconciled the draft with merged and closed #2107. The formal issue draft defines typed configuration and composition errors through `setup()` and `app::run()` while retaining post-setup runtime failures and `check_seed()` outside scope.
- 2026-08-31 15:46 UTC - GitHub Copilot/User - Added a concrete, non-exhaustive list of current refactoring targets. T1 remains responsible for reconciling it with the exact error types and callers before implementation.
- 2026-08-31 16:03 UTC - GitHub Copilot/User - Expanded the intended boundary from `setup()` to complete initial startup. `app::run()` must propagate expected failures from `setup()`, `start()`, initial persistence loading, and configured job startup to `main()`, cancelling partial startup jobs before returning an error.
- 2026-08-31 16:09 UTC - GitHub Copilot/User - Approved the specification. Created GitHub issue #2121 with the `task` label and moved this document into `docs/issues/open/`.
- 2026-08-31 16:45 UTC - GitHub Copilot/User - Converted this specification to folder-style layout so issue-local implementation evidence can be added without a later layout migration.
- 2026-08-31 17:13 UTC - GitHub Copilot/User - Opened spec-only PR #2123 for this specification and #2122. It is related to, not an implementation that closes, either issue.

## Acceptance Criteria

- [ ] AC1: Configuration-source creation and loading failures return typed errors from `initialize_configuration()` instead of panicking.
- [ ] AC2: Semantic configuration and persistence-requirement validation failures return source-preserving startup errors before global services or application containers are initialized.
- [ ] AC3: Expected configured-driver, migration, and application-container composition failures return contextual typed errors rather than `expect` or an ambiguous `Option`.
- [ ] AC4: Initial persistence-data loading and configured TLS, registration, and listener-start failures return source-preserving startup errors instead of panicking.
- [ ] AC5: `setup()`, `start()`, and `app::run()` propagate typed startup errors; `run()` returns `Ok` only after all configured initial startup work succeeds.
- [ ] AC6: A failure after another initial job has started cancels and joins the partial startup jobs before `run()` returns the error.
- [ ] AC7: The tracker executable and profiling executable report startup failures with context and exit nonzero.
- [ ] AC8: Valid persistence-free and configured-persistence composition behavior from #2107 remains unchanged.
- [ ] AC9: `check_seed()` remains an assertion for its internal invariant, and asynchronous task failures after successful task startup remain outside this task's contract.
- [ ] AC10: Focused tests cover representative source, semantic, requirement, composition, persistence-load, and listener-start failures without starting unrelated services.
- [ ] AC11: `linter all` exits with code `0`, relevant tests pass, manual verification scenarios are executed and documented, and acceptance criteria are re-reviewed against actual behavior.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- Focused unit tests for configuration loading, bootstrap validation, and fallible container composition.
- Focused application tests that prove a `setup()` failure prevents job startup and listener binding and that a later failure cleans up already-started jobs.
- Focused loader and job-starter tests for database-load, TLS, registration, and listener-start errors.
- Entrypoint/subprocess tests for contextual stderr output and nonzero status where the test harness permits them.
- Regression tests for both persistence-free and configured-persistence composition.
- `cargo fmt`, `linter all`, relevant package tests, and pre-push checks when applicable.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                         | Command/Steps                                                                                         | Expected Result                                                                                           | Status | Evidence                                                  |
| --- | -------------------------------- | ----------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------------------------- |
| M1  | Invalid configuration source     | Run the tracker with `TORRUST_TRACKER_CONFIG_TOML_PATH` set to a nonexistent file.                    | The executable reports a contextual configuration-source failure, exits nonzero, and creates no listener. | TODO   | {log path and exit status}                                |
| M2  | Invalid persistence requirements | Run the tracker with a v3 configuration that enables `core.private = true` and omits `core.database`. | The executable reports the typed requirement failure before application composition and exits nonzero.    | TODO   | {configuration, log path, and exit status}                |
| M3  | Unavailable configured listener  | Run the tracker with a valid configuration whose configured HTTP or UDP listener cannot bind.         | The executable reports the listener-start error, exits nonzero, and stops any previously started jobs.    | TODO   | {configuration, log path, exit status, and port evidence} |
| M4  | Valid startup regression         | Run one documented persistence-free v3 configuration and one configured SQLite v3 configuration.      | Both configurations retain #2107's successful startup behavior.                                           | TODO   | {commands, logs, and health-check evidence}               |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |
| AC6   | TODO                   | {test/log/PR link} |
| AC7   | TODO                   | {test/log/PR link} |
| AC8   | TODO                   | {test/log/PR link} |
| AC9   | TODO                   | {test/log/PR link} |
| AC10  | TODO                   | {test/log/PR link} |
| AC11  | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **Partial startup:** A listener can fail after other jobs have started. Mitigation: make `run()` own partial-startup cancellation and joining before it returns the source-preserving error.
- **Error-boundary breadth:** Recursive startup propagation can accidentally include asynchronous task supervision. Mitigation: the boundary ends when each configured startup operation returns successfully; later task failures remain outside this task.
- **Source fidelity:** Converting lower-layer errors to strings would make callers and tests unable to distinguish source categories. Mitigation: preserve error chains in typed variants through bootstrap and delay formatting until executable reporting.
- **Persistence regression:** Refactoring container initialization can accidentally make valid persistence-free composition fallible. Mitigation: retain #2107 regression coverage for both composition branches.

## References

- GitHub issue: #2121
- Completed prerequisite: #2107
- Parent EPIC of completed prerequisite: #1978
- Startup policy: `src/AGENTS.md`
- Configuration bootstrap: `src/bootstrap/config.rs`
- Bootstrap composition: `src/bootstrap/app.rs`
- Application startup: `src/app.rs`
- Executable entrypoint: `src/main.rs`
