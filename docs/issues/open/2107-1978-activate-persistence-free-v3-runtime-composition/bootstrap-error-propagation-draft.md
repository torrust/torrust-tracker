---
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/open/2107-1978-activate-persistence-free-v3-runtime-composition/bootstrap-error-propagation-draft.md
branch: "{issue-number}-propagate-bootstrap-errors"
related-pr: null
depends-on:
  - 2107
last-updated-utc: 2026-08-28 11:58
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2107-1978-activate-persistence-free-v3-runtime-composition/ISSUE.md
    - src/AGENTS.md
    - src/bootstrap/app.rs
    - src/bootstrap/config.rs
    - src/bootstrap/persistence.rs
    - src/container.rs
    - src/app.rs
    - src/main.rs
---

# Draft - Propagate bootstrap startup errors

> **Deferred follow-up:** Do not create this GitHub issue until #2107 is
> complete. #2107's persistence-free composition work may change the final
> application-container error boundary and should supply the concrete failures
> this refactor must represent.

## Goal

Make expected startup, configuration, and composition failures explicit typed
errors that bubble from bootstrap to the executable boundary, where they are
reported with context and terminate the process with a nonzero exit status.

## Background

The configuration source APIs already expose fallible operations:
`Info::new`, `Configuration::load`, semantic validation, and bootstrap
persistence-requirement validation each return errors. Current startup code
converts these expected operator-facing failures to `expect` or `panic` in
`initialize_configuration` and `setup`. Application-container composition also
assumes successful initialization.

Fail-fast startup remains the intended operational behavior, but panicking
hides the typed cause at intermediate boundaries and makes individual failure
paths harder to test. A `Result`-based bootstrap boundary will show what can
fail, preserve error context, and give executable entrypoints one consistent
way to report startup failure.

## Scope

### In Scope

- Return typed errors from configuration source creation and loading instead of
  panicking in `initialize_configuration`.
- Define a focused bootstrap error type that retains source errors from
  configuration loading, semantic validation, persistence-requirement
  validation, and application-container composition.
- Change bootstrap `setup()` to return a `Result` and propagate it through
  `app::run()` and executable entrypoints, including profiling or test helpers
  where their startup contracts require it.
- Refactor application-container initialization to return a contextual typed
  error once #2107 establishes the persistence-enabled and persistence-free
  composition paths.
- Report startup failures at the executable boundary with useful context and a
  nonzero exit status.
- Add focused tests for representative configuration, validation, and
  composition failures without binding listener ports or creating unrelated
  runtime services.

### Out of Scope

- Changing `check_seed()` from its assertion-based internal cryptographic
  invariant. It is not operator configuration input.
- Treating asynchronous failures after successful startup as bootstrap results.
- Reclassifying operational database failures as configuration validation
  errors.
- Making every server bind, TLS, job lifecycle, or shutdown path transactional
  unless the final typed startup boundary requires a narrowly scoped cleanup.
- Implementing this refactor as part of #2107.

## Proposed Design

1. Let `initialize_configuration()` return a configuration-load `Result`.
2. Introduce `bootstrap::app::Error` with variants that preserve configuration
   source, semantic validation, persistence requirement, and composition
   causes.
3. Let `setup()` return `Result<(Configuration, AppContainer), Error>`.
4. Propagate the error through `app::run()` to each executable boundary.
5. At the executable boundary, format the error for operators and exit
   unsuccessfully while leaving internal invariants as assertions.

The exact error types and whether `run()` has a distinct wrapper error must be
decided after #2107's final container composition is known. Avoid converting
errors to strings early: callers and tests need access to the original error
category.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                            | Notes / Expected Output                                                            |
| --- | ------ | ------------------------------- | ---------------------------------------------------------------------------------- |
| T1  | TODO   | Reconcile after #2107           | Identify final fallible container and startup boundaries from #2107's merged code. |
| T2  | TODO   | Define typed bootstrap errors   | Preserve original sources and stable contextual categories.                        |
| T3  | TODO   | Propagate setup and run results | Update startup callers and executable error reporting.                             |
| T4  | TODO   | Add focused failure-path tests  | Exercise source, semantic, requirement, and composition failures.                  |
| T5  | TODO   | Document final startup contract | Update `src/AGENTS.md` and operator-facing docs if the policy changes.             |

## Acceptance Criteria

- [ ] Expected configuration, validation, and composition failures are returned
      as typed errors through bootstrap rather than panicking.
- [ ] Error chains retain their original source category and meaningful
      bootstrap context.
- [ ] The executable boundary reports startup failure and exits nonzero.
- [ ] `check_seed()` remains an assertion for its internal invariant.
- [ ] Focused tests cover representative failures and prove no unrelated
      services begin after bootstrap failure.
- [ ] Documentation describes the resulting startup-error contract truthfully.

## Verification Plan

- Focused bootstrap and application tests for each error category.
- Entry-point tests for contextual reporting and nonzero termination where the
  test harness permits it.
- `cargo fmt`, relevant package tests, `linter all`, and the required
  repository quality gates.

## Progress Tracking

### Workflow Checkpoints

- [x] Deferred draft recorded while implementing #2107.
- [ ] #2107 completed and final composition error boundaries reviewed.
- [ ] Draft refined and approved for GitHub issue creation.
- [ ] GitHub issue created and linked to its final parent or related work.

### Progress Log

- 2026-08-28 11:58 UTC - GitHub Copilot/User - Recorded this deferred draft
  after observing expected startup failures converted to `expect` or `panic`.
  The draft must be reconciled after #2107; it is not current implementation
  scope.

## References

- Current implementation issue: #2107
- Startup policy: `src/AGENTS.md`
- Configuration bootstrap: `src/bootstrap/config.rs`
- Bootstrap composition: `src/bootstrap/app.rs`
- Application startup: `src/app.rs`
- Executable entrypoint: `src/main.rs`
