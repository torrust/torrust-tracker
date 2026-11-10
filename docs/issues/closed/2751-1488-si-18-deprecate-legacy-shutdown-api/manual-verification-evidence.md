---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2751-1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
last-updated-utc: "2026-11-10 09:43"
---

# Manual Verification Evidence: Deprecate Legacy Shutdown API

## Environment and Prerequisites

- Date and time (UTC): the first run was on 2026-11-09 from 10:50 to 10:52. Every command below was rerun from
  12:56:11 to 12:56:23 UTC the same day, and all output in this file is from that rerun, except the Step 5 output, which is from a run at 15:46:04 UTC the same day (the same five lines, in another order). Commands that were not
  in the first run: the `.start(` search (step 3) and the `python3` extraction in step 7 were first listed by
  commits at 12:07 and 12:08 UTC, and the `manager.rs` and `#[cfg(test)]` searches (steps 4 and 5) were widened
  at 12:07 UTC.
- Artifact under test: the specification branch of PR 2752 (the specification only; no code changes yet)
- Operating system / environment: Linux 7.0, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
- Prerequisites and setup performed: a checkout of the branch; network access to crates.io

## Verification Processes

### V1 - Eligibility gate

- Goal: show that every eligibility-gate item of the issue holds.
- Initial state: branch of PR 2752 (the code is that of `torrust/develop` at `da8bd15d1`, which the
  branch was cut from).
- Status: `DONE`

#### Steps Performed

All commands ran from the repository root, one after another, in the rerun described under Environment (Step 5
also ran at 15:46:04 UTC, as Environment says). The
`grep` output order is whatever this machine's `grep` printed; it does not follow the argument order, so the
blocks below are copied as printed, not sorted.

1. `grep -n "start_with_cancellation" src/bootstrap/jobs/*.rs`: the application components.
2. `grep -n "\.stop()\.await\|started\.stop\|Halted::Normal" packages/axum-http-server/src/server.rs
   packages/axum-rest-api-server/src/server.rs packages/udp-server/src/server/mod.rs
   packages/udp-server/src/server/states.rs`: legacy stops and halt sends.
3. `grep -n "\.start(" packages/axum-http-server/src/server.rs packages/axum-rest-api-server/src/server.rs
   packages/udp-server/src/server/mod.rs`: every `.start(` call, legacy or token-aware.
4. `grep -n "Halted::Normal\|HaltSignal\|NestedServerTask" src/bootstrap/jobs/manager.rs`.
5. `grep -m1 -Hn '#\[cfg(test)\]' packages/axum-http-server/src/server.rs
   packages/axum-rest-api-server/src/server.rs packages/udp-server/src/server/mod.rs
   packages/udp-server/src/server/states.rs src/bootstrap/jobs/manager.rs`: the first test-module line of each
   file.
6. `git grep -q "HaltSignal\|NestedServerTask" v3.0.0; echo "git grep exit=$?"`: the root types against the
   published tag.
7. The crates.io query, run as one command:
   `curl -s https://crates.io/api/v1/crates/torrust-server-lib -A x | python3 -c "import json,sys;print([(v['num'],v['created_at'][:10]) for v in json.load(sys.stdin)['versions']][:2])"`
8. Reading the cited documents (links below).

#### Observed Result

Step 1:

```text
src/bootstrap/jobs/health_check_api.rs:52:    let server = server::start_with_cancellation(
src/bootstrap/jobs/http_tracker.rs:124:    .start_with_cancellation(http_tracker_container, form, metadata, cancellation_token.clone())
src/bootstrap/jobs/tracker_apis.rs:132:        .start_with_cancellation(http_api_container, form, metadata, access_tokens, cancellation_token.clone())
src/bootstrap/jobs/udp_tracker.rs:72:        .start_with_cancellation(
```

Step 2:

```text
packages/axum-rest-api-server/src/server.rs:209:            let _ = tx_halt.send(Halted::Normal);
packages/axum-rest-api-server/src/server.rs:357:        self.state.halt_task.send(Halted::Normal).map_err(|_| Error::Stop {
packages/axum-rest-api-server/src/server.rs:607:        let stopped = started.stop().await.expect("it should stop the server");
packages/udp-server/src/server/mod.rs:138:        let stopped = started.stop().await.expect("it should stop the server");
packages/udp-server/src/server/mod.rs:189:        let stopped = started.stop().await.expect("it should stop the server");
packages/udp-server/src/server/states.rs:153:            let _ = tx_halt.send(Halted::Normal);
packages/udp-server/src/server/states.rs:284:            .send(Halted::Normal)
packages/axum-http-server/src/server.rs:357:            let _ = tx_halt.send(Halted::Normal);
packages/axum-http-server/src/server.rs:474:        self.state.halt_task.send(Halted::Normal).map_err(|_| Error::Stop {
packages/axum-http-server/src/server.rs:832:        let stopped = started.stop().await.expect("it should stop the server");
```

Step 3 (the hits at `udp-server` `mod.rs:342`, `370` and `411` pass a cancellation token, so they call the
token-aware `start`; the others are the legacy `start`, `launcher.start` and the test callers):

```text
packages/udp-server/src/server/mod.rs:127:            .start(
packages/udp-server/src/server/mod.rs:176:            .start(
packages/udp-server/src/server/mod.rs:230:            .start(
packages/udp-server/src/server/mod.rs:342:                .start(&Registar::default(), cancellation_token.clone())
packages/udp-server/src/server/mod.rs:370:                .start(&registar, cancellation_token.clone())
packages/udp-server/src/server/mod.rs:411:            let result = server.start(&registar, CancellationToken::new()).await;
packages/axum-rest-api-server/src/server.rs:192:        let running = launcher.start(&http_api_container, access_tokens, tx_start, rx_halt)?;
packages/axum-rest-api-server/src/server.rs:599:            .start(
packages/axum-http-server/src/server.rs:336:        let server = launcher.start(&http_tracker_container, tx_start, rx_halt)?;
packages/axum-http-server/src/server.rs:829:            .start(http_tracker_container, scenario.registration_form(), scenario.metadata())
packages/axum-http-server/src/server.rs:1000:            .start(http_tracker_container, scenario.registration_form(), scenario.metadata())
```

Step 4 (16 lines; lines 128 to 223 are the definitions and adapter code outside the test module, lines 1225 and
later are in the test module):

```text
128:pub struct HaltSignal {
132:impl HaltSignal {
144:        self.sender.take().map_or(Ok(()), |sender| sender.send(Halted::Normal))
148:impl Drop for HaltSignal {
161:pub struct NestedServerTask<T> {
162:    halt_signal: HaltSignal,
167:impl<T> NestedServerTask<T> {
171:            halt_signal: HaltSignal::new(halt_task),
185:            halt_signal: HaltSignal::new(halt_task),
223:impl<T> Drop for NestedServerTask<T> {
1225:            let mut server = super::NestedServerTask::new(
1253:        assert!(matches!(halt_receiver.await, Ok(Halted::Normal)));
1265:            let mut server = super::NestedServerTask::with_shutdown_controller(
1292:        assert!(matches!(halt_receiver.await, Ok(Halted::Normal)));
1311:            let _halt_signal = super::HaltSignal::new(halt_sender);
1327:        assert!(matches!(halt_receiver.await, Ok(Halted::Normal)));
```

Step 5. Every legacy `.start(` and `.stop().await` caller hit above sits after its file's line, with two
exceptions that are adapter code, not callers: the `launcher.start(..., rx_halt)` hits at `axum-rest-api-server`
`server.rs:192` and `axum-http-server` `server.rs:336` sit before their files' lines (521 and 528), inside the
legacy `ApiServer<Stopped>::start` and `HttpServer<Stopped>::start_with_health_check`. The `Halted::Normal` hits
before that line are adapter code too. The command was run once more at 15:46:04 UTC, and this is its output,
in the order grep printed it:

```text
packages/udp-server/src/server/mod.rs:68:#[cfg(test)]
packages/udp-server/src/server/states.rs:302:#[cfg(test)]
packages/axum-rest-api-server/src/server.rs:521:#[cfg(test)]
src/bootstrap/jobs/manager.rs:1073:#[cfg(test)]
packages/axum-http-server/src/server.rs:528:#[cfg(test)]
```

Step 6:

```text
git grep exit=1
```

Step 7 (the command's own output; the list holds the two newest versions, as the crates.io API returns them):

```text
[('0.5.0', '2026-11-06'), ('0.4.0', '2026-10-26')]
```

#### Conclusion

Every gate item holds:

- Application components: the four bootstrap jobs call `start_with_cancellation` (step 1).
- Remaining legacy callers are test modules; the other hits are the legacy adapter implementations (steps 2 to 5).
- The root `HaltSignal` and `NestedServerTask` are not in tag `v3.0.0` (step 6, exit 1 means no match; they
  first appear in commit `3ee28b723`).
- `torrust-server-lib` 0.5.0 is published (step 7); its `CHANGELOG.md` gives the migration steps and the
  support window (items stay for all 0.5.x, removed in 0.6.0).
- Linked records for the migrations and Q5, from the repository root:
  - Standalone environments and examples: `docs/issues/closed/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md`
    and `docs/issues/closed/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md`.
  - Test environments: `docs/issues/closed/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md` and
    `docs/issues/closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md`.
  - Task inventory: `docs/features/shutdown-process/task-inventory.md`, finding 4.
  - Q5: `docs/features/shutdown-process/questions.md` (Resolved) and `docs/containers.md#stopping-the-tracker`.
- External: torrust-server-lib issue 3 and PR 4 (merge commit `0ad8ca0`) at
  `https://github.com/torrust/torrust-server-lib/pull/4`.

## Implementation Evidence (T6)

Toolchain: rustc 1.99.1. Run from the repository root on branch `2751-deprecate-legacy-shutdown-api` on
2026-11-10 (09:42 UTC for M3, M4 and the doctests; the M2 consumers ran earlier the same session).

### M2: legacy consumers compile with the expected warnings

A scratch `examples/m2.rs` in each package (and in the root crate) imported and named every deprecated item without
`allow(deprecated)`, then `cargo check -p <package> --example m2 --message-format short` ran. The workspace denies
warnings, so each deprecation is printed as an `error: use of deprecated ...` line that names the replacement, the
removal wording and the signal-ownership note (the lines are cut at the end of the replacement note; the
scratch files were deleted afterwards and are not in the PR):

```text
### axum-http-server
packages/axum-http-server/examples/m2.rs:1:60: error: use of deprecated struct `torrust_tracker_axum_http_server::server::Running`: use `CancellationRunning` from `start_with_cancellation` and cancel the injected `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-http-server/examples/m2.rs:1:69: error: use of deprecated type alias `torrust_tracker_axum_http_server::server::RunningHttpServer`: use `HttpServer<Stopped>::start_with_cancellation` and keep the returned `CancellationRunning`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-http-server/examples/m2.rs:5:26: error: use of deprecated struct `torrust_tracker_axum_http_server::server::Running`: use `CancellationRunning` from `start_with_cancellation` and cancel the injected `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-http-server/examples/m2.rs:6:19: error: use of deprecated type alias `torrust_tracker_axum_http_server::server::RunningHttpServer`: use `HttpServer<Stopped>::start_with_cancellation` and keep the returned `CancellationRunning`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-http-server/examples/m2.rs:3:36: error: use of deprecated method `torrust_tracker_axum_http_server::server::HttpServer::<torrust_tracker_axum_http_server::server::Stopped>::start`: use `HttpServer<Stopped>::start_with_cancellation` with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-http-server/examples/m2.rs:4:36: error: use of deprecated method `torrust_tracker_axum_http_server::server::HttpServer::<torrust_tracker_axum_http_server::server::Stopped>::start_with_health_check`: use `HttpServer<Stopped>::start_with_cancellation_and_health_check` with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-http-server/examples/m2.rs:5:36: error: use of deprecated method `torrust_tracker_axum_http_server::server::HttpServer::<torrust_tracker_axum_http_server::server::Running>::stop`: cancel the `CancellationToken` passed to `start_with_cancellation` and join `CancellationRunning::task` and `shutdown_controller`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
error: could not compile `torrust-tracker-axum-http-server` (example "m2") due to 8 previous errors
### axum-rest-api-server
packages/axum-rest-api-server/examples/m2.rs:1:73: error: use of deprecated struct `torrust_tracker_axum_rest_api_server::server::Running`: use `CancellationRunning` from `ApiServer::start_with_cancellation` and cancel the injected `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-rest-api-server/examples/m2.rs:1:82: error: use of deprecated type alias `torrust_tracker_axum_rest_api_server::server::RunningApiServer`: use `ApiServer::start_with_cancellation` and keep the returned `CancellationRunning`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-rest-api-server/examples/m2.rs:4:25: error: use of deprecated struct `torrust_tracker_axum_rest_api_server::server::Running`: use `CancellationRunning` from `ApiServer::start_with_cancellation` and cancel the injected `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-rest-api-server/examples/m2.rs:5:13: error: use of deprecated struct `torrust_tracker_axum_rest_api_server::server::Running`: use `CancellationRunning` from `ApiServer::start_with_cancellation` and cancel the injected `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-rest-api-server/examples/m2.rs:7:19: error: use of deprecated type alias `torrust_tracker_axum_rest_api_server::server::RunningApiServer`: use `ApiServer::start_with_cancellation` and keep the returned `CancellationRunning`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-rest-api-server/examples/m2.rs:3:35: error: use of deprecated method `torrust_tracker_axum_rest_api_server::server::ApiServer::<torrust_tracker_axum_rest_api_server::server::Stopped>::start`: use `ApiServer::start_with_cancellation` with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-rest-api-server/examples/m2.rs:4:35: error: use of deprecated method `torrust_tracker_axum_rest_api_server::server::ApiServer::<torrust_tracker_axum_rest_api_server::server::Running>::stop`: cancel the `CancellationToken` passed to `start_with_cancellation` and join `CancellationRunning::task` and `shutdown_controller`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-rest-api-server/examples/m2.rs:5:22: error: use of deprecated associated function `torrust_tracker_axum_rest_api_server::server::Running::new`: use `ApiServer::start_with_cancellation`, which returns `CancellationRunning`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-rest-api-server/examples/m2.rs:6:23: error: use of deprecated method `torrust_tracker_axum_rest_api_server::server::Launcher::start`: use the token-aware path through `ApiServer::start_with_cancellation`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
error: could not compile `torrust-tracker-axum-rest-api-server` (example "m2") due to 10 previous errors
### axum-health-check-api-server
packages/axum-health-check-api-server/examples/m2.rs:1:60: error: use of deprecated struct `torrust_tracker_axum_health_check_api_server::server::RunningServer`: use `CancellationRunning` from `start_with_cancellation`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-health-check-api-server/examples/m2.rs:1:75: error: use of deprecated function `torrust_tracker_axum_health_check_api_server::server::start`: use `start_with_cancellation` with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-health-check-api-server/examples/m2.rs:3:13: error: use of deprecated function `torrust_tracker_axum_health_check_api_server::server::start`: use `start_with_cancellation` with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-health-check-api-server/examples/m2.rs:4:19: error: use of deprecated struct `torrust_tracker_axum_health_check_api_server::server::RunningServer`: use `CancellationRunning` from `start_with_cancellation`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
error: could not compile `torrust-tracker-axum-health-check-api-server` (example "m2") due to 5 previous errors
### udp-server
packages/udp-server/examples/m2.rs:3:51: error: use of deprecated struct `torrust_tracker_udp_server::server::spawner::LaunchRequest`: use `Server<Stopped>::start_with_cancellation`, which spawns the launcher with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/udp-server/examples/m2.rs:4:50: error: use of deprecated struct `torrust_tracker_udp_server::server::states::Running`: use `CancellationRunning` from `Server<Stopped>::start_with_cancellation` and cancel the injected `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/udp-server/examples/m2.rs:4:59: error: use of deprecated type alias `torrust_tracker_udp_server::server::states::RunningUdpServer`: use `Server<Stopped>::start_with_cancellation` and keep the returned `CancellationRunning`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/udp-server/examples/m2.rs:7:22: error: use of deprecated struct `torrust_tracker_udp_server::server::states::Running`: use `CancellationRunning` from `Server<Stopped>::start_with_cancellation` and cancel the injected `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/udp-server/examples/m2.rs:10:19: error: use of deprecated struct `torrust_tracker_udp_server::server::spawner::LaunchRequest`: use `Server<Stopped>::start_with_cancellation`, which spawns the launcher with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/udp-server/examples/m2.rs:11:19: error: use of deprecated type alias `torrust_tracker_udp_server::server::states::RunningUdpServer`: use `Server<Stopped>::start_with_cancellation` and keep the returned `CancellationRunning`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/udp-server/examples/m2.rs:6:32: error: use of deprecated method `torrust_tracker_udp_server::server::states::<impl torrust_tracker_udp_server::server::Server<torrust_tracker_udp_server::server::states::Stopped>>::start`: use `Server<Stopped>::start_with_cancellation` with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/udp-server/examples/m2.rs:7:32: error: use of deprecated method `torrust_tracker_udp_server::server::states::<impl torrust_tracker_udp_server::server::Server<torrust_tracker_udp_server::server::states::Running>>::stop`: cancel the `CancellationToken` passed to `start_with_cancellation` and join the returned tasks; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/udp-server/examples/m2.rs:8:23: error: use of deprecated associated function `torrust_tracker_udp_server::server::launcher::Launcher::run_with_graceful_shutdown`: use `Server<Stopped>::start_with_cancellation`, which runs the token-aware launcher; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/udp-server/examples/m2.rs:9:22: error: use of deprecated method `torrust_tracker_udp_server::server::spawner::Spawner::spawn_launcher`: use `Server<Stopped>::start_with_cancellation`, which spawns the launcher with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
error: could not compile `torrust-tracker-udp-server` (example "m2") due to 12 previous errors
### axum-server
packages/axum-server/examples/m2.rs:1:43: error: use of deprecated function `torrust_tracker_axum_server::signals::graceful_shutdown`: use `graceful_shutdown_on_cancellation` with an injected `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
packages/axum-server/examples/m2.rs:3:13: error: use of deprecated function `torrust_tracker_axum_server::signals::graceful_shutdown`: use `graceful_shutdown_on_cancellation` with an injected `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
error: could not compile `torrust-tracker-axum-server` (example "m2") due to 2 previous errors
### torrust-tracker (root)
examples/m2.rs:1:53: error: use of deprecated struct `torrust_tracker_lib::bootstrap::jobs::manager::HaltSignal`: use a `CancellationToken` and the token-aware shutdown controller; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
examples/m2.rs:1:65: error: use of deprecated struct `torrust_tracker_lib::bootstrap::jobs::manager::NestedServerTask`: use a `CancellationToken` and the token-aware shutdown controller; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
examples/m2.rs:3:19: error: use of deprecated struct `torrust_tracker_lib::bootstrap::jobs::manager::HaltSignal`: use a `CancellationToken` and the token-aware shutdown controller; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
examples/m2.rs:4:19: error: use of deprecated struct `torrust_tracker_lib::bootstrap::jobs::manager::NestedServerTask`: use a `CancellationToken` and the token-aware shutdown controller; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)
error: could not compile `torrust-tracker` (example "m2") due to 6 previous errors
```

The same check runs on every `cargo test --doc`: the `compile_fail` example on each package's deprecated `start`
(and on `graceful_shutdown`) fails when the attribute is removed. Removing the attribute from `graceful_shutdown`
made its doctest report `FAILED`, then the attribute was restored.

### M3: migrated paths have no legacy use

`RUSTFLAGS="-D deprecated" cargo check --workspace --lib --bins` printed `Finished` with exit 0 (09:42:37 UTC). The
four `src/bootstrap/jobs/{http_tracker,tracker_apis,health_check_api,udp_tracker}.rs` jobs carry no allowance, and the
impl-wide allowance on `HttpServer<Stopped>` was removed so the token-aware start methods stay checked.

### M4: documentation review

`cargo doc --workspace --no-deps` printed 0 warnings (09:42:43 UTC). The notes read: replacement, "removal is planned
for the next breaking release of this package (SI-19, EPIC #1488)" and the signal-ownership sentence pointing to
`docs/features/shutdown-process/legacy-api-migration.md`, which holds the replacement table (UDP's single `task` form
included).

### Behavior

`cargo test --workspace --lib` passes. The in-crate tests that call the deprecated `start` and `stop` of the HTTP,
REST API and UDP servers, `HaltSignal` and `NestedServerTask` compile with local allowances. The new test
`it_should_stop_the_legacy_server_when_a_normal_halt_is_sent` covers the deprecated health-check `start`. Its
design, in prose: arrange a legacy server with the `Started` and `Halted` channels and wait for the bound address;
act by sending `Halted::Normal`; assert that the drain controller and the server task finish within the test
timeout without error.

### Not deprecated, by design

The private `Launcher::start` of `axum-http-server` (not public API) and the derived `Running::new` of
`udp-server` (covered by the deprecation of its struct).

## Failures and Follow-up

None.
