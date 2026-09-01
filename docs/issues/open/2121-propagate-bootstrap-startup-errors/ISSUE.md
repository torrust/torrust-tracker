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
last-updated-utc: 2026-09-01 12:30
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/closed/2107-1978-activate-persistence-free-v3-runtime-composition/ISSUE.md
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
error from `app::start()` through the tracker executable boundary. The executable
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
- Change `setup()`, `start()`, each fallible startup helper, and `app::start()`
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
- `app::start()` owns the typed startup boundary: it returns `Ok` only after
  `setup()`, initial persistence loading, and configured job startup succeed.
  Error variants retain their source categories instead of being flattened to
  strings.
- Startup errors use the existing `thiserror` pattern. Their `Display`
  messages must be friendly to operators and identify an actionable resolution;
  variants retain the typed source error for diagnostics and future structured
  reporting.
- The tracker and profiling entrypoints preserve their current output style for
  this scoped change: report the friendly startup error through the existing
  diagnostic mechanism and exit unsuccessfully. Do not add a verbosity flag or
  migrate output to the global JSONL contract in this issue; the global CLI
  output ADR permits progressive migration of existing commands.
- If a configured service fails after another job has started, `run()` cancels
  and joins the already-started jobs before returning that startup error. Each
  job gets a bounded graceful-shutdown period; a job that exceeds that period is
  aborted and joined so startup never returns while its handle is detached.
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

| ID  | Status | Task                                  | Notes / Expected Output                                                                                                                                                                                   |
| --- | ------ | ------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Map expected startup failures         | Mapped setup, composition, initial persistence loads, and job starter paths. `check_seed()` remains an invariant; task failures after successful starts remain outside the boundary.                      |
| T2  | DONE   | Make composition fallible             | Tracker-core and application composition return typed errors. Deterministic no-persistence persistent-statistics tests cover both seams; legacy convenience APIs remain intentionally outside this scope. |
| T3  | DONE   | Establish bootstrap boundary          | `initialize_configuration()` and `setup()` return source-preserving errors. Focused configuration-source, semantic-validation, and persistence-requirement tests were added.                              |
| T4  | DONE   | Propagate the complete startup result | Root startup and all configured HTTP, REST API, health-check, and UDP starters propagate typed TLS, bind, startup-notification, and registration errors. Focused TLS/listener tests pass.                 |
| T5  | DONE   | Report at executable callers          | Main tracker, profiling, and integration helper callers were adapted. M1-M3 provide executable nonzero-status diagnostics.                                                                                |
| T6  | DONE   | Prove failure behavior                | `src/AGENTS.md` documents the contract; focused tests, including real peer-key loader and public UDP registration-failure cleanup paths, M1-M4, and the mandatory pre-commit quality gate passed.         |

## Progress Tracking

### Workflow Checkpoints

- [x] Deferred draft recorded while implementing #2107.
- [x] #2107 completed and final composition error boundaries reviewed.
- [x] Spec drafted in `docs/issues/drafts/`.
- [x] Spec reviewed and approved by user/maintainer.
- [x] GitHub issue #2121 created and issue number added to this spec.
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation.
- [x] Implementation completed.
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks).
- [x] Manual verification scenarios executed and recorded (status + evidence).
- [x] Acceptance criteria reviewed after implementation and updated with evidence.
- [ ] Reviewer validated acceptance criteria and updated checkboxes.
- [ ] Committer verified spec progress is up to date before commit.
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-08-28 11:58 UTC - GitHub Copilot/User - Recorded a deferred draft after observing expected startup failures converted to `expect` or `panic` during #2107.
- 2026-08-31 15:46 UTC - GitHub Copilot - Reconciled the draft with merged and closed #2107. The formal issue draft defines typed configuration and composition errors through `setup()` and `app::start()` while retaining post-setup runtime failures and `check_seed()` outside scope.
- 2026-08-31 15:46 UTC - GitHub Copilot/User - Added a concrete, non-exhaustive list of current refactoring targets. T1 remains responsible for reconciling it with the exact error types and callers before implementation.
- 2026-08-31 16:03 UTC - GitHub Copilot/User - Expanded the intended boundary from `setup()` to complete initial startup. `app::start()` must propagate expected failures from `setup()`, startup completion, initial persistence loading, and configured job startup to `main()`, cancelling partial startup jobs before returning an error.
- 2026-08-31 16:09 UTC - GitHub Copilot/User - Approved the specification. Created GitHub issue #2121 with the `task` label and moved this document into `docs/issues/open/`.
- 2026-08-31 16:45 UTC - GitHub Copilot/User - Converted this specification to folder-style layout so issue-local implementation evidence can be added without a later layout migration.
- 2026-08-31 17:13 UTC - GitHub Copilot/User - Opened spec-only PR #2123 for this specification and #2122. It is related to, not an implementation that closes, either issue.
- 2026-09-01 00:00 UTC - GitHub Copilot/User - Confirmed that startup errors must use the existing `thiserror` pattern, with friendly operator messages and descriptive remediation while preserving typed sources. Agreed to retain the current tracker output style for this issue rather than adding verbosity controls or migrating existing entrypoints to the global JSONL output contract. Manual regression coverage must enable at least one service of every configured service type to reduce bootstrap-refactoring risk.
- 2026-09-01 - GitHub Copilot - Reconciled the startup call graph. Implemented initial typed `Result` propagation for configuration, semantic and persistence-requirement validation, persistence composition/migration, initial loads, and selected HTTP/UDP starters. `cargo check -p torrust-tracker`, `cargo test -p torrust-tracker --lib`, and `cargo test -p torrust-tracker-core --lib` passed. Completion remained blocked on refactoring the server-package launchers and health-check path, which then converted bind/registration failures into typed sources at the application startup boundary.
- 2026-09-01 00:30 UTC - GitHub Copilot - Replaced server-launcher bind, startup-notification, and registration panics with typed, source-preserving results in the HTTP tracker, REST API, health-check API, and UDP server paths. HTTP, REST API, and health-check starter jobs now observe the shared startup cancellation token and request service shutdown before joining. UDP binding occurs before the launcher task starts, so an address conflict is returned as the starter result rather than a task panic. Runtime server-task failures remain logged runtime outcomes rather than startup results. Validation is still in progress.
- 2026-09-01 01:00 UTC - GitHub Copilot - Added direct stderr startup diagnostics at tracker and profiling executable boundaries, so early failures remain operator-visible before logging is initialized. UDP launcher shutdown now aborts and joins its receive-loop task before resolving; the existing focused listener-release test covers the outer startup cleanup behavior. The persistent completed-statistics helper now returns a typed error rather than relying on an `expect`. Validation is still in progress.
- 2026-09-01 - GitHub Copilot - Added focused tests for a safely isolated configuration-source failure, semantic and persistence-requirement error categorization, deterministic fallible tracker-core/application composition, and public HTTP TLS/listener failures. `cargo fmt`, `cargo test -p torrust-tracker --lib` (79 tests), and `cargo test -p torrust-tracker-core --lib` (132 tests) passed. M1-M4 were executed with configurations and logs under `.tmp/2121-manual-20260901T113000Z` and `.tmp/2121-manual-20260901T114000Z`; all expected results passed. The final full quality gate remains pending.
- 2026-09-01 - GitHub Copilot - Corrected an existing strict Clippy unit-pattern diagnostic in the modified UDP launcher shutdown select arm. `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json` then passed (including `cargo machete`, `cargo deny check bans`, `linter all`, Containerfile lint, and workspace documentation tests).
- 2026-09-01 - GitHub Copilot - Final reviewer blockers: serialized and restored both configuration environment inputs for every in-process configuration reader; added deterministic source-retention coverage for an initial persistence-load failure; and made UDP launcher failures, including `BrokenPipe`, flow from the launcher task back to `Server::start`. `cargo fmt`, root library tests, UDP-server library tests, root integration tests, and the mandatory JSON pre-commit gate passed.
- 2026-09-01 12:00 UTC - GitHub Copilot - Closed the final acceptance-review evidence gaps without adding test-only production seams. `app::tests::it_should_retain_the_loader_error_when_initial_peer_key_loading_fails` drops the real composed SQLite schema, invokes the real initial peer-key loader, and proves `app::Error::InitialPersistenceLoad` retains the concrete database error and its SQL source. `udp_server::server::tests::it_should_preserve_registration_error_and_release_listener_when_registration_fails` forces the public `Server::start` registration path to encounter `DuplicateBinding`, asserts the typed source and binding, then re-binds the UDP address to prove listener cleanup. `cargo fmt`, relevant root/UDP-server tests, and the mandatory pre-commit gate were rerun successfully.
- 2026-09-01 12:15 UTC - GitHub Copilot/User - Renamed the public startup boundary from `app::run()` to `app::start()`, because it completes initial startup rather than owning the daemon lifecycle. Renamed the narrower post-setup helper to `complete_startup()`. The executable retains signal handling and shutdown ownership.
- 2026-09-01 12:30 UTC - GitHub Copilot - Updated open and draft integration-test documentation that referenced the renamed application startup API.

## Acceptance Criteria

- [x] AC1: Configuration-source creation and loading failures return typed errors from `initialize_configuration()` instead of panicking.
- [x] AC2: Semantic configuration and persistence-requirement validation failures return source-preserving startup errors before global services or application containers are initialized.
- [x] AC3: Expected configured-driver, migration, and application-container composition failures return contextual typed errors rather than `expect` or an ambiguous `Option`.
- [x] AC4: Initial persistence-data loading and configured TLS, registration, and listener-start failures return source-preserving startup errors instead of panicking.
- [x] AC5: `setup()`, `app::start()`, and its startup helpers propagate typed startup errors; `start()` returns `Ok` only after all configured initial startup work succeeds.
- [x] AC6: A failure after another initial job has started cancels and joins the partial startup jobs before `run()` returns the error.
- [x] AC7: The tracker executable and profiling executable report startup failures with context and exit nonzero.
- [x] AC8: Valid persistence-free and configured-persistence composition behavior from #2107 remains unchanged.
- [x] AC9: `check_seed()` remains an assertion for its internal invariant, and asynchronous task failures after successful task startup remain outside this task's contract.
- [x] AC10: Focused tests cover representative source, semantic, requirement, composition, persistence-load, and listener-start failures without starting unrelated services.
- [x] AC11: `linter all` exits with code `0`, relevant tests pass, manual verification scenarios are executed and documented, and acceptance criteria are re-reviewed against actual behavior.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- Focused unit tests for configuration loading, bootstrap validation, and fallible container composition.
- Focused application tests that prove a `setup()` failure prevents job startup and listener binding and that a later failure cleans up already-started jobs.
- Focused loader and job-starter tests for database-load, TLS, registration, and listener-start errors.
- Entrypoint/subprocess tests for the existing-style friendly diagnostic output and nonzero status where the test harness permits them.
- Regression tests for both persistence-free and configured-persistence composition, each enabling at least one instance of every applicable configured service type.
- `cargo fmt`, `linter all`, relevant package tests, and pre-push checks when applicable.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                         | Command/Steps                                                                                                                                                                                                                     | Expected Result                                                                                           | Status | Evidence                                                                                                                                                                            |
| --- | -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M1  | Invalid configuration source     | `env -u TORRUST_TRACKER_CONFIG_TOML TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/2121-manual-20260901T113000Z/missing.toml target/debug/torrust-tracker`                                                                                 | The executable reports a contextual configuration-source failure, exits nonzero, and creates no listener. | DONE   | Exit `1`; `.tmp/2121-manual-20260901T113000Z/m1.log` ends with `Tracker startup failed` and the configuration-load error.                                                           |
| M2  | Invalid persistence requirements | `env -u TORRUST_TRACKER_CONFIG_TOML TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/2121-manual-20260901T113000Z/invalid-private.toml target/debug/torrust-tracker`                                                                         | The executable reports the typed requirement failure before application composition and exits nonzero.    | DONE   | Exit `1`; isolated v3 configuration omits `[core.database]`; `.tmp/2121-manual-20260901T113000Z/m2.log` reports `core.private` requires persistence.                                |
| M3  | Unavailable configured listener  | A Python TCP socket held port `48965`; tracker was launched with an HTTP tracker bound to `127.0.0.1:48965`.                                                                                                                      | The executable reports the listener-start error, exits nonzero, and stops any previously started jobs.    | DONE   | Exit `1`; `.tmp/2121-manual-20260901T111500Z/m3.log` reports `Address already in use (os error 98)` through the HTTP startup error boundary.                                        |
| M4  | Valid startup regression         | Launched isolated persistence-free and SQLite TOML files under `.tmp/2121-manual-20260901T114000Z`; each enabled UDP, HTTP, health-check, and SQLite also enabled REST API. Queried `/health_check`, then sent SIGINT and waited. | Both configurations retain #2107's successful startup behavior across their enabled service types.        | DONE   | Both exit `0`; `summary.txt` records `health=0 exit=0`. Health payloads in `persistence-free-health.json` and `sqlite-health.json` report `status: Ok` for each applicable service. |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                                                                                                                                                              |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `bootstrap::config::tests::it_should_return_a_typed_load_error_when_the_configured_source_file_is_missing`; M1 log.                                                                                                                                                                   |
| AC2   | DONE                   | `bootstrap::app` semantic and persistence-requirement tests; M2 log.                                                                                                                                                                                                                  |
| AC3   | DONE                   | tracker-core and `AppContainer` deterministic persistent-statistics composition tests.                                                                                                                                                                                                |
| AC4   | DONE                   | Public HTTP starter TLS/listener tests; public UDP `Server::start` forced-registration test retains `RegistrationError::DuplicateBinding` and releases the listener; M3 log; real peer-key loader test proves the `InitialPersistenceLoad` database/SQL source chain.                 |
| AC5   | DONE                   | `app::start`, `complete_startup`, and starters return typed errors; focused library tests pass.                                                                                                                                                                                       |
| AC6   | DONE                   | `app::tests::it_should_release_udp_listener_before_returning_from_start_after_setup_when_later_http_startup_fails`.                                                                                                                                                                   |
| AC7   | DONE                   | M1-M3 exit `1` with `Tracker startup failed` contextual diagnostics.                                                                                                                                                                                                                  |
| AC8   | DONE                   | M4 persistence-free and SQLite health checks, clean exit `0`.                                                                                                                                                                                                                         |
| AC9   | DONE                   | `check_seed` assertion retained; server tasks log post-start runtime outcomes without becoming startup results.                                                                                                                                                                       |
| AC10  | DONE                   | Focused configuration, validation, composition, real peer-key loader source-chain, public UDP registration cleanup, TLS, UDP launcher `BrokenPipe`, listener, and partial-start cleanup tests; no unrelated service is started by the new failure tests.                              |
| AC11  | DONE                   | `cargo fmt`; `cargo test -p torrust-tracker --lib` (80); `cargo test -p torrust-tracker-udp-server --lib` (129); `cargo test -p torrust-tracker --tests` (8 targets); M1-M4; and `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json` (exit 0). |

## Risks and Trade-offs

- **Partial startup:** A listener can fail after other jobs have started. Mitigation: make `start()` own partial-startup cancellation and joining before it returns the source-preserving error.
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
