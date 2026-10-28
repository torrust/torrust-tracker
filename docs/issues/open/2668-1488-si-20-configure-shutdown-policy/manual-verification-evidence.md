---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
last-updated-utc: "2026-10-28 12:12"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands or interactions actually performed against the artifact. This file holds the pre-review reproduction of the defect that this issue corrects (process exit code `0` after a failed or aborted shutdown), the evidence of each implementation task, and the manual runs of the finished change (V2 to V9).

## Preserving Verification Artifacts

The tracker configuration (`baseline.toml`) and the script (`hold-and-stop.sh`) that V1 to V8 used were removed after use (see the process notes of T6 and the progress log); the V1 baseline is the HTTP tracker on `127.0.0.1:17070`, the REST API on `127.0.0.1:11212` and the health check on `127.0.0.1:11313` with no database, and the scripted steps are described in each section. The temporary fault-injection patch is embedded verbatim in V1 (V5 records its one-word adjustment). Full run logs stayed under the git-ignored `.tmp/si20/`; the relevant lines of every run are copied into this file.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-27, runs between 08:45 and 08:58.
- Artifact under test: the tracker at `develop` merge `72fbeacc1` ("Merge torrust/torrust-tracker#2666: chore: update dependencies"), which is an ancestor of the branch head. At the time of V1 the branch changed only documentation, so `src/`, `packages/`, `Cargo.toml`, and `Cargo.lock` equal `torrust/develop` (`git diff --stat torrust/develop HEAD -- src packages Cargo.toml Cargo.lock` printed nothing); the implementation commits came later. Debug build with `cargo build --bin torrust-tracker`.
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

### V2 to V9 - Manual verification of the finished change

- Goal: on the real binary and the real image, check the configured defaults, the drain budget, the invalid configurations, the exit code after a failed or aborted shutdown, and the deployment contract (Docker, systemd), with SIGTERM and SIGINT where the signal matters.
- Status: `DONE`, except Podman, which is `not run` (not installed).
- Artifact under test: the branch head after the last production change, whose commit subject is `refactor(configuration): read each shutdown budget through a named accessor`. Every later commit changes only documentation and evidence. Debug build of `torrust-tracker` (193,169,328 bytes) and a release image built from `Containerfile` (`docker build --target release`).
- Environment: Linux 7.0.0-38-generic, stable Rust 1.99.1, Docker 28.3.3, systemd user manager (`systemd-run --user`), no Podman. Date 2026-10-27, runs between 20:17 and 20:37 UTC.
- Method: the script `evidence/hold-and-stop.sh` of V1 starts the binary directly, so the signalled PID is the tracker binary, and prints the exit status and the seconds to the exit. Scratch configurations were written under `.tmp/si20/` (git-ignored) as the baseline of V1 plus an `[application.shutdown]` block. Each run ran alone, after the previous one finished.

#### V2 (M2) - Defaults, clean stop: pass

`baseline.toml` with no `[application]` section, SIGTERM then SIGINT, no held connection.

| Signal  | Exit status | Seconds to exit | Listening ports left (17070, 11212, 11313) |
| ------- | ----------- | --------------- | ------------------------------------------ |
| SIGTERM | 0           | 0.0             | 0                                          |
| SIGINT  | 0           | 0.0             | 0                                          |

The log shows `Waiting for job to finish under the shared shutdown deadline timeout_seconds=25`, three drains with `drain_timeout=15s` (the HTTP tracker, the REST API and the health check, which proves that all three receive `http_drain_in_secs`), `All connections closed`, and `Torrust tracker successfully shutdown.` The configuration dump printed the defaults 25, 15, 5, 5.

#### V3 (M3) - Configured budgets, drain budget reached: pass

`[application.shutdown]` with `deadline_in_secs = 5`, `http_drain_in_secs = 2`, `udp_request_drain_in_secs = 1`, `event_listener_drain_in_secs = 1`, a `partial-header` connection held on the HTTP tracker.

| Signal  | Exit status | Seconds to exit |
| ------- | ----------- | --------------- |
| SIGTERM | 1           | 2.0             |
| SIGINT  | 1           | 2.0             |

Both logs, in order: `Waiting for job to finish under the shared shutdown deadline timeout_seconds=5`; `Shutdown timeout of 2s reached in address 127.0.0.1:17070 with 1 active connections. Forcing shutdown.`; `Job reported a failure job=http_instance_0_127.0.0.1:17070 error=HTTP tracker graceful drain timed out`; every listener `Job completed after cooperative cancellation`; `ERROR ... Torrust tracker shutdown was incomplete ... incomplete_components=http_instance_0_127.0.0.1:17070`. There is no success line. The specification expected about 3 s; the run took 2.0 s because the listeners had nothing queued and finished at once, so the stop took the HTTP drain alone and not `max(H, U) + L`. The chain rule is a bound, not a prediction.

#### V4 (M4) - Invalid configuration: pass

Each started with `timeout 20 ./target/debug/torrust-tracker -c <file>`; none printed a `Started on` line, so no listener bound.

| Config | Setting | Exit | Message (shortened) |
| ------ | ------- | ---- | ------------------- |
| m4a | `deadline_in_secs = 0` | 1 | `A shutdown budget must be between 1 and 3600 seconds, but 0 was configured. Use a whole number of seconds within that range.` for key `default.application.shutdown.deadline_in_secs` |
| m4b | `http_drain_in_secs = 3601` | 1 | the same message with `3601` |
| m4c | unknown key `drain_in_secs = 5` | 1 | `unknown field: found drain_in_secs, expected one of deadline_in_secs, http_drain_in_secs, udp_request_drain_in_secs, event_listener_drain_in_secs` |
| m4d | `http_drain_in_secs = 20` | 1 | `application.shutdown.http_drain_in_secs (20 s) plus application.shutdown.event_listener_drain_in_secs (5 s) must be less than application.shutdown.deadline_in_secs (25 s): event listeners are stopped only after the HTTP and UDP servers finish draining, so both stages must fit in the deadline. Lower the drain budgets or raise the deadline (and the external stop timeout of your container runtime or service manager with it).` |
| m4e | `udp_request_drain_in_secs = 21` | 1 | the same message naming `udp_request_drain_in_secs (21 s)` |
| m4f | `http_drain_in_secs = 19` (accepted boundary, `19 + 5 = 24 < 25`) | 0 after SIGTERM | started, `timeout_seconds=25`, `successfully shutdown` |

#### V5 (M5) - Failed and aborted outcomes at the executable boundary: pass

The like-for-like recheck of V1 on the fixed tree. The saved temporary patch no longer compiled (it used a `Duration` import that the production code no longer needs, because the hard-coded 10 s is gone), so its `Duration` was written as `std::time::Duration`; nothing else differs from the V1 patch. The patch was applied to `src/app.rs`, built (193,134,096 bytes), run, and reverted. `baseline.toml`, hold `none`, default deadline 25 s.

| Run | `SI20_REPRO_FAULT` | Signal  | Exit status | Seconds to exit | Log |
| --- | ------------------ | ------- | ----------- | --------------- | --- |
| B1  | `slow-stop`        | SIGTERM | 1           | 25.0            | `Job shutdown deadline expired; aborting remaining jobs timeout_seconds=25`, `Job was aborted after the shutdown deadline job=peers_inactivity_update`, `ERROR ... shutdown was incomplete ... incomplete_components=peers_inactivity_update` |
| B2  | `slow-stop`        | SIGINT  | 1           | 25.0            | the same |
| B3  | `fail-stop`        | SIGTERM | 1           | 0.0             | `Job reported a failure job=peers_inactivity_update error=temporary injected failure`, then the same `ERROR` line |
| B4  | `fail-stop`        | SIGINT  | 1           | 0.0             | the same |

None of the four logs contains `Torrust tracker successfully shutdown.` V1 recorded exit `0`, a success line and a 10 s wait for the same four runs. The unpatched control is V2 (the same binary size, exit `0`, success line). Revert proof: `git checkout -- src/app.rs`, then `cmp` against the saved clean copy and `git show HEAD:src/app.rs | cmp - src/app.rs` both printed "identical", `git status --short` printed nothing, and the binary was rebuilt (193,169,328 bytes again).

D9 (does returning from `main` hang?): in B1 and B2 the process ended 25.0 s after the signal, the supervisor deadline, with one component aborted, so dropping the runtime added no measurable delay. This does not cover a blocking task that cannot be aborted; no component of the tracker uses one, but no test forces it.

#### V6 (M6) - Docker with a sufficient grace period: pass

Release image from `docker build --target release` (`Containerfile`, this branch). `docker run -d --stop-timeout 35 -e TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__HTTP_DRAIN_IN_SECS=3 -p 127.0.0.1:27070:7070/tcp`; the container became healthy, and the command line of process 1 read `/usr/bin/torrust-tracker`, so SIGTERM reaches the tracker as PID 1.

| Case | Exit code | `docker stop` took | Log |
| ---- | --------- | ------------------ | --- |
| `partial-header` connection held on the published HTTP port | 1 | 3.2 s | `Torrust tracker shutting down (SIGTERM)`, `drain_timeout=3s` on all drains (the override reached the tracker), `Shutdown timeout of 3s reached in address 0.0.0.0:7070 with 1 active connections. Forcing shutdown.`, `HTTP tracker graceful drain timed out`, `shutdown was incomplete ... incomplete_components=http_instance_0_0.0.0.0:7070` |
| no held connection | 0 | 0.1 s | `successfully shutdown.` |

#### V7 (M7) - Docker with the default 10 s: pass

A network client cannot keep the default tracker alive for 10 s (V1), so an image was built from a patched copy of the tree (the V5 patch with `SI20_REPRO_FAULT=slow-stop` passed with `docker run -e`), never committed; the tree was reverted and proven identical to `HEAD` before anything else ran.

| `docker stop` setting | Exit code | Time | Log |
| --------------------- | --------- | ---- | --- |
| default (10 s) | 137 | 10.1 s | `shutting down (SIGTERM)`, `Waiting ... timeout_seconds=25`, drains; no completion line: the process was killed |
| `--stop-timeout 35` | 1 | 25.1 s | `Job was aborted after the shutdown deadline job=peers_inactivity_update`, `shutdown was incomplete` |

This demonstrates the documented insufficiency of the default and the contract that 35 s is enough for the default deadline.

#### V8 (M8) - systemd and Podman

`systemd-run --user --property=TimeoutStopSec=35` with the real binary, then `systemctl --user stop`.

| Case | Journal | Time |
| ---- | ------- | ---- |
| clean | `successfully shutdown.`; unit `Result=success`, `ExecMainStatus=0` | immediate |
| `partial-header` connection held, the V3 budgets | `Main process exited, code=exited, status=1/FAILURE` and `Failed with result 'exit-code'` | 2.0 s |

For the failed case a `systemctl show` after the stop read the defaults of the collected transient unit, so the journal lines above are the evidence. This confirms that a requested stop which ends with exit code 1 marks the unit failed. Podman: `not run`, it is not installed on this machine.

#### V9 - Real-binary check of the UDP drain and the HTTP-family drain with distinct values (added after the independent review)

The independent review found that no maintained test guards which key `src/app.rs` passes to the UDP tracker (`udp_request_drain_in_secs`), so a swap with `http_drain_in_secs` would pass every test. V2 to V8 used a configuration without a UDP tracker. This run uses one, with four distinct budgets, so a swapped key shows in the log.

- Configuration (scratch file, git-ignored): the V2 baseline plus `[[udp_trackers]] bind_address = "127.0.0.1:16969"` and `[application.shutdown]` with `deadline_in_secs = 20`, `http_drain_in_secs = 6`, `udp_request_drain_in_secs = 3`, `event_listener_drain_in_secs = 2` (`max(6, 3) + 2 = 8 < 20`).
- Artifact: the debug binary built before the review; the only source change since is a comment restored in `src/container.rs`.
- Result, SIGTERM and SIGINT (the PID of the tracker binary, found with `pgrep -x`): the log of each shows `request_drain_deadline=3s` on the UDP tracker start (8 spans), `drain_timeout=6s` on the HTTP tracker, REST API and health check drains (19 spans), `timeout_seconds=20`, `Torrust tracker shutting down (SIGTERM)` or `(SIGINT)`, and `Torrust tracker successfully shutdown.`; no `incomplete` line, and no tracker process remained. The exit status was not captured in this run (the tracker was started as a background shell job and signalled from a second command); the success line is only logged on exit code 0 in this code path.
- Not covered: `event_listener_drain_in_secs`. The listener drain value is not logged, and it matters only when a handler is slower than the budget, so no executable-boundary check can show it. It is guarded by the unit test of the helper and the source-level guard of T4.

#### Acceptance evidence from the source (AC7)

`grep -rn "process::exit"` over `src`, `packages`, `console` and `contrib` found it only in `src/bin/http_health_check.rs`, `console/tracker-client/src/bin/*.rs` and a developer tool: separate command-line binaries. No tracker component and not `src/main.rs` calls it.

#### Cleanup

The test containers, the two images built for V6 and V7, the transient systemd units and the scratch files were removed or left under the git-ignored `.tmp/`; no tracker process remained. The disposable script and `baseline.toml` were removed in T9 after V2 to V9 had used them.

#### Process note, recorded

- The scratch configurations of V4 were generated with a short Python script instead of the file tools; they are git-ignored scratch files, not repository content.

### T2 - `[application.shutdown]` configuration section: tests, prose-first review, mutation proofs

- Goal: record that the 25 tests added by T2 describe the behavior they guard, and that each of them fails when that behavior is removed.
- Status: `DONE`
- Toolchain: stable Rust 1.99.1 for every `cargo test` run below, and the nightly Rust toolchain for `cargo +nightly fmt --all -- --check`.

#### Tests added by T2

`cargo test -p torrust-tracker-configuration` (stable toolchain) passed 129 tests on `develop` before the change and passes 154 after it: 25 new tests at T2 (27 in the final tree: T4 and the D5 follow-up added one each).

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
- The disposable script `evidence/hold-and-stop.sh` and `evidence/baseline.toml` were used by V1 to V8 and then **removed** in the T9 commit `chore(issues): remove the disposable verification script of #2668`. The held-connection and exit-code scenarios are covered by the Rust target `lifecycle-shutdown-policy`; the fault-injection runs and the Docker and systemd runs are one-off manual evidence that needed the script and a temporary patch that is never committed.

#### Automatic checks on the finished T6 code (14:47 UTC, after the commit that bounds the probes)

- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain): exit 0.
- `linter all`: exit 0.
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (stable Rust toolchain): exit 0, 3,575 passed, 0 failed.
- `cargo test --doc --workspace` (stable Rust toolchain): exit 0.

### T7 - Templates and documentation

- Goal: the shipped templates carry the settings, and every document the new configuration makes stale says what the code does.
- Status: `DONE`. Commits (by subject): `feat(configuration): state the default shutdown budgets in the shipped templates`, `chore(compose): give the E2E tracker service the documented stop grace period`, `docs(containers): document the shutdown settings and the stop timeout for #2668`, `docs(adrs): add the application section and shutdown budget decisions of #2668`, `docs(shutdown): update the statements that the configured shutdown budgets make stale`, `docs(shutdown-process): amend Q4 with the chain rule and the new HTTP drain default`.
- Toolchain: stable Rust 1.99.1 (nightly Rust toolchain for the formatter check), Docker 28.3.3 for the Compose validation.

#### Test added by T7 (1) and its mutation proofs

`bootstrap::config::tests::it_should_state_the_default_shutdown_budgets_in_the_container_and_development_templates` reads the four container templates and the development template, requires the text `[application.shutdown]`, and requires the loaded values to equal `Shutdown::default()`. A template that omitted the block would also load, which is why the text is checked. The workspace suite went from 3,575 to 3,576 passed.

| Prose | Arrange | Act | Assert |
| ----- | ------- | --- | ------ |
| Each template shows operators the shutdown settings. | The five template files. | Read each file and load it as a configuration. | The file states the block, and its values equal the defaults. |

| Id | Clause guarded | Mutation | Tests that failed |
| -- | -------------- | -------- | ----------------- |
| T1 | The values equal the defaults | `tracker.container.sqlite3.toml` sets `http_drain_in_secs = 16` (a valid value, so it loads) | the template test (1), naming the file |
| T2 | The template states the block | the block removed from the same file | the template test (1), naming the file |

`cargo test` ran on the stable toolchain, `could_not_compile=0` for both rows, and `cmp` printed "identical" after each restore. (One attempted T2 edit passed the same text twice and changed nothing; it was redone.)

#### Documents changed, and what was stale

- `docs/containers.md`: a new section "Stopping the Tracker" (keys, defaults, the chain rule, the environment override form, exit codes `0`, `1`, `137`, the deployment table for Docker, Podman, Compose, Kubernetes and systemd with 30 s minimum and 35 s recommended, the plain statement about Docker's 10 s default, and the rule not to add `1` to `SuccessExitStatus`).
- `docs/architecture/application-jobs.md` ("one ten-second process-wide grace period"), `docs/architecture/tracker-instance-architecture.md` (the sentence that shutdown policy was not yet part of the schema), `docs/architecture/glossary.md` (the 90-second and 5-second drain timeouts, "10 seconds today", "a later issue (SI-20) aligns").
- `docs/features/shutdown-process/README.md`: the "Outcome and Deadline Policy" numbers (20 s became 15 s, the listener budget and the chain rule were added) and the "Shutdown Deadline Policy" section ("SI-20 will add"). The "Problems" list at the top describes the situation before the EPIC and was left as history.
- `docs/features/shutdown-process/questions.md`: an amendment to Q4's "Actions Taken" (the decision text stays as approved), and the link in Q9 to this evidence file. Q9's link names the open path; the archive step repairs it.
- `packages/events/src/drain.rs`: the `DEFAULT_DRAIN_TIMEOUT` documentation (it said the default was "below the 10-second shared shutdown deadline" and "until it becomes configuration").
- `packages/configuration/docs/migrate-v2-to-v3.md`: a quick-reference row, "Step 11" with the changed defaults and the exit-code change, and a checklist entry. The configuration module documentation (section list and default TOML) was already updated by T2.
- Left as history, on purpose: the "10-second shared deadline" statements of the accepted shutdown-order ADR describe the code at the time of that decision.

#### Review of the repository's own Compose files

The only Compose files are the three qBittorrent E2E stacks (SQLite, MySQL, PostgreSQL). They stopped the tracker service with Docker's 10 s default. Each tracker service now sets `stop_grace_period: 35s`. `docker compose config` (Docker 28.3.3, dummy values for the required variables) accepts the three files and prints `stop_grace_period: 35s`. The E2E stacks themselves were not run here; they run in CI. No Compose file keeps a tracker service on the 10 s default.

#### ADRs

Two root ADRs, registered in `docs/adrs/index.md`: the placement rule for `[application.<topic>]` and the budget model. The maintainer raised no objection to D2, D7 and D10 in his review of the specification.

#### Process notes, recorded

- An empty placeholder file was created with a shell heredoc (and deleted at once) while I tested that `.tmp` is writable; the tool rules require the file tools.
- A replacement of an import block dropped lines it should have kept twice: in the test imports of `packages/axum-http-server/src/server.rs` (T5, three lines) and in `src/bootstrap/config.rs` (here, the `use super::...` line); `git diff` showed it both times and it was restored before any commit.
- The T5 documentation commit deleted the heading `## Failures and Follow-up` of this file by mistake (a replacement left it out), which left the three bullets below under the T6 section. Found while writing this section; the heading is restored in the commit that records T7.

#### Automatic checks on the finished T7 code (15:01 UTC)

- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain): exit 0.
- `linter all`: exit 0.
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (stable Rust toolchain): exit 0, 3,576 passed, 0 failed.
- `cargo test --doc --workspace` (stable Rust toolchain): exit 0.

### D5 follow-up - Cohesion and coupling review of the injected budgets

- Status: `DONE`. Commit (by subject): `refactor(configuration): read each shutdown budget through a named accessor`. Toolchain: stable Rust 1.99.1 (nightly Rust toolchain for the formatter check).

#### What was reviewed (read from source, not run)

- **Who reads the settings.** Only the composition root and the bootstrap job starters read `[application.shutdown]`: `src/main.rs` and `src/app.rs` (the deadline; the HTTP and UDP drains), `src/bootstrap/jobs/mod.rs` (the listener drain through one helper), and `AppContainer.application_config`. The four server crates receive plain `Duration` values and never see the configuration type in production code; only their test environments use `Shutdown::default()`.
- **Coupling.** The four server crates already depended on `torrust-tracker-configuration`, so D5 added no dependency. A consumer needs no knowledge of the chain rule or the newtype.
- **Cohesion.** The chain rule and the bounds live in one place, `Shutdown::validate` and `ShutdownBudgetInSecs`; the supervisor deadline is read twice (`main` and the cleanup after a failed start) from the same container.

#### Findings

1. **Repeated key-and-conversion at 31 call sites** (`shutdown.http_drain_in_secs.as_duration()`): each caller named the storage key and the conversion, so a rename of a key or a change of the unit would touch every site, and a caller could take the wrong key without the compiler noticing. **Changed:** `Shutdown::deadline`, `http_drain`, `udp_request_drain` and `event_listener_drain` return a `Duration`; every call site uses them. One new test (`it_should_expose_each_budget_as_a_duration_under_its_own_name`, four distinct values) guards the mapping; swapping two accessors makes it fail (`could_not_compile=0`, `cmp` identical after the restore). The workspace suite went from 3,576 to 3,577 passed.
2. **The two reads of the deadline** (`main` and `start_jobs`) take it from different holders (`app_container` in both after the refactor). Left as it is: both now use the container.
3. **`Shutdown::default()` in four test environments and one legacy path.** Left as it is: they are standalone fixtures or code that SI-19 deletes (recorded in T5). A shared "test environment" constant would add a package dependency for no behavior.
4. **The REST API and health check share `http_drain_in_secs`.** Left as it is (D3). If a deployment needs different drains, a second key is a backward-compatible addition.
5. **`event_listener_drain_timeout(config)` takes the whole `Configuration`**, although a listener needs one `Duration`. It is a root-crate helper used by five bootstrap files with a `config` parameter they already have; narrowing it would change the signatures of the listener starters. Left as it is; the source-level guard of T4 protects its use. **Superseded** during the pull-request review: the maintainer decided that the listeners read the budget from the container like the other budgets, and the helper was removed (section "Listener drain read from the container" below).

#### Re-proof of the wiring on the new path

Because the call sites changed, two executable-boundary mutations were repeated on the new accessors, one at a time, with `cmp` after each restore: passing `Shutdown::default().http_drain()` to the HTTP tracker in `src/app.rs` fails the two held-connection tests (run time 5.07 s); a fixed 10 s in the cleanup after a failed start fails the failed-start test.

#### Automatic checks on the finished refactor

- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain): exit 0.
- `linter all`: exit 0.
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (stable Rust toolchain): exit 0, 3,577 passed, 0 failed.
- `cargo test --doc --workspace` (stable Rust toolchain): exit 0.

#### Process note, recorded

- A combined gate command (format check, linter, workspace tests and doc tests in one call) was interrupted; I reran it in two steps and used only those results.
- The D5 section edit above replaced the heading `## Failures and Follow-up` by mistake, so the three bullets of that section sat under the D5 section until the commit that adds the next section restored it. This is the second time that heading was lost (the first is recorded under T7); both times the cause was a replacement text that left out the heading it replaced.

### Listener drain read from the container (maintainer decision during the review of PR #2678)

- Status: `DONE`. Commits (by subject): `refactor(bootstrap): read the listener drain budget from the application container` and `test(bootstrap): take the drain value of the starter tests from the application container`. Toolchain: stable Rust 1.99.1 (nightly Rust toolchain for the formatter check).
- Question asked by the maintainer: why do `run_event_listener` of `http_tracker_core.rs` and `udp_tracker_core.rs`, `run_stats_event_listener` and `run_banning_event_listener` of `udp_tracker_server.rs` take both `config: &Configuration` and `app_container`, now that `AppContainer.application_config` exists? Answer (read from source): on `develop` the first three took an unused `_config` next to `app_container`; this branch renamed it and used it only for the removed helper `event_listener_drain_timeout(config)`, and gave `run_banning_event_listener` a new `config` parameter. D5 asks for one source.
- Decision: all seven listener sites read `app_container.application_config.shutdown.event_listener_drain()`; the helper is removed. The four starters above drop `config` (the banning starter has its `develop` signature again, apart from the drain value), and so do their wrappers in `src/app.rs` (`start_http_core_event_listener`, `start_udp_core_event_listener`, `start_udp_server_stats_event_listener`, `start_udp_server_banning_event_listener`), whose `config` would otherwise be unused. `torrent_repository::run_event_listener` and the two `tracker_core` listeners keep `config` because they read `core.tracker_usage_statistics` and `core.tracker_policy`, but their drain budget now also comes from the container, so every listener has the same single source.
- Job starters: `start_job` of the HTTP tracker, the REST API, the health check and the UDP tracker take a `Duration` and read no configuration (D5), so there is nothing to move into the container there. Their tests that already build an `AppContainer` (HTTP tracker TLS and listener errors, UDP tracker socket release) take the value from it; the other tests have only a `Configuration` or a standalone container and keep what they have.
- Remaining reads of `config.application`: none in production code; `src/bootstrap/config.rs` reads it in a template test and the starter tests above read it where no container is in scope.
- Tests: the helper test `it_should_use_the_configured_event_listener_drain_budget_and_not_the_default` is removed. Its behavior is covered by `it_should_expose_each_budget_as_a_duration_under_its_own_name` (accessor, four distinct values) and by the `AppContainer` test that keeps the configured budgets. The source-level guard `it_should_not_leave_a_listener_site_on_the_default_drain_timeout` now counts `app_container.application_config.shutdown.event_listener_drain(),` per file (1, 1, 2, 1, 2 listeners) and still rejects the `events` default. The workspace suite goes from 3,652 to 3,651 passed on the rebased tree (one test removed, none added).
- Mutation proofs (stable toolchain, one at a time, `could_not_compile=0`, `cmp` identical after each restore): M1, a listener site with a hard-coded 5 s (`udp_tracker_core.rs`): the guard fails and names the file; M2, a site reading `http_drain()` from the container (`torrent_repository.rs`): the guard fails and names the file.
- Not covered: the guard checks the source text, and the value the container holds is guarded by other tests; no unit test can show a listener's drain value (T4).

## Failures and Follow-up

- Two discarded attempts at 08:46 (`partial-header` and `api-body`) failed at startup with `Address already in use (os error 98)`: sockets of the previous run were still in TIME_WAIT. Both exited with status `1` before the tracker started and are not part of V1. The script now waits until the three ports have no socket before it starts; A3 and A4 are the reruns.
- The first clean-stop run (hold `none`, SIGTERM, about 08:45) wrote a log that a later run overwrote. C1 and C2 replace it as the control.
- M2 to M8 of the specification ran at 20:17 to 20:37 UTC on the finished change and are recorded as V2 to V8 above. The only part not run is Podman (not installed), in V8.
