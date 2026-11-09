---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2751-1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
last-updated-utc: "2026-11-09 12:08"
---

# Manual Verification Evidence: Deprecate Legacy Shutdown API

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 10:50 to 10:52 (V1 rerun for the review of PR 2752)
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

All commands ran from the repository root, one after another, in the rerun described under Environment. The
`grep` output order is whatever this machine's `grep` printed; it does not follow the argument order, so the
blocks below are copied as printed, not sorted.

1. `grep -n "start_with_cancellation" src/bootstrap/jobs/*.rs`: the application components.
2. `grep -n "\.stop()\.await\|started\.stop\|Halted::Normal" packages/axum-http-server/src/server.rs
   packages/axum-rest-api-server/src/server.rs packages/udp-server/src/server/mod.rs
   packages/udp-server/src/server/states.rs`: legacy stops and halt sends.
3. `grep -n "\.start(" packages/axum-http-server/src/server.rs packages/axum-rest-api-server/src/server.rs
   packages/udp-server/src/server/mod.rs`: every `.start(` call, legacy or token-aware.
4. `grep -n "Halted::Normal\|HaltSignal\|NestedServerTask" src/bootstrap/jobs/manager.rs`.
5. `grep -m1 -Hn '#\[cfg(test)\]' <the five files above and below>`: the first test-module line of each file.
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

Step 5. Every legacy `.start(` and `.stop().await` hit above sits after its file's line, and the `Halted::Normal`
hits before that line are adapter code, not callers:

```text
packages/udp-server/src/server/mod.rs:68:#[cfg(test)]
packages/axum-rest-api-server/src/server.rs:521:#[cfg(test)]
packages/udp-server/src/server/states.rs:302:#[cfg(test)]
packages/axum-http-server/src/server.rs:528:#[cfg(test)]
src/bootstrap/jobs/manager.rs:1073:#[cfg(test)]
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
- Remaining legacy callers are test modules; the other hits are the legacy adapter implementations (step 2).
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

## Failures and Follow-up

None. V2 to V4 (see the issue's Manual Verification Scenarios) run after implementation.
