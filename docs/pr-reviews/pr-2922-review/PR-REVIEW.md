---
pr-number: 2922
pr-url: https://github.com/torrust/torrust-tracker/pull/2922
last-updated-utc: "2026-11-17 18:52"
---

<!-- skill-link: process-pr-review -->

# PR #2922 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2922.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

Round 1: da2ce7 review 5944086342 (APPROVED) with findings F1 to F4; the reviewer's IDs are this audit's IDs. Copilot review 5944100743 (COMMENTED) with one finding, R1; the reviewer's ID is this audit's ID. R1 asks for the same current-tree change as F2. Round 2: da2ce7 review 5944394364 (APPROVED) with findings F5 and F6; the reviewer's IDs are this audit's IDs. Round 3: Copilot review 5944579824 (COMMENTED, at the round 1 fix head) with two findings without reviewer IDs, as audit findings F7 and F8; da2ce7 review 5944605224 (APPROVED) reported no finding and has no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2922-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2922-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2922-f3 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2922-f4 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2922-r1 | Copilot | Nit | metadata | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F5 | review-finding:pr-2922-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2922-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2922-f7 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2922-f8 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The description pairs a closing keyword with an upstream issue reference

- PR number: 2922
- Source review ID: 5944086342
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603826215
- Concern: The description's T2 bullet ended with a closing keyword followed by the upstream `zerocopy` issue reference, which the `open-pull-request` closing-keyword pattern matches. The pull request must not close that issue, and the merge tool copies the description into the merge commit. The specification line carries the same words but files are not parsed for keywords.
- Solution: Edited the description: the T2 bullet now says "a `zerocopy` release with the fix tracked in" the upstream issue, and the T1 bullet says "a change proposed in" the upstream pull request. Swept the branch for the same wording: two specification entries (T2 and the hold) had it and were reworded by the commit `docs(issues): [#2360] reword the upstream references without a closing keyword`.
- Current-tree verification: The closing-keyword pattern of the `open-pull-request` skill, run with `grep -ciE` on the description returned by `gh pr view 2922 --json body` after the edit, printed `0`; run on the specification at the head `docs(issues): [#2360] mark the issue blocked in the #2003 EPIC table`, it printed `0`; the folded commit messages of `torrust/develop..HEAD` had no match.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2922#issuecomment-6957005270
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603914743

### F2 - The three new progress-log entries name the agent but not its model

- PR number: 2922
- Source review ID: 5944086342
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603826227
- Concern: The `create-issue` skill and `docs/templates/ISSUE.md` ask for the form `Agent (model)` in each progress-log entry an AI agent writes. The three new entries gave the agent alone.
- Solution: The three entries now read `Claude Code (Claude Fable 5.2)`. The session changed model between the first checks and the entries, so the T1 entry also says which model ran which check.
- Current-tree verification: `grep -c "UTC - Claude Code (Claude Fable 5.2) - " docs/issues/open/2360-2003-resolve-udp-protocol-empty-enums-allowance-scope/ISSUE.md` prints `4` (the three entries and the later clarification entry); `grep -c "UTC - Claude Code - "` on the same file prints `0`.
- Resolution reference: docs(issues): [#2360] name the model in the new progress-log entries
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603915281

### F3 - The hold entry leaves open what T3 becomes when the issue is reviewed again

- PR number: 2922
- Source review ID: 5944086342
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603826234
- Concern: T2 chose Temporary, whose reason rewrite depends on nothing upstream, yet the issue is `blocked` and T3 stays `TODO`. The plan did not let a reader tell a hold from a dependency, nor say what T3 becomes if a fixed `zerocopy` release exists at the review.
- Solution: Appended a progress-log entry instead of rewriting the hold entry. It says the hold is a deferral, that the rewrite is deferred on purpose, and what T3 becomes in each case: removing A159 if a `zerocopy` release carries the upstream change, otherwise the Temporary rewrite.
- Current-tree verification: `grep -c "The hold is a deferral, not a dependency" docs/issues/open/2360-2003-resolve-udp-protocol-empty-enums-allowance-scope/ISSUE.md` prints `1`.
- Resolution reference: docs(issues): [#2360] say what T3 becomes after the hold
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603915668

### F4 - The EPIC row for this issue still reads TODO while the specification says blocked

- PR number: 2922
- Source review ID: 5944086342
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603826239
- Concern: Outside the diff: order 13 of the #2003 EPIC table kept `TODO` while the specification's frontmatter says `status: blocked`. The table's status values include `BLOCKED`.
- Solution: Order 13 now reads `BLOCKED` with a short note on the hold. The commit also updates the EPIC's `last-updated-utc`.
- Current-tree verification: `grep -c "ISSUE.md\` | BLOCKED | Follow-up of #2347" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` prints `1`.
- Resolution reference: docs(issues): [#2360] mark the issue blocked in the #2003 EPIC table
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603916069

### R1 - Include the model in the new progress-log attributions

- PR number: 2922
- Source review ID: 5944100743
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603836372
- Concern: The three new entries identify only `Claude Code`; the issue template requires `Agent (model)`. Add the model to these three entries and leave the historical entries unchanged.
- Solution: Same current-tree change as F2, already made by the F2 commit; no further change. Replied with the prescribed `Superseded by` form.
- Current-tree verification: Same commands as F2: `4` entries with the model, `0` without. For the historical entries, `git diff torrust/develop...HEAD -- docs/issues/open/2360-2003-resolve-udp-protocol-empty-enums-allowance-scope/ISSUE.md | grep -c '^[-+]- 2026-09-28'` prints `0`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603916463
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603916463

### F5 - The row 13 status change has no line in the EPIC's progress log

- PR number: 2922
- Source review ID: 5944394364
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4604057139
- Concern: The EPIC template asks for one progress-log line per meaningful update, and this EPIC logs the row changes it makes. The change of row 13 to `BLOCKED` had no line.
- Solution: Appended a late progress-log entry to the EPIC for row 13, with the agent and its model, the time of the row change, and the reason for the hold.
- Current-tree verification: `grep -c "issue #2360 session) - Late entry for row 13" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` prints `1`. The time in the entry, 18:19 UTC, is the committer time `2026-11-17T18:19:46Z` that `git log --format=%cI` prints for the commit `docs(issues): [#2360] mark the issue blocked in the #2003 EPIC table`.
- Resolution reference: docs(issues): [#2360] log the row 13 status change in the #2003 EPIC
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4604118490

### F6 - The Processing Log does not say that the record was started after the replies

- PR number: 2922
- Source review ID: 5944394364
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4604057158
- Concern: The `process-pr-review` skill asks for the record before the first reply and, when a reply came first, for the late start to be logged. The first Processing Log entry, stamped 18:23, lists replies of 18:22:46Z to 18:22:56Z, so the order can be read off the times but is not stated.
- Solution: Appended a Processing Log entry that states the late start of round 1. The first entry is left as written, because the log is append-only. The same entry records the round 1 thread resolutions, which the first entry did not cover.
- Current-tree verification: `grep -c "Late start of round 1" docs/pr-reviews/pr-2922-review/PR-REVIEW.md` prints `2` (the log entry and this line).
- Resolution reference: docs(pr-reviews): [#2360] audit review round 2 of PR 2922
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4604118907

### F7 - EPIC row 13 reads as if the hold were an upstream dependency

- PR number: 2922
- Source review ID: 5944579824
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4604207004
- Concern: The text said T3 "waits for a later review of upstream" `zerocopy` issue 3414, which reads like an upstream dependency, while the #2360 specification says the hold is a deferral and that the Temporary reason rewrite depends on nothing upstream.
- Solution: Reworded the sentence to say that T3 is deferred until the next maintainer review, when the status of the upstream issue is checked again. The row also says outright that the hold is a deferral and not an upstream dependency.
- Current-tree verification: `grep -c "waits for a later review" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` prints `0`; `grep -c "The hold is a deferral, not an upstream dependency" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` prints `1`.
- Resolution reference: docs(issues): [#2360] say in the #2003 EPIC that the hold is a deferral
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4604240027

### F8 - The EPIC progress-log entry for row 13 reads as if the hold were an upstream dependency

- PR number: 2922
- Source review ID: 5944579824
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4604207053
- Concern: The text said T3 "waits for a later review of upstream" `zerocopy` issue 3414, which reads like an upstream dependency, while the #2360 specification says the hold is a deferral and that the Temporary reason rewrite depends on nothing upstream.
- Solution: Reworded the sentence to say that T3 is deferred until the next maintainer review, when the status of the upstream issue is checked again. The entry was written by this pull request and is corrected in place. F8 is the same concern as F7 at another line of the same file; it has its own row because it needs its own change.
- Current-tree verification: `grep -c "waits for a later review" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` prints `0`; `grep -c "T3 is deferred until the next maintainer review" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` prints `2` (the row and the progress-log entry).
- Resolution reference: docs(issues): [#2360] say in the #2003 EPIC that the hold is a deferral
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4604240529

## Processing Log

- 2026-11-17 18:23 UTC - Started audit. Round 1: da2ce7 review 5944086342 (submitted 18:14:41Z) and Copilot review 5944100743 (submitted 18:15:35Z). Committed the F2, F1 (specification wording), F3 and F4 fixes as four signed commits and pushed them with `git-bot-push`, ending at `docs(issues): [#2360] mark the issue blocked in the #2003 EPIC table`. Edited the PR description for F1 and posted the PR conversation response (18:22:36Z). Replied to the five threads (18:22:46Z to 18:22:56Z).
- 2026-11-17 18:41 UTC - Late start of round 1: this record was created at 18:23 UTC, after the five round 1 replies (18:22:46Z to 18:22:56Z), although the skill asks for the record before the first reply. Also not logged before: the five round 1 threads were resolved after the push of the commit `docs(pr-reviews): [#2360] audit review round 1 of PR 2922`, and a GraphQL refresh then found no unresolved thread.
- 2026-11-17 18:41 UTC - Round 2: da2ce7 review 5944394364 (APPROVED, submitted 18:35:43Z) with F5 and F6. Committed the F5 fix. Added the F5 and F6 rows to this record, then replied to the two threads (18:41:21Z and 18:41:24Z). The validator then reported 7 rows and 1 failure: the F6 resolution reference names the commit of this record, which did not exist yet. The two threads were resolved at about 18:41:43Z in the same command, before that failure was read; a GraphQL refresh found no unresolved thread. Nothing of round 2 is pushed when this entry is written.
- 2026-11-17 18:52 UTC - The round 2 commits were pushed with `git-bot-push`, ending at `docs(pr-reviews): [#2360] audit review round 2 of PR 2922`. Round 3: Copilot review 5944579824 (COMMENTED, submitted 18:49:22Z) with F7 and F8, and da2ce7 review 5944605224 (APPROVED, submitted 18:51:03Z) with no finding. Committed the fix for F7 and F8 as one commit. Added the two rows to this record, then replied to the two threads (18:52:22Z and 18:52:25Z). The validator reported 9 rows, 3 log entries, 0 failures. Then resolved the two threads at about 18:52:54Z; a GraphQL refresh found no unresolved thread. Nothing of round 3 is pushed when this entry is written.

<!-- Copied verbatim into each audit record. -->

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
