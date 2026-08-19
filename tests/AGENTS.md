---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - tests/metrics/port_zero.rs
    - tests/metrics/fixed_ports.rs
      - tests/common/mod.rs
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

| File                                              | Purpose                                                     |
| ------------------------------------------------- | ----------------------------------------------------------- |
| `tests/metrics/port_zero.rs`                      | Port-zero aggregate metrics and duplicate-instance identity |
| `tests/metrics/fixed_ports.rs`                    | Fixed-port aggregate metrics and routing                    |
| `tests/metrics/udp_error_*.rs`                    | Enabled/disabled UDP cookie-error metric policy             |
| `tests/banning/udp_metrics_disabled_port_zero.rs` | Disabled-listener banning metric                            |
| `tests/scaffold.rs`                               | Scaffolding demo — same pattern, isolated process           |

Each binary defines a single `#[tokio::test]` runner that starts the tracker
once, then calls scenario functions sequentially. Scenario functions are plain
async functions that receive the `AppContainer` and assert behavior.

### Scenario Design

- Follow Arrange, Act, Assert (AAA) visibly in every scenario.
- Give each scenario one observable contract and one reason to fail. Do not
  combine metric filtering, protocol responses, and ban enforcement in one
  scenario merely because they share setup.
- Default to port-zero configurations. They exercise parallel-safe bindings,
  duplicate configured addresses, and canonical runtime identity together.
- Keep fixed-port binaries only for behavior that specifically depends on an
  explicit configured address and binding.
- Add a top-level binary even when its initial configuration is similar if an
  existing binary's shared aggregate metrics or security state would make the
  scenario depend on prior traffic. A separate binary supplies a fresh process,
  repositories, and ban service.

Cargo may run these binaries in parallel. Each binary binds to port `0`
(OS-assigned ephemeral ports) by default, uses its own `TempDir` workspace,
and sets `TORRUST_TRACKER_CONFIG_TOML_PATH` only in its own process, so no
conflict occurs. Fixed-port binaries (e.g., `metrics-fixed-ports`)
use distinct non-overlapping ports and must not run concurrently with other
binaries that use the same ports.

### Why one binary per configuration?

The 1:1 mapping between integration-test binaries and tracker configurations
exists because the current application startup has several global side effects
that prevent running multiple isolated tracker instances in the same process:

1. **`tracing` global initialization** (main blocker): The `tracing` crate
   initializes a global subscriber. Once set, it cannot be reset for a second
   tracker instance in the same process. This means two tracker applications
   sharing a process would share logging state and configuration.
2. **Environment-variable config injection**: The tracker reads its
   configuration from the `TORRUST_TRACKER_CONFIG_TOML_PATH` environment
   variable. Multiple tracker instances in the same process would race on
   this variable.
3. **Static secrets and clock state**: Values like seed secrets and the
   deterministic test clock are process-global. While these could be refactored
   into injected dependencies, the tracing global subscriber remains the
   fundamental blocker.

Until these global side effects are eliminated (tracked in
[#1430](https://github.com/torrust/torrust-tracker/issues/1430)), each
integration-test binary must start exactly one tracker instance with one fixed
configuration. Scenario functions run sequentially against that shared instance.

## Test Infrastructure Requirements

All integration tests at this level must:

1. **Use port `0` for bind addresses by default**: The OS assigns free ephemeral ports,
   preventing conflicts when tests run in parallel. Fixed ports are permitted when the
   test scenario specifically requires distinct addresses (e.g., verifying per-instance
   behavior). Use non-overlapping port ranges and document the constraint.
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
├── AGENTS.md                         # This file
├── common/
│   ├── configuration.rs              # Shared integration-test configurations
│   ├── mod.rs                        # Re-exports from submodules
│   ├── workspace.rs                  # Tracker workspace setup and URL discovery
│   └── statistics.rs                 # Aggregate statistics query helpers
├── banning/
│   └── udp_metrics_disabled_port_zero.rs # Disabled-listener ban statistics
├── metrics/
│   ├── fixed_ports.rs                # Fixed-port metrics and routing
│   ├── port_zero.rs                  # Port-zero metrics and identity
│   ├── udp_error_disabled_port_zero.rs # Disabled-listener error filtering
│   └── udp_error_enabled_port_zero.rs # Enabled-listener error metrics
└── scaffold.rs                       # Scaffolding demo — pattern reference for new binaries
```

## Adding a New Integration-Test Binary

1. **Confirm it belongs here**: Can this test be written at the package level? If yes, write it there.
2. **Determine the initial configuration**: If your scenarios need a different tracker
   configuration than the existing suite, create a new explicit Cargo test target
   (e.g., `tests/metrics/fixed_ports.rs`). If they share the same configuration, add
   scenarios to the existing suite's runner function.
3. **Reuse shared utilities**: Import `mod common;` and use the helpers in `tests/common/mod.rs`
   for workspace setup, tracker startup, and port discovery.
4. **Use port `0` by default**: Bind services to port `0` unless the scenario specifically
   requires distinct fixed addresses.
5. **Extract bound ports**: Query the registar or `AppContainer` to discover actual socket addresses.
6. **Document the purpose**: Add clear doc comments explaining what application-level behavior is
   being tested.
7. **Reference existing code**: See `tests/metrics/fixed_ports.rs` for the canonical
   pattern: one `#[tokio::test]` runner, one config constant, scenario functions that receive
   the `AppContainer`.

## References

- [Issue #1419](../../docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md) - Infrastructure for parallel integration tests (execution model decision)
- [Integration test scaffolding](metrics/port_zero.rs)
- [Shared test utilities](common/mod.rs)
- [Scaffolding demo](scaffold.rs)
