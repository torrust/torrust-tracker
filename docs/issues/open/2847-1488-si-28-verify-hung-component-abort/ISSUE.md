---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 1488
github-issue: 2847
spec-path: docs/issues/open/2847-1488-si-28-verify-hung-component-abort/ISSUE.md
branch: "2847-verify-hung-component-abort"
related-pr: null
last-updated-utc: "2026-11-17 18:16"
semantic-links:
  skill-links:
    - create-issue
    - add-workspace-member
  related-artifacts:
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - src/main.rs
    - packages/tracker-core/src/statistics/event/listener.rs
    - packages/events/src/drain.rs
    - packages/configuration/src/v3_0_0/shutdown.rs
    - tests/lifecycle/shutdown_policy.rs
    - tests/common/native_tracker/mod.rs
    - docs/issues/closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/manual-verification-evidence.md
    - docs/issues/closed/2845-1488-si-26-consolidate-shutdown-supervisor/manual-verification-evidence.md
    - docs/issues/closed/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2847 - Verify the Hung-Component Abort at the Deadline on the Real Binary (SI-28)

> **EPIC position**: Roadmap row 25, sub-issue SI-28 of [EPIC #1488](../1488-overhaul-tracker-shutdown/ISSUE.md).
> A follow-up of [SI-21](../../closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md)
> (and SI-20). Implementation order 4 of 4, after SI-26. Priority p3. No production behavior changes:
> the only production edit is a behavior-preserving refactor of `main.rs` (decision D2).

## Goal

An **automated executable-boundary test** shows that a component which never finishes is aborted at the
process deadline `P`, that the process exits with code 1 within `P`, and that the log names the aborted
component, for `SIGTERM` and `SIGINT`. The process runs the real tracker startup and the real shutdown
code. Today only unit tests cover this path. The same test binary also gives the manual evidence the
EPIC protocol asks for.

## Background

Checked on 2026-11-12 against `torrust/develop` at `388faf59a`, and again on 2026-11-17 at `5833cdae9`.

- SI-20 gave the process one deadline `P` (`[application.shutdown].deadline_in_secs`, default 25 s);
  `JobManager::wait_for_all` aborts every component still running at `P` and `report_shutdown` maps
  that to exit code 1.
- The SI-21 evidence (section V6) records a failed component (a REST drain that timed out) and says
  the case "a hung component that never completes would be aborted at P together with the health
  server (unit test above); the validated budgets make that hard to provoke on the binary, and it was
  not run." SI-26 (scenario M3) checked the failed-component path again on the final supervisor; the
  hung path is still unchecked on a binary.
- The EPIC protocol requires issue-local executable-boundary verification for runtime-affecting
  shutdown work. This path (`Aborted` outcome, exit code 1, no delay past `P`) has none.

**Why it is hard.** Every component drains within a validated budget below `P` and the Axum drain
force-closes at its deadline, so an ordinary overrun ends as a *failed* component, not a hung one. A
hung component needs a task that stays parked past `P`. No production component ignores its token on
purpose, but one can hang on a stalled dependency: `tracker_core_persistent_completed_statistics_event_listener`
awaits each event's database write inside the `recv()` arm of its `select!`, outside the token branch
(`packages/tracker-core/src/statistics/event/listener.rs`), and its drain always finishes an event it
has taken (`packages/events/src/drain.rs`). A write stalled by a paused MySQL server keeps it parked
until `P` (case 1 below). Provoking that on demand needs a database container and a completed-download
event timed into the shutdown window, which is why this issue uses a synthetic component instead.

**What makes it feasible.** `app::start_with_explicit_config_toml_path` is public and returns the real
`JobManager` with every real component running. A separate binary can start the full real tracker,
reserve and spawn one extra component that never completes, and then run the same shutdown sequence
as `main.rs`.

### Why a component can hang, and what this test covers

Stopping is cooperative: on shutdown each component's token is cancelled, and the component must notice
it and return. A component hangs when it never notices:

1. it awaits something (a stalled database query, a write to a dead peer, a channel whose sender never
   drops, a child task that never ends) without also selecting on its token; the persistent statistics
   listener above is a real instance;
2. it waits behind something else that is stuck (a lock, a semaphore, another hung task, a deadlock);
3. its stop logic has a bug (checks the token only at the top of a loop, or does not forward
   cancellation to its children);
4. it blocks a thread instead of yielding: either the task itself blocks (`std::thread::sleep`,
   blocking I/O, a long-held `std::sync::Mutex`, a CPU loop without `.await`), or it waits on work it
   started with `spawn_blocking` that never returns.

The deadline abort (`JoinSet::abort_all`) only takes effect at an await point. It covers cases 1 to 3,
where the task is parked on a future. Case 4 escapes it in two ways. A task that blocks itself never
reaches an await point, so the supervisor waits to join it and `wait_for_all` does not return at `P`.
A task that awaits a `spawn_blocking` handle is at an await point, so it is aborted and joined, but the
blocking thread keeps running, and dropping the runtime when `main` returns waits for it, so the process
still does not exit at `P`. The test component awaits `std::future::pending()`, so it simulates cases 1
to 3. The evidence must state that it says nothing about case 4, which only the "no blocking work in
async components" rule prevents.

## Decisions

Maintainer decisions of 2026-11-17, in chat.

- **D1 - Test-only binary in a dev crate (alternative A).** A new workspace crate (proposed path
  `packages/hung-component-tracker`, `publish = false`, a member but not a default member, like
  `packages/e2e-tools`) builds one binary. The binary starts the real tracker through
  `app::start_with_explicit_config_toml_path`, reserves and spawns one component named
  `hung_component` whose future never completes and ignores its cancellation token, then runs the
  shared shutdown sequence of D2. The crate's own integration test gets the binary through
  `CARGO_BIN_EXE_<name>`, starts it, waits for the readiness marker, signals the child PID, and asserts
  the result. Nothing in the shipped `torrust-tracker` crate or binary contains test code: no Cargo
  feature, no environment hook, no fault-injection point.
- **D2 - Share the shutdown sequence between `main.rs` and the test binary (alternative B).**
  `wait_for_shutdown_signal` and `report_shutdown` move from `main.rs` into the library, behind one
  public function (proposed: `app::run_until_shutdown_signal(app_container, jobs) -> ExitCode`) that
  waits for the signal, calls `begin_shutdown`, waits with `wait_for_all(P)` and maps the outcomes to the
  exit code. `main.rs` and the test binary both call it, so the test binary differs from the real one
  only by the extra component and cannot drift. This is a refactor with no behavior change: the same
  signals, the same log lines (including the readiness marker `Tracker shutdown signal handlers
  installed.` that the native-tracker tests wait for), and the same exit codes. The default log format
  prints each event's target, which a plain move would change from the binary crate `torrust_tracker`
  to `torrust_tracker_lib::app`; the moved events therefore pin `target: "torrust_tracker"`, so the
  lines stay identical, target included. Exact names are settled in review of the implementation PR.
- **D3 - Retention.** The crate and its test stay in the tree as a permanent regression test. CI runs
  it: `testing.yaml` runs `cargo test --workspace --all-targets`, which includes non-default members.
  It is not a default member, so it does not enter the container test archives (the
  `add-workspace-member` skill rule for tracker verification tools). It lives under `packages/`, which
  `.dockerignore` already admits, so it needs no admission line; the interim AI-harness block there is
  for crates with no tracker dependency, and this one depends on the root crate.
- **D4 - Spec first.** The goal changed from manual evidence to an automated test and the scope now
  touches `main.rs`, so this spec change merges in its own PR before the implementation.

### Alternatives Considered

| ID | Alternative | Production code | Real process and signals | Verdict |
| -- | ----------- | --------------- | ------------------------ | ------- |
| A | Test-only binary in a dev crate, with its own integration test | None | Yes | **Chosen** (D1) |
| B | Move the signal wait and shutdown reporting from `main.rs` into the library | Refactor, no behavior change | n/a (combined with A) | **Chosen** (D2) |
| C | In-process integration test in root `tests/`: `app::start`, extra hung component, `begin_shutdown`, `wait_for_all`, assert `shutdown_exit_code` | None | No | Rejected: no real process, signals or exit code, which is what the executable boundary means. |
| D | External stall of a real dependency (for example pausing a MySQL container during a query) | None | Yes | Rejected on cost, not feasibility: the persistent statistics listener does hang on a stalled MySQL write (see Background), but the test needs a database container and a completed-download event timed into the shutdown window, which makes it heavy and timing-dependent. |
| E | Cargo example (`examples/`) run by hand, with the signal code copied from `main.rs` | None | Yes, manual only | Rejected: manual only, and the copied code can drift from `main.rs`. |
| F | Fault-injection point in the real binary behind a Cargo feature or an environment variable | Test code in production | Yes | Rejected: the maintainer does not want test code in production. |
| G | Integration test that runs `cargo run --example` | None | Yes | Rejected: nested Cargo builds are slow and fragile. |
| H | No-go: the unit tests are the strongest feasible evidence | None | No | Rejected: A shows a stronger technique is feasible. |

## Scope

### In Scope

- T1: refactor (D2): move the shutdown sequence from `main.rs` into a public library function; `main.rs`
  calls it. Existing unit and native-tracker tests pass unchanged.
- T2: the dev crate (D1): workspace membership per the `add-workspace-member` skill, the binary, and a
  small child-process helper of its own (spawn, wait for the readiness marker, signal, bounded wait,
  kill and reap on every path). It may reuse ideas from `tests/common/native_tracker`, which is wired to
  `CARGO_BIN_EXE_torrust-tracker` and is not importable from another crate.
- T3: the integration test, one case per signal (`SIGTERM`, `SIGINT`), with a small `deadline_in_secs`
  set through the configuration together with drain budgets that pass validation (the slowest drain
  plus the listener drain must stay below the deadline; the defaults, 15 s and 5 s, do not fit). For
  example `deadline_in_secs = 4` with `http_drain_in_secs`, `udp_request_drain_in_secs` and
  `event_listener_drain_in_secs` all `1`, as `tests/lifecycle/shutdown_policy.rs` lowers them. It
  asserts:
  - exit code 1;
  - the process exits after `P` and within `P` plus a small margin;
  - the log reports `hung_component` as aborted after the deadline;
  - the log reports `health_check_api` as aborted too (it stops last, so it waits for the hung
    component and is never asked to stop), and its abort does not push the exit past `P`;
  - the final error line lists both in `incomplete_components`.
- T4: manual evidence: run the same binary once per signal and record the output in
  `manual-verification-evidence.md`, with the case-4 limit stated.
- T5: update the shutdown docs if they still call the hung-component path unverified.

### Out of Scope

- Any change to production shutdown behavior or defaults. The D2 refactor must not change signals, log
  lines or exit codes.
- Test code, Cargo features or environment hooks in the shipped crate or binary.
- Changing the budget validation so that a hung component becomes easy to provoke.
- Re-verifying the failed-component path (done in SI-21 V6 and SI-26 M3).
- Proving that production components never hang, or covering case 4 (blocking work).

## Dependencies

- SI-20 (#2668), SI-21 (#2697) and SI-26 (#2845) are done and merged.
- Not required before the first publish (#2583) by itself, but #2583 is blocked on EPIC #1488 as a
  whole, so the EPIC close-out needs this issue done.

## Architectural Decisions

- Related ADRs: `docs/adrs/20261027145446_budget_shutdown_in_two_chained_stages_inside_one_process_deadline.md`
  (the deadline `P`), `docs/adrs/20261104120915_stop_the_health_check_api_last_to_report_not_ready_during_shutdown.md`.
- ADRs to create: none. A test-only binary in a dev crate follows the existing practice for verification
  tools; if review finds it sets a new pattern, an ADR is added in the implementation PR.

## Design and Ownership Review

- The D2 function owns the whole post-startup lifetime of the process: signal wait, shutdown request,
  deadline wait and exit-code mapping. `main.rs` keeps only CLI parsing, startup and the startup-failure
  report.
- The test binary owns one extra component and nothing else; it must not reimplement any step of the
  shutdown sequence.
- The integration test owns its child process: an absolute deadline on every wait, kill and reap of the
  child on every path (including a failed assertion), ephemeral ports, a temporary storage directory, and
  no port or file left behind.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Move the shutdown sequence into a library function; `main.rs` calls it (D2). | No behavior change; existing tests green. |
| T2 | DONE | Dev crate with the hung-component binary (D1, D3). | Under `packages/`; workspace member, not default member. |
| T3 | DONE | Integration test for `SIGTERM` and `SIGINT`. | Assertions listed in Scope. |
| T4 | DONE | Manual run of the binary; record the evidence. | `manual-verification-evidence.md`. |
| T5 | DONE | Update the shutdown docs if they say the path is unverified. | Doc edit or none. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | `main.rs` and the new library function | Commit after the existing tests pass and the required review. |
| T2, T3 | Dev crate, binary and integration test | Commit after the test passes for both signals, the test-design review below is recorded, and the maintainer has reviewed it. |
| T4 | Evidence | Commit after the run. |
| T5 | Documentation | Commit last. |

Test-design review (T2, T3): the test is written with the `write-unit-test` skill. After each passing
test increment, and before maintainer review and commit, run the prose-first Arrange-Act-Assert design
review of `docs/templates/ISSUE.md` (Commit Points): the one causal initial-state difference is the
extra `hung_component`; the child-process fixture owns only incidental mechanics (spawn, readiness
wait, signal, bounded wait, reap); the Act (signal the PID) and the independently specified expected
result (exit code 1 within `P` plus the margin, the named aborted components) stay visible in the test.
Record the review in `test-design-review.md` in this issue folder. Commit each reviewed increment
before starting the next one.

Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/`, moved to `docs/issues/open/`
- [x] Spec reviewed and approved by the maintainer (decisions of 2026-11-12, in chat)
- [x] GitHub issue created and issue number added to this spec (#2847)
- [x] Technique chosen by the maintainer (D1 to D4, 2026-11-17)
- [x] Spec-only PR with the decisions merged into `develop` before implementation (#2909)
- [x] Integration test passing for `SIGTERM` and `SIGINT`
- [x] Test-design review recorded in `test-design-review.md` and reviewed by the maintainer
- [x] Manual verification executed and recorded in `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-12 14:30 UTC - AI agent (Claude Sonnet 5.5) - Spec created from the SI-21 evidence gap on `torrust/develop` at `388faf59a`; recorded the maintainer's decision of 2026-11-12 to open it as SI-28, after SI-26.
- 2026-11-17 11:03 UTC - AI agent (Claude Opus 5.5) - Recorded the maintainer's decisions D1 to D4 of 2026-11-17 (test-only binary in a dev crate, shared shutdown sequence, kept as a regression test, spec first) and the alternatives considered; the goal is now an automated executable-boundary test. Checked against `torrust/develop` at `5833cdae9`.
- 2026-11-17 13:55 UTC - AI agent (Claude Opus 5.5) - Addressed review round 1 of PR #2909 (da2ce7, findings F1 to F6): alternative D is rejected on cost, since the persistent statistics listener can hang on a stalled MySQL write; the moved log events pin their target; T3 and M1 name drain budgets that pass validation; case 4 names both mechanisms; the crate path moves under `packages/`.
- 2026-11-17 14:01 UTC - AI agent (Claude Opus 5.5) - Addressed Copilot finding R1 of PR #2909: the T1 and T2/T3 commit points require the review, and the T2/T3 test work requires the prose-first Arrange-Act-Assert design review of the issue template, recorded in `test-design-review.md`, before maintainer review and commit.
- 2026-11-17 16:47 UTC - AI agent (Claude Opus 5.5) - Implemented T1 to T5 on `torrust/develop` at `a544a58fb`: `app::run_until_shutdown_signal` holds the shutdown sequence and `main.rs` calls it; the test-only crate `packages/hung-component-tracker` and its integration test pass for `SIGTERM` and `SIGINT` (exit code 1 about 4.01 s after the signal, `hung_component` and `health_check_api` aborted); the maintainer reviewed T1 to T3 and the test-design review before the commits. M1 to M4 are recorded in `manual-verification-evidence.md` (the real binary's normalized log is identical before and after the refactor). T5 updated `docs/architecture/application-jobs.md` and `docs/packages.md`; no shutdown doc called the path unverified. Completion review: no implementation retrospective is needed; the two surprises were local to the manual runs (a shared target directory left a stale library build, and one M4 run whose script stamps disagreed with the tracker's log was discarded and rerun) and are recorded in the evidence. [Corrected in review round 1 of PR #2911: this entry first said the stamps lagged on a loaded machine, a cause that cannot produce the reported order.]
- 2026-11-17 17:27 UTC - AI agent (Claude Opus 5.5) - Addressed review round 1 of PR #2911 (da2ce7, findings F1 to F6, none blocking): the release skill's version bumps name every manifest that requires the root crate; the evidence records the stale build in the shared target directory and how each binary copy of V1 to V4 was confirmed (`nm` finds `run_until_shutdown_signal` in the branch binaries only); the discarded M4 run's cause is recorded as unknown, and the 16:47 entry above is corrected in place; `src/AGENTS.md`, `packages/AGENTS.md` and the shutdown task inventory place the shutdown sequence in `app.rs` and list the crate; `run_until_shutdown_signal` documents its panics; the test binary's CLI comment states the missing empty-path check.
- 2026-11-17 18:02 UTC - AI agent (Claude Opus 5.5) - Addressed Copilot's round 2 of PR #2911 (findings R1 and R2, test helper only): the readiness poll checks the captured output under its lock instead of copying it, and a panic in an output reader thread now fails the test on the normal paths.
- 2026-11-17 18:16 UTC - AI agent (Claude Opus 5.5) - Addressed round 3 of PR #2911 (da2ce7 findings F7 and F8, test helper only): an output read error now panics the reader thread so the test reports incomplete output, and `signal_and_wait_for_exit` documents that panic.

## Acceptance Criteria

- [x] `main.rs` and the test binary run the same library shutdown function, and the refactor changes no
      signal handling, log line (target included) or exit code.
- [x] An automated test spawns a real process, sends `SIGTERM` and `SIGINT` in separate cases, and
      asserts: exit code 1, exit within `P` plus a small margin, `hung_component` and
      `health_check_api` reported as aborted, and both named in `incomplete_components`.
- [ ] The test runs in CI (`cargo test --workspace --all-targets`).
- [x] The shipped `torrust-tracker` crate and binary contain no test code, feature or hook for this.
- [x] `manual-verification-evidence.md` records one run per signal and states the case-4 limit.
- [x] `linter all` passes.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo check --workspace --all-targets` warning-free
- `cargo test --workspace --all-targets`, including the new integration test
- Existing native-tracker shutdown tests in root `tests/` pass unchanged (they cover the D2 refactor)
- `cargo machete`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Hung component, `SIGTERM` | Start the test binary with `deadline_in_secs = 4` and all three drain budgets `1` (as in T3), signal the actual PID. | Exit code 1 within `P` plus a small margin; the aborted component is named in the log. | DONE | `manual-verification-evidence.md` |
| M2 | Hung component, `SIGINT` | Same with `SIGINT`. | Same result. | DONE | `manual-verification-evidence.md` |
| M3 | Health server does not delay the exit | Read the M1 and M2 logs. | `health_check_api` is aborted at `P` with the hung component, and the process still exits within `P` plus the margin. | DONE | `manual-verification-evidence.md` |
| M4 | Real binary unchanged | Start the real `torrust-tracker`, signal it with `SIGTERM`. | Graceful shutdown, exit code 0, same log lines and targets as before the refactor. | DONE | `manual-verification-evidence.md` |

### Disposable Verification Scripts

None. The test binary and its integration test are permanent (D3). A script used for the manual runs is
inlined in the evidence file.

### Acceptance Verification

| AC | Status | Evidence |
| -- | ------ | -------- |
| Shared shutdown function, no behavior change | DONE | T1, M4, existing tests |
| Automated test for both signals | DONE | T3 |
| Test runs in CI | TODO | PR checks |
| No test code in the shipped crate | DONE | Diff review |
| Manual evidence with the case-4 limit | DONE | M1, M2, M3 |
| `linter all` passes | DONE | Automatic checks |

## Risks and Trade-offs

- The test runs the real startup and the real shutdown function but not a real hung component, so it
  proves the deadline path, not that production components never hang. That is the claim to make.
- It covers hangs at an await point only (cases 1 to 3); a component that blocks its thread (case 4)
  would not be aborted, and this test cannot show that.
- Timing assertions on a busy CI runner: the margin over `P` must be generous enough not to flake, and
  the test must not wait longer than an absolute bound.
- The D2 refactor touches the process entry point. The existing native-tracker tests and M4 guard it.
- A new workspace crate adds build time to `cargo test --workspace`; it has no dependency beyond the
  root crate and what the test needs.

## Implementation Completion Review

Before closing, review what the implementation discovered. If it found anything material, create
`implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this
directory; otherwise add a progress-log entry saying why none was needed.

## References

- Parent EPIC: #1488 (roadmap row 25)
- Related: #2697 (SI-21, evidence V6), #2845 (SI-26, evidence M3), #2668 (SI-20, the deadline), #2583
  (first publish)

## Rollback

Revert the implementation PR: the dev crate and its test are removed, and `main.rs` returns to holding
the shutdown sequence itself. No production behavior changes either way.
