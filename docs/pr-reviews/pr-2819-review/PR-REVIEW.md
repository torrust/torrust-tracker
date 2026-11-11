---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2819 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2819.

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

Three reviews carry findings: da2ce7 review 5856686186 and Copilot review 5856712271 (round 1), and da2ce7 review 5857049397 (round 2). Copilot review 5857015571 of round 2 reports no open issues. Copilot's four comments are `R1` to `R4` here, because its own labels `F2` to `F5` collide with the da2ce7 labels. Round 2 da2ce7 labels `F7` to `F12` are used as they are.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2819-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2819-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2819-f3 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2819-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2819-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2819-f6 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2819-r1 | Copilot | Nit | metadata | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| R2 | review-finding:pr-2819-r2 | Copilot | Nit | metadata | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| R3 | review-finding:pr-2819-r3 | Copilot | Nit | metadata | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| R4 | review-finding:pr-2819-r4 | Copilot | Nit | metadata | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F7 | review-finding:pr-2819-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2819-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2819-f9 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2819-f10 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2819-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2819-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The review count cites an audit that records seven of the eight rounds.

- PR number: 2819
- Source review ID: 5856686186
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536250132
- Concern: The review count cites an audit that records seven of the eight rounds.
- Solution: The row and the line 313 entry say the audit records the summary and rounds 1 to 7, with the final approval on the forge.
- Current-tree verification: `git grep -n "rounds 1 to 7" docs/issues/closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md` lists both lines.
- Resolution reference: docs(issues): [#2801] address the review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536297407

### F2 - Four affected documents keep their old `last-updated-utc`.

- PR number: 2819
- Source review ID: 5856686186
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536250142
- Concern: Four affected documents keep their old `last-updated-utc`.
- Solution: The four documents are restamped (later set to the commit minute, see F10).
- Current-tree verification: `git grep -n last-updated-utc` on the four files.
- Resolution reference: docs(issues): [#2801] address the review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536297763

### F3 - The moved evidence file has no frontmatter.

- PR number: 2819
- Source review ID: 5856686186
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536250150
- Concern: The moved evidence file has no frontmatter.
- Solution: The template's block is added with `issue-spec` set to the closed path.
- Current-tree verification: `head -5` of the evidence file.
- Resolution reference: docs(issues): [#2801] address the review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536298137

### F4 - The title carries `[#2801]` for an already closed issue.

- PR number: 2819
- Source review ID: 5856686186
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536250156
- Concern: The title carries `[#2801]` for an already closed issue.
- Solution: The PR title is changed to drop it; the commit subjects keep it, as the reply says.
- Current-tree verification: `gh pr view 2819 --json title`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536298477
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536298477

### F5 - The completion review was recorded after the issue closed.

- PR number: 2819
- Source review ID: 5856686186
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536250166
- Concern: The completion review was recorded after the issue closed.
- Solution: The line 244 entry says so.
- Current-tree verification: `git grep -n "recorded after the issue closed"` in the spec.
- Resolution reference: docs(issues): [#2801] address the review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536298858

### F6 - The follow-ups have no tracking reference.

- PR number: 2819
- Source review ID: 5856686186
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536250174
- Concern: The follow-ups have no tracking reference.
- Solution: Two drafts are added and cited (renamed in F9).
- Current-tree verification: `ls docs/issues/drafts` lists them.
- Resolution reference: docs(issues): [#2801] address the review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536299160

### R1 - The migration guide keeps its old `last-updated-utc`.

- PR number: 2819
- Source review ID: 5856712271
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536271105
- Concern: The migration guide keeps its old `last-updated-utc`.
- Solution: Restamped, as F2.
- Current-tree verification: As F2.
- Resolution reference: docs(issues): [#2801] address the review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536299418

### R2 - SI-23 keeps its old `last-updated-utc`.

- PR number: 2819
- Source review ID: 5856712271
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536271216
- Concern: SI-23 keeps its old `last-updated-utc`.
- Solution: Restamped, as F2.
- Current-tree verification: As F2.
- Resolution reference: docs(issues): [#2801] address the review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536299748

### R3 - SI-24 keeps its old `last-updated-utc`.

- PR number: 2819
- Source review ID: 5856712271
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536271278
- Concern: SI-24 keeps its old `last-updated-utc`.
- Solution: Restamped, as F2.
- Current-tree verification: As F2.
- Resolution reference: docs(issues): [#2801] address the review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536300146

### R4 - The EPIC keeps its old `last-updated-utc`.

- PR number: 2819
- Source review ID: 5856712271
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536271331
- Concern: The EPIC keeps its old `last-updated-utc`.
- Solution: Restamped, as F2.
- Current-tree verification: As F2.
- Resolution reference: docs(issues): [#2801] address the review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536300568

### F7 - The head carries no review audit record.

- PR number: 2819
- Source review ID: 5857049397
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536537545
- Concern: The head carries no review audit record.
- Solution: This record is added.
- Current-tree verification: The validator is run against it.
- Resolution reference: docs(pr-reviews): [#2819] record the review rounds in the audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536677484

### F8 - Nine replies cite the branch commit id as the fix.

- PR number: 2819
- Source review ID: 5857049397
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536537553
- Concern: Nine replies cite the branch commit id as the fix.
- Solution: Review comments cannot be edited: the Processing Log maps `4a6e476d6` to its subject, and later replies cite subjects.
- Current-tree verification: The Processing Log entry.
- Resolution reference: docs(pr-reviews): [#2819] record the review rounds in the audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536677902

### F9 - The drafts' folder prefix names 2801 and `epic: 1488` holds the EPIC open.

- PR number: 2819
- Source review ID: 5857049397
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536537564
- Concern: The drafts' folder prefix names 2801 and `epic: 1488` holds the EPIC open.
- Solution: The drafts are unnumbered with `epic: null`.
- Current-tree verification: `ls docs/issues/drafts | grep follow-up-si-19` and `git grep -n "^epic:" docs/issues/drafts/follow-up-si-19-*`.
- Resolution reference: docs(issues): [#2801] address the second review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536678269

### F10 - The new stamps postdate the commit that writes them.

- PR number: 2819
- Source review ID: 5857049397
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536537573
- Concern: The new stamps postdate the commit that writes them.
- Solution: Unedited files carry 09:40, the first commit's minute; files edited in the second commit carry its minute, 10:19.
- Current-tree verification: `git log -1 --format=%cI` of both commits against the stamps.
- Resolution reference: docs(issues): [#2801] address the second review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536678684

### F11 - The drafts lack the sections `create-issue` initializes.

- PR number: 2819
- Source review ID: 5857049397
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536537580
- Concern: The drafts lack the sections `create-issue` initializes.
- Solution: Both drafts carry the five sections, with placeholders where the content waits for refinement.
- Current-tree verification: `grep -n "^## "` of both drafts.
- Resolution reference: docs(issues): [#2801] address the second review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536679162

### F12 - The supervisor draft narrows the out-of-scope item.

- PR number: 2819
- Source review ID: 5857049397
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536537589
- Concern: The supervisor draft narrows the out-of-scope item.
- Solution: The title, goal and scope name the three parts.
- Current-tree verification: `git grep -n "owner types"` in the draft.
- Resolution reference: docs(issues): [#2801] address the second review of the close-out PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2819#discussion_r4536679519

## Processing Log

- 2026-11-11 09:43 UTC - Round 1 reviews received (da2ce7 at 09:37, Copilot at 09:40); replies posted and threads resolved between 09:42 and 09:43 UTC. The replies say "Fixed in 4a6e476d6"; that id is the commit `docs(issues): [#2801] address the review of the close-out PR`, mapped here because review comments cannot be edited (F8).
- 2026-11-11 10:07 UTC - Round 2 review of da2ce7 received (six findings, `F7` to `F12`); the Copilot review at 10:04 UTC reported no open issues.
- 2026-11-11 10:23 UTC - Round 2 replies posted after the fix commit `docs(issues): [#2801] address the second review of the close-out PR`, and this audit added in the commit that carries this entry.

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
