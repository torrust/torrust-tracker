---
semantic-links:
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - docs/agents/orchestration.md
---

<!-- markdownlint-disable MD003 -->

# Agent Review Reports - Migrate UDP Tracker to Token Lifecycle

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-09-26 13:36 UTC - GitHub Copilot (Task Reviewer)

- Invocation scope: Pre-PR task review of AC1-AC12, D1-D6, and constraints 1-7 for branch
  `2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle` (seven commits from "docs(issues):
  [#2342] record SI-14 start and UDP throughput baseline" to "docs(issues): [#2342] record SI-14
  manual and performance verification"; clean working tree). Changed code: `launcher.rs`,
  `states.rs`, `mod.rs` (udp-server), `manager.rs`, `udp_tracker.rs`, `src/app.rs`; docs:
  task inventory and the SI-15, SI-17, SI-19 drafts.
- Inputs: `ISSUE.md`, `verification.md`, `manual-verification-evidence.md`,
  `performance-evidence.md`, `git diff torrust/develop...HEAD`, `torrust/develop` versions of
  `launcher.rs` and `udp_tracker.rs`, `torrust-server-lib` 0.3.0 `signals.rs`, `JobManager`
  deadline escalation, SI-13 reference (`health_check_api.rs`, closed SI-13 folder), E2E
  `logs_parser.rs`, `review-task` Test Design checklist.
- Evidence (rustc `1.100.0-nightly (6eeff9a52 2026-09-23)`):
  - `cargo test -p torrust-tracker-udp-server`: 180 unit, 11 contract, 1 doc test passed.
  - `cargo test -p torrust-tracker --lib -- udp_tracker app::tests`: 21 passed, including the
    four new component tests and the new bootstrap test.
  - `cargo clippy -p torrust-tracker-udp-server -p torrust-tracker --all-targets --all-features
    -- -D warnings`: clean. `cargo machete`: clean. `linter all`: exit `0`.
  - `git diff --stat` against `develop`: no change to `spawner.rs`, `request_buffer.rs`,
    `processor.rs`, `receiver.rs`, `testing/`, or `logs_parser.rs`.
  - Every commit-like hex string in changed docs is a `develop` commit (`0f1dcd28`), the
    toolchain hash, the aquatic commit, or an infohash. All commits are signed.
  - Pre-push checks were not re-run by the reviewer.
- Acceptance criteria:
  - AC1 PASS: `src/app.rs:434` passes `new_cancellation_token().child_token()`. The bootstrap
    test proves token propagation and the named outcome; it would also pass with the root token,
    so child-ness rests on inspection only.
  - AC2 PASS: `start_with_cancellation` -> `start_receive_loop` -> `run_udp_server_main` has no
    signal or `Halted` use; `global_shutdown_signal` appears only in `legacy_stop_requested`.
    Token-stop package test returns `Ok(())` and rebinds.
  - AC3 PASS: `udp_tracker.rs:81` builds `OwnedTask` eagerly as the argument of
    `supervise_receive_loop`, before the future is returned (SI-13 lesson applied). Outcome
    name asserted by the bootstrap test.
  - AC4 PASS: `admit_received` maps errors, `Interrupted`, and stream end to `Break`; the loop
    returns them; the component maps error and panic to failure. The one-line glue in the loop
    is untested but visible.
  - AC5 PASS: registration rollback test and mutation-proven drop-before-run test. See minor
    findings 3 and 4.
  - AC6 PASS: public signatures of `Server::start`, `Server::stop`, `Running`, `Spawner`,
    `LaunchRequest`, and `run_with_graceful_shutdown` are unchanged; contract tests pass
    unchanged; both regression tests pass.
  - AC7 PASS: `request_buffer.rs` and `processor.rs` unchanged; their tests pass.
  - AC8 PASS: `start_udp_ban_cleanup_job` path in `src/app.rs` unchanged.
  - AC9 PASS: `log_listener_startup` emits the same three messages in the same order with
    `UDP_TRACKER_LOG_TARGET`; "Started UDP tracker" is unchanged; M1 corroborates.
  - AC10 PASS: M1/M2 record the `main()` SIGTERM line, the cooperative-cancellation outcome for
    `udp_instance_0_127.0.0.1:16969`, exit `0`, and immediate rebind.
  - AC11 PASS: reviewer re-ran `linter all`, clippy, and focused tests.
  - AC12 PENDING: the reading "no regression" is sound (every after run exceeds every baseline
    run), but the literal criterion is not met and needs maintainer confirmation. The baseline
    spread (about 8.9%) also means this benchmark can only rule out large regressions; the
    expected D1 cost is far below its resolution.
- Hot path (D1, constraint 7): `cancelled()` is created once and pinned before the loop; the
  `select!` is `biased` with the token first. Event order (`UdpRequestReceived`, discard/ban
  checks, processor spawn, `force_push`, `UdpRequestAborted`) is unchanged. Only differences:
  the stop branch, the error mapping, the removed `yield_now()` before the stream-end error,
  and the `loop (in)` trace moving after admission.
- Ownership on every path: token-aware success, registration rollback, receive error, panic,
  component drop before/after first poll, and `JobManager` escalation (abort component ->
  drop `OwnedTask` -> abort loop) are all owned. Legacy halt, dropped halt sender, OS signal,
  launcher abort, launcher panic, and startup-notification failure all cancel or abort the
  loop through `OwnedReceiveLoop`. No panic path remains on the legacy drop path.
- Findings:
  - Major - Completion review missing: no `implementation-retrospective.md` and no
    progress-log rationale for skipping one. Reusable lessons exist (two-sided AC12 pass rule
    fails on improvement; pre-existing double-signal panic found by M3; stale HTTP/REST
    inventory rows found only during SI-14).
  - Major - Prose-first Arrange-Act-Assert evidence is recorded for the T2/T3 tests (T4 entry)
    but not for the two T5 legacy launcher tests (`launcher.rs`
    `it_should_stop_without_panicking_...` and `it_should_release_the_socket_when_the_legacy_launcher_task_is_aborted`).
  - Minor - Progress-log timestamps are inconsistent with commit times: the "15:30 UTC" T4 entry
    (`ISSUE.md:341`) precedes the "14:40 UTC" entry (`ISSUE.md:366`), and the recorded commits
    are at 12:57 and 13:26 UTC. Manual evidence (13:11-13:25 UTC) also overlaps the
    after-implementation load test (14:15-14:20 +01:00 = 13:15-13:20 UTC); state whether they
    ran concurrently.
  - Minor - `states.rs:229-236`: between spawning the receive loop and returning, the raw
    `JoinHandle` crosses `form.register(...).await` unguarded. If the start future is dropped
    there, the loop detaches until its token is cancelled. Not reachable today (startup is not
    raced), but it is a drop path under constraint 5; an abort guard would close it.
  - Minor - `states.rs:3-10` module "Test ownership" doc is stale: halt signalling and task
    joining are now covered in `launcher.rs`, token-aware start in `server::tests::token_aware_start`,
    and SI-14 is the UDP lifecycle sub-issue it defers to.
  - Minor - `udp_tracker.rs:143`: the test name says "after cancellation", but no cancellation
    occurs; the causal state is "the receive loop returned `Ok(())`". The three outcome tests
    (`udp_tracker.rs:146`, `:155`, `:169`) await `supervise_receive_loop` without a timeout.
  - Minor - D5 lists the tested behavior changes, but two more are untested or unlisted: a
    legacy receive error now makes `Server::stop` return `UdpError::Launcher` (no test), and a
    receive-loop panic now surfaces as `Err` instead of `Ok(())`. Legacy halt now waits for the
    current loop iteration instead of aborting immediately.
  - Nit - `mod.rs:389` cleanup `drop(running.task.await)` is unbounded. `UdpServerInputs`
    (`mod.rs:282`) reads as a parameter bag and `inputs.start(...)` hides the production call;
    the causal inputs (registar, token) remain visible, so this is naming only.
  - Nit - Trace messages in `start_receive_loop` (`launcher.rs:143`, `:146`) still say
    `Udp::run_with_graceful_shutdown` on the token-aware path. The "Halting UDP Service" info
    line (`launcher.rs:368`) moved from the `torrust_server_lib::signals` target to the
    launcher module target.
  - Nit - SI-19 draft (`1488-si-19-remove-legacy-shutdown-api/ISSUE.md:84`) lists three
    supervisors to consolidate; add UDP `OwnedTask` and the package-local `OwnedReceiveLoop`.
- Issue spec updates: none. AC1-AC11 were already checked and are verified; AC12 stays
  unchecked pending maintainer confirmation.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Maintainer: confirm the AC12 reading and preferably reword the rule to a one-sided bound
    (for example, after mean not below the baseline minimum); then tick AC12.
  - Implementer: add `implementation-retrospective.md` (or a progress-log rationale) and record
    the prose-first review of the T5 tests.
  - Implementer: correct progress-log timestamps and clarify the manual/performance overlap.
  - Implementer (optional, non-blocking): address the minor and nit findings, or record them for
    SI-15/SI-17/SI-19.
