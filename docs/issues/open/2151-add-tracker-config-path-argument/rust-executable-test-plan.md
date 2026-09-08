---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - docs/issues/open/2151-add-tracker-config-path-argument/ISSUE.md
    - docs/testing.md
    - tests/AGENTS.md
    - tests/common/native_tracker.rs
    - tests/configuration/cli_configuration.rs
    - tests/lifecycle/signals.rs
    - .github/skills/dev/testing/write-unit-test/SKILL.md
---

# Rust Executable-Test Completion Plan

## Purpose

Replace the issue-local Python release CLI verification harness with maintained
Rust tests before completing issue #2151. The Python harness records useful
release-binary scenarios, but repository tests must be Rust. Do not add Python
for test code in this repository.

The existing Rust configuration and bootstrap tests remain the lowest-cost tests
for source-selection semantics. This plan adds only the executable-boundary
coverage that the Python harness currently supplies, then removes the Python
artifact after its behavior is preserved.

## Test-Layer Decisions

The reusable native tracker fixture belongs in `tests/common/`, not the
signal-specific `tests/lifecycle/` directory. The executable configuration
scenarios belong in `tests/configuration/`, while `tests/lifecycle/` retains only
operating-system signal behavior:

```text
tests/
├── common/
│   └── native_tracker.rs       # Child process, workspace, output, readiness, cleanup
├── configuration/
│   ├── cli_configuration.rs    # Cargo test entry point and shared fixture import
│   └── cli_configuration/
│       ├── base_source_precedence.rs # CLI versus environment base sources
│       ├── per_value_overrides.rs    # Override versus CLI file
│       └── invalid_sources.rs        # Planned CLI source failure contracts
└── lifecycle/
    └── signals.rs              # SIGINT, SIGTERM, and drop-path contracts
```

Each top-level test source remains a separate Cargo integration-test executable.
Both `configuration/cli_configuration.rs` and `lifecycle/signals.rs` include the
shared fixture through `#[path = "../common/native_tracker.rs"] mod native_tracker;`.
Within the configuration target, modules group scenarios by the configuration
contract they prove; the Cargo entry point owns the shared fixture import.

| Behavior                                                                | Test layer                               | Location                                   | Reason                                                                                                                                                                                    |
| ----------------------------------------------------------------------- | ---------------------------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI parsing, usage errors, and exit code `2`                            | Executable-boundary integration          | `tests/configuration/cli_configuration.rs` | A parser unit test cannot prove the compiled tracker process exits without starting.                                                                                                      |
| CLI path wins over inherited complete-TOML and path environment sources | Executable-boundary integration          | `tests/configuration/cli_configuration.rs` | Configuration and bootstrap tests already prove selection; a child process proves the main binary carries the parsed CLI path through all layers.                                         |
| Per-value override wins over a CLI base file                            | Executable-boundary integration          | `tests/configuration/cli_configuration.rs` | Configuration tests already prove the merge; a child process covers the user-facing process contract.                                                                                     |
| Missing, directory, malformed, and parent-only-relative CLI sources     | Executable-boundary integration          | `tests/configuration/cli_configuration.rs` | Must prove process exit `1`, contextual diagnostics, and no listener at the main-binary boundary.                                                                                         |
| Unreadable regular-file CLI source                                      | Unix executable-boundary integration     | `tests/configuration/cli_configuration.rs` | Permission semantics are platform-specific. Gate it on Unix and avoid assuming privileged runners cannot read mode-`000` files.                                                           |
| Two independent CLI-configured tracker children                         | Existing executable-boundary integration | `tests/lifecycle/signals.rs`               | Already covered; retain as the regression for process, workspace, storage, port-zero, and shutdown isolation.                                                                             |
| Release-profile artifact behavior                                       | Manual release verification              | Concise commands/evidence in `ISSUE.md`    | Cargo test binaries are the maintained automated regression layer. Building and executing `target/release` belongs to final manual validation, not a second scripted test implementation. |

Container E2E is not required: this feature changes argument parsing and native
startup selection, not container composition or BitTorrent-client
interoperability. The container receives arguments through the existing
entrypoint forwarding contract and is covered by container-image CI.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                  | Expected result                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| --- | ------ | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| R1  | DONE   | Extract common native fixture         | Moved `tests/lifecycle/native_tracker.rs` to `tests/common/native_tracker.rs` and updated signal tests to import it explicitly. Child-process, workspace, output-draining, absolute-deadline, normal-shutdown, and drop-path cleanup behavior is unchanged. `cargo test --test lifecycle-signals` passed (8 tests).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| R2  | DONE   | Add configuration CLI test target     | Added and registered `tests/configuration/cli_configuration.rs` with an executable CLI-precedence scenario. It imports the common fixture through an explicit path module declaration and asserts a live configuration contract rather than duplicating signal behavior. `cargo test --test cli-configuration` passed.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| R3  | DONE   | Add CLI precedence process tests      | Added independent executable-boundary tests for CLI precedence over both child-only base environment sources and for a child-only health-check override over the CLI file. Each waits for the selected endpoint and uses fixture-owned graceful cleanup. `cargo test --test cli-configuration` passed (7 tests).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| R3a | DONE   | Prove base-source exclusivity         | Added a configuration-package unit test where the ignored complete-TOML and environment-path sources each provide a distinct optional section absent from the explicit file. It asserts neither section appears in the loaded configuration, proving base sources are exclusive rather than merely resolved by conflicting values. `cargo test --package torrust-tracker-configuration --lib` passed (129 tests).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| R4  | DONE   | Add invalid-source process matrix     | Added `invalid_sources.rs` with compiled-child contracts for missing/empty option values, missing file, directory, malformed TOML, and parent-only relative path. They assert real exit codes and stable diagnostic fragments; malformed and parent-only sources prove their candidate ports are bindable after child reaping. The expected-failure fixture bounds waiting, forced reaping, and output draining. `cargo test --test cli-configuration` passed (15 tests).                                                                                                                                                                                                                                                                                                                                                                                                                              |
| R5  | DONE   | Add Unix unreadable-file process test | Added a fixture-owned valid mode-`000` regular file scenario. When permissions are enforced, the compiled child exits `1` with a path-bearing permission diagnostic and leaves its candidate port bindable after reaping; privileged runners report an explicit skip without spawning a child. Permission restoration is fallible in normal cleanup, non-panicking in drop cleanup, and verified after `wait_for_exit`. `cargo test --test cli-configuration` passed (17 tests).                                                                                                                                                                                                                                                                                                                                                                                                                       |
| R6  | DONE   | Review test design increment          | Maintainer review of `invalid_sources.rs` found multi-contract asserts, an unreadable-file test that mixed fixture checks into the Assert step, and the suite's only fixed loopback ports (43157-43159). Resolved by: fixture assertion helpers (`assert_usage_error`, `assert_startup_failure`, `assert_diagnostic_names_source_path`) that print child output on failure; one contract per test; `enforced_or_report_skip()` so the unreadable-file test reads as plain AAA; moving permission-restoration and drop-without-runtime checks into the fixture's unit tests; dropping the candidate-port probe (the bounded exit wait already proves no start) so every configuration uses port zero; and separating `NativeTrackerStartAttempt` preparation from `.start()` so Arrange does not launch the child process. `cargo test --test cli-configuration` passed (18 tests, 3 consecutive runs). |
| R7  | DONE   | Remove Python test code               | Removed `release-cli-verification.py` after R1-R5 preserved its durable product-behavior checks in Rust executable-boundary tests. The issue retains historical context in its progress log; real release-style manual verification remains separately required for M1-M5.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| R8  | DONE   | Document Rust-only test policy        | `tests/AGENTS.md` now defines the operational policy: tracked repository test code is Rust; Python is allowed only for separately justified non-test external tooling, never test automation, fixtures, or assertions. `docs/testing.md` states the policy and links to that authoritative guidance without duplicating the exception.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| R9  | DONE   | Final validation and evidence         | M1-M5 release scenarios are recorded in `manual-verification-evidence.md`. Final validation passed: 129 configuration-package tests, 18 CLI executable tests, 13 lifecycle tests, focused Clippy, `linter all`, and the required pre-commit gate. Acceptance criteria were re-reviewed; no retrospective is warranted because the relevant discoveries are retained in the issue plans and repository guidance.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

## Scenario Contracts

Each executable-boundary scenario uses an isolated `TempDir`, child-specific
configuration, port-zero bindings, and the fixture's existing absolute deadlines.

- **Precedence:** set conflicting valid `TORRUST_TRACKER_CONFIG_TOML` and
  `TORRUST_TRACKER_CONFIG_TOML_PATH` only in the child `Command`; the health
  endpoint from the CLI file must become ready. Do not mutate test-process
  environment variables.
- **Base-source exclusivity:** test this separately at the configuration-package
  layer, where loaded optional sections are directly observable. An
  executable-boundary port assertion proves which selected source is live but
  cannot alone distinguish exclusive source selection from a coincidental merge.
- **Override:** child command sets a distinguishable
  `TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS`; the override
  endpoint becomes ready, not the CLI file endpoint.
- **Failure:** each child is reaped before assertions complete. Each test
  asserts one contract: the exit code plus the tracker-owned diagnostic (and
  the source path when the source has one) through fixture assertion helpers
  that print the captured child output on failure. A tracker that wrongly
  started would never exit, so the bounded `wait_for_exit` deadline is the
  proof of no partial startup; no fixed candidate ports are used.
- **Unreadable file:** Unix-specific test setup and cleanup own the file mode.
  The assertion must account for a privileged runner that bypasses permission
  bits and report an explicit skip rather than masking a platform constraint.
  Fixture behaviour (permission restoration, drop without a runtime) is tested
  in the fixture's own unit tests, not in the executable contract tests.

## Validation

During development:

```text
cargo test --test lifecycle-signals
cargo test --package torrust-tracker
linter rustfmt
linter clippy
```

Before committing the completed coverage increment:

```text
./contrib/dev-tools/git/hooks/pre-commit.sh --format=json
```

## Completion Conditions

- Every executable behavior formerly covered by the disposable verifier has a
  Rust test at the selected layer, or has a documented manual-only reason.
- No tracked Python test code remains.
- The Rust-only test policy is documented in the canonical testing guidance and
  reflected in the root integration-test guidance.
- The issue spec is updated with final test evidence before implementation PR
  creation.
