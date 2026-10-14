---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/git-workflow/open-pull-request/SKILL.md
    - AGENTS.md
---

<!-- skill-link: process-pr-review -->

# PR #2531 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2531>.

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

Copilot review 5520675793 (COMMENTED, submitted 2026-10-14 15:47:39 UTC, at the head that first
opened the pull request) left one inline finding headed `[Minor][F1]`; it keeps its ID. The review
overview repeats it and summarizes the changes, so it adds no row. Cameron (`da2ce7`) is a requested
reviewer and had not submitted a review when this audit was recorded.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2531-f1` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Say what replaces a rewritten commit id

- PR number: 2531
- Source review ID: 5520675793
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2531#discussion_r4273033723>
- Concern: "update every reference to the old commit ids" could be read as replacing them with the
  rewritten branch's new ids, while the `open-pull-request` skill requires durable references to
  cite PR-branch commits by Conventional Commit subject. The reviewer asked to state the required
  replacement directly and keep the audit mapping for review comments the author cannot edit.
- Solution: the rule now says to replace every old commit id in repository artifacts and the PR
  body with the commit's Conventional Commit subject, not its new id, linking the
  `open-pull-request` skill, and to map the ids cited in review comments, which cannot be edited,
  to commit subjects in the review audit.
- Current-tree verification: `AGENTS.md`, "Commit history and PR scope", contains "Replace every
  old commit id in repository artifacts and the PR body with the commit's Conventional Commit
  subject, not with its new id" and "Review comments cannot be edited, so map the ids they cite".
- Resolution reference: `docs(agents): replace rewritten commit ids with commit subjects`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2531#discussion_r4273102708>

## Processing Log

- 2026-10-14 15:49 UTC - Started audit: read Copilot review 5520675793 (one inline finding); Cameron
  has not submitted a review yet.
- 2026-10-14 15:51 UTC - Fixed F1 in its own commit.
- 2026-10-14 15:53 UTC - Pushed the fix (pre-push checks passed) and replied on the F1 thread; the
  thread is resolved after this audit is pushed.

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
