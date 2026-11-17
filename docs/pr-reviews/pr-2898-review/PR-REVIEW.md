---
pr-number: 2898
pr-url: https://github.com/torrust/torrust-tracker/pull/2898
last-updated-utc: "2026-11-17 09:33"
---

<!-- skill-link: process-pr-review -->

# PR #2898 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2898.

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

Round 1: da2ce7 review 5935780107 (APPROVED) with findings F1 to F3, and Copilot review 5935810986 (COMMENTED, three findings without reviewer IDs) as audit findings F4 to F6; F5 re-raises F1.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2898-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2898-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2898-f3 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2898-f4 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2898-f5 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F6 | review-finding:pr-2898-f6 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Archived while #2394 is open

- PR number: 2898
- Source review ID: 5935780107
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597527906
- Concern: The cleanup skill archives a spec after its issue is closed, but #2394 is open and the archive log entry said "#2394 is closed by the maintainer", which reads as done.
- Solution: The log entry says #2394 is still open and that the maintainer closes it when this close-out merges; the "Issue closed" checkpoint stays unticked.
- Current-tree verification: `grep -n 'still open' docs/issues/closed/2394-2392-audit-shared-test-resources/ISSUE.md` finds the reworded entry; `grep -c 'is closed by the maintainer'` on the same file prints 0.
- Resolution reference: `docs(issues): [#2394] address review round 1 of PR 2898`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597552375

### F2 - C3's extension of order 4 stated as done

- PR number: 2898
- Source review ID: 5935780107
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597527923
- Concern: The EPIC order 2 row said C3 extends order 4's scope, but order 4's row and #2396's spec do not carry the extension yet.
- Solution: The row says C3 is covered once order 4's scope is extended to the local default image tags, which is pending.
- Current-tree verification: `grep -n 'which is pending' docs/issues/open/2392-test-isolation/EPIC.md` finds the order 2 row.
- Resolution reference: `docs(issues): [#2394] address review round 1 of PR 2898`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597552752

### F3 - The no-retrospective reason omits the C9 miss

- PR number: 2898
- Source review ID: 5935780107
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597527932
- Concern: The no-retrospective reason called all review findings corrections, but C9 was a problem the T1 searches missed, a reusable lesson for the next audit.
- Solution: The reason records the lesson: the T1 searches looked for shared names and values, not for resources released only on the success path; a future audit also checks failure and drop paths (EPIC rule 1).
- Current-tree verification: `grep -n 'failure and drop paths' docs/issues/closed/2394-2392-audit-shared-test-resources/ISSUE.md` finds the log entry.
- Resolution reference: `docs(issues): [#2394] address review round 1 of PR 2898`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597553169

### F4 - The closure checkpoint mixes the done move and the pending closure

- PR number: 2898
- Source review ID: 5935810986
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597552064
- Concern: The spec sits in `docs/issues/closed/` while its single checkpoint "Issue closed and spec moved" is unticked and #2394 is open; Copilot asked to split the checkpoint and word the log as awaiting closure.
- Solution: The checkpoint is split: the archive move is ticked, and "GitHub issue #2394 closed" stays unticked as pending. The log wording was fixed with F1.
- Current-tree verification: `grep -n 'GitHub issue #2394 closed' docs/issues/closed/2394-2392-audit-shared-test-resources/ISSUE.md` finds the unticked item next to the ticked move.
- Resolution reference: `docs(issues): [#2394] mark the pending closure of #2394 in the spec and EPIC`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597575482

### F5 - The log says #2394 is closed while it is open

- PR number: 2898
- Source review ID: 5935810986
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597552126
- Concern: Same thread text as F4, anchored on the log line that F1 already raised.
- Solution: Fixed by F1's rewording; the checkpoint split is F4's fix.
- Current-tree verification: `grep -c 'is closed by the maintainer' docs/issues/closed/2394-2392-audit-shared-test-resources/ISSUE.md` prints 0.
- Resolution reference: `docs(issues): [#2394] address review round 1 of PR 2898`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597575888

### F6 - The EPIC row says DONE while #2394 is open

- PR number: 2898
- Source review ID: 5935810986
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597552201
- Concern: Order 2 is DONE in the EPIC while GitHub issue #2394 is still open; Copilot asked for an explicit note.
- Solution: The order 2 row says #2394 is still open and the maintainer closes it when the close-out PR merges.
- Current-tree verification: `grep -n 'GitHub issue #2394 is still open' docs/issues/open/2392-test-isolation/EPIC.md` finds the order 2 row.
- Resolution reference: `docs(issues): [#2394] mark the pending closure of #2394 in the spec and EPIC`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2898#discussion_r4597576259

## Processing Log

- 2026-11-17 09:20 UTC - Round 1: da2ce7 review 5935780107 (APPROVED) with findings F1 (Minor), F2 (Nit) and F3 (Suggestion); the reviewer's IDs are this audit's IDs. Committed the three fixes (09:19:51Z), then replied to the three threads (09:19:57Z to 09:20:02Z). This record was created after the replies, a late start of the audit; the reviewer had said no record was due because the PR had no thread yet.
- 2026-11-17 09:21 UTC - Committed this record (09:20:57Z). `validate-audit-record.py --pr-number 2898 --base torrust/develop` (scratchpad copy, `gh` calls through `gh-bot`) reported 3 rows, 1 log entry, 0 failures.
- 2026-11-17 09:21 UTC - Resolved the unresolved threads at about 09:21:19Z. That resolved six threads, not three: Copilot review 5935810986 had posted three threads at 09:19:55Z, which I resolved before reading or replying to them, against the completion rules. The GraphQL refresh after it showed those three threads without a reply.
- 2026-11-17 09:22 UTC - Copilot's round: normalized its three findings as F4 to F6 (F5 re-raises F1), committed the F4 and F6 fixes (09:22:10Z), then replied to the three threads (09:22:16Z to 09:22:21Z), each reply noting the premature resolution. The threads stay resolved, now with their replies.
- 2026-11-17 09:33 UTC - The validator on the worktree record reported 6 rows, 4 log entries, 0 failures (about 09:23Z). A GraphQL refresh found every thread resolved with a reply from `josecelano-bot` as its last comment. Committed these entries.

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
