---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2809 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2809.

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

Five rounds: da2ce7 review 5845757723 (approval with findings F1 to F5) and Copilot review 5845810205 (two comments) in round 1; da2ce7 review 5846259189 (approval with findings F6 to F9) in round 2; Copilot review 5846488382 (two comments, run on the head that review 5846259189 read) in round 3. Copilot's `F1` and `F2` collided with da2ce7's and became `F6` and `F7`; round 2's `F6` to `F9` became `F8` to `F11`; round 3's two comments are `F12` and `F13`; round 4's da2ce7 review 5846762716 (approval with findings F12 to F16) collided with those and became `F14` to `F18`; round 5's Copilot review 5846854848 (three comments, run on the head after round 4's fixes) is `F19` to `F21`. The original IDs are recorded in the details.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2809-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2809-f2 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2809-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2809-f4 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2809-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2809-f6 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2809-f7 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2809-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2809-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2809-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2809-f11 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2809-f12 | Copilot | Suggestion (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F13 | review-finding:pr-2809-f13 | Copilot | Suggestion (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F14 | review-finding:pr-2809-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2809-f15 | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2809-f16 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2809-f17 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2809-f18 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2809-f19 | Copilot | Suggestion (inferred) | metadata | ORIGINAL | NO_ACTION | RESOLVED |
| F20 | review-finding:pr-2809-f20 | Copilot | Suggestion (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2809-f21 | Copilot | Suggestion (inferred) | formatting | ORIGINAL | NO_ACTION | RESOLVED |

## Finding Details

### F1 - `git diff -M` cannot show the T2 split as a move

- PR number: 2809
- Source review ID: 5845757723
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527751610
- Concern: T2 keeps `docs/release_process.md` as the index, so rename detection has no deleted source to pair; the evidence named for T2 cannot show a move.
- Solution: T2 and the Commit Points sentence name `git diff --color-moved` and say why `git diff -M` cannot pair the split.
- Current-tree verification: `grep -n 'color-moved' docs/issues/open/2808-workspace-package-release-process/ISSUE.md` shows the T2 row and the Commit Points sentence.
- Resolution reference: docs(issues): [#2808] address review of the specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527869989

### F2 - `issue #7` names this repository's issue 7

- PR number: 2809
- Source review ID: 5845757723
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527751621
- Concern: The bare `#7` links torrust-tracker issue 7, not the torrust-server-lib issue.
- Solution: The reference reads `torrust/torrust-server-lib#7`.
- Current-tree verification: `grep -n 'issue #7' docs/issues/open/2808-workspace-package-release-process/ISSUE.md` prints nothing.
- Resolution reference: docs(issues): [#2808] address review of the specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527870671

### F3 - A past progress-log entry was rewritten and contradicted itself

- PR number: 2809
- Source review ID: 5845757723
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527751632
- Concern: The second commit replaced `packages.md` with `workspace-packages.md` in an earlier log entry, which then named the new file while saying the pointer targets `packages.md`.
- Solution: The entry has its original wording again; a new entry records the restoration.
- Current-tree verification: The entry was compared with the same entry in the first commit of the branch (`docs(issues): add issue specification for #2808`); only its time differs, corrected for F4.
- Resolution reference: docs(issues): [#2808] address review of the specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527871164

### F4 - Times labelled UTC were later than the commits that recorded them

- PR number: 2809
- Source review ID: 5845757723
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527751641
- Concern: `last-updated-utc` and the log entries were written ahead of UTC.
- Solution: The log times and `last-updated-utc` are UTC, taken from the lint runs, the issue creation and the commit times; the spec, the PR body and the issue body have no other time stamps.
- Current-tree verification: Each log time was compared with `git log --format=%cI` of the commit that carries it: none is later.
- Resolution reference: docs(issues): [#2808] address review of the specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527871641

### F5 - T2-T6 read as ready while the implementation waits for #2576

- PR number: 2809
- Source review ID: 5845757723
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527751657
- Concern: The tasks were `TODO` although Background holds the implementation until #2576 is closed.
- Solution: T2-T6 are `BLOCKED`, with a line above the table naming #2576.
- Current-tree verification: `grep -n '^| T[1-6]  |' docs/issues/open/2808-workspace-package-release-process/ISSUE.md` shows T1 `DONE` and T2-T6 `BLOCKED`.
- Resolution reference: docs(issues): [#2808] address review of the specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527872127

### F6 - Steps 2 and 9 of `release-new-version` still bump every manifest

- PR number: 2809
- Source review ID: 5845810205
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527791361
- Concern: Removing the step 6 list does not satisfy AC3, because steps 2 and 9 still edit the version in all `Cargo.toml` files, against the ADR.
- Solution: Background, scope (b), T4 and its commit point include making steps 2 and 9 bump only `torrust-tracker`.
- Current-tree verification: `grep -n 'steps 2 and 9' docs/issues/open/2808-workspace-package-release-process/ISSUE.md` shows Background, scope (b), T4 and the progress-log entry, and `grep -n 'steps 2, 6 and 9' docs/issues/open/2808-workspace-package-release-process/ISSUE.md` shows the commit point; `grep -n 'all Cargo.toml' .github/skills/dev/git-workflow/release-new-version/SKILL.md` confirms the current wording.
- Resolution reference: docs(issues): [#2808] address the Copilot review of the specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527919029

### F7 - M2 expects no duplication in server-lib docs that this issue cannot edit

- PR number: 2809
- Source review ID: 5845810205
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527791463
- Concern: torrust-server-lib's release doc repeats the release steps in a checklist, and this issue neither edits it nor planned a cleanup.
- Solution: T6, scope (d) and AC5 include follow-up issue text that replaces the checklist with a pointer; M2 expects the standalone variant here to be complete on its own.
- Current-tree verification: `gh-bot api repos/torrust/torrust-server-lib/contents/docs/release_process.md?ref=main` shows the checklist; the T6 row and M2 were read after the edit.
- Resolution reference: docs(issues): [#2808] address the Copilot review of the specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4527919499

### F8 - The correction says the times were "one hour or more ahead of UTC", which the corrected times contradict

- PR number: 2809
- Source review ID: 5846259189
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528137439
- Concern: The earlier fix and reply described the offsets wrongly: one time was behind, three less than an hour ahead, one more than an hour ahead.
- Solution: An appended spec progress-log entry states the actual offsets; the earlier entry is not rewritten.
- Current-tree verification: `grep -n 'Correction to the 16:49 entry' docs/issues/open/2808-workspace-package-release-process/ISSUE.md` shows the entry. The F4 thread reply and this finding's reply carry edit notes pointing to the exact offsets (see F16).
- Resolution reference: docs(issues): [#2808] state the actual offsets of the corrected log times
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528257509

### F9 - The F6 verification grep does not print the commit point it names

- PR number: 2809
- Source review ID: 5846259189
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528137448
- Concern: The verification said one grep shows the commit point, but it prints Background, scope (b), T4 and a log entry.
- Solution: The F6 verification names what the first grep prints and adds the grep for the commit point.
- Current-tree verification: Both greps were run against the spec at the head: the first prints lines 57, 120, 168 and 211; `grep -n 'steps 2, 6 and 9'` prints line 178.
- Resolution reference: docs(pr-reviews): [#2808] correct the audit record of PR 2809
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528257953

### F10 - The F3 thread reply cites a PR-branch commit by id

- PR number: 2809
- Source review ID: 5846259189
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528137456
- Concern: A reply on the PR cited the first commit by abbreviated id, which a rebase would invalidate.
- Solution: The reply cites the commit subject and carries a one-line note of the edit (made at 17:22 UTC).
- Current-tree verification: Every reply of this account on the PR was matched against local commits: only that reply cited an id, and it no longer does. The fix is the reply edit, outside the tree, so the reference is the durable URL of that reply (see F17).
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528258393>
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528258393

### F11 - Two log entries record events later than their stamps

- PR number: 2809
- Source review ID: 5846259189
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528137464
- Concern: The 16:46 entry recorded a review submitted at 16:50:05 UTC and the 17:01 entry the audit commit dated 17:03:08 UTC.
- Solution: An appended log entry states both later times; the earlier entries are not rewritten.
- Current-tree verification: `git log --format=%cI` shows the audit commit at 17:03:08Z; the appended entry is stamped after both events.
- Resolution reference: docs(pr-reviews): [#2808] correct the audit record of PR 2809
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528258864

### F12 - A progress-log entry says "README pointer to `develop` with anchor", against the final decision

- PR number: 2809
- Source review ID: 5846488382
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528314930
- Concern: The 16:10 entry records an anchor pointer while the Background decisions say the pointer is not an anchor inside the tracker document.
- Solution: None, recorded as `Superseded by F3` in the reply (the prescribed form for a no-change thread; see F14). The entry records the decision as it stood at 16:10 UTC; the 16:15 and 16:24 entries record the later changes, which is the reviewer's own option (b), and rewriting the entry would repeat finding F3.
- Current-tree verification: `grep -n '^- 2026-11-10 16:1[05]' docs/issues/open/2808-workspace-package-release-process/ISSUE.md` shows the 16:10 and 16:15 entries, and the 16:24 entry records the rename; the Background decisions list holds the final decision.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528341871>
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528341871

### F13 - The spec references "audit F8", which the audit does not define

- PR number: 2809
- Source review ID: 5846488382
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528315016
- Concern: The 17:23 spec entry cites audit F8 while the audit defined findings only up to F7.
- Solution: None, recorded as `Superseded by F8` in the reply (the prescribed form for a no-change thread; see F14). The review was written before round 2 added F8 to the audit (the head it read is the one of da2ce7's round-2 review 5846259189); at the current head F8 exists and is the finding that entry corrects.
- Current-tree verification: `grep -n '^### F8' docs/pr-reviews/pr-2809-review/PR-REVIEW.md` prints the detail entry, and the spec entry stamped 17:23 UTC cites it.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528342295>
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528342295

### F14 - F12 and F13 are no-change threads without the prescribed reply form

- PR number: 2809
- Source review ID: 5846762716
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528523574
- Concern: The replies began "No action" and the rows recorded `NO_ACTION` with `RESOLVED` instead of `Superseded by <FindingId>: <reason>.` and `SUPERSEDED`.
- Solution: The F12 and F13 thread replies were edited to begin `Superseded by F3:` and `Superseded by F8:` with an edit note, and both rows record `NO_ACTION` with `SUPERSEDED`.
- Current-tree verification: `grep -n '| F1[23] ' docs/pr-reviews/pr-2809-review/PR-REVIEW.md` shows `NO_ACTION | SUPERSEDED` for both rows; both replies were read back from GitHub.
- Resolution reference: docs(pr-reviews): [#2808] fix the form of the PR 2809 audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528599524

### F15 - A reply cites a PR-branch commit by id

- PR number: 2809
- Source review ID: 5846762716
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528523585
- Concern: The F13 thread reply named the head it was written at by its abbreviated commit id.
- Solution: The reply no longer cites an id.
- Current-tree verification: Every reply of this account on the PR was matched against local commits; none cites an id now.
- Resolution reference: docs(pr-reviews): [#2808] fix the form of the PR 2809 audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528599979

### F16 - "Review replies are not rewritten" is contradicted by this record

- PR number: 2809
- Source review ID: 5846762716
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528523606
- Concern: The audit and the F8 reply gave a reason for keeping the F4 reply's wording that the F10 and F12 reply edits contradict.
- Solution: The F4 and F8 replies carry edit notes, the F8 reply's wrong sentence is replaced, and the F8 detail here says so.
- Current-tree verification: `grep -n 'not rewritten\|cannot be corrected' docs/pr-reviews/pr-2809-review/PR-REVIEW.md` prints only lines about log entries.
- Resolution reference: docs(pr-reviews): [#2808] fix the form of the PR 2809 audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528600396

### F17 - F10's resolution reference is the commit that records the fix, not the fix

- PR number: 2809
- Source review ID: 5846762716
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528523615
- Concern: The F10 fix is the edit of a reply, outside the tree, and the audit cited a commit subject for it.
- Solution: The F10 resolution reference is the durable URL of the reply that states the fix.
- Current-tree verification: `grep -n -A9 '^### F10' docs/pr-reviews/pr-2809-review/PR-REVIEW.md` shows the URL; the validator passes.
- Resolution reference: docs(pr-reviews): [#2808] fix the form of the PR 2809 audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528600977

### F18 - A line of the audit cites a PR-branch commit by id

- PR number: 2809
- Source review ID: 5846762716
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528523627
- Concern: The audit named a commit by its abbreviated id, and called a head "the round-1 head" although round 1 ran on an earlier commit.
- Solution: The id is removed and the head is named by the review that read it (review 5846259189).
- Current-tree verification: The commit-id check of `open-pull-request` prints nothing over the files the branch changes at this head, and a search of the audit for the abbreviated id of that commit prints nothing.
- Resolution reference: docs(pr-reviews): [#2808] fix the form of the PR 2809 audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528601560

### F19 - `related-pr` is `null` although PR 2809 introduces the spec

- PR number: 2809
- Source review ID: 5846854848
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528593488
- Concern: The reviewer asked to set `related-pr: 2809`.
- Solution: None. The field names the implementation PR; the sibling spec-only specs for issues 2798 and 2801 keep it `null`, and the spec's progress log says so.
- Current-tree verification: `grep -n '^related-pr' docs/issues/open/2798-*/ISSUE.md docs/issues/open/2801-*/ISSUE.md` prints `null` for both.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528646736>
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528646736

### F20 - The `PR 6` and `PR 8` references are not linkable and ambiguous

- PR number: 2809
- Source review ID: 5846854848
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528593572
- Concern: The Background named two torrust-server-lib pull requests as bare `PR 6` and `PR 8`.
- Solution: Both are fully qualified as `torrust/torrust-server-lib#6` and `#8`.
- Current-tree verification: `grep -n 'PR 6\|PR 8' docs/issues/open/2808-workspace-package-release-process/ISSUE.md` prints nothing.
- Resolution reference: docs(issues): [#2808] qualify the torrust-server-lib PR references
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528647120

### F21 - Use a Markdown link for the PR URL in the audit's `Source:` line

- PR number: 2809
- Source review ID: 5846854848
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528593630
- Concern: The reviewer asked for a Markdown link instead of the bare URL.
- Solution: None. The line follows `docs/templates/PR-REVIEW-TEMPLATE.md` line 15 and the 38 audits that use the same form; GitHub renders the URL and the lychee link check passes.
- Current-tree verification: `grep -n '^Source:' docs/templates/PR-REVIEW-TEMPLATE.md` shows the template line; `linter lychee` exited 0 at this head.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528647532>
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2809#discussion_r4528647532

## Processing Log

- 2026-11-10 16:46 UTC - Round 1 review of da2ce7 received (approval at 16:46:32 UTC with five findings, F1 to F5); Copilot review at 16:50:05 UTC (two comments, F6 and F7 here).
- 2026-11-10 16:57 UTC - Replies to F1 to F5 posted (16:57:01 to 16:57:13 UTC) after the fix commit was pushed, and their threads resolved.
- 2026-11-10 17:01 UTC - Replies to F6 and F7 posted (17:01:30 and 17:01:33 UTC) after their fix commit was pushed, and their threads resolved; this audit record was added in the commit that carries this entry.
- 2026-11-10 17:26 UTC - Correction to the log above: the Copilot review in the 16:46 entry was submitted at 16:50:05 UTC, after that entry's stamp, and the commit that added this audit record is dated 17:03:08 UTC, after the 17:01 entry's stamp. The entries are not rewritten; later events are stamped when they are recorded.
- 2026-11-10 17:33 UTC - Round 2 review of da2ce7 received (approval at 17:21:07 UTC with four findings, `F8` to `F11` here); fixes committed at 17:23:43 and 17:27:00 UTC and pushed, replies posted at 17:32:12 to 17:32:20 UTC and the threads resolved; the audit edited in the commit that carries this entry.
- 2026-11-10 17:42 UTC - Round 3 review of Copilot received (submitted at 17:37:35 UTC on the head that da2ce7's round-2 review 5846259189 read, two comments, `F12` and `F13` here); both closed with no change, replies posted at 17:40:09 and 17:40:11 UTC and the threads resolved. The F12 reply was edited at 17:40:29 UTC to correct a cited time. The audit was edited in the commit that carries this entry.
- 2026-11-10 18:08 UTC - Round 4 review of da2ce7 received (approval at 17:56:24 UTC with five findings, `F14` to `F18` here) and round 5 review of Copilot (submitted at 18:02:42 UTC, three comments, `F19` to `F21` here). Round 4 fixes were committed at 18:00:08 UTC and pushed; the F20 fix was committed at 18:04:51 UTC and pushed. Replies were posted at 18:03:12 to 18:03:23 UTC (round 4) and 18:07:39 to 18:07:43 UTC (round 5), and the threads resolved. One batch resolution at 18:03 UTC resolved the three round-5 threads before they had replies; they were reopened as soon as I noticed and resolved again after their replies. The audit was edited in the commit that carries this entry.

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
