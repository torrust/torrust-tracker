---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2867-udp-health-check-panics-on-invalid-response/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2879 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2879>.

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

F1 to F3 come from the first da2ce7 review (APPROVED, three Nits, none blocking), on the head ending in `docs(pr-reviews): [#2867] record the deferred second da2ce7 review of PR 2868 as follow-ups`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2879-f1 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2879-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2879-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Four timestamps are later than the commits that record them

- PR number: 2879
- Source review ID: 5918351027
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584471537>
- Concern: The ISSUE.md `last-updated-utc` and progress-log entry (09:55), the evidence file's `last-updated-utc` (09:50) and the PR 2868 audit's last processing-log entry (09:58) name times after the commits that carry them (09:44:30Z and 09:48:27Z).
- Solution: Set the progress-log entry and the evidence stamp to 09:44 and the PR 2868 audit entry to 09:48; added a progress-log entry that records the correction.
- Current-tree verification: `grep -n "09:55\|09:50\|09:58" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/*.md docs/pr-reviews/pr-2868-review/PR-REVIEW.md` prints nothing.
- Resolution reference: docs(issues): [#2867] correct four timestamps written ahead of their commits
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584500325>

### F2 - The T5 red log's test-file line does not match the committed test

- PR number: 2879
- Source review ID: 5918351027
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584471560>
- Concern: `evidence/t5-red-run.log` places the probe's panic at `contract.rs:663:34`, but the committed `panic!` is on line 666; the evidence row did not say which test-file state the red run used.
- Solution: The T5 row now says the red run happened before T3's assertion was tightened from five lines to eight, and names both lines.
- Current-tree verification: `grep -n "663" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/manual-verification-evidence.md` prints the T5 row.
- Resolution reference: docs(issues): [#2867] say which test-file state the T5 red run used
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584500687>

### F3 - The retrospective approval record states no original approval time

- PR number: 2879
- Source review ID: 5918351027
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584471565>
- Concern: The PR 2868 audit called its approval comment a retrospective record and dated the chat decision only as before the merge, which the process-pr-review skill does not allow here.
- Solution: The Ownership line no longer says retrospective; it states the decision window (07:35:29 to 08:03:34 UTC), the merge time and the comment's creation time.
- Current-tree verification: `grep -n "Post-merge workflow approval" docs/pr-reviews/pr-2868-review/PR-REVIEW.md` prints the line with the window.
- Resolution reference: docs(pr-reviews): [#2867] state when the maintainer approved deferring the PR 2868 findings
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584501060>

## Processing Log

- 2026-11-16 10:21 UTC - Started audit. da2ce7 reviewed the head ending in `docs(pr-reviews): [#2867] record the deferred second da2ce7 review of PR 2868 as follow-ups` at 10:18:14Z (APPROVED, three inline threads, all Nits, none blocking), recorded as F1 to F3. Fixed each in its own commit and replied on all three threads before the push.

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
