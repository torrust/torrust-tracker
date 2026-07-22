---
doc-type: issue
issue-type: enhancement
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/simplify-udp-server-main-loop.md
branch: "{issue-number}-simplify-udp-server-main-loop"
related-pr: null
last-updated-utc: 2026-07-22 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/src/server/request_buffer.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Simplify the UDP server main request loop

## Goal

Reduce the complexity of `Launcher::run_udp_server_main` in
`packages/udp-server/src/server/launcher.rs` without degrading the performance of
the UDP request hot path, and remove a per-request allocation that serves no
purpose.

## Background

`run_udp_server_main` is the core request loop of the UDP tracker server. It has
grown to ~120 lines mixing several concerns:

1. **Setup**: active-requests buffer, server service binding, ban-cleaner task spawn.
2. **Receive loop**: `receiver.next().await`, I/O error classification
   (`Interrupted` → return, other → break, `None` → break).
3. **Per-request policy pipeline**: emit `UdpRequestReceived` → discard port-0
   requests → discard banned IPs → spawn `Processor::process_request` →
   `force_push` into `ActiveRequests` → emit `UdpRequestAborted` on eviction.
4. **Event-emission boilerplate**: the pattern
   `if let Some(sender) = container.stats_event_sender.as_deref() { sender.send(Event::X { ... }).await }`
   is repeated four times (`UdpRequestReceived`, `UdpRequestDiscarded`,
   `UdpRequestBanned`, `UdpRequestAborted`).

There is also a genuine hot-path inefficiency: `server_service_binding` is
recreated with `ServiceBinding::new(...).expect(...)` **on every loop iteration**,
even though an identical value is already constructed once before the loop. The
per-iteration copy only exists so the last event-emission arm can consume it by
value. This is a per-request allocation + validation with no benefit.

### Performance constraints (why this needs care)

This function is a critical hot path: the UDP server handles thousands of
requests per second. Per-request costs today are dominated by the `recvfrom`
syscall (~1–2 µs), `tokio::task::spawn` (allocation + ~100s of ns), and event
channel sends. A statically-dispatched function call is ~1 ns and is typically
inlined to zero cost. Therefore:

- Extracting private `async fn`s with **static dispatch** is free: the compiler
  merges them into the same state machine and expands them inline.
- What must **not** be introduced: dynamic dispatch (`Box<dyn ...>`, trait
  objects), extra `Arc` clones, per-request heap allocations, or additional
  channel hops.
- Hoisting the per-iteration `ServiceBinding::new` **improves** performance.

### Test coverage caveat

`run_udp_server_main` has no unit tests; it is exercised only by integration
tests (`tests/integration.rs`, `packages/udp-server` contract tests) and E2E
suites. The refactoring must be mechanical and reviewed carefully against the
existing behavior, and should be validated with the full integration suite plus
the UDP load-test benchmarks where practical.

## Scope

### In Scope

- Hoist `server_service_binding` creation out of the request loop (create once,
  clone only where an event is actually emitted).
- Extract the ban-cleaner task spawn into a private helper (setup noise).
- Extract the per-request policy pipeline into a private `handle_request`
  helper, keeping the receive/error-classification concern in the main loop.
- Introduce a small private context struct (e.g. `RequestDispatchContext`)
  holding the per-server loop state built once before the loop:
  containers, service binding, socket, `local_addr`, `cookie_lifetime`.
- Collapse the 4× event-emission boilerplate into a single
  `send_event(&self, event: Event)` helper on the context struct.
- Verify no performance regression (see Verification Plan).

### Out of Scope

- Any change to request-processing semantics (event order, discard/ban
  behavior, force-push/eviction policy).
- Restructuring `Processor`, `ActiveRequests`, or the stats event system.
- Middleware/trait-based request pipeline abstractions (speculative generality,
  dynamic-dispatch risk).
- Changes to `run_with_graceful_shutdown`.
- Adding unit tests for the loop itself (would require raw-socket tooling; the
  existing integration/E2E coverage remains the safety net).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                              | Notes / Expected Output                                                                                      |
| --- | ------ | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Hoist per-iteration `ServiceBinding` recreation   | `ServiceBinding::new` called once before the loop; per-request allocation removed; behavior identical        |
| T2  | TODO   | Extract ban-cleaner spawn helper                  | `spawn_ban_cleaner(...)` private fn; `run_udp_server_main` setup section shrinks                             |
| T3  | TODO   | Introduce `RequestDispatchContext` + `send_event` | Private struct built once; 4× event boilerplate collapsed; static dispatch only; no new `Arc` clones per req |
| T4  | TODO   | Extract `handle_request` policy pipeline          | Main loop reduced to receive + error classification + `handle_request` call; diff reviewed line-by-line      |
| T5  | TODO   | Run full test suite + benchmarks                  | All unit/integration/E2E tests pass; UDP benchmark comparison shows no regression                            |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-22 00:00 UTC - Copilot - Draft spec created from design discussion in PR #2017 (port-0 discard work highlighted the loop's complexity) - https://github.com/torrust/torrust-tracker/pull/2017

## Acceptance Criteria

- [ ] AC1: `run_udp_server_main` contains only setup, the receive loop, and I/O
      error classification; the per-request policy pipeline lives in a private
      helper.
- [ ] AC2: `ServiceBinding` is constructed once per server (not once per
      request); no new per-request heap allocations or `Arc` clones are
      introduced relative to the current code.
- [ ] AC3: Event-emission boilerplate is expressed once (single `send_event`
      helper); event order and payloads are unchanged for all four events.
- [ ] AC4: No dynamic dispatch (`dyn`) is introduced anywhere in the request
      hot path.
- [ ] AC5: UDP benchmark comparison (before/after) shows no measurable
      throughput/latency regression.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --package torrust-tracker-udp-server`
- `cargo test --test integration`
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Command/Steps                                                                                | Expected Result                                                 | Status | Evidence |
| --- | ------------------------------------- | -------------------------------------------------------------------------------------------- | --------------------------------------------------------------- | ------ | -------- |
| M1  | UDP benchmark before/after comparison | Run the UDP load-test benchmarks (see `docs/benchmarking.md`) on `develop` and on the branch | No measurable throughput/latency regression                     | TODO   |          |
| M2  | Stats events still emitted correctly  | Start tracker, send normal / port-0 / banned-IP traffic, check REST API stats counters       | `received`, `discarded`, `banned`, `aborted` counters unchanged | TODO   |          |
| M3  | Graceful shutdown still works         | Start tracker, send SIGINT while under light load                                            | Clean shutdown, no panics, halt log lines present               | TODO   |          |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |

## Risks and Trade-offs

- **Hot-path regression risk**: mitigated by restricting the refactor to static
  dispatch and zero new allocations, plus benchmark comparison (M1/AC5).
- **Behavioral drift in a function with no unit tests**: mitigated by a
  mechanical, step-per-commit refactor (T1–T4 as separate commits), careful
  diff review, and the full integration/E2E suite.
- **`#[instrument]` spans**: extracting helpers may change tracing span
  structure; keep `#[instrument]` placement equivalent or deliberately document
  the change.

## References

- Related PRs: [#2017](https://github.com/torrust/torrust-tracker/pull/2017)
  (port-0 discard work that surfaced this complexity)
- Related file: `packages/udp-server/src/server/launcher.rs`
  (`Launcher::run_udp_server_main`)
- Benchmarking guide: `docs/benchmarking.md`
