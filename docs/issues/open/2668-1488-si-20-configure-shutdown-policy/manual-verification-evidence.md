---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
last-updated-utc: "2026-10-27 14:47"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands or interactions actually performed against the artifact. This file currently holds only the pre-review reproduction of the defect that this issue corrects (process exit code `0` after a failed or aborted shutdown); the remaining scenarios (M2 to M8) are recorded here when they run.

## Preserving Verification Artifacts

The tracker configuration and the script used are in [`evidence/`](evidence/) (`baseline.toml`, `hold-and-stop.sh`). The temporary fault-injection patch is embedded verbatim in V1. Full run logs stayed under the git-ignored `.tmp/si20/`; the relevant lines of every run are copied into this file.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-27, runs between 08:45 and 08:58.
- Artifact under test: the tracker at `develop` merge `72fbeacc1` ("Merge torrust/torrust-tracker#2666: chore: update dependencies"), which is an ancestor of the branch head. The branch changes only documentation, so `src/`, `packages/`, `Cargo.toml`, and `Cargo.lock` equal `torrust/develop` (`git diff --stat torrust/develop HEAD -- src packages Cargo.toml Cargo.lock` printed nothing). Debug build with `cargo build --bin torrust-tracker`.
- Operating system / environment: Linux 7.0.0-38-generic, `rustc 1.99.1 (58cf8f9c3 2026-10-13)` (stable toolchain), bash, no container.
- Prerequisites and setup performed: `evidence/baseline.toml` (HTTP tracker on `127.0.0.1:17070`, REST API on `127.0.0.1:11212`, health check on `127.0.0.1:11313`, no database); `mkdir -p .tmp/si20`; the script starts `./target/debug/torrust-tracker` directly, so the signalled PID is the tracker binary and not a `cargo run` launcher.

## Verification Processes

### V1 - Pre-review reproduction: exit code `0` after a failed or aborted shutdown

- Goal: decide whether the defect is real on the shipped artifact, and observe the wrong outcome itself.
- Initial state: `develop` code, unmodified, then the same code with a temporary, uncommitted fault-injection patch.
- Status: `DONE`
- Classification (`fix-bug`, "Reproduce Before Review"): **Reproduced, with a temporary fault-injection patch.** The shipped binary could not be driven into a failed or aborted shutdown through its network surface (A1 to A4), so the wrong outcome is observed on the real binary only after one component is made to fail or outlast the deadline (B1 to B4). This is not a reproduction on the unmodified artifact, and the specification does not claim one.

#### Original symptom and hypothesis

Symptom: `src/main.rs` calls `jobs.wait_for_all(Duration::from_secs(10)).await`, discards the returned `Vec<JobOutcome>`, and always logs "Torrust tracker successfully shutdown." before returning from `main`, so the process exits with code `0` whatever the outcomes (read from source, `src/main.rs`, lines 30 to 40). Hypothesis: a component that fails or is aborted at the deadline still gives exit code `0` and the success message. Q3 of `docs/features/shutdown-process/questions.md` requires exit code `1` in those cases.

#### Steps Performed

Each step ran alone, after the previous one finished, from the repository root.

1. `cargo build --bin torrust-tracker` (stable toolchain). A1 to A4 ran on this binary (192,716,720 bytes).
2. A1 to A4: `.tmp/si20/hold-and-stop.sh .tmp/si20/baseline.toml TERM <hold>`, one run per hold mode. The script starts the binary, waits for the health check, opens the connection as described, waits one second, sends the signal to the binary PID with `kill`, and prints the exit status and the seconds to exit.
3. Applied the temporary patch below to `src/app.rs`, saved it with `git diff src/app.rs`, rebuilt (binary of 192,681,488 bytes), and ran B1 to B4 with `SI20_REPRO_FAULT` set.
4. Reverted with `git checkout -- src/app.rs`, proved the revert (below), rebuilt, and ran the control C1 and C2 on the unpatched binary (192,716,720 bytes again).

Temporary patch (never committed):

```diff
diff --git a/src/app.rs b/src/app.rs
index 37a3e32e3..c0d69d5eb 100644
--- a/src/app.rs
+++ b/src/app.rs
@@ -642,7 +642,19 @@ fn start_peers_inactivity_update(
     if config.core.tracker_usage_statistics {
         let reservation = job_manager.reserve("peers_inactivity_update");
         let runner = activity_metrics_updater::run_job(config, app_container, reservation.cancellation_token());
-        spawn_component(job_manager, reservation, component_runner(runner))?;
+        // TEMPORARY fault injection for the SI-20 reproduction; never commit.
+        let injected = async move {
+            let result = component_runner(runner).await;
+            match std::env::var("SI20_REPRO_FAULT").as_deref() {
+                Ok("slow-stop") => {
+                    tokio::time::sleep(Duration::from_secs(30)).await;
+                    result
+                }
+                Ok("fail-stop") => Err(crate::bootstrap::jobs::manager::ComponentError::new("temporary injected failure")),
+                _ => result,
+            }
+        };
+        spawn_component(job_manager, reservation, injected)?;
     } else {
         tracing::info!("Peers inactivity update job is disabled.");
     }
```

#### Observed Result

Attempts on the unmodified binary (hold modes of `evidence/hold-and-stop.sh`):

| Run | Hold                                                                                  | Started (UTC) | Exit status | Seconds from signal to exit | Success message |
| --- | ------------------------------------------------------------------------------------- | ------------- | ----------- | --------------------------- | --------------- |
| A1  | `idle`: TCP connection to the HTTP tracker, nothing sent                              | 08:45:51      | 0           | 1.0                         | yes             |
| A2  | `announce-body`: announce request declaring `Content-Length: 1000`, body never sent   | 08:48:05      | 0           | 0.0                         | yes             |
| A3  | `partial-header`: request line and one header sent, no blank line                     | 08:49:18      | 0           | 4.0                         | yes             |
| A4  | `api-body`: `POST /api/v1/keys` to the REST API declaring a 1000-byte body, 7 bytes sent | 08:50:59   | 0           | 4.0                         | yes             |

None of the four held connections outlasted the 10-second deadline, and every run reported all components completed. The two runs that waited for a held connection (A3 and A4) took 4.0 s from the signal to the exit. Two timers in the source explain why a stalled request does not last longer (both read from source, not run): the HTTP server closes a connection whose headers are incomplete after `HTTP1_HEADER_READ_TIMEOUT`, 5 s (`packages/axum-server/src/custom_axum_server.rs:42`), and the REST API cuts a request after `DEFAULT_REQUEST_TIMEOUT`, 5 s (`packages/axum-rest-api-server/src/routes.rs:31`). The attempts do not show that no network client can hold a connection longer; they show that these four did not.

Runs with the temporary patch (binary of 192,681,488 bytes), hold `none`:

| Run | `SI20_REPRO_FAULT` | Signal  | Started (UTC) | Exit status | Seconds from signal to exit |
| --- | ------------------ | ------- | ------------- | ----------- | --------------------------- |
| B1  | `slow-stop`        | SIGTERM | 08:52:32      | 0           | 10.0                        |
| B2  | `slow-stop`        | SIGINT  | 08:53:35      | 0           | 10.0                        |
| B3  | `fail-stop`        | SIGTERM | 08:54:36      | 0           | 0.0                         |
| B4  | `fail-stop`        | SIGINT  | 08:55:38      | 0           | 0.0                         |

Relevant log lines (B1, `SI20_REPRO_FAULT=slow-stop`, SIGTERM; the line for B2 differs only in `(SIGINT)`):

```text
INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
INFO torrust_tracker_lib::bootstrap::jobs::manager: Waiting for job to finish under the shared shutdown deadline timeout_seconds=10
WARN torrust_tracker_lib::bootstrap::jobs::manager: Job shutdown deadline expired; aborting remaining jobs timeout_seconds=10
WARN torrust_tracker_lib::bootstrap::jobs::manager: Job was aborted after the shutdown deadline job=peers_inactivity_update
INFO torrust_tracker: Torrust tracker successfully shutdown.
```

Relevant log lines (B3, `SI20_REPRO_FAULT=fail-stop`, SIGTERM; the lines for B4 differ only in `(SIGINT)`):

```text
INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
INFO torrust_tracker_lib::bootstrap::jobs::manager: Waiting for job to finish under the shared shutdown deadline timeout_seconds=10
WARN torrust_tracker_lib::bootstrap::jobs::manager: Job reported a failure job=peers_inactivity_update error=temporary injected failure
INFO torrust_tracker: Torrust tracker successfully shutdown.
```

Control on the unmodified binary again, hold `none`:

| Run | Signal  | Started (UTC) | Exit status | Seconds from signal to exit | Log                                                                                                                   |
| --- | ------- | ------------- | ----------- | --------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| C1  | SIGTERM | 08:56:52      | 0           | 0.0                         | `shutting down (SIGTERM)`, `Waiting for job ... timeout_seconds=10`, every `Job completed ...`, `successfully shutdown` |
| C2  | SIGINT  | 08:57:53      | 0           | 0.0                         | same with `(SIGINT)`                                                                                                  |

Revert proof (`git checkout -- src/app.rs`, then `git show torrust/develop:src/app.rs | cmp - src/app.rs`, which printed nothing and exited 0; `git status --short` printed nothing).

#### Conclusion

The defect is real and reproduced for both signals: a component aborted at the deadline (B1, B2) and a component that reported a failure (B3, B4) both give exit status `0` and the message "Torrust tracker successfully shutdown.", although the supervisor logged the abort or failure. Q3 requires exit code `1`. The reproduction needs a temporary fault because the four held connections of A1 to A4 did not make the unmodified binary fail or abort a component. That limits what these attempts show and does not make the defect harmless: from the source of `src/main.rs`, any component that fails or outlasts the deadline in production exits `0` in the same way (inferred from the source; the injected faults B1 to B4 are the runs that observed it).

#### Regression-test boundary and recheck plan

- Smallest maintained deterministic boundary: a unit test of a pure function that maps `Vec<JobOutcome>` to the exit code, one case per `JobStatus` (T3). It is the causal seam, because `main` discards the outcomes today. It needs no OS signal and no network.
- Red before the fix: T3 first adds the function returning the current behavior (always success) with the parameterized test, runs the test red for the `Failed`, `Panicked`, and `Aborted` cases, records the failing output here, then implements the mapping and records the green output. If red-first proves impractical, the mutate-then-restore method of the `write-unit-test` skill applies and a `cmp` proves the restore.
- Executable boundary (T6): native-binary tests in `tests/lifecycle/` for a clean stop (`0`) and for a drain budget reached (`1`), using a held partial-header connection and a configured drain shorter than the 5 s header timeout (see M3).
- Final like-for-like recheck (T8): after the last production change, rebuild, re-apply the same temporary patch to the fixed tree (not committed), and rerun B1 to B4. Expected: exit status `1` and no "successfully shutdown" message; the control C1 and C2 stay at `0`. Name the commit subject the recheck ran on.

### T2 - `[application.shutdown]` configuration section: tests, prose-first review, mutation proofs

- Goal: record that the 25 tests added by T2 describe the behavior they guard, and that each of them fails when that behavior is removed.
- Status: `DONE`
- Toolchain: stable Rust 1.99.1 for every `cargo test` run below, and the nightly Rust toolchain for `cargo +nightly fmt --all -- --check`.

#### Tests added by T2

`cargo test -p torrust-tracker-configuration` (stable toolchain) passed 129 tests on `develop` before the change and passes 154 after it: 25 new tests.

- `packages/configuration/src/v3_0_0/shutdown.rs` (15): `it_should_default_to_the_documented_budgets`, `it_should_accept_the_default_budgets_as_consistent`, `it_should_use_the_defaults_for_every_omitted_key`, `it_should_default_each_key_that_is_omitted_while_the_other_keys_are_set`, `it_should_round_trip_through_toml`, `it_should_accept_a_budget_at_each_end_of_the_supported_range`, `it_should_reject_a_zero_budget_with_the_supported_range_in_the_message`, `it_should_reject_a_budget_above_the_maximum`, `it_should_reject_a_zero_budget_during_deserialization`, `it_should_reject_an_unknown_key`, `it_should_accept_a_chain_that_is_one_second_below_the_deadline`, `it_should_reject_a_chain_that_equals_the_deadline_naming_the_http_drain`, `it_should_name_the_udp_request_drain_when_it_is_the_longer_server_drain`, `it_should_name_the_http_drain_when_both_server_drains_are_equal`, `it_should_tell_the_operator_which_keys_and_values_to_change`.
- `packages/configuration/src/v3_0_0/application.rs` (2): `it_should_use_the_default_shutdown_budgets_when_the_table_is_omitted`, `it_should_reject_an_unknown_key`.
- `packages/configuration/src/v3_0_0/mod.rs` (8): `configuration_should_use_the_default_shutdown_budgets_when_the_application_table_is_omitted`, `configuration_should_keep_the_default_budgets_that_the_shutdown_table_omits`, `configuration_should_allow_to_override_the_shutdown_deadline_with_an_env_var`, `configuration_should_name_the_key_when_a_shutdown_budget_is_zero`, `configuration_should_reject_an_unknown_key_in_the_shutdown_table`, `configuration_should_use_the_default_application_settings_when_deserialized_without_the_application_table`, `configuration_validation_should_reject_shutdown_budgets_that_exceed_the_deadline`, `configuration_validation_should_accept_the_default_configuration`.

The four tests that pin the serialized default configuration (`configuration_should_have_default_values`, `configuration_should_be_saved_in_a_toml_config_file`, and the two `..._only_the_mandatory_options_are_provided...` tests) were updated with the new `[application.shutdown]` table; no test was removed or merged.

#### Prose-first Arrange-Act-Assert comparison

The temporary prose was written per test group after the tests existed (it was not kept in the source) and compared with the code. The comparison led to no change. The one test added afterwards, which omits each key in turn, came from the surviving mutation J below, and its loop is kept because its failure message names the omitted key.

| Tests | Arrange (causal state) | Act | Assert (independently specified result) |
| ----- | ---------------------- | --- | --------------------------------------- |
| defaults, partial tables, omitted keys | A `Shutdown` with no keys, or with some keys set to values that differ from every default. | Build it with `Shutdown::default()` or `toml::from_str`. | The four budgets equal the literals 25, 15, 5, 5 for the omitted keys, and the set keys keep their values. |
| range (accept at both ends, zero, above the maximum, zero in TOML) | A value of 0, 1, 3600, or 3601. | `ShutdownBudgetInSecs::new` or deserialization. | Accepted values read back unchanged; rejected ones return the error whose message gives the range 1 to 3600 and whose `value` is the rejected one. |
| chain rule (below, equal, UDP longer, tie, message) | Four budgets chosen so that `max(H, U) + L` is 24, 25, 26, or tied against the deadline 25. | `Shutdown::validate`. | Accepted below the deadline; otherwise the error names the longer drain's key (the HTTP key on a tie) with the four values; the message names the three keys and the advice. |
| unknown keys, round trip, serialization | A misspelled key, or a table with distinct values. | `toml::from_str` and `toml::to_string`. | The misspelled key is rejected; the round trip returns the same budgets. |
| `Configuration`-level tests | A configuration text with or without `[application.shutdown]`, one key from an environment variable, or a `Configuration` built in code. | `Configuration::load`, `toml::Table::try_into`, or `Configuration::validate`. | The loaded values, the key and range named in the load error, the default section when the table is absent, and the chain rule reached through `Configuration::validate`. |

Smell review: the `shutdown_with` helper names one complete action (build a `Shutdown` from four budgets) and hides only TOML formatting; every budget that selects a result stays visible in the test body. Expected defaults are literals, not read from `Shutdown::default()`.

#### Mutation proofs

Each row was run alone, in this order: restore the saved originals and prove the restore with `cmp`, apply one mutation with the edit tool, run `cargo test -p torrust-tracker-configuration` (stable toolchain), read the output. Every run reported `could_not_compile=0`, so each failure is a test failure and not a rejected mutation. Runs were between 11:15 and 11:24 UTC. After the last run `cmp` printed nothing for `shutdown.rs`, `application.rs`, `mod.rs`, and `validator.rs` against the saved originals, and `git status` listed only the intended changes.

| Id | Clause guarded | Mutation | Tests that failed |
| -- | -------------- | -------- | ----------------- |
| A | A zero budget is rejected | Drop the lower bound check | `it_should_reject_a_zero_budget_with_the_supported_range_in_the_message`, `it_should_reject_a_zero_budget_during_deserialization`, `configuration_should_name_the_key_when_a_shutdown_budget_is_zero` (3) |
| B | A budget above the maximum is rejected | Drop the upper bound check | `it_should_reject_a_budget_above_the_maximum` (1) |
| B2 | The maximum itself is accepted | `value > MAXIMUM` becomes `value >= MAXIMUM` | `it_should_accept_a_budget_at_each_end_of_the_supported_range` (1) |
| B3 | The minimum itself is accepted | `value < MINIMUM` becomes `value < MINIMUM + 1` | `it_should_accept_a_budget_at_each_end_of_the_supported_range` (1) |
| C | The chain rule rejects an impossible configuration | The rejecting condition never holds | 5: the three chain-rule tests that expect an error, `it_should_tell_the_operator_which_keys_and_values_to_change`, `configuration_validation_should_reject_shutdown_budgets_that_exceed_the_deadline` |
| D | A chain equal to the deadline is rejected (strict `<`) | `>=` becomes `>` | 4: `it_should_name_the_http_drain_when_both_server_drains_are_equal`, `it_should_reject_a_chain_that_equals_the_deadline_naming_the_http_drain`, `it_should_tell_the_operator_which_keys_and_values_to_change`, `configuration_validation_should_reject_shutdown_budgets_that_exceed_the_deadline` |
| D2 | A chain below the deadline is accepted | Add 1 to the sum | `it_should_accept_a_chain_that_is_one_second_below_the_deadline` (1) |
| E | The UDP drain counts in the chain | The UDP branch of the comparison never applies | `it_should_name_the_udp_request_drain_when_it_is_the_longer_server_drain` (1) |
| F | On a tie the HTTP key is named | `>` becomes `>=` in the choice of the longer drain | `it_should_name_the_http_drain_when_both_server_drains_are_equal` (1) |
| G | `Configuration::validate` reaches the application section | `Configuration::validate` returns `Ok(())` after the core check | `configuration_validation_should_reject_shutdown_budgets_that_exceed_the_deadline` (1) |
| G2 | `Application::validate` reaches the shutdown table | `Application::validate` returns `Ok(())` | `configuration_validation_should_reject_shutdown_budgets_that_exceed_the_deadline` (1) |
| H | The default HTTP drain is 15 s | The default is 20 | 8, among them `it_should_default_to_the_documented_budgets` and `it_should_accept_the_default_budgets_as_consistent` |
| I | `Shutdown` rejects an unknown key | Remove `deny_unknown_fields` from `Shutdown` | `it_should_reject_an_unknown_key` (shutdown.rs) and `configuration_should_reject_an_unknown_key_in_the_shutdown_table` (2) |
| I2 | `Application` rejects an unknown key | Remove `deny_unknown_fields` from `Application` | `it_should_reject_an_unknown_key` (application.rs) (1) |
| J | Each shutdown key is optional | Remove the serde default of `deadline_in_secs` | **Survived**: 152 passed, 0 failed. The existing omitted-key tests always supplied the deadline. Added `it_should_default_each_key_that_is_omitted_while_the_other_keys_are_set`. |
| J rerun | same | same | `it_should_default_each_key_that_is_omitted_while_the_other_keys_are_set`, with the message ``missing field `deadline_in_secs` `` (1) |
| K | The `shutdown` table is optional in `[application]` | Remove `#[serde(default)]` from `Application::shutdown` | `it_should_use_the_default_shutdown_budgets_when_the_table_is_omitted` (application.rs) (1) |
| L | A directly deserialized `Configuration` may omit `[application]` | Remove `#[serde(default)]` from `Configuration::application` | **Survived**: 153 passed, 0 failed. `Configuration::load` merges the defaults itself, so only direct deserialization depends on the attribute. Added `configuration_should_use_the_default_application_settings_when_deserialized_without_the_application_table`. |
| L rerun | same | same | that test (1) |
| M | A budget serializes as its value | Serialize `value + 1` | 9, among them `it_should_round_trip_through_toml` |
| N | The validation message tells the operator what to change | Replace the advice sentence | `it_should_tell_the_operator_which_keys_and_values_to_change` (1) |
| O | The key names are the documented ones (and so the environment variable path) | Rename `deadline_in_secs` with `#[serde(rename = "deadline")]` | 17, among them `configuration_should_allow_to_override_the_shutdown_deadline_with_an_env_var` |
| P | The range error reports the rejected value | `value: 0` | `it_should_reject_a_budget_above_the_maximum` (1) |
| Q | The default configuration carries the documented section | `Configuration::default()` uses a 99 s deadline | 6, among them `configuration_should_use_the_default_shutdown_budgets_when_the_application_table_is_omitted` |
| R | The default UDP drain is 5 s | The default is 6 | 8, among them `configuration_should_keep_the_default_budgets_that_the_shutdown_table_omits` |

Every one of the 25 new tests failed at least once: the first 15 are covered by rows A to R (for example the shutdown.rs tests by A, B, B2, B3, C, D, D2, E, F, H, I, J rerun, M, N, O, P), the two application.rs tests by K and I2, and the eight `Configuration`-level tests by Q, R, O, A, I, L rerun, C, and H.

A process mistake, recorded: B2 and B3 were first issued in the same batch as the restore that preceded them. Their edit failed to match (the file was still in the state of B), and their "failures" were B's. Both were discarded and rerun one step at a time; the table shows the valid reruns.

#### Automatic checks run on the finished T2 code (11:25 UTC)

- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain): exit 0.
- `linter all`: exit 0.
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (stable Rust toolchain): exit 0, 3,534 tests passed, 0 failed.
- `cargo test --doc --workspace` (stable Rust toolchain): exit 0.

#### Conclusion

The 25 tests are each proven to fail when the behavior they guard is removed. Two gaps found by surviving mutations (J, L) were closed with new tests and re-proven.

### T3a - Red regression test for the exit code of a failed or aborted shutdown

- Goal: show that the maintained regression test fails against the behavior of V1 (exit `0` whatever the outcomes) before the fix exists.
- Status: `DONE`
- Toolchain: stable Rust 1.99.1.

#### Boundary

`shutdown_exit_code(&[JobOutcome]) -> u8` in `src/bootstrap/jobs/manager.rs` is a pure function and the causal seam: `main` discards the outcomes today, so nothing decides the exit code. The tests need no OS signal and no network. The function returns `u8` because `std::process::ExitCode` cannot be compared in a test; `main` converts it with `ExitCode::from`.

#### Steps and observed result

1. Added the function returning `EXIT_CODE_GRACEFUL_SHUTDOWN` (0) for every input, which is the current behavior of `main`, and five tests: no outcomes, only `Completed` and `Cancelled`, a `Failed`, a `Panicked`, and an `Aborted` outcome (each next to a `Completed` one).
2. Ran `cargo test --lib -p torrust-tracker it_should_exit_with` (stable toolchain, 12:29:49 UTC). The code compiled; the output has no compile error.

```text
test bootstrap::jobs::manager::tests::it_should_exit_with_the_graceful_code_when_there_are_no_outcomes ... ok
test bootstrap::jobs::manager::tests::it_should_exit_with_the_graceful_code_when_every_component_completed_or_was_cancelled ... ok
test bootstrap::jobs::manager::tests::it_should_exit_with_the_incomplete_code_when_a_component_failed ... FAILED
test bootstrap::jobs::manager::tests::it_should_exit_with_the_incomplete_code_when_a_component_was_aborted_at_the_deadline ... FAILED
test bootstrap::jobs::manager::tests::it_should_exit_with_the_incomplete_code_when_a_component_panicked ... FAILED
assertion `left == right` failed: a Failed outcome (for example a reached drain budget) must not exit with the graceful code
  left: 0
 right: 1
test result: FAILED. 2 passed; 3 failed; 0 ignored; 0 measured; 146 filtered out; finished in 0.00s
```

#### Conclusion

The three tests for `Failed`, `Panicked`, and `Aborted` are red for the reason of the defect (`left: 0`, `right: 1`). The two graceful cases pass on purpose: they pin the behavior that must not change. T3b makes the three pass and records the green run.

### T3b - Fix: the process exit code follows the shutdown outcomes

- Goal: make the three red tests of T3a pass, prove the new tests guard each clause, and record the green run.
- Status: `DONE`. The executable-boundary recheck of V1 (B1 to B4) is T8's job and has not run yet.
- Toolchain: stable Rust 1.99.1 (nightly Rust toolchain for the formatter check).

#### Change

`JobStatus::is_graceful` (`Completed` and `Cancelled`), `shutdown_exit_code` (`0` only when every outcome is graceful, otherwise `1`), and `incomplete_components` in `src/bootstrap/jobs/manager.rs`. `src/main.rs` now returns `std::process::ExitCode`: the success message is logged only on exit code `0`, an incomplete shutdown logs an error line that names the incomplete components, and a startup failure returns `ExitCode::FAILURE` instead of calling `std::process::exit(1)`. The 10 s deadline is unchanged here; T4 replaces it.

#### Green run

`cargo test --lib -p torrust-tracker bootstrap::jobs::manager` (stable toolchain, 12:32 UTC): 39 passed, 0 failed, including the three tests that were red in T3a and the new `it_should_name_only_the_components_whose_shutdown_was_not_graceful_in_outcome_order`.

#### Mutation proofs

Each row: apply one mutation with the edit tool, run the manager tests (stable toolchain), read the output (`could_not_compile=0` in every run), then restore the saved file and prove it with `cmp`.

| Id | Clause guarded | Mutation | Tests that failed |
| -- | -------------- | -------- | ----------------- |
| S1 | Exit `0` only when every component is graceful | `all` becomes `any` | `it_should_exit_with_the_graceful_code_when_there_are_no_outcomes`, `..._when_a_component_failed`, `..._when_a_component_panicked`, `..._when_a_component_was_aborted_at_the_deadline` (4) |
| S2 | `Cancelled` is graceful | `is_graceful` without `Cancelled` | `it_should_exit_with_the_graceful_code_when_every_component_completed_or_was_cancelled`, `it_should_name_only_the_components_whose_shutdown_was_not_graceful_in_outcome_order` (2) |
| S3 | `Completed` is graceful | `is_graceful` without `Completed` | the same two tests (2) |
| S4 | No outcomes is graceful | `!outcomes.is_empty() &&` added | `it_should_exit_with_the_graceful_code_when_there_are_no_outcomes` (1) |
| S5 | Only incomplete components are named | filter inverted | `it_should_name_only_the_components_whose_shutdown_was_not_graceful_in_outcome_order` (1) |
| S6 | Names are in outcome order | `.rev()` added | the same test (1) |

After the last run `cmp .tmp/orig/manager.rs src/bootstrap/jobs/manager.rs` printed "identical", and `git diff --stat -- src` listed only `src/bootstrap/jobs/manager.rs` and `src/main.rs`.

A process mistake, recorded: the first S1 attempt was issued as one parallel batch with the creation of the mutation script and the backup of `manager.rs`. The mutation edit landed before the backup, so the backup held the mutated file, and the run failed because the script did not exist yet. I discarded it, restored the intended line by hand, recompiled and ran the 39 tests green, took the backup again, and ran S1 to S6 one step at a time. This is the second time in this issue that dependent steps were batched (the first was B2 and B3 of T2).

#### Not covered by a unit test

`report_shutdown` in `src/main.rs` (the success message only on exit `0`, the error line otherwise) and the conversion to `ExitCode` are in the binary and have no unit test. They are covered by the native-binary tests of T6 and by the recheck of V1 in T8, which check the exit status and the log of the real binary.

#### Automatic checks on the finished T3b code (12:36 UTC)

- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain): exit 0.
- `linter all`: exit 0.
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (stable Rust toolchain): exit 0, 3,540 passed, 0 failed.
- `cargo test --doc --workspace` (stable Rust toolchain): exit 0.

### T4 - Deliver the process deadline (P) and the listener drain (L)

- Goal: the configured deadline reaches `main` and the startup-failure cleanup, the configured listener drain reaches all seven listeners, and each new test fails when its behavior is removed.
- Status: `DONE` for the code and the unit tests. The executable-boundary proof that the configured deadline is the one the supervisor applies (the `timeout_seconds` of the `Waiting for job to finish under the shared shutdown deadline` log line) is T6's and T8's.
- Toolchain: stable Rust 1.99.1 (nightly Rust toolchain for the formatter check).

#### Design (maintainer decision D5: inject top to bottom)

- `ShutdownBudgetInSecs::as_duration` in the configuration package, so a callee receives a plain `Duration`.
- `AppContainer` carries `application_config` (an `Arc<Application>` built from the `Configuration` it is initialized from). `main` reads the deadline from it, and `start_jobs` reads it from `config` in the cleanup after a failed startup. No hard-coded 10 s remains in `src/main.rs` or `src/app.rs`.
- `event_listener_drain_timeout(&Configuration) -> Duration` in `src/bootstrap/jobs/mod.rs`; the five bootstrap files call it for the seven listeners. `run_banning_event_listener` gained a `config` parameter like its siblings, and `run_event_listener` of the HTTP core, UDP core and UDP server statistics sites now use theirs (they took an unused `_config`).

#### Tests added by T4 (4)

`v3_0_0::shutdown::tests::it_should_convert_a_budget_to_a_duration_of_whole_seconds`, `container::tests::it_should_keep_the_configured_shutdown_budgets_of_the_configuration_it_was_initialized_from`, `bootstrap::jobs::tests::it_should_use_the_configured_event_listener_drain_budget_and_not_the_default`, and `bootstrap::jobs::tests::it_should_not_leave_a_listener_site_on_the_default_drain_timeout`. The workspace suite went from 3,540 to 3,544 passed.

The last test is a source-level check and the reason is recorded here: the drain timeout of a listener has no observable seam in a unit test, because it matters only when a handler is slower than the timeout and the real handlers of these listeners are fast (under a paused clock the drain loop yields after every event and never lets the clock advance). Instead of testing a value no seam can show, the test reads the five bootstrap files with `include_str!` and requires, per file, that `event_listener_drain_timeout(config),` appears once per listener it starts (1, 1, 2, 1, 2) and that `DEFAULT_DRAIN_TIMEOUT` does not appear. It was red against the unchanged sites (all five files named) before they were changed.

#### Prose-first comparison

| Test | Arrange | Act | Assert |
| ---- | ------- | --- | ------ |
| `as_duration` | A budget of 7 s. | `as_duration()`. | Exactly `Duration::from_secs(7)`. |
| container | A `Configuration` whose deadline is 40 s (the default is 25 s). | `AppContainer::initialize`. | The container's `application_config` deadline is 40 s. |
| helper | A `Configuration` whose listener drain is 7 s (the default is 5 s). | `event_listener_drain_timeout`. | 7 s, not 5 s. |
| site guard | The sources of the five bootstrap files and the number of listeners each starts. | Count the helper calls used as an argument and search for the default constant. | No file is named in the failure list. |

The comparison led to no change except the guard: its first version (the file mentions the helper) is the one that let mutation T2b survive.

#### Mutation proofs

Each row: back up the file, apply one mutation with the edit tool, run `cargo test` (stable toolchain), read the output, restore, and prove the restore with `cmp`. `could_not_compile=0` for every row below.

| Id | Clause guarded | Mutation | Tests that failed |
| -- | -------------- | -------- | ----------------- |
| T1 | The helper returns the configured drain | The helper ignores the configuration and returns 5 s | `it_should_use_the_configured_event_listener_drain_budget_and_not_the_default` (1) |
| T2 | (rejected) a site uses a literal | The site's call was removed | Not a result: the compiler rejected it (unused import), so no test ran. |
| T2b | Every site passes the helper's result | `torrent_repository.rs` calls the helper, discards the result, and passes `5 s` | **Survived** with the first guard (155 passed, 0 failed). The guard was strengthened to count the call as an argument, per file. |
| T2c | same | the same change in `torrent_repository.rs` | `it_should_not_leave_a_listener_site_on_the_default_drain_timeout` (1), file named |
| T2d | Both listeners of `tracker_core.rs` | the same change on the in-memory listener only | the same test (1) |
| T2e | Both listeners of `udp_tracker_server.rs` | the same change on the banning listener only | the same test (1) |
| T2f | `http_tracker_core.rs` | the same change | the same test (1) |
| T2g | `udp_tracker_core.rs` | the same change | the same test (1) |
| T2h | No site passes the default constant | `torrent_repository.rs` passes `DEFAULT_DRAIN_TIMEOUT` | the same test (1) |
| T3 | The container keeps the configured settings | `Application::default()` instead of the configuration's | `it_should_keep_the_configured_shutdown_budgets_of_the_configuration_it_was_initialized_from` (1) |
| T4 | `as_duration` returns whole seconds | `from_millis` | `it_should_convert_a_budget_to_a_duration_of_whole_seconds` (1) |

After the last run `cmp` printed "identical" for every file and `git status --short` listed only the intended changes.

#### Not covered by a unit test, and where it is covered

The two deadline call sites (`main` and the cleanup in `start_jobs`) have no unit test: the deadline is observable only as the time a stuck component takes to be aborted, and a unit test cannot make a real component stuck. The supervisor logs the deadline it applies (`timeout_seconds`), so the native-binary tests of T6 assert that line for a configured deadline on a clean stop and on a failed startup, and T8 repeats it on the final code.

#### Process notes, recorded

- Two batches of dependent tool calls ran in parallel again (a restore with the next edit, and the creation of a script with its first use). In both the edit hit the wrong file state; the first changed nothing, the second rejected mutation T2 and was redone one step at a time. This is the third occurrence in this issue.
- One unused import was removed from `src/main.rs` with `sed -i` instead of the edit tool; the compiler checked the result.

#### Automatic checks on the finished T4 code (12:46 UTC)

- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain): exit 0.
- `linter all`: exit 0.
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (stable Rust toolchain): exit 0, 3,544 passed, 0 failed.
- `cargo test --doc --workspace` (stable Rust toolchain): exit 0.

### T5 - Deliver the HTTP drain (H) and the UDP request drain (U)

- Goal: the configured `H` reaches the HTTP tracker, REST API and health-check drains, the configured `U` reaches the UDP request drain, no server crate keeps its own constant, and each new test fails when its behavior is removed.
- Status: `DONE` for the code and the unit tests. The proof that the real binary applies the configured values (a held connection ends the drain at `http_drain_in_secs`, not at 15 s) is T6's and T8's.
- Toolchain: stable Rust 1.99.1 (nightly Rust toolchain for the formatter check).
- Commits (cited by subject): `fix(axum-server): bound the drain timeout so an oversized value cannot overflow the clock`, `feat(axum-http-server): apply the configured drain timeout to the HTTP tracker`, `feat(axum-rest-api-server): apply the configured drain timeout to the REST API`, `feat(axum-health-check-api-server): apply the configured drain timeout to the health check API`, `feat(udp-server): apply the configured request drain deadline to the UDP tracker`.

#### Design (maintainer decision D5: inject top to bottom)

- HTTP: `Launcher` has a new field `drain_timeout: Duration` (a required `new` argument). `start_token_aware_runtime` hands it to the drain helper. `HTTP_GRACEFUL_DRAIN_TIMEOUT` (90 s) is removed.
- REST API: `Launcher` has the same private field and the same use. `API_GRACEFUL_DRAIN_TIMEOUT` (90 s) is removed. The bootstrap `start_v1` now takes a `Launcher` instead of the socket and the TLS configuration, which keeps it within the argument limit.
- Health check: `start_with_cancellation` has a new `drain_timeout` argument. `HEALTH_CHECK_API_GRACEFUL_DRAIN_TIMEOUT` (5 s) is removed. It follows `http_drain_in_secs`, so its default goes from 5 s to 15 s.
- UDP: `Server::start_with_cancellation` has a new `request_drain_deadline` argument and `Launcher::start_receive_loop` takes it, so `REQUEST_DRAIN_DEADLINE` (5 s) and the wrapper that supplied it are removed. The legacy launcher path (`run_with_graceful_shutdown`, removed by SI-19) uses `Shutdown::default()`, a public default where injection would only add churn to code that is about to be deleted.
- Composition root: `src/app.rs` passes `http_drain_in_secs` to the HTTP tracker, the REST API and the health check, and `udp_request_drain_in_secs` to every UDP tracker. The standalone test environments pass `Shutdown::default()` values. The examples use those environments and compile unchanged (`cargo build --workspace --all-targets --all-features`: exit 0).
- Drain helper: a timeout above one year is shortened to one year. `Instant + Duration` panics when the sum does not fit, and the helper now takes any `Duration` (the configuration bound of 3,600 s protects only the composition root).
- Package versions are not bumped: the packages are at 0.1.0 and the earlier steps of the EPIC changed the same signatures without a bump; the independent-versioning ADR bumps at release.

#### Tests added by T5 (4) and one test fixture rewired

`server::tests::it_should_time_out_the_drain_after_the_configured_drain_timeout_when_a_connection_stays_open` in each of `axum-http-server`, `axum-rest-api-server` and `axum-health-check-api-server`, and `signals::tests::it_should_return_drained_without_panicking_when_the_drain_timeout_is_too_large_for_the_clock` in `axum-server`. The workspace suite went from 3,544 to 3,548 passed.

The three server tests use real time (a 300 ms drain timeout against a 5 s bound) instead of paused time: these crates do not enable the tokio `test-util` feature, and the point is an end-to-end drain over a real socket. They cannot be flaky on the happy path, because the drain ends by the timer, and the drain wait and the acceptance probe are bounded by `tokio::time::timeout` (the probe only after the commit `test(shutdown): bound the held-connection probe of the drain tests`, see the T6 process notes).

UDP has no new test. Its receive-loop shutdown scenarios (`ReceiveLoopHoldingRequests`, 100 ms deadline) used to start the receive loop directly through `Launcher::start_receive_loop_with_request_drain_deadline`; they now start through the public `Server::start_with_cancellation`, so every scenario also guards that the public start carries the deadline to the loop. No test was removed or merged (233 tests in `torrust-tracker-udp-server` before and after).

#### Prose-first comparison

| Test | Arrange | Act | Assert |
| ---- | ------- | --- | ------ |
| server drain (three crates) | A server whose launcher has a 300 ms drain timeout, one connection that sent only part of a request header, and a completed second request that proves the first connection was accepted. | Cancel the token and wait up to 5 s for the drain controller. | The outcome is `TimedOut`; the failure message names the 300 ms timeout. |
| oversized timeout | A drain helper with no open connection and `Duration::MAX` as the drain timeout. | Cancel the token. | The helper returns `Drained` within a second of paused time and does not panic. |

The comparison led to one change: the first version of the server tests did not wait for the held connection to be accepted, so a cancellation could arrive before the server knew about it and the drain would end as `Drained`. The second request fixes that, because the server accepts connections in order.

#### Mutation proofs

Each row: back up the file, apply one mutation with the edit tool, run `cargo test` (stable toolchain), read the output, restore, and prove the restore with `cmp`. `could_not_compile=0` for every result row.

| Id | Clause guarded | Mutation | Tests that failed |
| -- | -------------- | -------- | ----------------- |
| H1 | The HTTP server drains for the launcher's timeout | `start_token_aware_runtime` uses 90 s whatever the launcher holds | the HTTP drain test (1): "the drain should end by the configured timeout, not the default one: Elapsed" |
| R1 | The REST API drains for the launcher's timeout | the same change in the REST API | the REST drain test (1) |
| C1 | The health check drains for the given timeout | the drain helper receives 90 s | the health-check drain test (1) |
| U1 | The public UDP start carries the deadline to the receive loop | `request_drain_deadline.max(90 s)` in `start_with_cancellation` | `it_should_not_return_while_processors_orphaned_by_the_request_ring_are_still_running` (1); the run took 5.19 s, which is the test's own bound |
| S0 | (rejected) the helper does not shorten an oversized timeout | the bound was replaced by an expression that left `MAX_DRAIN_TIMEOUT` unused | Not a result: the compiler rejected it (`could_not_compile=1`), so no test ran. |
| S1 | The helper shortens an oversized timeout | the bound was replaced by `max(MAX_DRAIN_TIMEOUT.min(0))`, which keeps `Duration::MAX` | the oversized-timeout test (1): the tokio `Instant` overflow panic |

After the last run `cmp` printed "identical" for every file and `git status --short` listed only the intended changes. U1 is weaker than the others in one way: with the default 5 s instead of 90 s the orphaned-processors scenario would end near its own 5 s bound, so only a clearly larger value is caught reliably. The 90 s mutation is the realistic regression (the old constants were 90 s and 5 s; reverting UDP to 5 s is what T8's real-binary check covers).

#### Not covered by a unit test, and where it is covered

The composition-root lines in `src/app.rs` (which key goes to which crate) have no unit test: the drain budgets are observable only as the time a held connection or request keeps the shutdown waiting. T6 and T8 run the real binary with distinct values for `http_drain_in_secs` and `udp_request_drain_in_secs` and check the timing, which is also the only check that the REST API and the health check receive `http_drain_in_secs` and not another key.

#### Process notes, recorded

- One multi-line edit of `src/bootstrap/jobs/http_tracker.rs` was done with a throwaway Python script instead of the edit tool. The compiler and the diff checked it.
- One edit replaced a block and left out three import lines in its replacement; the next edit restored them and `git diff` showed the intended diff only.
- Mutation S0 did not compile (an unused constant); it was caught by the `could_not_compile` count and redone.
- The four app.rs hunks were split across the five commits with `git add -p`, so each crate's commit holds only its own caller change.

#### Automatic checks on the finished T5 code (13:03 UTC)

- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain): exit 0.
- `linter all`: exit 0.
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (stable Rust toolchain): exit 0, 3,548 passed, 0 failed.
- `cargo test --doc --workspace` (stable Rust toolchain): exit 0, 50 passed.

### T6 - Native-binary tests of the shutdown policy

- Goal: the real binary, signalled at its exact PID, exits `0` on a clean stop and `1` when a component fails, applies the configured process deadline and HTTP drain, and rejects invalid budgets before any service starts, for SIGTERM and SIGINT where the signal matters.
- Status: `DONE`. The tests are Rust integration tests in `tests/lifecycle/shutdown_policy.rs`, registered as the `lifecycle-shutdown-policy` target in `Cargo.toml`.
- Toolchain: stable Rust 1.99.1 (nightly Rust toolchain for the formatter check). Commit (by subject): `test(lifecycle): add native-binary tests for the shutdown policy of #2668`.

#### Tests added by T6 (9 in the target, plus 3 fixture tests)

`it_should_exit_with_code_zero_when_sigterm_stops_a_tracker_with_no_open_connections`, `it_should_exit_with_code_zero_when_sigint_stops_a_tracker_with_no_open_connections` (both also assert the default `timeout_seconds=25`), `it_should_apply_the_configured_process_deadline_when_the_tracker_stops` (`timeout_seconds=7`), `it_should_exit_with_code_one_when_a_connection_held_open_outlasts_the_http_drain_after_sigterm` and `..._after_sigint`, `it_should_exit_with_code_one_and_apply_the_configured_deadline_when_a_service_fails_to_start`, `it_should_reject_a_zero_process_deadline_before_any_service_starts`, `it_should_reject_budgets_whose_drains_do_not_fit_in_the_process_deadline_before_any_service_starts`, and `it_should_accept_budgets_whose_drains_leave_one_second_of_slack_in_the_process_deadline`.

The fixture tests are `it_should_set_one_child_only_override_per_shutdown_budget`, `it_should_extract_the_assigned_address_of_the_service_named_by_the_log_target` and `it_should_remove_the_color_escapes_around_a_logged_field`. The shared fixture is compiled into three test binaries, so the workspace suite went from 3,548 to 3,575 passed: 21 tests in the new target (9 above, the 3 fixture tests, and 9 that already existed in the shared fixture) and 3 fixture tests in each of the other two binaries.

The fixture changed as follows: `NativeTrackerShutdownBudgets` and `NativeTrackerConfigurationSources::with_shutdown_budgets` pass the four budgets through the `TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__*` variables (which also removes inherited ones); `NativeTracker::wait_for_exit` returns the exit status and the output (a tracker that exits by itself, like a rejected configuration, needs it); `http_tracker_address` reads the address the HTTP tracker logged; `plain_output` removes the color escapes. `shutdown()` now fails unless the exit status is `0`. **It ignored the status before**, so the existing lifecycle tests would have passed on an exit code of `1`; with the new exit-code contract that was a hole in the shared fixture.

Every wait is bounded: 10 s for readiness and 30 s for the exit in the fixture, and 5 s for the probe that proves the held connection was accepted. The target runs in about 2 s.

#### Deviations from the specification, recorded

- The "drain budget reached" scenario uses `deadline_in_secs = 10`, not the 5 of manual scenario M3. With a deadline equal to the 5 s header-read timeout of the held connection, a tracker that ignored the configured 2 s drain could still exit `1` by reaching the deadline, so the test would not tell the two causes apart. At 10 s the unconfigured tracker stops cleanly at about 5 s, and only the configured 2 s drain gives exit `1`. The chain rule still holds (`max(2, 1) + 1 = 3 < 10`). The manual scenario M3 keeps its values; there a person reads the log.
- The unknown-key and the 3,601 s rejections of M4 are not repeated at the executable boundary: they are rejected by the configuration loader, which the unit tests of T2 cover, and the executable boundary adds the exit code and the message for one range error (zero) and one chain error.

#### Findings while running, both test-side

- The tracker writes color escapes around a log field: the field name and the `=` each sit between an escape and a reset, so `contains("timeout_seconds=25")` failed on the first run. `plain_output` strips the escapes.
- An override from the environment names the key in upper case in the error (`APPLICATION.SHUTDOWN.DEADLINE_IN_SECS`), so the key assertions compare lower-cased text. The error for the zero deadline reads in full: "A shutdown budget must be between 1 and 3600 seconds, but 0 was configured. Use a whole number of seconds within that range."
- The first run failed 6 of 19 tests for those two reasons only; the exit codes were already right.

#### Prose-first comparison

| Test | Arrange | Act | Assert |
| ---- | ------- | --- | ------ |
| clean stop (SIGTERM, SIGINT) | A ready tracker with no shutdown settings and no open connection. | Send the signal to its PID and wait for the exit. | Exit code `0`, the success log, and the supervisor wait of 25 s. |
| configured deadline | A ready tracker whose deadline is 7 s. | SIGTERM and wait for the exit. | Exit code `0` and a supervisor wait of 7 s. |
| held connection (SIGTERM, SIGINT) | A ready tracker with a 10 s deadline and a 2 s HTTP drain, and a connection that sent only part of a header, proven accepted by a second request. | Send the signal and wait for the exit. | Exit code `1`, the incomplete-shutdown error, no success log, a supervisor wait of 10 s. |
| failed start | A tracker whose health-check port is taken, with a 7 s deadline. | Wait for the exit without a signal. | Exit code `1` and a cleanup wait of 7 s. |
| rejected budgets (two) | A tracker with a zero deadline, then with `20 + 5 >= 25`. | Wait for the exit without a signal. | Exit code `1`, the key named, and no `Started on:` line. |
| accepted boundary | A tracker with `19 + 5 = 24 < 25`. | SIGTERM and wait for the exit. | Exit code `0` and the success log. |

The comparison led to the 10 s deadline above and to moving the key assertions to lower case.

#### Mutation proofs

Each row: back up the file, apply one mutation with the edit tool, run `cargo test --test lifecycle-shutdown-policy` (stable toolchain), read the output, restore, and prove the restore with `cmp`. `could_not_compile=0` for every row.

| Id | Clause guarded | Mutation | Tests that failed |
| -- | -------------- | -------- | ----------------- |
| E1 | The exit code follows the outcomes | `report_shutdown` returns `ExitCode::from(exit_code & 0)` | the two held-connection tests (2) |
| E2 | `main` applies the configured deadline | `main` waits a fixed 10 s | the two clean-stop tests (default 25 s) and the configured-deadline test (7 s) (3); the held-connection tests still pass because their configured deadline is 10 s, so they do not guard this clause |
| E3 | The HTTP tracker receives the configured drain | `src/app.rs` passes `Shutdown::default()`'s 15 s to the HTTP tracker | the two held-connection tests (2); the run took 5.07 s, the header timeout that now ends the connection first |
| E4 | The startup chain rule runs | `Configuration::validate` skips the application validation | the chain-rule test (1) after 30 s, when the fixture force-killed the tracker that had started; the zero-deadline test passes, because that range is checked when the value is parsed |
| E5 | The cleanup after a failed start applies the configured deadline | `start_jobs` waits a fixed 10 s | the failed-start test (1) |

After the last run `cmp` printed "identical" for every file and `git status --short` listed only the intended changes. Two clauses have no mutation of their own: "before any service starts" (the `Started on:` absence in the two rejection tests; the runs that pass show `Started on:` lines in the output of a started tracker, so the assertion can fail, but no mutation moves the validation after the listeners), and the SIGINT variant of the failed exit, which differs from SIGTERM only in the signal and shares the helper.

#### Not covered by a native-binary test, and where it is covered

The UDP request drain and the listener drain have no timing test at the executable boundary: neither has a connection a client can hold open on the fixture's configuration (it has no UDP tracker). T5 proves the UDP deadline through the public start in a unit scenario, and T4 guards the listener helper and its call sites at source level. The REST API and the health check receive `http_drain_in_secs` in `src/app.rs`; the real-binary check that they do (distinct values, a held connection on each) is T8.

#### Process notes, recorded

- Two batches of dependent tool calls ran in parallel again: the creation of the test file and its `Cargo.toml` registration with the first `cargo test` (the target was not found), and a batch of edits with a `cargo test` (a rerun on its own confirmed the result). No result below comes from either batch. This is the fourth and fifth occurrence in this issue.
- `linter all` found four problems in the first version (a misspelling caused by an escape sequence written next to a word in a string, test code placed before items, a field-postfix lint that is allowed with a reason because the fields mirror the configuration keys, and an `if let` that `map_or` expresses); all were fixed in the code, not by a dictionary change.
- Reviewing my own tests against the "every wait bounded" rule, I found that the probe confirming the held connection was accepted (`reqwest::get`) had no timeout in the three T5 drain tests and in the lifecycle helper. The commit `test(shutdown): bound the held-connection probe of the drain tests` gives each a five-second bound; the earlier commits are kept as they are.
- The disposable script `evidence/hold-and-stop.sh` and `evidence/baseline.toml` are **not removed yet**: the Rust tests cover the held-connection exit codes (M3, V1 controls), but not the fault-injection runs of M5 (they need a temporary patch that is never committed), nor Docker (M6 and M7). T8 still needs them; T9 removes them.

#### Automatic checks on the finished T6 code (14:47 UTC, after the commit that bounds the probes)

- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain): exit 0.
- `linter all`: exit 0.
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (stable Rust toolchain): exit 0, 3,575 passed, 0 failed.
- `cargo test --doc --workspace` (stable Rust toolchain): exit 0.

- Two discarded attempts at 08:46 (`partial-header` and `api-body`) failed at startup with `Address already in use (os error 98)`: sockets of the previous run were still in TIME_WAIT. Both exited with status `1` before the tracker started and are not part of V1. The script now waits until the three ports have no socket before it starts; A3 and A4 are the reruns.
- The first clean-stop run (hold `none`, SIGTERM, about 08:45) wrote a log that a later run overwrote. C1 and C2 replace it as the control.
- Not yet run: M2 to M8 of the specification (they depend on the implementation).
