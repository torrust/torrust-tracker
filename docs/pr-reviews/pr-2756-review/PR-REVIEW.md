---
pr-number: 2756
pr-url: https://github.com/torrust/torrust-tracker/pull/2756
last-updated-utc: "2026-11-09 11:18"
---

# PR #2756 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2756.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A. Review 5803833977 was replied to and resolved on 2026-11-06, while PR 2756 was open, so the post-merge workflow does not apply; only this audit file was written after the merge, in the close-out of issue #2712.

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Round 1: Copilot review 5803833977 on the head `973816c66` (three inline findings), submitted 2026-11-06 16:55:54 UTC. All three are valid and fixed by commit 3b592143d on this pull request, which merged on 2026-11-09 10:47 UTC.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2756-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2756-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2756-f3 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - FOLLOW_UP findings cite commit subjects instead of reply URLs

- PR number: 2756
- Source review ID: 5803833977
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2756#discussion_r4495205513
- Concern: The round 2 findings F2 to F4 of the PR 2750 audit were `FOLLOW_UP`, but their resolution references were commit subjects rather than the reply URLs that `process-pr-review/SKILL.md` requires.
- Solution: Valid. Each resolution reference is now the finding's reply URL and the commit subject moved into Solution.
- Current-tree verification: At the merged head of PR 2756 (`92665ae17`), `sed -n` of the three F2 to F4 detail entries of `docs/pr-reviews/pr-2750-review/PR-REVIEW.md` shows a reply URL in each `Resolution reference`; the validation script reported 0 failures.
- Resolution reference: docs(pr-reviews): [#2712] fix the round 2 audit references and log times
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2756#discussion_r4496710608

### F2 - The recorded grep matched its own line

- PR number: 2756
- Source review ID: 5803833977
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2756#discussion_r4495205579
- Concern: The F3 verification of the PR 2750 audit quoted a `grep` whose pattern also occurs in the line itself, so the claim was not reproducible.
- Solution: Valid. The verification describes the tracking row and records a search restricted to table rows.
- Current-tree verification: At the merged head of PR 2756 (`92665ae17`), the restricted search `grep -nE '^\| F[0-9]+ .*\| Low \|'` on the audit finds nothing.
- Resolution reference: docs(pr-reviews): [#2712] fix the round 2 audit references and log times
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2756#discussion_r4496710905

### F3 - Log entries and frontmatter carried times earlier than the events

- PR number: 2756
- Source review ID: 5803833977
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2756#discussion_r4495205624
- Concern: The 16:50 UTC log entry and `last-updated-utc` of the PR 2750 audit were already in commit 973816c66 made at 16:35:44 UTC.
- Solution: Valid. The entries stay unchanged and an appended correction gives the evidenced times; `last-updated-utc` holds the correction time. The same mistake in the 14:10 UTC entry is corrected in the same entry.
- Current-tree verification: At the merged head of PR 2756 (`92665ae17`), the audit's Processing Log ends with the correction entry and the validation script reported 0 failures.
- Resolution reference: docs(pr-reviews): [#2712] fix the round 2 audit references and log times
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2756#discussion_r4496711244

## Processing Log

- 2026-11-09 10:50 UTC - Started audit during the close-out of issue #2712; round 1 (Copilot review 5803833977, processed on 2026-11-06) recorded: F1 to F3 were fixed in the commit cited above, replied to on 2026-11-06 at 19:30 UTC and resolved on this pull request; the follow-up PR merged on 2026-11-09 10:47 UTC.
- 2026-11-09 11:18 UTC - Correction: the entry stamped 2026-11-09 10:50 UTC and the first `last-updated-utc` were already in commit c3025f0da, made at 10:49:32 UTC, so those times are wrong; the entry is kept unchanged. This audit was started at 10:49:32 UTC. The verification lines of F1 to F3 are now scoped to the merged head of PR 2756, and the Ownership section says why no post-merge approval applies (maintainer review of PR 2780, findings F2 and F4).

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
