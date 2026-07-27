---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - tests/integration.rs
    - tests/servers/
    - src/app.rs
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level.md
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

## Test Infrastructure Requirements

All integration tests at this level must:

1. **Use port `0` for all bind addresses**: The OS assigns free ephemeral ports, preventing conflicts
   when tests run in parallel
2. **Use isolated temporary workspaces**: Never use `std::env::set_var()` for configuration —
   tests run concurrently and would overwrite each other's config. Use `tempfile::TempDir` to create
   isolated directories with separate config files and storage subdirectories
3. **Extract actual bound ports**: Query `AppContainer` or `Registar` to get the OS-assigned ports
   for making requests
4. **Be independent**: Each test must be able to run in isolation or concurrently with others
5. **Clean up resources**: Use RAII patterns (temp dirs, handles) for automatic cleanup

## Current Test Structure

```text
tests/
├── AGENTS.md                    # This file
├── integration.rs               # Test scaffolding and module declarations
├── helpers.rs                   # Shared test utilities (temp config, port extraction)
└── servers/
    └── api/
        └── contract/
            └── stats/
                └── mod.rs       # Global statistics tests
```

## Future Test Coverage

For a prioritized list of valuable integration tests to add at the main application level, see the
[draft issue for increasing integration test
coverage](../docs/issues/drafts/increase-main-app-integration-test-coverage.md).

This draft issue tracks 14 planned integration tests organized by priority (high/medium/low) and
complements [EPIC #1347](https://github.com/torrust/torrust-tracker/issues/1347) which focuses on
unit test coverage for workspace packages.

## Adding a New Integration Test

1. **Confirm it belongs here**: Can this test be written at the package level? If yes, write it there.
2. **Use test utilities**: Import helpers from `tests/helpers.rs` for temp config and port extraction
3. **Use port `0`**: All services must bind to port `0` in test configurations
4. **Extract bound ports**: Query `AppContainer.registar` or job handles to get actual socket addresses
5. **Make it independent**: Test must not depend on execution order or side effects from other tests
6. **Document the purpose**: Add a clear doc comment explaining what application-level behavior is
   being tested

For concrete examples, see the existing tests in `tests/servers/` — they serve as the canonical
reference for the integration test pattern.

## References

- [Issue #1419](../../docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level.md) - Infrastructure for parallel integration tests
- [Integration test scaffolding](integration.rs)
- [Test helpers](helpers.rs)
