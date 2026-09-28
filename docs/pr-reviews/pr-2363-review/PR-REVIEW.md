---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2363 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2363>.

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

Audit IDs `F1`-`F4` are the reviewer's IDs from review 5337742420 (round 1, at the draft head
`docs(issues): [#2347] record the rebase and the upstream F3 fix`). No earlier finding exists, so
no ID collides. The review body lists the same four findings and contains no separate request.
F2, F3, and F4 were already fixed by commits pushed after that head.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2363-f1` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2363-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2363-f3` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2363-f4` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The F29 in-place correction adds a line to this file, so three existing #2320 audit entries now cite wrong line numbers

- PR number: 2363
- Source review ID: 5337742420
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121490638>
- Concern: The #2320 F29 correction grew the retrospective's Timeline note from two lines to
  three, shifting every later line by one. Records F23, F24, and F26 of the #2320 audit then cited
  wrong lines, and the #2320 F28 `NO_ACTION` premise ("nothing asserts false") stopped holding.
- Solution: `docs(pr-reviews): [#2347] keep the #2320 head-id note at two lines` re-wraps the
  corrected note to two lines with the same wording, drops the F28 row's line-shift sentence, and
  logs the re-wrap in the #2320 audit. This is the reviewer's cheapest fix.
- Current-tree verification: comparing lines 66 onward of `review-retrospective.md` with `develop`
  shows no difference. `git grep -nE 'round-4 body|Branch ids: F2, F9|once, the 20:12 push'` on
  the file returns lines 161, 429, 433, and 434. `validate-audit-record.py --pr-number 2320
  --base a110200d` exits `0` (29 rows).
- Resolution reference: `docs(pr-reviews): [#2347] keep the #2320 head-id note at two lines`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121982138>

### F2 - This bullet still says all 14 approved fixes are recorded as `FOLLOW_UP`/`OPEN`

- PR number: 2363
- Source review ID: 5337742420
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121490655>
- Concern: The spec's "Approved fixes stay `FOLLOW_UP` until merge" bullet ignored amendment 2:
  #2293 F3 is `FIXED`/`RESOLVED` at the reply step, and its thread does not end with three
  replies.
- Solution: `docs(issues): [#2347] correct stamps and stale claims found by the pre-PR review`
  rewrites the bullet. It names the 13 fixes that land here and the #2293 F3 exception, and limits
  the three-reply sentence to threads fixed here. The pre-PR task review raised the same point.
- Current-tree verification: the spec reads "In this PR, the 13 approved fixes that land here are
  therefore recorded as `FOLLOW_UP`/`OPEN`" and "#2293 F3 is the exception".
- Resolution reference: `docs(issues): [#2347] correct stamps and stale claims found by the pre-PR review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121982440>

### F3 - `last-updated-utc` is older than the file's last two edits

- PR number: 2363
- Source review ID: 5337742420
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121490677>
- Concern: `triage.md` read `last-updated-utc: "2026-09-27 19:05"` after two later edits.
- Solution: The same commit, `docs(issues): [#2347] correct stamps and stale claims found by the
  pre-PR review`, refreshes the stamp. The pre-PR task review raised the same point.
- Current-tree verification: `grep -n last-updated triage.md` prints
  `3:last-updated-utc: "2026-09-28 11:48"`.
- Resolution reference: `docs(issues): [#2347] correct stamps and stale claims found by the pre-PR review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121982667>

### F4 - The #2290 F5 disposition reply cites an audit commit that this head does not contain yet

- PR number: 2363
- Source review ID: 5337742420
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121490695>
- Concern: The #2290 F5 reply names `docs(pr-reviews): [#2347] add PR #2290 post-merge review
  audit`, which stays true only if T4 commits the audit under exactly that subject.
- Solution: T4 committed the #2290 audit under that exact subject.
- Current-tree verification: `git log --oneline --fixed-strings --grep='add PR #2290 post-merge
  review audit'` returns exactly one commit, and it adds
  `docs/pr-reviews/pr-2290-review/PR-REVIEW.md`.
- Resolution reference: `docs(pr-reviews): [#2347] add PR #2290 post-merge review audit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121982923>

## Processing Log

- 2026-09-28 12:03 UTC - Started processing round 1: review 5337742420 (da2ce7, `CHANGES_REQUESTED`, submitted 11:16 UTC at the draft head) with four inline findings. At the current head, F2, F3, and F4 were already fixed by commits pushed after that head; F1 was live (`git grep` returned 162, 430, 434, and 435).
- 2026-09-28 12:10 UTC - Committed the F1 fix, `docs(pr-reviews): [#2347] keep the #2320 head-id note at two lines` (authored 12:05:02Z), and pushed it; the pre-push hook passed and the push finished at 12:10.
- 2026-09-28 12:12 UTC - Posted a reply on all four threads (`created_at` 12:11:54Z-12:12:00Z), each with the fixing commit subject and the current-tree result.
- 2026-09-28 12:17 UTC - `github-review-threads reply-status` confirmed a reply on all four threads, and I resolved them with `resolve-all-unresolved-threads.sh`. A GraphQL refetch shows zero unresolved threads on PR #2363.

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
