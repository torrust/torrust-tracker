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

Human review 5460380310 (da2ce7, round 2) confirmed F1-F5 and continued its own series. Its body
reported F6, a finding that was never posted inline: the 17:05 log stamp was later than the commit
that carried it. The review records F6 as already fixed by the appended correction, so F6 is
recorded with inferred Minor severity. Its inline F7 and F8 keep their IDs. The body restates F7
and F8 and verifies the carry-over, so it has no further row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2492-f1` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2492-f2` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2492-f3` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2492-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2492-f5` | Human | Nit (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F6 | `review-finding:pr-2492-f6` | Human | Minor (inferred) | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |
| F7 | `review-finding:pr-2492-f7` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2492-f8` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |

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

### F6 - The 17:05 log stamp is later than the commit that carries it

- PR number: 2492
- Source review ID: 5460380310
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2492#pullrequestreview-5460380310>
- Concern: The second Processing Log entry was stamped 17:05 UTC, but the commit that added it was
  made at 16:57:43Z. Cameron found this in round 2 and did not post it inline.
- Solution: Already fixed before the review by an appended correction entry. Following the
  append-only rule, the 17:05 entry was left in place. F8 later corrected that entry's explanation.
- Current-tree verification: The log keeps the 17:05 entry, followed by the 17:06 correction and
  the 17:19 correction of its explanation. Review 5460380310 records F6 as fixed.
- Resolution reference: `docs(pr-review): [#2492] record review completion`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2492#issuecomment-6065313510>

### F7 - The PR body still attributes support for the feature to both maintainers

- PR number: 2492
- Source review ID: 5460380310
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4222035742>
- Concern: The PR body Summary said "The maintainers support I2P peer support", the attribution F1
  removed from the README and the issue. The merge tool copies the body into the merge commit.
- Solution: Edited the PR body to say "Jose Celano supports I2P peer support, but it needs a
  careful design…", matching the issue #2491 body and the README. No repository file changed.
- Current-tree verification: `gh pr view 2492 --json body` shows the new sentence, and "The
  maintainers support" no longer appears in it.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2492#issuecomment-6065313510>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4222080797>

### F8 - The correction entry's explanation does not hold

- PR number: 2492
- Source review ID: 5460380310
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4222035727>
- Concern: The 17:06 correction said that the 17:05 stamp should be 16:55, "based on the signed
  fix-commit times", and that it was "recorded one hour late". The gap is ten minutes, and 16:55 is
  the force-push time.
- Solution: Appended a 17:19 UTC correction. It says the stamp was ten minutes late, that 16:55 UTC
  is the force-push time (16:55:01Z), that the fix commits were committed at 16:52:49Z-16:52:50Z,
  and that the replies were posted at 16:56:35Z-16:56:43Z. The earlier entries are unchanged.
- Current-tree verification: The PR timeline shows `head_ref_force_pushed` at 16:55:01Z, and
  `git log` shows the commit times. The 17:19 entry predates its commit (17:20:02Z).
- Resolution reference: `docs(pr-review): [#2492] correct the review-log timestamp explanation`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2492#discussion_r4222081096>

## Processing Log

- 2026-10-08 16:45 UTC - Started audit from all GraphQL review threads and submitted reviews.
- 2026-10-08 17:05 UTC - Fixed F1-F5, validated the changed documents, rebased onto current
  `develop`, passed the pre-push checks, pushed the fixes, and replied to every inline thread.
- 2026-10-08 17:06 UTC - Correction: the preceding entry's timestamp should be
  2026-10-08 16:55 UTC, based on the signed fix-commit times; it was recorded one hour late.
  Refreshed GraphQL after the replies, confirmed that each unresolved thread had a reply, resolved
  all four inline threads, fetched again, and confirmed zero unresolved threads. The audit
  validator reports five rows, three log entries, and zero failures.
- 2026-10-08 17:19 UTC - Correction to the 17:06 entry's explanation: the 17:05 stamp was ten
  minutes late, not one hour. 16:55 UTC is the force-push time (16:55:01Z), not a fix-commit
  time; the fix commits were committed at 16:52:49Z-16:52:50Z, and the replies the 17:05 entry
  records were posted at 16:56:35Z-16:56:43Z.
- 2026-10-08 17:23 UTC - Processed round 2 (review 5460380310): recorded F6-F8, edited the PR
  body for F7, replied to the F7 and F8 threads, posted the consolidated response, confirmed both
  threads had replies, resolved them, and fetched again: zero unresolved threads.

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
