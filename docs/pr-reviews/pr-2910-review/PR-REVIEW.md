---
pr-number: 2910
pr-url: https://github.com/torrust/torrust-tracker/pull/2910
last-updated-utc: "2026-11-17 15:31"
---

<!-- skill-link: process-pr-review -->

# PR #2910 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2910.

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

Round 1: da2ce7 review 5939730094 (CHANGES_REQUESTED) with findings F1 to F7; the reviewer's IDs are this audit's IDs. Copilot review 5939709564 (COMMENTED) reported no issue and has no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2910-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2910-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2910-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2910-f4 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2910-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2910-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2910-f7 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Order 6 spec reads as approved while its approach is undecided

- PR number: 2910
- Source review ID: 5939730094
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4600533320
- Concern: The #2906 spec ticks the approval checkpoint and its log says the draft was approved, while Scope, Architectural Decisions and T2 still read as if the loopback-helper extension would be decided in spec review; an implementer would read the extension as approved, contradicting the EPIC row 6 and the EPIC Decisions. The #2908 Bug-Fix Process step 2 has the same shape.
- Solution: The approval checkpoint stays (the maintainer approved the draft). Scope, Architectural Decisions and T2 now say the approval left the extension undecided and that the maintainer decides it before T2; a progress-log entry records it. #2908 step 2 now schedules the boundary decision in T2.
- Current-tree verification: `grep -n 'still undecided\|did not approve the extension\|Starts after the maintainer decides' docs/issues/open/2906-*/ISSUE.md` finds the three statements; `grep -n 'decide in T2' docs/issues/open/2908-*/ISSUE.md` finds step 2.
- Resolution reference: docs(issues): [#2392] keep the #2906 approach and #2908 test boundary open until T2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4601751714

### F2 - Order 8 note names the wrong sibling order

- PR number: 2910
- Source review ID: 5939730094
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4600533333
- Concern: Order 8's note says it touches the qBittorrent E2E runner "like order 3", but order 3 excludes that runner; order 4 now covers its default tag.
- Solution: The note now says "like order 4".
- Current-tree verification: `grep -n 'like order 4' docs/issues/open/2392-test-isolation/EPIC.md` finds order 8's row; `like order 3` is no longer present.
- Resolution reference: docs(issues): [#2392] place orders 6 to 8 in the EPIC delivery plan
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4601752263

### F3 - Order 2 row announces a close-out that already merged

- PR number: 2910
- Source review ID: 5939730094
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4600533344
- Concern: Order 2's row and the audit's last checkpoint say the maintainer closes #2394 when the close-out PR merges, but PR #2898 has merged.
- Solution: Both now record that PR #2898 has merged and that #2394 stays open until the maintainer closes it.
- Current-tree verification: `gh-bot issue view 2394 --json state` reports OPEN; both sentences name PR #2898 as merged.
- Resolution reference: docs(issues): [#2392] place orders 6 to 8 in the EPIC delivery plan
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4601752698

### F4 - First commit body matches the closing-keyword scan

- PR number: 2910
- Source review ID: 5939730094
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4600533358
- Concern: The body of the first commit wraps "point the closed" / "#2394 records", which the folded scan matches as a closing keyword for #2394.
- Solution: With the maintainer's approval, the commit body was reworded to "point the #2394 records in docs/issues/closed/ at the new specs" through a non-interactive rebase and pushed with `git-bot-push --force-with-lease`.
- Current-tree verification: `git diff` between the pre-rewrite head and the new head was empty; a folded scan of all branch commit bodies for `(close[sd]?|fix(e[sd])?|resolve[sd]?)\s+#N` prints nothing.
- Resolution reference: docs(issues): [#2392] add issue specifications for #2906, #2907 and #2908
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4601753319

### F5 - Inventory row C3 is stale

- PR number: 2910
- Source review ID: 5939730094
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4600533373
- Concern: Inventory row C3 still says #2396 covers it "once its scope is extended" and that its scope names only one default, which this PR changed.
- Solution: Row C3 now says #2396's scope was extended to both defaults; the stale clause is removed.
- Current-tree verification: `grep -n 'whose scope was extended to both defaults' docs/issues/closed/2394-*/inventory.md` finds row C3.
- Resolution reference: docs(issues): [#2392] refresh the #2394 records and evidence timestamps
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4601753867

### F6 - Three changed files keep their old timestamp

- PR number: 2910
- Source review ID: 5939730094
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4600533381
- Concern: The #2906 and #2907 evidence files and the audit's inventory changed without a `last-updated-utc` bump.
- Solution: All three now carry `2026-11-17 15:22`.
- Current-tree verification: `grep -n last-updated-utc` on the three files shows `2026-11-17 15:22`.
- Resolution reference: docs(issues): [#2392] refresh the #2394 records and evidence timestamps
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4601754430

### F7 - Orders 6 to 8 have no phase or delivery line

- PR number: 2910
- Source review ID: 5939730094
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4600533388
- Concern: No phase or Delivery Strategy sentence takes up orders 6 to 8.
- Solution: The Delivery Strategy says orders 6 and 7 gate AC2 and are independent; order 8 belongs to Phase 3, whose exit criteria now name orders 3, 4 and 8.
- Current-tree verification: `grep -n 'orders 3, 4 and 8\|gate AC2' docs/issues/open/2392-test-isolation/EPIC.md` finds both lines.
- Resolution reference: docs(issues): [#2392] place orders 6 to 8 in the EPIC delivery plan
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4601754919

## Processing Log

- 2026-11-17 15:29 UTC - Round 1: da2ce7 review 5939730094. Committed the F1, F2/F3/F7 and F3/F5/F6 fixes, reworded the first commit's body for F4 with the maintainer's approval, and force-pushed head `ae1f1e2d6` with `git-bot-push --force-with-lease`. Then replied to the seven threads (15:29:17Z to 15:29:32Z). This record was created after the replies, so the replies came before the audit started.
- 2026-11-17 15:31 UTC - The validator on the worktree record reported 7 rows, 1 log entry, 0 failures. Resolved the seven threads at about 15:30:32Z, selecting only unresolved threads whose last comment is a `josecelano-bot` reply. A GraphQL refresh found no unresolved thread. Committed this record.

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
