---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2578 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2578>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Copilot's review 5588723129 (submitted 2026-10-20 17:07 UTC, state `COMMENTED`) has three inline findings. Its body repeats them as an overview and contains no other independently actionable assertion, so it has no row. The comments carry no `[Severity]` prefix, so each severity is inferred from the wording.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2578-f1 | Copilot | Suggestion (inferred) | other | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2578-f2 | Copilot | Minor (inferred) | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2578-f3 | Copilot | Nit (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Yielding after every drained event can reduce drain throughput

- PR number: 2578
- Source review ID: 5588723129
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327877644
- Concern: `drain` in `packages/events/src/drain.rs` yields after every handled event, which can cut throughput for fast handlers and make timeouts more likely; the reviewer suggests yielding every N events or making the trade-off explicit.
- Solution: Kept the yield after every event and made the trade-off explicit. The cost was measured on a release build with a no-op handler and the full 65,536-event buffer: about 0.29 microseconds per event on the multi-thread runtime and 0.40 on a current-thread runtime, so 19 to 26 ms per full buffer (about 0.4 to 0.5 percent of the 5 s default timeout), against 2 to 3 ms per event for the persistent listener's SQLite write (V6). Yielding every 64 events would save about 18 to 25 ms per full buffer but would hold the worker thread for 64 handler runs at a time, which the cooperative-scheduling requirement of the #2414 spec exists to prevent. The cost for the six in-memory handlers was not measured, so the statement that it is small for them is an inference. The `drain` comment and `adr-draft.md` section 7 now state the cost and the reason; no behavior changed.
- Current-tree verification: `cargo test --release -p torrust-tracker-events --test scratch_yield -- --nocapture --test-threads=1` with a temporary, uncommitted test (embedded in V12 of `manual-verification-evidence.md`) printed the medians above. `cargo test -p torrust-tracker-events` passed 23 tests after the comment change, and `cargo +nightly fmt --check` exited 0. The drain loop in `drain.rs` still yields after `handle(event).await`.
- Resolution reference: `docs(events): [#2414] state the measured cost of the per-event yield`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327998154

### F2 - Aborted listener handle is not awaited in TestEnv::stop

- PR number: 2578
- Source review ID: 5588723129
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327877714
- Concern: After `timeout_at` expires, `Listeners::stop` in `packages/tracker-core/tests/common/test_env.rs` calls `handle.abort()` but does not await the aborted handle, so the task may keep running briefly and `stop()` gives a weaker guarantee that receivers are dropped.
- Solution: No change. The abort is in the timeout branch, which always records a failure, so `stop()` returns `Err` and the calling test fails; no test passes through that branch. The success path (`Ok(Ok(()))`) means the task already finished and dropped its receiver, and `it_should_release_every_listener_receiver_when_the_environment_is_stopped` asserts both `Ok(())` and zero remaining subscribers. Awaiting an aborted task that never reaches an await point would turn a failed test into a hung one unless another timeout were added, for a path that already fails.
- Current-tree verification: Read from source, not run: `packages/tracker-core/tests/common/test_env.rs` (the `Err(_elapsed)` arm pushes to `failures`, and `stop` returns `Err(failures.join(...))` when `failures` is not empty) and `packages/tracker-core/tests/integration.rs` (`it_should_release_every_listener_receiver_when_the_environment_is_stopped`). The test itself passed in the pre-push run on this tree.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327998499
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327998499

### F3 - drain_events takes &Arc<Repository> where &Repository would do

- PR number: 2578
- Source review ID: 5588723129
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327877802
- Concern: `drain_events` in `packages/swarm-coordination-registry/src/statistics/event/listener.rs` takes `&Arc<Repository>` although the downstream handler needs only `&Repository`; the reviewer suggests the narrower type.
- Solution: No change. The premise does not hold: `handle_event` in `packages/swarm-coordination-registry/src/statistics/event/handler.rs` is declared with `stats_repository: &Arc<Repository>`, so a `&Repository` would not coerce to it and the helper would not compile without also changing that function's signature, which is outside #2414. The sibling `handle_receive_result` already took `&Arc<Repository>` on `develop`, and `drain_events` mirrors it.
- Current-tree verification: Inspected `handler.rs` (the `handle_event` signature) and `listener.rs`, and `git show torrust/develop:packages/swarm-coordination-registry/src/statistics/event/listener.rs` (`handle_receive_result` takes `&Arc<Repository>`).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327998830
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327998830

## Processing Log

- 2026-10-20 17:12 UTC - Fetched review threads and review 5588723129 for PR #2578 (3 unresolved threads, none outdated); started this audit.
- 2026-10-20 17:14 UTC - Measured the cost of the per-event yield for F1 (V12).
- 2026-10-20 17:16 UTC - Pushed `docs(events): [#2414] state the measured cost of the per-event yield`, the fix for F1.
- 2026-10-20 17:19 UTC - Replied on the three threads: F1 https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327998154, F2 https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327998499, F3 https://github.com/torrust/torrust-tracker/pull/2578#discussion_r4327998830. The threads were not yet resolved at this point.
- 2026-10-20 17:20 UTC - Resolved the three threads as `josecelano` (each mutation returned `isResolved: true`). A refreshed GraphQL fetch at 17:20 UTC showed all 3 threads resolved and none unresolved; no thread is outdated.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
