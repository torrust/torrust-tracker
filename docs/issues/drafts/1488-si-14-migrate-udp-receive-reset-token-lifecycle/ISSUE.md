---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/app.rs
    - src/bootstrap/jobs/udp_tracker.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/spawner.rs
    - packages/udp-server/src/server/states.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/issues/drafts/1488-si-2-remove-global-shutdown-signal/ISSUE.md
    - docs/issues/drafts/1488-si-9-improve-udp-shutdown/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-14 — Migrate UDP Receive Loop to Token Lifecycle

> **EPIC position**: Roadmap step 10. One independently releasable UDP
> ownership slice. Active-request policy remains unchanged and is addressed next.

## Goal

Migrate the UDP tracker's top-level shutdown wait and receive loop to an
injected component `CancellationToken`. The UDP component must stop admission
of new UDP packets on cancellation, cancel and join the receive task it owns,
and report one named UDP component outcome to `JobManager`.

The current bounded `ActiveRequests` behavior and request-processor abort
semantics remain unchanged. They are a safe compatibility fallback until the
separate active-request-policy task defines deadlines, outcomes, and metrics.

## Current State

`src/bootstrap/jobs/udp_tracker.rs` starts a `Server<Running>` and returns a
wrapper handle to `JobManager`. The wrapper already receives the manager token,
forwards cancellation to its private `Halted` sender, and awaits the launcher.
In `packages/udp-server/src/server/launcher.rs`, the launcher spawns a
receive-loop task and directly awaits the private halt channel or library-level
OS signal in its `select!`, then aborts the receive loop when that wait completes.

UDP IP-ban cleanup is instead an application-level `udp_ban_cleanup` job. It is
already manager-owned, receives the manager cancellation token, and is not a
per-listener UDP child task. Active request processors are bounded through
`ActiveRequests` abort handles; this draft preserves that implementation and
does not change its policy.

## Scope

### In scope

- Add a token-aware UDP server start path while preserving legacy `Halted`
  start/stop behavior for consumers not yet migrated.
- Derive one UDP component child `CancellationToken` per configured UDP binding
  in `src/app.rs`.
- Replace the token-aware path's halt-signal task with cancellation waiting;
  it must not subscribe to OS signals.
- Retain and join the UDP receive loop after cancellation stops admission.
- Report one named `udp_instance_<index>_<address>` outcome to `JobManager`.
- Add deterministic cancellation tests and focused bootstrap propagation tests
  without OS signals.
- Add manual SIGTERM evidence after SI-1, confirming the migrated UDP component
  follows the token path from `main()`.

### Out of scope

- Changing `ActiveRequests` capacity, replacement behavior, or request-processor
  abort semantics.
- Adding active-request drain deadlines, completion/abort counters, or new UDP
  shutdown metrics.
- Altering HTTP, REST API, health-check API, or standalone UDP environment
  consumers.
- Removing/deprecating the legacy `Halted` API or `global_shutdown_signal()`.
- Final shutdown deadline values, configuration, and process exit codes.

## Implementation Constraints

1. Existing `Server::start` / `Server::stop` consumers remain source- and
   behavior-compatible until the separate deprecation/removal work.
2. The token-aware UDP path has no `SIGINT` or `SIGTERM` subscription inside the
   UDP server package.
3. Cancellation must stop new packet admission before the receive loop is
   joined. Existing request processors may still follow the current deliberate
   abort fallback.
4. The UDP component owns and joins its receive task before returning its
  top-level outcome. `JobManager` receives only that top-level handle and
  outcome, not nested UDP task handles.
5. Unexpected receive-loop completion/failure and cancellation races return an
  explicit component outcome; they must not panic or leave the receive-loop
  handle detached.

## Acceptance Criteria

- [ ] Each configured UDP tracker instance receives a component child
      `CancellationToken` derived from the `JobManager` root token.
- [ ] Token cancellation stops the UDP component without an OS-signal listener
      or shutdown `Halted` channel in its token-aware path.
- [ ] The UDP component stops packet admission and awaits the receive-loop task
      before reporting its named outcome to `JobManager`.
- [ ] The application-level UDP IP-ban cleanup job remains manager-owned,
  token-cancellable, and separate from each UDP listener component.
- [ ] Existing `ActiveRequests` capacity and deliberate request-processor abort
      behavior are unchanged and explicitly covered by a compatibility test.
- [ ] Deterministic tests cover injected-token cancellation and unexpected
      receive-loop completion/failure without OS signals.
- [ ] A bootstrap integration test proves root-token cancellation reaches a UDP
      instance without delivering an OS signal.
- [ ] Manual SIGTERM verification records the `main()` signal event followed by
      the migrated UDP component's completion.
- [ ] Legacy UDP start/stop consumers compile and preserve behavior.
- [ ] `linter all` passes.

## Dependencies

- The additive token-aware server lifecycle API from SI-2 is available and
  released.
- SI-1 is required only for manual SIGTERM verification.
- The subsequent UDP active-request-policy work depends on this migration; Q4
  defines its final component deadline.

## Rollback

Restore only the UDP tracker bootstrap and server call sites to the existing
legacy lifecycle path. The additive token-aware API remains available but
unused; no HTTP, REST API, health-check, or standalone UDP consumer changes.
The pre-existing `ActiveRequests` behavior is unchanged by this task.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run focused UDP component and bootstrap tests that cancel an injected token,
   recording their output.
2. Run the tracker with one UDP binding. After SI-1, send SIGTERM to the tracker
   binary and record the `main()` signal event followed by UDP component
   completion.
3. Confirm a legacy UDP start/stop call path still compiles and retains current
   behavior.
4. Review the token-aware path to confirm it retains and awaits the receive
  task and contains no OS-signal listener.
5. Confirm the active-request implementation is unchanged other than any
   necessary ownership wiring; defer behavior changes to the next UDP task.
