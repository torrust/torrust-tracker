---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2238-refactor-native-tracker-test-fixture/ISSUE.md
last-updated-utc: 2026-09-17 11:16
---

# Manual Verification Evidence

## Purpose

Record human-oriented verification that the refactored fixture preserves its executable-boundary
behavior and makes ownership and maintenance paths locally understandable.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-17 11:16
- Artifact under test: native tracker fixture on branch `2238-refactor-native-tracker-test-fixture`
- Operating system / environment: Linux, Rust workspace test profile
- Prerequisites and setup performed: issue #2238 plan items 1-7 and 9 implemented; focused tests
  passed after each extraction; narrow dead-code allowances committed after both consumers passed

## Verification Processes

### V1 - Exercise and Trace a Running Tracker Lifecycle

- Goal: exercise real child startup, readiness, signal shutdown, and drop cleanup, then trace every
  owned resource and deadline through the final modules.
- Initial state: clean worktree at signed commit
  `refactor(tests): [#2238] narrow fixture dead-code allowances`.
- Status: `DONE`

#### Steps Performed

1. Ran `cargo test --test lifecycle-signals -- --nocapture`.
2. Started from `tests/lifecycle/signals.rs` and followed `NativeTracker` through
   `tests/common/native_tracker/mod.rs`.
3. Traced workspace and command construction into `command.rs`, output reader ownership into
   `output.rs`, and health parsing/probing into `health.rs`.
4. Traced explicit shutdown and `Drop` cleanup back through the root module.

#### Observed Result

```text
running 13 tests
test it_should_force_kill_and_reap_the_tracker_binary_when_the_fixture_is_dropped ... ok
test it_should_gracefully_shutdown_the_tracker_binary_when_sigterm_is_delivered_to_its_exact_pid ... ok
test it_should_distinguish_sigint_from_sigterm_when_shutting_down_the_tracker_binary ... ok
test it_should_run_two_tracker_binaries_with_independent_cli_configurations ... ok
test result: ok. 13 passed; 0 failed; 0 ignored
```

Ownership trace:

- `NativeTracker` in `mod.rs` owns the running `Child`, `TrackerOutputCapture`,
  `NativeTrackerWorkspace`, health client, and drop-cleanup channel endpoints.
- `NativeTrackerWorkspace` in `command.rs` owns the temporary directory, rendered configuration,
  and storage path while the running fixture exists.
- `TrackerOutputCapture` in `output.rs` owns both reader tasks; explicit shutdown and drop cleanup
  join them before treating retained output as final diagnostics.
- `HealthCheckClient` in `health.rs` owns only the HTTP client and discovered address; it owns no
  process, workspace, task, or lifecycle deadline.
- `STARTUP_DEADLINE`, `SHUTDOWN_DEADLINE`, and the readiness retry policy remain in `mod.rs` with
  the running lifecycle.

#### Conclusion

The real signal and drop-path scenarios pass. The running lifecycle is traceable from its consumer
through one root owner and named leaf collaborators without entering failed-start implementation;
every process, workspace, reader task, and deadline has one visible owner.

### V2 - Exercise and Trace a Failed-Start Lifecycle

- Goal: exercise invalid-source child exits and diagnostics, then trace preparation, resource
  transfer, reaping, permission restoration, and fallback cleanup.
- Initial state: same implementation and environment as V1.
- Status: `DONE`

#### Steps Performed

1. Ran `cargo test --test cli-configuration invalid_sources -- --nocapture`.
2. Started from `tests/configuration/cli_configuration/invalid_sources.rs` and followed the
   `native_tracker::failed_start` facade.
3. Traced invalid-source command preparation, workspace/permission ownership transfer into
   `NativeTrackerFailedStart`, normal `wait_for_exit`, and its `Drop` fallback.
4. Confirmed output-reader joining and deadline ownership at each module boundary.

#### Observed Result

```text
running 7 tests
test invalid_sources::it_should_exit_with_a_usage_error_when_the_config_toml_path_value_is_missing ... ok
test invalid_sources::it_should_exit_with_a_usage_error_when_the_config_toml_path_value_is_empty ... ok
test invalid_sources::it_should_fail_startup_naming_the_path_when_the_cli_configuration_file_is_missing ... ok
test invalid_sources::it_should_fail_startup_naming_the_path_when_the_cli_configuration_source_is_a_directory ... ok
test invalid_sources::it_should_fail_startup_naming_the_path_when_the_cli_configuration_toml_is_malformed ... ok
test invalid_sources::it_should_not_search_parent_directories_for_a_relative_cli_configuration_file ... ok
test invalid_sources::it_should_fail_startup_naming_the_path_when_the_cli_configuration_file_is_unreadable ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 11 filtered out
```

Ownership trace:

- `NativeTrackerStartAttempt` in `failed_start.rs` owns the prepared `Command`, optional permission
  guard, temporary workspace, and source path before spawn.
- `start` transfers those resources into `NativeTrackerFailedStart`, which owns the failed child and
  output capture until normal wait or fallback drop cleanup.
- `wait_for_exit` uses `FAILURE_DEADLINE` for child wait, forced reap, and output-reader completion;
  it restores permissions before returning the retained result.
- `NativeTrackerPermissionRestore` owns mode restoration and never replaces an unwinding panic.
- `NativeTrackerFailedStartResult` retains the workspace and source path while assertions need
  stable diagnostic evidence.
- `Drop` is explicitly best effort: it restores permissions synchronously, then kills/reaps via the
  active Tokio runtime or a bounded synchronous fallback.

#### Conclusion

All seven invalid-source executable scenarios pass without starting a service. The failure path is
traceable within `failed_start.rs` plus the raw output collaborator, independently of normal
readiness; ownership transfers and `FAILURE_DEADLINE` coverage are explicit.

### V3 - Walk the Maintenance Task Map

- Goal: verify that common future changes have a bounded primary module and no hidden collaborator.
- Initial state: final five-module fixture layout and both consumers passing.
- Status: `DONE`

#### Steps Performed

1. Opened each primary module named in the refactor plan's maintenance task map.
2. Followed only its declared collaborators and consumer where applicable.
3. Compared the actual symbol locations, visibility, module docs, and focused checks with every map
   row.

#### Observed Result

```text
PASS invalid CLI source: failed_start.rs; invalid_sources.rs consumer; command.rs only for rendering
PASS rendered TOML/environment isolation: command.rs; no collaborator
PASS readiness meaning: mod.rs policy; health.rs probing/parsing collaborator
PASS output capture/joining: output.rs; no collaborator
PASS running deadline/drop policy: mod.rs; both consumers exercise shared lifecycle compilation
PASS failed-start reaping/restoration/drop: failed_start.rs; output.rs joining collaborator
```

The final module sizes are 339 lines (`mod.rs`), 302 (`command.rs`), 481 (`failed_start.rs`), 104
(`health.rs`), and 55 (`output.rs`). No row required an unlisted implementation collaborator.

#### Conclusion

Every maintenance-task-map row holds. Each common change has one primary implementation module,
at most one named implementation collaborator, a visible consumer where relevant, and a focused
validation command.

## Failures and Follow-up

No manual scenario failed or was blocked. During final automatic reconciliation, removing the broad
`#[allow(dead_code)]` from the configuration binary exposed exactly the lifecycle-only members
expected by the two-binary design. Narrow, documented allowances were added at those members, and
both integration-test binaries passed again.
