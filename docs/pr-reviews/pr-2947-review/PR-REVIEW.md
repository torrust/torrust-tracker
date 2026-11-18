---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2947 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2947.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2947-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The commit message and the description match the closing-keyword pattern

- PR number: 2947
- Source review ID: 5958207999
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2947#discussion_r4615240423
- Concern: "closed #2907" in the commit message of `chore(issues): archive closed issue #2907 spec to docs/issues/closed` and in the PR description matches the `open-pull-request` closing-keyword pattern. It cannot change #2907's state, which is already closed, so the reviewer rated it Minor and non-blocking.
- Solution: the PR description now says "#2907 is already closed on GitHub". The commit message stays as it is: rewording it needs a history rewrite and a force-push, which the repository's commit-history rules discourage, and the phrase reads truthfully because #2907 was closed by the merge of #2938 before this PR. The reviewer named leaving it on the record as an option.
- Current-tree verification: `gh-bot pr view 2947 --json body` returns a description with no closing-keyword match (`grep -E '(close[sd]?|fix(e[sd])?|resolve[sd]?) #[0-9]+'` prints nothing); `gh-bot issue view 2907 --json state` prints `CLOSED`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2947#discussion_r4615280787
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2947#discussion_r4615280787

## Processing Log

- 2026-11-18 14:09 UTC - da2ce7's review 5958207999 (`APPROVED`, on the head `chore(issues): archive closed issue #2907 spec to docs/issues/closed`) has one inline finding, F1. Copilot's review is an overview with no request. Reworded the PR description at 14:09:40 UTC; kept the commit message.
- 2026-11-18 14:10 UTC - Replied to F1 at 14:09:56 UTC; resolved the thread at 14:10 UTC; a GraphQL refresh showed 0 unresolved threads.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
