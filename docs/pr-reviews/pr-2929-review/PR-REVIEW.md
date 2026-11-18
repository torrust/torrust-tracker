---
pr-number: 2929
pr-url: https://github.com/torrust/torrust-tracker/pull/2929
last-updated-utc: "2026-11-18 11:00"
---

<!-- skill-link: process-pr-review -->

# PR #2929 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2929.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: https://github.com/torrust/torrust-tracker/pull/2929#issuecomment-6974884908 (comment created 2026-11-18 09:46:56 UTC by `josecelano-bot` on behalf of the maintainer, who chose the follow-up PR in the agent chat shortly before; review 5953861373 was submitted at 09:30:12 UTC and the PR merged at 09:42:00 UTC, before the push carrying F5 to F7; nothing was pushed or posted for them after the merge before this comment)

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

Round 1: Copilot review 5946976502 (COMMENTED, a lite review, since its full agentic review did not start before the timeout) with one inline finding, rated medium by Copilot. The audit severity scale (`Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`) has no medium level, so this audit maps Copilot's `Medium severity` badge to `Minor (inferred)` and its `Low severity` badge to `Nit (inferred)`, as the PR #2397, PR #2421 and PR #2430 audits do; `(inferred)` marks that the comment carries no `[Severity]` bracket. The `F` IDs are this audit's. Round 2: Copilot review 5947779724 (COMMENTED, a lite review again) on the head that fixed F1, with findings F2 to F4. The badges rate F2 and F3 `Medium severity` (`Minor (inferred)`) and F4 `Low severity` (`Nit (inferred)`). F2 asks for the rule above; F3 and F4 describe content the file does not have. Round 3: Copilot review 5948460850 (COMMENTED, a balanced review, on the head that recorded round 2) reported no open issue. Its summary mentions a minor archive-metadata concern omitted from the published review; its content is not retrievable, so it makes no row. Round 4: da2ce7 review 5953861373 (APPROVED) with findings F1 to F3, all `[Nit]` and none blocking. Those IDs are already used in this audit, so they are recorded as F5 to F7, each with the reviewer's ID in its detail entry.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2929-f1 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2929-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2929-f3 | Copilot | Minor (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2929-f4 | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F5 | review-finding:pr-2929-f5 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2929-f6 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2929-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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

### F5 - The archived test-design review has no frontmatter

- PR number: 2929
- Source review ID: 5953861373
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4611691532
- Concern: Step 3 of `cleanup-completed-issues` gives a supplementary document without frontmatter a minimal block linking back to the spec; `test-design-review.md` moved as a pure rename and had none.
- Solution: Fixed by follow-up PR #2934, merged into `develop` on 2026-11-18 at 10:59:19 UTC (`043255835`), because PR #2929 merged before the push that carried the fix. Added the minimal block: `spec-path`, `last-updated-utc`, and `semantic-links` with the archived `ISSUE.md`. The four older archived `test-design-review.md` files are outside this close-out and stay unchanged.
- Current-tree verification: `git show torrust/develop:docs/issues/closed/2847-1488-si-28-verify-hung-component-abort/test-design-review.md | head -7` shows the frontmatter block on `develop`.
- Resolution reference: docs(issues): [#2847] add the archive frontmatter to the SI-28 test-design review
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2934
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4612798091

### F6 - The round-1 reply stamp is a second late

- PR number: 2929
- Source review ID: 5953861373
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4611691540
- Concern: The round-1 entry gives the reply as `21:35:59Z`; the platform records `2026-11-17T21:35:58Z`.
- Solution: Fixed by follow-up PR #2934, merged into `develop` on 2026-11-18 at 10:59:19 UTC (`043255835`), because PR #2929 merged before the push that carried the fix. The `21:35:59Z` came from a `date -u` run after the reply command, not from its output. The round-1 entry stays as logged, since the log is append-only; a correction entry gives `21:35:58Z` and its source.
- Current-tree verification: On `develop`, the Processing Log's F6 correction entry gives `2026-11-17T21:35:58Z`, the reply's `created_at`.
- Resolution reference: docs(pr-reviews): [#2847] record da2ce7's round 4 findings F5 to F7 for PR 2929
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2934
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4612798644

### F7 - The record's late start is not logged

- PR number: 2929
- Source review ID: 5953861373
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4611691552
- Concern: `process-pr-review` creates the record before the first reply and logs a late start otherwise; the round-1 entry ends with "Created this record." after the reply, with no late-start note.
- Solution: Fixed by follow-up PR #2934, merged into `develop` on 2026-11-18 at 10:59:19 UTC (`043255835`), because PR #2929 merged before the push that carried the fix. The record did not exist before the reply: it was written after the `21:35:58Z` reply and committed at `21:36:28Z`. A correction entry logs the late start.
- Current-tree verification: On `develop`, the Processing Log's late-start entry states that the record was created after the round-1 reply.
- Resolution reference: docs(pr-reviews): [#2847] record da2ce7's round 4 findings F5 to F7 for PR 2929
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2934
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4612799202

## Processing Log

- 2026-11-17 21:36 UTC - Round 1: Copilot review 5946976502 (COMMENTED, 21:33:38Z) with finding F1. Fixed it, following the maintainer's choice for the non-blocking findings of PR #2911: committed the fix, then, before pushing so that one push carries the fix and this record, replied to the thread (21:35:59Z) naming the commit subject the push brings, and resolved it (21:36:07Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Created this record.
- 2026-11-17 22:37 UTC - Round 2: Copilot review 5947779724 (COMMENTED, 22:35:04Z) with findings F2 to F4. Fixed F2 in a commit; F3 and F4 describe content the file does not have, so they need no change. Replied to the three threads (22:36Z) and resolved them (22:36:45Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Recorded F2 to F4.
- 2026-11-17 23:37 UTC - Round 3: Copilot review 5948460850 (COMMENTED, 23:36:25Z) reported no open issue and listed F2 to F4 as resolved; the omitted archive-metadata concern in its summary has no retrievable content and makes no row. Correction: the round 2 Findings paragraph and the F4 row recorded F4 as medium and `Minor (inferred)`; the review's badge rates it `Low severity`, so the row now reads `Nit (inferred)` and the paragraph states both mappings.
- 2026-11-18 09:33 UTC - Round 4: da2ce7 review 5953861373 (APPROVED, 09:30:12Z per the platform) with findings F1 to F3, all Nit, recorded as F5 to F7. Fixed all three, following the maintainer's choice for non-blocking findings: committed the F5 fix, replied to the three threads (09:32:54Z, 09:32:57Z and 09:32:59Z, the replies' `created_at`), and resolved them by 09:33:11Z (a `date -u` run after the resolve commands), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Recorded F5 to F7.
- 2026-11-18 09:33 UTC - Correction (F6): the round-1 entry's reply stamp `21:35:59Z` came from a `date -u` run after the reply command; the reply's `created_at` is `2026-11-17T21:35:58Z`. The round-1 resolution stamp `21:36:07Z` and the round-2 resolution stamp `22:36:45Z` came the same way, from a `date -u` run after the resolve commands.
- 2026-11-18 09:33 UTC - Late start (F7): this record was created after the round-1 reply, not before it as `process-pr-review` asks. The reply was posted at `21:35:58Z`; the record was written after it and committed at `21:36:28Z`. The round-1 entry's closing "Created this record." reflects that order.
- 2026-11-18 09:50 UTC - PR #2929 merged at 09:42:00 UTC at head `5d1efbfb9`, before the push that carried the F5 fix and this record's round 4 entries; the replies of 09:32 UTC named that push. The maintainer chose a follow-up PR; `josecelano-bot` posted the approval comment at 09:46:56 UTC. The two fix commits were cherry-picked onto `develop` as follow-up PR #2934. Replied on the three threads with a correction naming #2934 (09:50:29Z, 09:50:32Z and 09:50:34Z, the replies' `created_at`) and reopened them by 09:50:41Z (a `date -u` run after the unresolve commands). F5 to F7 are `FOLLOW_UP` / `OPEN` until #2934 merges.
- 2026-11-18 11:00 UTC - Follow-up PR #2934 merged at 10:59:19 UTC (`043255835`). Replied on the three threads with the merged fix (10:59:46Z, 10:59:48Z and 10:59:51Z, the replies' `created_at`) and resolved them by 10:59:57Z (a `date -u` run after the resolve commands); F5 to F7 are `FIXED` / `RESOLVED`. GraphQL refresh: zero unresolved threads on PR #2929.

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
