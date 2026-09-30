---
semantic-links:
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - docs/agents/orchestration.md
---

<!-- markdownlint-disable MD003 -->

# Agent Review Reports - Define UDP Active-Request Shutdown Policy

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-09-30 06:21 UTC - GitHub Copilot (Task Reviewer)

- Invocation scope: Pre-PR task review of AC1-AC16, the generic criteria, D1-D9, and implementation
  constraints 1-9 for branch `2370-1488-si-15-define-udp-active-request-policy` (base `02c026c0`;
  commits `484697a3` T1 through `fc750380` T7; uncommitted T8 evidence edits in `ISSUE.md` and
  `manual-verification-evidence.md`). Changed code: `launcher.rs`, `processor.rs`,
  `request_buffer.rs`, and `Cargo.toml` (udp-server). Changed docs: new request-concurrency ADR,
  eviction ADR, ADR index, udp-server `README.md`, `task-inventory.md`, and EPIC row 11.
- Inputs: `ISSUE.md`, `manual-verification-evidence.md`, `performance-evidence.md`,
  `git diff 02c026c0..HEAD`, `git diff`, `src/bootstrap/jobs/udp_tracker.rs` (unchanged),
  `testing/environment.rs`, the `review-task` Test Design checklist, and the SI-14 report precedent.
- Evidence (reviewer re-runs):
  - `cargo test -p torrust-tracker-udp-server`: 190 unit, 11 contract, 1 doc test passed; 0 ignored.
  - `cargo test -p torrust-tracker --lib udp`: 12 passed (the progress log records 11).
  - `receive_loop_shutdown` and `request_drain` tests: 10 of 10 consecutive runs passed (7 tests).
  - `linter all`: exit `0`. Frontmatter validator on the issue folder, udp-server ADRs,
    shutdown feature docs, and EPIC folder: exit `0`. `linter lychee`: exit `0`.
  - `.tmp/si15-pre-push.out` (written after `fc750380`): all pre-push checks passed.
  - `git log --format=%G?`: all seven commits signed. `0b870a68` (PR #2372, spec-only) is an
    ancestor of the base.
  - B0/B1/B2 means recomputed from the run tables: 159413.39, 164634.37, 167716.20.
  - Mutation of the receive-error path was reasoned, not executed (no code changes allowed).
- Acceptance criteria:
  - AC1 PASS: `launcher.rs:416` spawns every processor into the loop-owned `JoinSet`; nothing
    else spawns processors. Regression test red with 48 orphans (V2), green, and mutation-proven.
    The 48 count matches the `force_push` traversal analysis, so it exercises the orphaning path.
  - AC2 PASS: `biased` `select!` runs the drain inside the cancelled branch (`launcher.rs:353`);
    the receiver is not polled afterwards.
  - AC3 PASS: paused-time completion test; companion loopback test proves an accepted request
    released after cancellation answers before the loop returns.
  - AC4 PASS: paused-time deadline test counts `aborted: 1`; the loopback regression test uses a
    never-released gate, so the 100 ms drain must abort and join (0 running at return).
  - AC5 PASS: panic -> `error` log and `failed`; `Err` -> `failed` with no duplicate log; the drain
    loops have no early exit. See finding 4 for the test.
  - AC6 PASS: pre-drain cancellation counts `evicted`; the deadline branch reaps finished tasks
    with `try_join_next` before `abort_all`, and only post-`abort_all` cancellations count `aborted`.
  - AC7 PASS: cancellation path tested (0 running, `Ok(())`, socket rebinds). Receive error calls
    `JoinSet::shutdown` (abort all and await all) before `return Err` (`launcher.rs:363`); verified
    by inspection only. See finding 2.
  - AC8 PASS: one summary with all four counters and `elapsed`; `warn` when `failed` or `aborted`
    is non-zero, else `info`; drain start `debug`/`info`; deadline `warn` with `remaining` and
    `deadline`. No `Event` variant, metric, or statistics change in the diff.
  - AC9 PASS: `udp_tracker.rs` is unchanged; the loop still returns `Ok(())` after any drain, and
    component tests pass. M1/M2 log "Job completed after cooperative cancellation".
  - AC10 PASS: `request_buffer.rs` changes only name the capacity (`ACTIVE_REQUESTS_CAPACITY = 50`)
    and add link comments; `force_push` and its tests are byte-identical.
  - AC11 PASS: B1 mean above lowest B0 run; B2 mean above lowest B0 and B1 runs.
  - AC12 PENDING: the cancellation paths (AC3-AC6 and AC7 cancellation) are covered without OS
    signals; loopback is local, and the real-time bounds do not decide outcomes. The AC7
    receive-error join has no test and no recorded rationale (finding 2). The D9 real-time
    deviation is not recorded (finding 5).
  - AC13 PASS: M2 records the `SIGTERM` line, the drain summary `active=5 completed=5`, exit `0`;
    M3 rebinds immediately and serves an announce.
  - AC14 PASS: `ProcessorError::{EncodeResponse, SendResponse}`; tests assert `Ok` for discarded and
    error-response requests and `SendResponse` for an unreachable IPv6 client. Log messages and
    field names (`e`, `error`) are unchanged. Encode failure is untested, which is reasonable
    because it cannot be induced from valid responses.
  - AC15 PENDING: design, history, ownership split, classified alternatives, and triggers are
    present; index row and eviction-ADR link exist. The planned known trade-offs (orphaned
    handles, inexact accounting) are missing (finding 3).
  - AC16 PASS: every Semantic Link Map entry exists. The drain is an item in `launcher.rs`, so
    item-level `// ADR:` and `// issue: #2370` comments on `drain_request_processors` satisfy the
    map's intent, which is discoverability from the code that implements the drain. Validator and
    lychee pass.
  - Generic: `linter all` PASS; relevant tests PASS; manual scenarios PASS; documentation PASS;
    "acceptance criteria re-reviewed" PENDING until AC12 and AC15 are resolved.
- Ownership on every path: token cancellation (drain and join), receive error (`shutdown`),
  receive-loop abort through `OwnedReceiveLoop`/`OwnedTask` drop (`JoinSet` drop aborts all),
  legacy halt, dropped halt sender, and launcher abort (same loop). No spawn bypasses the set.
- Test Design: names, visible Arrange/Act/Assert, and independent expected results are good.
  `ReceiveLoopWithOrphanedProcessors` and `ReceiveLoopWithOneHeldRequest` are named for their
  causal state and own only the ordering mechanics. Prose-first evidence is recorded for T3, T4,
  and T5. Findings 4 and 5 apply.
- Findings:
  1. Major - Completion review missing. `ISSUE.md` "Implementation Completion Review" still says
     `Retrospective: Not yet assessed`, and there is no progress-log rationale. Reusable lessons
     exist: paused Tokio time could not drive the loopback tests, so a real 100 ms deadline and a
     deadline-injection entry point were used instead; `tracing` macros forced a
     cognitive-complexity split; shared-SQLite benchmark runs were invalid; the M3 harness
     ordering caused an IP ban; and the summary's `evicted` count excludes evictions already
     reaped (M2: 85 evictions, `evicted=0`). Fix: create `implementation-retrospective.md` from
     `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` and update the completion-review line.
  2. Medium - `launcher.rs:363`: the receive-error `processors.shutdown().await` is untested.
     Deleting it passes every test, because dropping the `JoinSet` aborts processors but does not
     join them, so the socket release is no longer awaited. Fix: add a deterministic test if an
     existing seam can inject a receive error without a production hook. Otherwise record under
     AC12 (policy 4) why a real UDP receive error cannot be induced deterministically on loopback,
     and cite the `JoinSet::shutdown` contract as the evidence.
  3. Medium - `20260929181216_bound_udp_request_concurrency_with_task_per_request_ring.md` does not
     record the known trade-off the spec's ADR plan requires. A full-ring `force_push` can drop up
     to 48 live handles, so those processors run outside the 50-request bound (inexact overload
     accounting) and, before issue #2370, were orphaned. Agreement item 2 also describes
     `force_push` incompletely. Once this issue closes, the fact survives only in the spec. Fix:
     add a "Known Trade-offs" section and correct Agreement 2. Optionally hyperlink issues #566,
     #611, #918, and #2149 as the plan asked ("links to every PR, issue, and commit").
  4. Low - `launcher.rs:1023` `it_should_count_failed_processors_and_keep_draining_the_rest` does
     not prove the drain continues after a panic. `panicking_processor` yields first
     (`launcher.rs:978`), so it is joined last; a `break` after a panic would still yield
     `completed: 1, failed: 2`. Fix: order completion causally, for example have the panicking
     processor send on a `oneshot` before panicking and the `Ok` processor await it.
  5. Low - D9 level 2 planned paused Tokio time and asserting summary counts. The loopback tests
     use a real 100 ms drain deadline (`launcher.rs:1105`), real-time `LIFECYCLE_TIMEOUT` bounds,
     and `wait_until_bindable` polling, and they assert running count, result, and rebind rather
     than counts. The outcomes stay deterministic because the gate alone decides them, and counts
     are asserted by the paused-time unit tests. This is acceptable under constraint 6, but the
     deviation is unrecorded. Fix: note it in D9 or the retrospective.
  6. Info - `testing/environment.rs:24` `DEFAULT_SERVER_LIFECYCLE_TIMEOUT` (5 s) equals
     `REQUEST_DRAIN_DEADLINE` (5 s). An environment stop with a slow processor would hit the test
     timeout panic before the drain aborts it. This does not fail today, because real processors
     finish in microseconds. Fix: note it for SI-17, or give the environment a bound above the
     drain deadline.
  7. Nit - The operator-facing counters are ambiguous: the `active` field also counts finished tasks
     not yet joined (M1 `active=1` for a finished announce), and `evicted` counts only evictions
     not yet joined. Both follow D3, but neither is documented where operators look. Also,
     `log_request_drain_start` omits `deadline` when idle. Fix: one sentence in the task
     inventory or ADR.
  8. Nit - `performance-evidence.md` repeats the shared-SQLite exclusion paragraph before the B0
     table and after "Result". Remove one copy.
  9. Nit - Commit `9fe13a95` lacks the `[#2370]` reference, and the T2/T7 scopes (`adrs`,
     `shutdown`) differ from the Commit Points instruction to use `udp-server`. Do not rewrite
     history; noted for consistency only.
  10. Nit - The map places the `run_udp_server_main` markers "at the processor spawn and drain
      call"; they sit on the function instead. Accepted as equivalent.
- Issue spec updates: ticked AC1-AC11, AC13, AC14, AC16, and the generic `linter all`,
  relevant tests, manual verification, and documentation criteria. Set their Acceptance
  Verification rows to `DONE` with evidence. Ticked workflow items: spec-only PR merged (PR
  #2372), implementation completed, automatic verification, manual verification, reviewer
  validation, and reviewer report recorded. Left AC12, AC15, "acceptance criteria re-reviewed",
  "completion review recorded", and issue closure open. Set T9 to `IN_PROGRESS` and added a
  progress-log entry.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Implementer: resolve findings 1-3 (required), then 4-5 (recommended); re-request Task Reviewer
    for AC12, AC15, and the completion review.
  - Implementer (optional): findings 6-10.

### 2026-09-30 07:52 UTC - GitHub Copilot (Task Reviewer)

- Invocation scope: Re-review of the 2026-09-30 06:21 UTC `REVIEW FAILED` report after the
  implementer's fixes (progress-log entry 2026-09-30 07:27 UTC). Committed history up to `fc750380`
  was reviewed earlier; this entry covers the uncommitted working tree: `launcher.rs`, the
  request-concurrency ADR, `task-inventory.md`, the SI-17 draft, `performance-evidence.md`,
  `ISSUE.md`, and the new `implementation-retrospective.md`.
- Inputs: the earlier report (findings 1-10), `git diff`, `git status`, `request_buffer.rs`
  (`force_push`), `implementation-retrospective.md` against
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`, and the `review-task` Test Design checklist.
- Evidence (reviewer re-runs):
  - `cargo test -p torrust-tracker-udp-server`: 191 unit, 11 contract, 1 doc test passed; 0 ignored.
    Before and after the mutations below. `request_drain`: 10 of 10 consecutive runs, 6 tests each.
  - Mutation A (finding 2): `processors.shutdown().await` replaced with `processors.abort_all()` in
    `join_request_processors_after_receive_error`. The new receive-error test fails with `running`
    `left: 1, right: 0`.
  - Mutation B (finding 2, wiring): the receive-error branch in `run_udp_server_main` returned
    `Err(error)` without the helper. Lib tests pass, but `cargo check -p
    torrust-tracker-udp-server` fails with ``function `join_request_processors_after_receive_error`
    is never used`` under the workspace `-D warnings`. The wiring is therefore guarded by the build.
  - Mutation C (finding 4): the first drain loop breaks after a panicked join. The panic test fails
    with `completed: 0, failed: 2` against the expected `completed: 1, failed: 2`.
  - Every mutation was restored by hand. `git diff -- launcher.rs` was saved before the mutations and
    is byte-identical afterwards (`cmp`); the file SHA-256 is unchanged (`60a1f265...`).
  - `cargo +nightly fmt --all -- --check`, `linter clippy`, `linter markdown`, `linter cspell`,
    `linter lychee`, and `linter all`: all exit `0`. The frontmatter validator on the eight changed
    or new Markdown files (issue folder, ADR, task inventory, SI-17 draft) exits `0`.
  - Retrospective claims checked: commit SHAs match `git log`; `b0-tracker.toml` exists; the M2
    `stop_ms=1108` and 1.8 ms figures appear in `manual-verification-evidence.md`.
  - ADR Agreement 2 and Known Trade-offs checked against `force_push`: oldest-first traversal, one
    yield per still-active task, abort only when nothing finished first, re-insertion of only the
    last traversed active handle, so 50 - 1 - 1 = 48 live handles can be dropped.
- Prior findings:
  1. Resolved. `implementation-retrospective.md` follows the template headings, records D9, the
     receive-error test gap, the panic ordering, the clippy-driven split, the snapshot semantics,
     and root causes, and its evidence is accurate. The completion-review line links it.
  2. Resolved. The extracted helper is tested deterministically with paused time and is
     mutation-proven (A). Bypassing it at the call site fails the non-test build (B). The
     retrospective records why a real receive error cannot be produced on loopback without a
     production hook.
  3. Resolved. Agreement 2 now matches `force_push`. A "Known Trade-offs" section records the inexact
     bound, the up-to-48 dropped live handles, the `JoinSet` ownership split, and the snapshot
     semantics. Issues #566, #611, #918, and #2149 are hyperlinked.
  4. Resolved. A `oneshot` orders the `Ok` processor after the panic; mutation C proves the test now
     detects a drain that stops after a panic.
  5. Resolved. The D9 real-time deviation is recorded in the retrospective.
  6. Resolved. Recorded in the SI-17 draft Background and in the progress log.
  7. Resolved. `task-inventory.md` explains the `active` and `evicted` snapshot semantics and names
     the `aborting request` warning as the complete eviction record.
  8. Resolved. One copy of the shared-SQLite paragraph remains, before the B0 table.
  9. Accepted, no change. History is not rewritten.
  10. Accepted, no change.
- Acceptance criteria (changes since the earlier report; all others remain `PASS`):
  - AC5 PASS: the panic test now proves the drain continues after a panic (mutation C).
  - AC7 PASS: receive-error join is tested and mutation-proven (A), and its wiring is build-guarded
    (B).
  - AC12 PASS: AC3-AC6 and the AC7 cancellation and receive-error paths are covered by
    deterministic tests. They use paused time or loopback sockets whose outcome is decided by the
    release gate, not by timing. No OS signals or external network. The D9 deviation is recorded.
  - AC15 PASS: the ADR now records the known trade-offs the plan required. Index row and
    eviction-ADR link were verified earlier.
  - Generic "acceptance criteria re-reviewed" PASS: every AC row matches the verified behavior.
- Test Design (changed tests `it_should_count_failed_processors_and_keep_draining_the_rest` and
  new `it_should_join_every_processor_before_returning_the_receive_error`): names, a visible
  causal Arrange, a visible production Act, independent expected results, and deterministic
  execution all pass. The retained ordering comment gives irreducible context. See finding 11 for
  the missing prose-first evidence and finding 13 for the duplicated guard.
- New findings:
  11. Medium (blocking under the Test Design checklist) - The 2026-09-30 07:27 UTC progress-log entry
      and the retrospective record no prose-first Arrange-Act-Assert comparison for the two changed
      tests, although the T3, T4, and T5 entries did. Fix: add a "Prose-first review" bullet for
      both tests to the 07:27 entry or a new progress-log entry. For each test, state the temporary
      Arrange, Act, and Assert prose, confirm that the code expresses it, and justify the retained
      ordering comment.
  12. Nit - `last-updated-utc` was not bumped in files edited on 2026-09-30:
      `docs/features/shutdown-process/task-inventory.md` (`2026-09-29`) and
      `docs/issues/drafts/1488-si-17-migrate-standalone-udp-environment/ISSUE.md`
      (`2026-09-29 11:56`). Fix: update both values.
  13. Nit - `launcher.rs` test module `request_drain::RunningProcessor` duplicates
      `receive_loop_shutdown::RunningHeldProcessor` (identical `Drop` guard). The receive-error
      test also initializes the counter to `1` separately from creating the guard. Optional fix:
      hoist one guard into the parent `tests` module, with a constructor that increments the
      counter.
- Issue spec updates: ticked AC12, AC15, the generic "acceptance criteria re-reviewed" criterion,
  and the workflow checkpoints "Acceptance criteria reviewed after implementation" and
  "Evidence-based implementation completion review recorded". Updated the AC5, AC7, AC12, and AC15
  Acceptance Verification rows. T9 stays `IN_PROGRESS` because of finding 11. Added a progress-log
  entry. No production or test code changed.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Implementer: resolve finding 11 (documentation only), then request a focused Task Reviewer
    confirmation of the Test Design evidence. After that, T9 can be set to `DONE`.
  - Implementer (optional): findings 12 and 13.

### 2026-09-30 07:58 UTC - GitHub Copilot (Task Reviewer)

- Invocation scope: Final re-review of the 2026-09-30 07:52 UTC `REVIEW FAILED` report after the
  implementer's fixes for findings 11-13 (progress-log entry 2026-09-30 07:56 UTC). This covers the
  uncommitted working tree: `launcher.rs` tests, `ISSUE.md`, `task-inventory.md`, and the SI-17
  draft. Acceptance criteria verified in the earlier reports were not re-derived. Only their
  regression evidence was re-run.
- Inputs: the 07:52 report, the 07:56 progress-log entry, `git diff -- launcher.rs`, the full
  `launcher.rs` test module, and the `review-task` Test Design checklist.
- Evidence (reviewer re-runs):
  - `cargo test -p torrust-tracker-udp-server`: 191 unit, 11 contract, 1 doc test passed; 0 ignored.
  - Mutation D: `processors.shutdown().await` was replaced with `processors.abort_all()` in
    `join_request_processors_after_receive_error`. With the shared guard,
    `it_should_join_every_processor_before_returning_the_receive_error` fails with `left: 1,
    right: 0`. The other five `request_drain` tests pass. The mutation was restored by hand.
    `git diff -- launcher.rs` was saved beforehand and is byte-identical afterwards (`cmp`), and
    the file SHA-256 is unchanged (`5fd420b8...`). Lib tests pass again: 191.
  - `cargo +nightly fmt --all -- --check` and `linter clippy` both exit `0`.
  - `linter markdown`, `linter cspell`, and `linter lychee` all exit `0`. The frontmatter
    validator on the changed and new Markdown files also exits `0`.
- Prior findings:
  11. Resolved. The recorded prose matches both test bodies.
      - `it_should_count_failed_processors_and_keep_draining_the_rest`: the Arrange spawns an
        `Err` processor, a panicking processor that signals a `oneshot`, and an `Ok` processor
        that awaits it. The Act is `drain_request_processors`. The Assert is an independent
        `RequestDrainOutcome { completed: 1, failed: 2, .. }`. The one comment gives the
        irreducible ordering context, which the spawn lines do not show.
      - `it_should_join_every_processor_before_returning_the_receive_error`: the Arrange is one
        pending processor counted by `RunningProcessor::start`. The Act is
        `join_request_processors_after_receive_error` with a `ConnectionReset` error. The asserts
        check the returned error kind, a zero running count with a message, and an empty set.
        "Handed back unchanged" is asserted through the error kind, which is sufficient because
        the helper only moves the value.
  12. Resolved. `task-inventory.md` is now `2026-09-30`, keeping its date-only format. The SI-17
      draft is now `"2026-09-30 07:56"`.
  13. Resolved without semantic change. The single `tests::RunningProcessor` has a `start`
      constructor that calls `fetch_add` before constructing the guard, and its `Drop` calls
      `fetch_sub`.
      - `HoldAcceptedRequestsSender::send` still counts the processor before `held.send`, so
        `wait_until_held` can never observe a held processor that is not yet counted. The guard is
        still moved into the returned future, as before.
      - The receive-error test counter starts at `0`, and `start` raises it to `1` before the
        guard is moved into the pending processor. Mutation D confirms that the guard still
        detects a processor left running.
      - `RunningHeldProcessor` is gone.
- Acceptance criteria: AC1-AC16 and the generic criteria remain `PASS`, with the evidence recorded
  in the 06:21 and 07:52 reports. No behavior changed since 07:52. The only production diff is the
  receive-error helper that the 07:52 report verified.
- Test Design: the prose-first comparison is now recorded for every changed test. The shared guard
  gives one coherent capability ("a running processor") a meaningful name at the caller's
  abstraction level. No violations remain.
- New findings: none.
- Completion review: `implementation-retrospective.md` is recorded and linked (verified at 07:52).
  No new material deviation.
- Issue spec updates: set T9 to `DONE`, updated the `last-updated-utc` value, and added a
  progress-log entry. Every acceptance criterion and every verifiable workflow checkpoint was
  already ticked. "Issue closed and spec moved" stays open.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Caller: request Committer to include this report with the reviewed working-tree change set.
    Issue closure and the spec move to `closed/` happen after merge.
