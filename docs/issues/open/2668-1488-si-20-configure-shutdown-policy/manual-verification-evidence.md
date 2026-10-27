---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
last-updated-utc: "2026-10-27 11:27"
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

## Failures and Follow-up

- Two discarded attempts at 08:46 (`partial-header` and `api-body`) failed at startup with `Address already in use (os error 98)`: sockets of the previous run were still in TIME_WAIT. Both exited with status `1` before the tracker started and are not part of V1. The script now waits until the three ports have no socket before it starts; A3 and A4 are the reruns.
- The first clean-stop run (hold `none`, SIGTERM, about 08:45) wrote a log that a later run overwrote. C1 and C2 replace it as the control.
- Not yet run: M2 to M8 of the specification (they depend on the implementation).
