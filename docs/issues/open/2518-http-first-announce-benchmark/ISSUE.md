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
last-updated-utc: "2026-10-14 07:16"
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
cost would drift with its size and with the iteration count Criterion picks. Issue #2479 recorded
the fix as a pre-filled swarm of fixed size and a bounded set of peers; see `docs/benchmarking.md`,
section "HTTP announce microbenchmark".

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
- The `RoutineGuard` from issue #2479 on the new benchmark, plus an assertion after the run that
  the swarm still has exactly `N` peers, so a benchmark whose swarm drifts fails under
  `cargo test --benches`.
- Documentation in `docs/benchmarking.md`: the catalog row, what the benchmark measures and
  excludes, how it relates to the repeat-announce benchmark, and example output.

### Out of Scope

- Changing announce, swarm, or peer-list production code, or its performance.
- UDP announce benchmarks. The UDP announce shares the tracker-core path; a UDP-specific benchmark
  would be a separate issue.
- The HTTP parsing and Axum server layers.
- Changing the existing repeat-announce benchmark, apart from shared helpers it can reuse.

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
  - Each measured time is exactly one first announce: insertion, the "peer added" event, and a
    peer list built from `N` other peers.
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
- **E - A rotating pool of K peers announced in turn, without removal.**
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

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none known. The benchmark follows the pattern issue #2479 established.

## Design and Ownership Review

Not applicable: no child processes, network readiness, or reusable test fixtures. The benchmark
context owns its services for the benchmark's lifetime, as in issue #2479.

## Bug-Fix Process

Not applicable: this adds a measurement; nothing is broken.

## Regression Test Strategy

Not applicable: not a bug. The benchmark's own guards (the `RoutineGuard` and the swarm-size
assertion) run under `cargo test --benches` in the pre-push hook and CI.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                       | Notes / Expected Output                                                                                                                                                                                                                                  |
| --- | ------ | ------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | First-announce context with a fixed swarm  | A benchmark context that pre-fills a swarm with `N` peers and announces a new peer and removes it. Its guards (the `RoutineGuard` and the swarm-size assertion) are seen to fail under a deliberate mutation, for example removal skipped. Design review recorded. |
| T2  | TODO   | Benchmark group over the swarm sizes       | The `http_tracker/first_announce/<N>` group, using `iter_custom` as decided in D1; `cargo test --benches` green; `cargo bench` output recorded.                                                                                                        |
| T3  | TODO   | Maintainer review of the benchmark         | Stop after T2: the maintainer reviews the benchmark design, the guard evidence, and the first results before documentation and the pull request. Outcome logged in the Progress Log.                                                                   |
| T4  | TODO   | Documentation and completion review        | `docs/benchmarking.md` updated; manual verification recorded; acceptance criteria reviewed; completion review recorded.                                                                                                                                 |

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

## Acceptance Criteria

- [ ] AC1: `cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark` reports
      one result per swarm size, each timing one `started` announce of a peer that was not in a
      swarm of exactly `N` other peers.
- [ ] AC2: The swarm has exactly `N` peers after the run; the benchmark asserts this and fails
      under `cargo test --benches` when it does not (shown by a deliberate mutation).
- [ ] AC3: The new benchmark uses the `RoutineGuard`, and the guard is shown to fail when the
      routine is not awaited.
- [ ] AC4: Results grow with the peer-list size in the expected direction (the 74 and 1000 sizes
      above the empty swarm), and every result is above the repeat-announce result, or the
      evidence explains why not.
- [ ] AC5: `docs/benchmarking.md` lists and explains the benchmark, including what its timing
      includes (the per-iteration clock reads, D1) and why the discarded options were not used.
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

| ID  | Scenario                                   | Human-oriented command/steps                                                                                                                         | Expected Result                                                                                                    | Status | Evidence                                     |
| --- | ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ | ------ | -------------------------------------------- |
| M1  | Each iteration is a first announce         | Run `cargo bench` with a temporary probe that counts inserts versus updates in the timed announces and prints the response peer count               | Every timed announce inserts a new peer; the response holds `min(N, 74)` peers; the swarm holds `N` after the run | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Results are stable and ordered as expected | Run `cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark` three times, with `uptime` before each run                         | Run-to-run variation is recorded; the results follow AC4                                                           | TODO   | `manual-verification-evidence.md` section V2 |

### Disposable Verification Scripts

- **Insert-versus-update probe (M1).** A temporary edit of the benchmark helpers, in Rust, that
  counts inserts and updates and prints them. It is recorded verbatim in
  `manual-verification-evidence.md` and reverted after the run. It is not a maintained test
  because the production code has no public seam reporting insert versus update; the maintained
  swarm-size assertion covers the durable invariant.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                   |
| ----- | ---------------------- | ------------------------------------------ |
| AC1   | TODO                   | `cargo bench` output, M1                   |
| AC2   | TODO                   | Swarm-size assertion mutation run          |
| AC3   | TODO                   | `RoutineGuard` mutation run                |
| AC4   | TODO                   | M2                                         |
| AC5   | TODO                   | `docs/benchmarking.md` diff                |

## Risks and Trade-offs

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
