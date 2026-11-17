---
pr-number: 2910
pr-url: https://github.com/torrust/torrust-tracker/pull/2910
last-updated-utc: "2026-11-17 15:56"
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

Round 1: da2ce7 review 5939730094 (CHANGES_REQUESTED) with findings F1 to F7; the reviewer's IDs are this audit's IDs. Copilot review 5939709564 (COMMENTED) reported no issue and has no row. Round 2: da2ce7 review 5941589792 (APPROVED) with findings F8 and F9; the reviewer's IDs are this audit's IDs. Copilot review 5941743705 (COMMENTED, at the round 1 fix head) with three findings without reviewer IDs, as audit findings F10 to F12.

Round 1 cited commit ids from before the history rewrite (F4). Mapped to commit subjects:

- `ba0df8f14`: `docs(issues): [#2392] add issue specifications for #2906, #2907 and #2908`
- `1ee2afe4d`: `docs(issues): [#2392] add orders 6 to 8, close AC4 and extend #2396's scope`

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2910-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2910-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2910-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2910-f4 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2910-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2910-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2910-f7 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2910-f8 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2910-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2910-f10 | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F11 | review-finding:pr-2910-f11 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2910-f12 | Copilot | Nit (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |

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

### F8 - Audit cites a PR-branch commit id and maps no rewritten ids

- PR number: 2910
- Source review ID: 5941589792
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4601916124
- Concern: The round 1 log entry named the branch head by its id, which the `open-pull-request` commit-id check reports; and after the rewrite the audit did not map the ids that round 1 comments cite.
- Solution: The round 1 entry now names the head by its commit subject, a later log entry records that in-place correction, and the Findings section maps `ba0df8f14` and `1ee2afe4d` to their subjects.
- Current-tree verification: The `open-pull-request` commit-id check prints only `1ee2afe4d` and `ba0df8f14`, the pre-rewrite ids that the mapping must cite; `git branch -a --contains` finds no branch with either.
- Resolution reference: docs(pr-reviews): [#2392] cite commit subjects in the PR #2910 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4602003760

### F9 - Description still says two commits and omits the rewrite

- PR number: 2910
- Source review ID: 5941589792
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4601916146
- Concern: The PR description said the pre-commit hook passes "on both commits" while the branch has six, and it did not say that the history was rewritten.
- Solution: The description now says the hook passes on each of the branch's commits and has one paragraph on the approved rewrite (F4), the unchanged tree and the id mapping in this audit.
- Current-tree verification: `gh-bot pr view 2910 --json body` shows the new Validation line and the rewrite paragraph.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2910#issuecomment-6952991621
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4601975878

### F10 - Specs share the spec PR's branch name

- PR number: 2910
- Source review ID: 5941743705
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4602038694
- Concern: The #2906, #2907 and #2908 specs set `branch:` to this PR's branch instead of an issue-specific implementation branch, which tooling could read as ambiguous.
- Solution: No change. The `create-issue` skill's Optional Step 6 (Spec-Only PR) sets `branch:` to the `-spec` branch of the spec PR, and all three specs came in through this one spec PR. Each implementation session sets its own `{issue-number}-{short-description}` branch when it starts.
- Current-tree verification: `.claude/skills/create-issue/SKILL.md` Optional Step 6 (Spec-Only PR) says the frontmatter `branch:` value is the same `-spec` branch name; the three specs carry `branch: "2392-audit-follow-up-issues-spec"`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4602077202
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4602077202

### F11 - Undecided approach described as settled

- PR number: 2910
- Source review ID: 5941743705
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4602038791
- Concern: The #2906 Scope says the approach is undecided and then describes it in definitive terms ("It binds…").
- Solution: The candidate is now written in the conditional ("If chosen, it would bind…"), and a progress-log entry records it.
- Current-tree verification: `grep -n 'If chosen, it would bind' docs/issues/open/2906-*/ISSUE.md` finds the Scope line.
- Resolution reference: docs(issues): [#2392] describe the #2906 candidate approach in the conditional
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4602102061

### F12 - Drafted-in checkpoint names the old drafts path

- PR number: 2910
- Source review ID: 5941743705
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4602038876
- Concern: The checkpoint names a `docs/issues/drafts/` path that no longer exists, which can be misread as the spec's location.
- Solution: No change. The checkpoint is the template's wording and records where the spec was drafted; `spec-path` carries the current location, and archived specs keep the drafts path the same way.
- Current-tree verification: `docs/templates/ISSUE.md` line 136 has the checkpoint with the drafts path; `docs/issues/closed/2157-2003-require-documented-clippy-allows/ISSUE.md` line 89 keeps it.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4602077793
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2910#discussion_r4602077793

## Processing Log

- 2026-11-17 15:29 UTC - Round 1: da2ce7 review 5939730094. Committed the F1, F2/F3/F7 and F3/F5/F6 fixes, reworded the first commit's body for F4 with the maintainer's approval, and force-pushed the branch, ending at `docs(issues): [#2392] refresh the #2394 records and evidence timestamps`, with `git-bot-push --force-with-lease`. Then replied to the seven threads (15:29:17Z to 15:29:32Z). This record was created after the replies, so the replies came before the audit started.
- 2026-11-17 15:31 UTC - The validator on the worktree record reported 7 rows, 1 log entry, 0 failures. Resolved the seven threads at about 15:30:32Z, selecting only unresolved threads whose last comment is a `josecelano-bot` reply. A GraphQL refresh found no unresolved thread. Committed this record.
- 2026-11-17 15:44 UTC - Round 2: da2ce7 review 5941589792 (APPROVED). Edited the PR description for its F9 (15:44:37Z). For its F8, corrected the 15:29 entry in place: it named the branch head by id (`force-pushed head <id>`), and now names it by its commit subject. Added the id mapping to the Findings section.
- 2026-11-17 15:48 UTC - Committed and pushed the F8 fix (`docs(pr-reviews): [#2392] cite commit subjects in the PR #2910 audit`). Posted the F9 PR conversation response (15:46:02Z), then replied to the F9 thread (15:46:10Z) and the F8 thread (15:48:05Z). Added rows F8 and F9.
- 2026-11-17 15:49 UTC - The validator on the worktree record reported 9 rows, 4 log entries, 0 failures. Resolved the F8 and F9 threads (15:48:37Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Committed these entries.
- 2026-11-17 15:55 UTC - Copilot review 5941743705 (at the round 1 fix head) with F10 to F12. Replied to F10 and F12 with no change (15:53:36Z and 15:53:39Z). Committed and pushed the F11 fix (`docs(issues): [#2392] describe the #2906 candidate approach in the conditional`), then replied to F11 (15:55:29Z).
- 2026-11-17 15:56 UTC - The F10 reply named the skill step as "Step 4, Open Spec PR"; the rule is in Optional Step 6 (Spec-Only PR). Posted a correction on the F10 thread (15:56:16Z) and named the right step in F10's detail entry. The validator on the worktree record reported 12 rows, 6 log entries, 0 failures. Resolved the three Copilot threads (15:56:35Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Committed these entries.

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
