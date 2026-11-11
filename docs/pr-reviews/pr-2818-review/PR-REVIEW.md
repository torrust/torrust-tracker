---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2818 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2818.

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

Round 1: da2ce7's review `5856655819` (F1 to F3), processed after the replies were posted. The record is therefore late, which is the pattern this pull request describes; the replies were posted first because I read the validator's "before replying" line as requiring reply URLs that do not exist yet. The line now says to run the validator once the reply URLs are recorded. Round 2: Copilot's review `5856716279` (F4, F5), on the first commit, processed with the record already present. Round 3: da2ce7's review `5857046891`, whose reviewer IDs F4 to F7 collide with existing rows and are recorded as F6 to F9.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2818-f1 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2818-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2818-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2818-f4 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2818-f5 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2818-f6 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2818-f7 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2818-f8 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2818-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A late start is forbidden rather than handled

- PR number: 2818
- Source review ID: 5856655819
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536225666
- Concern: The preface said "Do not start it after a later round", which tells an agent that takes over after round 1, or one told the record is missing, not to create the record at all.
- Solution: Replaced the sentence with the remedy: if a reply was already posted without the record, create it now from the template and log the late start in the Processing Log.
- Current-tree verification: Read the new preface in process-pr-review/SKILL.md.
- Resolution reference: docs(skills): scope the audit record start to events that need action
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536269270

### F2 - A clean first event now creates a record whose push the loop does not settle

- PR number: 2818
- Source review ID: 5856655819
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536225718
- Concern: The record was created on the first review or comment whatever it carried, so a clean review, informational comment or ACK left an uncommitted record or an extra push that moves the head.
- Solution: Scoped the trigger in `process-pr-review` to the first review or comment that needs a reply or a fix; `wait-for-pr-review` points there. An earlier wording of the clean-event sentence in `wait-for-pr-review` was removed by a later commit.
- Current-tree verification: Read the process-pr-review preface ("that needs a reply or a fix").
- Resolution reference: docs(skills): scope the audit record start to events that need action
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536269640

### F3 - The description patterns cover four of the six PRs

- PR number: 2818
- Source review ID: 5856655819
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536225734
- Concern: The description said all six started the record after round 2 or 3, after a reviewer cited the skill, or from an earlier audit, which the audits and retrospectives of 2809 and 2810 do not show.
- Solution: Edited the PR description to say only what holds for all six: each audit record was first committed after the first replies.
- Current-tree verification: gh api repos/torrust/torrust-tracker/pulls/2818 shows the edited body.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536270103
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536270103

### F4 - The trigger terms review and comment are ambiguous and unformatted

- PR number: 2818
- Source review ID: 5856716279
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536274300
- Concern: The new preface used unformatted "review or comment", which could mean several kinds of comment.
- Solution: Used backticks and named the kinds that count: a submitted review, or a PR, review or inline comment.
- Current-tree verification: Read the preface in process-pr-review/SKILL.md.
- Resolution reference: docs(skills): point wait-for-pr-review to the audit start rule
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536321407

### F5 - The rule is stated in full in two skills

- PR number: 2818
- Source review ID: 5856716279
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536274496
- Concern: The same rule was spelled out in wait-for-pr-review and process-pr-review, which can drift.
- Solution: Shortened wait-for-pr-review to a pointer to the rule in process-pr-review.
- Current-tree verification: Read step 2 of the review loop in wait-for-pr-review/SKILL.md.
- Resolution reference: docs(skills): point wait-for-pr-review to the audit start rule
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536321795

### F6 - The retrospective says the new start rule cannot end validated

- PR number: 2818
- Source review ID: 5857046891
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536535928
- Concern: Root cause 3, the Findings introduction and the 09:41 entry said the rule cannot end validated, but the validator only fails rows whose replies do not exist; the conflict is the "before replying" line of the validation step.
- Solution: Said in the start rule that reply URLs are filled in after replying, scoped the validator step to recorded reply URLs, and corrected the introduction and retrospective.
- Current-tree verification: Read the preface and Validation Script paragraph in process-pr-review/SKILL.md.
- Resolution reference: docs(skills): fill in reply URLs after replying in the audit start rule
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536568853

### F7 - The F2 Solution cites a sentence that a later commit removed

- PR number: 2818
- Source review ID: 5857046891
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536535943
- Concern: The F2 Solution and verification described a sentence in wait-for-pr-review that a later commit removed.
- Solution: Restated the F2 Solution and verification as the trigger scope in process-pr-review.
- Current-tree verification: Read the F2 detail entry and the process-pr-review preface.
- Resolution reference: docs(skills): fill in reply URLs after replying in the audit start rule
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536569250

### F8 - The 09:45 entry stamps the second review arrival late

- PR number: 2818
- Source review ID: 5857046891
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536535955
- Concern: The arrival of review 5856716279 was stamped 09:45, the time of the replies.
- Solution: Added an arrival entry at 09:40 and kept 09:45 for the fix and replies.
- Current-tree verification: Read the Processing Log.
- Resolution reference: docs(skills): fill in reply URLs after replying in the audit start rule
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536569598

### F9 - The description omits the two new files

- PR number: 2818
- Source review ID: 5857046891
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536535973
- Concern: The description did not name the audit record and retrospective files the branch adds.
- Solution: Edited the PR description to name them.
- Current-tree verification: gh pr view 2818 shows the edited body.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536569994
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536569994

## Processing Log

- 2026-11-11 09:35 UTC - Review 5856655819 arrived with three findings.
- 2026-11-11 09:40 UTC - Replied to the three threads after pushing the fix and editing the description.
- 2026-11-11 09:41 UTC - Started audit late, after the replies, because the validator needs the reply URLs.
- 2026-11-11 09:45 UTC - Copilot review 5856716279 arrived with two findings; fixed, replied, and added F4 and F5.
- 2026-11-11 10:07 UTC - Review 5857046891 arrived with four findings.
- 2026-11-11 10:10 UTC - Fixed, replied, and added F6 to F9.

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
