---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2536-1669-hide-statistics-repository-read-lock/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2537 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2537>.

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

Copilot review 5521920535 (COMMENTED, submitted 2026-10-14 17:25:40 UTC, at head `819909534`)
carries three inline findings with `[<Severity>][<FindingId>]` headers: F1 and F2 (`Major`) and F3
(`Minor`). Its overview rates them medium, low, and low; the rows use the inline headers. The
overview's summary and change table contain no further request, so they add no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2537-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2537-f2` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2537-f3` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A synchronous closure does not prevent blocking

- PR number: 2537
- Source review ID: 5521920535
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274060593>
- Concern: the Design and Ownership Review said a synchronous `FnOnce(&Metrics) -> R` prevents the
  closure from blocking. It prevents only `.await`; blocking I/O, `thread::sleep`, or long CPU work
  can still stall writers while the read lock is held.
- Solution: the section now separates what the API enforces (no `.await`) from the caller
  obligation (no blocking, no nested `read_metrics()` on the same repository). The proposed
  `read_metrics()` rustdoc states the obligation, and the design rationale points to it.
- Current-tree verification: at head `0f2a02bed`, `ISSUE.md`:179-188 lists the enforced rule and
  the caller obligation, :94-97 is the proposed rustdoc, and :120-122 is the updated rationale.
- Resolution reference: docs(issues): [#2536] record non-blocking read closures as a caller obligation
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275421008>

### F2 - Required test-design and maintainer checkpoints are missing

- PR number: 2537
- Source review ID: 5521920535
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274060692>
- Concern: T3-T7 add doctests, but the plan omitted the `create-issue` skill's per-increment
  test-design review with the prose-first Arrange-Act-Assert comparison and the maintainer stop
  after the final increment.
- Solution: a Test-Development Workflow subsection applies the `write-unit-test` skill to each of
  T3-T7: red/green for each `compile_fail` doctest, then a recorded design review and prose-first
  comparison, finished before the next task. For a `compile_fail` doctest the comparison is recorded
  in compile-and-error terms rather than skipped. A maintainer stop follows T7, T8's notes repeat
  the gate, and two workflow checkpoints track both.
- Current-tree verification: at head `0f2a02bed`, `ISSUE.md`:227-247 holds the subsection, the T8
  row names the gate, and :257-258 are the new checkpoints.
- Resolution reference: docs(issues): [#2536] require the per-increment test-design review and the maintainer stop
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275421321>

### F3 - The debug-log example changes the log text

- PR number: 2537
- Source review ID: 5521920535
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274060789>
- Concern: formatting the `String` returned by the closure with `{:?}` adds quotes and escapes, so
  the example would change `stats: Metrics { .. }` into a quoted string.
- Solution: the example formats the returned `String` with `{}`, and the risk explains why.
- Current-tree verification: at head `0f2a02bed`, `ISSUE.md`:361 uses `"stats: {}"`. In tokio
  1.53.3, the read guard's `impl fmt::Debug` (in its `read_guard.rs`) delegates to the inner value,
  so today's log already prints the `Metrics` debug text that the fixed example keeps.
- Resolution reference: docs(issues): [#2536] keep the debug-log example's output unchanged
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275421568>

## Processing Log

- 2026-10-14 17:30 UTC - Started audit: read Copilot review 5521920535 (three inline findings) and
  fixed F1. Its signed commit first failed with a GPG passphrase timeout; at the maintainer's
  choice the same signed commit was retried and succeeded.
- 2026-10-14 19:40 UTC - Fixed F2 and F3 in separate commits, pushed through the pre-commit and
  pre-push hooks, re-verified the three claims at head `0f2a02bed`, and replied on all three
  threads. The threads are resolved after this audit is pushed.

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
