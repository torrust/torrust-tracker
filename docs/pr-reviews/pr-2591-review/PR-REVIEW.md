---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2591 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2591>.

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

Copilot's review 5596471839 (submitted 2026-10-21 07:45 UTC, state `COMMENTED`) has three inline findings, all on `src/bootstrap/jobs/manager.rs`. Its body says Copilot could not run its full agentic review before a timeout, then repeats the three findings as an overview and summarizes the change; it contains no other independently actionable assertion, so it has no row. The comments carry no `[Severity]` prefix; the overview labels each "Medium severity", which is not on this audit's scale, so each severity is inferred from the wording. The second comment repeats the first on the increment line, so F2 is recorded as a re-raise of F1. The maintainer approved the dispositions and the F1 test design before the fixes.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2591-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2591-f2 | Copilot | Minor (inferred) | correctness | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2591-f3 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The component id counter can wrap and reuse ids in release builds

- PR number: 2591
- Source review ID: 5596471839
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2591#discussion_r4334265218
- Concern: `ComponentRegistry::next_id` is advanced with `+= 1`, which wraps in release builds; a wrapped counter would reuse a `ComponentId` and corrupt the registry (the wrong component cancelled or released). The reviewer suggests a checked increment with an explicit failure mode.
- Solution: `reserve` now advances `next_id` with `checked_add(1)` and an `expect` naming the exhausted id space, with a comment saying why. The workspace `[profile.release]` sets no `overflow-checks`, so the old code wrapped silently there. Reaching it needs 2^64 reservations while bootstrap makes about 14, so `reserve` keeps returning a plain value instead of a `Result`, following the ADR that limits `Result` to concretely fallible public APIs. A new test, `it_should_refuse_to_issue_a_reused_component_id_when_the_id_space_is_exhausted`, builds a registry whose next id is `u64::MAX`, reserves once, and expects that panic message.
- Current-tree verification: `cargo test -q -p torrust-tracker --lib bootstrap::jobs::manager` passed 17 tests. With the old `self.next_id += 1;` put back (working tree only, restored from a copy), the new test failed in a dev build (it panicked with "attempt to add with overflow", not the expected message) and in a `cargo test --release` build ("test did not panic as expected"); with `wrapping_add(1)` it failed with "test did not panic as expected". The pre-commit checks passed, and the pre-push hook passed on the push that carried the fix.
- Resolution reference: `fix(bootstrap): [#2415] refuse to wrap the component id counter instead of reusing ids`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2591#discussion_r4334523215

### F2 - The same counter overflow, raised on the increment line

- PR number: 2591
- Source review ID: 5596471839
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2591#discussion_r4334265273
- Concern: The same text as F1, anchored on the `self.next_id += 1;` line instead of the field declaration.
- Solution: No separate change; it requests the same current-tree change as F1, which fixed it.
- Current-tree verification: `grep -n 'next_id' src/bootstrap/jobs/manager.rs` prints five lines: the field, the `ComponentId(self.next_id)` read, the two lines of the `self.next_id = self.next_id` assignment whose next line is `.checked_add(1)`, and the test's `next_id: u64::MAX`; no `+= 1` remains.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2591#discussion_r4334523584
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2591#discussion_r4334523584

### F3 - mark_spawned silently ignores a missing component id

- PR number: 2591
- Source review ID: 5596471839
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2591#discussion_r4334265309
- Concern: `ComponentRegistry::mark_spawned` does nothing when the id is missing, which could mask an invariant violation and leave a token untracked; the reviewer suggests failing loudly in debug or testing, for example with `debug_assert!`, or returning a result.
- Solution: The id cannot be missing: `spawn` first checks that the reservation came from this manager, and an entry is removed only by its own reservation's `Drop`, which runs after `mark_spawned`. `mark_spawned` now states that invariant in a comment and checks it with `debug_assert!`. No test was added, because the case is unreachable through the API.
- Current-tree verification: `cargo test -q -p torrust-tracker --lib bootstrap::jobs::manager` passed 17 tests and `cargo test -q -p torrust-tracker --lib app::` passed 17. With `spawn` changed to pass `ComponentId(u64::MAX)` to `mark_spawned` (working tree only, restored from a copy), the manager tests in a dev build panicked with "a live reservation must be registered in its manager". The pre-commit checks passed, and the pre-push hook passed on the push that carried the fix.
- Resolution reference: `refactor(bootstrap): [#2415] assert that a spawned reservation is registered`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2591#discussion_r4334523848

## Processing Log

- 2026-10-21 07:49 UTC - Fetched the review threads of PR #2591 (3 unresolved, none outdated) and review 5596471839; started this audit.
- 2026-10-21 08:13 UTC - Committed `fix(bootstrap): [#2415] refuse to wrap the component id counter instead of reusing ids` (F1) after the maintainer approved the dispositions and the F1 test design.
- 2026-10-21 08:15 UTC - Committed `refactor(bootstrap): [#2415] assert that a spawned reservation is registered` (F3).
- 2026-10-21 08:16 UTC - Pushed both fixes; the pre-push hook passed.
- 2026-10-21 08:16 UTC - Replied on the three threads: F1 https://github.com/torrust/torrust-tracker/pull/2591#discussion_r4334523215, F2 https://github.com/torrust/torrust-tracker/pull/2591#discussion_r4334523584, F3 https://github.com/torrust/torrust-tracker/pull/2591#discussion_r4334523848. The threads were not yet resolved at this point.
- 2026-10-21 08:21 UTC - Resolved the three threads as `josecelano` after `github-review-threads reply-status` confirmed a reply on each (each mutation returned `isResolved: true`). A refreshed GraphQL fetch at 08:21 UTC showed all 3 threads resolved and none unresolved; the F2 thread is now outdated, because the fix changed its line.

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
