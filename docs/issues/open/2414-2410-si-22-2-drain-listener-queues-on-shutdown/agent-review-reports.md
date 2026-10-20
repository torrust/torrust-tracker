---
semantic-links:
  related-artifacts:
    - .github/agents/task-reviewer.agent.md
    - docs/issues/open/2414-2410-si-22-2-drain-listener-queues-on-shutdown/ISSUE.md
---

# Agent Review Reports - Drain Listener Queues on Shutdown

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-20 15:53 UTC - Task Reviewer (before push)

- Invocation scope: the whole branch `2414-drain-listener-queues-on-shutdown` before any push or pull
  request: the 11 signed commits on `torrust/develop` at
  `docs(issues): [#2414] record the T5 evidence and the final M1 to M3 recheck`, against #2414's
  acceptance criteria AC1 to AC7 (intentionally left open, for the reviewer), the Verification Plan
  M1 to M3, and the sub-EPIC design record. Read-only.
- Inputs: the spec, `manual-verification-evidence.md` V1 to V10, and `evidence/`; the sub-EPIC's
  entries 27 and 28, D1, D9, D13, D14, D18, Chosen Design, Regression Test Strategy, and
  `adr-draft.md` section 7; the code of `packages/events`, the six listener files, the tracker-core
  test files, the bootstrap jobs, and the two test environments; the write-unit-test, fix-bug, and
  structured-runtime-logging skills; the evidence template; the commit-id check of the
  open-pull-request skill.
- Evidence (code facts read from source unless a command is named):
  - Ran `cargo test -p torrust-tracker-events` (20 passed), `cargo test -p torrust-tracker-core` (155,
    12, 16, and 2 passed), and `linter all` (exit 0). The commit-id check printed nothing, and
    `rg 'it_should_prioritize' packages src` found nothing.
  - Seven drain call sites, each with the `biased; cancelled()` arm, `drain(&mut **receiver, ...)`,
    `log_drain_report!`, and `return Completion::Cancelled`. `drain.rs` checks the deadline once per loop turn
    and awaits a taken event with no `timeout_at`. The macro logs only scalar fields; no `?` or `{:?}`
    appears in production logging.
  - tokio 1.53.3: `Receiver::len` is `tail.pos - self.next`, and `Instant + Duration` is
    `Instant::from_std(self.std + other)`, which panics on overflow.
  - Three mutation claims of V5 and V7 were spot-checked by reasoning from the source (not re-run) and
    matched; the V6 arithmetic matched its elapsed times.
  - `last-updated-utc` equals the newest log entry in each touched spec and is not after the committing
    commit; log entries name the model; the evidence follows the template.
- Findings:
  - S1 (should-fix): the spec's note "test cooperative scheduling as well" is neither met nor
    addressed; `drain` never yields, so a handler that never waits holds a worker until the timeout.
  - S2 (should-fix, D14): the new `axum-http-server` to `torrust-tracker-events` dependency edge, and
    the `Duration::ZERO` listener-level timeout tests instead of the Regression Test Strategy's
    paused-time wording, are not in record entry 28, and the maintainer's agreement to them is not
    recorded.
  - N1 (nit): six scripted receivers import `TryRecvError` inside a function, against the "imports at
    the top of the file" rule.
  - N2 (nit): `drain` computes `Instant::now() + timeout`, which panics for a huge `Duration`; no
    production caller passes one today.
  - N3 (nit): V6 says "four runs with one torrent" (three runs, plus one with distinct torrents) and "about
    17% apart" (16% or 19%, depending on the base).
  - N4 (nit): the metric assertions of the new listener tests carry no causal message; the http-core
    policy test's zero assertion also holds under the cancel-first bug, and only the bypass mutation
    guards it.
  - N5 (nit): no unit test covers the log lines, `TryRecvError` `Display`, the `Receiver::is_empty`
    default, a handler panic during a drain, or the lag path during a listener-level drain.
  - N6 (nit): the spec's frontmatter `status` is `planned` while the sub-EPIC row says `IN_PROGRESS`.
- Verdict: REVIEW PASSED
- Acceptance criteria (reviewer's table, to be ticked by the caller as the reviewer proposed): AC1
  PASS; AC2 PASS WITH NOTE (the log fields are shown on the real binary only); AC3 PASS WITH NOTE (the
  measurement is recorded and the spec states honestly that a full buffer cannot drain within the
  deadline, so the 5 s default is a policy choice the maintainer agreed); AC4 PASS; AC5 PASS; AC6 PASS;
  AC7 PASS WITH NOTE (helper-level coverage of the admission rule; S1 open); the final box PASS.
- Follow-up actions (recorded by the caller, GitHub Copilot, Claude Sonnet 5.5, after the maintainer's
  decisions of 2026-10-20):
  - S1: fixed with a red-first test and a `yield_now` after each handled event (V11).
  - N2: confirmed by a scratch program and fixed with a red-first test and `checked_add` (V11).
  - S2: the maintainer chose to re-export the constant from `http-core` instead of keeping the
    dependency edge (done), and agreed to the zero-timeout listener tests; both are in record entry 29
    and `adr-draft.md` is updated.
  - N1, N4: fixed (no behavior change). N3: the V6 text is corrected. N6: the status is `in-progress`.
  - N5: the log lines stay verified on the real binary; `TryRecvError` `Display` is not tested, because
    `RecvError` has none either; a handler panic is a non-goal; the lag path stays covered at helper
    level; an `is_empty` test was added (V11).
  - The fixes changed production code (the drain), so the recheck of M1 to M3 was repeated on the new
    build (V11). A re-review of the fixes follows in the next entry.
