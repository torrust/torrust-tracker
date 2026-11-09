---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p3
epic: 2003
github-issue: 2712
spec-path: docs/issues/closed/2712-2003-wait-for-pr-review-pipe-wait-after-exit/ISSUE.md
branch: "2712-wait-for-pr-review-bug-and-adr-specs-spec"
related-pr: 2750
last-updated-utc: "2026-11-09 11:16"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/pr-reviews/wait-for-pr-review/SKILL.md
    - .github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/gh.rs
    - docs/issues/closed/2581-2003-pr-review-wait-poller/ISSUE.md
    - docs/issues/closed/2712-2003-wait-for-pr-review-pipe-wait-after-exit/manual-verification-evidence.md
---

<!-- skill-link: create-issue -->

# Issue #2712 - `wait-for-pr-review` waits without limit for the output of a `gh` call that already exited

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make the `--timeout-seconds` budget and the `SIGINT` and `SIGTERM` handling hold on every path of a `gh` call,
including the one where the `gh` process exits successfully but something it started still has its output pipes
open, and protect that behavior with a test.

## Background

`wait-for-pr-review` (issue #2581) runs `gh` as a child process and reads the child's stdout and stderr on two
helper threads. Its specification promises two things: the timeout is a hard bound, so "no request starts once
the budget is spent and a `gh` call still running at that moment is killed", and `SIGINT` and `SIGTERM` stop the
poller promptly.

What happens, step by step, in `run()` in `.github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/gh.rs`
(on `develop` at `f7fecdf14`):

1. The poller starts `gh`, starts the two reader threads, and calls `wait_for`, which polls the child and, on a
   signal or at the deadline, kills it.
2. The `gh` process exits successfully, so `wait_for` returns `Ok` and never looks at the signal flag or the
   deadline again.
3. `run()` then calls `collect(reader, None)` for each pipe. With `None` there is no grace period, so `collect`
   calls `recv()` on the reader's channel and blocks until the reader reaches the end of the pipe.
4. A pipe reaches its end only when every process that holds its write end has closed it. If `gh` started a
   descendant that inherited the pipe and is still running, the reader never finishes, and the poller waits for
   that descendant with no limit, checking neither the deadline nor the signal flag.

The error path does not have this problem: it passes a 200 ms grace period (`RECLAIM_GRACE`) to `collect`. Only
the success path blocks without a bound.

Impact: the timeout becomes as long as the descendant lives, and a `SIGTERM` or `SIGINT` is honored only when it
exits. The poller is a tool for AI agents, which rely on the timeout to get control back. Nothing observable is
wrong while `gh` and its descendants close their pipes, which is the normal case, and this was not observed with
the real `gh`. The reproduction uses a fake `gh`; see `manual-verification-evidence.md`.

## Scope

### In Scope

- Bound the wait for the readers on the success path by the same deadline and interrupt flag that bound the
  child wait.
- Add maintained regression tests for the deadline and for a signal.
- Update the `wait-for-pr-review` skill and the issue #2581 specification text only where they describe the
  behavior this fix changes.

### Out of Scope

- Changing how the error path reclaims its readers, other than sharing code with the success path.
- Any other behavior of the poller.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md` (exit codes and diagnostics the
  fix must keep).
- ADRs to create: `None known`.

## Design and Ownership Review

This work involves child processes, helper threads, and resource cleanup, so the following is defined before
implementation.

- **Responsibilities.** `GhCli::run` owns the child and its two reader threads for one call. `wait_for` owns the
  decision to stop waiting (exit, signal, or deadline). A reader thread owns only its pipe and its buffer.
- **Lifetime invariants.** On every path, `run()` returns only after the child has exited or been killed and
  reaped. A reader thread never outlives the process that is its only reason to exist unless a descendant holds
  the pipe, in which case it ends on its own when that descendant closes it, and the poller does not wait for it.
- **Deadline coverage.** Every wait in `run()`, including the wait for the readers after a successful exit, is
  bounded by the call's absolute deadline and by the interrupt flag, checked in slices no longer than the
  existing 25 ms child poll interval.
- **Design-review checkpoint.** After the first passing vertical slice (the deadline test), review whether the
  success and error paths can share one bounded collection routine before adding the signal test.

### Decision: output not fully read when the wait ends

Question: when `gh` exits successfully but a descendant still holds its pipes open and the budget or a signal ends
the wait, what does `run()` do with output that has not been fully read?

**Decided (maintainer, 2026-11-06): option 1, treat it as a failure.** `run()` returns `deadline_exceeded` or
`interrupted` and no data. The wait for the readers is a loop in slices no longer than the 25 ms child poll
interval that checks the interrupt flag and the call's absolute deadline. The error path keeps its 200 ms grace
(`RECLAIM_GRACE`) and shares the same routine. A `gh` that closes its pipes normally is unaffected: its readers
finish and the full output is returned (AC3). The fix must not discard data that is still unread while the
child's pipes are being closed normally.

Alternatives considered:

| Option | Behavior | Pros | Cons |
| ------ | -------- | ---- | ---- |
| 1 (chosen) | Fail with `deadline_exceeded` or `interrupted`, return no data. | Simplest: no shared state, only the wait changes. Matches AC1 and AC2 as written. Never returns partial data, which is worse than none for a `gh api` response. One routine serves both paths. | A lingering descendant makes the call wait out the whole remaining budget and then fail. A call whose `gh` succeeded is reported as a failure. |
| 2 (rejected) | Readers append to a shared buffer. When `gh` exited 0 and only a descendant holds the pipe, wait a short grace and return the bytes read so far. | A successful call stays successful and does not burn the budget on a descendant. More robust if a real `gh` ever leaves a stray descendant. | Needs shared state (mutex), so a larger change. "Exit 0 means everything was written" does not guarantee the bytes were already read, so truncated data could reach the parser, and a truncated document that still parses would be silently wrong. Contradicts AC1 and AC2 as written. Adds a new fixed grace timeout, which the Risks section warns against. |

Why option 1: the poller is a tool for AI agents that rely on the timeout to get control back, so failing at the
deadline serves that goal, and the change is the smaller one. This was not observed with the real `gh`; revisit
option 2 only if it is.

Other implementation decisions (maintainer, 2026-11-06):

- The two failing tests and the fix are committed together, because the pre-commit gate runs the tests. The red
  runs are recorded in `manual-verification-evidence.md`.
- The success and error paths share one bounded collection routine.
- `evidence/fake-gh-leaves-pipe-open.sh` is kept for the V2 recheck and moves with the spec folder at close-out.

## Bug-Fix Process

Use [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md). The sequence:

- Analysis: the code path above, and the hypothesis that the unbounded `recv()` in the success path is the whole
  cause. The error path already shows the bound works.
- Real-artifact reproduction: done while drafting; see `manual-verification-evidence.md` section V1. Outcome:
  **Reproduced** with the real `wait-for-pr-review` binary and a fake `gh`.
- Regression test boundary and red run: T1 and T2 below.
- Production fix: T3.
- Green run and like-for-like recheck of V1, after the last production change: T4.

## Regression Test Strategy

The smallest deterministic maintained boundary that can fail for this defect is an integration test of the
binary in `tests/cli.rs`, using a fake `gh` shell script like the ones its other scenarios already use: a script
that exits at once and leaves a background `sleep` holding its stdout and stderr. The behavior is a property of
real processes and inherited file descriptors, so a unit test with a channel would only restate the
implementation. Two tests:

- the run ends within the budget plus a small margin, with `deadline_exceeded` and exit `1`;
- a `SIGTERM` sent while the descendant holds the pipe ends the run promptly with `interrupted` and exit `1`.

Each test must be run red against the unfixed code and green after. A unit test of the new bounded collection
routine, with a reader that never finishes, is welcome for the routine itself.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                 | Notes / Expected Output                                                    |
| --- | ------ | -------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| T1  | DONE   | Add the deadline integration test and run it red                     | Red output recorded in `manual-verification-evidence.md`.                  |
| T2  | DONE   | Add the signal integration test and run it red                       | Red output recorded.                                                       |
| T3  | DONE   | Bound the reader wait on the success path by the deadline and signal | One collection routine used by both paths, or a recorded reason it is not. |
| T4  | DONE   | Run the tests green, then repeat V1 on the final build               | Green output and the recheck recorded, naming the commit subject it ran on.|
| T5  | DONE   | Update the skill and the #2581 spec text that describe this behavior | Links and wording only where the behavior changed.                         |

## Commit Points

| Task   | Coherent change set                                          | Commit policy                                        |
| ------ | ------------------------------------------------------------ | ---------------------------------------------------- |
| T1, T2 | The two failing tests (fix applied in the next commit)       | Commit with the fix if the repository requires green commits, otherwise commit red first and say so. |
| T3     | The bounded collection on the success path                   | Commit after focused validation and required review. |
| T4, T5 | Evidence and documentation                                   | Commit after focused validation.                     |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2712-2003-wait-for-pr-review-pipe-wait-after-exit/ISSUE.md`
- [x] Reproduction attempted and recorded before review (outcome: Reproduced)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created (#2712) and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Final like-for-like recheck of V1 recorded after the last production change
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/` (this close-out pull request)

### Progress Log

- 2026-11-04 12:00 UTC - AI assistant (Copilot SDK in VS Code) - Drafted the specification and reproduced the
  defect with the real binary and a fake `gh`; a 3 s budget took 25.0 s and a `SIGTERM` sent at 3.0 s was honored
  at 25.0 s - `manual-verification-evidence.md` V1.
- 2026-11-04 12:12 UTC - AI assistant (Copilot SDK in VS Code) - The maintainer approved the specification; GitHub issue #2712 created, labelled `Bug` and `Automation`, and linked as a sub-issue of EPIC #2003; the specification moved from `docs/issues/drafts/` to `docs/issues/open/` for a spec-only pull request.
- 2026-11-04 13:07 UTC - AI assistant (Copilot SDK in VS Code) - Correction: the drafting entry above (12:00) was stamped before the commands it
  describes ran. The reproduction and lint commands ran between 11:49 and 11:56 UTC, as the command output and
  `manual-verification-evidence.md` record, so the drafting stamp is wrong. The earlier entries are kept unchanged,
  and `last-updated-utc` now holds the time of this edit.
- 2026-11-06 16:01 UTC - AI assistant (Claude Code) - Pull request #2750 merged: the success path of `run()` now bounds the wait for the readers by the deadline and the interrupt flag, with the two regression tests (red on the unfixed code, green after) and the V2 recheck.
- 2026-11-09 10:47 UTC - AI assistant (Claude Code) - Pull request #2756 merged: it addressed Copilot's review of #2750, which was submitted 67 s before that merge (fake `gh` descendant ignores `SIGHUP`, one local interrupt flag). The like-for-like recheck of V1 was repeated on that final build, `develop` at `92665ae17`: V3 in `manual-verification-evidence.md`. Spec moved to `docs/issues/closed/` by the close-out pull request.
- 2026-11-09 11:16 UTC - AI assistant (Claude Code) - Implementation completion review: no retrospective is needed. The design followed the plan (one bounded collection routine for both paths, option 1 for partial output), the regression tests ran red and then green, and V1 was rechecked on the final build as V3. The only deviation was process, not design: Copilot's review of #2750 arrived 67 s before its merge, so its three findings were handled as the post-merge follow-up #2756, as `process-pr-review` prescribes. No other reusable lesson came up.

## Acceptance Criteria

- [x] AC1: With a `gh` that exits successfully while a descendant keeps its pipes open, a run with
      `--timeout-seconds N` ends within N seconds plus a small margin, with a `deadline_exceeded` record and exit `1`.
- [x] AC2: In the same situation, `SIGINT` and `SIGTERM` end the run promptly with an `interrupted` record and exit `1`.
- [x] AC3: A `gh` that closes its pipes normally still returns its full output on every path.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] The final recheck of V1 is recorded in `manual-verification-evidence.md`
- [x] Documentation is updated where the behavior description changed

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p wait-for-pr-review`
- `cargo clippy -p wait-for-pr-review --all-targets`
- Pre-push checks

### Manual Verification Scenarios

| ID | Scenario                                                | Human-oriented command/steps                                                                                   | Expected Result                                                                  | Status | Evidence                                     |
| -- | ------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1 | A descendant holds the pipes after `gh` exits (before)  | `evidence/fake-gh-leaves-pipe-open.sh` as `gh` on `PATH`, `wait-for-pr-review --pr-number 7 --timeout-seconds 3` | The run takes about 25 s instead of 3 s                                          | DONE   | `manual-verification-evidence.md` section V1 |
| M2 | The same, after the fix                                 | The same command on the final build                                                                            | Ends at about 3 s with `deadline_exceeded` and exit `1`; `SIGTERM` at 3 s ends it at about 3 s | DONE   | `manual-verification-evidence.md` section V3 |

Every recorded command result names the toolchain that produced it.

### Disposable Verification Scripts

`evidence/fake-gh-leaves-pipe-open.sh` is a disposable reproduction script, not a maintained test. A shell script
is the right tool because the scenario needs a separate process on `PATH` that exits while a descendant keeps the
pipes open, which a Rust unit test cannot express without spawning the same shell script. It lives in this
folder. The durable check is the integration test of T1 and T2, which embeds an equivalent script like the
existing scenarios in `tests/cli.rs`; the implementer removes or archives the disposable script with the issue.
It is shell, not Python.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | Test `it_should_end_at_the_deadline_when_gh_exits_but_a_descendant_keeps_its_pipes_open`; V3 run 1 (final build) |
| AC2   | DONE                   | Test `it_should_stop_on_sigterm_when_gh_exits_but_a_descendant_keeps_its_pipes_open`; V3 run 2 (final build; SIGINT shares the flag, covered by the unit test of `collect`) |
| AC3   | DONE                   | Existing `tests/cli.rs` scenarios and `it_should_return_what_a_reader_sends_while_the_wait_is_in_progress` |

## Risks and Trade-offs

- A bound that is too tight could cut off the output of a slow but healthy `gh`. Mitigation: bound by the call's
  own deadline and interrupt flag, not by a new fixed timeout, and keep a short grace only for the case where the
  child has already exited.
- Returning partial data would be worse than failing. Mitigation: the design decision above (option 1).

## Implementation Completion Review

- Retrospective: Not needed. The reasons are in the progress-log entry of 2026-11-09.
- If no retrospective is needed, add a concise progress-log entry explaining why.

## References

- Related issues: #2581 (closed, implemented the poller), #2003 (parent EPIC)
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
