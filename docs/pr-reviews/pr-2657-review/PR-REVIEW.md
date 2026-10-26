# PR #2657 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2657>.

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

Copilot's review 5655168094 (submitted 2026-10-26 16:09 UTC) has one actionable finding, raised as `F5`,
which the review calls a re-raise of finding F5 of the PR #2641 audit. That finding belongs to
another pull request's audit, so it is `ORIGINAL` here.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F5 | review-finding:pr-2657-f5 | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F5 - Regression-case count says three, only two were red

- PR number: 2657
- Source review ID: 5655168094
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2657#discussion_r4379554068>
- Concern: the closed #2606 spec said the first three listed cases fail before the fix, but the third is the unknown-subject case, which the same sentence and the T6 row call a guard of existing behavior.
- Solution: changed "first three" to "first two" in the sentence.
- Current-tree verification: `grep -n "first three" docs/issues/closed/2606-2278-validate-audit-record-merged-fix-subjects/ISSUE.md` finds nothing, and line 137 reads "the first two and the `gh` failure case fail on the code before the fix".
- Resolution reference: docs(issues): [#2606] count the red regression cases as two in the closed spec
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2657#discussion_r4380238845>

## Processing Log

- 2026-10-26 17:10 UTC - Committed the wording fix and pushed it.
- 2026-10-26 17:13 UTC - Replied on the thread with disposition `FIXED` and resolved it.

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
