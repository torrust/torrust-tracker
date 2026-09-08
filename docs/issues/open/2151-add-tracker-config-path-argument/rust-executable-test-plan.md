---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - docs/issues/open/2151-add-tracker-config-path-argument/ISSUE.md
    - docs/issues/open/2151-add-tracker-config-path-argument/release-cli-verification.py
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
│   └── cli_configuration.rs    # Tracker CLI source-selection process contracts
└── lifecycle/
  └── signals.rs              # SIGINT, SIGTERM, and drop-path contracts
```

Each top-level test source remains a separate Cargo integration-test executable.
Both `configuration/cli_configuration.rs` and `lifecycle/signals.rs` include the
shared fixture through `#[path = "../common/native_tracker.rs"] mod native_tracker;`.

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

| ID  | Status | Task                                  | Expected result                                                                                                                                                                                                                                                                                                                    |
| --- | ------ | ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R1  | TODO   | Extract common native fixture         | Move `tests/lifecycle/native_tracker.rs` to `tests/common/native_tracker.rs`. Preserve its child-process, workspace, output-draining, absolute-deadline, normal-shutdown, and drop-path cleanup ownership. Add only the narrow child-command options required for configuration tests; default lifecycle use must remain isolated. |
| R2  | TODO   | Add configuration CLI test target     | Add and register `tests/configuration/cli_configuration.rs`. Reuse the fixture through an explicit path module declaration; do not duplicate child-process management.                                                                                                                                                             |
| R3  | TODO   | Add CLI precedence process tests      | Add separate AAA scenarios to `cli_configuration.rs` for CLI-only configuration, CLI plus both inherited base environment sources, and CLI plus a per-value override. Use separate valid config files with observable health-check ports; verify readiness and graceful shutdown.                                                  |
| R4  | TODO   | Add invalid-source process matrix     | Add executable tests to `cli_configuration.rs` for absent option value, empty value, missing file, directory, malformed TOML, and parent-only relative file. Assert exit `2` for parser usage errors and exit `1` for source failures, path-bearing diagnostics where applicable, and no listener at a configured candidate port.  |
| R5  | TODO   | Add Unix unreadable-file process test | Create a regular `mode 000` file, attempt a child start as the current user, and assert the permission error only when the platform enforces it. If a privileged runner can read it, explicitly skip with documented rationale rather than asserting a false failure. Restore file permissions during cleanup.                     |
| R6  | TODO   | Review test design increment          | After each behavior-focused increment, run the relevant target (`cli-configuration` or `lifecycle-signals`) and review responsibility, ownership, absolute readiness deadlines, output retention, and panic/drop cleanup before adding the next scenario. Stop for maintainer review after R5.                                     |
| R7  | TODO   | Remove Python test code               | After R1-R5 pass and reviewer approval, remove `release-cli-verification.py` and its artifact references. Replace the current scripted verifier section with concise manual release commands only if final manual validation remains useful.                                                                                       |
| R8  | TODO   | Document Rust-only test policy        | Update `docs/testing.md` and `tests/AGENTS.md` to state that tracked repository test code is Rust; use Python only for non-test external tooling when separately justified. Link this decision to the test-layer guidance without duplicating it.                                                                                  |
| R9  | TODO   | Final validation and evidence         | Run the required focused tests, `linter all`, pre-commit, and manual release scenarios. Re-review acceptance criteria and record whether a retrospective is needed.                                                                                                                                                                |

## Scenario Contracts

Each executable-boundary scenario uses an isolated `TempDir`, child-specific
configuration, port-zero bindings unless a fixed candidate port is necessary to
prove no listener, and the fixture's existing absolute deadlines.

- **Precedence:** set conflicting valid `TORRUST_TRACKER_CONFIG_TOML` and
  `TORRUST_TRACKER_CONFIG_TOML_PATH` only in the child `Command`; the health
  endpoint from the CLI file must become ready. Do not mutate test-process
  environment variables.
- **Override:** child command sets a distinguishable
  `TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS`; the override
  endpoint becomes ready, not the CLI file endpoint.
- **Failure:** each child is reaped before assertions complete. A fixed
  loopback candidate port is bound/probed after failure only when the invalid
  source was otherwise capable of providing that port (malformed and
  parent-only-relative cases).
- **Unreadable file:** Unix-specific test setup and cleanup own the file mode.
  The assertion must account for a privileged runner that bypasses permission
  bits and report an explicit skip rather than masking a platform constraint.

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

- Every executable behavior formerly asserted by `release-cli-verification.py`
  has a Rust test at the selected layer, or has a documented manual-only reason.
- No tracked Python test code remains.
- The Rust-only test policy is documented in the canonical testing guidance and
  reflected in the root integration-test guidance.
- The issue spec is updated with final test evidence before implementation PR
  creation.
