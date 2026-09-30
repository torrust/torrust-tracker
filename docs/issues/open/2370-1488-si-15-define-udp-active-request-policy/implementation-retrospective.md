---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md
    - docs/issues/open/2370-1488-si-15-define-udp-active-request-policy/agent-review-reports.md
    - docs/issues/open/2370-1488-si-15-define-udp-active-request-policy/manual-verification-evidence.md
    - docs/issues/open/2370-1488-si-15-define-udp-active-request-policy/performance-evidence.md
    - packages/udp-server/docs/adrs/20260929181216_bound_udp_request_concurrency_with_task_per_request_ring.md
---

# Implementation Retrospective — Define UDP Active-Request Shutdown Policy

## Purpose

Record evidence-based process improvements discovered while implementing
issue #2370 (SI-15 of EPIC #1488).

## Outcome

The UDP receive loop now owns every request processor in a `JoinSet`. The
eviction ring keeps its overload decision but no longer owns the tasks. On
cancellation the loop stops admitting datagrams and drains its processors for
up to five seconds. It then aborts and joins the rest and logs one summary of
`completed`, `failed`, `aborted`, and `evicted` counts. A receive error aborts
and joins every processor before returning. Processors return
`Result<(), ProcessorError>`.

Evidence:

- The orphaned-processor regression test was red 10 of 10 times, is green 10
  of 10 times, and fails again when the ownership wiring is reverted.
- B0, B1, and B2 mean throughput: 159413.39, 164634.37, and 167716.20
  responses/s.
- M1-M3 exit 0 with reconciled summaries and immediate rebind.
- M4 reproduces the known SI-17 panic unchanged.

## What Went Well

1. Recording a benchmark checkpoint per decision (B0 baseline, B1 for D7, B2
   for the `JoinSet`) made each hot-path change measurable and separately
   revertible. None regressed.
2. Holding real processors open through the injected event `Sender` gave a
   deterministic reproduction of an internal bug with no production hook, as D9
   required. The bug was unobservable from the executable boundary (V1
   `Trigger only`).
3. Every new regression test was mutation-proven: the ownership wiring (V2),
   the receive-error join, and the drain continuing after a panic. The
   Task Reviewer found two tests that passed against broken code, and mutation
   proofs confirmed the fixes.

## What Changed During Implementation

- **D9 test mechanics.** D9 planned paused-time tests that assert the summary
  counts. The unit tests of the drain primitive do that. The receive-loop
  collaboration tests drive a real loopback socket instead, so they use real
  time: a 100 ms injected drain deadline, `LIFECYCLE_TIMEOUT` bounds, and
  `wait_until_bindable` polling. They assert the loop result, the running
  processor count, and the response or rebind rather than counts. Paused time
  is unsuitable with pending socket IO, because auto-advance would fire the
  timeouts. The outcome is decided only by the release gate, not by timing.
  This needed a package-private
  `start_receive_loop_with_request_drain_deadline` entry point, because the
  five-second production deadline equals the test lifecycle bound.
- **Receive-error join had no test.** The Task Reviewer found that deleting
  `JoinSet::shutdown` left every test green. A real receive error cannot be
  produced reliably on loopback, so the branch was extracted into
  `join_request_processors_after_receive_error` and unit-tested. The test fails
  when the join is replaced by `abort_all`.
- **Panic test ordering.** The original panicking processor yielded first, so
  it was joined last, and a drain that stopped after a panic still passed. A
  `oneshot` now makes the surviving processor finish only after the panic.
- **Clippy shaped the code.** The `tracing` macros pushed
  `drain_request_processors_on_shutdown` over the cognitive-complexity limit,
  so the start and outcome logs became helpers. `unused_async` and
  `manual_async_fn` conflict for a test helper without `.await`, so the
  panicking processor is written inline.
- **Summary semantics.** Because finished processors are reaped before each
  spawn, the drain summary is a snapshot. M2 saw 85 evictions but
  `evicted=0`. This is correct under D3 but surprising, so the ADR's Known
  Trade-offs and the task inventory now explain it.
- **The ADR did not state the orphaning trade-off.** The review found that
  neither ADR said `force_push` can drop up to 48 live handles. The ADR now
  records it under Known Trade-offs and describes `force_push` precisely.

## Root Cause

- The spec named the receive-error join as a required behavior but assigned it
  no test in the D9 levels, so no plan step made it testable.
- The spec described the ADR's content as "history and evidence" but did not
  require the trade-off that motivated the issue to be written in the ADR
  itself. It stayed in the issue spec.
- The executable-boundary protocol does not say how to sequence a load
  generator around a restart. A generator still running with old connection
  cookies triggered the IP ban on 127.0.0.1 (see Failures and Follow-up in the
  manual evidence).
- The benchmark procedure did not require isolated, nonpersistent tracker
  state per run, so the first B0 attempt accumulated SQLite state and was
  invalid.

## Improvements for Future Work

1. When a spec lists a failure-path behavior (for example, "join before
   returning an error"), give it a named test or an explicit
   "untestable because ..." note in the test plan, and mutation-prove it.
2. When an ADR exists because of a discovered defect or trade-off, record that
   trade-off in the ADR itself, not only in the issue spec.
3. Executable-boundary scenarios that restart a server after load must stop
   the load generator before the restart. Otherwise stale connection cookies
   can trigger connection-ID IP bans.
4. UDP throughput checkpoints should use a fresh tracker process per run with
   nonpersistent state; `b0-tracker.toml` is a working example. Propose adding
   this to `docs/benchmarking.md` in a follow-up rather than here.
5. When measuring stop time from outside, prefer in-process log timestamps.
   `tail --pid` polls roughly once a second and inflated M2 from 1.8 ms to
   1108 ms.

## Avoiding Overcorrection

- Do not add production injection seams to make receive errors observable.
  A small extracted function tested with a `JoinSet` is enough.
- Do not replace the eviction ring or make the bound exact here. D8 keeps that
  outside SI-15 and requires its own benchmarked issue.
- Do not require paused time for every lifecycle test. It fits pure task
  orchestration, not real sockets.
- Improvements 3-5 are harness guidance; they do not justify new tooling yet.

## Evidence

- Issue specification: [ISSUE.md](ISSUE.md) (T1-T9, D1-D9, progress log)
- Review: [agent-review-reports.md](agent-review-reports.md)
- Manual evidence: [manual-verification-evidence.md](manual-verification-evidence.md)
- Benchmarks: [performance-evidence.md](performance-evidence.md)
- ADR: [request-concurrency ADR](../../../../packages/udp-server/docs/adrs/20260929181216_bound_udp_request_concurrency_with_task_per_request_ring.md)
- Commits (by subject; branch ids change on rebase):
  - T1 `docs(udp-server): [#2370] record shutdown baseline`
  - T2 `docs(adrs): [#2370] document UDP request concurrency`
  - T3 `test(udp-server): [#2370] add request drain and orphan regression test`
  - T4 `feat(udp-server): return a Result from UDP request processors`
  - T5 `fix(udp-server): [#2370] own and drain UDP request processors on shutdown`
  - T6 `docs(udp-server): [#2370] link the request drain to its ADR`
  - T7 `docs(shutdown): [#2370] document owned and drained UDP request processors`
  - T9 `test(udp-server): [#2370] prove the receive-error join and drain after a panic`
