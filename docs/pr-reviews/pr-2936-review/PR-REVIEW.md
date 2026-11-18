---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2936 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2936.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2936-f1 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2936-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2936-f3 | Copilot | Minor | link-integrity | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F12 | review-finding:pr-2936-f12 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2936-f13 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Archived spec's parent EPIC link is broken

- PR number: 2936
- Source review ID: 5955371315
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4612890912
- Concern: After the move to `docs/issues/closed/`, line 29 of the SI-1 spec links `../2912-remove-source-locations-from-errors/EPIC.md`, which resolves to a path that does not exist; the EPIC is still in `docs/issues/open/`, and the link checker excludes `docs/issues/closed/`.
- Solution: The link is now `../../open/2912-remove-source-locations-from-errors/EPIC.md`, the form other archived sub-issues of open EPICs use. The spec has no other relative link; the one relative link in the evidence file is pasted skill output and stays verbatim.
- Current-tree verification: `sed -n 29p` of the archived ISSUE.md shows the new link, and `docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` exists.
- Resolution reference: docs(issues): [#2913] fix the parent EPIC link of the archived SI-1 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4612978835

### F2 - Pre-#2850 counts left outside the ADR

- PR number: 2936
- Source review ID: 5955371315
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4612890930
- Concern: The F10 correction of PR 2930 reached only the ADR: the EPIC spec (lines 46 and 56), the archived SI-1 spec (line 43) and the #384 spec (line 57) still give 23 `Display` formats and 24 variants without the "since #2850" qualifier.
- Solution: Those four statements now add "(22 since #2850)" or "(23 since #2850)", the form the documents already use for the 37 sites. A sweep of the EPIC, its sub-issue specs and the #384 spec found no other instance.
- Current-tree verification: Before the fix, two searches ran. A `grep -n` for the phrases "23 Display formats" and "24 ... variants" (in their Markdown spelling) over `docs/issues/open/29*-2912-*/ISSUE.md`, the EPIC, the #384 spec, the archived SI-1 spec and the ADR found the four lines, plus the two ADR lines already qualified. `grep -n -E '\b(23|24)\b[^|]{0,40}(format|variant|field)'` over `docs/issues/open/29*-2912-*/ISSUE.md`, the EPIC and the #384 spec found three of them; the archived SI-1 spec is outside those paths (review finding F12). Reproduced for this record with the second pattern over the EPIC, the #384 spec and the archived SI-1 spec as they were at the commit "docs(issues): [#2913] fix the parent EPIC link of the archived SI-1 spec" (read with `git show`): EPIC lines 46 and 56, #384 line 57, SI-1 line 43. After the fix each carries the qualifier. `linter markdown` passed.
- Resolution reference: docs(issues): [#2913] give the location counts since #2850 in the EPIC and its specs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4612979344

### F3 - Same broken parent EPIC link, raised by Copilot

- PR number: 2936
- Source review ID: 5955458544
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4612963101
- Concern: Moving the specification to `closed/` breaks the relative "Parent EPIC" link on line 29; it should point to `../../open/2912-remove-source-locations-from-errors/EPIC.md`.
- Solution: Same change as F1; no further change.
- Current-tree verification: Same inspection as F1.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4612985483
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4612985483

### F12 - F2 verification names a grep that misses one line

- PR number: 2936
- Source review ID: 5955641353
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4613126395
- Concern: The F2 entry says a `grep -n -E` over the open sub-issue specs, the EPIC and the #384 spec found all four lines before the fix, but the archived SI-1 spec is outside those paths, so that command finds three.
- Solution: The F2 verification now names both searches that ran (the phrase search, which covered the archived SI-1 spec, and the pattern search, which did not) and a reproduction of the pattern search over the three files as they were before the fix.
- Current-tree verification: Ran `grep -n -E '\b(23|24)\b[^|]{0,40}(format|variant|field)'` over the EPIC, the #384 spec and the archived SI-1 spec read with `git show` at the commit "docs(issues): [#2913] fix the parent EPIC link of the archived SI-1 spec": four lines (EPIC 46 and 56, #384 57, SI-1 43).
- Resolution reference: docs(pr-reviews): [#2913] name the searches that found the four count lines of PR 2936 F2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4613189548

### F13 - PR description does not cover the round-1 changes

- PR number: 2936
- Source review ID: 5955641353
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4613126419
- Concern: The description was unchanged since round 1 and listed neither the round-1 fixes (the #384 spec, the SI-1 parent link, the count lines) nor the new audit record, so the merge commit would describe less than it merges.
- Solution: Edited the PR description: a new bullet covers review round 1 of this PR (F1 and F3 link, F2 counts in the EPIC, SI-1 and #384 specs) and the audit record.
- Current-tree verification: `gh-bot pr view 2936 --json body` shows the bullet "Review round 1 of this PR".
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2936#issuecomment-6977205006
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2936#discussion_r4613190033

## Processing Log

- 2026-11-18 11:08 UTC - Started audit for da2ce7's review 5955371315 (APPROVED, submitted 11:06:16Z on the head commit "docs(pr-reviews): [#2913] record da2ce7's post-merge follow-up findings of PR 2930"), with two Minor findings recorded as F1 and F2. The maintainer and da2ce7 had ACKed that head; the maintainer chose in chat to fix both findings in this PR.
- 2026-11-18 11:11 UTC - Copilot review 5955458544 (submitted 11:11:38Z on the same head) raised the same broken link; recorded as F3, a re-raise of F1, with Copilot's own ID F1 kept as the reviewer finding ID.
- 2026-11-18 11:12 UTC - Pushed the F1 and F2 fixes (two commits; push finished before the first reply, exact time not recorded) and replied to F1 and F2 (replies created 11:12:49Z and 11:12:51Z).
- 2026-11-18 11:13 UTC - Resolved every unresolved thread at 11:13:03Z. That included the F3 thread, which had no reply yet: a process error (reply before resolving). Replied to F3 afterwards (reply created 11:13:22Z) and left the thread resolved.
- 2026-11-18 11:26 UTC - Round 2: da2ce7's review 5955641353 (APPROVED, submitted 11:24:38Z on the head commit "docs(pr-reviews): [#2913] audit review round 1 of PR 2936") raised F12 and F13 (reviewer IDs kept). Fixed F12 in a commit and F13 by editing the PR description, stated in a PR comment created 11:26:26Z.
- 2026-11-18 11:30 UTC - Pushed the F12 fix, replied to F12 and F13 (replies created 11:29:40Z and 11:29:43Z), then resolved both threads (`isResolved: true`).

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
