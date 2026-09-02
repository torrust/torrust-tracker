---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - src/main.rs
    - Cargo.toml
    - tests/AGENTS.md
    - tests/common/workspace.rs
    - packages/e2e-tools/src/bin/e2e_tests_runner.rs
    - docs/issues/open/2132-add-sigterm-to-main/ISSUE.md
    - docs/issues/open/2132-add-sigterm-to-main/verification.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

# Native Executable Shutdown Test Plan

## Context

Issue #2132 adds Unix `SIGTERM` handling at the tracker executable boundary in
`src/main.rs`. The entry point maps `SIGINT` and `SIGTERM` to the current
`JobManager` shutdown sequence. Manual verification proved the behavior by
launching `./target/release/torrust-tracker`, signaling its exact PID, and
checking its logs and exit.

The current root `tests/` suites exercise the complete application in-process
through `torrust_tracker_lib::app::start()`. They cannot execute `main()` or
send an operating-system signal to the tracker process. Existing
`packages/e2e-tools` coverage runs container-based E2E scenarios and is the
right layer for Docker image and runtime behavior, but it adds container build
and runtime cost that is unnecessary for this executable-boundary contract.

## Goal

Add a fast, deterministic, Rust-native, Unix-only integration test that launches
the compiled tracker executable as a child process, requests shutdown using a
real POSIX signal, and verifies its observable shutdown behavior. Keep the
fixture narrow enough to validate #2132 now, while allowing later shutdown EPIC
slices to extend it without duplicating process lifecycle code.

## Problem Statement

The new SIGTERM behavior currently has only manual regression coverage. A future
change could remove or bypass `main()` signal registration, alter the shared
shutdown ordering, or accidentally target the Cargo launcher rather than the
tracker process without a CI check detecting it.

Calling the in-process startup API is insufficient because it does not execute
the executable boundary. Using `Child::kill()` or a generic command-test timeout
is also insufficient because those mechanisms force termination with SIGKILL on
Unix instead of exercising graceful SIGTERM or SIGINT handling.

## Scope

### In Scope

- A root Cargo integration-test target for Unix tracker executable lifecycle
  behavior.
- A reusable native child-process fixture for one tracker executable instance.
- Isolated configuration, storage, log capture, bounded readiness, signal
  delivery, process exit, and failure cleanup.
- Initial SIGTERM coverage for #2132, including the distinct source log and
  existing `JobManager` shutdown progress.
- SIGINT compatibility coverage using the shared fixture, including its distinct
  source log.

### Out of Scope

- Docker, Podman, Kubernetes, systemd, or image lifecycle tests; these remain
  container/deployment E2E concerns.
- Windows service-control-manager behavior or emulating Unix signals on Windows.
- Changing tracker shutdown policy, job ownership, grace periods, exit-result
  mapping, server lifecycle APIs, or readiness semantics.
- Refactoring all current root integration tests to use child processes.
- A general-purpose external process framework for unrelated commands.

## Test Classification and Location

This is an **executable-boundary integration test**. It runs one real compiled
tracker binary in a separate operating-system process, but does not require a
container or external service. It belongs in the root package because `main.rs`
and the tracker binary target belong to that package.

Proposed layout:

```text
tests/
├── common/                         # existing in-process application fixture
└── lifecycle/
    ├── native_tracker.rs           # child-process fixture and cleanup helpers
    └── signals.rs                  # executable signal-boundary scenarios
```

Register `tests/lifecycle/signals.rs` as an explicit `[[test]]` target in the
root `Cargo.toml`, named `lifecycle-signals`. The test target must be Unix-only;
non-Unix builds should compile a zero-test placeholder or otherwise skip this
suite deliberately, not fail while trying to import POSIX-only APIs.

## Proposed Solution

### Binary Discovery

Resolve the executable through a small helper. Prefer the runtime
`NEXTEST_BIN_EXE_torrust-tracker` override when supplied by cargo-nextest, then
the runtime `CARGO_BIN_EXE_torrust-tracker` value, then the compile-time
`env!("CARGO_BIN_EXE_torrust-tracker")` path emitted by Cargo. Cargo builds the
binary for the integration test and provides its absolute path. Do not infer a
path under `target/`, and do not launch `cargo run`.

### Isolated Tracker Workspace

Reuse the root integration tests' `TempDir` pattern, but create a dedicated
child-process fixture rather than reusing `TrackerApplicationFixture`. The
fixture writes a complete tracker configuration into its temporary workspace,
keeps the workspace alive until the child is reaped, and passes its config path
to the child through the supported configuration environment variable.

Configure every listener, including the health-check API, with loopback port
`0`. The fixture discovers the health-check API's OS-assigned address from its
startup log while it drains the child output, then uses that address for
readiness. This avoids the unsafe find-a-free-port, release it, and
then spawn pattern, as well as fixed-port conflicts between Cargo test binaries.
The fixture must make the expected health-check startup line and address parsing
an explicit test contract with useful output diagnostics. A later, separate
design may introduce a supported machine-readable bound-address contract if log
parsing no longer provides sufficient stability.

### Readiness

Wait with a bounded retry loop for an externally observable readiness condition,
not a fixed sleep. After discovering the health-check address, request
`GET /health_check`, require a successful HTTP response, deserialize its JSON
body as `Report`, and require `report.status == Status::Ok` before signaling the
child. The endpoint reports unhealthy services in its JSON body rather than an
HTTP error status. The readiness deadline must produce diagnostics containing the
child output if the tracker exits early, the address cannot be discovered, or the
service report is unhealthy.

### Process and Output Handling

Launch the child with `tokio::process::Command`. Retain the `Child` handle for
the full test lifecycle. Capture stdout and stderr, consume them concurrently
without allowing pipe-buffer backpressure to block the child, and retain each
stream for assertion and failure messages. Assertions need message presence, not
cross-stream ordering, so concatenate the fully drained streams only after exit.

Use a bounded wait for graceful completion. A fixture teardown API must await a
normally exited child and force-kill then reap it after a timeout, panic, or
failed assertion, so no zombie process is left behind. SIGKILL is test-failure
cleanup only, never normal scenario delivery.

### Signal Delivery

Use the safe typed Unix API from `nix` as a target-specific dev-dependency, with
the `signal` feature enabled. Deliver `Signal::SIGTERM` or `Signal::SIGINT` to
the exact PID returned by the retained child handle. Do not signal a terminal
process group and do not use `Child::kill()` for normal scenarios.

The first implementation does not need process-group management because the
tracker executable does not intentionally spawn child processes. If later
investigation identifies managed descendants, use stable
`std::os::unix::process::CommandExt::process_group(0)` and a documented
process-group cleanup policy rather than adding an unmaintained wrapper crate.

### Initial Assertions

The SIGTERM scenario should assert all of the following:

1. The configured health endpoint returns successfully and reports
   `Status::Ok` before the signal is sent.
2. SIGTERM was delivered to the exact tracker child process.
3. The child exited before the test deadline without cleanup SIGKILL.
4. Combined output contains `Torrust tracker shutting down (SIGTERM) ...`.
5. Combined output contains at least one `Waiting for job to finish` entry.
6. Combined output contains `Torrust tracker successfully shutdown.` while that
   remains the current executable contract.

A SIGINT scenario must assert the corresponding SIGINT source message and assert
that the SIGTERM source message is absent.

The existing sequential per-job timeout behavior may make SIGTERM slower than
SIGINT. The test deadline must accommodate the current implementation without
asserting a policy owned by SI-20. The exact deadline should be chosen from
measured CI behavior and documented in the test.

## Alternatives Considered

### Extend Existing In-Process Root Integration Tests

**Discarded.** `tests/common/workspace.rs` calls `app::start()` directly. It is
excellent for application composition, but cannot execute `main()`, exercise
Tokio's process-level signal registration, or prove that an OS signal reaches
the executable boundary.

### Add the Test to `packages/e2e-tools`

**Discarded.** That package owns Docker/container E2E workflows. Adding a
bare-binary suite there would blur responsibility and make a fast executable
contract depend on container infrastructure. Native process tests are a root
application concern and should run with the normal root test suite.

### Reuse the Bash Verification Script in CI

**Discarded.** The manual shell procedure established useful behavioral evidence,
but a Rust fixture gives typed PID and signal handling, structured cleanup,
portable test diagnostics, and a reusable API for later shutdown scenarios.

### Use `std::process::Child::kill()` or `assert_cmd` Timeout

**Discarded.** These are forceful cleanup mechanisms on Unix and do not test
SIGTERM or SIGINT. They may be used only after a test timeout as a last-resort
cleanup action.

### Call `libc::kill` Directly

**Discarded.** It requires unsafe code and manual signal/PID handling. `nix`
provides the required typed, safe Unix signal interface with a compatible MSRV.

### Introduce `command-group`

**Discarded for the initial scope.** Stable Rust already supports Unix process
groups when needed, while `command-group` is superseded upstream. There is no
current evidence that the tracker executable needs descendant process-tree
management for this suite.

## Risks and Mitigations

| Risk                                             | Mitigation                                                                                                                                                  |
| ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Port conflict during parallel tests              | Configure loopback port `0` and discover the health-check binding from drained startup output; do not reserve and release a candidate port before spawning. |
| Child output blocks on a full pipe               | Drain output concurrently or direct it to fixture-owned files before waiting for exit.                                                                      |
| Child remains running after an assertion failure | Make cleanup own the child, send SIGKILL only on failure/timeout, then reap it.                                                                             |
| CI is slower than a developer machine            | Use explicit readiness and exit deadlines with child-output diagnostics; set the grace bound from observed current behavior.                                |
| Unix-only signal API breaks Windows builds       | Gate POSIX imports, fixture implementation, and scenarios with `cfg(unix)`; define Windows behavior separately later.                                       |
| Test encodes later shutdown-policy decisions     | Assert the current SI-1 observable contract only; defer aggregate outcome, final exit code, and deadline policy to their owning EPIC slices.                |

## Implementation Plan

| Step | Work                                                                                                                                                                                        | Status    |
| ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------- |
| 1    | Confirm the health-check startup log provides the bound address and that `GET /health_check` returns successfully with `Report.status == Status::Ok` for the minimal fixture configuration. | Completed |
| 2    | Add Unix-only test target configuration, Tokio `process`, `io-util`, and `time` features, and the minimal Unix target-specific `nix` dev-dependency with its `signal` feature.              | Completed |
| 3    | Implement `NativeTracker` temporary workspace, configuration, child spawning, output capture, readiness, bounded waiting, and cleanup.                                                      | Completed |
| 4    | Add the SIGTERM executable-boundary scenario for #2132.                                                                                                                                     | Completed |
| 5    | Add the SIGINT source-distinction scenario using the shared fixture.                                                                                                                        | Completed |
| 6    | Verify failure cleanup by intentionally exercising a controlled failing path or fixture-level test seam.                                                                                    | Completed |
| 7    | Run the focused lifecycle target, root `cargo test`, `linter all`, and manual direct-binary verification.                                                                                   | Completed |
| 8    | Update `ISSUE.md`, `verification.md`, and this plan with observed commands, outcomes, time budgets, and remaining limitations.                                                              | Completed |

## Acceptance Criteria

- [x] A Unix CI runner can execute the native lifecycle target without Docker or Podman.
- [x] The test starts the actual Cargo-built `torrust-tracker` executable, not an
      in-process app or Cargo launcher.
- [x] The test waits for external readiness without fixed-sleep readiness proof.
- [x] The fixture uses port `0` for its listeners and discovers the health-check
      binding without a find-free-port race.
- [x] The SIGTERM test delivers SIGTERM to the tracked child PID and asserts the
      source-specific shutdown log, JobManager progress, and bounded exit.
- [x] The SIGINT test delivers SIGINT to the tracked child PID and asserts the
      SIGINT source-specific shutdown log without a SIGTERM source message.
- [x] Cleanup reaps the child in successful and failed paths; SIGKILL is used
      only for timeout/failure cleanup.
- [x] The Unix-only implementation does not break non-Unix compilation.
- [x] The new target, root `cargo test`, and `linter all` pass locally on Linux.
- [x] The resulting fixture is documented well enough for later shutdown EPIC
      slices to reuse without copying child-process lifecycle code.

## Validation Plan

Automatic validation after implementation:

```text
cargo test --test lifecycle-signals
cargo test
linter all
```

Observed locally on Linux on 2026-09-02:

- `cargo test --test lifecycle-signals`: 4 passed in 20.07 seconds. The
  scenarios cover SIGTERM, SIGINT, startup-log parsing, and deliberate fixture
  drop cleanup.
- `cargo test`: passed. The new lifecycle target completed in 20.07 seconds.
- `linter all`: passed.
- `cargo machete`: reported only existing unused-dependency findings in
  `packages/e2e-tools` and `packages/test-helpers`; it did not report the new
  root `nix` dependency.

The fixture permits 10 seconds for readiness and 30 seconds for graceful
shutdown. The latter accommodates the current observed SIGTERM shutdown, which
uses the legacy sequential 10-second job waits and completed in about 20
seconds. CI has not yet supplied evidence, so CI-specific acceptance remains
pending merge-pipeline execution.

Manual validation remains the direct release-binary procedure in
[verification.md](verification.md). The automated suite complements that evidence
by exercising the same executable boundary in CI; it does not replace
container/deployment graceful-stop testing owned by later work.

## Decisions Needed Before Implementation

1. Does the health-check API's startup log expose the OS-assigned binding in a
   format stable enough for this fixture to parse and diagnose failures?
2. What initial graceful-exit deadline accommodates current legacy sequential
   timeouts while remaining practical for CI?
3. Should SIGTERM and SIGINT scenarios land in one focused test commit or in
   separate commits after the shared fixture is available?

## References

- [Issue specification](ISSUE.md)
- [Manual verification evidence](verification.md)
- [Shutdown EPIC](../1488-overhaul-tracker-shutdown/ISSUE.md)
- [Root integration-test guidelines](../../../../tests/AGENTS.md)
- [Existing in-process fixture](../../../../tests/common/workspace.rs)
- [Cargo integration-test environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
- [`std::process::Child` documentation](https://doc.rust-lang.org/std/process/struct.Child.html)
- [`nix::sys::signal::kill` documentation](https://docs.rs/nix/latest/nix/sys/signal/fn.kill.html)
