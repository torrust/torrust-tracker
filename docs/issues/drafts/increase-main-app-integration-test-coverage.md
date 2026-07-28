---
doc-type: issue
issue-type: enhancement
status: draft
priority: p3
github-issue: null
spec-path: docs/issues/drafts/increase-main-app-integration-test-coverage.md
branch: null
related-pr: null
last-updated-utc: 2026-07-27 12:00
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - tests/stats.rs
    - tests/AGENTS.md
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level.md
  related-issues:
    - 1347
    - 1419
---

# Draft Issue - Increase Main Application-Level Integration Test Coverage

## Goal

Systematically expand integration test coverage at the main application level (`tests/`) to verify
application-level behaviors that can only be tested with the complete Torrust Tracker application
and multiple coordinated services.

## Background

The Torrust Tracker project uses a three-layer testing strategy:

1. **Unit tests** (`packages/*/tests/`) — Fast, isolated tests for individual components
2. **Integration tests** (`tests/`) — Main application-level tests with full app context
3. **E2E tests** (`packages/e2e-tools/`, `src/console/ci/e2e/`, `src/console/ci/qbittorrent_e2e/`)
   — Container-based tests with Docker Compose

After implementing issue #1419 (parallel integration test infrastructure), the project has a
foundation for writing independent, concurrent integration tests at the main application level.
Currently, only one test suite exists (`tests/servers/api/contract/stats/`), which verifies global
metrics aggregation across multiple tracker instances.

This issue tracks the expansion of **integration test coverage** (layer 2) for application-level
concerns that cannot be tested at the package level:

- Multiple tracker instances running simultaneously
- Cross-service coordination and metrics aggregation
- Application container lifecycle and job orchestration
- Health check aggregation across all services
- Bootstrap and configuration integration
- Graceful shutdown coordination

### Relationship to EPIC #1347 and Testing Layers

This issue complements [EPIC #1347 - Increase unit testing for workspace
packages](https://github.com/torrust/torrust-tracker/issues/1347).

| Layer           | Location                                                         | Focus                                    | EPIC/Issue |
| --------------- | ---------------------------------------------------------------- | ---------------------------------------- | ---------- |
| **Unit tests**  | `packages/*/tests/`                                              | Individual component behavior            | EPIC #1347 |
| **Integration** | `tests/` (main app-level)                                        | Application-level coordination           | This issue |
| **E2E tests**   | `packages/e2e-tools/`, `src/console/ci/e2e/`, `qbittorrent_e2e/` | Container-based cross-process validation | (separate) |

All three layers are part of a broader effort to improve overall test coverage and reliability.

## Scope

### In Scope

- Integration tests that require the full application context (`app::run()`)
- Tests that verify behavior across multiple coordinated services
- Tests that verify application container initialization and lifecycle
- Tests that verify job manager orchestration and background tasks
- Tests for global metrics, health checks, and cross-service coordination
- Tests that run in parallel without port conflicts (using port `0` and temp config)

### Out of Scope

- **Package-level unit tests** — belongs in `packages/*/tests/` (covered by EPIC #1347)
- **E2E tests using Docker Compose** — belongs in `packages/e2e-tools/`, `src/console/ci/e2e/`,
  and `src/console/ci/qbittorrent_e2e/` (runs against containerized tracker with external clients)
- **Protocol parsing tests** — belongs in `packages/http-protocol/tests/` or
  `packages/udp-protocol/tests/`
- **Single-service behavior tests** — belongs in corresponding server package tests
- **Database-only tests** — belongs in `packages/swarm-coordination-registry/tests/`

**Guideline**: If a test can be written at the package level, it should be. Only add integration
tests when the full application context is genuinely required. If a test requires Docker Compose
orchestration or external BitTorrent clients, it belongs in the E2E layer.

## Prioritized Test List

### High Priority

1. **Multiple trackers with different protocols**  
   Verify HTTP and UDP trackers run simultaneously, handle announces independently, and contribute
   to separate metrics.

2. **Health check aggregates all services**  
   Verify health check API returns status for all registered services (HTTP API, HTTP trackers, UDP
   trackers).

3. **Torrent cleanup job with active trackers**  
   Run the cleanup job while trackers are handling announces; verify it removes inactive peers
   without interfering with active announces.

4. **Global scrape across multiple trackers**  
   Send scrape requests to multiple HTTP tracker instances and verify the responses reflect the
   correct swarm state.

5. **Metrics counters across HTTP and UDP**  
   Verify that announce counters aggregate correctly when requests come to both HTTP and UDP
   trackers.

### Medium Priority

1. **Graceful shutdown coordination**  
   Start all services, send requests, trigger shutdown, verify all services stop cleanly without
   dropping active connections.

2. **Job manager handles job failures**  
   Trigger a job failure; verify the job manager restarts or reports the failure without crashing
   the application.

3. **Concurrent announce load across multiple trackers**  
   Send simultaneous announces to multiple tracker instances; verify correct peer aggregation and
   no race conditions.

4. **Activity metrics updater job**  
   Verify the activity metrics updater job correctly processes peer activity and updates global
   stats across all running services.

5. **Event listener coordination**  
   Verify event listeners for different services process events without interference when multiple
   services emit events simultaneously.

### Low Priority

1. **Container dependency validation**  
   Verify the application refuses to start with invalid service combinations or detects
   configuration conflicts at bootstrap.

2. **Application bootstrap with minimal configuration**  
   Start the application with minimal required config; verify all default services initialize
   correctly.

3. **Multiple database backends**  
   Start the application with SQLite, MySQL, and PostgreSQL configurations; verify the bootstrap
   process correctly initializes each database backend and all services start without errors.

4. **Service registration completeness**  
   Verify all configured services register correctly in the Registrar with their actual bound
   addresses and metadata.

## Implementation Plan

This is a tracking issue. Each test case should be implemented as a subtask or separate small issue.

Suggested approach:

1. Start with high-priority tests (tests 1-5)
2. Implement one test per PR to keep changes reviewable
3. Follow the test pattern established in issue #1419
4. Use test utilities from `tests/helpers.rs` (temp config, port extraction)
5. Ensure all tests use port `0` and temporary configuration files
6. Document test purpose with clear doc comments

## Acceptance Criteria

- [ ] AC1: All high-priority tests (tests 1-5) are implemented and passing
- [ ] AC2: Test utilities in `tests/helpers.rs` are expanded as needed for common patterns
- [ ] AC3: All new tests run in parallel without conflicts (port `0`, temp config)
- [ ] AC4: Each test has clear documentation explaining what application-level behavior is verified
- [ ] AC5: `linter all` passes
- [ ] AC6: All tests pass in CI

## Verification Plan

### Automatic Checks

- `linter all` exits with code `0`
- `cargo test --test stats` passes all new integration tests
- `cargo test --workspace` passes (no regressions)
- CI pipeline passes with new tests running in parallel

### Manual Checks

| ID  | Check                         | Expected Outcome                                                   |
| --- | ----------------------------- | ------------------------------------------------------------------ |
| M1  | Run `cargo test --test stats` | All integration tests pass, no port conflicts or config collisions |
| M2  | Run with `RUST_LOG=debug`     | Verify multiple services log startup without errors                |
| M3  | Review test execution time    | Integration tests complete faster than equivalent E2E tests        |

## Dependencies

- Issue #1419 must be completed (infrastructure for parallel integration tests)

## Related Issues

- [Issue #1419 - Allow multiple integration tests at the main app
  level](../open/1419-allow-multiple-integration-tests-at-main-app-level.md) - Infrastructure
  foundation
- [EPIC #1347 - Increase unit testing for workspace
  packages](https://github.com/torrust/torrust-tracker/issues/1347) - Package-level unit test
  coverage

## References

### Integration Test Infrastructure

- [tests/AGENTS.md](../../../tests/AGENTS.md) - Guidelines for main-level vs package-level tests
- [tests/stats.rs](../../../tests/stats.rs) - Integration test scaffolding
- [tests/servers/api/contract/stats/](../../../tests/servers/api/contract/stats/) - Current global
  stats test example

### Testing Strategy Documentation

- [.github/skills/dev/testing/write-unit-test/SKILL.md](../../../.github/skills/dev/testing/write-unit-test/SKILL.md)
  \- Unit testing conventions and Test Desiderata principles
- [docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md](../../adrs/20260603000000_keep_unit_tests_inside_container_build.md)
  \- ADR documenting the three-layer testing strategy (GHA unit tests, in-container unit tests,
  E2E tests)
- [packages/e2e-tools/README.md](../../../packages/e2e-tools/README.md) - E2E test runners
  (`e2e_tests_runner`, `qbittorrent_e2e_runner`)

**Note**: There is currently no comprehensive testing strategy document in `docs/`. Testing
guidance is distributed across skills, ADRs, and package README files. A future improvement
could consolidate this into a canonical `docs/testing.md` document.

## Progress Tracking

### Completion Checklist

High-priority tests:

- [ ] Test 1: Multiple trackers with different protocols
- [ ] Test 2: Health check aggregates all services
- [ ] Test 3: Torrent cleanup job with active trackers
- [ ] Test 4: Global scrape across multiple trackers
- [ ] Test 5: Metrics counters across HTTP and UDP

Medium-priority tests:

- [ ] Test 6: Graceful shutdown coordination
- [ ] Test 7: Job manager handles job failures
- [ ] Test 8: Concurrent announce load across multiple trackers
- [ ] Test 9: Activity metrics updater job
- [ ] Test 10: Event listener coordination

Low-priority tests tracked separately when high/medium priorities are complete.

### Progress Log

- 2026-07-27 12:00 UTC - agent - Created draft issue to track integration test coverage expansion
  after #1419 infrastructure implementation.
