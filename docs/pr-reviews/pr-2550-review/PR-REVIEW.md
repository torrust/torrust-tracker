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
no open findings and adds no row. da2ce7's approving review 5545292694 (later dismissed at 10:26:39 UTC
by the push of "docs(issues): [#2507] log the second PR #2550 review round") read the head whose
subject is "docs(pr-reviews): add the PR #2550 review audit" and carries five inline threads whose
reviewer IDs F1-F5 collide with F1-F5 above, so they are recorded as F10-F14 with the reviewer's ID
kept in the detail entry. Its body is a round summary that repeats those five findings and confirms
the round-1 findings as resolved except two gaps it names (the class test and the re-review's stated
input), which are raised as F10 and F12, so it adds no rows; a separate PR conversation comment,
`ACK`, records its verdict for that head.

A third round followed the push of the second round's fixes. Copilot's review 5546689152 carries one
finding in its body and no inline thread, so it is recorded as F15 with `NON_RESOLVABLE` and the
review URL as its source. da2ce7's approving review 5547008497 (later dismissed at 11:22:22 UTC by the
push of "docs(issues): [#2507] log the third PR #2550 review round") read the head whose subject is
"docs(pr-reviews): record the dismissed approval and the second review round of PR #2550" and carries
four inline threads whose reviewer IDs F1-F4 collide with F1-F4 above, so they are recorded as F16-F19
with the reviewer's ID kept in the detail entry. Its F1 is the same concern as F15, so F16 is recorded
as `RE_RAISE_OF:F15`. Its body is a round summary that repeats those four findings and confirms three
of the earlier ones as resolved and finds two partly resolved (its R3-F3 and R3-F5), which are raised
as F18 and F17, so it adds no rows; an `ACK` comment records its verdict for that head.

A fourth round followed the push of the third round's fixes. Copilot's review 5547291916 carries one
inline thread, recorded as F20. da2ce7's approving review 5547344550 read the head whose subject is
"docs(pr-reviews): add the reply URLs of the third review round of PR #2550" and carries three inline
threads whose reviewer IDs F1-F3 collide with F1-F3 above, so they are recorded as F21-F23 with the
reviewer's ID kept in the detail entry. Its F3 is the same concern as F20, so F23 is recorded as
`RE_RAISE_OF:F20`. Its body is a round summary that repeats those three findings and confirms the
earlier ones as resolved, so it adds no rows; an `ACK` comment records its verdict for that head.

A fifth round followed the push of the fourth round's fixes. da2ce7's approving review 5547591114
read the head whose subject is "docs(pr-reviews): add the reply URLs of the fourth review round of
PR #2550" and carries three inline threads whose reviewer IDs F1-F3 collide with F1-F3 above, so they
are recorded as F24-F26 with the reviewer's ID kept in the detail entry. Its body is a round summary
that repeats those three findings, marks one earlier finding partly resolved (its R5-F1, raised as
F24), and confirms the rest, so it adds no rows; an `ACK` comment records its verdict for that head.

Every push dismisses the standing approval (the pull-request ruleset dismisses stale reviews on
push), always with the author's account as actor and no message. This record names the dismissals of
the first three approvals, at 08:31:50, 10:26:39, and 11:22:22 UTC, and the one that the push of the
fourth round's fixes caused, at 11:56:26 UTC (review 5547344550). Later dismissals follow the same
pattern and are not logged one by one, because each `ACK` comment names the head it applies to and
the next review supersedes it.

A sixth round followed the push of the fifth round's fixes. Copilot's review 5547644540, posted on the
head of the fourth round's last commit, recommends approval with no open findings and adds no row.
da2ce7's approving review 5547815458 read the head whose subject is "docs(pr-reviews): add the reply
URLs of the fifth review round of PR #2550" and carries one inline thread whose reviewer ID F1
collides with F1 above, so it is recorded as F27 with the reviewer's ID kept in the detail entry. Its
body is a round summary that repeats that finding and confirms the rest, so it adds no rows; an `ACK`
comment records its verdict for that head.

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
| F15 | review-finding:pr-2550-f15 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |
| F16 | review-finding:pr-2550-f16 | Human | Minor | correctness | RE_RAISE_OF:F15 | FIXED | RESOLVED |
| F17 | review-finding:pr-2550-f17 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2550-f18 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2550-f19 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2550-f20 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2550-f21 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2550-f22 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2550-f23 | Human | Nit | correctness | RE_RAISE_OF:F20 | FIXED | RESOLVED |
| F24 | review-finding:pr-2550-f24 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2550-f25 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2550-f26 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2550-f27 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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
- Solution: neither offered wording is true, because that line was one of the four actions the entry
  lists but was excluded by the invocation, so no reviewer checked it. A correction note, appended as
  a new bullet and not as an edit of the entry, says that the 08:30 verdict does not cover that
  one-sentence change. The note was later dated and made to name the contradiction with the entry's
  scope sentence (F18).
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
- Current-tree verification: the audit has both additions. When this entry was written the timeline
  held two `review_dismissed` events, both with actor `josecelano` and no message: 2026-10-16T08:31:50Z
  for review 5544619071 and 2026-10-16T10:26:39Z for review 5545292694. This entry first said that it
  held one; F17 corrects that and records the second. The reply this row cites is the second one on
  the thread, posted from the pushed head (F19).
- Resolution reference: docs(pr-reviews): record the dismissed approval and the second review round of PR #2550
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294126277>

### F15 - The force-push total and its classification use different populations

- PR number: 2550
- Source review ID: 5546689152
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#pullrequestreview-5546689152>
- Concern: baseline section 5 selects pull requests by merge date and counts their whole force-push
  timelines, but the #2547 scope bullet and AC2 select pushes by event date. #2537 merged on 15
  October but its required rebase was on 14 October, so it contributes to the baseline-style total
  and not to the classification. Use one population and align AC2 and the issue body.
- Solution: the population of every force-push measure is the pull requests merged in the window
  (`merged_at`), with all of their force-pushes, including events before the window. The scope, AC2,
  and the GitHub issue body say so. The finding has no inline thread, so its state is
  `NON_RESOLVABLE`. This entry first said "every measure", which overstated the fix; F20 and F23
  limit it to the force-push measures.
- Current-tree verification: the `ISSUE.md` scope bullet names the population, its classification
  bullet says "every force-push of those pull requests", and AC2 says "including events before it".
  V4 of the evidence already uses this population: the 18 merges from #2526 to #2543 with all of their
  force-pushes, including #2537's rebase on 14 October.
- Resolution reference: docs(issues): [#2547] count and classify the force-pushes of the PRs merged in the window
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#issuecomment-6214418071>

### F16 - The total and the classification count different force-pushes

- PR number: 2550
- Source review ID: 5547008497
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294046063>
- Concern: the same as F15: classifying the pushes dated in the window mixes populations in the
  rebase-only figure, since a pull request merged in the window with an earlier push is in the total
  but not classified (#2537), and a push in the window on a pull request merged later is classified
  but not counted.
- Solution: fixed together with F15, in the specification, AC2, and the GitHub issue body.
- Current-tree verification: the same as F15.
- Resolution reference: docs(issues): [#2547] count and classify the force-pushes of the PRs merged in the window
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294125384>

### F17 - The second approval was dismissed too, and the record says one event

- PR number: 2550
- Source review ID: 5547008497
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294046070>
- Concern: the record calls review 5545292694 "approving", but that approval was dismissed at 10:26:39
  UTC by the push of the "log the second PR #2550 review round" commit, before the commit that wrote
  F14's verification, which says the timeline holds one `review_dismissed` event when it held two.
- Solution: the Findings paragraph says the approval was later dismissed and names the push, F14's
  verification states both events and says it first said one, and the Processing Log records the
  second dismissal.
- Current-tree verification: when this entry was written the timeline held two `review_dismissed`
  events, both with actor `josecelano`: 2026-10-16T08:31:50Z (review 5544619071) and
  2026-10-16T10:26:39Z (review 5545292694). A third came with the next push, at 2026-10-16T11:22:22Z
  for review 5547008497, and is recorded by F21. The audit names all three.
- Resolution reference: docs(pr-reviews): record the second dismissal and the third review round of PR #2550
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294125635>

### F18 - The correction note contradicts the entry it corrects

- PR number: 2550
- Source review ID: 5547008497
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294046080>
- Concern: the note says the reworded 06:40 log line was "excluded from that review's scope on
  purpose", but the 08:30 entry gives its scope as the four follow-up actions of the 08:15 entry, the
  log line being the fourth. The note is an undated bullet and F12 repeats the claim.
- Solution: the note is dated, names the contradiction (the entry lists four actions and checked
  three), and says that the entry's scope sentence is wrong about the fourth. F12's text now says the
  line was excluded by the invocation, as the entry's list includes it.
- Current-tree verification: the Correction notes section of `agent-review-reports.md` has the dated
  note with the contradiction, and the 08:30 entry is unchanged.
- Resolution reference: docs(issues): [#2507] name the contradiction in the second re-review's scope
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294125860>

### F19 - The F14 reply preceded its fix

- PR number: 2550
- Source review ID: 5547008497
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294046090>
- Concern: the F14 reply (10:27:03 UTC) says "FIXED. The Findings paragraph now says" the dismissal,
  but the commit that says it is dated 10:28:50 UTC, and the record's Completion Rules ask to
  re-derive a reply claim against the current tree before replying.
- Solution: the reply was posted before its fix was committed. This entry records the order, and a
  second reply on the F14 thread, written from the pushed head after this round's fixes, is the one
  F14 cites. The first reply remains on the thread.
- Current-tree verification: the first F14 reply is
  <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4293589228> (10:27:03 UTC), and
  the commit with the Findings paragraph is dated 10:28:50 UTC. For this round, each reply was posted
  after the commit it cites was pushed.
- Resolution reference: docs(pr-reviews): record the second dismissal and the third review round of PR #2550
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294126082>

### F20 - The CI measurement population should keep the baseline's selection

- PR number: 2550
- Source review ID: 5547291916
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294275456>
- Concern: baseline section 6 estimates the `pull_request` workflow runs of 1 to 13 October, not the
  lifetime runs of the pull requests merged then. A population rule that applies to every measure
  would exclude CI for still-open pull requests and include pre-window runs, so the comparison with
  197.8 runner-hours would be inconsistent. Limit the merged-pull-request rule to the force-push
  figures, select CI runs by creation date within the window, and align the progress entry and the
  GitHub issue body.
- Solution: the shared population applies only to the force-push total and its classification. The
  scope bullet states how each other measure selects its data: CI runs by creation date and fallout
  commits by author date, and volume as in baseline section 4. The progress entry says "every
  force-push measure", and the issue body was updated to match. This entry first attributed the CI
  and fallout rules to baseline sections 6 and 7, which do not state them; F26 corrects that.
- Current-tree verification: the `ISSUE.md` scope bullet lists the selection of each measure and its
  11:16 progress entry says "every force-push measure". Baseline section 6 reports 899 runs for 1 to
  13 October and names no date field, so the creation-date rule is the specification's own.
- Resolution reference: docs(issues): [#2547] limit the shared population to the force-push measures
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294393917>

### F21 - The third approval was dismissed too, and the record does not say so

- PR number: 2550
- Source review ID: 5547344550
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294315577>
- Concern: review 5547008497 is called "approving", but its approval was dismissed at 11:22:22 UTC by
  the push of the "log the third PR #2550 review round" commit, before this record's last commit. F17's
  verification still says the timeline holds two events, and the 11:24 entry, which records that push,
  does not name the dismissal. This is F17's gap again for the third approval.
- Solution: the Findings paragraph says the approval was later dismissed and names the push, F17's
  verification names the third event, and the Processing Log records it. The paragraph also says that
  every push dismisses the standing approval and that the push carrying this round's fixes dismisses
  review 5547344550 in the same way, so that gap does not recur for each later push: the `ACK`
  comments name the head each one applies to.
- Current-tree verification: when this entry was written the timeline held three `review_dismissed`
  events, all with actor `josecelano` and no message: 2026-10-16T08:31:50Z (review 5544619071),
  10:26:39Z (review 5545292694), and 11:22:22Z (review 5547008497). The audit names all three. The
  push that carried this round's fixes dismissed a fourth at 11:56:26Z (review 5547344550), as the
  Findings paragraph says it would.
- Resolution reference: docs(pr-reviews): record the third dismissal and the fourth review round of PR #2550
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294394133>

### F22 - The round-3 summary is described as confirming findings it called partly resolved

- PR number: 2550
- Source review ID: 5547344550
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294315594>
- Concern: the audit says the round-3 body "confirms the earlier ones as resolved", but that body
  called two of them partly resolved ("R3-F3 partly" and "R3-F5 partly"), and raised the rest as its
  F3 and F2 (F18 and F17 here).
- Solution: the third-round paragraph now says the body confirms three earlier findings as resolved
  and finds two partly resolved, raised as F18 and F17. The second-round paragraph had the same
  overstatement, because that body names two remaining gaps raised as F10 and F12, and now says so.
- Current-tree verification: the Findings paragraphs for the second and third rounds say so, and
  both bodies were re-read.
- Resolution reference: docs(pr-reviews): record the third dismissal and the fourth review round of PR #2550
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294394352>

### F23 - "The population of every measure" overstates the fix

- PR number: 2550
- Source review ID: 5547344550
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294315602>
- Concern: the same as F20: the scope bullet applies the merged-in-the-window population to the
  force-push total and its classification only, while baseline section 6 counts runs dated 1 to 13
  October and section 7 counts commits by author date. The conversation response and F15 repeat
  "every measure".
- Solution: fixed together with F20. The progress entry, F15, and the conversation response say
  "every force-push measure".
- Current-tree verification: `grep -n "every measure"` on the `ISSUE.md` and the audit finds nothing
  that applies the population to other measures, and the conversation response reads "every
  force-push measure".
- Resolution reference: docs(issues): [#2547] limit the shared population to the force-push measures
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294394552>

### F24 - F21's verification asserts a count that went stale

- PR number: 2550
- Source review ID: 5547591114
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294512807>
- Concern: F21's verification says "the timeline holds three `review_dismissed` events", but it held
  four when this record's last commit was made (the fourth at 11:56:26Z, review 5547344550, the push
  the Findings paragraph predicts). Write "held three when this entry was written", or drop the count.
- Solution: F21's verification now says the timeline held three events when the entry was written and
  names the fourth. The Findings paragraph on dismissals states times for the events it names and no
  longer says how many the timeline holds.
- Current-tree verification: the timeline holds four `review_dismissed` events as of this entry, at
  08:31:50, 10:26:39, 11:22:22, and 11:56:26 UTC, all with actor `josecelano` and no message.
- Resolution reference: docs(pr-reviews): record the fifth review round of PR #2550
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294574667>

### F25 - The 11:57 log entry lists actions in an order that did not happen

- PR number: 2550
- Source review ID: 5547591114
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294512826>
- Concern: the entry lists "Pushed the fixes, updated the GitHub issue body of #2547, edited the
  conversation response", but the response edit (11:53:18Z) and the issue body update (11:56:21Z)
  came before the push (11:56:26Z). The edited response described a spec fix three minutes before it
  reached the branch, the reply-before-push order that F19 recorded.
- Solution: the Processing Log is append-only, so the 11:57 entry is unchanged and a correction entry
  gives the real order and says that the edit preceded the push.
- Current-tree verification: the Processing Log has the correction entry after the 11:57 entry. The
  edit (11:53:18Z), the issue body update (11:56:21Z), and the push (11:56:26Z) are the times the
  reviewer captured.
- Resolution reference: docs(pr-reviews): record the fifth review round of PR #2550
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294574962>

### F26 - The selection rules are attributed to baseline sections that do not state them

- PR number: 2550
- Source review ID: 5547591114
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294512840>
- Concern: baseline section 6 has no reproduce command and names no date field, and the command in
  section 7 filters with `--since`, which is the committer date, while printing author dates. The
  specification and F20 attributed "runs by creation date" and "commits by author date" to those
  sections. State the rules as the specification's own.
- Solution: the specification says that it fixes how each measure selects its data (runs created in
  the window; commits whose author date is in the window, as the 13-of-23 recount did) and notes
  what sections 6 and 7 say, and the GitHub issue body matches. F20's text no longer attributes the
  rules to the baseline sections.
- Current-tree verification: the `ISSUE.md` scope bullet says so, baseline section 6 reports 899 runs
  and names no date field, and the section 7 command begins
  `git log --no-merges --since=2026-06-01 --format='%h %ad %s' --date=short` and continues with `-i -E`
  and a `--grep` for the rebase-related words. It filters with `--since`, which is the committer
  date, and prints author dates (`%ad`). This entry first quoted only the first line (F27).
- Resolution reference: docs(issues): [#2547] state the selection rules as this specification's own
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294575223>

### F27 - The quoted section 7 command drops its filter

- PR number: 2550
- Source review ID: 5547815458
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294689542>
- Concern: F26's verification quotes the baseline section 7 command as ending at `--date=short`, but
  that command continues with `-i -E` and a `--grep` for the rebase-related words. Run as quoted, it
  lists every non-merge commit since June. The date point stands. Quote both lines, or say that the
  command filters with `--since=2026-06-01`.
- Solution: F26's verification now says the command begins with the quoted line and continues with
  `-i -E` and a `--grep`, and that it filters with `--since` (committer date) and prints `%ad`
  (author date).
- Current-tree verification: baseline section 7's reproduce command is
  `git log --no-merges --since=2026-06-01 --format='%h %ad %s' --date=short -i -E`
  followed by `--grep='rebase|rebased|...|orphan'` on the next line, and F26 says so.
- Resolution reference: docs(pr-reviews): record the sixth review round of PR #2550
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2550#discussion_r4294739924>

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
- 2026-10-16 10:30 UTC - Pushed that record, resolved the five threads, and re-requested da2ce7 and
  Copilot (10:30:08 UTC).
- 2026-10-16 10:46 UTC - Copilot's review 5546689152 arrived at the pushed head, with one finding in
  its body and no inline thread (F15).
- 2026-10-16 11:12 UTC - da2ce7's approving review 5547008497 and an `ACK` comment arrived for the
  same head, with four new threads (F16-F19).
- 2026-10-16 11:19 UTC - Recorded that review 5545292694's approval was dismissed at 10:26:39 UTC by
  the push of the "log the second PR #2550 review round" commit (F17). Fixed F15 and F16, and F18, in
  signed commits, and fixed F17 and F19 by adding F15-F19 and these entries to this record.
- 2026-10-16 11:24 UTC - Pushed the fixes, updated the GitHub issue body of #2547, replied on the
  four new threads and, from the pushed head, on the F14 thread, posted a PR conversation response
  for F15, and added the reply URLs to this record.
- 2026-10-16 11:25 UTC - Resolved the four threads and re-requested da2ce7 and Copilot.
- 2026-10-16 11:42 UTC - Copilot's review 5547291916 arrived at the pushed head, with one inline
  thread (F20).
- 2026-10-16 11:47 UTC - da2ce7's approving review 5547344550 and an `ACK` comment arrived for the
  same head, with three new threads (F21-F23).
- 2026-10-16 11:54 UTC - Recorded that review 5547008497's approval was dismissed at 11:22:22 UTC by
  the push of the "log the third PR #2550 review round" commit (F21). Fixed F20 and F23 in one
  signed commit and F21 and F22 in this record, which also says why later pushes are not logged one
  by one.
- 2026-10-16 11:57 UTC - Pushed the fixes, updated the GitHub issue body of #2547, edited the
  conversation response to say "every force-push measure", replied on the four new threads, and added
  the reply URLs to this record.
- 2026-10-16 12:12 UTC - da2ce7's approving review 5547591114 and an `ACK` comment arrived for the
  record's head, with three new threads (F24-F26).
- 2026-10-16 12:17 UTC - Correction of the 11:57 UTC entry's order (F25): the conversation-response
  edit (11:53:18 UTC) and the GitHub issue body update (11:56:21 UTC) came before the push of the
  fixes (11:56:26 UTC), so the edited response described the specification fix before it reached the
  branch, the same order that F19 recorded.
- 2026-10-16 12:17 UTC - Fixed F26 in one signed commit and F24 and F25 in this record.
- 2026-10-16 12:20 UTC - Pushed the fixes, updated the GitHub issue body of #2547, and, after the
  push, replied on the three threads and added the reply URLs to this record.
- 2026-10-16 12:33 UTC - da2ce7's approving review 5547815458 and an `ACK` comment arrived for the
  record's head, with one new thread (F27). Copilot's review 5547644540 had recommended approval with
  no open findings at 12:17 UTC.
- 2026-10-16 12:37 UTC - Fixed F27 in this record.
- 2026-10-16 12:40 UTC - Pushed the fix, replied on the thread after the push, and added the reply
  URL to this record.

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
