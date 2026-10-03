---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2429-templated-documentation/EPIC.md
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

Copilot review 5401297360 (round 1, state `COMMENTED`, submitted 2026-10-03 14:49 UTC on head `fba5328b0`, "Lite" effort) left three inline comments. Its body opens with a `[!WARNING]` alert: "Copilot couldn't run its full agentic review because it didn't start before the timeout. Make sure your repository has a runner available, or add a `copilot-code-review.yml` file specifying one with the `runs-on` attribute. See the [docs](https://gh.io/AA11tgch) for more details." The rest of the body is an overview, so it creates no additional finding.

The comments carry no `[Severity]` bracket. The review's badge markup rates all three `Medium severity`; as in the PR #2421 audit, they are recorded as `Minor (inferred)`. F3 repeats F2's claim for a different table and asks for a change to other rows, so it is an original finding, not a re-raise. Thread states are recorded as captured after the replies: all three threads are resolved, recorded as `RESOLVED` for the fixed F1 and `SUPERSEDED` for the no-change F2 and F3, as the Status Values prescribe.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2430-f1` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2430-f2` | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | `review-finding:pr-2430-f3` | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The counts sentence has no commit anchor

- PR number: 2430
- Source review ID: 5401297360
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173600206>
- Concern: The sentence at `docs/issues/open/2429-templated-documentation/EPIC.md:51` quotes concrete counts (970 Markdown files, 272 specifications, and others) without anchoring them to a commit, so the EPIC is itself prone to the drift it describes; the reviewer suggested the "as of" framing the document already uses.
- Solution: Anchored the sentence to `develop` `eb96d2957`, the framing the same section already uses ("More evidence at `develop` `eb96d2957`:"). No number changed. `last-updated-utc` was bumped; no Progress Log line was added, because the EPIC template asks for one line per meaningful update and this wording anchor changes no scope, design or status.
- Current-tree verification: `sed -n '51p' docs/issues/open/2429-templated-documentation/EPIC.md` prints the sentence opening "At `develop` `eb96d2957` the tree tracks 970 Markdown files"; `git diff` of the fix commit changes only that sentence and `last-updated-utc`.
- Resolution reference: `docs(issues): [#2429] anchor the counts to the develop head`
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

## Processing Log

- 2026-10-03 14:52 UTC - Started audit for Copilot review 5401297360 from the captured REST reviews, REST review comments and GraphQL thread list: three inline threads, opened by review comments 4173600206 (F1), 4173600228 (F2) and 4173600238 (F3) and all `resolved=false`, no reviewer finding IDs, no human review, and a body with no independently actionable assertion. Normalized as F1-F3.
- 2026-10-03 14:53 UTC - Re-derived the three claims at the branch head: F1 holds; F2 and F3 do not (`grep -c '^||'` prints `0`). Committed the F1 fix as `docs(issues): [#2429] anchor the counts to the develop head`.
- 2026-10-03 14:54 UTC - Recorded the audit with the thread states as captured. Replies are prepared for all three threads (F1 `FIXED`; F2 and F3 in the prescribed `Superseded by <FindingId>:` form, each naming itself as an original no-change finding) and are posted after the fix and this record are pushed; their URLs, the F2 and F3 resolution references and the final thread states are recorded then.
- 2026-10-03 14:54 UTC - Ran `validate-audit-record.py --pr-number 2430 --base eb96d2957` against the captured round-1 review comments: 3 rows, 3 log entries, 5 failures, all of them the pending replies (no Reply URL yet on F1, F2 and F3; the F2 and F3 resolution references await their reply URLs). Review IDs, detail entries, the F1 commit subject and log order pass; the validator must exit `0` once the reply URLs are recorded.
- 2026-10-03 15:01 UTC - The replies were posted word for word at 15:00:13 (F1), 15:00:15 (F2) and 15:00:17 UTC (F3), each on its own source thread, after the fix commit was pushed. The thread opened by review comment 4173600206 was then resolved with reply <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173637906>, the one opened by 4173600228 with reply <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173638021>, and the one opened by 4173600238 with reply <https://github.com/torrust/torrust-tracker/pull/2430#discussion_r4173638121>; a fresh GraphQL capture reports all three `resolved=true`. Recorded the reply URLs, the F2 and F3 resolution references and the thread states. The earlier record commit, made before the replies existed, was replaced by this one, so the committed record carries the posted URLs.
- 2026-10-03 15:01 UTC - Ran `validate-audit-record.py --pr-number 2430 --base eb96d2957` against the fresh review-comment capture, replies included: `{"status": "ok", "rows": 3, "log_entries": 5, "failures": 0}`.

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
