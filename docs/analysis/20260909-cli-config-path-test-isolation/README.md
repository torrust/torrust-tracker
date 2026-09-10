---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - tests/common/workspace.rs
    - tests/common/native_tracker.rs
    - src/app.rs
    - src/main.rs
    - packages/configuration/src/lib.rs
    - src/console/ci/e2e/runner.rs
---

# CLI Configuration-Path Test-Isolation Analysis

**Date:** 2026-09-09

**Status:** Current-code analysis; no implementation decision
**Question:** Where can tracker tests replace configuration base-source environment injection with `--config-toml-path` now that issue #2151 is merged?

## Executive Summary

Issue [#2151](https://github.com/torrust/torrust-tracker/issues/2151)
added `-c` / `--config-toml-path <PATH>` to the main tracker binary. It
selects one explicit TOML file before either base-source environment variable:

$$
\text{CLI path} > \text{TORRUST\_TRACKER\_CONFIG\_TOML} > \text{TORRUST\_TRACKER\_CONFIG\_TOML\_PATH} > \text{default file}
$$

Per-value `TORRUST_TRACKER_CONFIG_OVERRIDE_*` variables remain above every
base source.

The feature is immediately beneficial for **child-process** tests because a
child can receive its file through arguments without depending on inherited
configuration base-source variables. This is already the design of
`tests/common/native_tracker.rs`.

It is also beneficial for the main-level **in-process** integration fixture:
`tests/common/workspace.rs` already creates one isolated TOML file per suite,
but writes its path into the current test process environment before calling
`app::start()`. Changing the fixture to call
`app::start_with_explicit_config_toml_path(Some(workspace.config_path().to_path_buf()))`
would remove that base-source environment mutation and its synchronization
guard for all seven current functional fixture consumers.

That migration would **not** allow multiple tracker instances in one test
executable. `tracing`, deterministic clock state, and static secrets remain
process-global constraints documented in issue #1419 and `tests/AGENTS.md`.

The path-only option does not replace use cases that supply a complete TOML
string. Those remain candidates for a distinct future CLI capability only if a
real executable-boundary need arises.

## Issue-Spec Evidence

Issue [#1419](../../issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md)
identifies the original problem directly:

- tests in one process share environment variables;
- `std::env::set_var("TORRUST_TRACKER_CONFIG_TOML", ...)` allows concurrent
  tests to overwrite each other's configuration;
- per-test temporary workspaces and port-zero listeners address storage and
  socket conflicts, but not environment mutation.

At the time #1419 was written, the main binary had no configuration CLI. The
issue now contains an outdated historical note that describes the CLI as a
future improvement. #2151 supplies the path-only portion of that improvement.

The current #1419 fixture model still deliberately uses one tracker per Cargo
integration-test executable. Its current documentation correctly identifies
three remaining process-global constraints:

1. global `tracing` initialization;
2. static secrets and deterministic clock state; and
3. one in-process tracker lifecycle per executable.

Therefore this analysis recommends a fixture cleanup, not a change to #1419's
one-binary/one-tracker execution model.

## Configuration-Injection Inventory

The inventory searched Rust code for both base-source variable names and
current-process `set_var` / `remove_var` calls.

| Location                                                                   | Execution boundary                        | Current configuration mechanism                                                                                              | Whole TOML involved?                                     | CLI path useful?                          | Recommendation                                                                                                                                                                                                                               |
| -------------------------------------------------------------------------- | ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- | ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tests/common/workspace.rs`                                                | In-process main-level integration fixture | Mutates test-process `TORRUST_TRACKER_CONFIG_TOML_PATH`; clears/restores `TORRUST_TRACKER_CONFIG_TOML`; calls `app::start()` | No; fixture writes a per-suite file                      | Yes                                       | Highest-value migration candidate. Pass the existing workspace path to `app::start_with_explicit_config_toml_path`; remove `ConfigurationEnvironmentGuard` and `ENVIRONMENT_LOCK` after focused tests prove no base-source mutation remains. |
| Seven current functional `TrackerApplicationFixture` consumers             | In-process integration executables        | Indirectly use the shared fixture above                                                                                      | No                                                       | Yes, through shared fixture               | No per-suite migration should be necessary. Consumers: two UDP banning suites, the UDP disabled metrics suite, fixed-port metrics, port-zero metrics, and two UDP-error metrics suites. `scaffold` is the documented example fixture consumer. |
| `src/bootstrap/config.rs` tests                                            | In-process unit tests                     | Deliberately mutate both base-source variables under a module lock                                                           | Yes, for precedence coverage                             | Partly                                    | Keep environment mutation. These tests verify legacy environment precedence and override compatibility, which an explicit-path call would bypass. They are serialized and are not integration-fixture startup.                               |
| `tests/common/native_tracker.rs`                                           | Child process                             | `Command` passes `--config-toml-path`; test-specific environment values are child-only; inherited base sources are removed   | Optional environment TOML exists only to test precedence | Already used                              | No migration needed. This is the preferred executable-boundary pattern.                                                                                                                                                                      |
| `tests/configuration/cli_configuration/*` and `tests/lifecycle/signals.rs` | Child process via shared native fixture   | Use the fixture above                                                                                                        | Optional for precedence contract                         | Already used                              | No migration needed.                                                                                                                                                                                                                         |
| `src/console/ci/e2e/runner.rs`                                             | E2E runner to tracker container           | Runner accepts a file path or complete TOML, reads it, then injects complete TOML into the container environment             | Yes                                                      | Not directly                              | Separate container-runner design. The new main-binary argument is not automatically a container runtime argument; mounted file availability and argument forwarding would need explicit design.                                              |
| `src/console/profiling.rs`, docs, scripts, and historical evidence         | CLI/manual operational use                | Environment-selected file or content                                                                                         | Some commands use complete TOML                          | Sometimes, but not a test-parallelism fix | Do not migrate as part of test isolation. Review individually if operator-facing CLI examples are being modernized.                                                                                                                          |

### Main-Level Fixture Consumers

The shared in-process fixture currently serves these top-level test executables:

- `tests/banning/udp_metrics_disabled_port_zero.rs`;
- `tests/banning/udp_shared_connection_id_error_limit.rs`;
- `tests/banning/udp_shared_connection_id_error_limit_reverse_order.rs`;
- `tests/metrics/fixed_ports.rs`;
- `tests/metrics/port_zero.rs`;
- `tests/metrics/udp_error_disabled_port_zero.rs`;
- `tests/metrics/udp_error_enabled_port_zero.rs`; and
- `tests/scaffold.rs` as the documented example fixture consumer.

The seven functional suites receive the benefit from one change in
`tests/common/workspace.rs`; the `scaffold` example demonstrates the same API
for future suites without adding a functional test scenario.

## Why the Shared Fixture Is a Candidate

`EphemeralTrackerWorkspace::new` already does the work required by the CLI:

1. creates a unique `TempDir`;
2. creates a separate storage directory;
3. renders and writes `tracker-config.toml`; and
4. retains the file path for the fixture lifetime.

The current `start_tracker_with_config` then acquires an async lock, installs a
`ConfigurationEnvironmentGuard`, calls `app::start()`, and restores both base
source variables. Replacing only the startup call with the explicit-path API
would provide a typed input boundary and eliminate the global environment
mutation entirely.

A proposed focused change would have this shape:

```rust
let (container, jobs) = app::start_with_explicit_config_toml_path(Some(
    workspace.config_path().to_path_buf(),
))
.await
.expect("tracker application should start");
```

After test coverage confirms it, the fixture could remove:

- `ENVIRONMENT_LOCK`;
- `ConfigurationEnvironmentGuard`; and
- all `unsafe` `std::env::{set_var, remove_var}` calls from that helper.

This is beneficial even though the current fixture lock makes its environment
access deterministic: it removes a global mutable dependency, exposes the
selected source at the call site, and makes the fixture resilient to unrelated
configuration base-source values inherited by the test process.

## Boundaries and Non-Candidates

### Base-Source Tests Must Keep Environment Setup

The configuration package and bootstrap tests intentionally prove legacy
behavior for these inputs:

- complete configuration in `TORRUST_TRACKER_CONFIG_TOML`;
- path configuration in `TORRUST_TRACKER_CONFIG_TOML_PATH`; and
- the precedence between them, defaults, and per-value overrides.

Replacing these with the CLI path would stop testing the compatibility contract.
The appropriate improvement for these tests is continued locking and restoration
of test-process environment access, not migration away from it.

### Child-Process Environment Is Not Shared Test-Process State

A `tokio::process::Command` owns an environment map for one child. Setting
`TORRUST_TRACKER_CONFIG_TOML_PATH`, `TORRUST_TRACKER_CONFIG_TOML`, or an
override with `Command::env` does not mutate the parent test process. It is
therefore suitable for executable precedence tests, provided each child removes
inherited base sources and uses its own temporary workspace.

### Complete TOML Cannot Use the New Argument

`--config-toml-path` accepts a `PathBuf`; it intentionally does not accept TOML
content. A caller that has only an inline string has three choices:

1. retain `TORRUST_TRACKER_CONFIG_TOML` when environment configuration is the
   intended public interface;
2. materialize the string into a per-run temporary file and pass its path when
   the caller controls file lifecycle and needs process isolation; or
3. propose a separate CLI option for direct TOML content.

The second choice is appropriate only where writing a sensitive configuration
file is acceptable and the temporary-file permissions/lifetime are explicitly
designed. It is not a generic replacement for environment content, which can be
provided by an orchestrator without a file.

## Future Feature Assessment: Direct TOML CLI Option

A separate `--config-toml <TOML>` option could offer typed, process-local base
source selection without writing a temporary file. It would be different from,
and should not be bundled with, the #1419 fixture migration.

Potential benefits:

- child-process tests could pass generated TOML without a temporary file;
- launchers with in-memory configuration could avoid environment base sources;
- it could complete the symmetry between existing complete-TOML and path base
  sources.

Risks and design questions:

- shell argument length limits make full configuration content unreliable;
- command lines are commonly visible in process listings and diagnostics, so
  secrets in TOML would have a greater exposure risk than a protected file or
  carefully controlled environment;
- command-line precedence must remain exclusive with `--config-toml-path` and
  compatible with the existing environment precedence;
- error messages and logs must not expose TOML content; and
- Clap help, redaction, and test coverage would need the same care as #2151.

**Conclusion:** do not add direct TOML CLI input merely for the known
main-level integration fixture. That fixture already owns a valid temporary
file. Create a separate feature issue only if a concrete production or
executable-boundary caller needs in-memory configuration and can justify the
security and platform trade-offs.

## Recommended Follow-Up

A future, narrow follow-up to issue #1419 should:

1. replace environment-based startup in `tests/common/workspace.rs` with
   `app::start_with_explicit_config_toml_path` and the existing workspace file;
2. remove the fixture's environment lock and restoration guard;
3. retain base-source environment mutation in bootstrap/configuration tests;
4. run all top-level main-level integration targets together, preserving the
   current port-zero, workspace-isolation, and shutdown checks; and
5. update #1419 and `tests/AGENTS.md` to say in-process fixtures pass an
   explicit path rather than claiming they are environment-based.

This would reduce one known source of shared mutable state. It must not claim to
resolve #1419 completely or permit multiple trackers per test executable until
the remaining `tracing`, clock, secrets, and lifecycle constraints are resolved.

## Evidence and Search Scope

Reviewed on 2026-09-09 against merged #2151 on `develop`:

- issue [#1419](../../issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md), especially Problem 3 and current execution model;
- `tests/common/workspace.rs` and all `TrackerApplicationFixture::start` call sites;
- `tests/common/native_tracker.rs` and executable-boundary test targets;
- `src/app.rs` explicit-path startup boundary;
- `src/bootstrap/config.rs` legacy environment-precedence unit tests; and
- `src/console/ci/e2e/runner.rs` file-path/complete-TOML container runner.

Searches covered `TORRUST_TRACKER_CONFIG_TOML`,
`TORRUST_TRACKER_CONFIG_TOML_PATH`, configuration overrides, and current-process
`std::env::{set_var, remove_var}` uses in Rust production and test code.
