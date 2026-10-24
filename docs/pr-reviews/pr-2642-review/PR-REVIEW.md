---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2525-ci-gate-required-check/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2642 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2642>.

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

One review was submitted so far, by the reviewer `da2ce7`, with the verdict `APPROVED` and a comment that
acknowledges the head `324418bbd`. Its body repeats its three inline threads, F1 to F3, so the body adds no
rows. No review by Copilot had been submitted when this record was written.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2642-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2642-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2642-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Historical statements now name a path that did not exist then

- PR number: 2642
- Source review ID: 5639972660
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2642#discussion_r4368480806>
- Concern: the T8 row and the 2026-10-16 progress-log entry of the archived #2507 specification record where
  the #2525 specification was created, but the archive had rewritten the path to `docs/issues/closed/`, which
  did not exist then. The log entry was edited in place, the file's `last-updated-utc` was not bumped, and
  step 4 of `cleanup-completed-issues` says not to rewrite historical records that accurately recorded the
  path of their time.
- Solution: restored the file to its content before the archive move, so both statements name the `open/` path
  again. That also makes the missing stamp bump unnecessary, since the file no longer differs from the base.
  The finding was right: both statements were code spans in a historical record, not links, so nothing
  required the repair. The description of the pull request said that this specification was repaired; it was
  corrected.
- Current-tree verification: `git diff torrust/develop...HEAD --stat --
  docs/issues/closed/2507-stop-requiring-pull-request-rebases/ISSUE.md` prints nothing, and
  `grep -c docs/issues/open/2525-ci-gate-required-check` on the file prints 2.
- Resolution reference: docs(issues): [#2525] keep the historical paths in the #2507 specification
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2642#discussion_r4368508484>

### F2 - The log says the review records kept their path, but one was repaired

- PR number: 2642
- Source review ID: 5639972660
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2642#discussion_r4368480811>
- Concern: the close-out log entry of the #2525 specification said that the pull-request review records keep
  the path they had at the time, but the change repaired
  `docs/pr-reviews/pr-2605-review/pr-2610-audit-record.md` (its frontmatter and a live link).
- Solution: the entry now names that record as the one with a live link, says that the other seven review
  records and the #2507 specification keep their path, and no longer lists the #2507 specification among the
  repaired documents, which F1 reverted. The entry is a log of the specification, not this audit's Processing
  Log, so it was corrected in place.
- Current-tree verification: `grep -c pr-2610-audit-record.md
  docs/issues/closed/2525-ci-gate-required-check/ISSUE.md` prints 1, and `git grep -l
  issues/open/2525-ci-gate-required-check -- docs/pr-reviews` lists seven records, which are the ones the
  entry names.
- Resolution reference: docs(issues): [#2525] name the repaired review record in the close-out log entry
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2642#discussion_r4368508544>

### F3 - Two stamps were not bumped

- PR number: 2642
- Source review ID: 5639972660
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2642#discussion_r4368480813>
- Concern: `last-updated-utc` of the #2525 specification was not bumped for the second commit, and the
  archive changed `issue-spec` in `manual-verification-evidence.md` without bumping that file's
  `last-updated-utc`, which step 4 of `cleanup-completed-issues` asks for.
- Solution: both stamps are now `2026-10-24 20:58`, the time of the fix to the specification. The other
  changed Markdown files that carry the field are the two new specifications, whose stamps are their creation
  times, and the #2507 specification, which F1 restored.
- Current-tree verification: `grep -n ^last-updated-utc` on the two files prints the new values, and the
  commit that set them is stamped 21:58:37 in UTC+1, which is 20:58:37 UTC.
- Resolution reference: docs(issues): [#2525] bump last-updated-utc of the documents the close-out changed
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2642#discussion_r4368508648>

## Processing Log

- 2026-10-24 20:52 UTC - The review of `da2ce7` arrived with three inline threads, F1 to F3, and an `APPROVED`
  verdict. All three were checked against the tree and confirmed; F1 also showed that a sentence of the pull
  request description was wrong.
- 2026-10-24 20:58 UTC - Fixed each finding in its own signed commit, in the order F1, F2, F3.
- 2026-10-24 21:02 UTC - Replied on the three threads after pushing the fixes, and corrected the description
  of the pull request.

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
