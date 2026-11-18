---
pr-number: 2934
pr-url: https://github.com/torrust/torrust-tracker/pull/2934
last-updated-utc: "2026-11-18 10:20"
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

Round 1: da2ce7 review 5954570853 (APPROVED) with finding F1 (`[Nit]`, non-blocking); the reviewer's ID is this audit's ID. Round 2: Copilot review 5954615881 (COMMENTED, a lite review of head `d0e31cc0a`) with three inline findings, F2 to F4 in this audit. Its badges rate all three `Medium severity`, recorded as `Minor (inferred)`, as in the PR 2929 audit. All three ask for style changes to the PR 2929 audit that its template or its append-only log rule rules out, so none is changed.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2934-f1 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2934-f2 | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2934-f3 | Copilot | Minor (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2934-f4 | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |

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

## Processing Log

- 2026-11-18 10:15 UTC - Started audit, before any reply: da2ce7 review 5954570853 (APPROVED, 10:13:30Z per the platform) with finding F1.
- 2026-11-18 10:16 UTC - Fixed F1 outside the tree: changed the title after 10:15:52Z (a `date -u` run just before the edit command), posted the PR conversation response (10:16:03Z), replied on the thread (10:16:12Z; both stamps are the platform's `created_at`) and resolved it by 10:16:16Z (a `date -u` run after the resolve command); a GraphQL refresh found no unresolved thread.
- 2026-11-18 10:20 UTC - Round 2: Copilot review 5954615881 (COMMENTED, 10:16:33Z per the watcher) with findings F2 to F4, all style suggestions on the PR 2929 audit, none changed. Replied `Superseded by <FindingId>: <reason>.` on the three threads (10:20:18Z, 10:20:20Z and 10:20:22Z, the replies' `created_at`) and resolved them by 10:20:28Z (a `date -u` run after the resolve commands); a GraphQL refresh found no unresolved thread.

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
