---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p3
epic: null
github-issue: 2479
spec-path: docs/issues/open/2479-fix-http-core-announce-benchmark/ISSUE.md
branch: "2479-fix-http-core-announce-benchmark-spec"
related-pr: null
last-updated-utc: "2026-10-08 06:46"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - run-benchmarks
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/benchmarking/run-benchmarks/SKILL.md
    - docs/benchmarking.md
    - packages/http-core/benches/http_tracker_core_benchmark.rs
    - packages/http-core/benches/helpers/sync.rs
    - packages/http-core/benches/helpers/util.rs
    - packages/udp-core/benches/udp_tracker_core_benchmark.rs
---

<!-- skill-link: create-issue -->

# Issue #2479 - Make the HTTP Announce Benchmark Measure the Announce

## Goal

Make `http_tracker_core_benchmark` measure one HTTP tracker announce, as its name says, instead of
the creation of a future that is never run, and add a guard that fails the test run when a
benchmark routine does not execute.

## Background

### What the benchmark is meant to measure

`packages/http-core/benches/http_tracker_core_benchmark.rs` defines the Criterion benchmark
`http_tracker_handle_announce_once/handle_announce_data`. Its routine calls the helper
`sync::return_announce_data_once(100)` (`packages/http-core/benches/helpers/sync.rs`), an
`async fn` that builds the tracker core services, builds an `AnnounceService`, and then awaits
`handle_announce` 100 times.

### The bug, in plain terms

1. The benchmark passes the helper to Criterion's synchronous `b.iter`:
   `b.iter(|| sync::return_announce_data_once(100))`.
2. Calling an `async fn` only creates a future. `b.iter` times the closure, receives the future as
   its return value, and drops it. Nothing polls it, so the function body never runs.
3. The Tokio runtime the benchmark builds (`let _rt = ...`) is never used.
4. So the reported time, about 85-87 ns, is the cost of creating, moving, and dropping a
   1160-byte future. It contains no announce, no service setup, and none of the 100 iterations.

**Contract violated:** a benchmark named `handle_announce_once` must time an announce. Anyone who
uses it to judge an HTTP announce change, for example to compare before and after, gets a number
that cannot change with the code under test. Nothing reports an error: Criterion prints a precise
result, and `cargo test --benches` prints `Success`.

**Impact today:** no production behavior is wrong. The benchmark is listed under Known Defects in
`docs/benchmarking.md`, so its results should not be trusted until it is fixed. The same defect in
the `udp-core` `connect_once` benchmark was found and repaired in issue #2458.

### Reproduction (done before review; outcome: Reproduced)

A temporary counter, incremented on the first line of the helper's body, read 0 after Criterion
ran about 12 million iterations of the routine. The probe and its output are in
`manual-verification-evidence.md` section V1.

## Scope

### In Scope

- Restructure the HTTP announce benchmark like the repaired `udp-core` `connect_once`: build the
  tracker services and the `AnnounceService` once, outside the measured routine, and time one
  awaited `handle_announce` per iteration with `b.to_async(&runtime).iter(...)`.
- Rename or document the measured operation so the benchmark's name and what it times agree.
- Add a regression guard: the benchmark counts the routines it actually ran and panics after
  `bench_function` if the count is zero. Criterion runs each benchmark once under `cargo test`,
  and both the pre-push hook and CI run `cargo test --benches`, so a routine that never runs fails
  those gates.
- Add the same guard to the `udp-core` `connect_once` benchmark, repaired in issue #2458 without
  one.
- Update `docs/benchmarking.md`: remove the Known Defects entry, update the table row, and describe
  the guard in "Checking That a Benchmark Measures Something".

### Out of Scope

- Changing the HTTP announce code, or its performance.
- Benchmarking other HTTP operations (scrape, the Axum server).
- Adding guards to benchmarks that already await their routine (`udp-server` scrape) or are
  synchronous (`connection_cookie_benchmark`, `ban_service_benchmark`, the repository benchmarks).
- A lint or CI check that finds futures that are never awaited in any benchmark automatically.

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none known. The fix follows an existing pattern (the `udp-server` scrape and
  `udp-core` connect benchmarks).

## Design and Ownership Review

Not applicable: no child processes, network readiness, or reusable test fixtures. The benchmark
context owns the services for the benchmark's lifetime.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. **Analysis:** done in this specification (Background).
2. **Reproduction (done before review; outcome: Reproduced):** the temporary counter shows the
   helper body ran 0 times; see `manual-verification-evidence.md` section V1.
3. **Regression-test boundary:** the guard inside the benchmark (see Regression Test Strategy).
4. **Red evidence:** add the guard to the current benchmark first and record
   `cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark` failing, before
   changing how the routine runs (T1).
5. **Fix:** restructure the benchmark (T2).
6. **Green and recheck:** the guard passes under `cargo test`, and `cargo bench` reports a time
   that includes an announce, rechecked like-for-like with the V1 probe (T4).

## Regression Test Strategy

- **R1 (maintained guard in the benchmark):** the benchmark context counts completed announces;
  after `bench_function`, the benchmark panics with a message naming the benchmark if the count is
  zero. It runs under `cargo test --benches` (pre-push and CI) and under `cargo bench`.
  - Red before the fix: with the guard added to today's routine, the count stays 0 and the test
    run panics. This is the T1 red run.
  - Why not a unit test: the defect is in how the benchmark calls Criterion, not in production
    code; only the benchmark target can observe it.
  - The guard detects a routine that never runs. It cannot tell whether the measured time is
    representative; the before-and-after `cargo bench` output and the V1 probe cover that.
- **R2 (same guard on `udp-core` `connect_once`):** red by mutate-then-restore, replacing
  `b.to_async(&runtime).iter(...)` with the old `b.iter(...)` call.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                    | Notes / Expected Output                                                                                                   |
| --- | ------ | --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Regression guard R1, red before the fix | Guard added to the current benchmark; red `cargo test` output recorded in `manual-verification-evidence.md`.              |
| T2  | TODO   | Fix: measure one awaited announce       | Context built once; `b.to_async(&runtime).iter(...)`; R1 green; before and after `cargo bench` output recorded.           |
| T3  | TODO   | Guard R2 on `udp-core` `connect_once`   | Guard added; mutate-then-restore red run recorded.                                                                        |
| T4  | TODO   | Recheck and documentation               | V1 probe repeated like-for-like on the fixed benchmark; `docs/benchmarking.md` updated; acceptance and completion review. |

## Commit Points

| Task | Coherent change set                     | Commit policy                                                                                      |
| ---- | --------------------------------------- | -------------------------------------------------------------------------------------------------- |
| T1   | Red-run evidence                        | One `docs(issues)` commit. The red guard is not committed alone, because pre-push would reject it. |
| T2   | Benchmark fix with the guard (R1)       | One `fix(http-core)` commit after focused validation.                                              |
| T3   | Guard on `connect_once` (R2)            | One `test(udp-core)` commit.                                                                       |
| T4   | Documentation and verification evidence | Separate `docs(benchmarking)` and `docs(issues)` commits.                                          |

All commits are GPG signed and follow Conventional Commits.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/fix-http-core-announce-benchmark/ISSUE.md`
- [x] Reproduction attempted and classified before review
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

- 2026-10-07 13:30 UTC - Copilot - Drafted at the maintainer's request after issue #2458 found the
  same defect in the `udp-core` `connect_once` benchmark. Reproduced with a temporary counter
  (0 body runs) and checked that Criterion runs each benchmark once under `cargo test`, which makes
  an in-benchmark guard practical. Evidence: `manual-verification-evidence.md` V1. This spec
  depends on the `run-benchmarks` skill and the `docs/benchmarking.md` sections added by the
  issue #2458 PR, so it is not opened until that PR merges.
- 2026-10-08 05:43 UTC - Copilot - PR #2470 merged on 2026-10-08 as `develop` `44a280c0a`, so the
  `run-benchmarks` skill and the `docs/benchmarking.md` sections this spec cites now exist on
  `develop`. Moved the draft from `.tmp/` to `docs/issues/drafts/` for review.
  `git log c7cb2b3fa..44a280c0a -- packages/http-core/benches` is empty, so the V1 reproduction
  still describes the current code.
- 2026-10-08 06:46 UTC - Copilot - The maintainer approved the draft, including the guard on the
  `udp-core` `connect_once` benchmark, and chose a spec-only PR first. Created GitHub issue #2479
  and moved the specification and its evidence to `docs/issues/open/2479-fix-http-core-announce-benchmark/`
  on branch `2479-fix-http-core-announce-benchmark-spec`; the base name
  `2479-fix-http-core-announce-benchmark` is reserved for the implementation.

## Acceptance Criteria

- [ ] AC1: `cargo bench -p torrust-tracker-http-core` times one awaited HTTP announce per
      iteration, and its result is consistent with that work (for example microseconds or high
      nanoseconds, not the cost of creating a future).
- [ ] AC2: The HTTP announce benchmark fails under `cargo test --benches` when its routine does not
      run (R1, red before the fix).
- [ ] AC3: The `udp-core` `connect_once` benchmark has the same guard (R2).
- [ ] AC4: `docs/benchmarking.md` no longer lists the benchmark as a known defect and describes the
      guard.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-http-core -p torrust-tracker-udp-core --benches`
- `linter all`
- Pre-commit and pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                        | Human-oriented command/steps                                                                                      | Expected Result                                               | Status | Evidence                                     |
| --- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Benchmark measures nothing      | Run `cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark` with a temporary counter probe | The helper body runs 0 times                                  | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Benchmark measures the announce | Repeat M1 like-for-like on the fixed benchmark                                                                    | The routine runs every iteration; the time includes announces | TODO   | `manual-verification-evidence.md` section V2 |

### Disposable Verification Scripts

- **Counter probe (M1, M2).** A temporary edit of `packages/http-core/benches/helpers/sync.rs`
  and `http_tracker_core_benchmark.rs` that counts how often the routine body runs and prints it.
  It is recorded verbatim in `manual-verification-evidence.md` and reverted after each run. The
  maintained guard R1 replaces it as the durable check; the probe exists to show the pre-fix
  state, which the maintained code cannot keep. Rust, so no Python rationale is needed.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                  |
| ----- | ---------------------- | ----------------------------------------- |
| AC1   | TODO                   | Before and after `cargo bench` output, M2 |
| AC2   | TODO                   | R1 red and green runs                     |
| AC3   | TODO                   | R2 mutate-then-restore run                |
| AC4   | TODO                   | `docs/benchmarking.md` diff               |

## Risks and Trade-offs

- **The guard adds code to benchmarks.** Mitigation: a counter and one assertion per benchmark,
  with a message naming the benchmark.
- **Repeated announces of the same peer measure an update, not a first announce.** This matches
  the `udp-server` scrape benchmark. Record it in the benchmark's documentation; vary the peer
  only if the maintainer wants first-announce costs.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory;
  otherwise add a progress-log entry explaining why the work had no material discovery.

## References

- Related issues: #2458 (found and repaired the same defect in `udp-core`)
- Related documentation: `docs/benchmarking.md` (Known Defects; Checking That a Benchmark Measures
  Something); the `run-benchmarks` skill
