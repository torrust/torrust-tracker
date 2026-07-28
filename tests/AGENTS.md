---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - tests/stats.rs
    - tests/servers/
    - src/app.rs
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
  issue-spec: docs/issues/drafts/increase-main-app-integration-test-coverage.md
---

# Integration Tests — AI Agent Guidelines

## Purpose

This directory contains **main application-level integration tests**. These tests verify behavior
that can only be tested by running the complete Torrust Tracker application with multiple services
coordinated through the application container.

## What Belongs Here

Integration tests at this level should focus on **application-level concerns**:

- **Multiple tracker instances**: Running HTTP and UDP trackers simultaneously on different ports
- **Global metrics aggregation**: Metrics that aggregate data across all running tracker instances
- **Application container lifecycle**: Container initialization, service registration, shutdown coordination
- **Job manager orchestration**: Background jobs interacting with multiple services
- **Cross-service coordination**: Interactions between HTTP API, trackers, and core services
- **Bootstrap and configuration**: Application startup with complex multi-service configurations
- **Health check aggregation**: Health status across all registered services

## What Does NOT Belong Here

Most tests should be in the corresponding `packages/*/tests/` directories:

- **Single-service behavior**: Test HTTP tracker logic in `packages/axum-http-server/tests/`
- **Protocol parsing**: Test in `packages/http-protocol/tests/` or `packages/udp-protocol/tests/`
- **Core tracker logic**: Test in `packages/tracker-core/tests/`
- **Database operations**: Test in `packages/swarm-coordination-registry/tests/`
- **API endpoints**: Test in `packages/axum-rest-api-server/tests/`
- **Individual component behavior**: Always prefer package-level tests for isolated components

**Guideline**: If the test can be written at the package level, it should be. Only use main-level
integration tests when you genuinely need the full application context.

## Execution Model

Each top-level Rust source file in `tests/` is a **separate Cargo integration-test
executable** (and therefore a separate operating-system process). A single test
executable manages **one tracker application instance** with a fixed initial
configuration. Scenario functions run sequentially against that instance.

A different initial configuration requires a separate top-level file. For example:

| File                 | Purpose                                                  |
| -------------------- | -------------------------------------------------------- |
| `tests/stats.rs`     | Global statistics suite (public tracker, two HTTP nodes) |
| `tests/scaffold.rs`  | Scaffolding demo — same pattern, isolated process        |
| `tests/bootstrap.rs` | _(future)_ Bootstrap/shutdown lifecycle scenarios        |

Cargo may run these binaries in parallel. Each binary binds to port `0` (OS-assigned
ephemeral ports), uses its own `TempDir` workspace, and sets
`TORRUST_TRACKER_CONFIG_TOML_PATH` only in its own process, so no conflict occurs.

## Test Infrastructure Requirements

All integration tests at this level must:

1. **Use port `0` for all bind addresses**: The OS assigns free ephemeral ports, preventing
   conflicts when tests run in parallel
2. **Use isolated temporary workspaces**: Use `tempfile::TempDir` to create
   isolated directories with separate config files and storage subdirectories
3. **Extract actual bound ports**: Query `AppContainer`'s `Registar` to get the OS-assigned ports
   for making requests
4. **Be independent**: Each top-level test binary must be able to run in isolation or concurrently
   with others (it is the binary, not the function, that is the unit of isolation)
5. **Clean up resources**: Use RAII patterns (temp dirs, handles) for automatic cleanup

## Current Test Structure

```text
tests/
├── AGENTS.md                    # This file
├── common/
│   └── mod.rs                   # Shared test utilities (temp config, port extraction)
├── integration.rs               # Global statistics suite (main integration tests)
├── scaffold.rs                  # Scaffolding demo — pattern reference for new binaries
└── servers/
    └── api/
        └── contract/
            └── stats/
                └── mod.rs       # Global statistics test scenarios
```

## Adding a New Integration-Test Binary

1. **Confirm it belongs here**: Can this test be written at the package level? If yes, write it there.
2. **Determine the initial configuration**: If your scenarios need a different tracker
   configuration than the existing suite, create a new top-level file (e.g., `tests/bootstrap.rs`).
   If they share the same configuration, add scenarios to the existing suite.
3. **Reuse shared utilities**: Import `mod common;` and use the helpers in `tests/common/mod.rs`
   for workspace setup, tracker startup, and port discovery.
4. **Use port `0`**: All services must bind to port `0` in test configurations.
5. **Extract bound ports**: Query the registar or `AppContainer` to discover actual socket addresses.
6. **Document the purpose**: Add clear doc comments explaining what application-level behavior is
   being tested.
7. **Reference existing code**: See `tests/scaffold.rs` for a minimal working example of a new
   integration-test binary, or `tests/servers/api/contract/stats/mod.rs` for the scenario pattern.

For concrete examples, see the existing tests in `tests/servers/` — they serve as the canonical
reference for the integration test pattern.

## References

- [Issue #1419](../../docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md) - Infrastructure for parallel integration tests (execution model decision)
- [Integration test scaffolding](stats.rs)
- [Shared test utilities](common/mod.rs)
- [Scaffolding demo](scaffold.rs)
