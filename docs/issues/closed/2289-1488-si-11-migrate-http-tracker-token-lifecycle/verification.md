# Verification Evidence — HTTP Tracker Token Lifecycle Migration

> **Status**: Complete as of 2026-09-22 on Linux with nightly Rust 1.100.0.

## Deterministic Tests

- `cargo test -p torrust-tracker-axum-http-server`: passed.
      Includes token-aware drain, registration-failure cleanup, and unchanged
      legacy start/stop coverage.
- `cargo test -p torrust-tracker bootstrap::jobs::http_tracker`: passed.
      Covers injected cancellation, independent server completion, and panicking
      server-task outcomes. Completion and failure tests hold the controller after
      it observes cancellation, prove the supervisor remains pending, then release
      it and assert the final component outcome.
- `cargo test -p torrust-tracker it_should_cancel_the_http_tracker_component_through_the_job_manager`: passed.
      Proves root-token cancellation reaches the named
      `http_instance_0_<address>` component without an OS signal.
- `cargo test -p torrust-tracker`: passed.

### Prose-First Test Review

The tested modules decide lifecycle ownership and component outcomes; registry
and HTTP serving details remain collaborator mechanics. Each changed test was
compared against its Arrange-Act-Assert prose before recording this evidence:

- Token-aware server drain: an available HTTP binding and injected token start
      the server; cancelling that token is the visible Act; the server launcher and
      `Drained` controller outcome are asserted.
- Token-aware registration failure: duplicate registry state is the causal
      initial condition; token-aware startup is the visible Act; the typed duplicate
      binding error and released listener are asserted.
- Independent completion and panic: a completed or panicking server task plus a
      controller held after cancellation is the causal state; supervision is the
      visible Act; the tests prove supervision remains pending until controller
      release and then assert completed or failed outcome.
- Bootstrap propagation: a configured HTTP tracker registered through
      `start_http_instance` is the causal state; `JobManager::cancel` is the visible
      Act; the named cancelled HTTP component outcome is asserted.

These tests keep behavior-selecting state, production actions, and independently
specified outcomes visible. Legacy start/stop coverage remains unchanged and
continues to test the separate compatibility API.

## Compatibility and Ownership Review

- `HttpServer::start` and `HttpServer::stop` remain the unchanged legacy path;
      their compatibility test passes.
- The token-aware path calls `graceful_shutdown_on_cancellation` and does not
      subscribe to operating-system signals.
- The HTTP component owns both direct children. It cancels and joins the drain
      controller before reporting cancellation, independent completion, or runtime
      failure.

## Manual Evidence

Direct binary PID shutdown and HTTP listener rebind evidence is recorded in
[manual-verification-evidence.md](manual-verification-evidence.md). The direct
tracker process exited with status `0` in two consecutive SIGTERM runs and the
logs recorded the token-aware drain path.

The migrated executable has no configuration switch for its retained legacy
server API. Legacy compatibility is therefore automatic coverage, not a manual
scenario.

## Quality Gates

- `linter all`: passed.
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`:
      passed all eight checks.
