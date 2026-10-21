---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2604 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2604>.

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

Copilot's review 5598583227 (submitted 2026-10-21 10:53 UTC, state `COMMENTED`) has one inline finding, on the per-clause mutation example in the proposed skill text of `docs/issues/open/2603-write-unit-test-skill-rule-mutations-and-removed-tests/ISSUE.md`. The review has no body beyond that comment, so it adds no other row. The comment carries no `[Severity]` prefix, so its severity is inferred from the wording.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2604-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The mutation example names a clause that the drop rule does not have

- PR number: 2604
- Source review ID: 5598583227
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2604#discussion_r4335928928
- Concern: The example said that breaking the clause "does not cancel others" by cancelling every token also breaks the clause "cancels its own token". Cancelling every token does not break "cancels its own token". In the evidence of #2415 (V4), M7 fails because the released reservation's token should stay uncancelled, while M7b cancels only the other tokens and exposes the missing assertion. The reviewer asks to use that invariant and to say "on drop", so the example explains why a mutation affecting both clauses can hide a test gap.
- Solution: Agreed. The example now states the rule as "a dropped reservation releases its state without cancelling unrelated components". Making a drop cancel every token breaks both clauses (the released reservation's own token is cancelled, and so are the others), so an assertion on the released token fails the test and hides that the other clause has no assertion; making a drop cancel only the other tokens breaks just the second clause and exposes the missing assertion. I also searched the spec, the issue body and the PR body for the same wording; the only instance was this one.
- Current-tree verification: `grep -n 'cancels its own token' docs/issues/open/2603-write-unit-test-skill-rule-mutations-and-removed-tests/ISSUE.md` prints nothing, and the staged diff before the commit contained only the replaced lines of the example. `linter markdown`, `linter cspell` and `linter lychee` exited 0, and the pre-commit checks passed. M7 and M7b are the rows 181 and 182 of `docs/issues/closed/2415-2410-si-22-3-per-component-cancellation-tokens/manual-verification-evidence.md`, whose wording the new example follows. The pre-push hook passed on the push that carried the fix.
- Resolution reference: `docs(issues): [#2603] correct the clauses in the per-clause mutation example`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2604#discussion_r4336003713

## Processing Log

- 2026-10-21 10:59 UTC - Fetched the review threads of PR #2604 (1 unresolved, none outdated) and review 5598583227; started this audit.
- 2026-10-21 11:01 UTC - Committed `docs(issues): [#2603] correct the clauses in the per-clause mutation example` (F1).
- 2026-10-21 11:02 UTC - Pushed the fix; the pre-push hook passed.
- 2026-10-21 11:02 UTC - Replied on the thread: F1 https://github.com/torrust/torrust-tracker/pull/2604#discussion_r4336003713. The thread was not yet resolved at this point.
- 2026-10-21 11:02 UTC - Resolved the thread as `josecelano` after `github-review-threads reply-status` confirmed the reply (the mutation returned `isResolved: true`). A refreshed GraphQL fetch at 11:03 UTC showed the one thread resolved and none unresolved; the thread is now outdated, because the fix changed its lines.

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
