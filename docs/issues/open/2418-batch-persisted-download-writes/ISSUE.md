---
schema-version: 1
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
epic: null
github-issue: 2418
spec-path: docs/issues/open/2418-batch-persisted-download-writes/ISSUE.md
branch: "2410-process-queued-events-before-listeners-stop-spec"
related-pr: null
last-updated-utc: "2026-10-02 18:45"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2418 - Batch Persisted Download Writes

## Goal

The persistent completed-downloads listener keeps up with the completion rate
under load, so its event backlog stays small in normal operation and at
shutdown.

## Background

With `persistent_torrent_completed_stat` enabled, the tracker-core persistent
listener handles every `PeerDownloadCompleted` event with two database writes
(`handle_persistent_completed_statistics_event` in
`packages/tracker-core/src/statistics/event/handler.rs`):

1. `increase_downloads_for_torrent(info_hash)`
2. `increase_global_downloads()`

The #1488 SI-22 reproduction measured roughly 134 to 370 completions per
second (SQLite, `dev` build, `info` logging). Faster completion bursts build a
backlog in the event channel (capacity 65,536). A full channel would need
about 3 to 8 minutes to drain, and events are lost if the channel overflows
(`Lagged`) or the tracker stops first. SI-22 adds ordered draining and loss/failure
reporting subject to deadlines; it does not guarantee zero loss or make
persistence faster.

Decision: SI-22 D18 (Q8). This issue is implemented **after the shutdown
refactor (#1488) finishes**.

## Scope

### In Scope

- Measure the current rate in a `release` build for each database driver
  (SQLite, MySQL, PostgreSQL), reusing SI-22's T4 measurement if available.
- Batch the writes, for example: take every event already queued, aggregate
  completions per info hash, and write them in one transaction (one upsert
  per torrent plus one global increment).
- Measure again and record the improvement.

### Out of Scope

- Shutdown draining (SI-22).
- Changing what is persisted or the database schema, unless the measurement
  shows it is needed; that needs a separate decision.

## Design Notes

To decide during implementation:

- Batch trigger: take what is already queued (no added latency), a maximum
  batch size, a time window, or a mix.
- Failure handling: distinguish a confirmed rollback, a committed transaction,
  and an unknown commit outcome. Retain a failed batch until the retry/drop
  policy decides its fate. Do not blindly retry additive increments after an
  ambiguous commit: they can double-count. Choose bounded retry and reporting
  semantics with the maintainer before driver changes; do not call an unknown
  outcome a known count of lost completions.
- Interaction with SI-22's shutdown drain: the drain should flush the current
  batch before the listener stops.
- Whether `increase_global_downloads` can be one `+N` update per batch.

## Architectural Decisions

- Related ADRs: the SI-22 ADR on listener draining (draft in the SI-22 folder).
- ADRs to create: none known.

## Design and Ownership Review

Asynchronous I/O is involved. Before implementation, define who owns a batch
that is in progress at shutdown and the deadline that bounds its final write.
Keep batch size/memory bounded and document failure/drop paths. SI-22 checks
its admission timeout between handlers; a batch handler may cross that timeout
and still be interrupted by the shared deadline. Review the first passing
aggregation-plus-driver slice before extending it to every driver.

## Bug-Fix Process

Not applicable: this is a performance improvement.

## Regression Test Strategy

Not applicable. Tests cover aggregation (unit) and the persisted totals after
a burst (integration in `packages/tracker-core/tests/`).

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | BLOCKED | Baseline measurement | After #1488 closes: completions per second per driver, release build, agreed representative workload and improvement target |
| T2 | BLOCKED | Aggregation | After T1: bounded batch policy and pure function, unit tested |
| T3 | BLOCKED | Batched writes | After T2 and failure-policy approval: atomic per-torrent/global updates, per driver; first-slice design review |
| T4 | BLOCKED | Shutdown flush | After T3: flush current batch; explicitly test shared-deadline interruption and unknown outcomes |
| T5 | BLOCKED | Measurement after | After T4: same workload/environment as T1, rate and latency/backlog measurements |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Aggregation and tests | Commit after focused validation and review. |
| T3 | Driver changes | One commit per driver if they differ. |
| T4 | Shutdown flush | Commit after focused validation and review. |
| T1, T5 | Measurements | Commit with the spec updates. |

Any shared database-trait change includes all implementations and callers in
one compiling commit, or keeps an additive adapter while drivers migrate.
Follow `write-unit-test` in small increments: validate, review prose-first
Arrange-Act-Assert design, and obtain maintainer review before continuing.

## Acceptance Criteria

- [ ] AC1: From recorded nonzero baselines, per-torrent deltas match each
  torrent's completions and the global delta matches their sum, on all drivers.
- [ ] AC2: Rate meets the target agreed after T1, with comparable before/after
  numbers and no unreviewed latency or memory regression.
- [ ] AC3: Rollback, retry, discard, and unknown outcomes are tested and logged
  distinctly; retry cannot silently double-count an ambiguous commit.
- [ ] AC4: Shutdown flushes the batch within available time, or reports the
  deadline/failure outcome without claiming persistence completed.
- [ ] `linter all` exits with code `0`; relevant tests pass.

## Verification Plan

### Automatic Checks

- Unit tests for aggregation and batch bounds; collaboration tests for each
  database driver's transaction semantics, nonzero counters, rollback/retry,
  and final-flush/deadline behavior.
- `linter all`, focused package tests, and required pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Expected Result | Status |
| --- | --- | --- | --- |
| M1 | Release baseline and batched burst on each driver | Same inputs, initial counts, hardware, logging and database settings; record throughput, latency, backlog, and final count deltas | BLOCKED |
| M2 | Shutdown during a partial batch | Final flush or explicit failure/deadline outcome, with database state inspected after exit | BLOCKED |

Record exact commands, toolchain/database versions, configuration, outputs,
and logs in issue-local `manual-verification-evidence.md`.

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1 | TODO | Driver tests and M1 |
| AC2 | TODO | Approved target and comparative measurements |
| AC3 | TODO | Fault-injection and retry tests |
| AC4 | TODO | Shutdown tests and M2 |

## Progress Tracking

### Workflow Checkpoints

- [x] Draft prepared; implementation blocked on #1488 completion
- [x] GitHub issue created with the dependency recorded
- [ ] Baseline and failure policy reviewed before driver changes
- [ ] First passing vertical slice reviewed
- [ ] Implementation and automatic/manual validation complete
- [ ] Acceptance criteria re-reviewed with independent Task Reviewer report

### Progress Log

- SI-22 D18 authorizes this separate improvement after #1488, not as part of
  the shutdown bug fix. Release results and final batch policy remain pending.

## Implementation Completion Review

Assess whether `implementation-retrospective.md` is needed and record the
reason if not. Obtain independent Task Reviewer findings in
`agent-review-reports.md` before the implementation PR; spec approval is not
evidence that the performance target has been met.

## References

- Related issues: #1488 (SI-22 D18)
- Related code: `packages/tracker-core/src/statistics/event/handler.rs`,
  `packages/tracker-core/src/statistics/persisted/downloads.rs`
