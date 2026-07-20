---
doc-type: issue
issue-type: enhancement
status: open
priority: p3
github-issue: 1586
spec-path: docs/issues/open/1586-use-joinset-in-jobmanager.md
branch: "1586-document-joinset-refactor"
related-pr: null
last-updated-utc: 2026-07-20 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/bootstrap/jobs/manager.rs
    - src/bootstrap/jobs/
    - src/app.rs
    - src/main.rs
    - src/AGENTS.md
---


# Issue #1586 - Consider using `tokio::task::JoinSet` in `JobManager`

> **EPIC position**: Proposed future subissue of
> [EPIC #1488 - Overhaul: Tracker Shutdown](https://github.com/torrust/torrust-tracker/issues/1488).
> The EPIC and its subissues are still under review in
> [draft PR #1993](https://github.com/torrust/torrust-tracker/pull/1993). Review and re-scope
> this specification in that context before implementation, then add #1586 to the EPIC.

## Goal

Evaluate and, if it remains appropriate after the shutdown overhaul is designed, replace the
manual `Vec<Job>` task collection in `JobManager` with `tokio::task::JoinSet<()>` so the
application has explicit ownership of its background tasks and can coordinate their completion
and cancellation without nested task wrappers.

## Background

`JobManager` in `src/bootstrap/jobs/manager.rs` currently stores a `Vec<Job>`. Each `Job`
contains a human-readable name and an already-spawned `JoinHandle<()>`:

```rust
pub struct Job {
    name: String,
    handle: JoinHandle<()>,
}

pub struct JobManager {
    jobs: Vec<Job>,
    cancellation_token: CancellationToken,
}
```

Its `wait_for_all` method awaits those handles sequentially with a timeout for each job. A job
that consumes its full timeout delays observation of every handle after it, and the total wait
can grow to the number of jobs multiplied by the grace period. Dropping a timed-out
`JoinHandle` detaches its task rather than aborting it.

[`tokio::task::JoinSet`](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html) provides
task ownership and completion-order joining for a dynamic set of tasks. It also aborts tracked
tasks when dropped and provides explicit cancellation operations such as `abort_all` and
`shutdown`.

However, replacing the vector while retaining the current `push(name, JoinHandle)` API would
require spawning a second task merely to await each existing handle. That nested-task design
adds an unnecessary ownership layer and defeats the purpose of adopting `JoinSet`.

The background-job launchers currently own calls to `tokio::spawn` and return handles, often
after completing asynchronous server-startup handshakes. A sound implementation must therefore
revisit the boundary between those launchers and `JobManager`, preserving startup guarantees
while giving the manager direct ownership of tracked task spawning.

## Scope

### In Scope

- Re-evaluate this proposal against the final architecture and shutdown contract from EPIC
  #1488 before implementation starts.
- Replace the manual task collection with `JoinSet<()>` if that remains compatible with the
  overhaul design.
- Redesign `JobManager`'s registration API and affected job launchers/call sites as needed so
  tracked futures are spawned directly into the `JoinSet`.
- Preserve asynchronous startup handshakes and startup failure behaviour when moving task
  ownership.
- Preserve human-readable job names in completion, panic, timeout, and cancellation logs.
- Define one explicit shutdown deadline policy in coordination with EPIC #1488.
- Add focused tests for completion order, panic reporting, deadline expiry, and cancellation of
  unfinished tasks.
- Update `src/AGENTS.md` after the implementation changes the documented architecture.

### Out of Scope

- Wrapping an existing `JoinHandle` in another spawned task solely to insert it into a
  `JoinSet`.
- Implementing this issue before EPIC #1488 and draft PR #1993 settle the shutdown architecture.
- Independently changing signal handling, shutdown propagation, or server-specific grace
  periods that belong to other EPIC #1488 subissues.
- Adding #1586 as an EPIC subissue before the EPIC review is ready for that relationship.

## Design Decisions Deferred to EPIC #1488

- Whether `JobManager` remains the shutdown coordinator or becomes a lower-level task registry.
- Whether launchers return futures that have not been spawned, register tasks through a
  manager-owned spawning API, or use another abstraction that preserves their startup
  handshakes.
- Whether the `CancellationToken` remains owned by `JobManager` or is supplied by a higher-level
  shutdown coordinator.
- Whether graceful waiting uses one global deadline, phased deadlines, or another policy.
- Whether unfinished tasks are aborted by `JoinSet::shutdown`, `abort_all`, or a separate
  escalation phase after cooperative cancellation.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status  | Task                                          | Notes / Expected Output                                    |
| --- | ------- | --------------------------------------------- | ---------------------------------------------------------- |
| T1  | BLOCKED | Review against the accepted EPIC #1488 design | Resolve the deferred design decisions and update this spec |
| T2  | TODO    | Define the task ownership and launcher API    | No nested task wrappers; preserve startup handshakes       |
| T3  | TODO    | Add focused `JobManager` shutdown tests       | Cover completion, panic, deadline expiry, and cancellation |
| T4  | TODO    | Implement direct `JoinSet` task ownership     | Update all affected launchers and call sites               |
| T5  | TODO    | Update architecture documentation             | Align `src/AGENTS.md` with the implemented design          |
| T6  | TODO    | Run automatic and manual verification         | Record evidence and re-review every acceptance criterion   |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] Existing GitHub issue number added to this spec
- [ ] Spec-only PR merged into `develop`
- [ ] Issue added as a subissue of EPIC #1488 after the EPIC review is ready
- [ ] Specification reviewed and re-scoped against the accepted EPIC #1488 design
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and applicable pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-20 00:00 UTC - Copilot - Drafted the initial local specification.
- 2026-07-20 00:00 UTC - Maintainer - Approved a spec-only change and deferred implementation
  until the shutdown overhaul is finalized.
- 2026-07-20 00:00 UTC - Copilot - Removed the nested-task proposal, documented direct task
  ownership as a design constraint, and moved the spec to the open backlog.
- 2026-07-20 00:00 UTC - Committer - Verified the spec progress and deferred implementation
  state are up to date for the spec-only commit.

## Acceptance Criteria

- [ ] AC1: The implementation is reviewed and re-scoped against the accepted EPIC #1488
      shutdown architecture before code changes begin.
- [ ] AC2: `JobManager`, or its replacement selected by the overhaul, owns tracked background
      tasks directly through `JoinSet` or an explicitly justified alternative.
- [ ] AC3: No task is spawned solely to await an already-spawned `JoinHandle` for registration.
- [ ] AC4: Affected launcher and registration APIs preserve existing asynchronous startup
      guarantees and failure behaviour.
- [ ] AC5: Completed and panicked tasks are observed in completion order and logged with their
      human-readable job names.
- [ ] AC6: The shutdown deadline and escalation policy are explicit and consistent with EPIC
      #1488.
- [ ] AC7: Tasks still running after cooperative shutdown are not silently detached.
- [ ] AC8: Focused automated tests cover completion, panic, deadline expiry, and cancellation.
- [ ] AC9: `linter all` and all relevant tests exit with code `0`.
- [ ] AC10: Manual verification scenarios are executed and documented with evidence.
- [ ] AC11: Acceptance criteria and architecture documentation are re-reviewed after
      implementation.

## Verification Plan

Define final commands and expected timing after the EPIC #1488 design resolves the deferred
shutdown policy.

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`
- Focused `JobManager` unit tests covering completion order, panic reporting, deadline expiry,
  and cancellation
- Relevant integration tests for the affected server launchers
- Pre-push checks when applicable

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                     | Command/Steps                                                                                         | Expected Result                                                                                         | Status  | Evidence                              |
| --- | ---------------------------- | ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- | ------- | ------------------------------------- |
| M1  | Graceful completion          | Start multiple test jobs, request shutdown, and observe logs                                          | Jobs finish within the selected grace policy and are logged in completion order                         | BLOCKED | Awaiting EPIC #1488 deadline policy   |
| M2  | Deadline escalation          | Include a job that ignores cooperative cancellation, request shutdown, and observe process/task state | The deadline expires once according to policy and the unfinished task is cancelled rather than detached | BLOCKED | Awaiting EPIC #1488 escalation policy |
| M3  | Panic isolation              | Include one panicking job alongside normally completing jobs                                          | The panic is attributed to the named job and does not prevent observation of other task results         | TODO    |                                       |
| M4  | Startup handshake regression | Start each affected server type after launcher API changes                                            | Startup readiness and startup failures retain their existing externally visible behaviour               | TODO    |                                       |
