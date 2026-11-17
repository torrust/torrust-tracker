---
pr-number: 2929
pr-url: https://github.com/torrust/torrust-tracker/pull/2929
last-updated-utc: "2026-11-17 23:37"
---

<!-- skill-link: process-pr-review -->

# PR #2929 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2929.

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

Round 1: Copilot review 5946976502 (COMMENTED, a lite review, since its full agentic review did not start before the timeout) with one inline finding, rated medium by Copilot. The audit severity scale (`Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`) has no medium level, so this audit maps Copilot's `Medium severity` badge to `Minor (inferred)` and its `Low severity` badge to `Nit (inferred)`, as the PR #2397, PR #2421 and PR #2430 audits do; `(inferred)` marks that the comment carries no `[Severity]` bracket. The `F` IDs are this audit's. Round 2: Copilot review 5947779724 (COMMENTED, a lite review again) on the head that fixed F1, with findings F2 to F4. The badges rate F2 and F3 `Medium severity` (`Minor (inferred)`) and F4 `Low severity` (`Nit (inferred)`). F2 asks for the rule above; F3 and F4 describe content the file does not have. Round 3: Copilot review 5948460850 (COMMENTED, a balanced review, on the head that recorded round 2) reported no open issue. Its summary mentions a minor archive-metadata concern omitted from the published review; its content is not retrievable, so it makes no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2929-f1 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2929-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2929-f3 | Copilot | Minor (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2929-f4 | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Unquoted `last-updated-utc` in the evidence frontmatter

- PR number: 2929
- Source review ID: 5946976502
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606077014
- Concern: The archived evidence file's `last-updated-utc` was unquoted, while the spec and the EPIC quote it; YAML can parse an unquoted timestamp as a datetime instead of a string.
- Solution: Quoted the value, as the other frontmatter in this PR does. The value was already unquoted before the archive move; this PR only changed it.
- Current-tree verification: `head -5` of `docs/issues/closed/2847-1488-si-28-verify-hung-component-abort/manual-verification-evidence.md` shows `last-updated-utc: "2026-11-17 21:35"`; the pre-commit staged-frontmatter validator passed.
- Resolution reference: `docs(issues): [#2847] quote the evidence file's last-updated-utc timestamp`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606101763

### F2 - The medium-to-minor severity mapping is not explained

- PR number: 2929
- Source review ID: 5947779724
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606720677
- Concern: The Findings paragraph mapped Copilot's medium rating to `Minor (inferred)` without a stated rule, so the normalization was not reproducible.
- Solution: The paragraph states the rule: the audit scale has no medium level, so a `Medium severity` badge maps to `Minor (inferred)`, as in the PR #2421 and PR #2430 audits.
- Current-tree verification: `grep -n 'has no medium level' docs/pr-reviews/pr-2929-review/PR-REVIEW.md` finds the rule in the Findings paragraph; `grep -n 'Minor (inferred)' docs/pr-reviews/pr-2430-review/PR-REVIEW.md` shows the same mapping there.
- Resolution reference: `docs(pr-reviews): [#2847] state the medium-to-minor severity mapping in the PR 2929 audit`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606734628

### F3 - A short SHA in the Processing Log

- PR number: 2929
- Source review ID: 5947779724
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606720745
- Concern: The Processing Log was said to cite PR #2911 findings by a short SHA such as `da2ce7`, against the record's rule to cite commit subjects.
- Solution: No change. The record cites no commit SHA; `da2ce7` is a reviewer's GitHub login, which did not occur in the reviewed file (only this entry names it), and the F1 fix is cited by its commit subject.
- Current-tree verification: on the reviewed head, `git show <reviewed head>:docs/pr-reviews/pr-2929-review/PR-REVIEW.md | grep -n da2ce7` found nothing, nor did a search for six-to-forty-character hex words outside review and comment IDs; on the current tree, `grep -n da2ce7` matches only this F3 entry.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606735041
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606735041

### F4 - Table rows starting with `||`

- PR number: 2929
- Source review ID: 5947779724
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606720806
- Concern: The table rows were said to start with `||`, rendering an empty leading column.
- Solution: No change. Every row of the Findings table starts with a single `|`.
- Current-tree verification: `grep -n '^||' docs/pr-reviews/pr-2929-review/PR-REVIEW.md` finds nothing.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606735442
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606735442

## Processing Log

- 2026-11-17 21:36 UTC - Round 1: Copilot review 5946976502 (COMMENTED, 21:33:38Z) with finding F1. Fixed it, following the maintainer's choice for the non-blocking findings of PR #2911: committed the fix, then, before pushing so that one push carries the fix and this record, replied to the thread (21:35:59Z) naming the commit subject the push brings, and resolved it (21:36:07Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Created this record.
- 2026-11-17 22:37 UTC - Round 2: Copilot review 5947779724 (COMMENTED, 22:35:04Z) with findings F2 to F4. Fixed F2 in a commit; F3 and F4 describe content the file does not have, so they need no change. Replied to the three threads (22:36Z) and resolved them (22:36:45Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Recorded F2 to F4.
- 2026-11-17 23:37 UTC - Round 3: Copilot review 5948460850 (COMMENTED, 23:36:25Z) reported no open issue and listed F2 to F4 as resolved; the omitted archive-metadata concern in its summary has no retrievable content and makes no row. Correction: the round 2 Findings paragraph and the F4 row recorded F4 as medium and `Minor (inferred)`; the review's badge rates it `Low severity`, so the row now reads `Nit (inferred)` and the paragraph states both mappings.

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
