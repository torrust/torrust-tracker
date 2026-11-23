---
pr-number: 3007
pr-url: https://github.com/torrust/torrust-tracker/pull/3007
last-updated-utc: "2026-11-23 16:41"
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore Greptile -->

# PR #3007 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3007.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (every finding was fixed before the merge).

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

Round 1, on the first head: Greptile review 6023114012 (one thread), CodeRabbit review 6023135871 (one thread), da2ce7 review 6023338079 (F1 to F4) and Copilot review 6023386734 (one thread, reviewer ID F1). The Greptile, CodeRabbit and Copilot threads raise the same concern as da2ce7's F1; they are recorded as F5, F6 and F7, each fixed by the same change. The earliest report is the Greptile thread (14:10), so F5 is the original and F1, F6 and F7 are `RE_RAISE_OF:F5` (corrected in round 2).

Round 2, on the head that carries the audit commit: Greptile review 6023502560 and CodeRabbit review 6023522118 (one thread each), and da2ce7 review 6023616767 (APPROVED, findings F5 to F7, recorded here as F10 to F12 because those IDs are taken). da2ce7's F6 asks for the change of the Greptile thread (F8), and his F7 asks for the change of the CodeRabbit thread (F9), so F11 and F12 are re-raises. CodeRabbit resolved its own thread after the reply.

Round 3, on the head that carries the round 2 audit: Copilot review 6025006260 (COMMENTED, with its "didn't start before the timeout" warning) left four threads. Three ask for the same change on lines 98, 107 and 114 of the EPIC, so F14 and F15 are re-raises of F13. F16 is not a defect and gets the prescribed `Superseded by` reply.

Round 4, on the head that carries the round 3 audit: da2ce7 review 6025458205 (APPROVED) with one Nit, recorded as F17 (his ID F13 is taken here).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3007-f1 | Human | Minor | correctness | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F2 | review-finding:pr-3007-f2 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3007-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3007-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3007-f5 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3007-f6 | Unknown | Major | correctness | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F7 | review-finding:pr-3007-f7 | Copilot | Minor | correctness | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F8 | review-finding:pr-3007-f8 | Unknown | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3007-f9 | Unknown | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3007-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3007-f11 | Human | Minor | metadata | RE_RAISE_OF:F8 | FIXED | RESOLVED |
| F12 | review-finding:pr-3007-f12 | Human | Minor | documentation | RE_RAISE_OF:F9 | FIXED | RESOLVED |
| F13 | review-finding:pr-3007-f13 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3007-f14 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F13 | FIXED | RESOLVED |
| F15 | review-finding:pr-3007-f15 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F13 | FIXED | RESOLVED |
| F16 | review-finding:pr-3007-f16 | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F17 | review-finding:pr-3007-f17 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The #2770 file-touch glob misses the new child specs

- PR number: 3007
- Source review ID: 6023338079
- Reviewer finding ID: da2ce7 F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667112137
- Concern: The #2770 file list uses `docs/issues/{open,closed}/27*-2758-*/ISSUE.md`, which does not match the #3004 and #3006 specs, although the close-out must repair every child spec's reference to the EPIC path.
- Solution: The list now uses `docs/issues/{open,closed}/*-2758-*/ISSUE.md`.
- Current-tree verification: `grep -c '27\*-2758' docs/issues/open/2770-2758-close-out-adr-and-epic/ISSUE.md` printed 0 at the fix head.
- Resolution reference: `docs(issues): [#2758] address da2ce7's review 6023338079 of PR 3007`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667165469

### F2 - Row 13's Local Spec names no spec PR

- PR number: 3007
- Source review ID: 6023338079
- Reviewer finding ID: da2ce7 F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667112158
- Concern: Row 13's Local Spec does not exist at this head or on `develop`, and the row names no spec PR, unlike row 12.
- Solution: Row 13 now says `(spec PR #3008)`.
- Current-tree verification: `grep -c '(spec PR #3008)' docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` printed 1 at the fix head.
- Resolution reference: `docs(issues): [#2758] address da2ce7's review 6023338079 of PR 3007`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667165971

### F3 - Counts and the row 1 note are stale

- PR number: 3007
- Source review ID: 6023338079
- Reviewer finding ID: da2ce7 F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667112179
- Concern: Three statements are stale after the two new rows: #2770's "Order 11 of 11", the EPIC risk "Eleven issues", and row 1's "no other issue edits it".
- Solution: #2770 says "Order 11 of 13", the risk says "Thirteen issues", and row 1 says no other issue edits the ADR's table (issue 13 amends its text, issue 11 updates the notes).
- Current-tree verification: `grep -c 'Eleven issues' docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` printed 0 and `grep -c 'Order 11 of 13' docs/issues/open/2770-2758-close-out-adr-and-epic/ISSUE.md` printed 1 at the fix head.
- Resolution reference: `docs(issues): [#2758] address da2ce7's review 6023338079 of PR 3007`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667166410

### F4 - The maintainer order lives only in the log

- PR number: 3007
- Source review ID: 6023338079
- Reviewer finding ID: da2ce7 F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667112192
- Concern: The maintainer's order of the remaining work is only in the progress log and the description, not in the Delivery Strategy.
- Solution: The Delivery Strategy now states the order and that #3006 merges before #2770.
- Current-tree verification: `grep -c 'set the order of the remaining work' docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` printed 1 at the fix head.
- Resolution reference: `docs(issues): [#2758] address da2ce7's review 6023338079 of PR 3007`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667167022

### F5 - Greptile: the #2770 glob misses the new children

- PR number: 3007
- Source review ID: 6023114012
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4666938735
- Concern: The #2770 file list uses `docs/issues/{open,closed}/27*-2758-*/ISSUE.md`, which does not match the #3004 and #3006 specs, although the close-out must repair every child spec's reference to the EPIC path.
- Solution: The list now uses `docs/issues/{open,closed}/*-2758-*/ISSUE.md`.
- Current-tree verification: `grep -c '27\*-2758' docs/issues/open/2770-2758-close-out-adr-and-epic/ISSUE.md` printed 0 at the fix head.
- Resolution reference: `docs(issues): [#2758] address da2ce7's review 6023338079 of PR 3007`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667164491

### F6 - CodeRabbit: widen the #2770 child-spec glob

- PR number: 3007
- Source review ID: 6023135871
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4666956418
- Concern: The #2770 file list uses `docs/issues/{open,closed}/27*-2758-*/ISSUE.md`, which does not match the #3004 and #3006 specs, although the close-out must repair every child spec's reference to the EPIC path.
- Solution: The list now uses `docs/issues/{open,closed}/*-2758-*/ISSUE.md`.
- Current-tree verification: `grep -c '27\*-2758' docs/issues/open/2770-2758-close-out-adr-and-epic/ISSUE.md` printed 0 at the fix head.
- Resolution reference: `docs(issues): [#2758] address da2ce7's review 6023338079 of PR 3007`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667164973

### F7 - Copilot: include the new children in the #2770 scope

- PR number: 3007
- Source review ID: 6023386734
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667150986
- Concern: The #2770 file list uses `docs/issues/{open,closed}/27*-2758-*/ISSUE.md`, which does not match the #3004 and #3006 specs, although the close-out must repair every child spec's reference to the EPIC path.
- Solution: The list now uses `docs/issues/{open,closed}/*-2758-*/ISSUE.md`.
- Current-tree verification: `grep -c '27\*-2758' docs/issues/open/2770-2758-close-out-adr-and-epic/ISSUE.md` printed 0 at the fix head.
- Resolution reference: `docs(issues): [#2758] address da2ce7's review 6023338079 of PR 3007`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667172635

### F8 - Greptile: repeated findings marked as original

- PR number: 3007
- Source review ID: 6023502560
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667242430
- Concern: The audit says F1, F5, F6 and F7 are one concern, but all four rows read `ORIGINAL`; `process-pr-review` records a later report of the same change as `RE_RAISE_OF:<FindingId>`.
- Solution: F5 (the Greptile thread at 14:10) is the earliest report, so F1, F6 and F7 now read `RE_RAISE_OF:F5`; IDs, dispositions, thread states and replies are unchanged.
- Current-tree verification: `grep -c '^| F[0-9]* | .* | RE_RAISE_OF:F5 |' docs/pr-reviews/pr-3007-review/PR-REVIEW.md` printed 3 at the fix head.
- Resolution reference: `docs(issues): [#2758] address round 2 of the PR 3007 review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667394172

### F9 - CodeRabbit: state whether the listed order is strict

- PR number: 3007
- Source review ID: 6023522118
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667257329
- Concern: The Delivery Strategy lists an order that puts wave 1 issues #2763 and #2767 after wave 2 issues, but does not say whether that order overrides the rule that wave 1 issues may run in parallel.
- Solution: The paragraph now says the order is a schedule the maintainer set (one implementation session at a time, bugs first), not a file dependency, and that for the listed issues it takes precedence over the wave 1 parallel rule.
- Current-tree verification: `grep -c 'takes precedence over the wave 1 rule' docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` printed 1 at the fix head.
- Resolution reference: `docs(issues): [#2758] address round 2 of the PR 3007 review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667394779

### F10 - Row 1 contradicts issue 11's table edit

- PR number: 3007
- Source review ID: 6023616767
- Reviewer finding ID: da2ce7 F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667332494
- Concern: Row 1 says no other issue edits the ADR's table, then names issue 11 updating the notes, which are the table's `Notes` column, while the EPIC and #2770 say issue 11 edits the table.
- Solution: Row 1 now reads "no issue but issue 11 edits its table (issue 13 amends its text)".
- Current-tree verification: `grep -c 'no issue but issue 11 edits its table' docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` printed 1 at the fix head.
- Resolution reference: `docs(issues): [#2758] address round 2 of the PR 3007 review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667395234

### F11 - The four glob rows are all ORIGINAL

- PR number: 3007
- Source review ID: 6023616767
- Reviewer finding ID: da2ce7 F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667332512
- Concern: The audit says F1, F5, F6 and F7 are one concern, but all four rows read `ORIGINAL`; `process-pr-review` records a later report of the same change as `RE_RAISE_OF:<FindingId>`.
- Solution: F5 (the Greptile thread at 14:10) is the earliest report, so F1, F6 and F7 now read `RE_RAISE_OF:F5`; IDs, dispositions, thread states and replies are unchanged.
- Current-tree verification: `grep -c '^| F[0-9]* | .* | RE_RAISE_OF:F5 |' docs/pr-reviews/pr-3007-review/PR-REVIEW.md` printed 3 at the fix head.
- Resolution reference: `docs(issues): [#2758] address round 2 of the PR 3007 review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667396184

### F12 - Later threads unanswered; the order's strictness

- PR number: 3007
- Source review ID: 6023616767
- Reviewer finding ID: da2ce7 F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667332521
- Concern: Two threads opened after the audit commit had no reply, row or resolution, and the order paragraph does not say whether it overrides the wave 1 parallel rule.
- Solution: Both threads have replies and rows (F8, F9), and the order paragraph states that it is a schedule which takes precedence over the wave 1 parallel rule for the listed issues.
- Current-tree verification: `grep -c 'takes precedence over the wave 1 rule' docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` printed 1 at the fix head.
- Resolution reference: `docs(issues): [#2758] address round 2 of the PR 3007 review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4667396693

### F13 - The Order column reads as the execution order

- PR number: 3007
- Source review ID: 6025006260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668432634
- Concern: The Subissues table's column is labeled `Order`, but the schedule in the Delivery Strategy runs issue 12 first and issue 11 last, so a reader may take `Order` for the execution order.
- Solution: The Subissues intro now says `Order` is a stable row number that the specs cite, not the execution order, and that the Delivery Strategy schedule sets it. The rows are not renumbered, because the child specs and #2770 refer to them by number.
- Current-tree verification: `grep -c 'is a stable row number that the specs cite' docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` printed 1 at the fix head.
- Resolution reference: `docs(issues): [#2758] say the EPIC Order column is a row number, not the schedule`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668611959

### F14 - The Order column reads as the execution order (row 13)

- PR number: 3007
- Source review ID: 6025006260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668432723
- Concern: The Subissues table's column is labeled `Order`, but the schedule in the Delivery Strategy runs issue 12 first and issue 11 last, so a reader may take `Order` for the execution order.
- Solution: The Subissues intro now says `Order` is a stable row number that the specs cite, not the execution order, and that the Delivery Strategy schedule sets it. The rows are not renumbered, because the child specs and #2770 refer to them by number.
- Current-tree verification: `grep -c 'is a stable row number that the specs cite' docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` printed 1 at the fix head.
- Resolution reference: `docs(issues): [#2758] say the EPIC Order column is a row number, not the schedule`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668612602

### F15 - The Order column reads as the execution order (schedule)

- PR number: 3007
- Source review ID: 6025006260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668432781
- Concern: The Subissues table's column is labeled `Order`, but the schedule in the Delivery Strategy runs issue 12 first and issue 11 last, so a reader may take `Order` for the execution order.
- Solution: The Subissues intro now says `Order` is a stable row number that the specs cite, not the execution order, and that the Delivery Strategy schedule sets it. The rows are not renumbered, because the child specs and #2770 refer to them by number.
- Current-tree verification: `grep -c 'is a stable row number that the specs cite' docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` printed 1 at the fix head.
- Resolution reference: `docs(issues): [#2758] say the EPIC Order column is a row number, not the schedule`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668613086

### F16 - The findings table rows start with a double pipe

- PR number: 3007
- Source review ID: 6025006260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668432871
- Concern: The findings table of this audit was said to start each row with `||`, which would add an empty column.
- Solution: None: every row of the findings table, including the header and the separator, starts with a single `|`; the `||` is likely the diff view's `+` prefix next to the pipe.
- Current-tree verification: `grep -c '^||' docs/pr-reviews/pr-3007-review/PR-REVIEW.md` printed 0 at the reviewed head.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668613568
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668613568

### F17 - The open child specs still give a total of 11

- PR number: 3007
- Source review ID: 6025458205
- Reviewer finding ID: da2ce7 F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668801169
- Concern: The EPIC cites "Order 11 of 13" as the form the specs use, but the open specs of #2763, #2766, #2767 and #2769 still say "of 11".
- Solution: Those four specs now say "Order N of 13"; the closed specs stay as history.
- Current-tree verification: `cat docs/issues/open/*-2758-*/ISSUE.md | grep -c 'Order [0-9]* of 11'` printed 0 at the fix head.
- Resolution reference: `docs(issues): [#2758] update the EPIC total in the open child specs to 13`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3007#discussion_r4668845107

## Processing Log

- 2026-11-23 14:24 UTC - Reviews arrived on the first head: Greptile (14:10:28), CodeRabbit (14:11:52) and da2ce7 (14:24:56).
- 2026-11-23 14:29 UTC - Pushed the fix commit for all four da2ce7 findings, then replied on the six threads seen so far (14:29:14 to 14:29:26). Copilot's review (14:28:08) arrived during the fix.
- 2026-11-23 14:29 UTC - Resolved the open threads at 14:29:42. The resolve step also closed the Copilot thread (F7) before it had a reply, a breach of reply-before-resolve; the reply followed at 14:29:55, with the fix already on the head. Lesson: list the threads again right before resolving.
- 2026-11-23 14:30 UTC - Created this audit record after the replies and the resolution.
- 2026-11-23 14:42 UTC - Round 2: Greptile (14:35:39), CodeRabbit (14:36:56) and da2ce7 (14:42:55, APPROVED with F5 to F7) reviewed the audit head.
- 2026-11-23 14:48 UTC - Pushed the round 2 fix, listed the threads again, replied on the five threads (14:48:03 and before) and resolved the four left open (14:48:18); CodeRabbit had resolved its own.
- 2026-11-23 14:48 UTC - Added rows F8 to F12 after the replies and the resolution.
- 2026-11-23 16:08 UTC - Round 3: Copilot review 6025006260 (16:08:45) on the round 2 audit head, with four threads.
- 2026-11-23 16:23 UTC - The first commit of the fix failed when the `/tmp` volume ran out of disk quota; after space came back, pushed the fix, replied on the four threads (16:23:03 and before) and resolved them (16:23:09).
- 2026-11-23 16:23 UTC - Added rows F13 to F16 after the replies and the resolution.
- 2026-11-23 16:41 UTC - Round 4: da2ce7 review 6025458205 (16:37:55, APPROVED, one Nit); pushed the fix, replied (16:41:33) and resolved the thread (16:41:35).
- 2026-11-23 16:41 UTC - Added row F17 after the reply and the resolution.

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
