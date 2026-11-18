---
pr-number: 2934
pr-url: https://github.com/torrust/torrust-tracker/pull/2934
last-updated-utc: "2026-11-18 10:35"
---

<!-- skill-link: process-pr-review -->

# PR #2934 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2934.

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

Round 1: da2ce7 review 5954570853 (APPROVED) with finding F1 (`[Nit]`, non-blocking); the reviewer's ID is this audit's ID. Round 2: Copilot review 5954615881 (COMMENTED, a lite review of the head whose subject is `docs(pr-reviews): [#2847] record the post-merge follow-up of F5 to F7 in the PR 2929 audit`) with three inline findings, F2 to F4 in this audit. Its badges rate all three `Medium severity`, recorded as `Minor (inferred)`, as in the PR 2929 audit. All three ask for style changes to the PR 2929 audit, and none is changed: the template's one-line Ownership field rules out F2, the append-only log rule rules out F4, and F3 would only restyle a merged record. Round 3: da2ce7 review 5954868002 (APPROVED) with findings F9 (`[Nit]`), F10 (`[Minor]`) and F11 (`[Nit]`), none blocking; the reviewer's IDs are this audit's IDs.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2934-f1 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2934-f2 | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2934-f3 | Copilot | Minor (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2934-f4 | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F9 | review-finding:pr-2934-f9 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2934-f10 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2934-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The title cites a closed issue and calls pre-merge Nits post-merge

- PR number: 2934
- Source review ID: 5954570853
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612220496
- Concern: The pull-request title `docs(issues): [#2847] apply da2ce7's post-merge Nits on PR 2929` cites issue #2847, which is closed and which this PR does not close, against the Title rule of `open-pull-request`; and the Nits were submitted before PR #2929 merged, so they are not post-merge Nits.
- Solution: The title is now `docs(pr-reviews): apply the unpushed round-4 Nits of PR 2929 as a follow-up`, without an issue tag; the description keeps `Refs #2847`. The fix is outside the repository tree.
- Current-tree verification: `gh-bot pr view 2934 --repo torrust/torrust-tracker --json title --jq .title` prints `docs(pr-reviews): apply the unpushed round-4 Nits of PR 2929 as a follow-up`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2934#issuecomment-6975556976
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612253201

### F2 - The post-merge approval line is dense

- PR number: 2934
- Source review ID: 5954615881
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612257180
- Concern: The PR 2929 audit's Ownership line packs the approval URL, several stamps and the reason into one bullet; Copilot suggests sub-bullets.
- Solution: No change. The template's Ownership field is one line, and the line keeps the one-bullet form the PR 2690, 2730 and 2870 audits use for the same record: URL first, context in parentheses.
- Current-tree verification: `grep -n 'Post-merge workflow approval' docs/templates/PR-REVIEW-TEMPLATE.md` prints line 24, a single bullet; each of the PR 2690, 2730 and 2870 audits has one such bullet with a URL and parenthesized context.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612303053
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612303053

### F3 - The Findings paragraph is dense

- PR number: 2934
- Source review ID: 5954615881
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612257258
- Concern: The PR 2929 audit's Findings paragraph covers four rounds and the severity rule in one paragraph; Copilot suggests per-round bullets and a separate rule paragraph.
- Solution: No change. The paragraph keeps the round-by-round prose of the PR 2911 and PR 2929 audits, and the severity rule sits at its first use; restructuring would change only the style of a merged record.
- Current-tree verification: Inspection: the PR 2911 audit's Findings paragraph uses the same round-by-round prose.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612303516
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612303516

### F4 - Mixed timestamp formats in the Processing Log

- PR number: 2934
- Source review ID: 5954615881
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612257325
- Concern: Log entries mix `HH:MM UTC`, `HH:MM:SSZ` and full ISO stamps; Copilot suggests one ISO format throughout.
- Solution: No change. The log is append-only, so earlier entries cannot be rewritten; each entry keeps the template's `YYYY-MM-DD HH:MM UTC` prefix, and second-precision stamps quote the platform's `created_at` or name a `date -u` run, which keeps each stamp's source visible.
- Current-tree verification: `docs/templates/PR-REVIEW-TEMPLATE.md` line 113 gives the entry prefix and line 115 says to append entries only.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612303959
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612303959

### F9 - The rename stamp comes from a `date -u` run

- PR number: 2934
- Source review ID: 5954868002
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612464798
- Concern: The round-1 entry gives the title change as after a `date -u` run of 10:15:52Z, while the timeline records the rename at 10:15:53Z; `process-pr-review` takes such a stamp from the command output or the platform.
- Solution: A log entry, appended because the log is append-only, gives the rename as `2026-11-18T10:15:53Z` from the issue timeline's `renamed` event. A sweep of this record's other stamps found the rest are the platform's `created_at`, the watcher's stamps, or labelled `date -u` bounds for actions the platform records no time for (thread resolution).
- Current-tree verification: `gh-bot api repos/torrust/torrust-tracker/issues/2934/timeline --jq '.[] | select(.event=="renamed") | .created_at'` prints `2026-11-18T10:15:53Z`; the appended log entry names it.
- Resolution reference: `docs(pr-reviews): record da2ce7's round 3 findings F9 to F11 for PR 2934`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612488966

### F10 - The round-2 sentence cites a PR-branch commit id

- PR number: 2934
- Source review ID: 5954868002
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612464785
- Concern: The round-2 sentence named the reviewed head by its PR-branch id, which a rebase would rewrite; the `open-pull-request` commit-id check printed it.
- Solution: The sentence names that head by its commit subject instead, as the PR 2693 audit does.
- Current-tree verification: The `open-pull-request` commit-id check, run on this branch against `torrust/develop`, prints nothing.
- Resolution reference: `docs(pr-reviews): record da2ce7's round 3 findings F9 to F11 for PR 2934`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612489380

### F11 - The round-2 sentence gives all three findings the same reason

- PR number: 2934
- Source review ID: 5954868002
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612464806
- Concern: The sentence said the template or the append-only log rule rules out all three of F2 to F4, which holds for F2 and F4 but not F3.
- Solution: The sentence now gives each its own reason: the template for F2, the log rule for F4, and for F3 that it would only restyle a merged record, matching the detail entries.
- Current-tree verification: `grep -n 'F3 would only restyle a merged record' docs/pr-reviews/pr-2934-review/PR-REVIEW.md` finds the sentence in the Findings paragraph.
- Resolution reference: `docs(pr-reviews): record da2ce7's round 3 findings F9 to F11 for PR 2934`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2934#discussion_r4612489795

## Processing Log

- 2026-11-18 10:15 UTC - Started audit, before any reply: da2ce7 review 5954570853 (APPROVED, 10:13:30Z per the platform) with finding F1.
- 2026-11-18 10:16 UTC - Fixed F1 outside the tree: changed the title after 10:15:52Z (a `date -u` run just before the edit command), posted the PR conversation response (10:16:03Z), replied on the thread (10:16:12Z; both stamps are the platform's `created_at`) and resolved it by 10:16:16Z (a `date -u` run after the resolve command); a GraphQL refresh found no unresolved thread.
- 2026-11-18 10:20 UTC - Round 2: Copilot review 5954615881 (COMMENTED, 10:16:33Z per the watcher) with findings F2 to F4, all style suggestions on the PR 2929 audit, none changed. Replied `Superseded by <FindingId>: <reason>.` on the three threads (10:20:18Z, 10:20:20Z and 10:20:22Z, the replies' `created_at`) and resolved them by 10:20:28Z (a `date -u` run after the resolve commands); a GraphQL refresh found no unresolved thread.
- 2026-11-18 10:35 UTC - Round 3: da2ce7 review 5954868002 (APPROVED, 10:33:14Z per the platform) with findings F9 to F11, none blocking; fixed all three in this record.
- 2026-11-18 10:35 UTC - Correction (F9): the round-1 entry stamps the title change from a `date -u` run (after 10:15:52Z); the issue timeline's `renamed` event records it at `2026-11-18T10:15:53Z`.
- 2026-11-18 10:35 UTC - Replied on the three round-3 threads (10:35:13Z, 10:35:16Z and 10:35:18Z, the replies' `created_at`), each naming the commit subject the following push brings, and resolved them by 10:35:24Z (a `date -u` run after the resolve commands); a GraphQL refresh found no unresolved thread.

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
