---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: null
github-issue: 2518
spec-path: docs/issues/open/2518-http-first-announce-benchmark/ISSUE.md
branch: "2518-http-first-announce-benchmark-spec"
related-pr: null
last-updated-utc: "2026-10-14 09:12"
semantic-links:
  skill-links:
    - create-issue
    - run-benchmarks
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/benchmarking/run-benchmarks/SKILL.md
    - docs/benchmarking.md
    - docs/issues/closed/2479-fix-http-core-announce-benchmark/ISSUE.md
    - packages/http-core/benches/http_tracker_core_benchmark.rs
    - packages/http-core/benches/helpers/announce.rs
    - packages/http-core/benches/helpers/routine_guard.rs
    - packages/swarm-coordination-registry/examples/bench_peers.rs
---

<!-- skill-link: create-issue -->

# Issue #2518 - Benchmark First HTTP Announces into a Swarm of Fixed Size

## Goal

Add a Criterion benchmark that times one HTTP announce of a peer that is new to its swarm, for
swarms of a few fixed sizes, so that changes to peer insertion and peer-list building can be
measured. Today only repeat announces are measured.

## Background

Issue #2479 repaired `http_tracker/announce_once/announce_once`. By maintainer decision D3 of that
issue, it announces the same peer on every iteration, so after the first iteration it times a
repeat announce: the update of the only peer in a one-peer swarm, with an empty peer list in the
response. That keeps runs comparable, but the result is a lower bound on announce cost. Two parts
of a real announce are never measured:

1. **Inserting a new peer** (`Coordinator::upsert_peer` insert path, the swarm metadata update for
   an insert, and the "peer added" event) instead of replacing an existing entry.
2. **Building a non-empty peer list.** The response contains up to `numwant` peers, capped by
   `max_peers_per_announce` (default 74), so its cost depends on how many other peers the swarm
   has.

The obvious change, a new peer on every iteration, does not give stable numbers: Criterion runs
millions of iterations with no cleanup, so the swarm would grow by one peer per iteration and the
cost would drift with its size and with the iteration count Criterion picks. Issue #2479 sketched
the alternative as "a pre-filled swarm of fixed size and a bounded rotating peer pool" (its
decision D3), repeated in `docs/benchmarking.md`, section "HTTP announce microbenchmark", as "a
bounded rotating pool of peers". This specification keeps the pre-filled swarm but departs from
the rotating pool: each new peer is removed again after its timed announce (D1, option A), because
a rotating pool measures repeat announces after its first pass (D1, option E).

The example `packages/swarm-coordination-registry/examples/bench_peers.rs` times
`Coordinator::peers_excluding` alone. It does not include the announce service, authentication,
IP resolution, insertion, or the statistics event.

## Scope

### In Scope

- A new benchmark group in `packages/http-core/benches/http_tracker_core_benchmark.rs`, for
  example `http_tracker/first_announce/<swarm size>`, timing one awaited
  `AnnounceService::handle_announce` with event `started` for a peer that is not in the swarm.
- Before measurement, a swarm pre-filled with `N` other seeders, for `N` = 0, 74, and 1000 (D2,
  D3).
- Keeping the swarm at exactly `N` peers for the whole run (D1): each iteration times only the
  `started` announce of the new peer, and the peer then leaves the swarm again with a `stopped`
  announce outside the measured time.
- The new context wires the swarm registry's event sender and its statistics listener as
  production does (D4), instead of reusing the shared helper's
  `InMemoryTorrentRepository::default()`, whose registry sends no events.
- The `RoutineGuard` from issue #2479 on the new benchmark, plus maintained assertions after the
  run: the swarm still has exactly `N` peers, and the swarm metrics count only inserts and
  removals, no updates (D5). A benchmark whose swarm drifts or whose announces are not first
  announces fails under `cargo test --benches`.
- Documentation in `docs/benchmarking.md`: the catalog row, what the benchmark measures and
  excludes, how it relates to the repeat-announce benchmark, and example output.

### Out of Scope

- Changing announce, swarm, or peer-list production code, or its performance.
- UDP announce benchmarks. The UDP announce shares the tracker-core path; a UDP-specific benchmark
  would be a separate issue.
- The HTTP parsing and Axum server layers.
- Changing the existing repeat-announce benchmark, apart from shared helpers it can reuse. Its
  context also sends no swarm-registry events, but its documentation claims only the HTTP
  statistics event, which is accurate.

## Maintainer Decisions

Recorded on 2026-10-14 during the review of this draft. The maintainer accepted the proposed
option for each question and asked that the discarded options be kept here with their pros, cons,
and the reason they were not chosen.

### D1 - Keeping the Swarm at a Fixed Size: Time Only the Join (`iter_custom`)

**Chosen: A, `iter_custom` with the removal outside the measured time.** Criterion passes the
number of iterations, and the benchmark returns the time it measured. For each iteration it reads
the clock, awaits the `started` announce of a peer that is not in the swarm, reads the clock again,
and adds the difference; then it awaits the `stopped` announce of the same peer, which is not
measured.

- Pros:
  - Each measured time is exactly one first announce: insertion, the "peer added" event sent to
    the swarm statistics listener (wired as in production, D4), and a peer list built from `N`
    other peers.
  - The swarm is back to `N` peers before every iteration, so the result does not drift with the
    number of iterations.
  - It uses the public announce service only; no production code changes.
- Cons:
  - Two clock reads per iteration fall inside the measured time: tens of nanoseconds, about 5-10%
    of a roughly 350 ns announce.
  - The removal's statistics events may still be processed by the listener during the next
    measured announce, so a small part of the removal cost can leak in.
- Why it was chosen: it is the only option that measures the operation the issue is about, a
  single first announce, on a swarm of known size. Its main cost, the clock reads, is a constant
  that is the same in every run, so before-and-after comparisons of this benchmark stay valid; it is
  documented and the benchmark is never compared with `announce_once`, whose iterations are
  measured in bulk.

Discarded options:

- **B - Time the join and the leave together** (`b.to_async(&runtime).iter(...)` over a routine
  that announces `started` and then `stopped`).
  - Pros: no per-iteration clock reads; the simplest code; the swarm is bounded as in A.
  - Cons: the result is the sum of an insert and a removal, two operations with different costs,
    so a change to either one moves the same number and cannot be attributed. Removal also
    includes its own work (a "peer removed" event, metadata updates) that is not part of a first
    announce.
  - Why not: it does not measure a first announce; it measures a join-and-leave cycle.
- **C - Criterion's `iter_batched` with the removal in the setup closure.**
  - Pros: Criterion's own mechanism for work excluded from the measurement.
  - Cons: the setup closure is synchronous, while the removal is an async announce. Calling
    `block_on` inside it would block on the same Tokio runtime Criterion is already running the
    benchmark on, which panics. For batches larger than one, the setup runs for the whole batch
    before any measured routine, so the removals could not be interleaved anyway.
  - Why not: it cannot be implemented with the async removal.
- **D - A new peer every iteration, with the swarm rebuilt at the start of each sample.**
  - Pros: no removal at all, so nothing outside the measured time can leak in.
  - Cons: Criterion runs up to millions of iterations per sample, so the swarm still grows by that
    many peers inside one sample; the cost drifts with the iteration count, the problem issue
    #2479 identified.
  - Why not: the swarm size is not fixed during measurement.
- **E - A rotating pool of K peers announced in turn, without removal.** This is the design issue
  #2479 sketched ("a bounded rotating peer pool", its decision D3), which this specification
  departs from.
  - Pros: bounded swarm, no removal, no per-iteration clock reads.
  - Cons: after the first pass through the pool, every announce updates a peer that is already in
    the swarm, so it measures repeat announces again, which `announce_once` already covers.
  - Why not: it does not measure first announces.

### D2 - Swarm Sizes: 0, 74, and 1000

**Chosen: three sizes, 0, 74, and 1000 other peers.**

- 0 measures the insert path with an empty peer list.
- 74 is the default `max_peers_per_announce`, so the response carries a full default peer list.
- 1000 also returns a full list of 74 peers, but from a larger map, which shows whether swarm size
  alone, rather than list length, changes the cost.

Discarded options:

- **A single size (for example 74).**
  - Pros: shortest run time; one number to compare.
  - Cons: cannot separate the cost of inserting from the cost of building the peer list, and
    cannot show the effect of swarm size.
  - Why not: those two separations are the main reason for the benchmark.
- **Many sizes (for example powers of ten up to 100,000).**
  - Pros: a detailed scaling curve.
  - Cons: each size adds its own warm-up and measurement time to every `cargo bench` run; large
    swarms make the setup slow. The scaling of peer-list building alone is already measured by
    the `bench_peers` example.
  - Why not: three sizes answer the questions this benchmark is for at a fraction of the cost;
    more sizes can be added later if a change needs them.

### D3 - Peer Type: Seeders

**Chosen: the new peer and the pre-filled peers are seeders (`left = 0`)**, like the sample peer
the existing announce benchmark uses.

Peer selection for the response (`Coordinator::peers_excluding`) only excludes the announcing
peer's own address; it does not look at whether peers are seeders. The peer type therefore changes
the swarm's seeder and leecher counters, not the work of building the peer list.

Discarded options:

- **Leechers.**
  - Pros: closer to a client starting a download.
  - Cons: no measurable difference in what the benchmark exercises, while it would differ from the
    sample peer the existing benchmark uses.
  - Why not: consistency with the existing benchmark, at no loss of coverage.
- **A mix of seeders and leechers.**
  - Pros: resembles a real swarm.
  - Cons: adds a parameter that does not change peer selection and makes the setup harder to read.
  - Why not: as for leechers.

### D4 - Swarm-Registry Events: Wired as in Production

Recorded on 2026-10-14 during the review of PR #2519, in answer to
`review-finding:pr-2519-f3`. The shared bench helper builds
`InMemoryTorrentRepository::default()`, whose swarm `Registry` has no event sender, so the
`PeerAdded`, `PeerUpdated`, and `PeerRemoved` events are never sent. Production builds the
registry with `Registry::new(stats_event_sender)` in
`SwarmCoordinationRegistryContainer::initialize`, and a listener updates the swarm statistics
metrics.

**Chosen: the new benchmark context wires the registry's event sender and runs its statistics
listener as production does**, for example through `SwarmCoordinationRegistryContainer::initialize`
and `InMemoryTorrentRepository::new`.

- Pros:
  - The timed announce includes the event send, as in a running tracker, so D1's description of
    what is timed is true.
  - The listener's metrics give the benchmark a public seam that reports inserts, updates, and
    removals (D5).
- Cons:
  - The context no longer shares the swarm wiring of the repeat-announce benchmark.
  - The listener runs on the benchmark's Tokio runtime, so its work competes with the measured
    announces, as it does in production.
- Why it was chosen: the benchmark exists to measure production's first-announce cost, and the
  event is part of it.

Discarded option:

- **Keep the shared helper and remove the event from D1.**
  - Pros: simplest setup; the same wiring as the repeat-announce benchmark.
  - Cons: it measures less than production does, and it leaves no seam for counting inserts.
  - Why not: the maintainer chose the production-like measurement.

### D5 - Counting Inserts: A Maintained Assertion on the Swarm Metrics

Recorded on 2026-10-14 during the review of PR #2519, in answer to
`review-finding:pr-2519-f5`. The first draft planned a disposable probe to count inserts against
updates, justified by "the production code has no public seam reporting insert versus update".
That was false: `Coordinator::upsert_peer` sends `PeerAdded` or `PeerUpdated`, and with the sender
wired (D4) the statistics listener counts them in `swarm_coordination_registry_peers_added_total`,
`swarm_coordination_registry_peers_updated_total`, and
`swarm_coordination_registry_peers_removed_total`, readable through the swarm statistics
`Repository::get_metrics`.

**Chosen: a maintained assertion.** The swarm statistics listener subscribes to the event bus before
the swarm is pre-filled, because a broadcast receiver sees only events sent after it subscribes.
After the run, outside the measured time, the benchmark waits with a bounded deadline for the
listener to process the queued events, then asserts that:

- peers added equals the pre-filled peers plus every first announce the context made, timed or
  not;
- peers removed equals the departures the context made; and
- peers updated is 0.

The context counts its own announces; the `RoutineGuard` counts the timed ones. Before measurement,
the context also makes one first announce outside the measured time and asserts that its response
holds `min(N, 74)` peers, so the peer-list size is checked by maintained code too.

**Lag rule** (added on 2026-10-14 in answer to `review-finding:pr-2519-f8`). The swarm event bus is
a Tokio broadcast channel with a capacity of 65,536 events (`CHANNEL_CAPACITY` in
`packages/events/src/broadcaster.rs`). A listener that falls further behind receives
`RecvError::Lagged`, logs a warning, and never counts the skipped events. Under `cargo bench`,
millions of iterations each send two swarm events, so exact counts could fail even though every
announce was an insert. The benchmark therefore applies the exact counts above only when the run
sent no more swarm events than the channel capacity, so that no event can have been dropped; this
always holds under `cargo test --benches`, which runs one iteration. When the run sent more, it
asserts that peers updated is 0 and that peers added and peers removed are not above their expected
values, because a dropped event can only lower a count. The context knows how many events it sent
from its own announce count.

- Pros of the lag rule: the exact check runs in the pre-push hook and CI, where no lag is
  possible; long runs keep the checks that remain valid under lag; it depends on the number of
  events, not on detecting Criterion's mode.
- Cons: under long `cargo bench` runs, a missing insert is no longer detected, only an update.

Discarded lag rules:

- **Exact counts in Criterion's test mode only, no count checks under `cargo bench`.**
  - Pros: simplest rule.
  - Cons: the benchmark would need to detect Criterion's mode, which Criterion does not expose
    to the benchmark function, and long runs would lose the `updated == 0` check that stays valid.
  - Why not: the event-count rule gives the same exact check under test mode without detecting the
    mode, and keeps a meaningful check under `cargo bench`.
- **Under `cargo bench`, `updated == 0` plus a lower bound on the counts.**
  - Pros: would detect a large loss of inserts.
  - Cons: dropped events lower the counts by an amount that depends on scheduling, so no lower
    bound is both safe and meaningful.
  - Why not: any useful lower bound could fail on a correct run.

- Pros:
  - It runs on every `cargo test --benches` in the pre-push hook and CI, not once by hand.
  - It uses only seams production already has, and adds no work to the measured announce.
- Cons:
  - More benchmark code, and a bounded wait for an asynchronous listener after the run.
- Why it was chosen: a check that runs forever beats one that runs once, and the seam exists.

Discarded option:

- **Keep the disposable probe and give it a true rationale** (a counting sender in the timed path
  would add work production does not do).
  - Pros: no maintained code beyond the swarm-size assertion.
  - Cons: the insert count would be checked once by hand and never again.
  - Why not: the maintainer chose the maintained assertion.

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none known. The benchmark follows the pattern issue #2479 established.

## Design and Ownership Review

Not applicable: no child processes, network readiness, or reusable test fixtures. The benchmark
context owns its services for the benchmark's lifetime, as in issue #2479.

## Bug-Fix Process

Not applicable: this adds a measurement; nothing is broken.

## Regression Test Strategy

Not applicable: not a bug. The benchmark's own maintained checks (the `RoutineGuard`, the
swarm-size assertion, and the D5 swarm-metrics and response-size assertions) run under
`cargo test --benches` in the pre-push hook and CI.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                      | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                                                   |
| --- | ------ | ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | First-announce context with a fixed swarm | A benchmark context that wires the swarm-registry event sender and listener as production does (D4), pre-fills a swarm with `N` peers, and announces a new peer and removes it. Its maintained checks (the `RoutineGuard`, the swarm-size assertion, and the swarm-metrics and response-size assertions of D5) are each seen to fail under a deliberate mutation. Design review recorded. |
| T2  | TODO   | Benchmark group over the swarm sizes      | The `http_tracker/first_announce/<N>` group, using `iter_custom` as decided in D1; `cargo test --benches` green; `cargo bench` output recorded.                                                                                                                                                                                                                                           |
| T3  | TODO   | Maintainer review of the benchmark        | Stop after T2: the maintainer reviews the benchmark design, the guard evidence, and the first results before documentation and the pull request. Outcome logged in the Progress Log.                                                                                                                                                                                                      |
| T4  | TODO   | Documentation and completion review       | `docs/benchmarking.md` updated, including replacing the closing sentence of "HTTP announce microbenchmark", which still suggests "a bounded rotating pool of peers" (D1, option E), with a pointer to the new benchmark; manual verification recorded; acceptance criteria reviewed; completion review recorded.                                                                          |

## Commit Points

| Task | Coherent change set                         | Commit policy                                                                         |
| ---- | ------------------------------------------- | ------------------------------------------------------------------------------------- |
| T1   | Context and guards                          | One `test(http-core)` or `feat(http-core)` commit after focused validation and review. |
| T2   | Benchmark group                             | One commit after focused validation and the T3 maintainer review.                     |
| T3   | Maintainer review                           | No commit; recorded in the Progress Log.                                              |
| T4   | Documentation and verification evidence     | Separate `docs(benchmarking)` and `docs(issues)` commits.                             |

Use the `write-unit-test` skill for the guards and complete the prose-first Arrange-Act-Assert
design review after each passing increment, before the next task: the causal condition (a new peer
is announced into a swarm of exactly `N` peers) must be visible, the context must own only
incidental mechanics, and the timed announce must stay visible in the benchmark function. Record
the review in `manual-verification-evidence.md`. All commits are GPG signed Conventional Commits.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/http-first-announce-benchmark/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-14 07:03 UTC - Copilot - Drafted at the maintainer's request as the follow-up recorded
  in issue #2479 (decision D3 and its implementation retrospective). Open questions Q1-Q3 are for
  the maintainer's review.
- 2026-10-14 07:13 UTC - Copilot - The maintainer accepted the proposed option for each open
  question and asked that the discarded options be documented with their pros, cons, and the
  reason for the choice. Replaced Open Questions with Maintainer Decisions D1-D3. While writing D3,
  found that the draft's claim that leechers would change which peers the response returns was
  wrong: `peers_excluding` filters only the announcing peer's address. Corrected it.
- 2026-10-14 07:16 UTC - Copilot - The maintainer approved the specification and chose a
  spec-only PR first. Created GitHub issue #2518 (labels `task`, `performance`) and moved the
  specification from `docs/issues/drafts/` to `docs/issues/open/2518-http-first-announce-benchmark/`
  on branch `2518-http-first-announce-benchmark-spec`; the base name
  `2518-http-first-announce-benchmark` is reserved for the implementation.
- 2026-10-14 08:18 UTC - Copilot - PR #2519 review by Copilot: rewrote AC4 so the first-announce
  results are compared only with each other, never with `announce_once`
  (`review-finding:pr-2519-f1`), and made M2 read each swarm size's Criterion median right after
  every run (`review-finding:pr-2519-f2`).
- 2026-10-14 09:12 UTC - Copilot - PR #2519 review by Cameron (`da2ce7`): the benchmark context sent no
  swarm-registry events, so the "peer added" event D1 claimed was never timed;
  the maintainer chose to wire the events as production does (D4, `review-finding:pr-2519-f3`).
  The Background now quotes the rotating-pool design #2479 sketched and states the departure from
  it (`review-finding:pr-2519-f4`). The probe's "no public seam" rationale was false; the
  maintainer chose a maintained assertion on the swarm metrics instead (D5,
  `review-finding:pr-2519-f5`). This entry and the updated stamp answer
  `review-finding:pr-2519-f7`.

## Acceptance Criteria

- [ ] AC1: `cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark` reports
      one result per swarm size, each timing one `started` announce of a peer that was not in a
      swarm of exactly `N` other peers.
- [ ] AC2: The swarm has exactly `N` peers after the run; the benchmark asserts this and fails
      under `cargo test --benches` when it does not (shown by a deliberate mutation).
- [ ] AC3: The new benchmark uses the `RoutineGuard`, and the guard is shown to fail when the
      routine is not awaited.
- [ ] AC4: The results are ordered as the design expects: the 74- and 1000-peer results are above
      the 0-peer result, or the evidence explains why not. The results are compared only with each
      other, never with `announce_once`, which is measured differently (D1).
- [ ] AC5: `docs/benchmarking.md` lists and explains the benchmark, including what its timing
      includes (the per-iteration clock reads, D1) and why the discarded options were not used.
- [ ] AC6: After the run, the benchmark asserts from the swarm statistics metrics that every
      announce the context made was an insert or a removal, with no updates, applying exact counts
      only under the D5 lag rule, and that the first announce made before measurement returned
      `min(N, 74)` peers (D5). Each assertion is shown to fail under a deliberate mutation, for
      example announcing the same new peer twice.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-http-core --benches` (stable Rust toolchain)
- `linter all`
- Pre-commit and pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                   | Human-oriented command/steps                                                                                                                                                                                                                                                                                                                                                                                         | Expected Result                                                                                                                                                                     | Status | Evidence                                     |
| --- | ------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Each iteration is a first announce         | Run `cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark` and `cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark`; then, without staging, apply each mutation (the same new peer announced twice; the departure skipped), rerun `cargo test`, record the failure, and restore                                                                                     | Both runs without a mutation pass all assertions (D5); each mutation fails with a message naming the benchmark, the swarm size, and the counted inserts, updates, removals, or swarm size    | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Results are stable and ordered as expected | Run `cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark` three times. Before each run, record `uptime`. Immediately after each run, before the next one, read the median of every swarm size from `target/criterion/http_tracker_first_announce/<N>/new/estimates.json`, because Criterion overwrites that file on every run (`docs/benchmarking.md`, Recording Before-and-After Evidence) | A table with three medians per swarm size, the load average of each run, and the toolchain (`rustc --version`); the run-to-run variation per size is stated; the medians follow AC4 | TODO   | `manual-verification-evidence.md` section V2 |

### Disposable Verification Scripts

None planned. The first draft planned an insert-versus-update probe, justified by a missing public
seam; that seam exists, so the checks it would have made are maintained assertions (D5). The only
possible temporary variant is the drain-wait measurement in Risks, used only if the removal leak
proves material; it would be recorded verbatim in `manual-verification-evidence.md` and reverted,
because it measures a one-off question rather than a durable invariant.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                    |
| ----- | ---------------------- | ----------------------------------------------------------- |
| AC1   | TODO                   | `cargo bench` output, M1, M2                                |
| AC2   | TODO                   | Swarm-size assertion mutation run                           |
| AC3   | TODO                   | `RoutineGuard` mutation run                                 |
| AC4   | TODO                   | M2                                                          |
| AC5   | TODO                   | `docs/benchmarking.md` diff                                 |
| AC6   | TODO                   | Swarm-metrics and response-size assertion mutation runs, M1 |

## Risks and Trade-offs

- **Dropped swarm events (D5).** The swarm event bus holds 65,536 events; a listener that falls
  further behind skips events, so exact counts can fail on long `cargo bench` runs. Mitigation: the
  D5 lag rule applies exact counts only when the run sent no more events than the capacity, and
  otherwise checks only what lag cannot break.
- **Timer overhead (D1).** Per-iteration clock reads add a constant to every result. Mitigation:
  documented; comparisons are made between runs of the same benchmark, never against
  `announce_once`, whose iterations are timed in bulk.
- **Removal work leaking into the measurement.** If the `stopped` announce queues statistics
  events that the listener processes during the next timed announce, part of the removal cost is
  measured. Mitigation: record it in the evidence. If it is material, check it by waiting for the
  listener to drain between iterations in a temporary probe.
- **Code-layout shifts.** Adding a benchmark group can move other results in the same binary by a
  few nanoseconds, as issue #2479 measured for `connect_once`. Mitigation: re-measure
  `announce_once` before and after and record any shift.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory;
  otherwise add a progress-log entry explaining why the work had no material discovery.

## References

- Related issues: #2479 (repaired the HTTP announce benchmark; decision D3 left first announces
  out of scope)
- Related PRs: #2509
- Related documentation: `docs/benchmarking.md` (HTTP announce microbenchmark; Routine Guard for
  Async Benchmarks)
