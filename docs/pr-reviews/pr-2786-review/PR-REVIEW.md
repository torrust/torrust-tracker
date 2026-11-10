---
pr-number: 2786
pr-url: https://github.com/torrust/torrust-tracker/pull/2786
last-updated-utc: "2026-11-10 08:46"
---

# PR #2786 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2786.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Round 1: Copilot review 5831908204 on the head then under review: no inline comments and no open issues (the review body only repeats that its full agentic review timed out), so it has no finding rows. da2ce7 review 5838199443 (CHANGES_REQUESTED) on the same head: six findings, all valid; F1 blocked and is fixed, F5 needs no change. Thread state is OPEN for the four inline threads: the auto mode classifier denies the bot resolveReviewThread mutation. F5 and F6 have no thread; a conversation comment answers them.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2786-f1 | Human | Major | link-integrity | ORIGINAL | FIXED | OPEN |
| F2 | review-finding:pr-2786-f2 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F3 | review-finding:pr-2786-f3 | Human | Nit | metadata | ORIGINAL | FIXED | OPEN |
| F4 | review-finding:pr-2786-f4 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F5 | review-finding:pr-2786-f5 | Human | Nit | other | ORIGINAL | NO_ACTION | NON_RESOLVABLE |
| F6 | review-finding:pr-2786-f6 | Human | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The evidence issue-spec frontmatter still names the old open path.

- PR number: 2786
- Source review ID: 5838199443
- Reviewer finding ID: F1 (Major, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2786#discussion_r4522202173
- Concern: The evidence issue-spec frontmatter still names the old open path.
- Solution: Valid. The field names the closed folder.
- Current-tree verification: sed -n 3p of the evidence file shows the closed path.
- Resolution reference: docs(issues): [#2757] point the evidence issue-spec at the archived folder
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2786#discussion_r4522800714

### F2 - The EPIC log lists three of the five close-out steps.

- PR number: 2786
- Source review ID: 5838199443
- Reviewer finding ID: F2 (Nit, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2786#discussion_r4522202180
- Concern: The EPIC log lists three of the five close-out steps.
- Solution: Valid. The log names all five.
- Current-tree verification: The EPIC progress log entry of 2026-11-09 19:25 lists five steps.
- Resolution reference: docs(issues): [#2757] list all five close-out steps in the EPIC log
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2786#discussion_r4522801270

### F3 - The inventory last-updated-utc is not bumped.

- PR number: 2786
- Source review ID: 5838199443
- Reviewer finding ID: F3 (Nit, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2786#discussion_r4522202186
- Concern: The inventory last-updated-utc is not bumped.
- Solution: Valid. It is 2026-11-10.
- Current-tree verification: sed -n 5p of the inventory.
- Resolution reference: docs(issues): [#2757] bump the inventory last-updated date
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2786#discussion_r4522801775

### F4 - The spec log cites F15 of the third review round, which does not resolve.

- PR number: 2786
- Source review ID: 5838199443
- Reviewer finding ID: F4 (Nit, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2786#discussion_r4522202193
- Concern: The spec log cites F15 of the third review round, which does not resolve.
- Solution: Valid. It names note F15 of the approval review 5830184353, which is not an audit row.
- Current-tree verification: The spec progress log entry of 2026-11-09 19:25.
- Resolution reference: docs(issues): [#2757] name the review note that the archived spec cites
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2786#discussion_r4522802245

### F5 - The title names the already closed issue.

- PR number: 2786
- Source review ID: 5838199443
- Reviewer finding ID: F5 (Nit, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2786#pullrequestreview-5838199443
- Concern: The title names the already closed issue.
- Solution: No change: rows 4 to 6 archive titles do the same and the title names the issue whose spec moves; the maintainer may reword it at merge.
- Current-tree verification: gh pr view 2786 --json title.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2786#issuecomment-6763008503
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2786#issuecomment-6763008503

### F6 - The description has no validation line.

- PR number: 2786
- Source review ID: 5838199443
- Reviewer finding ID: F6 (Nit, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2786#pullrequestreview-5838199443
- Concern: The description has no validation line.
- Solution: Valid. The PR description now has a Validation line.
- Current-tree verification: gh pr view 2786 --json body shows the Validation bullet.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2786#issuecomment-6763008503
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2786#issuecomment-6763008503

## Processing Log

- 2026-11-10 08:46 UTC - Started audit; round 1 (Copilot review 5831908204 without findings, da2ce7 review 5838199443 with six findings) processed: F1 to F4 and F6 fixed, F5 declined with the reason, all four threads replied, one conversation comment for F5 and F6. Resolving the threads is blocked by the classifier. Each fix is its own commit; one push.

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
