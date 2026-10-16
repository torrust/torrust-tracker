---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2547-measure-effect-of-optional-rebases/ISSUE.md
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2550 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2550>.

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

Two reviews were submitted. Copilot's review 5544328074 carries two inline threads, F1 and F2; its
overview repeats them, so the body adds no rows. da2ce7's approving review 5544619071 (later dismissed
at 08:31:50 UTC by the push of "docs(issues): [#2507] record the Task Reviewer's re-review of the
close-out fixes") carries seven
inline threads, whose reviewer IDs are F1-F7. Reviewer IDs F3-F7 do not collide with the audit's IDs
and are kept. Reviewer IDs F1 and F2 collide with Copilot's F1 and F2, so they are recorded as F8 and
F9 with the reviewer's ID kept in the detail entry. They are also the same concerns as Copilot's F2
and F1, so F8 is recorded as `RE_RAISE_OF:F2` and F9 as `RE_RAISE_OF:F1`. Its body is a round summary
that repeats those seven findings and confirms the rest of the close-out, so it adds no rows. Both
reviews read the head whose subject is "docs(issues): [#2525] record the agreed delivery sequence".

A second round followed the push of the audit. Copilot's review 5545295102 recommends approval with
no open findings and adds no row. da2ce7's approving review 5545292694 read the head whose subject is
"docs(pr-reviews): add the PR #2550 review audit" and carries five inline threads whose reviewer IDs
F1-F5 collide with F1-F5 above, so they are recorded as F10-F14 with the reviewer's ID kept in the
detail entry. Its body is a round summary that repeats those five findings and confirms the earlier
ones as resolved, so it adds no rows; a separate PR conversation comment, `ACK`, records its verdict
for that head.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2550-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2550-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2550-f3 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2550-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2550-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2550-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2550-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2550-f8 | Human | Minor | correctness | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F9 | review-finding:pr-2550-f9 | Human | Minor | correctness | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F10 | review-finding:pr-2550-f10 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2550-f11 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2550-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2550-f13 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2550-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The rebase-fallout baseline is not restricted to 1 to 13 October

- PR number: 2550
- Source review ID: 5544328074
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4291937145>
- Concern: section 7 of the baseline lists commits "since September", unlike the October force-push
  and CI figures, so normalizing the whole list by 13 days or 67 merges would inflate the baseline.
- Solution: the specification now requires recounting the metric for 1 to 13 October with the same
  subject reading, in the scope bullet, in T2, and in AC1. The GitHub issue body was updated to match.
- Current-tree verification: of the 23 commits in section 7, 13 have author dates from 1 to 13
  October (counted with `git log -1 --format=%ad` for each). The `ISSUE.md` scope bullet, T2 row, and
  AC1 state the recount.
- Resolution reference: docs(issues): [#2547] limit the rebase-fallout baseline to 1 to 13 October
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292507751>

### F2 - The refresh-path cutoff should be the pre-merge target tip

- PR number: 2550
- Source review ID: 5544328074
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4291937202>
- Concern: the specification counted merges without a head push after `develop` reached the merge
  base, but V1 uses `M^1`. A head is always pushed after its merge base, so the rule excluded
  examples such as #2528.
- Solution: the scope bullet names `M^1` as the cutoff, adds "not pushed again before the merge", and
  keeps V1's note that the count is inferred. The GitHub issue body was updated to match.
- Current-tree verification: the `ISSUE.md` scope bullet names `M^1`. #2528's head was first pushed
  at 14:58:41 UTC, after its merge base and before `M^1`, per V1.
- Resolution reference: docs(issues): [#2547] use the merge's target tip as the refresh-path cutoff
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292508046>

### F3 - The two-way classification assumes every force-push is a rebase

- PR number: 2550
- Source review ID: 5544619071
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292166899>
- Concern: since PR #2531 merged, `AGENTS.md` allows rewrites of an unmerged pull request's history
  that are not rebases, while baseline section 5 counts every force-push as a rebase. A split or
  reorder would be counted as recommended and inflate the figure.
- Solution: added a third class, not a rebase (the new head is not based on the `develop` tip of its
  time), report both the total and the rebase-only figures, and list the policy change among the
  confounders in Risks. V4 now shows the method on all 11 force-pushes of the 18 merges.
- Current-tree verification: the `ISSUE.md` scope, AC2, and Risks name the third class. V4 lists
  #2524 at 10:28:49 as not a rebase, recomputed by the Task Reviewer.
- Resolution reference: docs(issues): [#2547] classify force-pushes that are not rebases separately
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292508773>

### F4 - The log says the Task Reviewer validated the close-out, but its verdict was a failure

- PR number: 2550
- Source review ID: 5544619071
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292166911>
- Concern: the only recorded verdict was `REVIEW FAILED`, no re-review followed, and the fixes were
  written inside the reviewer's own entry, where the template asks for a correction as a new entry.
- Solution: restored the original entry, recorded a re-review as new entries, and worded the log
  after the verdicts. The first re-review failed (residual "proven" wording for M1, an incomplete V4
  account, and the #2531 merge time), which led to three more commits; a second re-review passed.
- Current-tree verification: `agent-review-reports.md` has the 06:30 entry with its original
  follow-up action, the 08:15 `REVIEW FAILED` entry, and the 08:30 `REVIEW PASSED` entry. The #2507
  `ISSUE.md` 06:40 entry says `REVIEW FAILED`.
- Resolution reference: docs(issues): [#2507] record the Task Reviewer's re-review of the close-out fixes
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292508963>

### F5 - The three force-push ids are on no `develop` commit or tag

- PR number: 2550
- Source review ID: 5544619071
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292166915>
- Concern: the ids in the V4 table are on no `develop` commit or tag, against the `open-pull-request`
  commit-id check and the evidence template, and the commands record no step that obtains them.
- Solution: took the first option. The V4 columns are labelled `beforeCommit` and `afterCommit`, the
  section says these heads are on no `develop` commit and must be fetched, the commands gain
  `git fetch torrust <oid>`, and the full `oid` values are recorded. No exception was requested in
  the skill; the section says the check still reports them.
- Current-tree verification: the V4 section has the labelled columns, the fetch step, and the four
  full `oid` values. `git fetch torrust 55a414e98a1df60999ae5aa9689b6662ee168c5a` succeeds.
- Resolution reference: docs(issues): [#2507] label the force-push event commits and record how to fetch them
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292509191>

### F6 - The last-updated stamp is newer than the newest progress-log entry

- PR number: 2550
- Source review ID: 5544619071
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292166922>
- Concern: `last-updated-utc` was 06:40, but the newest progress-log entry of the #2547 specification
  was 06:12, because the reviewer-finding fix had no log line.
- Solution: added the missing 06:40 entry and one for this review round, and set the stamp to the
  newest entry.
- Current-tree verification: the #2547 `ISSUE.md` frontmatter says `2026-10-16 08:25` and its last
  progress-log entry has the same time.
- Resolution reference: docs(issues): [#2547] log the edits that moved the last-updated stamp
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292509410>

### F7 - The delivery-sequence log line is misplaced and tagged C3

- PR number: 2550
- Source review ID: 5544619071
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292166925>
- Concern: the 2026-10-16 06:12 entry placed the #2525 delivery sequence at 06:12 and tagged it C3,
  which is the move of T7 and AC9, while "that specification" read as the measurement specification.
- Solution: the sentence now points at the #2525 specification and its own log entry of 2026-10-15
  17:20 UTC, and C3 stays on the move of T7 and AC9 only.
- Current-tree verification: the 06:12 entry in the #2507 `ISSUE.md` names
  `docs/issues/open/2525-ci-gate-required-check/ISSUE.md`.
- Resolution reference: docs(issues): [#2507] attribute the delivery-sequence log line to the #2525 spec
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292509646>

### F8 - The refresh-path count uses the merge base as its cutoff, where V1 uses M^1

- PR number: 2550
- Source review ID: 5544619071
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292166891>
- Concern: the same as F2: read literally, the cutoff admits no pull request, and V1's #2528 case
  shows the gap. The created issue's body carries the same sentence.
- Solution: fixed together with F2, in the specification and in the GitHub issue body.
- Current-tree verification: the `ISSUE.md` scope bullet names `M^1`; the #2547 issue body carries
  the same sentence.
- Resolution reference: docs(issues): [#2547] use the merge's target tip as the refresh-path cutoff
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292508322>

### F9 - The rebase-fallout baseline is not a 1-to-13-October count

- PR number: 2550
- Source review ID: 5544619071
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292166893>
- Concern: the same as F1: the per-day normalization has nothing comparable, because section 7 lists
  23 commits since September, 13 of which fall on 1 to 13 October.
- Solution: fixed together with F1: the baseline is recounted for 1 to 13 October, and T2 says so.
- Current-tree verification: the `ISSUE.md` scope bullet, T2 row, and AC1 state the recount. The
  count of 13 of 23 was reproduced.
- Resolution reference: docs(issues): [#2547] limit the rebase-fallout baseline to 1 to 13 October
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292508565>

### F10 - The three classes cannot tell a rebase from a rewrite

- PR number: 2550
- Source review ID: 5545292694
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292678053>
- Concern: the classes test only whether the new head is based on the `develop` tip of its time. A
  rebase onto a tip the author had not yet fetched lands in "not a rebase" (#2524 at 10:28:49, 4 min
  31 s after its tip was committed), and a rewrite of a head already on the tip lands in
  "recommended". Classify by whether the push moved the base.
- Solution: the specification and V4 compare `git merge-base <tip> <beforeCommit>` with
  `git merge-base <tip> <afterCommit>`: a moved base is a rebase, required when the before commit
  conflicts with the tip and recommended otherwise, and an unchanged base is not a rebase. The tip is
  the last first-parent `develop` commit whose pull request `merged_at` precedes the push. V4 applies
  the method to all 11 force-pushes; the classes are unchanged (two required, eight recommended, one
  not a rebase). The GitHub issue body was updated to match.
- Current-tree verification: the `ISSUE.md` scope bullet and the V4 tables state the method and show
  both bases. For each of the 11 pushes the `merged_at` of the tip's pull request is before the push,
  and #2538's push moved the base from `48c4d3013` to `9c8bcdf8a`.
- Resolution reference: docs(issues): [#2547] classify a force-push by whether it moved the base
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4293588299>

### F11 - The evidence stamp is older than two later commits

- PR number: 2550
- Source review ID: 5545292694
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292678062>
- Concern: `last-updated-utc` of `manual-verification-evidence.md` was 08:17, but two later commits
  changed the file (08:24:31 and 08:25:04 UTC). Set it to 08:25.
- Solution: set the stamp to 08:25 as asked. The V4 rewrite of this round then moved it to the time
  of that edit, 10:22 UTC.
- Current-tree verification: the evidence frontmatter says `2026-10-16 10:22`, the minute of the last
  commit that changes the file.
- Resolution reference: docs(issues): [#2507] set the evidence stamp to its newest edit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4293588540>

### F12 - The second re-review's input omits that one fix was committed with its entry

- PR number: 2550
- Source review ID: 5545292694
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292678068>
- Concern: the second re-review lists "the committed branch" as its input, but the reworded 06:40
  log line was committed with that entry.
- Solution: neither offered wording is true, because that line was excluded from the re-review's
  scope on purpose and no reviewer checked it. A correction note, appended as a new bullet and not as
  an edit of the entry, says that the 08:30 verdict does not cover that one-sentence change.
- Current-tree verification: the Correction notes section of `agent-review-reports.md` has the new
  bullet, and the 08:30 entry is unchanged.
- Resolution reference: docs(issues): [#2507] state what the second re-review did not check
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4293588749>

### F13 - The #2531 merge time is unlabelled where the evidence uses committer time

- PR number: 2550
- Source review ID: 5545292694
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292678076>
- Concern: the specification gives the #2531 merge as 16:09 UTC without saying it is the reported
  time, while the evidence dates merges by Git committer time (16:06:29 UTC for the same merge).
- Solution: both places in the specification say `merged_at`, and its progress log explains the
  three-minute gap. The GitHub issue body was updated to match.
- Current-tree verification: `grep -n merged_at` on the `ISSUE.md` finds the scope bullet and the
  Risks entry.
- Resolution reference: docs(issues): [#2547] label the PR #2531 merge time as merged_at
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4293589021>

### F14 - The audit does not say the first approval was dismissed

- PR number: 2550
- Source review ID: 5545292694
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4292678085>
- Concern: the record calls review 5544619071 "approving" without saying that the author dismissed
  it at 08:31:50 UTC on the push of the re-review commit, an event of this processing round.
- Solution: the Findings paragraph says the approval was later dismissed and names the push, and the
  Processing Log records the dismissal. The log is append-only and chronological, so the entry
  carries the time it was recorded and names the event time. The record also gains rows F10 to F14
  and the second round's entries.
- Current-tree verification: the audit has both additions. The timeline holds one `review_dismissed`
  event, actor `josecelano`, at 2026-10-16T08:31:50Z.
- Resolution reference: docs(pr-reviews): record the dismissed approval and the second review round of PR #2550
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4293589228>

## Processing Log

- 2026-10-16 08:15 UTC - Fetched every thread and review, classified and numbered the findings, and
  fixed F1 and F9, F2 and F8, and F3 in separate signed commits.
- 2026-10-16 08:30 UTC - Fixed F5, F7, F6, and F4 in separate signed commits. The Task Reviewer's
  re-review for F4 failed once and led to three more signed commits (M1 wording, V4 accounting, and
  the #2531 merge time) before it passed.
- 2026-10-16 08:33 UTC - Pushed, updated the GitHub issue body of #2547, re-derived each reply claim
  against the current tree, and replied on all nine threads.
- 2026-10-16 08:35 UTC - Started this record, with a reply URL in every row.
- 2026-10-16 08:38 UTC - Pushed the record, resolved all nine threads, and re-requested da2ce7 and
  Copilot.
- 2026-10-16 08:51 UTC - da2ce7's approving review 5545292694 and an `ACK` comment arrived for the
  record's head, with five new threads, and Copilot's review 5545295102 recommended approval with no
  open findings.
- 2026-10-16 10:28 UTC - Recorded the dismissal of review 5544619071's approval, which happened at
  08:31:50 UTC on the push of the re-review commit (F14). Fixed F10, F11, F12, F13 and F14 in separate
  signed commits, pushed, and replied on the five threads, then added F10-F14 to this record.

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
