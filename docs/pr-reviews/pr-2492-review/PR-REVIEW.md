---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2492 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2492>.

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

Copilot review 5459352386 reported that its reviewer failed and contained no actionable assertion,
so it has no row.

Human review 5459928682 (da2ce7, round 1) numbered four inline findings F1-F4. Its body also
reported, without grading it as an inline finding, that the PR Validation section said "both
commits" while the branch had three. That independently actionable assertion is recorded as F5
with inferred Nit severity. The rest of the review body summarizes the four inline findings and
verified material, so it has no further row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2492-f1` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2492-f2` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2492-f3` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2492-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2492-f5` | Human | Nit (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The context attributes an unrecorded review and views to Cameron

- PR number: 2492
- Source review ID: 5459928682
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4221655314>
- Concern: The discussion said that Jose and Cameron reviewed the proposal and shared positions,
  but neither PR records a review, comment, or position by Cameron. The round rules do not allow
  one participant to write another participant's position.
- Solution: Attributed the recorded review and positions only to Jose, retained the invitation for
  Cameron to add his own round, and corrected the GitHub body of issue #2491 consistently.
- Current-tree verification: A search of the discussion and EPIC for `Both maintainers`,
  `maintainers reviewed`, `not hard to`, and the former Cameron attribution returns no matches.
  Issue #2491 says that Jose reviewed the proposal and supports the feature.
- Resolution reference: `docs(discussions): [#2491] correct I2P review attribution`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4221829734>

### F2 - The EPIC credits the review with a reviewer and finding the record lacks

- PR number: 2492
- Source review ID: 5459928682
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4221655336>
- Concern: The EPIC attributed the review to both maintainers and said the review found the
  implementation not hard, though the record supports neither claim.
- Solution: Named Jose as the reviewer, removed the unsupported difficulty claim, updated the
  progress log and timestamp, and applied the same correction to the GitHub issue body.
- Current-tree verification: The EPIC says that Jose reviewed the proposal in draft PR #2059 and
  that his review requires careful design. The unsupported phrases return no matches.
- Resolution reference: `docs(issues): [#2491] correct I2P EPIC review attribution`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4221830237>

### F3 - PR #2050 was written in July 2026, not August

- PR number: 2492
- Source review ID: 5459928682
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4221655344>
- Concern: The EPIC said that both PRs were written in August, but PR #2050 and its commits date
  from July.
- Solution: Changed the stale-proposal risk to say that the PRs were written in July and August
  2026.
- Current-tree verification: The EPIC's stale-proposal risk names both months.
- Resolution reference: `docs(issues): [#2491] correct I2P proposal dates`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4221830613>

### F4 - The discussion commit body omits its EPIC edit

- PR number: 2492
- Source review ID: 5459928682
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4221655353>
- Concern: The opening-discussion commit also changed the EPIC but its body described only the
  discussion and copied review documents. The reviewer requested that the body name the EPIC edit
  if the branch was rewritten for another reason.
- Solution: While rebasing onto the latest `develop`, reworded the commit body to say that it links
  the EPIC to the discussion and records the opening round in the EPIC progress log. No commit was
  squashed or removed.
- Current-tree verification: `git log torrust/develop..HEAD --format='%s%n%b'` shows the EPIC
  sentence in the opening-discussion commit body, and every branch commit is GPG signed.
- Resolution reference: `docs(discussions): [#2491] open the I2P peer support design discussion`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4221830993>

### F5 - The PR body says the pre-commit hook passed on both commits

- PR number: 2492
- Source review ID: 5459928682
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2492#pullrequestreview-5459928682>
- Concern: The PR Validation section said "both commits", but the PR already had three commits.
- Solution: Changed the PR body to say that the pre-commit hook passed on each commit, so the claim
  stays true as review-fix and audit commits are added.
- Current-tree verification: The current PR body says "Pre-commit hook passed on each commit".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2492#issuecomment-6064881832>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2492#issuecomment-6064881832>

## Processing Log

- 2026-10-08 16:45 UTC - Started audit from all GraphQL review threads and submitted reviews.
- 2026-10-08 17:05 UTC - Fixed F1-F5, validated the changed documents, rebased onto current
  `develop`, passed the pre-push checks, pushed the fixes, and replied to every inline thread.

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
