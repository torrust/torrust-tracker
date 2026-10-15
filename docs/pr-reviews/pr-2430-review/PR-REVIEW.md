---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/discussions/2429-templated-documentation/20261009-templated-documentation-design/initial-proposal.md
---

<!-- skill-link: process-pr-review -->

# PR #2430 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2430>.

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

Copilot review 5401297360 (round 1, state `COMMENTED`, submitted 2026-10-03 14:49 UTC on head `docs(issues): [#2429] add the templated documentation EPIC specification`, "Lite" effort) left three inline comments. Its body opens with a `[!WARNING]` alert: "Copilot couldn't run its full agentic review because it didn't start before the timeout. Make sure your repository has a runner available, or add a `copilot-code-review.yml` file specifying one with the `runs-on` attribute. See the [docs](https://gh.io/AA11tgch) for more details." The rest of the body is an overview, so it creates no additional finding.

The comments carry no `[Severity]` bracket. The review's badge markup rates all three `Medium severity`; as in the PR #2421 audit, they are recorded as `Minor (inferred)`. F3 repeats F2's claim for a different table and asks for a change to other rows, so it is an original finding, not a re-raise. Thread states are recorded as captured after the replies: all three threads are resolved, recorded as `RESOLVED` for the fixed F1 and `SUPERSEDED` for the no-change F2 and F3, as the Status Values prescribe.

Review 5469057611 by the decision owner (`josecelano`, round 2, state `CHANGES_REQUESTED`, submitted 2026-10-09 10:51 UTC on head `docs(pr-reviews): [#2429] add the PR #2430 review audit record`) has no inline comments. Its body opens with a verdict, that an EPIC with a binding design is premature, and answers the PR's timing question (after the v4.0.0 release and the current work in progress); neither requests a change, so neither makes a row. Its three numbered requests carry four independently actionable assertions, F4 to F7, whose severity and category are inferred from the prose; F4 is a `Blocker` because the review requests changes to keep the EPIC specification from merging. Having no thread, all four are `NON_RESOLVABLE`. The decision owner's conversation comment 6145187551 of 2026-10-13 repeats these requests as five steps and adds one, retitling the PR to match the rework; a conversation comment is not a review, so that step is recorded in the Processing Log, not as a row.

Review 5506433536 by the review bot (round 3, state `COMMENTED`, submitted 2026-10-13 16:00 UTC on head `docs(pr-reviews): [#2429] add the PR #2430 review audit record`) has no inline comments and opens no thread. Its body is an overview: a verdict that summarizes its two items, "0 open findings", and three items resolved since its last review, which are the F1 to F3 threads and make no row. Its two "previously missed" items sit in collapsed sections whose full content is retrievable and actionable, so they are F8 and F9. The badge markup rates both `Medium severity`, recorded as `Minor (inferred)` as for F1 to F3. F8 asks for the change F4 records, against the same current tree, so it is `RE_RAISE_OF:F4` with F4's disposition; F9 asks for this record to normalize review 5469057611 and the rework, and is fixed by the commit that carries this record. Having no thread, both are `NON_RESOLVABLE`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2430-f1` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2430-f2` | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | `review-finding:pr-2430-f3` | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F4 | `review-finding:pr-2430-f4` | Human | Blocker (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F5 | `review-finding:pr-2430-f5` | Human | Major (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F6 | `review-finding:pr-2430-f6` | Human | Minor (inferred) | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F7 | `review-finding:pr-2430-f7` | Human | Suggestion (inferred) | documentation | ORIGINAL | FOLLOW_UP | NON_RESOLVABLE |
| F8 | `review-finding:pr-2430-f8` | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F4 | FIXED | NON_RESOLVABLE |
| F9 | `review-finding:pr-2430-f9` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The counts sentence has no commit anchor

- PR number: 2430
- Source review ID: 5401297360
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173600206>
- Concern: The sentence at `docs/issues/open/2429-templated-documentation/EPIC.md:51` quotes concrete counts (970 Markdown files, 272 specifications, and others) without anchoring them to a commit, so the EPIC is itself prone to the drift it describes; the reviewer suggested the "as of" framing the document already uses.
- Solution: Anchored the sentence to `develop` `eb96d2957`, the framing the same section already uses ("More evidence at `develop` `eb96d2957`:"). No number changed. `last-updated-utc` was bumped; no Progress Log line was added, because the EPIC template asks for one line per meaningful update and this wording anchor changes no scope, design or status.
- Current-tree verification: `sed -n '51p' docs/issues/open/2429-templated-documentation/EPIC.md` prints the sentence opening "At `develop` `eb96d2957` the tree tracks 970 Markdown files"; `git diff` of the fix commit changes only that sentence and `last-updated-utc`.
- Resolution reference: `docs(discussions): [#2429] keep the EPIC text as the proposal's source material`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173637906>

### F2 - Subissues table rows reported as starting with a double pipe

- PR number: 2430
- Source review ID: 5401297360
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173600228>
- Concern: The reviewer reported that the Subissues table rows (anchored at `docs/issues/open/2429-templated-documentation/EPIC.md:139`) start with `||`, which Markdown reads as an extra empty leading column, and asked for a single leading `|`.
- Solution: no change. No row in the file starts with `||`; every table row starts with a single `|`, so the requested change has nothing to apply to.
- Current-tree verification: at the branch head, `grep -c '^||' docs/issues/open/2429-templated-documentation/EPIC.md` prints `0` and `grep -c '^|' docs/issues/open/2429-templated-documentation/EPIC.md` prints `13` (the Subissues and Acceptance Verification tables, header and separator rows included); `sed -n '139p'` prints the row starting `| 6 |`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173638021>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173638021>

### F3 - Acceptance Verification table rows reported as starting with a double pipe

- PR number: 2430
- Source review ID: 5401297360
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173600238>
- Concern: The reviewer reported the same `||` row prefix in the Acceptance Verification table (anchored at `docs/issues/open/2429-templated-documentation/EPIC.md:213`) and asked for a single leading `|` on every row.
- Solution: no change. No row in the file starts with `||`; the three Acceptance Verification rows, its header and its separator each start with a single `|`.
- Current-tree verification: the F2 commands, run at the same head, print `0` and `13`; `sed -n '213p' docs/issues/open/2429-templated-documentation/EPIC.md` prints the row starting `| AC8-AC10 |`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173638121>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173638121>

### F4 - Keep the EPIC text as the discussion's initial proposal and remove the EPIC specification

- PR number: 2430
- Source review ID: 5469057611
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2430#pullrequestreview-5469057611>
- Concern: An EPIC with a binding design is premature; the reviewer asked for the EPIC text to be added verbatim as `initial-proposal.md` in the discussion folder that PR #2501 opened, and for `docs/issues/open/2429-templated-documentation/EPIC.md` to be removed (request 1).
- Solution: The branch was rebuilt from `develop` and no longer adds the EPIC specification. Its text is added as `docs/discussions/2429-templated-documentation/20261009-templated-documentation-design/initial-proposal.md`, frontmatter included, under a source-material note that says what was added and why nothing else had to change to pass the linters.
- Current-tree verification: `git ls-tree -r HEAD -- docs/issues/open/2429-templated-documentation` prints nothing; with the note's eight lines (the blockquote after the H1 and the blank line after it) removed, `initial-proposal.md` is byte-identical to the EPIC at the reviewed head.
- Resolution reference: `docs(discussions): [#2429] keep the EPIC text as the proposal's source material`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2430#issuecomment-6191069336>

### F5 - Add the PR author's positions on Q1 to Q9

- PR number: 2430
- Source review ID: 5469057611
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2430#pullrequestreview-5469057611>
- Concern: The reviewer asked for the PR author's positions on topics Q1 to Q9 of the discussion, with new topics optional (request 1).
- Solution: One entry per topic, appended under each topic's `#### Positions` after the decision owner's entry, or in place of "No entries yet." under Q4 and Q6. No new topic: every position fits an existing one. No other line of the discussion README changes.
- Current-tree verification: `grep -c '^##### Q[1-9] - .*, 2026-10-15, PR #2430$'` on the discussion README prints `9`; with the nine entries removed and "No entries yet." restored under Q4 and Q6, the README is byte-identical to its `develop` version.
- Resolution reference: `docs(discussions): [#2429] add my round on templated documentation`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2430#issuecomment-6191069336>

### F6 - Retitle issue #2429 and drop its EPIC label

- PR number: 2430
- Source review ID: 5469057611
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2430#pullrequestreview-5469057611>
- Concern: The reviewer asked for issue #2429 to be retitled and its `EPIC` label dropped; it stays the issue the discussion informs (request 2).
- Solution: An issue edit outside the repository tree, made when this rework is pushed: the issue is retitled as the discussion's subject, its `EPIC` label is removed, and its body points to the discussion and its initial proposal.
- Current-tree verification: N/A in the tree; the issue title and labels are checked on GitHub after the edit.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2430#issuecomment-6191069336>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2430#issuecomment-6191069336>

### F7 - Link the PR author's implementation once it is published

- PR number: 2430
- Source review ID: 5469057611
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2430#pullrequestreview-5469057611>
- Concern: The reviewer asked for a link to the PR author's own implementation of the approach once it is published, as evidence for the discussion (request 3).
- Solution: The implementation is not yet published on a public `develop` branch, so nothing is linked; the link is added in a later round of the discussion once it is.
- Current-tree verification: no file on the branch links the implementation.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2430#issuecomment-6191069336>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2430#issuecomment-6191069336>

### F8 - Remove the binding EPIC specification and keep its text as the initial proposal

- PR number: 2430
- Source review ID: 5506433536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2430#pullrequestreview-5506433536>
- Concern: A "previously missed" item anchored at `docs/issues/open/2429-templated-documentation/EPIC.md:69` reported that review 5469057611 and comment 6145187551 supersede the binding EPIC specification, and asked for the file to be removed and its text kept as `initial-proposal.md` in the discussion folder; it is the change F4 records.
- Solution: Re-raise of F4, resolved by F4's change; nothing further.
- Current-tree verification: F4's checks hold at the branch head: `git ls-tree -r HEAD -- docs/issues/open/2429-templated-documentation` prints nothing, and `initial-proposal.md` without its note is byte-identical to the EPIC at the reviewed head.
- Resolution reference: `docs(discussions): [#2429] keep the EPIC text as the proposal's source material`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2430#issuecomment-6191069336>

### F9 - Record the decision owner's review and the rework in the audit

- PR number: 2430
- Source review ID: 5506433536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2430#pullrequestreview-5506433536>
- Concern: A "previously missed" item anchored at `docs/pr-reviews/pr-2430-review/PR-REVIEW.md:50` reported that the audit stopped at the round-1 review although review 5469057611 and comment 6145187551 carry actionable rework requests, and asked for them to be normalized with details, dispositions and append-only Processing Log entries.
- Solution: Review 5469057611 is normalized as F4 to F7, the PR retitle that comment 6145187551 adds is recorded in the Processing Log, and this review is normalized as F8 and F9, each row with its detail entry and the Processing Log appended, not rewritten.
- Current-tree verification: the audit validator reports 9 rows, each with its detail entry, and the Processing Log in chronological order.
- Resolution reference: `docs(pr-reviews): [#2429] record the rework of PR #2430 in its audit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2430#issuecomment-6191069336>

## Processing Log

- 2026-10-03 14:52 UTC - Started audit for Copilot review 5401297360 from the captured REST reviews, REST review comments and GraphQL thread list: three inline threads, opened by review comments 4173600206 (F1), 4173600228 (F2) and 4173600238 (F3) and all `resolved=false`, no reviewer finding IDs, no human review, and a body with no independently actionable assertion. Normalized as F1-F3.
- 2026-10-03 14:53 UTC - Re-derived the three claims at the branch head: F1 holds; F2 and F3 do not (`grep -c '^||'` prints `0`). Committed the F1 fix as `docs(issues): [#2429] anchor the counts to the develop head`.
- 2026-10-03 14:54 UTC - Recorded the audit with the thread states as captured. Replies are prepared for all three threads (F1 `FIXED`; F2 and F3 in the prescribed `Superseded by <FindingId>:` form, each naming itself as an original no-change finding) and are posted after the fix and this record are pushed; their URLs, the F2 and F3 resolution references and the final thread states are recorded then.
- 2026-10-03 14:54 UTC - Ran `validate-audit-record.py --pr-number 2430 --base eb96d2957` against the captured round-1 review comments: 3 rows, 3 log entries, 5 failures, all of them the pending replies (no Reply URL yet on F1, F2 and F3; the F2 and F3 resolution references await their reply URLs). Review IDs, detail entries, the F1 commit subject and log order pass; the validator must exit `0` once the reply URLs are recorded.
- 2026-10-03 15:01 UTC - The replies were posted word for word at 15:00:13 (F1), 15:00:15 (F2) and 15:00:17 UTC (F3), each on its own source thread, after the fix commit was pushed. The thread opened by review comment 4173600206 was then resolved with reply <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173637906>, the one opened by 4173600228 with reply <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173638021>, and the one opened by 4173600238 with reply <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173638121>; a fresh GraphQL capture reports all three `resolved=true`. Recorded the reply URLs, the F2 and F3 resolution references and the thread states. The earlier record commit, made before the replies existed, was replaced by this one, so the committed record carries the posted URLs.
- 2026-10-03 15:01 UTC - Ran `validate-audit-record.py --pr-number 2430 --base eb96d2957` against the fresh review-comment capture, replies included: `{"status": "ok", "rows": 3, "log_entries": 5, "failures": 0}`.
- 2026-10-15 10:40 UTC - Started round 2 from the captured REST reviews and review comments: review 5469057611 (`CHANGES_REQUESTED`, no inline comments, no reviewer finding IDs) normalized as F4-F7 from its three numbered requests; its opening verdict and its answer on timing request no change. The decision owner's conversation comment 6145187551 repeats the requests and adds a retitle of the PR, recorded in the entry below.
- 2026-10-15 10:40 UTC - Rebuilt the branch from `develop` instead of rebasing it: the EPIC specification and its fix commit are no longer carried. Committed F4 as `docs(discussions): [#2429] keep the EPIC text as the proposal's source material` and F5 as `docs(discussions): [#2429] add my round on templated documentation`. F1's fix commit, `docs(issues): [#2429] anchor the counts to the develop head`, is therefore not on the branch; the anchored sentence is carried verbatim at line 59 of `initial-proposal.md`, so F1's Resolution reference now names the commit that carries it. F1 to F3 keep citing `docs/issues/open/2429-templated-documentation/EPIC.md` lines 51, 139 and 213 as they stood when raised; the same lines are 59, 147 and 221 of `initial-proposal.md`. The round-1 head in the Findings introduction is named by its commit subject instead of its id, which is not on `develop`.
- 2026-10-15 10:41 UTC - The PR is retitled from `docs(issues): [#2429] add the templated documentation EPIC specification` to `docs(discussions): [#2429] add my round on templated documentation`, as comment 6145187551 asks, and issue #2429 is retitled without its `EPIC` label (F6), both when this rework is pushed; the link F7 asks for waits until the implementation is published. F6 and F7, and the Reply URLs of F4 to F7, wait for the PR conversation response that states this; its URL is recorded once it is posted.
- 2026-10-15 10:41 UTC - Ran `validate-audit-record.py --pr-number 2430 --base eb96d2957` against the captured review comments: `{"status": "failed", "rows": 7, "log_entries": 10, "failures": 2}`. Both failures are the pending F6 and F7 resolution references; review IDs, detail entries, the F1, F4 and F5 commit subjects and log order pass. The validator must exit `0` once the response URL is recorded.
- 2026-10-15 10:49 UTC - Started round 3 from the captured REST reviews, review comments and thread list: review 5506433536 by the review bot (`COMMENTED`) has no review comment, and the thread list still holds only the three round-1 threads, all resolved. Its "0 open findings" and its three resolved items (the F1 to F3 threads) make no row; its two "previously missed" items, retrievable in full from the review body, are normalized as F8 (`RE_RAISE_OF:F4`, `FIXED` by F4's commit) and F9 (`ORIGINAL`, `FIXED` by the commit that carries this record). Neither has a thread to reply on, so each is answered by the PR conversation response the skill prescribes for review bodies; one response may cover reviews 5469057611 and 5506433536 only if it names both review IDs and F4 to F9 with their dispositions and resolution references, and its URL then goes in each of those rows.
- 2026-10-15 10:49 UTC - Ran `validate-audit-record.py --pr-number 2430 --base eb96d2957` against the captured review comments: `{"status": "failed", "rows": 9, "log_entries": 12, "failures": 2}`. Both failures are still the pending F6 and F7 resolution references; F8 and F9, including F9's subject for this record's own commit, pass.
- 2026-10-15 11:34 UTC - Posted the PR conversation response (comment 6191069336) to reviews 5469057611 and 5506433536 after the rework was pushed, and recorded its URL as the Reply URL of F4 to F9 and as the resolution reference of F6 (the issue #2429 edit: retitled `Templated documentation`, the `EPIC` label removed, the body replaced) and F7 (the follow-up link, which waits for publication).

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
