---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/planning/cleanup-completed-issues/SKILL.md
    - docs/issues/closed/2473-2003-asynchronous-discussion-rounds/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2485 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2485>.

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

Copilot review 5454515898 (09:30 UTC) reported "0 open findings". Its summary says the
manual-verification record's timestamp was not advanced, but it makes no request and opens no
thread, so it gets no row; human review 5454581655 raised the same concern as a finding.

Human review 5454581655 by `da2ce7` (`CHANGES_REQUESTED`, 09:36 UTC) supplied the finding ID `F1`
with a `[Minor]` severity; it is kept. Its body summarizes the same thread and lists the rest under
"Checked, no finding", so it adds no other row.

Human review 5455225153 by `da2ce7` (`CHANGES_REQUESTED`, round 2, 10:31 UTC) verified F1 fixed and
supplied the new finding ID `F2` with a `[Minor]` severity; it is kept. Its body re-checks F1 and
lists the rest under "Checked, no finding", so it adds no other row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2485-f1` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2485-f2` | Human | Minor | formatting | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The archived evidence record's last-updated stamp was not bumped

- PR number: 2485
- Source review ID: 5454581655
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2485#discussion_r4217310326>
- Concern: The archive updated the `issue-spec` frontmatter of
  `manual-verification-evidence.md` but left `last-updated-utc` at `2026-10-08 06:50`.
  `cleanup-completed-issues` Step 4 requires bumping the stamp of a supplementary record whose
  frontmatter changes, as the #2417 and #2446 archives did.
- Solution: Set the stamp to the time of the fix in a new commit; the repository keeps intermediate
  commits, so the archive commit was not amended.
- Current-tree verification: `manual-verification-evidence.md:4` reads
  `last-updated-utc: 2026-10-08 10:14`; `frontmatter-validator` exits 0 on the file.
- Resolution reference: chore(issues): [#2473] bump the archived evidence record's last-updated stamp
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2485#discussion_r4217681494>

### F2 - The Status Values block was not copied verbatim from the template

- PR number: 2485
- Source review ID: 5455225153
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2485#discussion_r4217825381>
- Concern: `PR-REVIEW-TEMPLATE.md` marks the Status Values block "Copied verbatim into each audit
  record.", but this record's Category item wrapped after `documentation`, like the PR #2481 and
  PR #2474 records it was copied from, instead of after `correctness`. The same rewrap was raised on
  PR #2468.
- Solution: Replaced the block with the template's own block instead of an earlier record's.
- Current-tree verification: the Category item (:34-35) wraps after `correctness`; `diff` against
  the template's Status Values block, excluding its guidance comment, shows no change.
- Resolution reference: docs(pr-reviews): [#2473] copy the Status Values block verbatim in the PR #2485 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2485#discussion_r4217841415>

## Processing Log

- 2026-10-08 10:13 UTC - Started audit. Fetched the one review thread (unresolved, not outdated)
  and both review bodies; normalized them into F1.
- 2026-10-08 10:15 UTC - Pushed the F1 fix after the pre-commit gate passed, re-checked the claim
  against the pushed tree, and replied on the thread.
- 2026-10-08 10:33 UTC - Human review 5455225153 (round 2) verified F1 and raised F2. Pushed the
  F2 fix after the pre-commit gate passed, re-checked it against the pushed tree, and replied on the
  thread.

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
