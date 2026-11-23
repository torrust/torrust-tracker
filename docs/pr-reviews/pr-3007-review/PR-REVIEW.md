---
pr-number: 3007
pr-url: https://github.com/torrust/torrust-tracker/pull/3007
last-updated-utc: "2026-11-23 14:30"
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

Round 1, on the first head: Greptile review 6023114012 (one thread), CodeRabbit review 6023135871 (one thread), da2ce7 review 6023338079 (F1 to F4) and Copilot review 6023386734 (one thread, reviewer ID F1). The Greptile, CodeRabbit and Copilot threads raise the same concern as da2ce7's F1; they are recorded as F5, F6 and F7, each fixed by the same change.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3007-f1 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3007-f2 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3007-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3007-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3007-f5 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3007-f6 | Unknown | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3007-f7 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

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

## Processing Log

- 2026-11-23 14:24 UTC - Reviews arrived on the first head: Greptile (14:10:28), CodeRabbit (14:11:52) and da2ce7 (14:24:56).
- 2026-11-23 14:29 UTC - Pushed the fix commit for all four da2ce7 findings, then replied on the six threads seen so far (14:29:14 to 14:29:26). Copilot's review (14:28:08) arrived during the fix.
- 2026-11-23 14:29 UTC - Resolved the open threads at 14:29:42. The resolve step also closed the Copilot thread (F7) before it had a reply, a breach of reply-before-resolve; the reply followed at 14:29:55, with the fix already on the head. Lesson: list the threads again right before resolving.
- 2026-11-23 14:30 UTC - Created this audit record after the replies and the resolution.

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
